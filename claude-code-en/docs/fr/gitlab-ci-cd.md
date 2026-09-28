> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Découvrez comment intégrer Claude Code dans votre flux de travail de développement avec GitLab CI/CD

<Info>
  Claude Code pour GitLab CI/CD est actuellement en bêta. Les fonctionnalités et les capacités peuvent évoluer au fur et à mesure que nous affinons l'expérience.

  Cette intégration est maintenue par GitLab. Pour obtenir de l'aide, consultez le [problème GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776) suivant.
</Info>

<Note>
  Cette intégration est construite sur la base de [Claude Code CLI et Agent SDK](/docs/fr/agent-sdk/overview), permettant l'utilisation programmatique de Claude dans vos tâches CI/CD et vos flux de travail d'automatisation personnalisés.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Pourquoi utiliser Claude Code avec GitLab ?
</h2>

* **Création instantanée de MR** : Décrivez ce dont vous avez besoin, et Claude propose une MR complète avec les modifications et une explication
* **Implémentation automatisée** : Transformez les problèmes en code fonctionnel avec une seule commande ou mention
* **Conscient du projet** : Claude suit vos directives `CLAUDE.md` et les modèles de code existants
* **Configuration simple** : Ajoutez une tâche à `.gitlab-ci.yml` et une variable CI/CD masquée
* **Prêt pour l'entreprise** : Choisissez Claude API, Amazon Bedrock ou Google Cloud's Agent Platform pour répondre aux besoins de résidence des données et d'approvisionnement
* **Sécurisé par défaut** : S'exécute dans vos exécuteurs GitLab avec votre protection de branche et vos approbations

<h2 id="how-it-works">
  Comment ça marche
</h2>

Claude Code utilise GitLab CI/CD pour exécuter des tâches d'IA dans des tâches isolées et valider les résultats via des MR :

1. **Orchestration basée sur les événements** : GitLab écoute les déclencheurs que vous choisissez (par exemple, un commentaire qui mentionne `@claude` dans un problème, une MR ou un fil de discussion). La tâche collecte le contexte du fil et du référentiel, construit des invites à partir de cette entrée et exécute Claude Code.

2. **Abstraction du fournisseur** : Utilisez le fournisseur qui correspond à votre environnement :
   * Claude API (SaaS)
   * Amazon Bedrock (accès basé sur IAM, options multi-régions)
   * Google Cloud's Agent Platform (natif GCP, Workload Identity Federation)

3. **Exécution en bac à sable** : Chaque interaction s'exécute dans un conteneur avec des règles strictes de réseau et de système de fichiers. Claude Code applique des autorisations limitées à l'espace de travail pour limiter les écritures. Chaque modification passe par une MR afin que les examinateurs voient la différence et que les approbations s'appliquent toujours.

Choisissez des points de terminaison régionaux pour réduire la latence et respecter les exigences de souveraineté des données tout en utilisant les accords cloud existants.

<h2 id="what-can-claude-do">
  Que peut faire Claude ?
</h2>

Dans un pipeline GitLab, Claude Code peut :

* Créer et mettre à jour des MR à partir de descriptions ou de commentaires de problèmes
* Analyser les régressions de performance et proposer des optimisations
* Implémenter des fonctionnalités directement dans une branche, puis ouvrir une MR
* Corriger les bogues et les régressions identifiés par les tests ou les commentaires
* Répondre aux commentaires de suivi pour itérer sur les modifications demandées

<h2 id="setup">
  Configuration
</h2>

<h3 id="quick-setup">
  Configuration rapide
</h3>

Le moyen le plus rapide de commencer est d'ajouter un travail minimal à votre `.gitlab-ci.yml` et de définir votre clé API en tant que variable masquée.

1. **Ajouter une variable CI/CD masquée**
   * Allez à **Paramètres** → **CI/CD** → **Variables**
   * Ajoutez `ANTHROPIC_API_KEY` (masquée, protégée selon les besoins)

