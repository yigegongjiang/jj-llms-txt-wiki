> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Disponibilité des fonctionnalités

> Comparez les fonctionnalités de Claude Code disponibles sur les plans d'abonnement Anthropic, la Console Anthropic, Amazon Bedrock, Claude Platform sur AWS, Google Cloud's Agent Platform et Microsoft Foundry.

L'interface CLI de Claude Code et tout ce qui s'exécute localement fonctionnent de manière identique chez chaque fournisseur. Pour les instructions de configuration par fournisseur, consultez l'[aperçu du déploiement en entreprise](/docs/fr/third-party-integrations). Pour accéder directement à ce qui manque chez votre fournisseur, consultez les onglets [résumé par fournisseur](#summary-by-provider).

Dans les tableaux ci-dessous, ✓ signifie disponible, ✗ signifie non disponible, et « Voir note » renvoie à une note de bas de page pour un support partiel. Un qualificatif après ✓ limite la disponibilité à ce sous-ensemble, et « Admin-enabled » signifie que la fonctionnalité est désactivée jusqu'à ce qu'un administrateur de l'organisation l'active.

<h2 id="availability-by-model-provider">
  Disponibilité par fournisseur de modèle
</h2>

La façon dont vous vous authentifiez détermine les fonctionnalités que Claude Code peut atteindre. Pour une liste unique de ce qui manque chez votre fournisseur, consultez les onglets [résumé par fournisseur](#summary-by-provider). Pour trouver votre colonne dans les tableaux :

* **Abonnement Claude** : vous vous connectez avec un compte claude.ai sur le plan Pro, Max, Team ou Enterprise
* **Console Anthropic** : vous vous authentifiez avec une clé API Anthropic ou en [vous connectant à un compte Console sans clé](/docs/fr/authentication#sign-in-without-an-api-key)
* **Amazon Bedrock** : vous utilisez les modèles Claude du catalogue de modèles Bedrock et définissez `CLAUDE_CODE_USE_BEDROCK`. Le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) est couvert par cette colonne
* **Claude Platform sur AWS** : vous avez acheté Claude via AWS Marketplace mais appelez l'API Anthropic, et définissez `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform** : géré par Google ; vous définissez `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry** : géré par Anthropic ; vous définissez `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Fonctionnalités disponibles chez chaque fournisseur
</h3>

Celles-ci fonctionnent chez chaque fournisseur :

* [CLI](/docs/fr/quickstart) et [Agent SDK](/docs/fr/agent-sdk/overview)
* Extensions [VS Code](/docs/fr/vs-code) et [JetBrains](/docs/fr/jetbrains)
* [Subagents](/docs/fr/sub-agents), [hooks](/docs/fr/hooks-guide), [commands](/docs/fr/commands) et [skills](/docs/fr/skills)
* Mémoire [CLAUDE.md](/docs/fr/memory), [plugins](/docs/fr/plugins/overview) et [serveurs MCP](/docs/fr/mcp)
* [Checkpoints](/docs/fr/checkpointing), [sandboxing](/docs/fr/sandboxing) et [Workflows](/docs/fr/workflows)
* Métriques [OpenTelemetry](/docs/fr/monitoring-usage) et le [fichier de paramètres géré](/docs/fr/managed-settings#delivery-mechanisms)

Celles-ci ont des différences spécifiques au fournisseur :

* **Serveurs MCP** : les [connecteurs de claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) se chargent uniquement lorsque votre abonnement claude.ai est la méthode d'authentification active. La [recherche d'outils](/docs/fr/mcp#configure-tool-search) est désactivée par défaut lorsque `ANTHROPIC_BASE_URL` pointe vers un hôte non-propriétaire, et n'est pas supportée sur les modèles Google Cloud's Agent Platform antérieurs à la génération Claude 4.5 ou sur les [déploiements Microsoft Foundry hébergés sur Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)
* **Subagents** : le [Subagent Explore intégré](/docs/fr/sub-agents#built-in-subagents) limite son modèle hérité à Opus sur l'API Claude, et hérite directement du modèle de la conversation principale sur tout autre fournisseur, y compris Claude Platform sur AWS
* **[Commands](/docs/fr/commands#all-commands)** :
  * `/design-sync` et `/import` avec sa forme de sous-commande `claude import` ne sont pas disponibles sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry et Claude Platform sur AWS, et via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway#availability-and-limitations)
  * `/voice` nécessite un compte claude.ai
  * `/list-agents` et son alias `/peers` sont disponibles uniquement dans les sessions où la [messagerie inter-sessions est activée](/docs/fr/cross-session-messaging#availability)

<h3 id="features-that-require-a-claude-subscription">
  Fonctionnalités qui nécessitent un abonnement Claude
</h3>

Celles-ci nécessitent de se connecter avec un compte claude.ai et ne sont pas accessibles avec une clé API de la Console Anthropic ou d'un fournisseur tiers :

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web), Claude Code sur mobile et [Claude Code dans Slack](/docs/fr/slack)
* [Claude Code Desktop](/docs/fr/desktop)
* [Routines](/docs/fr/routines) (`/schedule`)
* [Ultrareview](/docs/fr/ultrareview)
* [Code Review](/docs/fr/code-review) : plans Team et Enterprise
* [Remote Control](/docs/fr/remote-control)
* [Extension Chrome](/docs/fr/chrome)
* [Computer use](/docs/fr/computer-use) : plans Pro et Max
* [Artifacts](/docs/fr/artifacts) : plans Pro, Max, Team et Enterprise
* [Voice dictation](/docs/fr/voice-dictation)

Desktop est l'exception partielle : l'[acheminement par passerelle peut être configuré dans l'application ou par un administrateur](/docs/fr/llm-gateway-connect#desktop-app), les déploiements Enterprise peuvent acheminer Desktop vers Google Cloud's Agent Platform ou un fournisseur de passerelle via les [paramètres gérés](https://claude.com/docs/third-party/claude-desktop/configuration), et [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) exécute l'onglet Code sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou une passerelle LLM auto-hébergée. Pour la disponibilité par plan de ces fonctionnalités, consultez [Disponibilité par plan d'abonnement](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Capacités CLI qui varient selon le fournisseur
</h3>

Ces fonctionnalités fonctionnent dans l'interface CLI locale mais dépendent d'une capacité côté serveur que tous les fournisseurs n'exposent pas.

<table>
  <thead>
    <tr>
      <th>Fonctionnalité</th>
      <th>Abonnement Claude</th>
      <th>Console Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform sur AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/fr/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Voir note <sup><a href="#fn1">1</a></sup></td>
      <td>✓ ([déploiements hébergés sur Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options))</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/fr/fast-mode)</td>
      <td>✓ ([Activé par le propriétaire](/docs/fr/fast-mode#enable-fast-mode-for-your-organization) sur Team et Enterprise)</td>
      <td>✓ (organisations provisionnées)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/fr/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Voir note <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Voir note <sup><a href="#fn2">2</a></sup></td>
      <td>Voir note <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/fr/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Messagerie inter-sessions](/docs/fr/cross-session-messaging)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✓ (même machine) <sup><a href="#fn5">5</a></sup></td>
      <td>✓ (même machine) <sup><a href="#fn5">5</a></sup></td>
      <td>✓ (même machine) <sup><a href="#fn5">5</a></sup></td>
      <td>✓ (même machine) <sup><a href="#fn5">5</a></sup></td>
      <td>✓ (même machine) <sup><a href="#fn5">5</a></sup></td>
    </tr>

    <tr>
      <td>[Channels](/docs/fr/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/fr/github-actions)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[GitLab CI/CD](/docs/fr/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Admin et analytique
</h3>

Contrôles au niveau de l'organisation et visibilité de l'utilisation.

<table>
  <thead>
    <tr>
      <th>Fonctionnalité</th>
      <th>Abonnement Claude</th>
      <th>Console Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform sur AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Tableau de bord analytique et API](/docs/fr/analytics)</td>
      <td>✓ (tableau de bord : Team et Enterprise ; API : Enterprise)</td>
      <td>✓ <sup><a href="#fn4">4</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Paramètres gérés par le serveur](/docs/fr/server-managed-settings)</td>
      <td>✓ (Team et Enterprise)</td>
      <td>✓ (Team et Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Rétention zéro donnée](/docs/fr/zero-data-retention)</td>
      <td>✓ (comptes Enterprise qualifiés)</td>
      <td>✓ (comptes qualifiés)</td>
      <td>Voir note <sup><a href="#fn3">3</a></sup></td>
      <td>✓ (comptes qualifiés)</td>
      <td>Voir note <sup><a href="#fn3">3</a></sup></td>
      <td>Voir note <sup><a href="#fn3">3</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> Sur Google Cloud's Agent Platform, la recherche web est disponible pour les modèles Claude 4 et versions ultérieures.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> Sur ces fournisseurs, le mode auto prend en charge uniquement Claude Sonnet 5, Opus 4.7 ou versions ultérieures, et les modèles Fable. Consultez [Configuration du mode Auto](/docs/fr/auto-mode-config). Le mode de permission de démarrage intégré sur ces fournisseurs est Manuel. Consultez [quel mode une session démarre](/docs/fr/permission-modes#which-mode-a-session-starts-in). Dans les versions v2.1.158 à v2.1.206, le mode auto sur ces fournisseurs nécessitait également de définir `CLAUDE_CODE_ENABLE_AUTO_MODE=1` ; v2.1.207 a supprimé cette exigence.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Soumis à votre accord avec le fournisseur de cloud.<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Tableau de bord et API uniquement. Les [métriques de contribution](/docs/fr/analytics#enable-contribution-metrics) nécessitent une organisation Claude.ai Team ou Enterprise.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Nécessite Claude Code v2.1.224 ou version ultérieure sur macOS et Linux, y compris Linux dans WSL 2. Sur Windows natif, nécessite Claude Code v2.1.234 ou version ultérieure. Avec l'authentification par clé API, la messagerie est limitée à la même machine uniquement. Sur Amazon Bedrock, Claude Platform sur AWS, Google Cloud's Agent Platform et Microsoft Foundry, la messagerie est limitée à la même machine uniquement et nécessite Claude Code v2.1.248 ou version ultérieure. Claude peut trouver vos sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web) et vos sessions sur d'autres machines uniquement à partir d'une session connectée à [Remote Control](/docs/fr/remote-control). Pour vous connecter, vous avez besoin d'une connexion claude.ai et des autres [exigences de Remote Control](/docs/fr/remote-control#requirements). Consultez [Messagerie des sessions sur d'autres machines](/docs/fr/cross-session-messaging#message-sessions-on-other-machines).

<Note>
  Si vous vous authentifiez via une [passerelle LLM](/docs/fr/llm-gateway), la disponibilité des fonctionnalités correspond au fournisseur sous-jacent vers lequel la passerelle transfère, sauf pour les fonctionnalités que Claude Code désactive lui-même. Chaque fois que `ANTHROPIC_BASE_URL` pointe vers un hôte autre que `api.anthropic.com`, Claude Code désactive des fonctionnalités telles que [Remote Control](/docs/fr/remote-control#requirements) et [paramètres gérés par le serveur](/docs/fr/server-managed-settings#platform-availability), quelle que soit la passerelle transfère. Certaines fonctionnalités exclusives à Anthropic telles que l'[Advisor](/docs/fr/advisor) ne fonctionnent que si la passerelle transfère les demandes intactes à l'API Anthropic.

  Pour connaître les différences entre les demandes que Claude Code envoie entre une passerelle au format Amazon Bedrock ou Agent Platform, une passerelle `ANTHROPIC_BASE_URL` et une connexion à une passerelle d'applications Claude, consultez [comportement du client par méthode de connexion](/docs/fr/llm-gateway-protocol#how-the-connection-method-changes-client-behavior).
</Note>

<h3 id="summary-by-provider">
  Résumé par fournisseur
</h3>

Chaque onglet répertorie ce qui n'est pas disponible ou partiellement supporté chez ce fournisseur, avec des alternatives le cas échéant. Tout ce qui n'est pas répertorié fonctionne de la même manière que sur un abonnement Claude, à part les [différences spécifiques au fournisseur](#features-available-on-every-provider) notées ci-dessus. Sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry et Claude Platform sur AWS, la création de rapports d'erreurs et la télémétrie vers Anthropic sont désactivées par défaut. Consultez [comportements par défaut par fournisseur API](/docs/fr/data-usage#default-behaviors-by-api-provider) pour connaître le trafic qui atteint toujours Anthropic et comment refuser.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Non disponible :** toutes les [fonctionnalités qui nécessitent un abonnement Claude](#features-that-require-a-claude-subscription), plus [web search](/docs/fr/tools-reference#websearch-tool-behavior), [fast mode](/docs/fr/fast-mode), [Advisor](/docs/fr/advisor), [Channels](/docs/fr/channels), le [tableau de bord analytique](/docs/fr/analytics), les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) et les [commandes `/design-sync` et `/import`](/docs/fr/commands#all-commands).

    **Support partiel :**

    * [Desktop](/docs/fr/desktop) : uniquement via [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/fr/auto-mode-config) : Sonnet 5, Opus 4.7 ou versions ultérieures, et modèles Fable uniquement
    * [Messagerie inter-sessions](/docs/fr/cross-session-messaging) : entre vos sessions sur cette machine uniquement <sup><a href="#fn5">5</a></sup>
    * [Rétention zéro donnée](/docs/fr/zero-data-retention) : soumis à votre accord AWS

    **Alternatives :** pour la planification, utilisez [`/loop`](/docs/fr/scheduled-tasks) au lieu de `/schedule`. Pour les sessions cloud, utilisez [GitHub Actions](/docs/fr/github-actions) ou [GitLab CI/CD](/docs/fr/gitlab-ci-cd). Pour les recherches web, utilisez l'[outil WebFetch](/docs/fr/tools-reference#webfetch-tool-behavior) avec une URL spécifique.
  </Tab>

  <Tab title="Claude Platform sur AWS">
    **Non disponible :** toutes les [fonctionnalités qui nécessitent un abonnement Claude](#features-that-require-a-claude-subscription), plus [fast mode](/docs/fr/fast-mode), [Advisor](/docs/fr/advisor), [Channels](/docs/fr/channels), [GitHub Actions](/docs/fr/github-actions), le [tableau de bord analytique](/docs/fr/analytics), les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) et les [commandes `/design-sync` et `/import`](/docs/fr/commands#all-commands).

    **Disponible là où Amazon Bedrock ne l'est pas :** [web search](/docs/fr/tools-reference#websearch-tool-behavior).

    **Support partiel :**

    * [Messagerie inter-sessions](/docs/fr/cross-session-messaging) : entre vos sessions sur cette machine uniquement <sup><a href="#fn5">5</a></sup>

    **Alternatives :** pour la planification, utilisez [`/loop`](/docs/fr/scheduled-tasks) au lieu de `/schedule`. Pour les sessions cloud, utilisez [GitLab CI/CD](/docs/fr/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **Non disponible :** toutes les [fonctionnalités qui nécessitent un abonnement Claude](#features-that-require-a-claude-subscription), plus [fast mode](/docs/fr/fast-mode), [Advisor](/docs/fr/advisor), [Channels](/docs/fr/channels), le [tableau de bord analytique](/docs/fr/analytics), les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) et les [commandes `/design-sync` et `/import`](/docs/fr/commands#all-commands).

    **Support partiel :**

    * [Desktop](/docs/fr/desktop) : via les [paramètres gérés](https://claude.com/docs/third-party/claude-desktop/configuration) ou [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/fr/tools-reference#websearch-tool-behavior) : modèles Claude 4 et versions ultérieures
    * [Auto mode](/docs/fr/auto-mode-config) : Sonnet 5, Opus 4.7 ou versions ultérieures, et modèles Fable uniquement
    * [Messagerie inter-sessions](/docs/fr/cross-session-messaging) : entre vos sessions sur cette machine uniquement <sup><a href="#fn5">5</a></sup>
    * [Rétention zéro donnée](/docs/fr/zero-data-retention) : soumis à votre accord Google Cloud

    **Alternatives :** pour la planification, utilisez [`/loop`](/docs/fr/scheduled-tasks) au lieu de `/schedule`. Pour les sessions cloud, utilisez [GitHub Actions](/docs/fr/github-actions) ou [GitLab CI/CD](/docs/fr/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Non disponible :** toutes les [fonctionnalités qui nécessitent un abonnement Claude](#features-that-require-a-claude-subscription), plus [fast mode](/docs/fr/fast-mode), [Advisor](/docs/fr/advisor), [Channels](/docs/fr/channels), [GitLab CI/CD](/docs/fr/gitlab-ci-cd), le [tableau de bord analytique](/docs/fr/analytics), les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) et les [commandes `/design-sync` et `/import`](/docs/fr/commands#all-commands).

    **Support partiel :**

    * [Desktop](/docs/fr/desktop) : uniquement via [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/fr/tools-reference#websearch-tool-behavior) : [déploiements hébergés sur Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) uniquement
    * [Auto mode](/docs/fr/auto-mode-config) : Sonnet 5, Opus 4.7 ou versions ultérieures, et modèles Fable uniquement
    * [Messagerie inter-sessions](/docs/fr/cross-session-messaging) : entre vos sessions sur cette machine uniquement <sup><a href="#fn5">5</a></sup>
    * [Rétention zéro donnée](/docs/fr/zero-data-retention) : soumis à votre accord Azure

    **Alternatives :** pour la planification, utilisez [`/loop`](/docs/fr/scheduled-tasks) au lieu de `/schedule`. Pour les sessions cloud, utilisez [GitHub Actions](/docs/fr/github-actions).
  </Tab>

  <Tab title="Console Anthropic">
    **Non disponible :** toutes les [fonctionnalités qui nécessitent un abonnement Claude](#features-that-require-a-claude-subscription).

    Tout ce qui se trouve dans [Capacités CLI qui varient selon le fournisseur](#cli-capabilities-that-vary-by-provider) est disponible, sauf que [fast mode](/docs/fr/fast-mode) nécessite un [accès provisionné](/docs/fr/fast-mode#enable-fast-mode-for-your-organization). Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) sont également disponibles lorsque votre clé API appartient à une organisation Team ou Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Disponibilité par plan d'abonnement
</h2>

Si vous vous authentifiez via Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou une clé API de la Console Anthropic, cette section ne s'applique pas à vous. Lorsque vous vous connectez avec un compte claude.ai, votre plan détermine les fonctionnalités ci-dessous qui sont disponibles.

| Fonctionnalité                                                              | Pro | Max | Team          | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------------ | :-------------------------------- |
| [Cloud sessions](/docs/fr/claude-code-on-the-web)                                | ✓   | ✓   | ✓             | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/fr/routines)                                                    | ✓   | ✓   | ✓             | ✓                                 |
| [Remote Control](/docs/fr/remote-control)                                        | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Channels](/docs/fr/channels)                                                    | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Computer use](/docs/fr/computer-use)                                            | ✓   | ✓   | ✗             | ✗                                 |
| Dispatch ([Desktop](/docs/fr/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗             | ✗                                 |
| [Code Review](/docs/fr/code-review)                                              | ✗   | ✗   | ✓             | ✓                                 |
| [Artifacts](/docs/fr/artifacts)                                                  | ✓   | ✓   | ✓             | Admin-enabled                     |
| [Tableau de bord Analytics et métriques de contribution](/docs/fr/analytics)     | ✗   | ✗   | ✓             | ✓                                 |
| [API Enterprise Analytics](/docs/fr/analytics#access-data-programmatically)      | ✗   | ✗   | ✗             | ✓                                 |
| [Server-managed settings](/docs/fr/server-managed-settings)                      | ✗   | ✗   | ✓             | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓             | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗             | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗             | ✓                                 |
| [Zero Data Retention](/docs/fr/zero-data-retention)                              | ✗   | ✗   | ✗             | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> Sur Enterprise, nécessite un siège premium ou un siège Chat + Claude Code. Consultez [Cloud sessions](/docs/fr/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Non inclus dans le plan Enterprise standard. Nécessite une activation séparée par Anthropic pour les comptes qualifiés. Consultez [Zero Data Retention](/docs/fr/zero-data-retention).

Pour la tarification et la comparaison complète des plans, consultez [Plans Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) et [Plans Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Disponibilité des modèles
</h2>

Pour connaître les modèles Claude et les tailles de fenêtre contextuelle disponibles par fournisseur et région, consultez [Configuration des modèles](/docs/fr/model-config) et l'[aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, entrée PDF et réflexion étendue sont des capacités de modèle plutôt que des fonctionnalités de Claude Code et fonctionnent chez chaque fournisseur qui propose le modèle. [Prompt caching](/docs/fr/prompt-caching) fonctionne de la même manière chez la plupart des fournisseurs ; sur Amazon Bedrock, le support varie selon le modèle.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Aperçu du déploiement en entreprise](/docs/fr/third-party-integrations) : comparez l'authentification, la facturation et les régions entre les fournisseurs
* Guides de configuration des fournisseurs : [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry)
* [Plateformes et intégrations](/docs/fr/platforms) : où Claude Code s'exécute, y compris l'interface CLI, Desktop, les extensions IDE, le web, mobile et CI/CD
