> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sécurité

> Découvrez les protections de sécurité de Claude Code et les meilleures pratiques pour une utilisation sûre.

<h2 id="how-we-approach-security">
  Comment nous abordons la sécurité
</h2>

<h3 id="security-foundation">
  Fondation de sécurité
</h3>

La sécurité de votre code est primordiale. Claude Code est construit avec la sécurité au cœur, développé selon le programme de sécurité complet d'Anthropic. En savoir plus et accéder aux ressources (rapport SOC 2 Type 2, certificat ISO 27001, etc.) sur le [Centre de confiance Anthropic](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Architecture basée sur les permissions
</h3>

Claude Code utilise des permissions strictes en lecture seule par défaut. Lorsque des actions supplémentaires sont nécessaires (édition de fichiers, exécution de tests, exécution de commandes), Claude Code demande une permission explicite. Les utilisateurs contrôlent s'il faut approuver les actions une seule fois ou les autoriser automatiquement.

Claude Code nécessite une approbation avant d'exécuter les commandes Bash qui peuvent modifier votre système. Un ensemble intégré de [commandes en lecture seule](/docs/fr/permissions#read-only-commands) telles que `ls`, `cat` et `git status` s'exécute sans invite. Cette approche permet aux utilisateurs et aux organisations de configurer les permissions directement.

Pour une configuration détaillée des permissions, consultez [Permissions](/docs/fr/permissions).

<h3 id="built-in-protections">
  Protections intégrées
</h3>

Pour atténuer les risques dans les systèmes agentiques :

* **Outil bash en sandbox** : [Sandbox](/docs/fr/sandboxing) les commandes bash avec isolation du système de fichiers et du réseau, réduisant les invites de permission tout en maintenant la sécurité. Activez avec `/sandbox` pour définir les limites où Claude Code peut travailler de manière autonome
* **Limite du répertoire de travail** : Claude Code ne peut écrire que dans le dossier où il a été démarré et ses sous-dossiers, et ne peut pas modifier les fichiers dans les répertoires parents sans permission explicite. La lecture de chemins en dehors de cette limite avec les outils Read, Grep et Glob est possible après une invite d'approbation. Étendez la limite avec des [répertoires supplémentaires](/docs/fr/permissions#working-directories) pour ignorer l'invite, ou restreignez l'accès en lecture plus large disponible pour les commandes Bash en lecture seule avec les [règles `denyRead` du sandbox](/docs/fr/sandboxing#filesystem-isolation), qui s'appliquent uniquement lorsque le sandboxing est activé
* **Atténuation de la fatigue des invites** : Support pour la liste blanche des commandes sûres fréquemment utilisées par utilisateur, par base de code ou par organisation
* **Mode Accepter les modifications** : Accepte automatiquement les modifications de fichiers et un ensemble fixe de commandes Bash du système de fichiers comme `mkdir`, `touch`, `rm`, `mv`, `cp` et `sed` pour les chemins du répertoire de travail. Les autres commandes Bash et les chemins hors de la portée continuent à afficher des invites

<h3 id="user-responsibility">
  Responsabilité de l'utilisateur
</h3>

Claude Code n'a que les permissions que vous lui accordez. Vous êtes responsable de l'examen du code et des commandes proposés pour la sécurité avant approbation.

<h2 id="protect-against-prompt-injection">
  Protégez-vous contre l'injection de prompt
</h2>

L'injection de prompt est une technique où un attaquant tente de contourner ou de manipuler les instructions d'un assistant IA en insérant du texte malveillant. Claude Code inclut plusieurs protections contre ces attaques :

<h3 id="core-protections">
  Protections principales
</h3>

