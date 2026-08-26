> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Découvrez comment intégrer Claude Code dans votre flux de travail de développement avec Claude Code GitHub Actions

Claude Code GitHub Actions apporte l'automatisation alimentée par l'IA à votre flux de travail GitHub. Avec une simple mention `@claude` dans n'importe quelle PR ou issue, Claude peut analyser votre code, créer des pull requests, implémenter des fonctionnalités et corriger des bugs - tout en respectant les normes de votre projet. Pour les révisions automatiques publiées sur chaque PR sans déclencheur, consultez [GitHub Code Review](/docs/fr/code-review).

<Note>
  Claude Code GitHub Actions est construit sur le [Claude Agent SDK](/docs/fr/agent-sdk/overview), qui permet l'intégration programmatique de Claude Code dans vos applications. Vous pouvez utiliser le SDK pour créer des flux de travail d'automatisation personnalisés au-delà de GitHub Actions.
</Note>

<h2 id="why-use-claude-code-github-actions">
  Pourquoi utiliser Claude Code GitHub Actions ?
</h2>

* **Création instantanée de PR** : Décrivez ce dont vous avez besoin, et Claude crée une PR complète avec tous les changements nécessaires
* **Implémentation de code automatisée** : Transformez les issues en code fonctionnel avec une seule commande
* **Respecte vos normes** : Claude respecte vos directives `CLAUDE.md` et les modèles de code existants
* **Configuration simple** : Commencez en quelques minutes avec notre installateur et votre clé API
* **Sécurisé par défaut** : Votre code reste sur les runners de Github

<h2 id="what-can-claude-do">
  Que peut faire Claude ?
</h2>

Claude Code fournit une GitHub Action puissante qui transforme votre façon de travailler avec le code :

<h3 id="claude-code-action">
  Claude Code Action
</h3>

Cette GitHub Action vous permet d'exécuter Claude Code dans vos flux de travail GitHub Actions. Vous pouvez l'utiliser pour créer n'importe quel flux de travail personnalisé sur Claude Code.

