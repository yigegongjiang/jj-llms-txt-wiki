> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contrôlez l'accès aux serveurs MCP pour votre organisation

> Limitez les serveurs MCP que les utilisateurs peuvent ajouter ou connecter, ou fournissez des serveurs à tous les utilisateurs, avec des fichiers de configuration gérés, des paramètres gérés, des listes blanches et des listes noires.

Par défaut, toute personne exécutant Claude Code peut connecter n'importe quel [serveur MCP](/docs/fr/mcp) de son choix. Anthropic examine les connecteurs par rapport à ses [critères d'examen](https://claude.com/docs/connectors/building/review-criteria) avant de les ajouter à l'[Annuaire Anthropic](https://claude.ai/directory), mais n'effectue pas d'audit de sécurité ni ne gère aucun serveur MCP. En tant qu'administrateur, vous pouvez limiter les serveurs qui s'exécutent dans votre organisation, en déployant un ensemble approuvé fixe ou en désactivant complètement MCP, et vous pouvez fournir des serveurs à tous les utilisateurs.

Ces restrictions couvrent les serveurs que Claude Code charge lui-même, y compris les connecteurs qu'il récupère depuis claude.ai. Les connecteurs que l'application de bureau fournit à ses sessions locales et SSH arrivent en processus et sont gouvernés par vos paramètres d'organisation claude.ai à la place ; [Comment les connecteurs atteignent Claude Code](/docs/fr/mcp#how-connectors-reach-claude-code) montre quels contrôles s'appliquent aux connecteurs dans chaque type de session, y compris les sessions cloud.

Cette page couvre comment :

* [Choisir un modèle](#choose-a-pattern) qui correspond au niveau de contrôle dont vous avez besoin
* [Déployer un ensemble de serveurs fixe avec `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), y compris comment [désactiver complètement MCP](#disable-mcp-entirely)
* [Fournir des serveurs via les paramètres gérés](#provide-servers-through-managed-settings) tandis que les utilisateurs conservent les leurs
* [Contrôler les serveurs avec des listes blanches et des listes noires](#policy-based-control-with-allowlists-and-denylists)
* [Informer les utilisateurs de ce à quoi s'attendre](#how-restrictions-appear-to-users) quand une restriction bloque un serveur
* [Surveiller les serveurs que votre organisation utilise réellement](#monitor-mcp-usage)

<Note>
  La page [Sécurité](/docs/fr/security) couvre le modèle de menace MCP et comment évaluer un serveur avant de l'approuver. [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce) couvre les restrictions MCP aux côtés des autres contrôles administratifs.
</Note>

<h2 id="choose-a-pattern">
  Choisir un modèle
</h2>

Claude Code prend en charge une gamme de niveaux de restriction. Chaque modèle utilise un ou plusieurs des mécanismes couverts ci-dessous : `managed-mcp.json` pour déployer un ensemble fixe, le paramètre géré `managedMcpServers` pour fournir des serveurs aux côtés de ceux que les utilisateurs ajoutent, et `allowedMcpServers`/`deniedMcpServers` pour filtrer ce que les utilisateurs configurent.

| Modèle                            | Ce qu'il fait                                                                                                                                                                                                                                                        | Configurer                                                                                                       |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| **Désactiver MCP**                | Aucun serveur ne se charge, à part [les serveurs en processus que l'application qui a démarré la session enregistre](#exclusive-control-with-managed-mcp-json) et tous ceux que vous [fournissez via `managedMcpServers`](#provide-servers-through-managed-settings) | `managed-mcp.json` avec une carte de serveurs vide                                                               |
| **Déploiement fixe**              | Chaque utilisateur obtient les mêmes serveurs et ne peut pas en ajouter d'autres                                                                                                                                                                                     | `managed-mcp.json` avec les serveurs que vous souhaitez                                                          |
| **Serveurs fournis**              | Chaque utilisateur obtient les serveurs distants que vous listez et conserve les siens                                                                                                                                                                               | `managedMcpServers` dans les paramètres gérés                                                                    |
| **Catalogue approuvé**            | Publiez une liste de serveurs approuvés ; les utilisateurs ajoutent ceux qu'ils souhaitent, tout le reste est bloqué                                                                                                                                                 | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                         |
| **Serveurs de plugin uniquement** | Les utilisateurs ne peuvent pas ajouter de serveurs via `~/.claude.json` ou `.mcp.json` ; les serveurs de plugin se chargent toujours                                                                                                                                | [`strictPluginOnlyCustomization`](/docs/fr/settings-reference#strictpluginonlycustomization) avec `mcp` dans la liste |
| **Liste d'approbation souple**    | Appliquer une liste d'approbation que les utilisateurs peuvent élargir dans leurs propres paramètres                                                                                                                                                                 | `allowedMcpServers` sans `allowManagedMcpServersOnly`                                                            |
| **Liste de refus uniquement**     | Bloquer les serveurs connus comme mauvais, autoriser tout le reste                                                                                                                                                                                                   | `deniedMcpServers`                                                                                               |
| **Aucune restriction**            | Les utilisateurs ajoutent n'importe quoi                                                                                                                                                                                                                             | Ne déployez aucune configuration MCP gérée                                                                       |

<Note>
  Claude Code n'a pas de registre de serveur MCP intégré que les utilisateurs peuvent parcourir et installer. Pour le modèle de catalogue approuvé, partagez la liste approuvée et ses commandes `claude mcp add` quelque part où vos utilisateurs les trouveront, comme un wiki interne, ou distribuez les serveurs en tant que plugins via une [place de marché de plugins gérée](/docs/fr/plugins/org#restrict-what-users-can-install) afin que les utilisateurs puissent les parcourir et les installer depuis `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Contrôle exclusif avec managed-mcp.json
</h2>

Lorsque vous déployez un fichier `managed-mcp.json`, Claude Code charge uniquement ces serveurs MCP :

* Les serveurs que le fichier définit
* Les serveurs que vous [fournissez via `managedMcpServers`](#provide-servers-through-managed-settings)
* Les serveurs en processus que l'application qui a démarré la session enregistre, comme le serveur propre de l'extension VS Code ou les [connecteurs que l'application de bureau fournit](/docs/fr/mcp#how-connectors-reach-claude-code)

Les utilisateurs ne peuvent pas ajouter, modifier ou utiliser d'autres serveurs MCP, y compris les serveurs fournis par les plugins et les serveurs transmis avec le [drapeau CLI `--mcp-config`](/docs/fr/cli-reference#cli-flags). Le fichier supprime également les connecteurs claude.ai que Claude Code récupère lui-même, sauf si vous [les autorisez aux côtés de l'ensemble géré](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  Déployer managed-mcp.json
</h3>

`managed-mcp.json` est un fichier autonome, il ne peut donc pas être livré via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings). Pour livrer les serveurs via les paramètres gérés à la place, sans contrôle exclusif, utilisez [`managedMcpServers`](#provide-servers-through-managed-settings).

Tout processus pouvant écrire dans un chemin système avec des privilèges d'administrateur peut déployer le fichier. Sur une flotte, c'est généralement via des outils de gestion d'appareils, comme Jamf ou un profil de configuration sur macOS, une stratégie de groupe ou Intune sur Windows, ou votre gestion de flotte de choix sur Linux. Claude Code recherche le fichier à l'un de ces chemins :

| Plateforme   | Chemin                                                     |
| :----------- | :--------------------------------------------------------- |
| macOS        | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux et WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows      | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

Le fichier utilise le même format qu'un fichier [`.mcp.json`](/docs/fr/mcp#project-scope) de projet :

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

N'importe quel utilisateur sur la machine peut lire ce fichier, donc ne stockez pas de clés API ou d'autres identifiants dans les blocs `env`. Transmettez les identifiants par utilisateur avec l'une de ces options à la place :

* [Expansion `${VAR}`](/docs/fr/mcp#environment-variable-expansion-in-mcp-json) pour lire les secrets de l'environnement de chaque utilisateur.
* [OAuth ou en-têtes par utilisateur](/docs/fr/mcp#authenticate-with-remote-mcp-servers) pour que chaque utilisateur s'authentifie en tant que lui-même.
* [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) pour générer des identifiants au moment de la connexion.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Serveurs transmis avec `--mcp-config` ou `--strict-mcp-config`
</h3>

Lorsqu'une session reçoit des serveurs via `--mcp-config` tandis qu'un `managed-mcp.json` que Claude Code peut lire et analyser est déployé, ce que l'utilisateur voit diffère entre une station de travail et une session cloud :

* Sur une station de travail, Claude Code se ferme au démarrage avec `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Dans les [sessions cloud](/docs/fr/claude-code-on-the-web) sur un hôte où le fichier est déployé, comme un [exécuteur auto-hébergé](/docs/fr/self-hosted-environments-configuration#mcp-servers), Claude Code démarre avec les serveurs gérés uniquement et ignore les connecteurs claude.ai et les autres serveurs que l'hôte cloud fournit via `--mcp-config`. Rien dans la session n'indique à l'utilisateur quels serveurs ont été omis. Claude Code les nomme dans un avertissement sur son stderr, qu'un exécuteur auto-hébergé enregistre au niveau de journal `debug`.

Le drapeau `--strict-mcp-config` demande de remplacer l'ensemble géré. Si un utilisateur le transmet tandis qu'un tel fichier est déployé, Claude Code se ferme au démarrage sur une station de travail et dans une session cloud.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Comment les listes blanches et les listes noires s'appliquent à l'ensemble géré
</h3>

La liste noire peut filtrer davantage les serveurs dans `managed-mcp.json` :

* `deniedMcpServers` s'applique également aux serveurs gérés, donc un serveur géré qui correspond à une entrée ne se chargera pas.
* La propre `deniedMcpServers` d'un utilisateur se fusionne à partir de ses paramètres, donc les utilisateurs peuvent bloquer un serveur géré pour eux-mêmes.

`allowedMcpServers` ne s'applique pas aux serveurs dans `managed-mcp.json`, à une exception près : Claude Code vérifie toujours un serveur dont la définition utilise [l'expansion `${VAR}`](/docs/fr/mcp#environment-variable-expansion-in-mcp-json) par rapport à la liste blanche, car la configuration effective de ce serveur provient de l'environnement de chaque utilisateur plutôt que du fichier seul. Avant la v2.1.259, chaque serveur géré devait passer la liste blanche chaque fois qu'une était définie. Consultez [Comment un serveur est évalué](#how-a-server-is-evaluated) pour savoir quels champs déclenchent la vérification `${VAR}` et l'ordre complet des vérifications.

Si vous avez utilisé `allowedMcpServers` pour empêcher certains de vos propres serveurs `managed-mcp.json` de se charger, ces serveurs commencent à se charger au premier lancement de chaque utilisateur de la v2.1.259 ou ultérieure, sauf s'ils utilisent l'expansion `${VAR}`, sans invite ni avis : seul `deniedMcpServers` soustrait toujours de ces serveurs. Ajoutez des entrées de liste noire pour eux, ou déployez un `managed-mcp.json` séparé par groupe, avant que vos utilisateurs ne mettent à niveau.

<h3 id="validate-the-configuration">
  Valider la configuration
</h3>

Pour confirmer que le fichier est en vigueur, exécutez deux vérifications sur une machine gérée :

1. `claude mcp list` affiche uniquement les serveurs dans `managed-mcp.json`, plus tous ceux que vous fournissez via `managedMcpServers`. Deux autres résultats signifient que quelque chose ne va pas :
   * Si les propres serveurs d'un utilisateur apparaissent toujours, Claude Code ne lit pas le fichier, donc vérifiez son chemin et les permissions sur ses répertoires parents.
   * Si les serveurs du fichier n'apparaissent pas et que la section `MCP config diagnostics` marque la configuration d'entreprise comme échouée à l'analyse, Claude Code ne peut pas lire ou analyser le fichier. Corrigez l'erreur que cette section nomme, puis demandez à l'utilisateur de redémarrer Claude Code.
2. `claude mcp add --transport http test https://example.com/mcp` échoue avec `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. L'URL n'a pas besoin d'être un serveur réel, car la vérification de la politique rejette la commande avant que quoi que ce soit ne soit contacté.

<h3 id="disable-mcp-entirely">
  Désactiver MCP entièrement
</h3>

Déployez un `managed-mcp.json` contenant une carte de serveur vide pour bloquer chaque serveur MCP à part les [serveurs en processus que l'application qui a démarré la session enregistre](#exclusive-control-with-managed-mcp-json) :

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` échoue avec l'erreur de politique d'entreprise ci-dessus. Les serveurs que les utilisateurs avaient précédemment configurés cessent de se charger la prochaine fois qu'ils démarrent une session, sans avertissement que la politique en est la raison. Les serveurs que vous fournissez via `managedMcpServers` se chargent toujours sous une carte vide, donc laissez cette clé non définie également pour désactiver MCP complètement.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Autoriser les connecteurs claude.ai aux côtés de l'ensemble géré
</h3>

Par défaut, le déploiement de `managed-mcp.json` supprime les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) que Claude Code récupère lui-même, y compris les connecteurs qu'un administrateur a configurés pour l'organisation dans la console d'administration claude.ai. Pour charger ces connecteurs aux côtés des serveurs dans `managed-mcp.json`, définissez `"allowAllClaudeAiMcps": true` dans une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices).

Avec le paramètre activé, Claude Code charge les mêmes connecteurs claude.ai qu'il chargerait si `managed-mcp.json` n'était pas déployé. Les [listes blanches et les listes noires](#policy-based-control-with-allowlists-and-denylists) s'appliquent toujours à ces connecteurs, donc vous pouvez en bloquer des spécifiques avec `deniedMcpServers`. Le paramètre affecte uniquement les connecteurs claude.ai que Claude Code récupère lui-même ; les serveurs fournis par les plugins restent supprimés.

Les sessions cloud et les sessions locales et SSH de l'application de bureau reçoivent les connecteurs d'une autre manière, décrite dans [Comment les connecteurs atteignent Claude Code](/docs/fr/mcp#how-connectors-reach-claude-code). Un `managed-mcp.json` sur l'hôte qui exécute une session cloud, comme un [hôte d'exécuteur auto-hébergé](/docs/fr/self-hosted-environments-configuration#mcp-servers), supprime les connecteurs de cette session, que vous définissiez ou non `allowAllClaudeAiMcps`. Aucun `managed-mcp.json` n'atteint les connecteurs que l'application de bureau fournit à ses sessions locales et SSH.

Claude Code lit `allowAllClaudeAiMcps` uniquement à partir des niveaux de politique contrôlés par l'administrateur : paramètres gérés par le serveur, une clé plist déployée par MDM ou une clé de registre HKLM, ou un fichier `managed-settings.json` système. Le placer dans les paramètres utilisateur ou projet n'a aucun effet, donc les utilisateurs ne peuvent pas réactiver les connecteurs que le contrôle exclusif a supprimés.

<h2 id="provide-servers-through-managed-settings">
  Fournir des serveurs via les paramètres gérés
</h2>

Pour donner à chaque utilisateur un ensemble de serveurs MCP distants sans prendre le contrôle exclusif de MCP, listez-les sous `managedMcpServers` dans une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices) : paramètres gérés par le serveur, une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway-config#what-goes-in-cli), un profil MDM ou une politique de registre, ou `managed-settings.json`. Les utilisateurs conservent les serveurs qu'ils ajoutent eux-mêmes et reçoivent les vôtres en plus. Nécessite Claude Code v2.1.259 ou ultérieur. Les clients antérieurs ignorent la clé.

La valeur est un objet indexé par le nom du serveur. Chaque entrée a la même forme qu'un serveur HTTP ou SSE dans un fichier [`.mcp.json`](/docs/fr/mcp#project-scope) de projet, y compris les membres optionnels `headers` et `oauth` décrits dans [S'authentifier auprès des serveurs MCP distants](/docs/fr/mcp#authenticate-with-remote-mcp-servers). Cet exemple fournit un serveur de recherche auquel chaque utilisateur se connecte avec OAuth, et un serveur d'enregistrements qui envoie un en-tête que votre organisation émet :

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Quiconque peut lire les paramètres gérés sur une machine, y compris l'utilisateur, peut lire une valeur d'en-tête que vous définissez ici. Utilisez une credential émise pour tout ce public, ou omettez `headers` et laissez chaque utilisateur se connecter avec OAuth.

<h3 id="what-an-entry-can-contain">
  Ce qu'une entrée peut contenir
</h3>

Claude Code charge une entrée uniquement si elle réussit chaque vérification ci-dessous. Il supprime une entrée qui en échoue une, enregistre un avis que vous pouvez lire avec `/status`, et charge toujours les autres entrées :

* `type` est `http` ou `sse`. Comme dans `.mcp.json`, `streamable-http` est accepté comme alias pour `http`.
* `url` est une URL `https://`. Claude Code refuse une URL `http://` simple, y compris une qui pointe sur `localhost`.
* L'entrée n'a pas de membre `command`, `args`, `env`, ou `headersHelper`, donc un document de paramètres gérés ne nomme jamais un programme à exécuter sur la machine d'un utilisateur.
* Aucune valeur ne contient de référence `${VAR}`. Claude Code n'étend pas les variables d'environnement dans ces entrées, donc écrivez des valeurs littérales.
* Le nom du serveur contient uniquement des lettres, des chiffres, des tirets et des traits de soulignement, et aucune clé ou valeur ne contient de caractères de contrôle ou de formatage invisible.

Claude Desktop a un paramètre géré portant le même nom dont la valeur est un tableau d'une forme d'entrée différente, donc ne copiez pas l'un dans l'autre. Claude Code n'accepte pas la forme de tableau et enregistre un avis au lieu de la charger.

Une passerelle d'applications Claude exécute les mêmes vérifications au démarrage ; voir [Serveurs MCP dans une politique](/docs/fr/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Comment les serveurs fournis se chargent
</h3>

Ces règles décident ce qui se charge quand un serveur fourni chevauche une autre définition de serveur ou un autre paramètre sur cette page :

* Un serveur fourni a la priorité sur un serveur portant le même nom dans la portée locale, de projet ou d'utilisateur, et sur un serveur de plugin ou un connecteur claude.ai qui pointe sur la même URL.
* Si vous déployez également `managed-mcp.json`, Claude Code charge ses serveurs et les serveurs fournis ensemble, et l'entrée du fichier a la priorité quand les deux définissent un nom.
* Les serveurs fournis continuent de se charger quand [`strictPluginOnlyCustomization`](/docs/fr/settings-reference#strictpluginonlycustomization) verrouille la surface `mcp`.
* `deniedMcpServers` s'applique aux serveurs fournis, y compris les entrées des propres paramètres d'un utilisateur, donc un utilisateur peut en bloquer un pour lui-même. Les serveurs fournis n'ont besoin d'aucune entrée `allowedMcpServers`.

Quand vous n'avez pas également déployé `managed-mcp.json`, les drapeaux par exécution conservent leur signification :

* Un serveur qu'un utilisateur transmet avec `--mcp-config` sous le même nom remplace le serveur fourni pour cette exécution et est vérifié par rapport à `allowedMcpServers`.
* `--strict-mcp-config` laisse les serveurs fournis de côté ainsi que tous les autres serveurs configurés.

Avec `managed-mcp.json` déployé, les deux drapeaux se comportent comme [Contrôle exclusif avec managed-mcp.json](#exclusive-control-with-managed-mcp-json) le décrit.

<h3 id="what-users-can-see-and-change">
  Ce que les utilisateurs peuvent voir et modifier
</h3>

Les utilisateurs ne peuvent pas modifier ou supprimer un serveur fourni :

* `claude mcp remove` signale que le serveur est fourni par l'organisation.
* Quand vous n'avez pas également déployé `managed-mcp.json`, une entrée qu'un utilisateur ajoute sous le même nom est enregistrée mais non utilisée tant que la vôtre est présente.
* Les utilisateurs peuvent toujours désactiver un serveur fourni pour eux-mêmes dans [`/mcp`](/docs/fr/mcp#disable-a-server-without-removing-it), qui liste les serveurs fournis sous **Managed MCPs**.

`claude mcp get` et `/mcp` affichent l'URL d'un serveur fourni comme son hôte uniquement, par exemple `https://mcp.example.com/…`, et `claude mcp get` affiche les noms de ses en-têtes sans leurs valeurs.

<h3 id="where-managedmcpservers-applies">
  Où `managedMcpServers` s'applique
</h3>

Claude Code lit `managedMcpServers` à partir de la source gérée qu'il sélectionne sous [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources). Quand cette source définit [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) sur `"merge"`, Claude Code fournit à la place les serveurs de chaque source d'administrateur, et quand deux sources définissent le même nom, l'entrée de la source de rang supérieur s'applique entièrement. Il ne lit jamais la clé à partir du registre HKCU accessible en écriture par l'utilisateur, à partir des [paramètres parents qu'un hôte d'intégration fournit](/docs/fr/managed-settings#parent-settings-from-embedding-hosts), ou à partir des fichiers de paramètres utilisateur, de projet ou locaux, où il supprime la clé avec un avertissement.

Claude Code ne lit pas la clé dans l'onglet Code de l'application Claude Desktop sur un déploiement tiers ou dans les sessions Cowork de l'application, car Claude Desktop fournit et verrouille les serveurs MCP de ces sessions lui-même. `/status` et `claude doctor` le disent quand vos paramètres gérés portent la clé là.

<h3 id="when-provided-servers-connect">
  Quand les serveurs fournis se connectent
</h3>

Quand `managedMcpServers` arrive via les paramètres gérés par le serveur, son calendrier suit [Comportement de récupération et de mise en cache](/docs/fr/server-managed-settings#fetch-and-caching-behavior) :

* Sur une machine avec des paramètres en cache, Claude Code retient la copie en cache de cette clé jusqu'à ce que le serveur confirme les paramètres pour la session, et attend cette confirmation avant de charger les serveurs MCP. Si la confirmation échoue, la session continue sans les serveurs fournis et `/status` dit qu'ils sont retenus.
* Au premier lancement d'une machine, sans rien en cache encore, une session interactive qui démarre avant l'arrivée des paramètres connecte les serveurs fournis dès qu'ils arrivent, et une exécution `claude -p` qui a déjà commencé peut se terminer sans eux.

Avec [la connexion à la passerelle](/docs/fr/claude-apps-gateway-config#precedence-with-other-managed-sources), Claude Code charge la politique avant le démarrage de la session, donc aucun des deux cas ne retarde ou ne saute les serveurs fournis.

Les sessions interactives déjà en cours appliquent vos modifications à la clé :

* **Ajouter un serveur** : Claude Code le connecte quand les paramètres mis à jour arrivent, sans redémarrage.
* **Modifier l'entrée d'un serveur** : ces sessions se reconnectent à celui-ci avec la nouvelle définition.
* **Supprimer un serveur** : une session interactive en cours le déconnecte une fois qu'elle lit les paramètres modifiés. Une exécution non-interactive (`-p`) le conserve jusqu'à ce qu'elle se termine.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Contrôle basé sur les politiques avec listes blanches et listes noires
</h2>

Les listes blanches et les listes noires filtrent les serveurs configurés autorisés à se charger. Ce ne sont pas un registre : un serveur doit toujours être ajouté par un utilisateur, un plugin ou votre organisation avant que l'une ou l'autre liste ne s'applique à lui.

Les serveurs que votre organisation fournit via `managedMcpServers` se chargent sans entrée de liste blanche, et [Comment un serveur est évalué](#how-a-server-is-evaluated) couvre les serveurs `managed-mcp.json`. La liste noire s'applique à tous les serveurs, peu importe d'où ils proviennent, sauf pour les entrées `type: "sdk"` en processus.

Pour déployer des serveurs aux utilisateurs, utilisez [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) ou [`managedMcpServers`](#provide-servers-through-managed-settings). Les deux listes filtrent également les serveurs transmis avec l'indicateur CLI [`--mcp-config`](/docs/fr/cli-reference#cli-flags), sauf pour les entrées `type: "sdk"` en processus ; `--strict-mcp-config` limite les fichiers de configuration qui se chargent et ne contourne aucune des deux listes.

Pour rendre la liste blanche faisant autorité, définissez `allowedMcpServers` et `allowManagedMcpServersOnly: true` ensemble dans une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices), comme les paramètres gérés par le serveur ou un fichier `managed-settings.json` déployé.

Le verrou s'applique à partir de chaque source gérée contrôlée par l'administrateur, donc un verrouillage dans un fichier déployé s'applique toujours quand les paramètres gérés par le serveur qui ne mentionnent pas MCP sont également en cours d'utilisation. Pendant que le verrou est activé, la liste blanche gérée provient de la source d'administrateur la mieux classée qui en définit une. La lecture du verrou et de la liste blanche entre les sources nécessite Claude Code v2.1.273 ou ultérieur.

[Restreindre la liste blanche aux paramètres gérés uniquement](#restrict-the-allowlist-to-managed-settings-only) montre la configuration.

Sans `allowManagedMcpServersOnly`, les listes blanches de chaque portée de paramètres fusionnent, y compris le propre `~/.claude/settings.json` d'un utilisateur, donc un utilisateur peut élargir ce que votre liste blanche permet. Les listes noires fusionnent de chaque portée indépendamment.

<Note>
  `allowManagedMcpServersOnly` est distinct de `allowManagedPermissionRulesOnly`, qui verrouille uniquement les [règles de permission](/docs/fr/permissions#managed-settings). La définition de cet indicateur n'applique pas la liste blanche MCP.
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

| Paramètre           | Non défini (par défaut)     | Tableau vide `[]`                                                                           | Rempli                                                                                                           |
| :------------------ | :-------------------------- | :------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers` | Tous les serveurs autorisés | Aucun serveur autorisé, à part [les serveurs de l'organisation](#how-a-server-is-evaluated) | Seuls les serveurs correspondants autorisés, à part [les serveurs de l'organisation](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | Aucun serveur bloqué        | Aucun serveur bloqué                                                                        | Serveurs correspondants bloqués                                                                                  |

Voir [Entrées invalides dans les paramètres gérés](/docs/fr/managed-settings#invalid-entries-in-managed-settings) pour savoir ce qui se passe quand une entrée échoue la validation du schéma.

<Warning>
  Une entrée `serverName`, dans l'une ou l'autre liste, n'est pas un contrôle de sécurité. Le nom est l'étiquette qu'un utilisateur assigne lors de l'exécution de `claude mcp add` ou de la modification d'un fichier de configuration, pas le serveur sous-jacent, donc un utilisateur peut appeler n'importe quel serveur `github`. Pour les connecteurs claude.ai, le nom est le nom d'affichage renvoyé par claude.ai, qui peut changer. Pour appliquer les serveurs qui s'exécutent réellement, ajoutez des entrées `serverCommand` ou `serverUrl`.
</Warning>

La validation `serverName` diffère entre les deux listes :

* Dans `deniedMcpServers`, `serverName` accepte n'importe quelle chaîne non vide sans espaces de début ou de fin, vous pouvez donc bloquer les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) par leur nom d'affichage. Par exemple, `{ "serverName": "claude.ai Slack" }` bloque le connecteur Slack. Préférez une entrée `serverUrl` quand vous avez besoin que le refus soit robuste aux renommages, ou quand un nom de connecteur entre en collision et gagne un suffixe ` (N)`.
* Dans `allowedMcpServers`, `serverName` est limité aux lettres, chiffres, traits d'union et traits de soulignement. Utilisez `serverUrl` pour ajouter à la liste blanche un connecteur claude.ai que Claude Code récupère lui-même ; pour les connecteurs qu'un hôte cloud fournit aux sessions auto-hébergées, utilisez plutôt les entrées listées sous [Le trafic des connecteurs quitte votre réseau](/docs/fr/self-hosted-environments-deploy#connector-traffic-leaves-your-network).

Pour désactiver tous les connecteurs claude.ai que Claude Code récupère lui-même, voir [`disableClaudeAiConnectors`](/docs/fr/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Comment un serveur est évalué
</h3>

Avant de charger un serveur, y compris un serveur de `managed-mcp.json`, Claude Code exécute les trois vérifications ci-dessous dans l'ordre. Il les exécute à nouveau quand un utilisateur reconnecte un serveur ou réactive un serveur désactivé dans `/mcp`. Les serveurs `type: "sdk"` en processus, que [l'application qui a démarré la session enregistre](/docs/fr/mcp#how-connectors-reach-claude-code), ignorent les trois.

1. **Fusionner les listes.** Les entrées de liste blanche et de liste noire de chaque portée de paramètres se combinent en une liste blanche et une liste noire. Quand `allowManagedMcpServersOnly` est `true`, seule la liste blanche gérée est conservée ; la liste noire fusionne toujours de chaque portée. Quand plus d'une source gérée est présente, [Les clés lues à partir de chaque source d'administrateur](/docs/fr/managed-settings#keys-read-from-every-admin-source) indique lesquelles d'entre elles fournissent les listes de la portée gérée.
2. **Vérifier la liste noire.** Un serveur qui correspond à n'importe quelle entrée de liste noire, par URL, commande ou nom, est bloqué. Rien ne remplace une correspondance de liste noire.
3. **Vérifier la liste blanche.** Si `allowedMcpServers` n'est défini nulle part, tous les serveurs qui ont réussi la liste noire se chargent. S'il est défini, ce que le serveur doit correspondre dépend de son type, montré dans le tableau ci-dessous.

   Les serveurs de l'organisation ignorent cette vérification : chaque entrée `managedMcpServers`, et toute entrée `managed-mcp.json` dont les valeurs n'utilisent pas d'expansion `${VAR}`. Les serveurs intégrés l'ignorent aussi, comme Claude dans Chrome, le serveur `ide` auquel Claude Code se connecte dans un IDE VS Code ou JetBrains en cours d'exécution, et les serveurs que l'interface CLI elle-même configure.

   Un serveur `managed-mcp.json` qui utilise l'expansion `${VAR}` dans sa commande, ses arguments, `env`, son URL ou ses en-têtes est toujours vérifié, tout comme tous les serveurs qu'un utilisateur, un plugin, `--mcp-config` ou claude.ai ajoute.

| Type de serveur       | Autorisé quand il correspond                                                                                                                   |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Distant (HTTP ou SSE) | Une entrée `serverUrl`. Une correspondance `serverName` compte uniquement quand la liste blanche ne contient pas d'entrées `serverUrl`         |
| Stdio                 | Une entrée `serverCommand`. Une correspondance `serverName` compte uniquement quand la liste blanche ne contient pas d'entrées `serverCommand` |

Trois règles de correspondance s'appliquent dans ces vérifications :

* **Les commandes correspondent exactement.** Chaque argument, dans l'ordre. `["npx", "-y", "server"]` ne correspond pas à `["npx", "server"]` ou `["npx", "-y", "server", "--flag"]`.
* **Les valeurs `serverCommand` et `serverUrl` se développent avant la correspondance.** L'entrée de politique et la valeur configurée du serveur passent toutes les deux par l'expansion [`${VAR}` et `${VAR:-default}`](/docs/fr/mcp#environment-variable-expansion-in-mcp-json), donc une entrée écrite comme `["${HOME}/bin/server"]` correspond à une configuration de serveur qui utilise soit la même référence, soit le chemin développé. Sous Windows, référencez une variable d'environnement qui y est définie, comme `${USERPROFILE}` au lieu de `${HOME}`. Les valeurs `serverName` correspondent littéralement et ne se développent jamais. Les deux côtés lisent des environnements différents ; [Comment les entrées de politique se développent](#how-policy-entries-expand) couvre lesquels, et comment les entrées de liste blanche et de liste noire diffèrent.
* **Les URL supportent les caractères génériques `*`** n'importe où dans le motif, y compris le schéma. La correspondance du nom d'hôte est insensible à la casse et ignore un point FQDN final, donc `https://Mcp.Example.com/*` correspond à `https://mcp.example.com/api`. Les chemins restent sensibles à la casse.

| Motif                       | Autorise                                                                                       |
| :-------------------------- | :--------------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Tous les chemins sur un domaine spécifique                                                     |
| `https://mcp.example.com`   | Aussi tous les chemins sur ce domaine. Un motif sans chemin correspond à n'importe quel chemin |
| `https://*.example.com/*`   | N'importe quel sous-domaine de `example.com`                                                   |
| `http://localhost:*/*`      | N'importe quel port sur localhost                                                              |
| `*://mcp.example.com/*`     | N'importe quel schéma vers un domaine spécifique                                               |

<h4 id="how-policy-entries-expand">
  Comment les entrées de politique se développent
</h4>

La valeur configurée du serveur se développe à partir de l'environnement de processus en direct, comme le reste de `.mcp.json`. Une entrée de politique se développe à partir d'un environnement épinglé à la place, donc une variable définie par un projet ou un fichier de paramètres utilisateur ne peut pas changer ce qu'une entrée de liste blanche signifie. Parce qu'une entrée de politique dépend toujours de la valeur du shell de lancement pour toute variable qu'elle référence, utilisez des URL et des commandes littérales pour les entrées sur lesquelles vous comptez pour l'application.

| Liste d'entrées     | Se développe à partir de                                                                                                                                                                                                                        | Expansion qui changerait le schéma, l'hôte ou la portée du chemin d'une entrée URL |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `allowedMcpServers` | L'environnement avec lequel Claude Code a démarré, plus les valeurs `env` des paramètres gérés                                                                                                                                                  | Claude Code ignore l'entrée                                                        |
| `deniedMcpServers`  | Le même, et une variable sans valeur de démarrage et sans `:-default` se remplit à partir des fichiers de paramètres en dehors du référentiel, comme les paramètres utilisateur ou gérés, qui élargissent uniquement ce que l'entrée correspond | L'entrée correspond toujours                                                       |

Nécessite Claude Code v2.1.219 ou ultérieur.

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

* **Ligne 3** : la première entrée `serverUrl`. Une fois qu'elle existe, chaque serveur distant doit correspondre à un motif d'URL, donc un utilisateur ne peut pas obtenir un serveur distant non listé en lui donnant un nom autorisé.
* **Ligne 5** : la première entrée `serverCommand`. Même effet pour les serveurs stdio, donc chaque serveur local doit correspondre exactement à une commande listée.
* **Ligne 11** : une entrée `serverName` dans la liste noire. Les entrées de liste noire s'appliquent toujours, donc n'importe quel serveur nommé `dangerous-server` est bloqué indépendamment de son URL ou de sa commande.

Une entrée `serverName` dans cette liste blanche ne correspondrait jamais à rien, puisque les deux types de transport ont déjà des entrées plus strictes.

Les accordéons ci-dessous parcourent la façon dont un serveur est évalué par rapport à d'autres combinaisons de liste blanche et de liste noire.

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
  | Serveur HTTP à `https://mcp.example.com/api`          | Autorisé : correspond au motif d'URL                           |
  | Serveur HTTP à `https://api.internal.example.com/mcp` | Autorisé : correspond au sous-domaine générique                |
  | Serveur HTTP à `https://external.example.com/mcp`     | Bloqué : ne correspond à aucun motif d'URL                     |
  | Serveur stdio avec n'importe quelle commande          | Bloqué : pas d'entrées de nom ou de commande pour correspondre |
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
  | Serveur HTTP nommé `my-api`                            | Bloqué : pas d'entrées de nom pour correspondre |
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

  | Serveur                                                            | Résultat                                   |
  | :----------------------------------------------------------------- | :----------------------------------------- |
  | Serveur stdio nommé `github` avec n'importe quelle commande        | Autorisé : pas de restrictions de commande |
  | Serveur stdio nommé `internal-tool` avec n'importe quelle commande | Autorisé : pas de restrictions de commande |
  | Serveur HTTP nommé `github`                                        | Autorisé : correspond au nom               |
  | N'importe quel serveur nommé `other`                               | Bloqué : le nom ne correspond pas          |
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

  | Serveur                                          | Résultat                                                                                    |
  | :----------------------------------------------- | :------------------------------------------------------------------------------------------ |
  | Serveur HTTP à `https://mcp.example.com/api`     | Autorisé : correspond au motif d'URL de liste blanche, pas de correspondance de liste noire |
  | Serveur HTTP à `https://staging.example.com/api` | Bloqué : correspond aux deux, mais la liste noire a la priorité                             |
  | Serveur HTTP à `https://other.com/mcp`           | Bloqué : ne correspond pas à la liste blanche                                               |
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

Quand `allowManagedMcpServersOnly` est `true`, les listes blanches des paramètres utilisateur, projet et locaux sont ignorées. La liste noire fusionne toujours de chaque portée de paramètres, donc les utilisateurs peuvent toujours bloquer les serveurs pour eux-mêmes.

<h2 id="how-restrictions-appear-to-users">
  Comment les restrictions apparaissent aux utilisateurs
</h2>

Pour voir ce que les utilisateurs voient au démarrage quand `managed-mcp.json` est déployé et que la session a également des serveurs `--mcp-config`, consultez [Contrôle exclusif avec managed-mcp.json](#exclusive-control-with-managed-mcp-json). Utilisez ce tableau pour reconnaître les autres rapports et pour indiquer aux utilisateurs à quoi s'attendre avant de déployer une modification :

| Restriction                                                                                                                                         | Ce que l'utilisateur voit                                                                                                    |
| :-------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` est présent et l'utilisateur exécute `claude mcp add`                                                                            | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| Le serveur est sur une liste de blocage et l'utilisateur exécute `claude mcp add`                                                                   | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| Le serveur n'est pas sur la liste d'autorisation et l'utilisateur exécute `claude mcp add`                                                          | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| L'utilisateur exécute `claude mcp remove` sur un serveur de `managedMcpServers`                                                                     | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Un serveur précédemment configuré est maintenant bloqué par la politique                                                                            | Le serveur disparaît de `/mcp` et `claude mcp list`                                                                          |
| Un serveur est bloqué pendant qu'une session est en cours d'exécution, et l'utilisateur sélectionne **Reconnect** ou l'active à nouveau dans `/mcp` | [`MCP server <name> is blocked by enterprise managed policy`](/docs/fr/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Quand un serveur disparaît silencieusement, l'utilisateur ne reçoit aucun signal indiquant que la politique en est la raison, alors informez les utilisateurs affectés des serveurs bloqués quand vous déployez une nouvelle restriction.

<h2 id="monitor-mcp-usage">
  Surveiller l'utilisation de MCP
</h2>

Quand [l'export OpenTelemetry](/docs/fr/monitoring-usage) est configuré, Claude Code peut enregistrer les serveurs MCP et les outils que les utilisateurs invoquent. Définissez `OTEL_LOG_TOOL_DETAILS=1` pour inclure les noms de serveur MCP et d'outils dans les événements d'outils, puis agrégez-les dans votre collecteur pour voir les serveurs auxquels vos utilisateurs se connectent réellement. Voir [Surveillance](/docs/fr/monitoring-usage) pour configurer l'exportateur et pour le schéma d'événement complet.

<h2 id="configuration-summary">
  Résumé de la configuration
</h2>

Chaque fichier et paramètre couvert par cette page, ce qu'il contrôle et comment le livrer :

| Surface                      | Ce qu'il contrôle                                                                                                                                                                                                                                                              | Où il se trouve                                                                                                                                                                                                                                    | Comment le livrer                                                                                                                                                                                         |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Ensemble de serveurs fixe, contrôle exclusif                                                                                                                                                                                                                                   | Chemin système : `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, ou `C:\Program Files\ClaudeCode\`                                                                                                                                | MDM, GPO, gestion de flotte, ou tout processus disposant de privilèges administrateur. Ne peut pas être défini via les paramètres gérés par le serveur                                                    |
| `managedMcpServers`          | Serveurs distants fournis à chaque utilisateur aux côtés des leurs                                                                                                                                                                                                             | Sources de paramètres gérés uniquement ; le paramètre n'a aucun effet ailleurs                                                                                                                                                                     | Une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices) : paramètres gérés par le serveur, une politique de passerelle, `managed-settings.json`, profil MDM, ou registre HKLM |
| `allowedMcpServers`          | Liste d'autorisation des serveurs autorisés                                                                                                                                                                                                                                    | N'importe quel [scope de paramètres](/docs/fr/settings#where-settings-live) ; [Comment un serveur est évalué](#how-a-server-is-evaluated) indique comment les listes de plusieurs scopes et sources gérées se combinent                                 | Pour l'application, une [source de paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices) : paramètres gérés par le serveur, `managed-settings.json`, profil MDM, ou registre               |
| `deniedMcpServers`           | Liste de refus des serveurs bloqués                                                                                                                                                                                                                                            | N'importe quel scope de paramètres ; [Comment un serveur est évalué](#how-a-server-is-evaluated) indique comment les listes de plusieurs scopes et sources gérées se combinent                                                                     | Identique à `allowedMcpServers`                                                                                                                                                                           |
| `allowManagedMcpServersOnly` | Verrouille la liste d'autorisation aux sources gérées uniquement                                                                                                                                                                                                               | Sources de paramètres gérés uniquement ; [Clés lues à partir de chaque source admin](/docs/fr/managed-settings#keys-read-from-every-admin-source) indique quelles sources gérées peuvent l'activer. Le paramètre n'a aucun effet dans les autres scopes | Identique à `allowedMcpServers`                                                                                                                                                                           |
| `allowAllClaudeAiMcps`       | Charge les connecteurs claude.ai que Claude Code récupère lui-même aux côtés de `managed-mcp.json`. [Un `managed-mcp.json` sur l'hôte qui exécute une session cloud supprime toujours les connecteurs de cette session](#allow-claude-ai-connectors-alongside-the-managed-set) | Sources de paramètres gérés uniquement ; le paramètre n'a aucun effet ailleurs                                                                                                                                                                     | Identique à `allowedMcpServers`                                                                                                                                                                           |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce) : restrictions MCP aux côtés des règles de permission, du sandboxing et des autres contrôles d'administration
* [Connecter Claude Code aux outils via MCP](/docs/fr/mcp) : la référence MCP complète, y compris les transports, les portées et l'authentification
* [Paramètres](/docs/fr/settings) : la hiérarchie des paramètres et comment les paramètres gérés ont la priorité
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrer `allowedMcpServers` et `deniedMcpServers` à partir de la console d'administration Claude.ai
* [Sécurité](/docs/fr/security) : le modèle de menace que ces contrôles défendent
* [Guide de l'administrateur Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) : SSO, SCIM, gestion des sièges et playbook de déploiement
