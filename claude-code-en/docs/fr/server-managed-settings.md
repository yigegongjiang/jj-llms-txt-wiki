> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les paramètres gérés par le serveur

> Configurez centralement Claude Code pour votre organisation via des paramètres livrés par le serveur, sans nécessiter d'infrastructure de gestion des appareils.

Les paramètres gérés par le serveur permettent aux propriétaires d'organisation de configurer centralement Claude Code à partir de [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) dans la console claude.ai. Les clients Claude Code récupèrent automatiquement ces paramètres lorsque les utilisateurs s'authentifient avec une connexion éligible sur une plateforme où la livraison gérée par le serveur est prise en charge. Voir [Disponibilité des plateformes](#platform-availability) pour les connexions et les plateformes qui se qualifient.

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

Claude Code prend en charge deux approches pour la configuration centralisée. Les paramètres gérés par le serveur livrent la configuration à partir des serveurs d'Anthropic. Les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) sont déployés directement sur les appareils via des stratégies natives du système d'exploitation (préférences gérées macOS, registre Windows) ou des fichiers de paramètres gérés.

| Approche                                                                                     | Idéal pour                                                          | Modèle de sécurité                                                                                                                         |
| :------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------- |
| **Paramètres gérés par le serveur**                                                          | Organisations sans MDM, ou utilisateurs sur des appareils non gérés | Paramètres que Claude Code récupère à partir des serveurs d'Anthropic au démarrage et actualise toutes les heures pendant la session       |
| **[Paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms)** | Organisations avec MDM ou gestion des points de terminaison         | Paramètres déployés sur les appareils via des profils de configuration MDM, des stratégies de registre ou des fichiers de paramètres gérés |

Si vos appareils sont inscrits dans une solution MDM ou de gestion des points de terminaison, les paramètres gérés par le point de terminaison offrent des garanties de sécurité plus fortes car le fichier de paramètres peut être protégé contre les modifications de l'utilisateur au niveau du système d'exploitation. Les paramètres gérés par le point de terminaison n'atteignent pas les [sessions cloud](/docs/fr/model-config#surface-coverage) dans les environnements hébergés par Anthropic, donc les organisations dont les développeurs exécutent des sessions cloud doivent également configurer les paramètres gérés par le serveur. Les sessions dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments) lisent également le fichier de paramètres gérés dans l'image du runner. La [précédence des paramètres](#settings-precedence) ci-dessous indique quand ce fichier s'applique.

<h2 id="configure-server-managed-settings">
  Configurer les paramètres gérés par le serveur
</h2>

<Steps>
  <Step title="Ouvrir la console d'administration">
    Dans la console claude.ai, accédez à [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Si le lien vous redirige vers une page Admin Settings différente au lieu de la page Claude Code, votre compte n'a pas le rôle requis. Les rôles Admin et autres rôles non-Propriétaire ne peuvent pas afficher ou modifier les paramètres gérés, donc demandez à un Propriétaire ou Propriétaire principal de votre organisation de faire la modification. Consultez [Contrôle d'accès](#access-control).
  </Step>

  <Step title="Définir vos paramètres">
    Ajoutez votre configuration en JSON. Tous les [paramètres disponibles dans `settings.json`](/docs/fr/settings-reference#all-settings) sont pris en charge, sauf ceux limités à la livraison de politique au niveau du système d'exploitation ; consultez [Limitations actuelles](#current-limitations) pour cette courte liste. Cela inclut les [hooks](/docs/fr/hooks), les [variables d'environnement](/docs/fr/env-vars) et les [paramètres réservés à la gestion](/docs/fr/managed-settings#managed-only-settings) comme `allowManagedPermissionRulesOnly`.

    Cet exemple applique une liste de refus de permissions, empêche les utilisateurs de contourner les permissions et restreint les règles de permission à celles définies dans les paramètres gérés. La règle `Bash(curl *)` correspond à `curl` [tel que Claude l'écrit](/docs/fr/permissions#bash-rule-limits), et non à `/usr/bin/curl` ou `sh -c 'curl …'` ; pour l'application de la sécurité réseau qui ne dépend pas du texte de la commande, ajoutez un [bloc `sandbox` avec `allowManagedDomainsOnly`](/docs/fr/sandboxing#configure-the-sandbox-for-your-organization).

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

    Parce que les hooks exécutent des commandes shell, les utilisateurs dans les sessions interactives voient une [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs) avant que Claude Code ne les applique.

    Pour configurer le classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) afin qu'il connaisse les dépôts, les buckets et les domaines de confiance de votre organisation, livrez un bloc `autoMode` de la même manière ; consultez [Configurer le mode auto](/docs/fr/auto-mode-config) pour savoir comment les entrées `autoMode` affectent ce que le classificateur bloque et les avertissements importants concernant les champs `environment`, `allow`, `soft_deny` et `hard_deny`.
  </Step>

  <Step title="Enregistrer et déployer">
    Enregistrez vos modifications. Les clients Claude Code reçoivent les paramètres mis à jour au prochain démarrage ou lors du cycle d'interrogation horaire.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Vérifier la livraison des paramètres
</h3>

Pour confirmer que les paramètres sont appliqués, demandez à un utilisateur de redémarrer Claude Code. Si la configuration inclut des paramètres qui déclenchent la [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs), l'utilisateur voit une invite décrivant les paramètres gérés la prochaine fois que Claude Code les récupère : au prochain démarrage, ou dans l'heure suivante dans une session interactive en cours. Vous pouvez également vérifier que les règles de permission gérées sont actives en demandant à un utilisateur d'exécuter `/permissions` pour afficher ses règles de permission effectives.

Pour vérifier le résultat de la récupération sur une machine spécifique, demandez à l'utilisateur d'exécuter `claude doctor` et de lire la ligne `Managed settings (remote)`. Nécessite Claude Code v2.1.248 ou version ultérieure. La ligne signale l'un des quatre résultats :

* Les paramètres livrés ont été chargés
* Votre organisation n'a pas de paramètres gérés par le serveur configurés
* La récupération a échoué, avec la cause et si une politique en cache s'applique toujours
* Claude Code a ignoré la récupération, avec la raison. Consultez [Disponibilité de la plateforme](#platform-availability) pour les fournisseurs et configurations qui l'ignorent

Pendant que la récupération est toujours en cours, la ligne signale cela à la place.

Dans une session en cours, `/status` affiche la même ligne après une récupération échouée, et pour certaines causes d'ignorance de récupération, comme une variable de fournisseur tiers ou une `ANTHROPIC_BASE_URL` personnalisée exportée dans le shell de l'utilisateur.

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

La plupart des [clés de paramètres](/docs/fr/settings-reference#all-settings) fonctionnent dans n'importe quel domaine. Une poignée de clés ne sont lues que dans les paramètres gérés et n'ont aucun effet lorsqu'elles sont placées dans les fichiers de paramètres utilisateur ou projet. Consultez [paramètres réservés à la gestion](/docs/fr/managed-settings#managed-only-settings) pour les contrôles de permission et de plugin, ou lisez la colonne Scope de l'index [Tous les paramètres](/docs/fr/settings-reference#all-settings) pour l'ensemble complet.

<h3 id="current-limitations">
  Limitations actuelles
</h3>

Les paramètres gérés par le serveur ont les limitations suivantes :

* Les paramètres s'appliquent uniformément à tous les utilisateurs de l'organisation. Les configurations par groupe ne sont pas encore prises en charge.
* Un fichier [`managed-mcp.json`](/docs/fr/managed-mcp) ne peut pas être distribué via les paramètres gérés par le serveur. Livrez plutôt les clés de politique `allowedMcpServers` et `deniedMcpServers` à la place. Dans Claude Code v2.1.259 ou version ultérieure, vous pouvez également fournir des serveurs distants avec [`managedMcpServers`](/docs/fr/managed-mcp#provide-servers-through-managed-settings), qui accepte uniquement les serveurs `http` et `sse` et ne prend pas le contrôle exclusif de la même manière que le fichier.

  Claude Code lit un fichier `managed-mcp.json` déployé à son [chemin système](/docs/fr/managed-mcp#exclusive-control-with-managed-mcp-json) séparément du niveau des paramètres gérés, donc le fichier s'applique toujours lorsque les paramètres gérés par le serveur sont en vigueur.
* Les paramètres limités aux sources de politique au niveau du système d'exploitation, tels que `policyHelper` et `wslInheritsWindowsSettings`, ne sont pas respectés. Déployez-les plutôt via MDM ou un fichier `managed-settings.json` système. Un `policyHelper` déployé de cette manière s'exécute uniquement lorsque sa source est celle sélectionnée sous [priorité au sein du niveau géré](/docs/fr/managed-settings#precedence-within-the-managed-tier).

<h2 id="settings-delivery">
  Livraison des paramètres
</h2>

<h3 id="settings-precedence">
  Précédence des paramètres
</h3>

Les paramètres gérés par le serveur et les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) occupent tous deux le niveau le plus élevé dans la [hiérarchie des paramètres](/docs/fr/settings#settings-precedence) de Claude Code. Aucun autre niveau de paramètres ne peut les remplacer, y compris les arguments de ligne de commande, à l'exception des [exceptions à la précédence des paramètres gérés](/docs/fr/settings#exceptions-to-managed-settings-precedence).

Au sein du niveau géré, Claude Code utilise par défaut la première source qui livre au moins une clé de stratégie, en vérifiant d'abord les paramètres gérés par le serveur, puis les paramètres gérés par le point de terminaison, à l'exception des [clés d'exception couvertes ensuite](#per-key-exceptions-across-managed-sources). [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#precedence-within-the-managed-tier) contient le classement complet, l'exception pour les clés de contrôle, et l'opt-in qui s'applique à chaque source.

Si la source sélectionnée est une stratégie MDM ou un fichier de paramètres gérés dont le [`policyHelper`](/docs/fr/settings-reference#policyhelper) fournit des paramètres gérés, la sortie du helper remplace cette source en tant que seule configuration gérée pour l'exécution. Claude Code ne consulte pas un `policyHelper` configuré dans les paramètres MDM ou basés sur des fichiers tandis que les paramètres gérés par le serveur livrent une clé de stratégie.

Si une récupération ultérieure trouve les paramètres gérés par le serveur supprimés, Claude Code exécute ce helper immédiatement au lieu du prochain lancement. L'entrée [`policyHelper`](/docs/fr/settings-reference#policyhelper) couvre ce qui se passe lorsque cette exécution échoue.

Si vous effacez votre configuration de paramètres gérés par le serveur dans la console d'administration avec l'intention de revenir à une stratégie plist ou registre gérée par le point de terminaison, sachez que les [paramètres en cache](#fetch-and-caching-behavior) persistent sur les machines clientes jusqu'à la prochaine récupération réussie, et les clés qui [s'appliquent uniquement au prochain lancement](#fetch-and-caching-behavior), telles que `model`, restent en vigueur jusqu'à ce que chaque client redémarre. Exécutez `/status` pour voir quelle source gérée est active.

<h3 id="per-key-exceptions-across-managed-sources">
  Exceptions par clé entre sources gérées
</h3>

Trois types de clés sont des exceptions à la règle de non-fusion :

* **Clés de verrouillage entre sources** : un petit ensemble de clés, telles que les verrous de liste d'autorisation du sandbox, [listées sur la page des paramètres gérés](/docs/fr/managed-settings#precedence-within-the-managed-tier). Claude Code les honore lorsqu'une source gérée contrôlée par un administrateur les définit ; le niveau de registre HKCU inscriptible par l'utilisateur est exclu.

  Lorsqu'un [`policyHelper`](/docs/fr/settings-reference#policyhelper) fournit des paramètres gérés, sa sortie est la seule source que ces vérifications lisent, à l'exception de [`forceRemoteSettingsRefresh`](/docs/fr/settings-reference#forceremotesettingsrefresh), que Claude Code lit à partir des sources d'administrateur directement au démarrage.
* **Le bloc `env`** : à l'exception de l'unité de télémétrie et des variables de routage associées à une clé d'identifiant, toutes deux couvertes ci-dessous, il fusionne par clé entre les sources contrôlées par l'administrateur. Pour chaque variable d'environnement, la source de priorité la plus élevée qui la définit gagne, et les sources d'administrateur inférieures remplissent les variables que les sources supérieures laissent non définies. Une entrée `env` gérée par le point de terminaison s'applique donc chaque fois que la configuration gérée par le serveur laisse cette variable non définie, ou tandis qu'une valeur de serveur en cache pour elle est [retenue en attente de confirmation du serveur](#fetch-and-caching-behavior). Nécessite Claude Code v2.1.223 ou version ultérieure. Avant v2.1.223, Claude Code applique uniquement le bloc `env` de la source sélectionnée.
  * **Unité de télémétrie** : les clés d'exportateur `OTEL_EXPORTER_OTLP_*`, les bascules de capture de contenu `OTEL_LOG_*`, `OTEL_LOGS_EXPORTER`, et les variables de traçage bêta `ENABLE_BETA_TRACING_DETAILED` et `BETA_TRACING_ENDPOINT` suivent la source la plus élevée qui en définit l'une comme unité. Une source qui livre la clé d'identifiant `otelHeadersHelper` revendique également l'unité, mais ne place ces variables que lorsqu'elle est la source sélectionnée : une source qui n'est pas sélectionnée mais livre la clé ne contribue à aucune d'elles et bloque toujours les sources inférieures de les remplir. De toute façon, un point de terminaison d'exportateur d'une source ne peut jamais être associé à des identifiants d'une autre.
  * **Routage associé à un identifiant** : une source qui associe des variables de routage à une clé d'identifiant sélectionnée uniquement, telle que `apiKeyHelper` ou `otelHeadersHelper`, contribue ces variables de routage uniquement lorsqu'elle remporte l'emplacement.
* **Clés de connexion à la passerelle** : Claude Code ne lit jamais [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/fr/settings-reference#gatewayinternalnetworks), ou la valeur `"gateway"` de [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) à partir des paramètres gérés par le serveur, de sorte qu'une valeur là-bas ne s'applique ni ne masque une définie dans une stratégie MDM ou un fichier de paramètres gérés. L'entrée [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) indique quelle source d'administrateur sur la machine les fournit.

<h3 id="fetch-and-caching-behavior">
  Comportement de récupération et de mise en cache
</h3>

Claude Code récupère les paramètres à partir des serveurs d'Anthropic au démarrage et interroge les mises à jour toutes les heures pendant les sessions actives.

Un client connecté via une [passerelle d'applications Claude](#platform-availability) récupère ses paramètres à partir de la passerelle et attend cette récupération avant le démarrage de la session, donc la récupération dans les listes ci-dessous ne s'applique pas à lui. [Appliquer un démarrage fermé par défaut](#enforce-fail-closed-startup) couvre ce qui se passe lorsque cette récupération échoue.

**Premier lancement sans paramètres en cache :**

* Lorsqu'un développeur se connecte au démarrage, par exemple lors d'une première exécution ou après `/logout`, Claude Code attend jusqu'à cinq secondes la récupération avant d'ouvrir la session. Lorsque la stratégie arrive à temps, Claude Code l'applique à partir du premier écran et affiche vos [`companyAnnouncements`](/docs/fr/settings-reference#companyannouncements) dessus. Lorsque la charge utile nécessite une [approbation de sécurité](#security-approval-dialogs), Claude Code termine l'attente et applique la charge utile une fois que le développeur l'approuve
* À tout autre démarrage, et lorsque cette attente de cinq secondes s'écoule, Claude Code ouvre la session tandis que la récupération continue, de sorte qu'une brève fenêtre passe avant le chargement des paramètres et l'entrée en vigueur des restrictions
* Si la récupération échoue, Claude Code continue sans paramètres gérés par le serveur et avertit dans les sessions interactives qu'aucune stratégie distante ne s'applique ; les paramètres gérés par le point de terminaison s'appliquent toujours. Si une source gérée définit [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup), Claude Code se ferme à la place

**Lancements ultérieurs avec paramètres en cache :**

* Les paramètres en cache s'appliquent immédiatement au démarrage, sauf pour les valeurs en cache `modelPricing` et `managedMcpServers` et les variables d'environnement que Claude Code retient jusqu'à ce que le serveur confirme la charge utile
* Un [`modelPricing`](/docs/fr/settings-reference#modelpricing) en cache ne s'applique pas jusqu'à ce que la récupération de la session confirme la charge utile. Jusqu'à ce moment, les chiffres de coût que les développeurs voient dans `/usage` et la ligne d'état sont au prix catalogue
* Un bloc [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers) en cache ne s'applique pas jusqu'à ce que la récupération de la session confirme la charge utile. Claude Code attend jusqu'à 30 secondes cette récupération avant de connecter les serveurs MCP. Si la récupération échoue ou expire, la session démarre sans les serveurs de l'organisation, `/status` l'indique, et ils se connectent une fois qu'une récupération ultérieure les confirme. Voir [Quand les serveurs fournis se connectent](/docs/fr/managed-mcp#when-provided-servers-connect) pour le comportement complet, y compris le premier lancement. Nécessite Claude Code v2.1.259 ou version ultérieure
* Claude Code récupère les paramètres actualisés en arrière-plan
* Les paramètres en cache persistent en cas de défaillance réseau. Si la récupération au démarrage échoue, Claude Code avertit dans les sessions interactives que la stratégie en cache est en vigueur
* Jusqu'à ce qu'une récupération réussisse, les valeurs retenues au démarrage restent retenues

Claude Code retient plusieurs catégories de variables dans le bloc `env` en cache jusqu'à ce que le serveur confirme la charge utile pour la session. Cela empêche une valeur de proxy, d'autorité de certification, de point de terminaison ou d'identifiant en cache de rediriger, d'intercepter ou de réauthentifier la récupération des paramètres qui confirme la charge utile. Le renforcement s'applique uniquement au cache des paramètres récupérés par le serveur : les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) déployés via MDM ou `managed-settings.json` ne sont pas affectés. La rétention nécessite Claude Code v2.1.198 ou version ultérieure ; avant v2.1.198, le bloc `env` en cache entier s'applique au démarrage. Les catégories retenues incluent :

* Configuration du proxy et TLS, telle que `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS`, et les variables de certificat client mTLS `CLAUDE_CODE_CLIENT_CERT` et `CLAUDE_CODE_CLIENT_KEY`
* Routage d'API et sélection de fournisseur, y compris `ANTHROPIC_BASE_URL`, les variables de sélection de fournisseur telles que `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_VERTEX`, et les URL de point de terminaison du fournisseur telles que `ANTHROPIC_BEDROCK_BASE_URL`
* Identifiants d'authentification, tels que `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, et `CLAUDE_CODE_OAUTH_TOKEN`
* Le sélecteur de répertoire de configuration `CLAUDE_CONFIG_DIR`
* Sélecteurs de source d'identifiant et de répertoire de configuration, dans Claude Code v2.1.223 ou version ultérieure : les variables de Workload Identity Federation telles que `ANTHROPIC_FEDERATION_RULE_ID` et `ANTHROPIC_IDENTITY_TOKEN`, les sélecteurs de profil et de répertoire de configuration `ANTHROPIC_PROFILE` et `ANTHROPIC_CONFIG_DIR`, et les variables de répertoire du système d'exploitation `HOME`, `XDG_CONFIG_HOME`, `APPDATA`, et `USERPROFILE`

Claude Code lit les variables de Workload Identity Federation et les sélecteurs `ANTHROPIC_PROFILE` et `ANTHROPIC_CONFIG_DIR` uniquement au démarrage, de sorte qu'une valeur livrée par le serveur pour eux ne bascule pas la source d'identifiant de la session même après la réussite de la récupération. Pour livrer ces sélecteurs sur Claude Code v2.1.223 ou version ultérieure, utilisez les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) tels que MDM ou `managed-settings.json`. Pour `CLAUDE_CONFIG_DIR` et les variables de répertoire du système d'exploitation, la rétention elle-même est la protection : la valeur en cache reste hors de l'environnement jusqu'à ce que le serveur confirme la charge utile.

Toute autre clé du bloc `env` en cache s'applique au démarrage. Une fois que le serveur confirme la charge utile, et que vous l'approuvez si elle nécessite une [approbation de sécurité](#security-approval-dialogs), les variables retenues s'appliquent pour le reste de la session.

Si votre organisation a besoin d'un proxy pour atteindre `api.anthropic.com`, la rétention affecte uniquement le bloc `env` livré par le serveur lui-même : un proxy défini dans un bloc `env` [géré par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) via MDM ou `managed-settings.json`, dans l'environnement shell, ou dans les [paramètres utilisateur](/docs/fr/settings#where-settings-live) atteint la récupération des paramètres. La source gérée par le point de terminaison nécessite Claude Code v2.1.223 ou version ultérieure : la valeur de proxy gérée par le serveur en cache est retenue jusqu'à ce que la récupération la confirme, de sorte que la valeur gérée par le point de terminaison remplit par clé et atteint la récupération elle-même. Avant v2.1.223, utilisez l'environnement shell ou les paramètres utilisateur afin que le proxy s'applique aux côtés d'une charge utile de serveur en cache. Le premier lancement n'a pas de cache, donc une source gérée par le point de terminaison, l'environnement shell, ou les paramètres utilisateur est toujours requis pour la récupération initiale.

Claude Code applique la plupart des mises à jour des paramètres aux sessions en cours sans redémarrage. Certaines mises à jour s'appliquent uniquement au prochain lancement, y compris la configuration de l'exportateur OpenTelemetry, la clé `model`, et la suppression d'une variable du bloc `env`.

<h3 id="invalid-entries-in-delivered-settings">
  Entrées invalides dans les paramètres livrés
</h3>

Lorsqu'une partie d'une charge utile échoue la validation du schéma, Claude Code affiche une erreur de validation et applique tous les paramètres valides restants ; [Entrées invalides dans les paramètres gérés](/docs/fr/managed-settings#invalid-entries-in-managed-settings) indique ce qu'il supprime et quelles clés reviennent à une valeur plus stricte. Nécessite Claude Code v2.1.169 ou version ultérieure.

La livraison gérée par le serveur ajoute ces comportements :

* Le cache à `~/.claude/remote-settings.json` stocke la charge utile sauvegardée avec les entrées invalides supprimées, à l'exception des valeurs invalides `cleanupPeriodDays` et `desktopSessionCleanupPeriodDays`, qui restent dans la copie en cache et ne sont jamais appliquées.
* Lorsqu'aucun champ de la charge utile ne peut être sauvegardé et que la charge utile n'est pas uniquement ces clés de rétention, Claude Code rejette la charge utile, conserve les derniers paramètres en cache acceptés, et écrit `Remote settings: Settings validation failed - no fields could be salvaged` dans le journal de débogage. Avec `forceRemoteSettingsRefresh` défini, l'interface de ligne de commande se ferme à la place.
* La [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs) évalue la charge utile sauvegardée, de sorte qu'une entrée invalide supprimée n'est jamais présentée pour approbation et n'exécute jamais.

Pour déboguer les problèmes de livraison, exécutez `claude --debug-file <path>` et recherchez `Remote settings` dans le journal. Validez un changement de charge utile avec `claude doctor` sur une machine de test avant de le déployer dans l'organisation.

<h3 id="enforce-fail-closed-startup">
  Appliquer un démarrage fermé par défaut
</h3>

Par défaut, si la récupération des paramètres distants échoue au démarrage, l'interface de ligne de commande continue avec les paramètres en cache à partir de la dernière récupération réussie, à l'exception des [valeurs que Claude Code retient](#fetch-and-caching-behavior) jusqu'à ce qu'une récupération réussisse. Sur une machine qui ne les a jamais récupérés, l'interface de ligne de commande continue sans paramètres gérés par le serveur et applique toujours tous les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) sur l'appareil.

Pour empêcher les clients de démarrer sur des paramètres gérés par le serveur en cache ou absents, définissez `forceRemoteSettingsRefresh: true` dans vos paramètres gérés.

Les clients connectés via une [passerelle d'applications Claude](#platform-availability) attendent la récupération au démarrage que vous définissiez ce paramètre ou non, et gèrent un échec de récupération comme suit :

* Si la passerelle répond à un lancement interactif assisté avec un `401` et que ce paramètre est désactivé, la passerelle a terminé cette connexion. Claude Code imprime [`Cloud gateway session expired — run /login to reconnect.`](/docs/fr/errors#cloud-gateway-session-expired) et ouvre la session déconnectée de la passerelle jusqu'à ce que l'utilisateur exécute `/login`.
* Lorsque la récupération échoue de toute autre manière, ou dans tout autre type de lancement sauf une sous-commande `claude auth`, le client se ferme avec une erreur.

Lorsque ce paramètre est actif dans une session qui récupère les paramètres gérés par le serveur, l'interface de ligne de commande se bloque au démarrage jusqu'à ce que les paramètres distants soient récupérés à nouveau. Si la récupération échoue, l'interface de ligne de commande se ferme plutôt que de continuer sans la stratégie. Ce paramètre s'auto-perpétue : une fois livré par le serveur, il est également mis en cache localement afin que les démarrages ultérieurs appliquent le même comportement même avant la première récupération réussie d'une nouvelle session. Une session qui [ne récupère pas les paramètres gérés par le serveur](#platform-availability) démarre sans attendre.

Pour activer cela, ajoutez la clé à votre configuration de paramètres gérés :

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Vous pouvez également définir cette clé dans un [profil MDM géré par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) ou un fichier `managed-settings.json` système pour appliquer un comportement fermé par défaut au premier lancement, avant la livraison de toute charge utile du serveur. Cet indicateur est une exception à la [règle de précédence](#settings-precedence) ci-dessus : Claude Code l'honore lorsqu'il est défini dans n'importe quelle source gérée contrôlée par un administrateur même si une charge utile en cache gérée par le serveur est également présente, de sorte qu'une valeur livrée par MDM n'est pas ignorée lorsque des paramètres gérés par le serveur existent.

Lorsqu'un [`policyHelper`](/docs/fr/settings-reference#policyhelper) fournit des paramètres gérés, sa sortie remplace toute autre source gérée pour les clés que Claude Code lit après le démarrage. Pour les sources à partir desquelles Claude Code lit cette clé, voir [son entrée de paramètres](/docs/fr/settings-reference#forceremotesettingsrefresh). L'entrée `policyHelper` indique quelles sources Claude Code lit le helper et quand il s'exécute.

La récupération des paramètres envoie également un en-tête `Cache-Control: no-cache` afin que les proxies HTTP intermédiaires ne servent pas une réponse obsolète.

Avant d'activer ce paramètre, assurez-vous que vos stratégies réseau permettent la connectivité à `api.anthropic.com`. Si ce point de terminaison est inaccessible, l'interface de ligne de commande se ferme au démarrage et les utilisateurs ne peuvent pas démarrer Claude Code.

Les sous-commandes `claude auth` telles que `claude auth login` sont exemptes de cette vérification et de la sortie de démarrage de la passerelle, afin que les utilisateurs puissent se réauthentifier lorsque des identifiants expirés sont la raison de l'échec de la récupération des paramètres.

<h3 id="security-approval-dialogs">
  Boîtes de dialogue d'approbation de sécurité
</h3>

Certains paramètres qui pourraient présenter des risques de sécurité nécessitent une approbation explicite de l'utilisateur avant que Claude Code les applique dans une session interactive :

* **Paramètres de commande shell** : paramètres qui exécutent des commandes shell, tels que `apiKeyHelper`, `statusLine`, et `otelHeadersHelper`
* **Paramètres binaires du sandbox** : `sandbox.bwrapPath`, `sandbox.socatPath`, et `sandbox.ripgrep`. Chacun de ces paramètres pointe vers un exécutable, et Claude Code exécute cet exécutable
* **Paramètres de réseau et d'isolation du sandbox** : paramètres de [sandbox](/docs/fr/sandboxing) qui permettent au proxy du sandbox de lire, de réacheminer ou d'authentifier le trafic, ou qui affaiblissent l'isolation du sandbox : `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets`, et `sandbox.network.allowMachLookup`. Un bloc `sandbox.credentials` qui contient uniquement des règles `deny` n'a pas besoin d'approbation, car il restreint le sandbox sans donner au proxy un identifiant. Avant v2.1.251, Claude Code appliquait ces paramètres sans approbation
* **Variables d'environnement personnalisées** : variables `env` livrées qui nécessitent l'approbation de l'utilisateur, telles que les variables de proxy et d'URL de base ; voir [Variables d'environnement et la boîte de dialogue d'approbation](#environment-variables-and-the-approval-dialog)
* **Configurations de hook** : toute définition de hook

Lorsque ces paramètres sont présents, les utilisateurs voient une boîte de dialogue de sécurité expliquant ce qui est configuré. Les utilisateurs doivent approuver pour continuer. Si un utilisateur rejette les paramètres, Claude Code se ferme.

Un CLAUDE.md géré livré via la clé [`claudeMd`](/docs/fr/settings-reference#claudemd) n'a pas besoin d'approbation, car c'est du texte d'instruction pour Claude plutôt qu'une commande que Claude Code exécute. Claude Code vérifie toujours les [permissions](/docs/fr/permissions) pour les outils que Claude utilise en suivant ces instructions. Avant v2.1.260, une valeur `claudeMd` nécessitait une approbation aussi.

<h4 id="approval-memory">
  Mémoire d'approbation
</h4>

Claude Code enregistre votre approbation dans votre répertoire de configuration, `~/.claude` sauf si vous définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars). Ce qu'il enregistre dépend de l'identifiant que la récupération des paramètres utilise :

* **Une connexion claude.ai enregistrée par `/login` ou `claude auth login`, ou la [connexion Console sans clé](/docs/fr/authentication#sign-in-without-an-api-key)** : une approbation par organisation, détenue par le compte qui a approuvé le plus récemment.
* **Une connexion [passerelle d'applications Claude](/docs/fr/claude-apps-gateway)** : une approbation par passerelle.

  Si vous vous déconnectez et vous reconnectez à la même passerelle, Claude Code n'affiche pas la boîte de dialogue à nouveau tandis que les paramètres qui nécessitent une approbation sont inchangés. Claude Code l'affiche à nouveau lorsque ces paramètres changent, lorsque vous vous connectez à une passerelle différente, et lorsque vous acceptez un nouveau certificat pour la même passerelle.

  Claude Code n'enregistre aucune approbation pour une passerelle de développement de boucle locale atteinte via HTTP simple, de sorte que la boîte de dialogue apparaît à nouveau après chaque connexion.
* **Tout autre identifiant**, tel qu'une clé API ou `CLAUDE_CODE_OAUTH_TOKEN` : une approbation pour les paramètres livrés, conservée avec la copie en cache des paramètres dans ce répertoire de configuration. Claude Code affiche la boîte de dialogue à nouveau lorsque les paramètres qui nécessitent une approbation changent, et après avoir exécuté `/logout` ou `claude auth logout`, l'un ou l'autre supprimant la copie en cache.

Une approbation pour `sandbox.credentials` ou `sandbox.network.tlsTerminate` couvre également les entrées [`sandbox.network.allowedDomains`](/docs/fr/settings-reference#sandbox-network-alloweddomains) dans ces mêmes paramètres livrés, car les deux paramètres agissent sur cette liste d'autorisation. La boîte de dialogue apparaît à nouveau lorsque votre administrateur ajoute ou supprime l'une de ces entrées, même si `sandbox.network.allowedDomains` ne nécessite pas d'approbation en soi.

Avec une connexion claude.ai enregistrée :

* Si vous vous déconnectez et vous reconnectez, ou basculez vers une autre organisation et y revenez plus tard, Claude Code n'affiche pas la boîte de dialogue à nouveau tandis que ces paramètres sont inchangés, sauf si un autre compte les a approuvés pour cette organisation dans le même répertoire de configuration entre-temps.
* Si vous vous connectez à la même organisation avec un compte différent, Claude Code affiche la boîte de dialogue à nouveau même lorsque les paramètres sont inchangés. L'approbation de ce compte remplace la précédente, de sorte que lorsque vous revenez, Claude Code affiche la boîte de dialogue une fois de plus.

Claude Code ne peut pas toujours afficher la boîte de dialogue. Chaque cas ci-dessous indique quels paramètres s'appliquent lorsqu'il ne peut pas et quand vous verrez ensuite la boîte de dialogue :

* **Une session interactive qui ne peut pas afficher la boîte de dialogue** : Claude Code n'applique pas les paramètres livrés et conserve les derniers paramètres approuvés. La boîte de dialogue apparaît dans la prochaine session qui peut l'afficher. Nécessite Claude Code v2.1.211 ou version ultérieure.
* **`claude install` ou `claude update`** : Claude Code n'affiche pas la boîte de dialogue pendant l'une ou l'autre commande. La commande s'exécute avec les derniers paramètres approuvés, et la boîte de dialogue apparaît dans votre prochaine session interactive. Si Claude Code attend la récupération des paramètres au démarrage, par exemple avec [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) défini ou sur un déploiement [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), il affiche la boîte de dialogue pendant la commande à la place, et une exécution d'installation à partir d'un tuyau échoue ; voir [`Raw mode is not supported` during install](/docs/fr/troubleshoot-install#raw-mode-is-not-supported-during-install). Avant v2.1.246, Claude Code essayait d'afficher la boîte de dialogue pendant ces commandes aussi.
* **Une erreur ferme la boîte de dialogue avant que vous répondiez** : Claude Code n'applique pas les paramètres livrés et conserve les derniers paramètres approuvés. Il affiche la boîte de dialogue à nouveau dans la prochaine session qui peut l'afficher.
* **Une exécution non interactive**, telle que `claude -p` ou une session Agent SDK : Claude Code ne peut pas afficher la boîte de dialogue, de sorte que lorsque les paramètres livrés nécessiteraient une approbation, il les applique pour cette exécution uniquement. Il ne les enregistre pas comme approuvés ou ne les écrit pas dans le [cache local](#fetch-and-caching-behavior), et la prochaine session interactive affiche la boîte de dialogue. Jusqu'à ce qu'un utilisateur approuve dans une session interactive, chaque exécution non interactive récupère les paramètres à nouveau au démarrage. Avant v2.1.207, une exécution non interactive enregistrait les paramètres comme approuvés, de sorte que les sessions interactives ultérieures n'affichaient jamais la boîte de dialogue pour eux.

<h4 id="environment-variables-and-the-approval-dialog">
  Variables d'environnement et la boîte de dialogue d'approbation
</h4>

Claude Code applique certaines variables `env` livrées sans afficher à l'utilisateur la boîte de dialogue d'approbation, y compris :

* Bascules de fonctionnalité et de commande
* Paramètres de sélection et de comportement du modèle, tels que `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING`, et `CLAUDE_CODE_EFFORT_LEVEL`
* Paramètres de fenêtre de contexte et de compaction, tels que `DISABLE_AUTO_COMPACT`
* Options d'interface utilisateur de terminal et d'accessibilité
* Limites numériques, budgets et délais d'expiration

D'autres variables livrées peuvent nécessiter l'approbation de l'utilisateur avant de prendre effet ; une valeur de proxy, d'URL de base, ou `OTEL_EXPORTER_OTLP_ENDPOINT` non vide le fait toujours. Lorsqu'une variable livrée a besoin d'approbation, la boîte de dialogue la nomme, de sorte que l'utilisateur voit exactement ce que la stratégie demande de définir. Avant v2.1.218, Claude Code appliquait moins de variables sans demander à l'utilisateur, de sorte que des paramètres tels que `DISABLE_AUTO_COMPACT` déclenchaient la boîte de dialogue à n'importe quelle valeur non vide.

Claude Code décide si quatre bascules de confidentialité ont besoin d'approbation par la valeur livrée plutôt que par le nom de la variable : `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY`, et `DO_NOT_TRACK`. Une valeur véridique telle que `1` ou `true` désactive uniquement le suivi, la création de rapports ou autre trafic non essentiel, de sorte que Claude Code l'applique sans demander à l'utilisateur. Pour toute autre valeur non vide, Claude Code affiche la boîte de dialogue. Avant v2.1.218, tous sauf `DO_NOT_TRACK` s'appliquaient sans approbation à n'importe quelle valeur, et `DO_NOT_TRACK` déclenchait la boîte de dialogue à n'importe quelle valeur non vide.

Claude Code décide également si [`API_FORCE_IDLE_TIMEOUT`](/docs/fr/env-vars) a besoin d'approbation par la valeur livrée : une valeur véridique active uniquement le [délai d'expiration d'inactivité du corps](/docs/fr/network-config#streaming-idle-watchdogs), de sorte que Claude Code l'applique sans demander à l'utilisateur. Pour toute autre valeur non vide, Claude Code affiche la boîte de dialogue. Avant v2.1.248, n'importe quelle valeur non vide déclenchait la boîte de dialogue.

Si [`ANTHROPIC_CUSTOM_HEADERS`](/docs/fr/env-vars#variables) a besoin d'approbation dépend également de la valeur livrée. Les en-têtes qui marquent uniquement les demandes, tels que `Accept-Language`, s'appliquent sans la boîte de dialogue. Une ligne qui nomme un identifiant, un sélecteur d'organisation ou de locataire, un routage ou un remplacement d'hôte, ou un en-tête de comportement d'API, tels que `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta`, ou les en-têtes `X-Amzn-Bedrock-*`, nécessite une approbation. Il en va de même pour une ligne dont le nom n'est pas un jeton d'en-tête HTTP valide, ou dont la valeur contient un caractère qu'un en-tête HTTP ne peut pas porter. La vérification correspond aux mots à l'intérieur du nom d'en-tête, de sorte que `X-Client-Version`, qui contient `client` et `version`, nécessite également une approbation. Avant v2.1.251, n'importe quelle valeur `ANTHROPIC_CUSTOM_HEADERS` s'appliquait sans elle.

Une valeur falsy telle que `0` ou `false` pour [`ENABLE_BETA_TRACING_DETAILED`](/docs/fr/env-vars#variables) ou [`OTEL_LOG_RAW_API_BODIES`](/docs/fr/env-vars#variables) s'applique sans la boîte de dialogue, car elle désactive uniquement le traçage détaillé ou la capture du corps API brut. Toute autre valeur non vide pour l'une ou l'autre variable nécessite une approbation.

<h2 id="platform-availability">
  Disponibilité de la plateforme
</h2>

Les paramètres gérés par le serveur nécessitent une connexion directe à `api.anthropic.com`. La livraison nécessite également que la session s'authentifie avec l'une de ces informations d'identification :

* Une connexion OAuth d'équipe ou d'entreprise
* Un jeton OAuth fourni via `CLAUDE_CODE_OAUTH_TOKEN`
* Une clé API directement configurée
* Un profil `user_oauth` [Anthropic](/docs/fr/authentication#anthropic-profiles-and-federation-credentials), sauf si le profil définit une `base_url` autre que l'API Anthropic. Nécessite Claude Code v2.1.257 ou version ultérieure.

Ni les clés renvoyées par un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) ni les informations d'identification de [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) ne déclenchent la récupération des paramètres.

Dans une session [Cowork](https://claude.com/docs/cowork/overview) dans l'application Claude Desktop, Claude Code ne récupère pas les paramètres gérés par le serveur à partir de la console d'administration claude.ai, même lorsque l'utilisateur se connecte avec un compte d'équipe ou d'entreprise. [Où et quand une politique s'applique](/docs/fr/managed-settings#where-and-when-a-policy-applies) couvre quelle politique atteint les sessions Cowork sur la machine de l'utilisateur et les sessions Cowork distantes. claude.ai applique toujours vos listes [`strictKnownMarketplaces`](/docs/fr/settings-reference#strictknownmarketplaces) et [`blockedMarketplaces`](/docs/fr/settings-reference#blockedmarketplaces) lorsqu'un utilisateur Cowork ajoute une marketplace à partir d'un référentiel git sur claude.ai ou à partir de **Personnaliser** dans l'onglet Cowork. [Comment fonctionnent les restrictions](/docs/fr/plugins/org#restrict-what-users-can-install) décrit cette vérification.

Si vous exportez une variable de fournisseur `CLAUDE_CODE_USE_*` ou une `ANTHROPIC_BASE_URL` non définie par défaut dans votre shell, Claude Code ignore la récupération des paramètres pour vos sessions. [`claude doctor` et `/status` signalent la récupération ignorée et sa cause](#verify-settings-delivery).

Vous ne pouvez pas effacer l'export avec un bloc `env` géré par le serveur, car le bloc arrive via la récupération que l'export empêche. Un bloc `env` [géré par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) ne restaure pas non plus la récupération : Claude Code vérifie l'éligibilité avant d'appliquer les blocs `env` gérés, donc la valeur gérée par le point de terminaison change la sélection du fournisseur de la session mais la récupération reste ignorée.

Pour restaurer la livraison gérée par le serveur, supprimez l'export de votre shell, ou définissez la variable sur `""` dans votre bloc `env` des paramètres utilisateur, qui s'applique avant la vérification d'éligibilité. Pour appliquer la politique sans dépendre des utilisateurs pour modifier leurs shells, livrez les paramètres via le canal géré par le point de terminaison à la place.

Pour les déploiements Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée fournit la livraison équivalente des paramètres gérés à distance : les clients signés à la passerelle récupèrent les paramètres gérés auprès de la passerelle au lieu de `api.anthropic.com`. La sémantique des défaillances diffère au démarrage : un client de passerelle qui ne peut pas atteindre la passerelle se termine avec une erreur au lieu de revenir aux paramètres en cache, tandis que l'actualisation en arrière-plan horaire est fail-open sur les deux canaux.

<h2 id="audit-logging">
  Journalisation d'audit
</h2>

Les événements du journal d'audit pour les modifications de paramètres sont disponibles via l'API de conformité ou l'export du journal d'audit. Contactez votre équipe de compte Anthropic pour accéder.

Les événements d'audit incluent le type d'action effectuée, le compte et l'appareil qui ont effectué l'action, et les références aux valeurs précédentes et nouvelles.

<h2 id="security-considerations">
  Considérations de sécurité
</h2>

Les paramètres gérés par le serveur fournissent une application de stratégie centralisée, mais ils fonctionnent comme un contrôle côté client, pas comme une limite de sécurité. Sur les appareils non gérés, un utilisateur n'a pas besoin d'un accès administrateur ou sudo pour les contourner.

| Scénario                                                                         | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L'utilisateur modifie le fichier de paramètres en cache                          | Le fichier falsifié s'applique au démarrage, sauf pour les [valeurs que Claude Code retient](#fetch-and-caching-behavior) jusqu'à ce que le serveur confirme la charge utile. La prochaine récupération du serveur restaure les paramètres corrects, sauf pour les [clés qui s'appliquent uniquement au prochain lancement](#fetch-and-caching-behavior), telles que `model` ou une variable ajoutée au bloc `env`, qui restent en vigueur jusqu'au redémarrage                                                                                                                                                                                                                                                                                                                                      |
| L'utilisateur supprime le fichier de paramètres en cache                         | Le [comportement du premier lancement](#fetch-and-caching-behavior) se produit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| L'utilisateur exécute un binaire Claude Code modifié                             | Un utilisateur qui peut exécuter un client modifié peut contourner n'importe quel contrôle côté client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| L'utilisateur exécute une version antérieure de Claude Code                      | Les versions antérieures aux paramètres gérés par le serveur ne les récupèrent ni ne les appliquent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| L'API est indisponible                                                           | Les paramètres en cache s'appliquent s'ils sont disponibles, sauf pour les [valeurs que Claude Code retient](#fetch-and-caching-behavior) jusqu'à ce qu'une récupération réussisse. Sans cache, Claude Code n'applique aucun paramètre géré par le serveur jusqu'à la prochaine récupération réussie et applique toujours les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) sur l'appareil. Avec `forceRemoteSettingsRefresh: true`, l'interface de ligne de commande se ferme au lieu de continuer, sauf pour les [sous-commandes `claude auth`](#enforce-fail-closed-startup). Les clients connectés via une [passerelle d'applications Claude](#platform-availability) se ferment au démarrage sans ce paramètre, avec la même exemption `claude auth` |
| L'utilisateur s'authentifie avec une organisation différente                     | Les paramètres ne sont pas livrés pour les comptes en dehors de l'organisation gérée                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| L'utilisateur configure un [fournisseur de modèle tiers](#platform-availability) | Les paramètres gérés par le serveur sont contournés. Cela inclut la définition de `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, ou un `ANTHROPIC_BASE_URL` non défini par défaut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Le trafic réseau est intercepté ou redirigé                                      | La validation TLS désactivée ou le trafic intercepté peut modifier les paramètres que le client reçoit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

Pour enregistrer les modifications des fichiers de paramètres locaux, y compris `managed-settings.json`, utilisez les [hooks `ConfigChange`](/docs/fr/hooks#configchange). Claude Code ne les exécute pas lorsque les paramètres gérés par le serveur arrivent ou se rafraîchissent, ou lorsqu'un profil MDM ou une stratégie de registre change, et un hook ne peut pas bloquer une modification `policy_settings`.

Pour restreindre les organisations auxquelles vos utilisateurs peuvent accéder avec les identifiants que le client fournit, consultez [Appliquer le contrôle d'accès au niveau du réseau avec les restrictions de locataire](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) dans le Centre d'aide Claude. Pour des garanties d'application plus fortes, utilisez les [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) sur les appareils inscrits dans une solution MDM.

<h2 id="see-also">
  Voir aussi
</h2>

Pages connexes pour gérer la configuration de Claude Code :

* [Tous les paramètres](/docs/fr/settings-reference) : chaque clé de paramètres
* [Paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) : paramètres gérés déployés sur les appareils par l'informatique
* [Authentification](/docs/fr/authentication) : configurer l'accès des utilisateurs à Claude Code
* [Sécurité](/docs/fr/security) : garanties de sécurité et meilleures pratiques