2. **Ajouter un travail Claude à `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Adjust rules to fit how you want to trigger the job:
  # - manual runs
  # - merge request events
  # - web/API triggers when a comment contains '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Optional: start a GitLab MCP server if your setup provides one
    - /bin/gitlab-mcp-server || true
    # Use AI_FLOW_* variables when invoking via web/API triggers with context payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Après avoir ajouté le travail et votre variable `ANTHROPIC_API_KEY`, testez en exécutant le travail manuellement à partir de **CI/CD** → **Pipelines**, ou déclenchez-le à partir d'une MR pour laisser Claude proposer des mises à jour dans une branche et ouvrir une MR si nécessaire.

<Note>
  Pour exécuter sur Amazon Bedrock ou la plateforme Agent de Google Cloud au lieu de l'API Claude, consultez la section [Utilisation avec Amazon Bedrock et Google Cloud](#using-with-amazon-bedrock-and-google-cloud) ci-dessous pour la configuration de l'authentification et de l'environnement.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configuration manuelle (recommandée pour la production)
</h3>

Si vous préférez une configuration plus contrôlée ou si vous avez besoin de fournisseurs d'entreprise :

1. **Configurer l'accès au fournisseur** :
   * **Claude API** : Créez et stockez `ANTHROPIC_API_KEY` en tant que variable CI/CD masquée
   * **Amazon Bedrock** : **Configurer GitLab** → **AWS OIDC** et créez un rôle IAM pour Amazon Bedrock
   * **Plateforme Agent de Google Cloud** : **Configurer Workload Identity Federation pour GitLab** → **GCP**

2. **Ajouter les identifiants du projet pour les opérations de l'API GitLab** :
   * Utilisez `CI_JOB_TOKEN` par défaut, ou créez un jeton d'accès au projet avec la portée `api`
   * Stockez en tant que `GITLAB_ACCESS_TOKEN` (masqué) si vous utilisez un PAT

3. **Ajouter le travail Claude à `.gitlab-ci.yml`** : utilisez le travail [Configuration rapide](#quick-setup) pour l'API Claude, ou un travail de fournisseur à partir de [Exemples de configuration](#configuration-examples)

4. **(Optionnel) Activer les déclencheurs basés sur les mentions** :
   * Ajoutez un webhook de projet pour « Commentaires (notes) » à votre écouteur d'événements (si vous en utilisez un)
   * Faites en sorte que l'écouteur appelle l'API de déclenchement du pipeline avec des variables comme `AI_FLOW_INPUT` et `AI_FLOW_CONTEXT` lorsqu'un commentaire contient `@claude`

<h2 id="example-use-cases">
  Exemples de cas d'usage
</h2>

<h3 id="turn-issues-into-mrs">
  Transformer les problèmes en MR
</h3>

Dans un commentaire de problème :

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude analyse le problème et la base de code, écrit les modifications dans une branche et ouvre une MR pour examen.

<h3 id="get-implementation-help">
  Obtenir de l'aide à l'implémentation
</h3>

Dans une discussion de MR :

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propose des modifications, ajoute du code avec la mise en cache appropriée et met à jour la MR.

<h3 id="fix-bugs-quickly">
  Corriger les bugs rapidement
</h3>

Dans un commentaire de problème ou de MR :

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude localise le bug, implémente un correctif et met à jour la branche ou ouvre une nouvelle MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Utilisation avec Amazon Bedrock et Google Cloud
</h2>

Pour les environnements d'entreprise, vous pouvez exécuter Claude Code entièrement sur votre infrastructure cloud avec la même expérience développeur.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Prérequis

    Avant de configurer Claude Code avec Amazon Bedrock, vous avez besoin de :

    1. Un compte AWS avec accès à Amazon Bedrock pour les modèles Claude souhaités
    2. GitLab configuré en tant que fournisseur d'identité OIDC dans AWS IAM
    3. Un rôle IAM avec les permissions Amazon Bedrock et une politique de confiance limitée à votre projet/références GitLab
    4. Des variables CI/CD GitLab pour l'assomption de rôle :
       * `AWS_ROLE_TO_ASSUME` (ARN du rôle)
       * `AWS_REGION` (région Amazon Bedrock)

    ### Instructions de configuration

    Configurez AWS pour permettre aux tâches CI GitLab d'assumer un rôle IAM via OIDC (sans clés statiques).

    **Configuration requise :**

    1. Activez Amazon Bedrock et demandez l'accès à vos modèles Claude cibles
    2. Créez un fournisseur OIDC IAM pour GitLab s'il n'existe pas déjà
    3. Créez un rôle IAM approuvé par le fournisseur OIDC GitLab, limité à votre projet et aux références protégées
    4. Attachez les permissions de moindre privilège pour les API d'invocation Amazon Bedrock

    Utilisez l'[exemple de tâche Amazon Bedrock](#configuration-examples) pour échanger le jeton OIDC de la tâche contre des identifiants AWS temporaires au moment de l'exécution.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Prérequis

    Avant de configurer Claude Code avec Google Cloud's Agent Platform, vous avez besoin de :

    1. Un projet Google Cloud avec :
       * L'API Google Cloud's Agent Platform activée
       * Workload Identity Federation configurée pour approuver OIDC GitLab
    2. Un compte de service dédié avec uniquement les rôles Google Cloud's Agent Platform requis
    3. Des variables CI/CD GitLab :
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nom de ressource du fournisseur sans le préfixe `//iam.googleapis.com/`, tel que `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (adresse e-mail du compte de service)
       * `GCP_PROJECT_ID` (ID du projet Google Cloud)

    ### Instructions de configuration

    Configurez Google Cloud pour permettre aux tâches CI GitLab d'emprunter l'identité d'un compte de service via Workload Identity Federation.

    **Configuration requise :**

    1. Activez l'API IAM Credentials, l'API STS et l'API Google Cloud's Agent Platform
    2. Créez un pool Workload Identity et un fournisseur pour OIDC GitLab
    3. Créez un compte de service dédié avec les rôles Google Cloud's Agent Platform
    4. Accordez au principal WIF la permission d'emprunter l'identité du compte de service

    Utilisez l'[exemple de tâche Agent Platform](#configuration-examples) pour vous authentifier sans stocker de clés.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Exemples de configuration
