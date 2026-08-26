> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utilisation des données

> Découvrez les politiques d'utilisation des données d'Anthropic pour Claude

<h2 id="data-policies">
  Politiques de données
</h2>

<h3 id="data-training-policy">
  Politique de formation aux données
</h3>

**Utilisateurs grand public (plans Free, Pro et Max)** :
Nous vous donnons le choix de permettre à vos données d'être utilisées pour améliorer les futurs modèles Claude. Nous formerons de nouveaux modèles en utilisant les données des comptes Free, Pro et Max lorsque ce paramètre est activé (y compris lorsque vous utilisez Claude Code à partir de ces comptes).

**Utilisateurs commerciaux** : (plans Team et Enterprise, API, plateformes tierces et Claude Gov) maintiennent les politiques existantes : Anthropic ne forme pas de modèles génératifs en utilisant le code ou les invites envoyés à Claude Code selon les conditions commerciales, sauf si le client a choisi de nous fournir ses données pour l'amélioration du modèle (par exemple, le [Programme de partenariat pour développeurs](https://support.claude.com/en/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Programme de partenariat pour développeurs
</h3>

Si vous acceptez explicitement les méthodes pour nous fournir des matériaux à utiliser pour la formation, comme via le [Programme de partenariat pour développeurs](https://support.claude.com/en/articles/11174108-about-the-development-partner-program), nous pouvons utiliser ces matériaux fournis pour former nos modèles. Un administrateur d'organisation peut accepter explicitement le Programme de partenariat pour développeurs pour son organisation. Notez que ce programme est disponible uniquement pour l'API Anthropic propriétaire, et non pour les utilisateurs d'Amazon Bedrock ou de la plateforme Agent de Google Cloud.

<h3 id="feedback-using-the-/feedback-command">
  Retours d'information à l'aide de la commande `/feedback`
</h3>

Si vous choisissez de nous envoyer des retours d'information sur Claude Code à l'aide de la commande `/feedback`, nous pouvons utiliser vos retours d'information pour améliorer nos produits et services. Les transcriptions partagées via `/feedback` sont conservées pendant 5 ans.

<h3 id="session-quality-surveys">
  Sondages de qualité de session
</h3>

Lorsque vous voyez l'invite « Comment Claude s'en sort-il cette session ? » dans Claude Code, répondre à ce sondage, y compris en sélectionnant « Ignorer », enregistre uniquement votre note. Nous ne collectons ni ne stockons de transcriptions de conversation, d'entrées, de sorties ou d'autres données de session dans le cadre de l'invite de notation elle-même. Contrairement aux retours d'information avec pouces vers le haut/bas ou aux rapports `/feedback`, ce sondage de qualité de session est une simple métrique de satisfaction du produit.

Après l'invite de notation, vous pouvez voir une question de suivi distincte demandant « Anthropic peut-il consulter votre transcription de session pour nous aider à améliorer Claude Code ? ». Ceci est une deuxième étape optionnelle distincte de la notation :

* **Oui** : télécharge votre transcription de conversation, toute transcription de sous-agent et le fichier journal de session brut du disque vers Anthropic. Les modèles de clé API et de jeton connus sont masqués avant le téléchargement. Le code source, le contenu des fichiers et tout autre contenu de conversation sont téléchargés tels quels. Les transcriptions partagées sont conservées jusqu'à 6 mois. Sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées, Oui écrit la même charge utile dans une archive locale sous `~/.claude/feedback-bundles/` au lieu de télécharger ; rien ne quitte votre machine jusqu'à ce que vous transmettiez ce fichier.
* **Non** : refuse sans rien envoyer
* **Ne plus demander** : refuse et arrête cette question de suivi d'apparaître dans les futures sessions

Rien n'est téléchargé à moins que vous ne sélectionniez explicitement **Oui**. Les organisations avec [conservation zéro des données](/docs/fr/zero-data-retention), ou où les retours d'information sur les produits sont désactivés par la politique de l'organisation, ou où `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` est défini, ne voient jamais cette question de suivi. Vos réponses à ce sondage, y compris les transcriptions de session soumises après l'invite de notation, n'affectent pas vos préférences de formation aux données et ne peuvent pas être utilisées pour former nos modèles d'IA.

Pour désactiver ces sondages, définissez `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. Le sondage est également désactivé lorsque `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` est défini. Les organisations qui bloquent le trafic non essentiel mais capturent les réponses aux sondages via leur propre [collecteur OpenTelemetry](/docs/fr/monitoring-usage) peuvent réactiver le sondage en définissant `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1`. Le sondage enregistre alors les notes uniquement au collecteur configuré. La question de suivi de partage de transcription et tout autre trafic de retours d'information lié à Anthropic restent désactivés. Pour contrôler la fréquence au lieu de désactiver, définissez [`feedbackSurveyRate`](/docs/fr/settings#available-settings) dans votre fichier de paramètres sur une probabilité entre `0` et `1`.

<h3 id="data-retention">
  Conservation des données
</h3>

Anthropic conserve les données de Claude Code en fonction de votre type de compte et de vos préférences.

**Utilisateurs grand public (plans Free, Pro et Max)** :

* Utilisateurs qui autorisent l'utilisation des données pour l'amélioration du modèle : période de conservation de 5 ans pour soutenir le développement et les améliorations de sécurité du modèle
* Utilisateurs qui n'autorisent pas l'utilisation des données pour l'amélioration du modèle : période de conservation de 30 jours
* Les paramètres de confidentialité peuvent être modifiés à tout moment sur [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls).

**Utilisateurs commerciaux (Team, Enterprise et API)** :

* Standard : période de conservation de 30 jours
* [Conservation zéro des données](/docs/fr/zero-data-retention) : disponible pour Claude Code sur Claude for Enterprise. La conservation zéro des données n'est pas incluse dans le plan Enterprise standard ; elle est activée par organisation après confirmation de l'admissibilité par votre équipe de compte
* Mise en cache locale : les clients Claude Code stockent les transcriptions de session localement en texte brut sous `~/.claude/projects/` pendant 30 jours par défaut pour permettre la reprise de session. Ajustez la période avec `cleanupPeriodDays`. Consultez [données d'application](/docs/fr/claude-directory#application-data) pour savoir ce qui est stocké et comment l'effacer.

Vous pouvez supprimer les sessions individuelles de Claude Code sur le web à tout moment. La suppression d'une session supprime définitivement les données d'événement de la session. Pour obtenir des instructions sur la suppression des sessions, consultez [Supprimer les sessions](/docs/fr/claude-code-on-the-web#delete-sessions).

Découvrez-en plus sur les pratiques de conservation des données dans notre [Centre de confidentialité](https://privacy.anthropic.com/).

Pour plus de détails, veuillez consulter nos [Conditions commerciales](https://www.anthropic.com/legal/commercial-terms) (pour les utilisateurs Team, Enterprise et API) ou [Conditions grand public](https://www.anthropic.com/legal/consumer-terms) (pour les utilisateurs Free, Pro et Max) et [Politique de confidentialité](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Accès aux données
</h2>

Pour tous les utilisateurs propriétaires, vous pouvez en savoir plus sur les données enregistrées pour [Claude Code local](#local-claude-code-data-flow-and-dependencies) et [Claude Code distant](#cloud-execution-data-flow-and-dependencies). Les sessions [Contrôle distant](/docs/fr/remote-control) suivent le flux de données local puisque toute l'exécution se fait sur votre machine ; pendant la connexion, la transcription de la session est également stockée sur les serveurs d'Anthropic pour synchroniser la conversation entre les appareils, comme décrit dans [Connexion et sécurité](/docs/fr/remote-control#connection-and-security). Notez que pour Claude Code distant, Claude accède au référentiel où vous lancez votre session Claude Code. Claude n'accède pas aux référentiels que vous avez connectés mais dans lesquels vous n'avez pas lancé de session.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Claude Code local : flux de données et dépendances
</h2>

Le diagramme ci-dessous montre comment Claude Code se connecte aux services externes lors de l'installation et du fonctionnement normal. Les lignes pleines indiquent les connexions requises, tandis que les lignes pointillées représentent les flux de données optionnels ou initiés par l'utilisateur.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagramme montrant les connexions externes de Claude Code : l'installation/mise à jour se connecte au serveur de distribution, et les demandes des utilisateurs se connectent à l'authentification Console et à l'API publique d'Anthropic, avec des flux de télémétrie optionnels transportant les métriques et les rapports d'erreurs vers Anthropic et les services tiers. Les commentaires envoyés avec /feedback vont vers Google Cloud Storage et créent optionnellement un problème GitHub" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code s'exécute localement. Pour interagir avec le LLM, Claude Code envoie des données sur le réseau. Ces données incluent tous les invites utilisateur et les sorties du modèle, chiffrées en transit via TLS 1.2+. Claude Code est compatible avec la plupart des VPN et proxies LLM populaires.

Le chiffrement au repos dépend de votre fournisseur de modèle :

| Fournisseur                   | Chiffrement au repos                                                                                                                                          |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic API                 | Chiffrement des disques au niveau de l'infrastructure (AES-256). Activez [Zero Data Retention](/docs/fr/zero-data-retention) pour aucune persistance côté serveur. |
| Amazon Bedrock                | AES-256 avec clés gérées par AWS. Clés gérées par le client disponibles via AWS KMS.                                                                          |
| Google Cloud's Agent Platform | Clés de chiffrement gérées par Google. CMEK disponible.                                                                                                       |
| Microsoft Foundry             | Les demandes sont acheminées vers l'infrastructure Anthropic avec chiffrement des disques AES-256.                                                            |

Claude Code est construit sur les API d'Anthropic. Pour plus de détails concernant les contrôles de sécurité de l'API, y compris les procédures de journalisation des API, consultez les artefacts de conformité dans le [Centre de confiance Anthropic](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Exécution cloud : flux de données et dépendances
</h3>

Lors de l'utilisation de [Claude Code sur le web](/docs/fr/claude-code-on-the-web), les sessions s'exécutent dans des machines virtuelles gérées par Anthropic au lieu de s'exécuter localement. Dans les environnements cloud :

* **Stockage du code et des données :** Votre référentiel est cloné sur une VM isolée. Le code et les données de session sont soumis aux politiques de conservation et d'utilisation pour votre type de compte (voir la section Conservation des données ci-dessus)
* **Identifiants :** L'authentification GitHub est gérée via un proxy sécurisé ; vos identifiants GitHub n'entrent jamais dans le bac à sable
* **Trafic réseau :** Tout le trafic sortant passe par un proxy de sécurité pour la journalisation d'audit et la prévention des abus
* **Données de session :** Les invites, les modifications de code et les sorties suivent les mêmes politiques de données que l'utilisation locale de Claude Code

Pour plus de détails sur la sécurité de l'exécution cloud, consultez [Sécurité](/docs/fr/security#cloud-execution-security).

<h2 id="telemetry-services">
  Services de télémétrie
</h2>

Claude Code envoie deux types de télémétrie opérationnelle : les métriques d'utilisation et les rapports d'erreurs. Vous pouvez désactiver chacun individuellement avec les variables d'environnement ci-dessous, ou désactiver tout le trafic non essentiel à la fois en définissant `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.

**Métriques** : latence, fiabilité et modèles d'utilisation, envoyés à Anthropic et à l'infrastructure de journalisation tierce via TLS. Les métriques n'incluent jamais votre code, vos invites ou vos chemins de fichiers. Définissez `DISABLE_TELEMETRY=1` pour refuser.

**Rapports d'erreurs** : messages d'erreur et traces de pile provenant des éléments internes de Claude Code, envoyés à un service de suivi des erreurs tiers via TLS. Claude Code masque les modèles connus de secrets, chemins de fichiers, adresses e-mail et autres informations personnelles avant que quoi que ce soit ne quitte votre machine. Définissez `DISABLE_ERROR_REPORTING=1` pour refuser.

La journalisation des erreurs n'est activée que lorsque tous ces éléments s'appliquent :

* vous vous connectez avec un abonnement Claude Pro ou Max
* vous exécutez Claude Code v2.1.198 ou version ultérieure
* vous vous connectez directement à l'API Claude
* votre organisation n'a pas d'accord de rétention zéro ou HIPAA

Lorsque vous exécutez la commande `/feedback`, une copie de votre historique de conversation complet, y compris le code, est envoyée à Anthropic. Avant de soumettre, vous choisissez la quantité d'historique à inclure : la session actuelle uniquement, qui est la valeur par défaut, ou également d'autres sessions du même projet au cours des 24 dernières heures ou 7 derniers jours. Les données sont chiffrées en transit via TLS et stockées dans Google Cloud Storage, qui chiffre les données stockées au repos par défaut. Optionnellement, un problème GitHub est créé dans le référentiel public. Pour refuser, définissez la variable d'environnement `DISABLE_FEEDBACK_COMMAND` sur `1`.

Lorsque vous utilisez un fournisseur tiers tel qu'Amazon Bedrock ou Google Cloud's Agent Platform, ou que vous n'avez pas d'identifiants Anthropic configurés, `/feedback` écrit le rapport dans une archive locale sous `~/.claude/feedback-bundles/` au lieu de l'envoyer à Anthropic. Les modèles de clé API et de jeton connus sont masqués avant que l'archive ne soit écrite. Rien ne quitte votre machine jusqu'à ce que vous envoyiez ce fichier à votre représentant du compte Anthropic ou que vous le joigniez à une demande d'assistance.

<h2 id="default-behaviors-by-api-provider">
  Comportements par défaut par fournisseur d'API
</h2>

Par défaut, les rapports d'erreurs, la télémétrie et les rapports de bogues sont désactivés lors de l'utilisation d'Amazon Bedrock, de Google Cloud's Agent Platform, de Microsoft Foundry ou de Claude Platform sur AWS. Les sondages de qualité de session et la vérification de sécurité du domaine WebFetch font exception et s'exécutent quel que soit le fournisseur. Sur une session de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectée, les analyses d'utilisation, les rapports d'erreurs et les évaluations de sondage à Anthropic sont désactivés par la passerelle elle-même, sans paramètre pour les réactiver. Vous pouvez refuser tout le trafic non essentiel, y compris les sondages, à la fois en définissant `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Cette variable n'affecte pas la vérification WebFetch, qui a sa propre option de refus. Voici les comportements par défaut complets :

| Service                                          | Claude API                                                                                                              | Google Cloud's Agent Platform API                                                                      | Amazon Bedrock API                                                                                     | Microsoft Foundry API                                                                                  | Claude Platform sur AWS                                                                                |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| **Métriques**                                    | Activé par défaut.<br />`DISABLE_TELEMETRY=1` pour désactiver.                                                          | Désactivé par défaut.<br />`CLAUDE_CODE_USE_VERTEX` doit être 1.                                       | Désactivé par défaut.<br />`CLAUDE_CODE_USE_BEDROCK` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_FOUNDRY` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` doit être 1.                                |
| **Rapports d'erreurs**                           | Activé pour les connexions Pro et Max sur v2.1.198+, sinon désactivé.<br />`DISABLE_ERROR_REPORTING=1` pour désactiver. | Désactivé par défaut.<br />`CLAUDE_CODE_USE_VERTEX` doit être 1.                                       | Désactivé par défaut.<br />`CLAUDE_CODE_USE_BEDROCK` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_FOUNDRY` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` doit être 1.                                |
| **Claude API (rapports `/feedback`)**            | Activé par défaut.<br />`DISABLE_FEEDBACK_COMMAND=1` pour désactiver.                                                   | Désactivé par défaut.<br />`CLAUDE_CODE_USE_VERTEX` doit être 1.                                       | Désactivé par défaut.<br />`CLAUDE_CODE_USE_BEDROCK` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_FOUNDRY` doit être 1.                                      | Désactivé par défaut.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` doit être 1.                                |
| **Sondages de qualité de session**               | Activé par défaut.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` pour désactiver.                                        | Activé par défaut.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` pour désactiver.                       | Activé par défaut.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` pour désactiver.                       | Activé par défaut.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` pour désactiver.                       | Activé par défaut.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` pour désactiver.                       |
| **Vérification de sécurité du domaine WebFetch** | Activé par défaut.<br />`skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings) pour désactiver.                  | Activé par défaut.<br />`skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings) pour désactiver. | Activé par défaut.<br />`skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings) pour désactiver. | Activé par défaut.<br />`skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings) pour désactiver. | Activé par défaut.<br />`skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings) pour désactiver. |

