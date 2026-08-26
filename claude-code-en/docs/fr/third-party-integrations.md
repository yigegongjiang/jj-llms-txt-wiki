> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aperçu du déploiement en entreprise

> Découvrez comment Claude Code peut s'intégrer à divers services tiers et infrastructures pour répondre aux exigences de déploiement en entreprise.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

Les organisations peuvent déployer Claude Code directement via Anthropic ou via un fournisseur de cloud. Cette page vous aide à choisir la bonne configuration.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Comparer les options de déploiement
</h2>

Pour la plupart des organisations, Claude for Teams ou Claude for Enterprise offre la meilleure expérience. Les membres de l'équipe ont accès à la fois à Claude Code et à Claude sur le web avec un seul abonnement, une facturation centralisée et aucune configuration d'infrastructure requise.

**Claude for Teams** est en libre-service et inclut des fonctionnalités de collaboration, des outils d'administration et la gestion de la facturation. Idéal pour les petites équipes qui ont besoin de démarrer rapidement.

**Claude for Enterprise** ajoute SSO et la capture de domaine, les autorisations basées sur les rôles, l'accès à l'API de conformité et les paramètres de politique gérés pour déployer des configurations Claude Code à l'échelle de l'organisation. Idéal pour les grandes organisations ayant des exigences de sécurité et de conformité.