[Voir le repository →](https://github.com/anthropics/claude-code-action)

<h2 id="setup">
  Configuration
</h2>

<h2 id="quick-setup">
  Configuration rapide
</h2>

Exécutez `/install-github-app` dans le terminal Claude Code pour configurer l'intégration de manière interactive. La commande installe l'application Claude GitHub sur votre repository et vous guide ensuite à travers l'ajout des workflows GitHub Actions et du secret de clé API.

Après l'installation de l'application GitHub, la commande vous demande si vous souhaitez continuer avec la configuration de GitHub Actions. Dans Claude Code v2.1.187 et versions ultérieures, vous pouvez choisir **Ignorer pour l'instant** pour arrêter avec seulement l'application installée et revenir aux étapes de workflow et de secret en exécutant `/install-github-app` à nouveau. Les versions antérieures passent directement à la sélection du workflow.

<Note>
  * Vous devez être administrateur du repository pour installer l'application GitHub et ajouter des secrets
  * L'application GitHub demandera des permissions de lecture et d'écriture pour Contents, Issues et Pull requests
  * Cette méthode de démarrage rapide n'est disponible que pour les utilisateurs directs de l'API Claude. Si vous utilisez Amazon Bedrock ou Google Cloud's Agent Platform, veuillez consulter la section [Utilisation avec Amazon Bedrock et Google Cloud](#using-with-amazon-bedrock-and-google-cloud).
</Note>

<h2 id="manual-setup">
  Configuration manuelle
</h2>

Si la commande `/install-github-app` échoue ou si vous préférez une configuration manuelle, veuillez suivre ces instructions de configuration manuelle :

1. **Installez l'application Claude GitHub** dans votre repository : [https://github.com/apps/claude](https://github.com/apps/claude)

   L'application Claude GitHub nécessite les permissions de repository suivantes :

   * **Contents** : Lecture et écriture (pour modifier les fichiers du repository)
   * **Issues** : Lecture et écriture (pour répondre aux issues)
   * **Pull requests** : Lecture et écriture (pour créer des PRs et pousser les changements)

   Pour plus de détails sur la sécurité et les permissions, consultez la [documentation de sécurité](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).
2. **Ajoutez ANTHROPIC\_API\_KEY** à vos secrets de repository ([Apprenez comment utiliser les secrets dans GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **Copiez le fichier de flux de travail** depuis [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) dans le répertoire `.github/workflows/` de votre repository

<Tip>
  Après avoir complété la configuration rapide ou manuelle, testez l'action en marquant `@claude` dans un commentaire d'issue ou de PR.
</Tip>

<h2 id="upgrading-from-beta">
  Mise à niveau depuis la version bêta
</h2>

<Warning>
  Claude Code GitHub Actions v1.0 introduit des changements majeurs qui nécessitent de mettre à jour vos fichiers de flux de travail pour passer de la version bêta à v1.0.
</Warning>

Si vous utilisez actuellement la version bêta de Claude Code GitHub Actions, nous vous recommandons de mettre à jour vos flux de travail pour utiliser la version GA. La nouvelle version simplifie la configuration tout en ajoutant des fonctionnalités puissantes comme la détection automatique du mode.

<h3 id="essential-changes">
  Changements essentiels
</h3>

Tous les utilisateurs bêta doivent apporter ces changements à leurs fichiers de flux de travail pour mettre à niveau :

1. **Mettez à jour la version de l'action** : Changez `@beta` en `@v1`
2. **Supprimez la configuration du mode** : Supprimez `mode: "tag"` ou `mode: "agent"` (maintenant détecté automatiquement)
3. **Mettez à jour les entrées de prompt** : Remplacez `direct_prompt` par `prompt`
4. **Déplacez les options CLI** : Convertissez `max_turns`, `model`, `custom_instructions`, etc. en `claude_args`

<h3 id="breaking-changes-reference">
  Référence des changements majeurs
</h3>

| Ancienne entrée bêta  | Nouvelle entrée v1.0                     |
| --------------------- | ---------------------------------------- |
| `mode`                | *(Supprimée - détectée automatiquement)* |
| `direct_prompt`       | `prompt`                                 |
| `override_prompt`     | `prompt` avec variables GitHub           |
| `custom_instructions` | `claude_args: --append-system-prompt`    |
| `max_turns`           | `claude_args: --max-turns`               |
| `model`               | `claude_args: --model`                   |
| `allowed_tools`       | `claude_args: --allowedTools`            |
| `disallowed_tools`    | `claude_args: --disallowedTools`         |
| `claude_env`          | `settings` format JSON                   |

<h3 id="before-and-after-example">
  Exemple avant et après
</h3>

**Version bêta :**

```yaml theme={null}
- uses: anthropics/claude-code-action@beta
  with:
    mode: "tag"
    direct_prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    custom_instructions: "Follow our coding standards"
    max_turns: "10"
    model: "claude-sonnet-5"
```

**Version GA (v1.0) :**

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    claude_args: |
      --append-system-prompt "Follow our coding standards"
      --max-turns 10
      --model claude-sonnet-5
```

<Tip>
  L'action détecte maintenant automatiquement s'il faut s'exécuter en mode interactif (répond aux mentions `@claude`) ou en mode automatisation (s'exécute immédiatement avec un prompt) en fonction de votre configuration.
</Tip>

<h2 id="example-use-cases">
  Exemples de cas d'usage
</h2>

Claude Code GitHub Actions peut vous aider avec une variété de tâches. Le [répertoire d'exemples](https://github.com/anthropics/claude-code-action/tree/main/examples) contient des flux de travail prêts à l'emploi pour différents scénarios.

<h3 id="basic-workflow">
  Flux de travail basique
</h3>

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          # Responds to @claude mentions in comments
```

<h3 id="using-skills">
  Utilisation de skills
</h3>

L'entrée `prompt` accepte une [invocation de skill](/docs/fr/skills) ainsi que du texte brut :

* Pour un skill dans le répertoire `.claude/skills/` de votre référentiel, exécutez `actions/checkout` avant l'étape d'action et passez `/skill-name`.
* Pour un skill empaqueté dans un plugin, installez le plugin avec les entrées `plugin_marketplaces` et `plugins` et passez le `/plugin-name:skill-name` avec espace de noms.

Le flux de travail suivant installe le plugin `code-review` et exécute son skill sur chaque pull request nouvelle ou mise à jour :

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
```

<h3 id="custom-automation-with-prompts">
  Automatisation personnalisée avec prompts
</h3>

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: "--model opus"
```

<h3 id="common-use-cases">
  Cas d'usage courants
</h3>

Dans les commentaires d'issue ou de PR :

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude analysera automatiquement le contexte et répondra de manière appropriée.

<h2 id="best-practices">
  Bonnes pratiques
</h2>

<h3 id="claude-md-configuration">
  Configuration CLAUDE.md
</h3>

Créez un fichier `CLAUDE.md` à la racine de votre repository pour définir les directives de style de code, les critères de révision, les règles spécifiques au projet et les modèles préférés. Ce fichier guide la compréhension de Claude des normes de votre projet.

<h3 id="security-considerations">
  Considérations de sécurité
</h3>

<Warning>Ne commitez jamais les clés API directement dans votre repository.</Warning>

Pour des conseils de sécurité complets incluant les permissions, l'authentification et les bonnes pratiques, consultez la [documentation de sécurité de Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

Utilisez toujours GitHub Secrets pour les clés API :

* Ajoutez votre clé API en tant que secret de repository nommé `ANTHROPIC_API_KEY`
* Référencez-la dans les flux de travail : `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* Limitez les permissions de l'action à ce qui est nécessaire uniquement
* Examinez les suggestions de Claude avant de fusionner

Utilisez toujours GitHub Secrets (par exemple, `${{ secrets.ANTHROPIC_API_KEY }}`) plutôt que de coder en dur les clés API directement dans vos fichiers de flux de travail.

<h3 id="optimizing-performance">
  Optimisation des performances
</h3>

Utilisez les modèles d'issue pour fournir du contexte, gardez votre `CLAUDE.md` concis et ciblé, et configurez les délais d'attente appropriés pour vos flux de travail.

<h3 id="ci-costs">
  Coûts CI
</h3>

Lorsque vous utilisez Claude Code GitHub Actions, soyez conscient des coûts associés :

**Coûts GitHub Actions :**

* Claude Code s'exécute sur les runners hébergés par GitHub, qui consomment vos minutes GitHub Actions
* Consultez la [documentation de facturation de GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) pour les tarifs détaillés et les limites de minutes

**Coûts API :**

* Chaque interaction Claude consomme des tokens API en fonction de la longueur des prompts et des réponses
* L'utilisation des tokens varie selon la complexité de la tâche et la taille de la base de code
* Consultez la [page de tarification de Claude](https://claude.com/platform/api) pour les tarifs actuels des tokens

**Conseils d'optimisation des coûts :**

* Utilisez des commandes `@claude` spécifiques pour réduire les appels API inutiles
* Configurez `--max-turns` approprié dans `claude_args` pour éviter les itérations excessives
* Définissez les délais d'attente au niveau du flux de travail pour éviter les jobs qui s'exécutent indéfiniment
* Envisagez d'utiliser les contrôles de concurrence de GitHub pour limiter les exécutions parallèles

<h2 id="configuration-examples">
  Exemples de configuration
</h2>

Claude Code Action v1 simplifie la configuration avec des paramètres unifiés :

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

Fonctionnalités clés :

* **Interface de prompt unifiée** - Utilisez `prompt` pour toutes les instructions
* **Skills** - Invoquez les [skills](/docs/fr/skills) installés directement depuis le prompt
* **Passthrough CLI** - N'importe quel argument Claude Code CLI via `claude_args`
* **Déclencheurs flexibles** - Fonctionne avec n'importe quel événement GitHub

Visitez le [répertoire d'exemples](https://github.com/anthropics/claude-code-action/tree/main/examples) pour les fichiers de flux de travail complets.

<Tip>
  Lorsque vous répondez à des commentaires d'issue ou de PR, Claude répond automatiquement aux mentions @claude. Pour les autres événements, utilisez le paramètre `prompt` pour fournir des instructions.
</Tip>

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Utilisation avec Amazon Bedrock et Google Cloud
</h2>

Pour les environnements d'entreprise, vous pouvez utiliser Claude Code GitHub Actions avec votre propre infrastructure cloud. Cette approche vous donne le contrôle sur la résidence des données et la facturation tout en maintenant les mêmes fonctionnalités.

<h3 id="prerequisites">
  Prérequis
</h3>

Avant de configurer Claude Code GitHub Actions avec les fournisseurs cloud, vous avez besoin de :

<h4 id="for-google-cloud’s-agent-platform">
  Pour Google Cloud's Agent Platform :
</h4>

1. Un projet Google Cloud avec Google Cloud's Agent Platform activé
2. Workload Identity Federation configuré pour GitHub Actions
3. Un compte de service avec les permissions requises
4. Une application GitHub (recommandée) ou utilisez le GITHUB\_TOKEN par défaut

<h4 id="for-amazon-bedrock">
  Pour Amazon Bedrock :
</h4>

1. Un compte AWS avec Amazon Bedrock activé
2. GitHub OIDC Identity Provider configuré dans AWS
3. Un rôle IAM avec les permissions Amazon Bedrock
4. Une application GitHub (recommandée) ou utilisez le GITHUB\_TOKEN par défaut

<Steps>
  <Step title="Créer une application GitHub personnalisée (Recommandée pour les fournisseurs tiers)">
    Pour un meilleur contrôle et une meilleure sécurité lors de l'utilisation de fournisseurs tiers comme Google Cloud's Agent Platform ou Amazon Bedrock, nous recommandons de créer votre propre application GitHub :

    1. Allez à [https://github.com/settings/apps/new](https://github.com/settings/apps/new)
    2. Remplissez les informations de base :
       * **Nom de l'application GitHub** : Choisissez un nom unique (par exemple, ' YourOrg Claude Assistant ')
       * **URL de la page d'accueil** : Le site web de votre organisation ou l'URL du repository
    3. Configurez les paramètres de l'application :
       * **Webhooks** : Décochez ' Active ' (non nécessaire pour cette intégration)
    4. Définissez les permissions requises :
       * **Permissions du repository** :
         * Contents : Lecture et écriture
         * Issues : Lecture et écriture
         * Pull requests : Lecture et écriture
    5. Cliquez sur ' Create GitHub App '
    6. Après la création, cliquez sur ' Generate a private key ' et enregistrez le fichier `.pem` téléchargé
    7. Notez votre ID d'application à partir de la page des paramètres de l'application
    8. Installez l'application dans votre repository :
       * À partir de la page des paramètres de votre application, cliquez sur ' Install App ' dans la barre latérale gauche
       * Sélectionnez votre compte ou organisation
       * Choisissez ' Only select repositories ' et sélectionnez le repository spécifique
       * Cliquez sur ' Install '
    9. Ajoutez la clé privée en tant que secret à votre repository :
       * Allez à Settings → Secrets and variables → Actions de votre repository
       * Créez un nouveau secret nommé `APP_PRIVATE_KEY` avec le contenu du fichier `.pem`
    10. Ajoutez l'ID de l'application en tant que secret :

    * Créez un nouveau secret nommé `APP_ID` avec l'ID de votre application GitHub

    <Note>
      Cette application sera utilisée avec l'action [actions/create-github-app-token](https://github.com/actions/create-github-app-token) pour générer des tokens d'authentification dans vos flux de travail.
    </Note>

    **Alternative pour l'API Claude ou si vous ne voulez pas configurer votre propre application Github** : Utilisez l'application officielle Anthropic :

    1. Installez depuis : [https://github.com/apps/claude](https://github.com/apps/claude)
    2. Aucune configuration supplémentaire nécessaire pour l'authentification
  </Step>

  <Step title="Configurer l'authentification du fournisseur cloud">
    Choisissez votre fournisseur cloud et configurez l'authentification sécurisée :

    <AccordionGroup>
      <Accordion title="Amazon Bedrock">
        **Configurez AWS pour permettre à GitHub Actions de s'authentifier de manière sécurisée sans stocker les credentials.**

        > **Note de sécurité** : Utilisez des configurations spécifiques au repository et accordez uniquement les permissions minimales requises.

        **Configuration requise** :

        1. **Activez Amazon Bedrock** :
           * Demandez l'accès aux modèles Claude dans Amazon Bedrock
           * Pour les modèles multi-régions, demandez l'accès dans toutes les régions requises

        2. **Configurez le fournisseur d'identité GitHub OIDC** :
           * URL du fournisseur : `https://token.actions.githubusercontent.com`
           * Audience : `sts.amazonaws.com`

        3. **Créez un rôle IAM pour GitHub Actions** :
           * Type d'entité de confiance : Web identity
           * Fournisseur d'identité : `token.actions.githubusercontent.com`
           * Permissions : politique `AmazonBedrockFullAccess`
           * Configurez la politique de confiance pour votre repository spécifique

        **Valeurs requises** :

        Après la configuration, vous aurez besoin de :

        * **AWS\_ROLE\_TO\_ASSUME** : L'ARN du rôle IAM que vous avez créé

        <Tip>
          OIDC est plus sécurisé que l'utilisation de clés d'accès AWS statiques car les credentials sont temporaires et automatiquement renouvelés.
        </Tip>

        Consultez la [documentation AWS](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) pour les instructions détaillées de configuration OIDC.
      </Accordion>

      <Accordion title="Google Cloud's Agent Platform">
        **Configurez Google Cloud pour permettre à GitHub Actions de s'authentifier de manière sécurisée sans stocker les credentials.**

        > **Note de sécurité** : Utilisez des configurations spécifiques au repository et accordez uniquement les permissions minimales requises.

        **Configuration requise** :

        1. **Activez les APIs** dans votre projet Google Cloud :
           * IAM Credentials API
           * Security Token Service (STS) API
           * Google Cloud's Agent Platform API

        2. **Créez les ressources Workload Identity Federation** :
           * Créez un Workload Identity Pool
           * Ajoutez un fournisseur GitHub OIDC avec :
             * Émetteur : `https://token.actions.githubusercontent.com`
             * Mappages d'attributs pour le repository et le propriétaire
             * **Recommandation de sécurité** : Utilisez des conditions d'attribut spécifiques au repository

        3. **Créez un compte de service** :
           * Accordez uniquement le rôle `Vertex AI User`
           * **Recommandation de sécurité** : Créez un compte de service dédié par repository

        4. **Configurez les liaisons IAM** :
           * Autorisez le Workload Identity Pool à emprunter l'identité du compte de service
           * **Recommandation de sécurité** : Utilisez des ensembles de principaux spécifiques au repository

        **Valeurs requises** :

        Après la configuration, vous aurez besoin de :

        * **GCP\_WORKLOAD\_IDENTITY\_PROVIDER** : Le nom complet de la ressource du fournisseur
        * **GCP\_SERVICE\_ACCOUNT** : L'adresse e-mail du compte de service

        <Tip>
          Workload Identity Federation élimine le besoin de clés de compte de service téléchargeables, améliorant la sécurité.
        </Tip>

        Pour les instructions de configuration détaillées, consultez la [documentation Google Cloud Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation).
      </Accordion>
    </AccordionGroup>
  </Step>

  <Step title="Ajouter les secrets requis">
    Ajoutez les secrets suivants à votre repository (Settings → Secrets and variables → Actions) :

    #### Pour l'API Claude (Direct) :

    1. **Pour l'authentification API** :
       * `ANTHROPIC_API_KEY` : Votre clé API Claude depuis [console.anthropic.com](https://console.anthropic.com)

    2. **Pour l'application GitHub (si vous utilisez votre propre application)** :
       * `APP_ID` : L'ID de votre application GitHub
       * `APP_PRIVATE_KEY` : Le contenu de la clé privée (.pem)

    #### Pour Google Cloud's Agent Platform

    1. **Pour l'authentification GCP** :
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`
       * `GCP_SERVICE_ACCOUNT`

    2. **Pour l'application GitHub (si vous utilisez votre propre application)** :
       * `APP_ID` : L'ID de votre application GitHub
       * `APP_PRIVATE_KEY` : Le contenu de la clé privée (.pem)

    #### Pour AWS Bedrock

    1. **Pour l'authentification AWS** :
       * `AWS_ROLE_TO_ASSUME`

    2. **Pour l'application GitHub (si vous utilisez votre propre application)** :
       * `APP_ID` : L'ID de votre application GitHub
       * `APP_PRIVATE_KEY` : Le contenu de la clé privée (.pem)
  </Step>

  <Step title="Créer des fichiers de flux de travail">
    Créez des fichiers de flux de travail GitHub Actions qui s'intègrent à votre fournisseur cloud. Les exemples ci-dessous montrent des configurations complètes pour Amazon Bedrock et Google Cloud's Agent Platform :

    <AccordionGroup>
      <Accordion title="Flux de travail Amazon Bedrock">
        **Prérequis :**

        * Accès Amazon Bedrock activé avec permissions de modèle Claude
        * GitHub configuré en tant que fournisseur d'identité OIDC dans AWS
        * Rôle IAM avec permissions Amazon Bedrock qui fait confiance à GitHub Actions

        **Secrets GitHub requis :**

        | Nom du secret        | Description                                                              |
        | -------------------- | ------------------------------------------------------------------------ |
        | `AWS_ROLE_TO_ASSUME` | ARN du rôle IAM pour l'accès à Amazon Bedrock                            |
        | `APP_ID`             | Votre ID d'application GitHub (à partir des paramètres de l'application) |
        | `APP_PRIVATE_KEY`    | La clé privée que vous avez générée pour votre application GitHub        |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            env:
              AWS_REGION: us-west-2
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6 --max-turns 10'
        ```

        <Tip>
          Le format d'ID de modèle pour Amazon Bedrock inclut un préfixe de région (par exemple, `us.anthropic.claude-sonnet-4-6`).
        </Tip>
      </Accordion>

      <Accordion title="Flux de travail Google Cloud's Agent Platform">
        **Prérequis :**

        * Google Cloud's Agent Platform API activée dans votre projet GCP
        * Workload Identity Federation configurée pour GitHub
        * Compte de service avec permissions Google Cloud's Agent Platform

        **Secrets GitHub requis :**

        | Nom du secret                    | Description                                                              |
        | -------------------------------- | ------------------------------------------------------------------------ |
        | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Nom de ressource du fournisseur d'identité de charge de travail          |
        | `GCP_SERVICE_ACCOUNT`            | E-mail du compte de service avec accès à Google Cloud's Agent Platform   |
        | `APP_ID`                         | Votre ID d'application GitHub (à partir des paramètres de l'application) |
        | `APP_PRIVATE_KEY`                | La clé privée que vous avez générée pour votre application GitHub        |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  trigger_phrase: "@claude"
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-4-5@20250929 --max-turns 10'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
                  VERTEX_REGION_CLAUDE_4_5_SONNET: us-east5
        ```

        <Tip>
          L'ID du projet est automatiquement récupéré à partir de l'étape d'authentification Google Cloud, vous n'avez donc pas besoin de le coder en dur.
        </Tip>
      </Accordion>
    </AccordionGroup>
  </Step>
</Steps>

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude ne répond pas aux commandes @claude
</h3>

Vérifiez que l'application GitHub est correctement installée, vérifiez que les flux de travail sont activés, assurez-vous que la clé API est définie dans les secrets du repository et confirmez que le commentaire contient `@claude` (pas `/claude`).

<h3 id="ci-not-running-on-claude’s-commits">
  CI ne s'exécute pas sur les commits de Claude
</h3>

Assurez-vous que vous utilisez l'application GitHub ou une application personnalisée (pas l'utilisateur Actions), vérifiez que les déclencheurs de flux de travail incluent les événements nécessaires et vérifiez que les permissions de l'application incluent les déclencheurs CI.

<h3 id="authentication-errors">
  Erreurs d'authentification
</h3>

Confirmez que la clé API est valide et dispose des permissions suffisantes. Pour Amazon Bedrock ou la plateforme Agent de Google Cloud, vérifiez la configuration des credentials et assurez-vous que les secrets sont nommés correctement dans les flux de travail.

<h2 id="advanced-configuration">
  Configuration avancée
</h2>

<h3 id="action-parameters">
  Paramètres de l'action
</h3>

Claude Code Action v1 utilise une configuration simplifiée :

| Paramètre             | Description                                                                            | Requis  |
| --------------------- | -------------------------------------------------------------------------------------- | ------- |
| `prompt`              | Instructions pour Claude (texte brut ou un nom de [skill](/docs/fr/skills))                 | Non\*   |
| `claude_args`         | Arguments CLI passés à Claude Code                                                     | Non     |
| `plugin_marketplaces` | Liste séparée par des sauts de ligne des URL Git des places de marché de plugins       | Non     |
| `plugins`             | Liste séparée par des sauts de ligne des noms de plugins à installer avant l'exécution | Non     |
| `anthropic_api_key`   | Clé API Claude                                                                         | Oui\*\* |
| `github_token`        | Token GitHub pour l'accès API                                                          | Non     |
| `trigger_phrase`      | Phrase de déclenchement personnalisée (par défaut : « @claude »)                       | Non     |
| `use_bedrock`         | Utiliser Amazon Bedrock au lieu de l'API Claude                                        | Non     |
| `use_vertex`          | Utiliser Google Cloud's Agent Platform au lieu de l'API Claude                         | Non     |

\*Le prompt est optionnel - lorsqu'il est omis pour les commentaires d'issue/PR, Claude répond à la phrase de déclenchement\
\*\*Requis pour l'API Claude directe, pas pour Amazon Bedrock ou Google Cloud's Agent Platform

<h4 id="pass-cli-arguments">
  Passer les arguments CLI
</h4>

Le paramètre `claude_args` accepte n'importe quel argument Claude Code CLI :

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Arguments courants :

* `--max-turns` : Nombre maximum de tours de conversation (par défaut : 10)
* `--model` : Modèle à utiliser (par exemple, `claude-sonnet-5`)
* `--mcp-config` : Chemin vers la configuration MCP
* `--allowedTools` : Liste séparée par des virgules des outils autorisés. L'alias `--allowed-tools` fonctionne également.
* `--debug` : Activer la sortie de débogage

<h3 id="alternative-integration-methods">
  Méthodes d'intégration alternatives
</h3>

Bien que la commande `/install-github-app` soit l'approche recommandée, vous pouvez également :

* **Application GitHub personnalisée** : Pour les organisations ayant besoin de noms d'utilisateur de marque ou de flux d'authentification personnalisés. Créez votre propre application GitHub avec les permissions requises (contents, issues, pull requests) et utilisez l'action actions/create-github-app-token pour générer des tokens dans vos flux de travail.
* **GitHub Actions manuel** : Configuration directe du flux de travail pour une flexibilité maximale
* **Configuration MCP** : Chargement dynamique des serveurs Model Context Protocol

Consultez la [documentation Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs) pour des guides détaillés sur l'authentification, la sécurité et la configuration avancée.

<h3 id="customizing-claude’s-behavior">
  Personnalisation du comportement de Claude
</h3>

Vous pouvez configurer le comportement de Claude de deux façons :

1. **CLAUDE.md** : Définissez les normes de codage, les critères de révision et les règles spécifiques au projet dans un fichier `CLAUDE.md` à la racine de votre repository. Claude suivra ces directives lors de la création de PRs et de la réponse aux demandes. Consultez notre [documentation Memory](/docs/fr/memory) pour plus de détails.
2. **Prompts personnalisés** : Utilisez le paramètre `prompt` dans le fichier de flux de travail pour fournir des instructions spécifiques au flux de travail. Cela vous permet de personnaliser le comportement de Claude pour différents flux de travail ou tâches.

Claude suivra ces directives lors de la création de PRs et de la réponse aux demandes.
