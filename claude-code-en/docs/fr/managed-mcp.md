> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contrôler l'accès aux serveurs MCP pour votre organisation

> Limitez les serveurs MCP que les utilisateurs peuvent ajouter ou connecter avec des fichiers de configuration gérés, des listes blanches et des listes noires.

Par défaut, toute personne exécutant Claude Code peut connecter n'importe quel [serveur MCP](/docs/fr/mcp) de son choix. Anthropic examine les connecteurs par rapport à ses [critères d'examen](https://claude.com/docs/connectors/building/review-criteria) avant de les ajouter à l'[Annuaire Anthropic](https://claude.ai/directory), mais n'effectue pas d'audit de sécurité ni ne gère aucun serveur MCP. En tant qu'administrateur, vous pouvez restreindre les serveurs qui s'exécutent dans votre organisation, du déploiement d'un ensemble approuvé fixe à la désactivation complète de MCP.

Cette page couvre comment :

* [Choisir un modèle](#choose-a-pattern) qui correspond au niveau de contrôle dont vous avez besoin
* [Déployer un ensemble de serveurs fixe avec `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), y compris comment [désactiver MCP entièrement](#disable-mcp-entirely)
* [Contrôler les serveurs avec des listes blanches et des listes noires](#policy-based-control-with-allowlists-and-denylists)
* [Indiquer aux utilisateurs à quoi s'attendre](#how-restrictions-appear-to-users) quand une restriction bloque un serveur
* [Surveiller les serveurs que votre organisation utilise réellement](#monitor-mcp-usage)

<Note>
  La page [Sécurité](/docs/fr/security) couvre le modèle de menace MCP et comment évaluer un serveur avant de l'approuver. [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce) couvre les restrictions MCP aux côtés des autres contrôles administratifs.
</Note>

<h2 id="choose-a-pattern">
  Choisir un modèle
</h2>

Claude Code prend en charge une gamme de niveaux de restriction. Chaque modèle utilise l'un ou les deux mécanismes couverts ci-dessous : `managed-mcp.json` pour déployer un ensemble fixe, et `allowedMcpServers`/`deniedMcpServers` pour filtrer ce que les utilisateurs configurent.

| Modèle                             | Ce qu'il fait                                                                                                     | Configurer                                                                                             |
| :--------------------------------- | :---------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| **Désactiver MCP**                 | Aucun serveur ne se charge nulle part                                                                             | `managed-mcp.json` avec une carte de serveurs vide                                                     |
| **Déploiement fixe**               | Chaque utilisateur obtient les mêmes serveurs et ne peut pas en ajouter d'autres                                  | `managed-mcp.json` avec les serveurs que vous voulez                                                   |
| **Catalogue approuvé**             | Publiez une liste de serveurs approuvés ; les utilisateurs ajoutent ceux qu'ils veulent, tout le reste est bloqué | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                               |
| **Serveurs de plugins uniquement** | Les serveurs ne peuvent provenir que de plugins ; les utilisateurs ne peuvent pas ajouter les leurs               | [`strictPluginOnlyCustomization`](/docs/fr/settings#strictpluginonlycustomization) avec `mcp` dans la liste |
| **Liste blanche souple**           | Appliquer une liste blanche que les utilisateurs peuvent élargir dans leurs propres paramètres                    | `allowedMcpServers` sans `allowManagedMcpServersOnly`                                                  |
| **Liste noire uniquement**         | Bloquer les serveurs connus comme mauvais, autoriser tout le reste                                                | `deniedMcpServers`                                                                                     |
| **Aucune restriction**             | Les utilisateurs ajoutent n'importe quoi                                                                          | Ne déployez aucune configuration MCP gérée                                                             |

<Note>
  Claude Code n'a pas de registre de serveurs MCP intégré que les utilisateurs peuvent parcourir et installer. Pour le modèle de catalogue approuvé, partagez la liste approuvée et ses commandes `claude mcp add` quelque part où vos utilisateurs les trouveront, comme un wiki interne, ou distribuez les serveurs en tant que plugins via une [place de marché de plugins gérée](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions) afin que les utilisateurs puissent les parcourir et les installer à partir de `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Contrôle exclusif avec managed-mcp.json
</h2>

Si vous déployez un fichier `managed-mcp.json`, Claude Code charge uniquement les serveurs que ce fichier définit. Les utilisateurs ne peuvent pas ajouter, modifier ou utiliser d'autres serveurs MCP, y compris les serveurs fournis par les plugins. Le fichier supprime également les connecteurs claude.ai sauf si vous [les autorisez aux côtés de l'ensemble géré](#allow-claude-ai-connectors-alongside-the-managed-set).

Deux autres paramètres peuvent filtrer davantage l'ensemble géré :

* `allowedMcpServers` et `deniedMcpServers` s'appliquent également aux serveurs gérés, donc un serveur géré qui ne les satisfait pas ne se chargera pas.
* Le propre `deniedMcpServers` d'un utilisateur se fusionne à partir de ses paramètres, donc les utilisateurs peuvent bloquer un serveur géré pour eux-mêmes.

Voir [Comment un serveur est évalué](#how-a-server-is-evaluated) pour l'ordre complet des vérifications.

`managed-mcp.json` est un fichier autonome, il ne peut donc pas être livré via [les paramètres gérés par le serveur](/docs/fr/server-managed-settings). Tout processus pouvant écrire dans un chemin système avec des privilèges d'administrateur peut le déployer. À grande échelle, c'est généralement via des outils de gestion des appareils, tels que Jamf ou un profil de configuration sur macOS, Group Policy ou Intune sur Windows, ou votre gestion de flotte de choix sur Linux. Claude Code recherche le fichier à l'un de ces chemins :

| Plateforme   | Chemin                                                     |
| :----------- | :--------------------------------------------------------- |
| macOS        | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux et WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows      | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

Le fichier utilise le même format qu'un fichier de projet [`.mcp.json`](/docs/fr/mcp#project-scope) :

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  S'authentifier avec des identifiants par utilisateur
</h3>

N'importe quel utilisateur sur la machine peut lire ce fichier, donc ne stockez pas de clés API ou d'autres identifiants dans les blocs `env`. Transmettez plutôt les identifiants par utilisateur avec l'un de ceux-ci :

* [Expansion `${VAR}`](/docs/fr/mcp#environment-variable-expansion-in-mcp-json) pour lire les secrets de l'environnement de chaque utilisateur.
* [OAuth ou en-têtes par utilisateur](/docs/fr/mcp#authenticate-with-remote-mcp-servers) afin que chaque utilisateur s'authentifie en tant que lui-même.
* [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) pour générer des identifiants au moment de la connexion.

<h3 id="validate-the-configuration">
  Valider la configuration
</h3>

Pour confirmer que le fichier est en vigueur, exécutez deux vérifications sur une machine gérée :

1. `claude mcp list` affiche uniquement les serveurs dans `managed-mcp.json`. Si les propres serveurs d'un utilisateur apparaissent toujours, le fichier n'est pas lu ; vérifiez le chemin et les permissions.
2. `claude mcp add --transport http test https://example.com/mcp` échoue avec `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. L'URL n'a pas besoin d'être un vrai serveur, car la vérification de la politique rejette la commande avant que quoi que ce soit ne soit contacté.

<h3 id="disable-mcp-entirely">
  Désactiver MCP entièrement
</h3>

Déployez un `managed-mcp.json` contenant une carte de serveurs vide pour bloquer tous les serveurs MCP :

```json theme={null}
{
  "mcpServers": {}
}
```

Les utilisateurs ne voient aucun serveur MCP dans `/mcp`, et `claude mcp add` échoue avec l'erreur de politique d'entreprise ci-dessus. Les serveurs que les utilisateurs avaient précédemment configurés cessent de se charger la prochaine fois qu'ils démarrent une session, sans avertissement que la politique en est la raison.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Autoriser les connecteurs claude.ai aux côtés de l'ensemble géré
</h3>

Le déploiement de `managed-mcp.json` supprime les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) par défaut, y compris les connecteurs qu'un administrateur a configurés pour l'organisation dans la console d'administration claude.ai. Pour charger ces connecteurs aux côtés des serveurs dans `managed-mcp.json`, définissez `"allowAllClaudeAiMcps": true` dans une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices). Nécessite Claude Code v2.1.149 ou ultérieur.

Avec le paramètre activé, Claude Code charge les mêmes connecteurs claude.ai qu'il chargerait si `managed-mcp.json` n'était pas déployé. Les [listes blanches et listes noires](#policy-based-control-with-allowlists-and-denylists) s'appliquent toujours à ces connecteurs, vous pouvez donc en bloquer des spécifiques avec `deniedMcpServers`. Le paramètre affecte uniquement les connecteurs claude.ai ; les serveurs fournis par les plugins restent supprimés.

Claude Code lit ce paramètre uniquement à partir des niveaux de politique contrôlés par l'administrateur : paramètres gérés par le serveur, une clé plist déployée par MDM ou une clé de registre HKLM, ou un fichier `managed-settings.json` système. Le placer dans les paramètres utilisateur ou projet n'a aucun effet, donc les utilisateurs ne peuvent pas réactiver les connecteurs que le contrôle exclusif a supprimés.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Contrôle basé sur les politiques avec listes blanches et listes noires
</h2>

Les listes blanches et les listes noires filtrent les serveurs configurés autorisés à se charger. Ce ne sont pas un registre : un serveur doit toujours être ajouté par un utilisateur, un plugin ou `managed-mcp.json` avant que la liste blanche ou la liste noire ne s'applique à lui. Pour déployer des serveurs aux utilisateurs, utilisez [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json). Les deux listes filtrent également les serveurs transmis avec l'indicateur CLI [`--mcp-config`](/docs/fr/cli-reference#cli-flags) ; `--strict-mcp-config` limite les fichiers de configuration qui se chargent et ne contourne aucune des deux listes.

Pour rendre la liste blanche faisant autorité, définissez `allowedMcpServers` et `allowManagedMcpServersOnly: true` ensemble dans une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices), telle que les paramètres gérés par le serveur ou un fichier `managed-settings.json` déployé. [Restreindre la liste blanche aux paramètres gérés uniquement](#restrict-the-allowlist-to-managed-settings-only) montre la configuration. Sans `allowManagedMcpServersOnly`, les listes blanches de chaque source de paramètres fusionnent, y compris le propre `~/.claude/settings.json` d'un utilisateur, donc un utilisateur peut élargir ce que votre liste blanche permet. Les listes noires fusionnent de chaque source indépendamment.

<Note>
  `allowManagedMcpServersOnly` est séparé de `allowManagedPermissionRulesOnly`, qui verrouille uniquement les [règles de permission](/docs/fr/permissions#managed-settings). La définition de cet indicateur n'applique pas la liste blanche MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Faire correspondre les serveurs par URL, commande ou nom
</h3>

`allowedMcpServers` et `deniedMcpServers` sont des listes d'entrées. Chaque entrée est un objet avec une seule clé qui identifie les serveurs par leur URL, leur commande ou leur nom :

| Clé             | Correspond à                                                                                                                | Utiliser pour                                              |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------- |
| `serverUrl`     | Une URL de serveur distant, exacte ou avec des caractères génériques `*`                                                    | Serveurs HTTP et SSE                                       |
| `serverCommand` | La commande exacte et les arguments qui démarrent un serveur stdio                                                          | Serveurs stdio                                             |
| `serverName`    | L'étiquette assignée par l'utilisateur. Correspondance exacte uniquement ; les caractères génériques ne sont pas développés | L'un ou l'autre type, mais voir l'Avertissement ci-dessous |

Laisser `allowedMcpServers` non défini est différent de le définir sur un tableau vide :

| Paramètre           | Non défini (par défaut)     | Tableau vide `[]`      | Rempli                                      |
| :------------------ | :-------------------------- | :--------------------- | :------------------------------------------ |
| `allowedMcpServers` | Tous les serveurs autorisés | Aucun serveur autorisé | Seuls les serveurs correspondants autorisés |
| `deniedMcpServers`  | Aucun serveur bloqué        | Aucun serveur bloqué   | Serveurs correspondants bloqués             |

Voir [Entrées invalides dans les paramètres gérés](/docs/fr/settings#invalid-entries-in-managed-settings) pour savoir ce qui se passe quand une entrée échoue la validation du schéma.

<Warning>
  Une entrée `serverName`, dans l'une ou l'autre liste, n'est pas un contrôle de sécurité. Le nom est l'étiquette qu'un utilisateur assigne lors de l'exécution de `claude mcp add` ou de la modification d'un fichier de configuration, pas le serveur sous-jacent, donc un utilisateur peut appeler n'importe quel serveur `github`. Pour les connecteurs claude.ai, le nom est le nom d'affichage renvoyé par claude.ai, qui peut changer. Pour appliquer les serveurs qui s'exécutent réellement, ajoutez des entrées `serverCommand` ou `serverUrl`.
</Warning>

La validation `serverName` diffère entre les deux listes :

* Dans `deniedMcpServers`, `serverName` accepte n'importe quelle chaîne non vide, donc vous pouvez bloquer les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) par leur nom d'affichage. Par exemple, `{ "serverName": "claude.ai Slack" }` bloque le connecteur Slack. Préférez une entrée `serverUrl` quand vous avez besoin que le refus soit robuste aux renommages, ou quand un nom de connecteur entre en collision et gagne un suffixe ` (N)`.
* Dans `allowedMcpServers`, `serverName` est limité aux lettres, chiffres, traits d'union et traits de soulignement. Utilisez `serverUrl` pour ajouter un connecteur claude.ai à la liste blanche.

Pour désactiver tous les connecteurs claude.ai, voir [`disableClaudeAiConnectors`](/docs/fr/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Comment un serveur est évalué
</h3>

Avant de charger un serveur, y compris un serveur de `managed-mcp.json`, Claude Code exécute trois vérifications dans l'ordre :

1. **Fusionner les listes.** Les entrées de liste blanche et de liste noire de chaque source de paramètres se combinent en une liste blanche et une liste noire. Quand `allowManagedMcpServersOnly` est `true`, seule la liste blanche gérée est conservée ; la liste noire fusionne toujours de chaque source.
2. **Vérifier la liste noire.** Un serveur qui correspond à n'importe quelle entrée de liste noire, par URL, commande ou nom, est bloqué. Rien ne remplace une correspondance de liste noire.
3. **Vérifier la liste blanche.** Si `allowedMcpServers` n'est défini nulle part, chaque serveur qui a réussi la liste noire se charge. S'il est défini, ce à quoi le serveur doit correspondre dépend de son type, indiqué dans le tableau ci-dessous.

| Type de serveur       | Autorisé quand il correspond à                                                                                                                 |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Distant (HTTP ou SSE) | Une entrée `serverUrl`. Une correspondance `serverName` compte uniquement quand la liste blanche ne contient aucune entrée `serverUrl`         |
| Stdio                 | Une entrée `serverCommand`. Une correspondance `serverName` compte uniquement quand la liste blanche ne contient aucune entrée `serverCommand` |

Trois règles de correspondance s'appliquent dans ces vérifications :

* **Les commandes correspondent exactement.** Chaque argument, dans l'ordre. `["npx", "-y", "server"]` ne correspond pas à `["npx", "server"]` ou `["npx", "-y", "server", "--flag"]`.
* **Les valeurs `serverCommand` et `serverUrl` se développent avant la correspondance.** La valeur de la politique et la valeur configurée du serveur passent toutes les deux par la même expansion [`${VAR}` et `${VAR:-default}`](/docs/fr/mcp#environment-variable-expansion-in-mcp-json) que `.mcp.json`, donc une entrée écrite comme `["${HOME}/bin/server"]` correspond à une configuration de serveur qui utilise soit la même référence, soit le chemin développé. Sur Windows, référencez une variable d'environnement qui y est définie, telle que `${USERPROFILE}` au lieu de `${HOME}`. Les valeurs `serverName` correspondent littéralement et ne se développent jamais.
* **Les URL supportent les caractères génériques `*`** n'importe où dans le modèle, y compris le schéma. La correspondance du nom d'hôte est insensible à la casse et ignore un point FQDN final, donc `https://Mcp.Example.com/*` correspond à `https://mcp.example.com/api`. Les chemins restent sensibles à la casse.

| Modèle                      | Autorise                                                                                        |
| :-------------------------- | :---------------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Tous les chemins sur un domaine spécifique                                                      |
| `https://mcp.example.com`   | Aussi tous les chemins sur ce domaine. Un modèle sans chemin correspond à n'importe quel chemin |
| `https://*.example.com/*`   | N'importe quel sous-domaine de `example.com`                                                    |
| `http://localhost:*/*`      | N'importe quel port sur localhost                                                               |
| `*://mcp.example.com/*`     | N'importe quel schéma vers un domaine spécifique                                                |

Parce que l'expansion `${VAR}` lit l'environnement de processus propre de Claude Code, une entrée de politique `serverCommand` ou `serverUrl` qui référence une variable se développe à la valeur qu'un utilisateur définit. Utilisez des URL et des commandes littérales pour les entrées sur lesquelles vous comptez pour l'application.

<h3 id="example-configuration">
  Exemple de configuration
</h3>

La configuration ci-dessous configure une liste blanche stricte avec une liste noire. Les lignes en surbrillance changent la façon dont le reste de la liste est évalué, et les légendes après le bloc expliquent chacune :

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Ligne 3** : la première entrée `serverUrl`. Une fois qu'elle existe, chaque serveur distant doit correspondre à un modèle d'URL, donc un utilisateur ne peut pas obtenir un serveur distant non répertorié en lui donnant un nom autorisé.
* **Ligne 5** : la première entrée `serverCommand`. Même effet pour les serveurs stdio, donc chaque serveur local doit correspondre exactement à une commande répertoriée.
* **Ligne 11** : une entrée `serverName` dans la liste noire. Les entrées de liste noire s'appliquent toujours, donc n'importe quel serveur nommé `dangerous-server` est bloqué indépendamment de son URL ou de sa commande.

Une entrée `serverName` dans cette liste blanche ne correspondrait jamais à rien, car les deux types de transport ont déjà des entrées plus strictes.

Les accordéons ci-dessous parcourent la façon dont un serveur est évalué par rapport à d'autres combinaisons de listes blanches et de listes noires.

<Accordion title="Liste blanche URL uniquement">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Serveur                                               | Résultat                                                       |
  | :---------------------------------------------------- | :------------------------------------------------------------- |
  | Serveur HTTP à `https://mcp.example.com/api`          | Autorisé : correspond au modèle d'URL                          |
  | Serveur HTTP à `https://api.internal.example.com/mcp` | Autorisé : correspond au sous-domaine générique                |
  | Serveur HTTP à `https://external.example.com/mcp`     | Bloqué : ne correspond à aucun modèle d'URL                    |
  | Serveur stdio avec n'importe quelle commande          | Bloqué : aucune entrée de nom ou de commande pour correspondre |
</Accordion>

<Accordion title="Liste blanche commande uniquement">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Serveur                                                | Résultat                                        |
  | :----------------------------------------------------- | :---------------------------------------------- |
  | Serveur stdio avec `["npx", "-y", "approved-package"]` | Autorisé : correspond à la commande             |
  | Serveur stdio avec `["node", "server.js"]`             | Bloqué : ne correspond pas à la commande        |
  | Serveur HTTP nommé `my-api`                            | Bloqué : aucune entrée de nom pour correspondre |
</Accordion>

<Accordion title="Liste blanche mixte nom et commande">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Serveur                                                                   | Résultat                                                                                              |
  | :------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------- |
  | Serveur stdio nommé `local-tool` avec `["npx", "-y", "approved-package"]` | Autorisé : correspond à la commande                                                                   |
  | Serveur stdio nommé `local-tool` avec `["node", "server.js"]`             | Bloqué : les entrées de commande existent mais ne correspondent pas                                   |
  | Serveur stdio nommé `github` avec `["node", "server.js"]`                 | Bloqué : les serveurs stdio doivent correspondre aux commandes quand les entrées de commande existent |
  | Serveur HTTP nommé `github`                                               | Autorisé : correspond au nom                                                                          |
  | Serveur HTTP nommé `other-api`                                            | Bloqué : le nom ne correspond pas                                                                     |
</Accordion>

<Accordion title="Liste blanche nom uniquement">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Serveur                                                            | Résultat                                  |
  | :----------------------------------------------------------------- | :---------------------------------------- |
  | Serveur stdio nommé `github` avec n'importe quelle commande        | Autorisé : aucune restriction de commande |
  | Serveur stdio nommé `internal-tool` avec n'importe quelle commande | Autorisé : aucune restriction de commande |
  | Serveur HTTP nommé `github`                                        | Autorisé : correspond au nom              |
  | N'importe quel serveur nommé `other`                               | Bloqué : le nom ne correspond pas         |
</Accordion>

<Accordion title="Liste blanche avec remplacement de liste noire">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Serveur                                          | Résultat                                                                                        |
  | :----------------------------------------------- | :---------------------------------------------------------------------------------------------- |
  | Serveur HTTP à `https://mcp.example.com/api`     | Autorisé : correspond au modèle d'URL de la liste blanche, aucune correspondance de liste noire |
  | Serveur HTTP à `https://staging.example.com/api` | Bloqué : correspond aux deux, mais la liste noire a la priorité                                 |
  | Serveur HTTP à `https://other.com/mcp`           | Bloqué : ne correspond pas à la liste blanche                                                   |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Restreindre la liste blanche aux paramètres gérés uniquement
</h3>

Pour que la liste blanche gérée soit la seule qui s'applique, définissez `allowManagedMcpServersOnly` dans le fichier de paramètres gérés :

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Quand `allowManagedMcpServersOnly` est `true`, les listes blanches des paramètres utilisateur, projet et local sont ignorées. La liste noire fusionne toujours de toutes les sources, donc les utilisateurs peuvent toujours bloquer les serveurs pour eux-mêmes.

<h2 id="how-restrictions-appear-to-users">
  Comment les restrictions apparaissent aux utilisateurs
</h2>

Quand une restriction bloque un serveur, l'utilisateur voit soit une erreur de `claude mcp add`, soit le serveur cesse silencieusement de se charger. Utilisez ce tableau pour reconnaître ces rapports et pour indiquer aux utilisateurs à quoi s'attendre avant de déployer une modification :

| Restriction                                                                         | Ce que l'utilisateur voit                                                                                  |
| :---------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` est présent et l'utilisateur exécute `claude mcp add`            | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| Le serveur est sur une liste noire et l'utilisateur exécute `claude mcp add`        | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| Le serveur n'est pas sur la liste blanche et l'utilisateur exécute `claude mcp add` | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| Un serveur précédemment configuré est maintenant bloqué par la politique            | Le serveur disparaît silencieusement de `/mcp` et `claude mcp list` sans avertissement                     |

Dans le dernier cas, l'utilisateur n'obtient aucun signal que la politique est la raison pour laquelle son serveur a disparu, donc informez les utilisateurs affectés des serveurs bloqués quand vous déployez une nouvelle restriction.

<h2 id="monitor-mcp-usage">
  Surveiller l'utilisation de MCP
</h2>

Quand [l'export OpenTelemetry](/docs/fr/monitoring-usage) est configuré, Claude Code peut enregistrer les serveurs MCP et les outils que les utilisateurs invoquent. Définissez `OTEL_LOG_TOOL_DETAILS=1` pour inclure les noms de serveur MCP et d'outils dans les événements d'outils, puis agrégez-les dans votre collecteur pour voir les serveurs auxquels vos utilisateurs se connectent réellement. Voir [Surveillance](/docs/fr/monitoring-usage) pour configurer l'exportateur et pour le schéma d'événement complet.

<h2 id="configuration-summary">
  Résumé de la configuration
</h2>

Chaque fichier et paramètre que cette page couvre, ce qu'il contrôle et comment le livrer :

| Surface                      | Ce qu'il contrôle                                                                         | Où il se trouve                                                                                                                                               | Comment le livrer                                                                                                                                                                           |
| :--------------------------- | :---------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `managed-mcp.json`           | Ensemble de serveurs fixe, contrôle exclusif                                              | Chemin système : `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, ou `C:\Program Files\ClaudeCode\`                                           | MDM, GPO, gestion de flotte, ou tout processus avec privilèges d'administrateur. Ne peut pas être défini via les paramètres gérés par le serveur                                            |
| `allowedMcpServers`          | Liste blanche des serveurs autorisés                                                      | N'importe quel [fichier de paramètres](/docs/fr/settings#settings-files) ; les entrées de chaque source fusionnent sauf si `allowManagedMcpServersOnly` est défini | Pour l'application, une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices) : paramètres gérés par le serveur, `managed-settings.json`, profil MDM, ou registre |
| `deniedMcpServers`           | Liste noire des serveurs bloqués                                                          | N'importe quel fichier de paramètres ; les entrées de chaque source fusionnent                                                                                | Identique à `allowedMcpServers`                                                                                                                                                             |
| `allowManagedMcpServersOnly` | Verrouille la liste blanche aux sources gérées uniquement                                 | Sources de paramètres gérés uniquement ; le paramètre n'a aucun effet ailleurs                                                                                | Identique à `allowedMcpServers`                                                                                                                                                             |
| `allowAllClaudeAiMcps`       | Charge les connecteurs claude.ai aux côtés de `managed-mcp.json` au lieu de les supprimer | Sources de paramètres gérés uniquement ; le paramètre n'a aucun effet ailleurs                                                                                | Identique à `allowedMcpServers`                                                                                                                                                             |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce) : restrictions MCP aux côtés des règles de permission, du sandboxing et des autres contrôles d'administration
* [Connecter Claude Code aux outils via MCP](/docs/fr/mcp) : la référence MCP complète, y compris les transports, les portées et l'authentification
* [Paramètres](/docs/fr/settings) : la hiérarchie des paramètres et comment les paramètres gérés ont la priorité
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrer `allowedMcpServers` et `deniedMcpServers` à partir de la console d'administration Claude.ai
* [Sécurité](/docs/fr/security) : le modèle de menace que ces contrôles défendent
* [Guide de l'administrateur Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) : SSO, SCIM, gestion des sièges et playbook de déploiement