* **Système de permissions** : Les opérations sensibles nécessitent une approbation explicite
* **Analyse contextuelle** : Détecte les instructions potentiellement nuisibles en analysant la demande complète
* **Assainissement des entrées** : Prévient l'injection de commandes en traitant les entrées utilisateur
* **Approbation des commandes réseau** : Les commandes qui récupèrent du contenu sur le web comme `curl` et `wget` ne sont pas approuvées automatiquement par défaut. Elles demandent une approbation comme n'importe quelle autre commande Bash non en lecture seule, vous pouvez donc toujours approuver une fois ou ajouter une règle d'autorisation explicite comme `Bash(curl *)`. Pour les bloquer entièrement, ajoutez-les à [`permissions.deny`](/docs/fr/permissions#tool-specific-permission-rules)

<h3 id="privacy-safeguards">
  Protections de la vie privée
</h3>

Nous avons mis en place plusieurs protections pour protéger vos données, notamment :

* Périodes de rétention limitées pour les informations sensibles (consultez le [Centre de confidentialité](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) pour en savoir plus)
* Accès restreint aux données de session utilisateur
* Contrôle utilisateur sur les préférences de formation des données. Les utilisateurs consommateurs peuvent modifier leurs [paramètres de confidentialité](https://claude.ai/settings/privacy) à tout moment.

Pour plus de détails, veuillez consulter nos [Conditions commerciales](https://www.anthropic.com/legal/commercial-terms) (pour les utilisateurs Team, Enterprise et API) ou [Conditions pour les consommateurs](https://www.anthropic.com/legal/consumer-terms) (pour les utilisateurs Free, Pro et Max) et [Politique de confidentialité](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Protections supplémentaires
</h3>

* **Approbation des demandes réseau** : Les outils qui effectuent des demandes réseau nécessitent une approbation utilisateur par défaut
* **Fenêtres de contexte isolées** : Web fetch utilise une fenêtre de contexte séparée pour éviter d'injecter des prompts potentiellement malveillants
* **Vérification de confiance** : Les premières exécutions de base de code et les nouveaux serveurs MCP nécessitent une vérification de confiance
  * Remarque : La vérification de confiance est désactivée lors de l'exécution non-interactive avec le drapeau `-p`
  * Remarque : Lorsque vous démarrez Claude Code directement dans votre répertoire personnel, l'acceptation de la confiance est conservée pour la session actuelle uniquement et n'est pas écrite sur le disque, donc l'invite réapparaît à chaque lancement. Il n'y a aucun paramètre pour la conserver. Démarrez Claude Code à partir d'un sous-répertoire de projet à la place, où l'acceptation de la confiance est enregistrée par répertoire
* **Détection d'injection de commande** : Les commandes bash suspectes nécessitent une approbation manuelle même si elles ont été précédemment autorisées
* **Correspondance en cas d'échec fermé** : Les commandes non appariées par défaut nécessitent une approbation manuelle
* **Descriptions en langage naturel** : Les commandes bash complexes incluent des explications pour la compréhension de l'utilisateur
* **Stockage sécurisé des identifiants** : Les clés API et les tokens sont stockés dans le Keychain macOS lorsqu'il est disponible, et protégés par les permissions de fichiers sur Windows et Linux. Consultez [Gestion des identifiants](/docs/fr/authentication#credential-management)

<Warning>
  **Risque de sécurité WebDAV Windows** : Lors de l'exécution de Claude Code sur Windows, nous recommandons de ne pas activer WebDAV ou de permettre à Claude Code d'accéder à des chemins tels que `\\*` qui peuvent contenir des sous-répertoires WebDAV. [WebDAV a été déprécié par Microsoft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) en raison de risques de sécurité. L'activation de WebDAV peut permettre à Claude Code de déclencher des demandes réseau vers des hôtes distants, contournant le système de permissions.
</Warning>

**Meilleures pratiques pour travailler avec du contenu non fiable** :

1. Examinez les commandes suggérées avant approbation
2. Évitez de diriger le contenu non fiable directement vers Claude
3. Vérifiez les modifications proposées aux fichiers critiques
4. Utilisez des machines virtuelles (VM) pour exécuter des scripts et effectuer des appels d'outils, en particulier lors de l'interaction avec des services web externes
5. Signalez les comportements suspects avec `/feedback`

<Warning>
  Bien que ces protections réduisent considérablement les risques, aucun système n'est complètement
  immunisé contre toutes les attaques. Maintenez toujours de bonnes pratiques de sécurité lors du travail
  avec n'importe quel outil IA.
</Warning>

<h2 id="mcp-security">
  Sécurité MCP
</h2>

Claude Code permet aux utilisateurs de configurer les serveurs Model Context Protocol (MCP). La liste des serveurs MCP autorisés est configurée dans votre code source, dans le cadre des paramètres Claude Code que les ingénieurs enregistrent dans le contrôle de source.

Nous vous encourageons à écrire vos propres serveurs MCP ou à utiliser des serveurs MCP de fournisseurs en qui vous avez confiance. Vous pouvez configurer les permissions Claude Code pour les serveurs MCP. Anthropic examine les connecteurs par rapport à ses [critères d'examen](https://claude.com/docs/connectors/building/review-criteria) avant de les ajouter au [Répertoire Anthropic](https://claude.ai/directory), mais n'effectue pas d'audit de sécurité ni ne gère aucun serveur MCP.

<h2 id="ide-security">
  Sécurité IDE
</h2>

Consultez [Sécurité et confidentialité VS Code](/docs/fr/vs-code#security-and-privacy) pour plus d'informations sur l'exécution de Claude Code dans un IDE.

<h2 id="cloud-execution-security">
  Sécurité de l'exécution cloud
</h2>

Lors de l'utilisation de [Claude Code sur le web](/docs/fr/claude-code-on-the-web), des contrôles de sécurité supplémentaires sont en place :

* **Machines virtuelles isolées** : Chaque session cloud s'exécute dans une VM isolée gérée par Anthropic
* **Contrôles d'accès réseau** : L'accès réseau est limité par défaut et peut être configuré pour être désactivé ou autoriser uniquement des domaines spécifiques
* **Protection des identifiants** : L'authentification est gérée via un proxy sécurisé qui utilise un identifiant limité à l'intérieur du sandbox, qui est ensuite traduit en votre jeton d'authentification GitHub réel
* **Restrictions de branche** : Les opérations de push Git sont limitées à la branche de travail actuelle
* **Journalisation d'audit** : Toutes les opérations dans les environnements cloud sont enregistrées à des fins de conformité et d'audit
* **Nettoyage automatique** : Les environnements cloud sont automatiquement terminés après la fin de la session

Pour plus de détails sur l'exécution cloud, consultez [Claude Code sur le web](/docs/fr/claude-code-on-the-web).

Les sessions de [Contrôle à distance](/docs/fr/remote-control) fonctionnent différemment : l'interface web se connecte à un processus Claude Code s'exécutant sur votre machine locale. Toute l'exécution du code et l'accès aux fichiers restent locaux, et le trafic de session transite par l'API Anthropic via TLS ; une fois connecté, la transcription de la session est stockée sur les serveurs Anthropic pour synchroniser la conversation entre les appareils, comme décrit dans [Connexion et sécurité](/docs/fr/remote-control#connection-and-security). Aucune VM cloud ou sandbox n'est impliquée. La connexion utilise plusieurs identifiants de courte durée et à portée étroite, chacun limité à un objectif spécifique et expirant indépendamment, pour limiter le rayon d'explosion de tout identifiant compromis unique.

<h2 id="security-best-practices">
  Meilleures pratiques de sécurité
</h2>

<h3 id="working-with-sensitive-code">
  Travail avec du code sensible
</h3>

* Examinez toutes les modifications suggérées avant approbation
* Utilisez les paramètres de permission spécifiques au projet pour les référentiels sensibles
* Envisagez d'utiliser les [devcontainers](/docs/fr/devcontainer) pour une isolation supplémentaire
* Auditez régulièrement vos paramètres de permission avec `/permissions`

<h3 id="team-security">
  Sécurité d'équipe
</h3>

* Utilisez les [paramètres gérés](/docs/fr/settings#settings-files) pour appliquer les normes organisationnelles
* Partagez les configurations de permission approuvées via le contrôle de source
* Formez les membres de l'équipe aux meilleures pratiques de sécurité
* Surveillez l'utilisation de Claude Code via les [métriques OpenTelemetry](/docs/fr/monitoring-usage)
* Auditez ou bloquez les modifications de paramètres pendant les sessions avec les [hooks `ConfigChange`](/docs/fr/hooks#configchange)

<h3 id="reporting-security-issues">
  Signalement des problèmes de sécurité
</h3>

Si vous découvrez une vulnérabilité de sécurité dans Claude Code :

1. Ne la divulguez pas publiquement
2. Signalez-la via notre [programme HackerOne](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)
3. Incluez les étapes de reproduction détaillées
4. Accordez-nous du temps pour résoudre le problème avant la divulgation publique

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Plugin de conseils en sécurité](/docs/fr/security-guidance) : faire examiner et corriger par Claude les vulnérabilités dans ses propres modifications de code pendant la session
* [Environnements sandbox](/docs/fr/sandbox-environments) : comparer les approches d'isolation et en choisir une pour votre modèle de menace
* [Sandboxing](/docs/fr/sandboxing) : isolation du système de fichiers et du réseau pour les commandes Bash
* [Permissions](/docs/fr/permissions) : configurer les permissions et les contrôles d'accès
* [Surveillance de l'utilisation](/docs/fr/monitoring-usage) : suivre et auditer l'activité Claude Code
* [Conteneurs de développement](/docs/fr/devcontainer) : environnements sécurisés et isolés
* [Centre de confiance Anthropic](https://trust.anthropic.com) : certifications de sécurité et conformité
