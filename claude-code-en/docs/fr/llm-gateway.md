> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autres passerelles LLM

> Acheminez Claude Code via une passerelle LLM que votre organisation exécute déjà. Couvre la connexion de Claude Code à une passerelle, le déploiement d'une passerelle pour votre organisation et ce que Claude Code envoie à une passerelle.

Cette section couvre l'utilisation d'un produit de passerelle que votre organisation exécute déjà, plutôt que la [passerelle Claude apps](/docs/fr/claude-apps-gateway). Pour savoir ce qu'est une passerelle, comment elle se situe entre Claude Code et votre fournisseur, et comment choisir entre la passerelle Claude apps et un autre produit, consultez l'[aperçu des passerelles](/docs/fr/gateways).

<Note>
  * Si vous êtes un développeur se connectant à une passerelle existante : [connectez Claude Code à votre passerelle](/docs/fr/llm-gateway-connect)
  * Si vous êtes un administrateur déployant une passerelle pour votre organisation : [déployez et distribuez une passerelle](/docs/fr/llm-gateway-rollout)
  * Si vous configurez un produit de passerelle : la [référence du protocole de passerelle](/docs/fr/llm-gateway-protocol)
</Note>

Toute passerelle qui expose un [format API supporté](/docs/fr/llm-gateway-protocol#api-formats) fonctionne. Anthropic n'approuve pas, ne maintient pas et n'audite pas les produits de passerelle tiers, et ne supporte pas l'acheminement de Claude Code vers des modèles non-Claude via aucune passerelle. Déployez la passerelle en suivant sa propre documentation, puis complétez le côté Claude Code avec les [étapes de déploiement ci-dessous](#roll-out-a-gateway).

<h2 id="what-a-gateway-provides">
  Ce qu'une passerelle fournit
</h2>

Une passerelle donne à votre organisation un seul endroit pour gérer :

* **Credentials** : la clé du fournisseur reste côté serveur ; les développeurs détiennent plutôt des credentials de passerelle
* **Suivi de l'utilisation** : attribuez l'utilisation par développeur ou équipe, quel que soit le fournisseur qui traite la requête
* **Contrôles des coûts** : appliquez les budgets et les limites de débit en un seul endroit
* **Journalisation d'audit** : enregistrez chaque requête de modèle pour la conformité
* **Changement de fournisseur** : changez le fournisseur dans la configuration de la passerelle, sans toucher aux machines des développeurs

Tous ces éléments sauf le changement de fournisseur s'appliquent que le flux amont soit l'API d'Anthropic ou un [fournisseur cloud](/docs/fr/third-party-integrations). Le changement de fournisseur sans reconfiguration des machines des développeurs dépend également de la passerelle exposant un seul [point de terminaison au format Anthropic](/docs/fr/llm-gateway-protocol#api-formats) quel que soit le flux amont ; une passerelle qui expose le propre format d'un fournisseur lie la configuration du client à ce fournisseur.

Le compromis est que la passerelle devient une infrastructure que votre organisation exploite. Claude Code ajoute des capacités à chaque version, et une passerelle qui ne les transfère pas casse les fonctionnalités correspondantes, donc le produit de passerelle doit être maintenu à jour à mesure que Claude Code évolue. La [référence du protocole de passerelle](/docs/fr/llm-gateway-protocol) couvre ce qu'il faut transférer.

<h2 id="roll-out-a-gateway">
  Déployer une passerelle
</h2>

Quand vous êtes prêt à déployer une passerelle LLM pour votre organisation, la séquence est la même quel que soit le produit de passerelle que vous choisissez :

1. Déployez la passerelle et donnez-lui votre credential du fournisseur, afin qu'elle puisse authentifier les requêtes qu'elle transfère.
2. Émettez à chaque développeur une credential de passerelle, afin que l'utilisation soit attribuée au développeur et que l'offboarding révoque une credential.
3. Distribuez la configuration via un [fichier de paramètres gérés](/docs/fr/settings#settings-files) et votre outillage de secrets, afin que chaque machine reçoive l'URL de base et une credential. Quand les deux sont distribués, les développeurs ne configurent rien. Si vous n'avez pas de distribution de paramètres en place, les développeurs suivent la [page de connexion](/docs/fr/llm-gateway-connect) pour définir les variables eux-mêmes.
4. Faites en sorte que chaque développeur [vérifie la configuration dans Claude Code](/docs/fr/llm-gateway-connect#check-for-an-existing-configuration), afin que les problèmes de distribution fassent surface avant qu'ils ne dépendent de la passerelle.

[Déployez une passerelle LLM pour votre organisation](/docs/fr/llm-gateway-rollout) parcourt chaque étape et montre les fichiers de configuration à distribuer à chacune. La passerelle est une partie de la configuration de l'organisation ; pour l'application des politiques, la visibilité de l'utilisation et les décisions de traitement des données, consultez [Configurez Claude Code pour votre organisation](/docs/fr/admin-setup).

<h2 id="subscriptions-and-gateways">
  Abonnements et passerelles
</h2>

Tandis qu'une [variable de credential de passerelle](/docs/fr/llm-gateway-connect#set-the-credential-variable) ou `apiKeyHelper` est active, l'abonnement claude.ai d'un développeur n'est pas utilisé : la credential remplace la connexion à l'abonnement pour cette session, et les limites d'utilisation de l'abonnement ne s'appliquent pas. Ce trafic est facturé par token à celui qui possède la credential que la passerelle transfère, tel que le compte Anthropic Console de votre organisation, ou votre compte Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry quand la passerelle achemine là.

[`ANTHROPIC_BASE_URL`](/docs/fr/llm-gateway-connect#set-the-base-url-and-credential) est la variable qui pointe Claude Code vers la passerelle. Définir uniquement cette variable, sans credential de passerelle, ne remplace pas l'abonnement. Les requêtes acheminent toujours via la passerelle, mais une connexion claude.ai sauvegardée reste la credential active, donc ses limites d'utilisation et sa facturation s'appliquent. Les passerelles qui transmettent ce trafic à Anthropic doivent transférer la capacité OAuth dans `anthropic-beta` ; consultez la [référence des en-têtes de requête](/docs/fr/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Pages connexes
</h2>

* [Aperçu des passerelles](/docs/fr/gateways) : comment une passerelle fonctionne et comment choisir entre la passerelle Claude apps et un autre produit
* [Passerelle Claude apps](/docs/fr/claude-apps-gateway) : la passerelle auto-hébergée d'Anthropic avec connexion SSO et télémétrie OTLP
* [Connectez Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) : définissez l'URL de base et la credential sur votre propre machine, avec la configuration par surface et un tableau de dépannage
* [Déployez une passerelle LLM pour votre organisation](/docs/fr/llm-gateway-rollout) : la liste de contrôle de l'administrateur pour déployer une passerelle, émettre des credentials de développeur et distribuer les paramètres gérés
* [Référence du protocole de passerelle](/docs/fr/llm-gateway-protocol) : ce que Claude Code envoie à une passerelle, pour les opérateurs en configurant une, couvrant les points de terminaison, les en-têtes à transférer et le passage des fonctionnalités
