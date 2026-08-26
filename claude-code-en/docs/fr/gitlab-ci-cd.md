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

Claude Code active des flux de travail CI/CD puissants qui transforment votre façon de travailler avec le code :

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

Le moyen le plus rapide de commencer est d'ajouter une tâche minimale à votre `.gitlab-ci.yml` et de définir votre clé API comme variable masquée.

1. **Ajouter une variable CI/CD masquée**
   * Allez à **Paramètres** → **CI/CD** → **Variables**
   * Ajoutez `ANTHROPIC_API_KEY` (masquée, protégée selon les besoins)

2. **Ajouter une tâche Claude à `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Ajustez les règles pour adapter la façon dont vous souhaitez déclencher la tâche :
  # - exécutions manuelles
  # - événements de demande de fusion
  # - déclencheurs web/API lorsqu'un commentaire contient « @claude »
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    # Optionnel : démarrer un serveur GitLab MCP si votre configuration en fournit un
    - /bin/gitlab-mcp-server || true
    # Utilisez les variables AI_FLOW_* lors de l'invocation via des déclencheurs web/API avec des charges utiles de contexte
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Après avoir ajouté la tâche et votre variable `ANTHROPIC_API_KEY`, testez en exécutant la tâche manuellement à partir de **CI/CD** → **Pipelines**, ou déclenchez-la à partir d'une MR pour laisser Claude proposer des mises à jour dans une branche et ouvrir une MR si nécessaire.

<Note>
  Pour exécuter sur Amazon Bedrock ou Google Cloud's Agent Platform au lieu de Claude API, consultez la section [Utilisation avec Amazon Bedrock et Google Cloud](#using-with-amazon-bedrock-and-google-cloud) ci-dessous pour la configuration de l'authentification et de l'environnement.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configuration manuelle (recommandée pour la production)
</h3>

Si vous préférez une configuration plus contrôlée ou si vous avez besoin de fournisseurs d'entreprise :

1. **Configurer l'accès au fournisseur** :
   * **Claude API** : Créez et stockez `ANTHROPIC_API_KEY` comme variable CI/CD masquée
   * **Amazon Bedrock** : **Configurer GitLab** → **AWS OIDC** et créer un rôle IAM pour Amazon Bedrock
   * **Google Cloud's Agent Platform** : **Configurer Workload Identity Federation pour GitLab** → **GCP**

2. **Ajouter les identifiants du projet pour les opérations de l'API GitLab** :
   * Utilisez `CI_JOB_TOKEN` par défaut, ou créez un jeton d'accès au projet avec la portée `api`
   * Stockez comme `GITLAB_ACCESS_TOKEN` (masqué) si vous utilisez un PAT

3. **Ajouter la tâche Claude à `.gitlab-ci.yml`** (voir les exemples ci-dessous)

4. **(Optionnel) Activer les déclencheurs basés sur les mentions** :
   * Ajoutez un webhook de projet pour « Commentaires (notes) » à votre écouteur d'événements (si vous en utilisez un)
   * Faites en sorte que l'écouteur appelle l'API de déclenchement du pipeline avec des variables comme `AI_FLOW_INPUT` et `AI_FLOW_CONTEXT` lorsqu'un commentaire contient `@claude`

<h2 id="example-use-cases">
  Exemples de cas d'utilisation
</h2>

<h3 id="turn-issues-into-mrs">
  Transformer les problèmes en MR
</h3>

Dans un commentaire de problème :

```text theme={null}
@claude implement this feature based on the issue description
```

Claude analyse le problème et la base de code, écrit les modifications dans une branche et ouvre une MR pour examen.

<h3 id="get-implementation-help">
  Obtenir de l'aide à l'implémentation
</h3>

Dans une discussion MR :

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propose des modifications, ajoute du code avec la mise en cache appropriée et met à jour la MR.

<h3 id="fix-bugs-quickly">
  Corriger les bogues rapidement
</h3>

Dans un commentaire de problème ou de MR :

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude localise le bogue, implémente un correctif et met à jour la branche ou ouvre une nouvelle MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Utilisation avec Amazon Bedrock et Google Cloud
</h2>

Pour les environnements d'entreprise, vous pouvez exécuter Claude Code entièrement sur votre infrastructure cloud avec la même expérience développeur.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Conditions préalables

    Avant de configurer Claude Code avec Amazon Bedrock, vous avez besoin de :

    1. Un compte AWS avec accès à Amazon Bedrock pour les modèles Claude souhaités
    2. GitLab configuré comme fournisseur d'identité OIDC dans AWS IAM
    3. Un rôle IAM avec les autorisations Amazon Bedrock et une politique de confiance limitée à votre projet/références GitLab
    4. Variables CI/CD GitLab pour l'assomption de rôle :
       * `AWS_ROLE_TO_ASSUME` (ARN du rôle)
       * `AWS_REGION` (région Amazon Bedrock)

    ### Instructions de configuration

    Configurez AWS pour permettre aux tâches CI GitLab d'assumer un rôle IAM via OIDC (pas de clés statiques).

    **Configuration requise :**

    1. Activez Amazon Bedrock et demandez l'accès à vos modèles Claude cibles
    2. Créez un fournisseur OIDC IAM pour GitLab s'il n'existe pas déjà
    3. Créez un rôle IAM approuvé par le fournisseur OIDC GitLab, limité à votre projet et aux références protégées
    4. Attachez les autorisations de moindre privilège pour les API d'invocation Amazon Bedrock

    **Valeurs requises à stocker dans les variables CI/CD :**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    Ajoutez les variables dans Paramètres → CI/CD → Variables :

    ```yaml theme={null}
    # Pour Amazon Bedrock :
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    Utilisez l'exemple de tâche Amazon Bedrock ci-dessus pour échanger le jeton de tâche GitLab contre des identifiants AWS temporaires au moment de l'exécution.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Conditions préalables

    Avant de configurer Claude Code avec Google Cloud's Agent Platform, vous avez besoin de :

    1. Un projet Google Cloud avec :
       * API Google Cloud's Agent Platform activée
       * Workload Identity Federation configurée pour faire confiance à GitLab OIDC
    2. Un compte de service dédié avec uniquement les rôles Google Cloud's Agent Platform requis
    3. Variables CI/CD GitLab pour WIF :
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nom complet de la ressource)
       * `GCP_SERVICE_ACCOUNT` (e-mail du compte de service)

    ### Instructions de configuration

    Configurez Google Cloud pour permettre aux tâches CI GitLab d'emprunter l'identité d'un compte de service via Workload Identity Federation.

    **Configuration requise :**

    1. Activez l'API IAM Credentials, l'API STS et l'API Google Cloud's Agent Platform
    2. Créez un pool Workload Identity et un fournisseur pour GitLab OIDC
    3. Créez un compte de service dédié avec les rôles Google Cloud's Agent Platform
    4. Accordez au principal WIF la permission d'emprunter l'identité du compte de service

    **Valeurs requises à stocker dans les variables CI/CD :**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    Ajoutez les variables dans Paramètres → CI/CD → Variables :

    ```yaml theme={null}
    # Pour Google Cloud's Agent Platform :
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (par exemple, us-east5)
    ```

    Utilisez l'exemple de tâche Google Cloud's Agent Platform ci-dessus pour vous authentifier sans stocker de clés.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Exemples de configuration