</h2>

Voici des extraits prêts à l'emploi que vous pouvez adapter à votre pipeline.

<h3 id="amazon-bedrock-job-example-oidc">
  Exemple de job Amazon Bedrock (OIDC)
</h3>

**Prérequis :**

* Amazon Bedrock activé avec accès à votre ou vos modèles Claude choisis
* OIDC GitLab configuré dans AWS avec un rôle qui fait confiance à votre projet GitLab et à vos refs
* Rôle IAM avec permissions Amazon Bedrock (privilèges minimaux recommandés)

**Variables CI/CD requises :**

* `AWS_ROLE_TO_ASSUME` : ARN du rôle IAM pour l'accès à Amazon Bedrock
* `AWS_REGION` : région Amazon Bedrock (par exemple, `us-west-2`)

GitLab génère le token OIDC du job à partir du bloc `id_tokens:` et l'expose en tant que `GITLAB_OIDC_TOKEN`. Définissez `aud` sur la valeur d'audience que vous avez configurée sur le fournisseur d'identité OIDC IAM dans AWS, par exemple l'URL de votre instance GitLab.

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Les identifiants de modèle pour Amazon Bedrock incluent des préfixes spécifiques à la région (par exemple, `us.anthropic.claude-sonnet-4-6`). Transmettez le modèle souhaité via votre configuration de job ou votre prompt si votre flux de travail le permet.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Exemple de job Agent Platform (Workload Identity Federation)
</h3>

**Prérequis :**

* API Agent Platform de Google Cloud activée dans votre projet GCP
* Workload Identity Federation configurée pour faire confiance à OIDC GitLab
* Un compte de service avec les permissions Google Cloud Agent Platform

**Variables CI/CD requises :**

* `GCP_WORKLOAD_IDENTITY_PROVIDER` : nom de ressource du fournisseur sans le préfixe `//iam.googleapis.com/`, tel que `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT` : adresse e-mail du compte de service
* `GCP_PROJECT_ID` : identifiant du projet Google Cloud
* `CLOUD_ML_REGION` : région Google Cloud Agent Platform (par exemple, `us-east5`)

GitLab génère le token OIDC du job à partir du bloc `id_tokens:` et l'expose en tant que `GITLAB_OIDC_TOKEN`. Définissez `aud` sur la valeur d'audience que vous avez configurée sur le fournisseur du pool Workload Identity, par exemple l'URL de votre instance GitLab. Le job écrit le token dans un fichier, et l'entrée `credential_source` de la configuration des identifiants indique aux bibliothèques d'authentification de Google de le lire à partir de là. Définir `GOOGLE_APPLICATION_CREDENTIALS` sur le fichier de configuration des identifiants le rend disponible pour Claude Code via [Application Default Credentials](/docs/fr/google-vertex-ai#3-configure-gcp-credentials).

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  Avec Workload Identity Federation, vous n'avez pas besoin de stocker les clés de compte de service. Utilisez des conditions de confiance spécifiques au référentiel et des comptes de service avec privilèges minimaux.
</Note>

<h2 id="best-practices">
  Bonnes pratiques
</h2>

<h3 id="claude-md-configuration">
  Configuration CLAUDE.md
</h3>

Créez un fichier `CLAUDE.md` à la racine du référentiel pour définir les normes de codage, les critères d'examen et les règles spécifiques au projet. Claude lit ce fichier lors des exécutions et suit vos conventions lors de la proposition de modifications.

<h3 id="security-considerations">
  Considérations de sécurité
</h3>

