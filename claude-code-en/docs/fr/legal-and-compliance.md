> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aspects juridiques et conformité

> Accords juridiques, certifications de conformité et informations de sécurité pour Claude Code.

<h2 id="legal-agreements">
  Accords juridiques
</h2>

<h3 id="license">
  Licence
</h3>

Votre utilisation de Claude Code est soumise à :

* [Conditions commerciales](https://www.anthropic.com/legal/commercial-terms) - pour les utilisateurs Team, Enterprise et Claude API
* [Conditions d'utilisation pour les consommateurs](https://www.anthropic.com/legal/consumer-terms) - pour les utilisateurs Free, Pro et Max

<h3 id="commercial-agreements">
  Accords commerciaux
</h3>

Que vous utilisiez l'API Claude directement (1P) ou y accédiez via Amazon Bedrock ou Google Cloud's Agent Platform (3P), votre accord commercial existant s'appliquera à l'utilisation de Claude Code, sauf si nous avons convenu autrement.

<h3 id="can-customers-offer-claude-code-in-their-products">
  Les clients peuvent-ils proposer Claude Code dans leurs produits ?
</h3>

Sauf si nous avons convenu autrement, la préinstallation ou l'exécution de Claude Code dans vos produits ou services (par exemple, dans des sandboxes hébergés ou d'autres infrastructures d'agent) nécessite d'accepter nos [Conditions commerciales](https://www.anthropic.com/legal/commercial-terms) et de respecter les conditions ci-dessous :

* **Le binaire Claude Code ne doit pas être modifié.** Claude Code doit être installé et exécuté tel que publié par Anthropic, et les clients ne peuvent pas supprimer, désactiver ou restreindre aucune méthode d'authentification intégrée (y compris les méthodes qui permettent de se connecter avec un compte Claude ou la clé API propre de l'utilisateur).
* **Les clients ne peuvent pas payer, revendre ou intermédiaire l'utilisation de Claude au nom de leurs utilisateurs finaux.** Chaque utilisateur final doit s'authentifier avec sa propre clé API Anthropic, ses identifiants de plan d'abonnement Claude ou ses identifiants de fournisseur d'inférence tiers (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry). Cet usage est facturé directement à l'utilisateur final selon son propre accord avec Anthropic ou, pour les fournisseurs d'inférence tiers, avec le fournisseur applicable.

**Utilisation du nom et du logo Claude Code.** Vous pouvez dire avec précision, en texte brut, que votre produit a Claude Code préinstallé ou qu'il exécute Claude Code. Mais vous ne pouvez pas utiliser les noms ou logos Claude Code ou Anthropic comme partie de votre propre nom de produit, de fonctionnalité ou d'entreprise, dans votre propre logo, ou d'une manière qui suggère qu'Anthropic a construit, approuvé ou est partenaire de votre produit. Tout autre usage des noms ou logos d'Anthropic est régi par nos [Directives relatives aux marques](https://www.anthropic.com/legal/trademark-guidelines) et nécessite notre permission écrite.

Claude Code reste régi par les conditions standard d'Anthropic (voir les sections Licence et Accords commerciaux ci-dessus) indépendamment de la plateforme par laquelle il est accédé.

<h2 id="compliance">
  Conformité
</h2>

<h3 id="healthcare-compliance-baa">
  Conformité aux normes de santé (BAA)
</h3>

Si un client a exécuté un accord d'associé commercial (BAA) avec Anthropic et a activé la [rétention zéro des données (ZDR)](/docs/fr/zero-data-retention) pour l'organisation concernée, ce BAA s'étend au trafic API du client via Claude Code.

<h2 id="usage-policy">
  Politique d'utilisation
</h2>

<h3 id="acceptable-use">
  Utilisation acceptable
</h3>

L'utilisation de Claude Code est soumise à la [politique d'utilisation d'Anthropic](https://www.anthropic.com/legal/aup). Les limites d'utilisation annoncées pour les plans Pro et Max supposent une utilisation ordinaire et individuelle de Claude Code et du SDK Agent.

<h3 id="authentication-and-credential-use">
  Authentification et utilisation des identifiants
</h3>

Claude Code s'authentifie auprès des serveurs d'Anthropic en utilisant des jetons OAuth ou des clés API. Ces méthodes d'authentification servent des objectifs différents :

* **L'authentification OAuth** est destinée exclusivement aux acheteurs des plans d'abonnement Claude Free, Pro, Max, Team et Enterprise et est conçue pour soutenir l'utilisation ordinaire de Claude Code et d'autres applications natives d'Anthropic. Pour les étapes de connexion, consultez [Connexion à votre compte Claude](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account) ; pour savoir comment Claude Code effectue l'authentification OAuth, consultez [Authentification](/docs/fr/authentication).
* **Les développeurs** créant des produits ou services qui interagissent avec les capacités de Claude, y compris ceux utilisant le [SDK Agent](/docs/fr/agent-sdk/overview), doivent utiliser l'authentification par clé API via la [Console Claude](https://platform.claude.com/) ou un fournisseur cloud pris en charge. Anthropic n'autorise pas les développeurs tiers à proposer la connexion Claude.ai dans leurs propres applications, ou à acheminer les demandes via les identifiants des plans Free, Pro ou Max au nom de leurs utilisateurs. De plus, les développeurs ne peuvent pas collecter, stocker ou intermédiaire les identifiants Claude.ai ou les jetons de session — la connexion à un compte Claude doit s'effectuer via le flux propre d'Anthropic.

Cela ne restreint pas la façon dont les clients provisionnent et gèrent leurs propres clés API ou les identifiants des fournisseurs d'inférence tiers — par exemple, configurer une clé API dans un environnement de développement, un gestionnaire de secrets, ou une image machine pour utilisation par les utilisateurs autorisés du client — à condition que l'utilisation résultante soit facturée au propriétaire de la clé selon son accord avec Anthropic (ou le fournisseur applicable) et ne soit pas revendue ou intermédiée comme décrit ci-dessus. Cela n'empêche pas non plus un utilisateur final de se connecter au binaire Claude Code non modifié avec son propre abonnement Claude, y compris lorsqu'une plateforme héberge Claude Code comme décrit sous *Les clients peuvent-ils proposer Claude Code dans leurs produits ?* ci-dessus.

Anthropic se réserve le droit de prendre des mesures pour appliquer ces restrictions et peut le faire sans préavis.

Pour des questions sur les méthodes d'authentification autorisées pour votre cas d'utilisation, veuillez [contacter l'équipe commerciale](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Sécurité et confiance
</h2>

<h3 id="trust-and-safety">
  Confiance et sécurité
</h3>

Vous pouvez trouver plus d'informations dans le [Centre de confiance d'Anthropic](https://trust.anthropic.com) et le [Hub de transparence](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Signalement des vulnérabilités de sécurité
</h3>

Anthropic gère notre programme de sécurité via HackerOne. [Utilisez ce formulaire pour signaler les vulnérabilités](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Tous droits réservés. L'utilisation est soumise aux conditions d'utilisation applicables d'Anthropic.