En savoir plus sur les [plans d'équipe](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) et les [plans d'entreprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Si votre organisation a des exigences d'infrastructure spécifiques, comparez les options ci-dessous :

<table>
  <thead>
    <tr>
      <th>Fonctionnalité</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, anciennement Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Idéal pour</td>
      <td>La plupart des organisations (recommandé)</td>
      <td>Développeurs individuels</td>
      <td>Déploiements natifs AWS</td>
      <td>Facturation AWS Marketplace avec les fonctionnalités de l'API Claude</td>
      <td>Déploiements natifs GCP</td>
      <td>Déploiements natifs Azure</td>
    </tr>

    <tr>
      <td>Facturation</td>
      <td><strong>Teams :</strong> 150 \$/siège (Premium) avec PAYG disponible<br /><strong>Enterprise :</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Contacter les ventes</a></td>
      <td>PAYG</td>
      <td>PAYG via AWS</td>
      <td>PAYG via AWS Marketplace</td>
      <td>PAYG via GCP</td>
      <td>PAYG via Azure</td>
    </tr>

    <tr>
      <td>Régions</td>
      <td>[Pays](https://www.anthropic.com/supported-countries) supportés</td>
      <td>[Pays](https://www.anthropic.com/supported-countries) supportés</td>
      <td>Plusieurs [régions](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) AWS</td>
      <td>Plusieurs régions AWS</td>
      <td>Plusieurs [régions](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) GCP</td>
      <td>Plusieurs [régions](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/) Azure</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Activé par défaut</td>
      <td>Activé par défaut</td>
      <td>Activé par défaut</td>
      <td>Activé par défaut</td>
      <td>Activé par défaut</td>
      <td>Activé par défaut</td>
    </tr>

    <tr>
      <td>Authentification</td>
      <td>Claude.ai SSO ou email</td>
      <td>Clé API</td>
      <td>Clé API ou identifiants AWS</td>
      <td>Clé API ou identifiants AWS</td>
      <td>Identifiants GCP</td>
      <td>Clé API ou Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Suivi des coûts</td>
      <td>Tableau de bord d'utilisation</td>
      <td>Tableau de bord d'utilisation</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>Facturation GCP</td>
      <td>Gestion des coûts Azure</td>
    </tr>

    <tr>
      <td>Inclut Claude sur le web</td>
      <td>Oui</td>
      <td>Non</td>
      <td>Non</td>
      <td>Non</td>
      <td>Non</td>
      <td>Non</td>
    </tr>

    <tr>
      <td>Fonctionnalités d'entreprise</td>
      <td>Gestion d'équipe, SSO, surveillance de l'utilisation</td>
      <td>Aucune</td>
      <td>Politiques IAM, CloudTrail</td>
      <td>Politiques IAM, CloudTrail</td>
      <td>Rôles IAM, journaux d'audit cloud</td>
      <td>Politiques RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Pour une ventilation fonctionnalité par fonctionnalité de ce qui est disponible sur chaque option, consultez [Disponibilité des fonctionnalités](/docs/fr/feature-availability).

Sélectionnez une option de déploiement pour afficher les instructions de configuration :

* [Claude for Teams ou Enterprise](/docs/fr/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/fr/authentication#claude-console-authentication)
* [Passerelle d'applications Claude](/docs/fr/claude-apps-gateway), une passerelle auto-hébergée qui ajoute la connexion IdP devant Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry ou l'API Anthropic
* [Amazon Bedrock](/docs/fr/amazon-bedrock)
* [Claude Platform on AWS](/docs/fr/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai)
* [Microsoft Foundry](/docs/fr/microsoft-foundry)

Pour Amazon Bedrock et Google Vertex AI, vous pouvez également exécuter `claude` et sélectionner **plateforme tierce** à l'invite de connexion pour lancer un assistant de configuration interactif.

<h2 id="configure-proxies-and-gateways">
  Configurer les proxies et les passerelles
</h2>

La plupart des organisations peuvent utiliser un fournisseur de cloud directement sans configuration supplémentaire. Cependant, vous devrez peut-être configurer un proxy d'entreprise ou une passerelle LLM si votre organisation a des exigences réseau ou de gestion spécifiques. Il s'agit de configurations différentes qui peuvent être utilisées ensemble :

* **Proxy d'entreprise** : Achemine le trafic via un proxy HTTP/HTTPS. Utilisez ceci si votre organisation exige que tout le trafic sortant passe par un serveur proxy pour la surveillance de la sécurité, la conformité ou l'application des politiques réseau. Configurez avec les variables d'environnement `HTTPS_PROXY` ou `HTTP_PROXY`. En savoir plus dans [Configuration du réseau d'entreprise](/docs/fr/network-config).
* **Passerelle LLM** : Un service qui se situe entre Claude Code et le fournisseur de cloud pour gérer l'authentification et le routage. Utilisez ceci si vous avez besoin d'un suivi centralisé de l'utilisation entre les équipes, d'une limitation de débit personnalisée ou de budgets, ou d'une gestion centralisée de l'authentification. Configurez avec les variables d'environnement `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, ou `ANTHROPIC_FOUNDRY_BASE_URL`. En savoir plus dans [Passerelles LLM](/docs/fr/llm-gateway).

Les exemples suivants montrent les variables d'environnement à définir dans votre shell ou profil shell (`.bashrc`, `.zshrc`). Voir [Paramètres](/docs/fr/settings) pour d'autres méthodes de configuration.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Proxy d'entreprise">
    Acheminez le trafic Amazon Bedrock via votre proxy d'entreprise en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Passerelle LLM">
    Acheminez le trafic Amazon Bedrock via votre passerelle LLM en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1

    # Configure LLM gateway
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # If gateway handles AWS auth
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="Proxy d'entreprise">
    Acheminez le trafic Microsoft Foundry via votre proxy d'entreprise en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Or omit for Entra ID auth

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Passerelle LLM">
    Acheminez le trafic Microsoft Foundry via votre passerelle LLM en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1

    # Configure LLM gateway
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # Sent as x-api-key
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="Proxy d'entreprise">
    Acheminez le trafic Google Cloud's Agent Platform via votre proxy d'entreprise en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Passerelle LLM">
    Acheminez le trafic Google Cloud's Agent Platform via votre passerelle LLM en définissant les [variables d'environnement](/docs/fr/env-vars) suivantes :

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1

    # Configure LLM gateway
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # If gateway handles GCP auth
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Utilisez `/status` dans Claude Code pour vérifier que votre configuration de proxy et de passerelle est appliquée correctement. Par exemple, avec la configuration de passerelle Bedrock ci-dessus, la sortie inclut des lignes comme :

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Si vous avez configuré un proxy d'entreprise, `/status` affiche également une ligne `Proxy` avec l'URL de votre proxy.
</Tip>

<h2 id="best-practices-for-organizations">
  Meilleures pratiques pour les organisations
</h2>

<h3 id="invest-in-documentation-and-memory">
  Investir dans la documentation et la mémoire
</h3>

Nous recommandons vivement d'investir dans la documentation afin que Claude Code comprenne votre base de code. Les organisations peuvent déployer des fichiers CLAUDE.md à plusieurs niveaux :

* **À l'échelle de l'organisation** : Déployez dans des répertoires système comme `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux et WSL), ou `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) pour les normes à l'échelle de l'entreprise
* **Au niveau du référentiel** : Créez des fichiers `CLAUDE.md` dans les racines de référentiel contenant l'architecture du projet, les commandes de compilation et les directives de contribution. Vérifiez-les dans le contrôle de source afin que tous les utilisateurs en bénéficient

En savoir plus dans [Mémoire et fichiers CLAUDE.md](/docs/fr/memory).

<h3 id="simplify-deployment">
  Simplifier le déploiement
</h3>

Si vous avez un environnement de développement personnalisé, nous constatons que créer un moyen « en un clic » d'installer Claude Code est essentiel pour augmenter l'adoption dans une organisation.

<h3 id="start-with-guided-usage">
  Commencer par une utilisation guidée
</h3>

Encouragez les nouveaux utilisateurs à essayer Claude Code pour les questions sur la base de code, ou sur les corrections de bogues plus petites ou les demandes de fonctionnalités. Demandez à Claude Code de faire un plan. Vérifiez les suggestions de Claude et donnez des commentaires si c'est hors piste. Au fil du temps, à mesure que les utilisateurs comprendront mieux ce nouveau paradigme, ils seront plus efficaces pour laisser Claude Code fonctionner de manière plus agentique.

<h3 id="pin-model-versions-for-cloud-providers">
  Épingler les versions de modèle pour les fournisseurs de cloud
</h3>

Si vous déployez via [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry) ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), épinglez les versions de modèle spécifiques en utilisant `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` et `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Sans épinglage, les alias de modèle se résolvent à la valeur par défaut intégrée de Claude Code pour ce fournisseur, ce qui peut être en retard par rapport à la version la plus récente et peut ne pas encore être activé dans votre compte. L'épinglage vous permet de contrôler quand vos utilisateurs passent à un nouveau modèle. Voir [Configuration du modèle](/docs/fr/model-config#pin-models-for-third-party-deployments) pour ce que chaque fournisseur fait lorsque la valeur par défaut n'est pas disponible.

<h3 id="configure-security-policies">
  Configurer les politiques de sécurité
</h3>

Les équipes de sécurité peuvent configurer des autorisations gérées pour ce que Claude Code est et n'est pas autorisé à faire, ce qui ne peut pas être remplacé par la configuration locale. [En savoir plus](/docs/fr/security).

<h3 id="leverage-mcp-for-integrations">
  Tirer parti de MCP pour les intégrations
</h3>

MCP est un excellent moyen de donner à Claude Code plus d'informations, comme la connexion à des systèmes de gestion de tickets ou des journaux d'erreurs. Nous recommandons qu'une équipe centrale configure les serveurs MCP et vérifie une configuration `.mcp.json` dans la base de code afin que tous les utilisateurs en bénéficient. [En savoir plus](/docs/fr/mcp).

Chez Anthropic, nous faisons confiance à Claude Code pour alimenter le développement dans chaque base de code Anthropic. Nous espérons que vous apprécierez d'utiliser Claude Code autant que nous.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Une fois que vous avez choisi une option de déploiement et configuré l'accès pour votre équipe :

1. **Déployer auprès de votre équipe** : Partagez les instructions d'installation et demandez aux membres de l'équipe d'[installer Claude Code](/docs/fr/setup) et de s'authentifier avec leurs identifiants.
2. **Configurer la configuration partagée** : Créez un [fichier CLAUDE.md](/docs/fr/memory) dans vos référentiels pour aider Claude Code à comprendre votre base de code et vos normes de codage.
3. **Configurer les autorisations** : Consultez les [paramètres de sécurité](/docs/fr/security) pour définir ce que Claude Code peut et ne peut pas faire dans votre environnement.