**Ne validez jamais les clés API ou les identifiants cloud dans votre référentiel**. Utilisez toujours les variables GitLab CI/CD :

* Ajoutez `ANTHROPIC_API_KEY` comme variable masquée (et protégez-la si nécessaire)
* Utilisez OIDC spécifique au fournisseur si possible (pas de clés longue durée)
* Limitez les permissions des tâches et l'accès réseau sortant
* Examinez les MR de Claude comme tout autre contributeur

<h3 id="optimizing-performance">
  Optimisation des performances
</h3>

* Gardez `CLAUDE.md` concentré et concis
* Fournissez des descriptions claires des problèmes/MR pour réduire les itérations
* Mettez en cache les installations npm et de paquets dans les exécuteurs si possible

<h3 id="ci-costs">
  Coûts CI
</h3>

Lorsque vous utilisez Claude Code avec GitLab CI/CD, soyez conscient des coûts associés :

* **Temps du GitLab Runner** :
  * Claude s'exécute sur vos exécuteurs GitLab et consomme des minutes de calcul
  * Consultez les détails de facturation des exécuteurs de votre plan GitLab

* **Coûts API** :
  * Chaque interaction Claude consomme des jetons en fonction de la taille de l'invite et de la réponse
  * L'utilisation des jetons varie selon la complexité de la tâche et la taille de la base de code
  * Consultez [Tarification Anthropic](https://platform.claude.com/docs/en/about-claude/pricing) pour plus de détails

* **Conseils d'optimisation des coûts** :
  * Utilisez des commandes `@claude` spécifiques pour réduire les tours inutiles
  * Définissez des valeurs `--max-turns` appropriées et un `timeout` de tâche
  * Limitez la concurrence pour contrôler les exécutions parallèles

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude ne répond pas aux commandes @claude
</h3>

* Vérifiez que votre pipeline est déclenché (manuellement, événement MR, ou via un écouteur d'événement de note/webhook)
* Assurez-vous que votre `ANTHROPIC_API_KEY` ou les variables du fournisseur cloud sont présentes
* Vérifiez que le commentaire contient `@claude` (pas `/claude`) et que votre déclencheur de mention est configuré

<h3 id="job-can’t-write-comments-or-open-mrs">
  Le job ne peut pas écrire de commentaires ou ouvrir des MR
</h3>

* Assurez-vous que `CI_JOB_TOKEN` dispose des permissions suffisantes pour le projet, ou utilisez un Project Access Token avec la portée `api`
* Vérifiez que l'outil `mcp__gitlab` est activé dans `--allowedTools`
* Confirmez que le job s'exécute dans le contexte de la MR ou dispose de suffisamment de contexte via les variables `AI_FLOW_*`

<h3 id="authentication-errors">
  Erreurs d'authentification
</h3>

* **Pour Claude API** : Confirmez que `ANTHROPIC_API_KEY` est valide et non expiré
* **Pour Amazon Bedrock ou Google Cloud's Agent Platform** : Vérifiez la configuration OIDC/WIF, l'usurpation de rôle et les noms de secrets ; confirmez la disponibilité de la région et du modèle

<h2 id="advanced-configuration">
  Configuration avancée
</h2>

<h3 id="common-parameters-and-variables">
  Paramètres et variables courants
</h3>

Contrôlez les exécutions de Claude Code dans vos jobs avec ces drapeaux CLI, mots-clés GitLab et variables :

* `-p` : fournissez des instructions en ligne, par exemple `claude -p "Review this MR"`
* `--max-turns` : limitez le nombre d'itérations aller-retour
* `timeout` : limitez le temps total d'exécution du job avec le mot-clé `timeout` au niveau du job de GitLab, par exemple `timeout: 30m`
* `ANTHROPIC_API_KEY` : requis pour l'API Claude (non utilisé pour Amazon Bedrock ou la plateforme Agent de Google Cloud)
* Environnement spécifique au fournisseur : `AWS_REGION`, variables de projet/région pour la plateforme Agent de Google Cloud

<Note>
  Les drapeaux et paramètres exacts peuvent varier selon la version de `@anthropic-ai/claude-code`. Exécutez `claude --help` dans votre job pour voir les options prises en charge.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personnalisation du comportement de Claude
</h3>

Vous pouvez guider Claude de deux façons principales :

1. **CLAUDE.md** : Définissez les normes de codage, les exigences de sécurité et les conventions du projet. Claude lit ceci lors des exécutions et suit vos règles.
2. **Invites personnalisées** : Transmettez des instructions spécifiques aux tâches via `-p` dans le job. Utilisez différentes invites pour différents jobs (par exemple, review, implement, refactor).