Toutes les variables d'environnement peuvent être vérifiées dans `settings.json` (voir [référence des paramètres](/docs/fr/settings)).

À partir de la v2.1.126, lorsqu'une plateforme hôte définit `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`, les métriques sont activées par défaut pour Google Cloud's Agent Platform, Amazon Bedrock et Microsoft Foundry, et suivent l'option de refus standard `DISABLE_TELEMETRY`. Les rapports d'erreurs et les rapports `/feedback` restent désactivés par défaut sur ces fournisseurs.

<h3 id="webfetch-domain-safety-check">
  Vérification de sécurité du domaine WebFetch
</h3>

Avant de récupérer une URL, l'outil WebFetch envoie le nom d'hôte demandé à `api.anthropic.com` pour le vérifier par rapport à une liste de blocage de sécurité maintenue par Anthropic. Seul le nom d'hôte est envoyé, pas l'URL complète, le chemin ou le contenu de la page. Les résultats sont mis en cache par nom d'hôte pendant cinq minutes.

Cette vérification s'exécute quel que soit le fournisseur de modèle que vous utilisez et n'est pas affectée par `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Si votre réseau bloque `api.anthropic.com`, les demandes WebFetch échouent jusqu'à ce que vous autorisiez le domaine ou définissiez `skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings). La désactivation de la vérification signifie que WebFetch tente de récupérer n'importe quelle URL sans consulter la liste de blocage, donc combinez-la avec les [règles de permission `WebFetch`](/docs/fr/permissions#webfetch) si vous devez restreindre les domaines que Claude peut atteindre.