</h2>

Voici des extraits prêts à l'emploi que vous pouvez adapter à votre pipeline.

<h3 id="basic-gitlab-ci-yml-claude-api">
  .gitlab-ci.yml basique (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code utilisera ANTHROPIC_API_KEY à partir des variables CI/CD
```

<h3 id="amazon-bedrock-job-example-oidc">
  Exemple de tâche Amazon Bedrock (OIDC)
</h3>

**Conditions préalables :**

* Amazon Bedrock activé avec accès à votre ou vos modèles Claude choisis
* GitLab OIDC configuré dans AWS avec un rôle qui fait confiance à votre projet et vos références GitLab
* Rôle IAM avec autorisations Amazon Bedrock (moindre privilège recommandé)

**Variables CI/CD requises :**

* `AWS_ROLE_TO_ASSUME` : ARN du rôle IAM pour l'accès à Amazon Bedrock
* `AWS_REGION` : Région Amazon Bedrock (par exemple, `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # Échanger le jeton OIDC GitLab contre les identifiants AWS
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
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
```

<Note>
  Les identifiants de modèle pour Amazon Bedrock incluent des préfixes spécifiques à la région (par exemple, `us.anthropic.claude-sonnet-4-6`). Transmettez le modèle souhaité via votre configuration de tâche ou votre invite si votre flux de travail le supporte.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Exemple de tâche Agent Platform (Workload Identity Federation)
</h3>

**Conditions préalables :**

* API Agent Platform de Google Cloud activée dans votre projet GCP
* Workload Identity Federation configurée pour faire confiance à GitLab OIDC
* Un compte de service avec les autorisations Agent Platform de Google Cloud

**Variables CI/CD requises :**

* `GCP_WORKLOAD_IDENTITY_PROVIDER` : Nom complet de la ressource du fournisseur
* `GCP_SERVICE_ACCOUNT` : E-mail du compte de service
* `CLOUD_ML_REGION` : Région Agent Platform de Google Cloud (par exemple, `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # S'authentifier auprès de Google Cloud via WIF (pas de clés téléchargées)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
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
```

<Note>
  Avec Workload Identity Federation, vous n'avez pas besoin de stocker les clés du compte de service. Utilisez des conditions de confiance spécifiques au référentiel et des comptes de service avec le moindre privilège.
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

**Ne validez jamais les clés API ou les identifiants cloud dans votre référentiel**. Utilisez toujours les variables CI/CD GitLab :

* Ajoutez `ANTHROPIC_API_KEY` comme variable masquée (et protégez-la si nécessaire)
* Utilisez OIDC spécifique au fournisseur si possible (pas de clés longue durée)
* Limitez les autorisations des tâches et la sortie réseau
* Examinez les MR de Claude comme tout autre contributeur

<h3 id="optimizing-performance">
  Optimisation des performances
</h3>

* Gardez `CLAUDE.md` concentré et concis
* Fournissez des descriptions claires de problèmes/MR pour réduire les itérations
* Configurez des délais d'expiration de tâche raisonnables pour éviter les exécutions incontrôlées
* Mettez en cache les installations npm et de paquets dans les exécuteurs si possible

<h3 id="ci-costs">
  Coûts CI
</h3>

Lorsque vous utilisez Claude Code avec GitLab CI/CD, soyez conscient des coûts associés :

* **Temps d'exécution GitLab Runner** :
  * Claude s'exécute sur vos exécuteurs GitLab et consomme des minutes de calcul
  * Consultez la facturation des exécuteurs de votre plan GitLab pour plus de détails

* **Coûts API** :
  * Chaque interaction Claude consomme des jetons en fonction de la taille de l'invite et de la réponse
  * L'utilisation des jetons varie selon la complexité de la tâche et la taille de la base de code
  * Consultez [Tarification Anthropic](https://platform.claude.com/docs/fr/about-claude/pricing) pour plus de détails

* **Conseils d'optimisation des coûts** :
  * Utilisez des commandes `@claude` spécifiques pour réduire les tours inutiles
  * Définissez les valeurs `max_turns` et le délai d'expiration de la tâche appropriés
  * Limitez la concurrence pour contrôler les exécutions parallèles

<h2 id="security-and-governance">
  Sécurité et gouvernance
</h2>

* Chaque tâche s'exécute dans un conteneur isolé avec accès réseau restreint
* Les modifications de Claude passent par des MR afin que les examinateurs voient chaque différence
* Les règles de protection de branche et d'approbation s'appliquent au code généré par l'IA
* Claude Code utilise des autorisations limitées à l'espace de travail pour limiter les écritures
* Les coûts restent sous votre contrôle car vous apportez vos propres identifiants de fournisseur

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude ne répond pas aux commandes @claude
</h3>

* Vérifiez que votre pipeline est déclenché (manuellement, événement MR ou via un écouteur d'événements/webhook de note)
* Assurez-vous que les variables CI/CD (`ANTHROPIC_API_KEY` ou les paramètres du fournisseur cloud) sont présentes et non masquées
* Vérifiez que le commentaire contient `@claude` (pas `/claude`) et que votre déclencheur de mention est configuré

<h3 id="job-can’t-write-comments-or-open-mrs">
  La tâche ne peut pas écrire de commentaires ou ouvrir des MR
</h3>

* Assurez-vous que `CI_JOB_TOKEN` dispose des autorisations suffisantes pour le projet, ou utilisez un jeton d'accès au projet avec la portée `api`
* Vérifiez que l'outil `mcp__gitlab` est activé dans `--allowedTools`
* Confirmez que la tâche s'exécute dans le contexte de la MR ou dispose de suffisamment de contexte via les variables `AI_FLOW_*`

<h3 id="authentication-errors">
  Erreurs d'authentification
</h3>

* **Pour Claude API** : Confirmez que `ANTHROPIC_API_KEY` est valide et non expiré
* **Pour Amazon Bedrock ou Google Cloud's Agent Platform** : Vérifiez la configuration OIDC/WIF, l'emprunt d'identité de rôle et les noms secrets ; confirmez la disponibilité de la région et du modèle

<h2 id="advanced-configuration">
  Configuration avancée
</h2>

<h3 id="common-parameters-and-variables">
  Paramètres et variables courants
</h3>

Claude Code supporte ces entrées couramment utilisées :

* `prompt` / `prompt_file` : Fournissez les instructions en ligne (`-p`) ou via un fichier
* `max_turns` : Limitez le nombre d'itérations aller-retour
* `timeout_minutes` : Limitez le temps d'exécution total
* `ANTHROPIC_API_KEY` : Requis pour Claude API (non utilisé pour Amazon Bedrock ou Google Cloud's Agent Platform)
* Environnement spécifique au fournisseur : `AWS_REGION`, variables de projet/région pour Google Cloud's Agent Platform

<Note>
  Les drapeaux et paramètres exacts peuvent varier selon la version de `@anthropic-ai/claude-code`. Exécutez `claude --help` dans votre tâche pour voir les options prises en charge.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personnalisation du comportement de Claude
</h3>

Vous pouvez guider Claude de deux façons principales :

1. **CLAUDE.md** : Définissez les normes de codage, les exigences de sécurité et les conventions du projet. Claude lit ceci lors des exécutions et suit vos règles.
2. **Invites personnalisées** : Transmettez les instructions spécifiques à la tâche via `prompt`/`prompt_file` dans la tâche. Utilisez différentes invites pour différentes tâches (par exemple, examen, implémentation, refactorisation).
