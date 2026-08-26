> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter Claude Code via une passerelle

> Acheminez Claude Code via une passerelle auto-hébergée pour les identifiants centralisés, le suivi de l'utilisation et les contrôles de coûts. Couvre l'architecture, la passerelle d'applications Claude d'Anthropic et l'utilisation d'autres produits de passerelle.

Une passerelle est un proxy que votre organisation exécute entre Claude Code et un fournisseur de modèles. Claude Code envoie le trafic API à la passerelle au lieu de l'envoyer directement au fournisseur, et la passerelle le transfère en utilisant un identifiant que votre organisation détient. Les développeurs s'authentifient auprès de la passerelle plutôt que de détenir des identifiants de fournisseur, de sorte que l'authentification, le suivi de l'utilisation, les budgets et la journalisation d'audit se produisent en un seul endroit que vous contrôlez.

Claude Code inclut une passerelle auto-hébergée, [Claude apps gateway](/docs/fr/claude-apps-gateway), dans le binaire `claude`, vous n'avez donc pas besoin d'adopter un produit de passerelle distinct pour en exécuter une. Si votre organisation exécute déjà une [passerelle LLM](/docs/fr/llm-gateway), Claude Code fonctionne également avec celle-ci.

Cette page couvre :

* [Comment une passerelle s'insère entre Claude Code et votre fournisseur](#how-a-gateway-works)
* [Choisir entre la passerelle d'applications Claude et une passerelle que vous exécutez déjà](#choose-a-gateway)
* [Comment les passerelles interagissent avec les abonnements claude.ai](#subscriptions-and-gateways)
* [Ce qui est configuré séparément de la passerelle](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  Comment fonctionne une passerelle
</h2>

Chaque Claude Code de développeur est pointé vers l'adresse de la passerelle et s'authentifie avec un identifiant émis par la passerelle.

La passerelle authentifie le développeur, applique les règles d'accès et de budget que vous configurez, et transfère la demande à votre fournisseur avec l'identifiant de l'organisation. Le fournisseur peut être l'API d'Anthropic ou un [fournisseur cloud](/docs/fr/third-party-integrations) tel qu'Amazon Bedrock, Agent Platform de Google Cloud ou Microsoft Foundry ; la configuration de la passerelle le décide. Avec la passerelle d'applications Claude, ou une autre passerelle qui expose un seul point de terminaison au format Anthropic, le changement de fournisseur ne nécessite pas de toucher aux machines des développeurs.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagramme montrant Claude Code acheminé via une passerelle. Dans une zone de machines de développeur, l'interface CLI Claude Code et l'extension VS Code envoient des demandes à l'adresse de la passerelle avec un identifiant par développeur. Dans une zone intitulée votre infrastructure, la passerelle gère l'authentification, le suivi de l'utilisation, les budgets et l'acheminement, et transfère les demandes avec l'identifiant de votre organisation. Dans une zone de fournisseurs de modèles, une flèche pleine mène au fournisseur que vous configurez, affiché comme l'API Anthropic, et des flèches pointillées mènent à d'autres options de fournisseur, illustrées avec Amazon Bedrock, Google Cloud et Microsoft Foundry comme exemples." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Deux types d'identifiants sont impliqués :

* **Identifiant de développeur** : chaque développeur en détient un, émis par la passerelle. Il l'authentifie auprès de la passerelle et l'identifie dans le suivi de l'utilisation
* **Identifiant de fournisseur** : la passerelle détient un identifiant pour votre compte fournisseur, partagé par tout le trafic transféré

<h2 id="choose-a-gateway">
  Choisir une passerelle
</h2>

Claude Code fonctionne avec la passerelle d'Anthropic ou avec une passerelle que votre organisation exécute déjà.

<h3 id="claude-apps-gateway">
  Passerelle d'applications Claude
</h3>

La passerelle d'applications Claude est la passerelle auto-hébergée d'Anthropic, incluse dans le binaire `claude`. Elle achemine vers Amazon Bedrock, Claude Platform sur AWS, Google Cloud, Microsoft Foundry ou l'API Anthropic en amont. Les développeurs se connectent avec votre fournisseur d'identité d'entreprise via `/login`, la passerelle applique l'accès aux modèles et les [paramètres gérés](/docs/fr/permissions#managed-settings) par groupe IdP, et elle émet des métriques d'utilisation du [protocole OpenTelemetry (OTLP)](/docs/fr/monitoring-usage) vers votre propre pile d'observabilité.

Parce qu'elle est construite et testée aux côtés de chaque version de Claude Code, elle transfère les en-têtes et les champs de demande que Claude Code envoie. Une passerelle maintenue séparément doit avoir ses [règles de transfert mises à jour](/docs/fr/llm-gateway-protocol#forward-as-open-lists) à mesure que ces en-têtes et champs changent à chaque version ; la passerelle d'applications Claude est publiée avec l'interface CLI, il n'y a donc pas de liste à tenir à jour. Voir [Disponibilité et limitations](/docs/fr/claude-apps-gateway#availability-and-limitations) pour le petit ensemble de fonctionnalités qui se comportent différemment sur une session de passerelle.

La connexion à la passerelle est une étape SSO du navigateur, et il n'y a pas de flux de jeton de service, donc un pipeline CI sans développeur pour approuver la connexion ne peut pas s'authentifier via celui-ci ; configurez-les directement auprès de votre fournisseur. Les sessions du SDK Agent et les exécutions `claude -p` sur une machine où un développeur s'est connecté utilisent la session de passerelle de cette machine et sont régies par ses politiques. Voir [Pipelines CI et machines distantes](/docs/fr/claude-apps-gateway#ci-pipelines-and-remote-machines).

Voir [Claude apps gateway](/docs/fr/claude-apps-gateway) pour la déployer.

<h3 id="other-gateways">
  Autres passerelles
</h3>

Si votre organisation exécute déjà une passerelle LLM ou une passerelle API, vous pouvez l'utiliser à la place. Anthropic n'approuve pas, ne maintient pas et n'audite pas d'autres produits de passerelle, et ne supporte pas l'acheminement de Claude Code vers des modèles non-Claude via aucune passerelle. Voir [Autres passerelles LLM](/docs/fr/llm-gateway) pour la liste de contrôle du déploiement administrateur, ce qu'une passerelle doit implémenter et comment pointer Claude Code vers celle-ci.

<h2 id="subscriptions-and-gateways">
  Abonnements et passerelles
</h2>

Lorsque les développeurs se connectent via une passerelle avec un identifiant de passerelle, l'utilisation est facturée au compte fournisseur de votre organisation aux tarifs de l'API, et leurs abonnements claude.ai ne sont pas utilisés ni facturés. La définition de [`ANTHROPIC_AUTH_TOKEN`](/docs/fr/env-vars) pour une passerelle que vous exécutez, ou la connexion à une passerelle d'applications Claude avec `/login`, désactive la connexion par abonnement pour cette session. Chaque demande transférée sous cet identifiant est facturée au compte derrière l'identifiant du fournisseur de la passerelle.

L'exception est de définir uniquement `ANTHROPIC_BASE_URL`, sans identifiant de passerelle. Les demandes s'acheminent toujours via la passerelle, mais une connexion claude.ai enregistrée reste l'identifiant actif, de sorte que les limites d'utilisation et la facturation de l'abonnement s'appliquent. [Autres passerelles LLM](/docs/fr/llm-gateway#subscriptions-and-gateways) couvre cette configuration et ce que la passerelle doit transférer pour que cela fonctionne.

<h2 id="configure-separately-from-the-gateway">
  Configurer séparément de la passerelle
</h2>

Une passerelle achemine les demandes d'API de modèle. Quelques éléments que vous pourriez vous attendre à ce qu'elle gère sont configurés ailleurs :

* **Quel modèle répond** : choisissez le modèle avec la commande `/model` ou les [variables d'environnement de modèle](/docs/fr/model-config#setting-your-model). La passerelle décide où les demandes vont, pas quel modèle le développeur sélectionne. La passerelle d'applications Claude peut limiter le choix avec une liste d'autorisation `availableModels` par groupe, mais le développeur choisit toujours dans celle-ci.
* **Autre trafic réseau** : Claude Code lui-même envoie les vérifications de version et les téléchargements directement à Anthropic, séparément du chemin de la passerelle. Le fait que le flux de télémétrie client optionnel soit également activé dépend de votre fournisseur ; le [tableau des valeurs par défaut de télémétrie](/docs/fr/data-usage#telemetry-services) couvre chaque cas. Sur une session de passerelle d'applications Claude signée, l'identifiant de passerelle désactive l'analyse liée à Anthropic et, lorsque le [transfert de télémétrie](/docs/fr/claude-apps-gateway-config#telemetry) est configuré, épingle l'export OTLP à la passerelle. Votre réseau a toujours besoin d'une sortie vers les [domaines requis](/docs/fr/network-config), ou définissez [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars) pour désactiver les flux optionnels.
* **Proxies HTTP d'entreprise** : un `HTTPS_PROXY` s'insère entre Claude Code et chaque serveur avec lequel il communique, y compris la passerelle. Si votre réseau en nécessite un, [configurez le proxy](/docs/fr/network-config) en plus de la passerelle. Pour une passerelle d'applications Claude que vous hébergez, [la connexion vérifie que l'hôte proxy est également sur un réseau privé](/docs/fr/claude-apps-gateway#prerequisites) ; si ce n'est pas le cas, ajoutez l'hôte de la passerelle à `NO_PROXY` pour que l'interface CLI s'y connecte directement.

<h2 id="next-steps">
  Étapes suivantes
</h2>

La page suivante dépend de qui exécute la passerelle. La passerelle d'Anthropic s'exécute à partir du binaire `claude` et a son propre guide de configuration ; une passerelle que votre organisation exécute déjà a un protocole à implémenter et une liste de contrôle du déploiement administrateur.

* [Claude apps gateway](/docs/fr/claude-apps-gateway) pour déployer la passerelle auto-hébergée d'Anthropic avec la connexion SSO et la télémétrie OTLP
* [Autres passerelles LLM](/docs/fr/llm-gateway) pour ce qu'une passerelle que votre organisation exécute déjà doit implémenter, et comment pointer Claude Code vers celle-ci
* [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup) pour les décisions de déploiement plus larges dont une passerelle est une partie
