> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Exécutez Claude Code dans les workflows GitHub Actions pour répondre aux mentions @claude, automatiser les tâches et transformer les issues en pull requests

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) est une GitHub Action qui exécute Claude Code dans les workflows de votre repository. Mentionnez `@claude` dans un commentaire de pull request ou d'issue pour que Claude analyse le code, implémente des modifications et pousse des commits. Vous pouvez également donner à la GitHub Action Claude Code un prompt pour s'exécuter automatiquement sur n'importe quel événement GitHub. Utilisez-la pour transformer les issues en pull requests, corriger les bugs à partir d'un commentaire ou automatiser les tâches récurrentes.

Plusieurs produits partagent le nom Claude Code. Cette page couvre l'intégration de workflow `claude-code-action`, que vous configurez avec des fichiers de workflow dans votre repository. Pour les produits connexes, consultez :

* [Code Review](/docs/fr/code-review) : révision automatique sur chaque pull request, sans écrire de workflow
* [Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) : sessions Claude Code qui s'exécutent sur une infrastructure cloud au lieu de votre machine
* [Claude Agent SDK](/docs/fr/agent-sdk/overview) : automatisation personnalisée en dehors de GitHub Actions. La GitHub Action Claude Code est construite sur le SDK
* [GitHub Enterprise Server](/docs/fr/github-enterprise-server) : Claude Code avec GitHub auto-hébergé

<h2 id="setup">
  Configuration
</h2>

Vous pouvez configurer la GitHub Action Claude Code de deux façons :

* **Configuration rapide** : exécutez `/install-github-app` depuis Claude Code. Claude Code installe la GitHub App, ajoute votre secret d'authentification et prépare la pull request de workflow pour vous
* **Configuration manuelle** : installez l'app, ajoutez le secret et copiez le fichier de workflow dans votre repository vous-même. Utilisez ce chemin lorsque vous n'exécutez pas Claude Code localement, lorsque la commande échoue ou lorsque vous voulez un contrôle total des fichiers de workflow

Pour l'un ou l'autre chemin, vous avez besoin d'un accès administrateur au repository.

<h3 id="quick-setup">
  Configuration rapide
</h3>

`/install-github-app` fonctionne uniquement avec les repositories github.com. Si la télécommande git de votre repository se trouve sur gitlab.com ou bitbucket.org, la commande affiche un avis et se termine au lieu de démarrer la configuration. Pour exécuter Claude Code à partir des pipelines GitLab, consultez [Claude Code GitLab CI/CD](/docs/fr/gitlab-ci-cd).

Avant de commencer, installez le [GitHub CLI](https://cli.github.com) et authentifiez-le avec `gh auth login`. Claude Code le vérifie et vous avertit s'il manque.

Ouvrez `claude` dans le repository que vous voulez connecter, exécutez `/install-github-app` et suivez les invites. Claude Code installe la Claude GitHub App, puis configure un secret d'authentification pour les workflows :

* Si Claude Code a déjà une clé API, il réutilise cette clé et propose de conserver le secret `ANTHROPIC_API_KEY` existant du repository s'il en existe un
* Sinon, choisissez entre créer un token de longue durée avec votre abonnement Claude et coller une clé API

Claude Code enregistre les credentials en tant que secret de repository, nommé `ANTHROPIC_API_KEY` pour une clé API ou `CLAUDE_CODE_OAUTH_TOKEN` pour un token d'abonnement.

Claude Code pousse ensuite une branche avec les fichiers de workflow que vous sélectionnez, déjà configurés pour utiliser ce secret, et ouvre GitHub dans votre navigateur avec une pull request prête à créer. Créez et fusionnez cette pull request, et `@claude` fonctionne dans le repository.

Si vous sélectionnez le workflow de révision, Claude publie chaque révision sur la pull request elle-même, en tant que commentaire en ligne sur chaque problème qu'il trouve ou en tant qu'un commentaire de résumé lorsqu'il n'en trouve aucun. Claude ignore certaines pull requests, comme les brouillons. L'[exemple de workflow de révision](#run-a-skill) utilise la même skill et les énumère. Avant v2.1.229, Claude écrivait sa révision uniquement dans le journal d'exécution du workflow.

Pour mettre à jour un workflow de révision qu'une version antérieure a généré, faites l'une des choses suivantes :

* Exécutez `/install-github-app` à nouveau. Lorsque le repository a déjà un `claude.yml`, sélectionnez **Mettre à jour le fichier de workflow avec la dernière version**. Claude Code pousse des copies fraîches des fichiers de workflow vers une nouvelle branche et ouvre la pull request, comme une première installation.
* Ajoutez l'argument `--comment` et la ligne `claude_args` de l'[exemple de workflow de révision](#run-a-skill) au fichier enregistré vous-même, ce qui conserve les autres modifications que vous y avez apportées.

Après l'installation de la GitHub App, Claude Code demande si vous voulez continuer avec la configuration de GitHub Actions. Choisissez **Ignorer pour l'instant** pour arrêter avec seulement la GitHub App installée. Exécutez `/install-github-app` à nouveau plus tard pour terminer les étapes de workflow et de secret.

<Note>
  * Lorsque vous installez la GitHub App, vous lui accordez plusieurs permissions. Consultez [Permissions de la GitHub App](#github-app-permissions) pour l'ensemble complet
  * La configuration rapide fonctionne avec l'API Claude et les abonnements Claude. Si vous utilisez Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, consultez [Utiliser Claude Code GitHub Actions avec les fournisseurs cloud](/docs/fr/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Configuration manuelle
</h3>

Pour configurer la GitHub Action Claude Code sans exécuter `/install-github-app`, installez l'app, ajoutez un secret et copiez un fichier de workflow vous-même :

<Steps>
  <Step title="Installer la Claude GitHub App">
    Installez la [Claude GitHub App](https://github.com/apps/claude) sur votre repository. La GitHub Action Claude Code s'appuie sur trois des permissions de l'app :

    * **Contents** : lecture et écriture, pour que Claude puisse modifier les fichiers du repository
    * **Issues** : lecture et écriture, pour que Claude puisse répondre aux issues
    * **Pull requests** : lecture et écriture, pour que Claude puisse créer des PRs et pousser des modifications

    Lors de l'installation, vous accordez également des permissions que d'autres fonctionnalités Claude utilisent. Consultez [Permissions de la GitHub App](#github-app-permissions) pour l'ensemble complet.
  </Step>

  <Step title="Ajouter un secret d'authentification">
    Ajoutez l'un des secrets suivants à votre repository, selon votre mode d'authentification. Consultez le guide de GitHub sur [l'utilisation des secrets dans GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY` : une clé API Claude de la [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN` : un token OAuth qui s'authentifie avec votre abonnement Claude, disponible sur les plans Pro, Max, Team et Enterprise. Générez-en un en exécutant `claude setup-token` localement. Consultez [Générer un token de longue durée](/docs/fr/authentication#generate-a-long-lived-token)

    Dans les fichiers de workflow, passez le secret à l'entrée correspondante : `anthropic_api_key` pour une clé API, ou `claude_code_oauth_token` pour un token OAuth.
  </Step>

  <Step title="Copier le fichier de workflow">
    Copiez [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) dans le répertoire `.github/workflows/` de votre repository. Le fichier est un workflow fonctionnel, pas seulement un exemple. Tel qu'enregistré, Claude répond chaque fois que quelqu'un mentionne `@claude` dans une issue ou une pull request, en s'authentifiant avec le secret `ANTHROPIC_API_KEY`. Si vous avez ajouté `CLAUDE_CODE_OAUTH_TOKEN` à la place, changez la ligne `anthropic_api_key` du workflow en `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Après la configuration, testez la GitHub Action Claude Code en marquant `@claude` dans un commentaire d'issue ou de PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Configurer pour une organisation
</h3>

Avec la configuration rapide ou manuelle, vous configurez un repository à la fois. Pour déployer la GitHub Action Claude Code dans une organisation :

* Installez la [Claude GitHub App](https://github.com/apps/claude) une fois au niveau de l'organisation, en choisissant tous les repositories ou une liste sélectionnée
* Stockez le secret d'authentification en tant que secret Actions au niveau de l'organisation pour que chaque repository n'ait pas besoin de sa propre copie
* Ajoutez le fichier de workflow à chaque repository qui devrait exécuter la GitHub Action Claude Code, ou définissez le job une fois en tant que [workflow réutilisable](https://docs.github.com/en/actions/using-workflows/reusing-workflows) que chaque repository appelle

Pour un secret partagé entre les repositories, authentifiez-vous avec une clé API de la [Claude Console](https://platform.claude.com) plutôt qu'un token OAuth, car un token OAuth est lié à l'abonnement de la personne qui a exécuté `claude setup-token`.

Pour éviter de stocker un secret de longue durée, authentifiez-vous via la fédération d'identité de charge de travail, où la GitHub Action Claude Code échange le token GitHub OpenID Connect (OIDC) du workflow pour l'accès à l'API Claude via un compte de service de la Claude Console. Définissez ces entrées :

* `anthropic_federation_rule_id` : l'ID de la règle de fédération, `fdrl_...`
* `anthropic_organization_id` : votre ID d'organisation Anthropic
* `anthropic_service_account_id` : l'ID du compte de service, `svac_...`. Optionnel, car la règle de fédération que vous créez dans la Console cible déjà un compte de service
* `anthropic_workspace_id` : l'ID de l'espace de travail, `wrkspc_...`. Optionnel lorsque la règle de fédération cible un seul espace de travail

Accordez au workflow la permission `id-token: write`, que la GitHub Action Claude Code nécessite pour l'échange de fédération même lorsque vous passez votre propre `github_token`. Consultez le [guide de configuration de la GitHub Action Claude Code](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) pour la configuration côté Console.

Pour les questions de traitement et de rétention des données dans un examen de sécurité, consultez [utilisation des données](/docs/fr/data-usage) et [sécurité](/docs/fr/security).

<h3 id="uninstall">
  Désinstaller
</h3>

Pour supprimer la GitHub Action Claude Code, annulez chaque partie de la configuration qui s'applique à votre installation :

* **Fichiers de workflow** : supprimez les workflows qui utilisent `anthropics/claude-code-action` de `.github/workflows/`. Si vous avez utilisé la configuration rapide, recherchez `claude.yml` et, si vous avez sélectionné le workflow de révision, `claude-code-review.yml`. Avec les workflows supprimés, la GitHub Action Claude Code ne s'exécute plus
* **Secrets** : supprimez le secret `ANTHROPIC_API_KEY` ou `CLAUDE_CODE_OAUTH_TOKEN` du repository, et des secrets Actions au niveau de l'organisation si vous l'[avez partagé entre les repositories](#set-up-for-an-organization). Si vous supprimez un secret, les credentials qu'il contenait restent valides. Pour retirer complètement une clé API, supprimez également la clé dans la [Claude Console](https://platform.claude.com)
* **GitHub App** : désinstallez la Claude GitHub App dans les paramètres de votre repository ou organisation sous GitHub Apps, mais seulement si vous ne l'utilisez pas pour une autre fonctionnalité Claude, comme Code Review ou auto-fix web

Si vous avez configuré un [fournisseur cloud](/docs/fr/github-actions-cloud-providers), supprimez également les secrets du fournisseur, comme `AWS_ROLE_TO_ASSUME`, les secrets `GCP_*` ou les secrets `AZURE_*`, et désinstallez la GitHub App personnalisée ainsi que ses secrets `APP_ID` et `APP_PRIVATE_KEY`.

<h3 id="github-app-permissions">
  Permissions de la GitHub App
</h3>

La [Claude GitHub App](https://github.com/apps/claude) est partagée par chaque fonctionnalité Claude qui s'intègre à GitHub, y compris la GitHub Action Claude Code, [Code Review](/docs/fr/code-review) et [auto-fix pour les pull requests](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests) dans les sessions cloud. Une GitHub App a un ensemble de permissions unique couvrant toutes ses fonctionnalités, donc l'ensemble inclut certaines permissions que la GitHub Action Claude Code n'utilise pas.

Lorsque vous installez l'app, vous accordez les permissions suivantes :

| Permission       | Accès               |
| ---------------- | ------------------- |
| Actions          | Lecture et écriture |
| Checks           | Lecture et écriture |
| Contents         | Lecture et écriture |
| Discussions      | Lecture et écriture |
| Issues           | Lecture et écriture |
| Members          | Lecture             |
| Metadata         | Lecture             |
| Pull requests    | Lecture et écriture |
| Repository hooks | Lecture et écriture |
| Statuses         | Lecture             |
| Workflows        | Lecture et écriture |

L'ensemble de permissions peut également changer avant les fonctionnalités qui l'utilisent. Lorsque l'app demande une permission qu'elle n'avait pas auparavant, GitHub demande au propriétaire du compte de l'approuver, à un propriétaire d'organisation pour une installation d'organisation, et l'installation conserve ses anciennes permissions jusqu'à ce qu'ils le fassent. Par exemple, lorsque l'accès Actions passe de lecture à écriture, l'app peut réexécuter les workflows plutôt que seulement afficher les exécutions et les journaux, donc GitHub demande au propriétaire d'approuver le changement.

Lorsque vous installez l'app, vous acceptez son ensemble de permissions complet. GitHub ne vous permet pas d'accepter un sous-ensemble. Si votre organisation nécessite uniquement les permissions que la GitHub Action Claude Code utilise, créez une GitHub App personnalisée avec Contents, Issues et Pull requests à la place, en suivant le [guide de configuration de la GitHub Action Claude Code](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Une app personnalisée couvre uniquement la GitHub Action Claude Code. Code Review et auto-fix web nécessitent toujours l'app officielle.

Pour plus de détails sur la façon dont la GitHub Action Claude Code limite ce que Claude peut faire avec ces permissions, consultez la [documentation de sécurité](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Modes interactif et automatisation
</h2>

La GitHub Action Claude Code détecte comment s'exécuter à partir de votre configuration de workflow :

* **Mode interactif** : lorsque le workflow ne fournit pas d'entrée `prompt`, Claude attend la phrase de déclenchement, `@claude` par défaut, dans un commentaire d'issue ou de pull request, dans une révision de pull request ou dans le corps ou le titre d'une issue nouvellement ouverte, puis répond à cette demande. La progression et les résultats apparaissent en tant que commentaire sur l'issue ou la PR de déclenchement.
* **Mode automatisation** : lorsque le workflow fournit une entrée `prompt`, Claude s'exécute sans attendre une mention, soumis uniquement aux [vérifications sur qui peut déclencher les exécutions](#who-can-trigger-runs). Par défaut, les résultats apparaissent dans le journal d'exécution du workflow plutôt qu'un commentaire. Claude peut publier sur l'issue ou la pull request lorsque le prompt le dirige et qu'il a un outil qui peut publier, comme dans l'[exemple de révision de code](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Qui peut déclencher les exécutions
</h3>

Dans les deux modes, la GitHub Action Claude Code exécute deux vérifications sur l'acteur de déclenchement avant que Claude ne commence, et l'exécution échoue lorsque l'une des deux vérifications la rejette :

* **Accès en écriture** : sur les événements d'issue et de pull request, l'utilisateur de déclenchement doit avoir un accès en écriture au repository. Pour permettre à des utilisateurs spécifiques sans accès en écriture, définissez `allowed_non_write_users` et passez votre propre entrée `github_token`. Les événements qu'aucun utilisateur n'auteur, comme un déclencheur `schedule`, ignorent cette vérification.
* **Acteur humain** : sur chaque événement, la GitHub Action Claude Code rejette un acteur bot à moins que vous le listiez dans `allowed_bots`, ce qui empêche les bots de déclencher Claude dans une boucle. Cette vérification s'applique également aux exécutions programmées, que GitHub attribue à un utilisateur du repository, généralement celui qui a modifié en dernier le calendrier `cron` du workflow. Si cet utilisateur est un bot, listez-le dans `allowed_bots`.

<h2 id="example-use-cases">
  Exemples de cas d'usage
</h2>

Le [répertoire d'exemples](https://github.com/anthropics/claude-code-action/tree/main/examples) contient des workflows prêts à l'emploi pour différents scénarios.

Les exemples de cette page montrent l'authentification par clé API. Si vous vous authentifiez avec un abonnement Claude, remplacez la ligne `anthropic_api_key` dans n'importe quel exemple par `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Répondre aux mentions @claude
</h3>

Ce workflow exécute la GitHub Action Claude Code en mode interactif, donc Claude répond chaque fois que quelqu'un mentionne `@claude` dans un commentaire d'issue ou de PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Les parties de ce workflow qui ne sont pas du code standard :

* `id-token: write` : requis pour l'authentification GitHub App par défaut de la GitHub Action Claude Code
* `actions: read` : permet à Claude de lire les résultats CI sur les PRs
* `actions/checkout` : donne à Claude une copie locale du repository pour travailler
* `if` : empêche les runners de démarrer sur les commentaires qui ne mentionnent pas `@claude`. La GitHub Action Claude Code vérifie également la phrase de déclenchement elle-même avant de répondre

Une fois le workflow en place, mentionnez `@claude` dans n'importe quel commentaire d'issue ou de PR avec une demande :

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude répond dans un commentaire sur la même issue ou PR et le met à jour au fur et à mesure qu'il travaille.

<h3 id="run-a-skill">
  Exécuter une skill
</h3>

L'entrée `prompt` accepte une invocation de [skill](/docs/fr/skills) ainsi que du texte brut :

* Pour une skill dans le répertoire `.claude/skills/` de votre repository, exécutez `actions/checkout` avant l'étape `anthropics/claude-code-action` pour que les fichiers de skill soient disponibles sur le runner, puis passez `/skill-name` comme `prompt`.
* Pour une skill emballée dans un [plugin](/docs/fr/plugins/overview), installez le plugin avec les entrées `plugin_marketplaces` et `plugins`, puis passez le `/plugin-name:skill-name` avec espace de noms comme `prompt`. L'entrée `plugins` prend `plugin-name@marketplace-name`, où le nom de la marketplace provient du manifeste de la marketplace elle-même plutôt que de l'URL de son repository.

Le workflow suivant installe le plugin `code-review` et exécute sa skill lorsqu'une pull request est ouverte, mise à jour, rouverte ou marquée comme prête pour révision. Il exécute le même plugin que le workflow de révision de la configuration rapide. Utilisez un workflow comme celui-ci lorsque vous voulez contrôler le prompt, le modèle et les déclencheurs vous-même. Pour les révisions automatiques sans maintenir un fichier de workflow, consultez [Code Review](/docs/fr/code-review). Sur les repositories publics, GitHub retient les secrets des exécutions déclenchées par des pull requests de fork, donc la révision s'exécute uniquement sur les pull requests des branches du même repository.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Deux lignes dans ce workflow contrôlent où la révision va :

* **`--comment`** : Claude publie sa révision sur la pull request, en tant que commentaire en ligne sur chaque problème qu'il trouve ou en tant qu'un commentaire de résumé lorsqu'il n'en trouve aucun. Sans cela, Claude ne publie rien, et vous lisez les résultats dans le journal d'exécution du workflow.
* **`claude_args`** : conservez cette ligne même si la frontmatter `allowed-tools` propre de la skill nomme le même outil, car la GitHub Action Claude Code démarre le serveur MCP qui publie les commentaires en ligne uniquement lorsque `--allowedTools` dans `claude_args` le nomme.

Claude ignore les pull requests brouillon et fermées, les pull requests qu'il juge ne pas avoir besoin d'une révision, comme les pull requests automatisées ou triviales, et les pull requests qui ont déjà un commentaire de Claude.

<h3 id="run-on-a-schedule">
  Exécuter selon un calendrier
</h3>

Avec une entrée `prompt`, la GitHub Action Claude Code s'exécute en mode automatisation sur n'importe quel événement GitHub, y compris un calendrier cron. Pour un prompt en texte brut, Claude n'a pas d'accès shell ou API GitHub jusqu'à ce que vous accordiez les outils que le prompt nécessite, avec `--allowedTools` dans `claude_args` ou une [règle `permissions.allow`](/docs/fr/permissions#permission-rule-syntax) dans l'entrée `settings`. Si vous invoquez une skill à la place, Claude peut utiliser les outils que sa [frontmatter `allowed-tools`](/docs/fr/skills#pre-approve-tools-for-a-skill) accorde. GitHub exécute les workflows programmés uniquement à partir de la branche par défaut et, dans les repositories publics, désactive le calendrier après 60 jours sans activité du repository.

Ce workflow génère un rapport dans le journal d'exécution du workflow à 09:00 UTC chaque jour. Sa ligne `claude_args` [passe les arguments CLI](#pass-cli-arguments) qui sélectionnent le modèle et permettent deux outils MCP GitHub. Claude lit les commits et les issues via l'API GitHub avec ces outils, donc vous pouvez omettre l'étape de checkout :

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Bonnes pratiques
</h2>

<h3 id="define-project-standards-in-claude-md">
  Définir les normes du projet dans CLAUDE.md
</h3>

Créez un fichier `CLAUDE.md` à la racine de votre repository pour définir les directives de style de code, les critères de révision, les règles spécifiques au projet et les modèles préférés. Claude suit ces directives lors de la création de PRs et de la réponse aux demandes. Consultez la [documentation de mémoire](/docs/fr/memory) pour plus de détails.

<h3 id="protect-your-credentials">
  Protéger vos credentials
</h3>

<Warning>
  Ne commitez jamais les clés API ou les tokens OAuth directement dans votre repository. Stockez-les toujours en tant que GitHub Secrets et référencez-les dans les workflows, par exemple `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Accordez au workflow uniquement les permissions dont il a besoin et examinez les modifications de Claude avant de fusionner.

Pour des conseils de sécurité complets incluant les permissions et l'authentification, consultez la [documentation de sécurité de la GitHub Action Claude Code](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Gérer les coûts
</h3>

Chaque exécution consomme deux types de ressources :

* **Minutes GitHub Actions** : la GitHub Action Claude Code s'exécute sur les runners hébergés par GitHub, qui consomment vos minutes GitHub Actions. Consultez la [documentation de facturation de GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) pour les tarifs et les limites de minutes.
* **Tokens API** : chaque interaction consomme des tokens en fonction de la longueur des prompts et des réponses, de la complexité des tâches et de la taille de la base de code. Consultez la [page de tarification de Claude](https://claude.com/platform/api) pour les tarifs actuels des tokens. Si vous vous authentifiez avec un token OAuth, les exécutions utilisent votre abonnement Claude au lieu de la facturation API.

Vous pouvez réduire les deux types de coûts en donnant à Claude un contexte plus clair et en limitant la quantité de travail que chaque exécution peut faire :

* Écrivez des demandes `@claude` spécifiques pour que Claude ait besoin de moins de tours pour terminer
* Utilisez les modèles d'issue pour fournir du contexte à l'avance
* Gardez votre `CLAUDE.md` concis, car Claude le lit à chaque exécution
* Définissez `--max-turns` dans `claude_args` pour limiter les itérations
* Définissez les délais d'attente au niveau du workflow pour éviter les jobs qui s'exécutent indéfiniment
* Utilisez les contrôles de concurrence de GitHub pour limiter les exécutions parallèles

Pour le suivi de l'utilisation dans votre organisation, consultez le [tableau de bord d'analyse](/docs/fr/analytics) et la [surveillance](/docs/fr/monitoring-usage). Pour savoir comment l'utilisation est mesurée et facturée, consultez [coûts](/docs/fr/costs).

<h2 id="use-a-cloud-provider">
  Utiliser un fournisseur cloud
</h2>

Par défaut, la GitHub Action Claude Code appelle l'API Claude directement avec votre clé API ou token OAuth. Pour acheminer l'inférence via votre propre compte cloud à la place, définissez l'entrée pour votre fournisseur et suivez [Utiliser Claude Code GitHub Actions avec les fournisseurs cloud](/docs/fr/github-actions-cloud-providers) :

* **Amazon Bedrock** : `use_bedrock: "true"`
* **Google Cloud's Agent Platform** : `use_vertex: "true"`
* **Microsoft Foundry** : `use_foundry: "true"`

Avec les trois fournisseurs, vous vous authentifiez via la fédération d'identité OIDC au lieu d'une clé API Claude, donc vous ne stockez aucune credential cloud statique dans votre repository.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude ne répond pas aux commandes @claude
</h3>

* Vérifiez que la GitHub App est installée sur le repository
* Vérifiez que les workflows sont activés pour le repository
* Assurez-vous que votre clé API ou token OAuth est défini dans les secrets du repository
* Confirmez que le commentaire contient `@claude` comme mot complet, pas `/claude` ou `@claude-bot`
* Confirmez que l'utilisateur qui commente a un accès en écriture au repository. Consultez [Qui peut déclencher les exécutions](#who-can-trigger-runs) pour les exceptions

<h3 id="ci-not-running-on-claude’s-commits">
  CI ne s'exécute pas sur les commits de Claude
</h3>

* GitHub ne déclenche pas les workflows sur les commits effectués avec le `GITHUB_TOKEN` par défaut. Si vous passez `github_token: ${{ secrets.GITHUB_TOKEN }}` à la GitHub Action Claude Code, supprimez-le pour qu'il s'authentifie en tant que Claude GitHub App, ou passez un token d'app personnalisé à la place
* Vérifiez que les déclencheurs du workflow CI incluent les événements que les pushes de Claude produisent, comme `push` ou `pull_request`

<h3 id="authentication-errors">
  Erreurs d'authentification
</h3>

* Confirmez que la clé API ou le token OAuth est valide en le testant localement avec `claude` avant de déboguer le workflow
* Pour Bedrock, Agent Platform et Foundry, consultez la [section dépannage](/docs/fr/github-actions-cloud-providers#troubleshooting) de la page du fournisseur cloud

Pour plus de solutions, consultez la [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) de la GitHub Action Claude Code.

<h2 id="advanced-configuration">
  Configuration avancée
</h2>

<h3 id="action-parameters">
  Paramètres de l'action
</h3>

Ce sont les entrées les plus couramment utilisées. Chacune correspond à une clé `with:` dans l'étape `anthropics/claude-code-action`.

| Paramètre                 | Description                                                                                                                                                                                      | Requis                                                                                                                                                                                               |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Instructions pour Claude, en texte brut ou une invocation de [skill](/docs/fr/skills). Lorsqu'il est omis, Claude répond à la [phrase de déclenchement](#interactive-and-automation-modes) à la place | Non                                                                                                                                                                                                  |
| `claude_args`             | Arguments CLI passés à Claude Code                                                                                                                                                               | Non                                                                                                                                                                                                  |
| `anthropic_api_key`       | Clé API Claude                                                                                                                                                                                   | Pour l'API Claude, sauf si vous utilisez `claude_code_oauth_token` ou [fédération d'identité de charge de travail](#set-up-for-an-organization). Non utilisé pour Bedrock, Agent Platform ou Foundry |
| `claude_code_oauth_token` | Token OAuth pour s'authentifier avec un abonnement Claude, généré avec `claude setup-token`                                                                                                      | Non                                                                                                                                                                                                  |
| `github_token`            | Token pour les opérations GitHub. Lorsqu'il est omis, la GitHub Action Claude Code s'authentifie en tant que Claude GitHub App                                                                   | Non                                                                                                                                                                                                  |
| `plugin_marketplaces`     | Liste séparée par des sauts de ligne des URL Git des places de marché de plugins                                                                                                                 | Non                                                                                                                                                                                                  |
| `plugins`                 | Liste séparée par des sauts de ligne des noms de plugins à installer avant l'exécution                                                                                                           | Non                                                                                                                                                                                                  |
| `settings`                | Paramètres Claude Code, en tant que chaîne JSON ou chemin vers un fichier JSON de paramètres                                                                                                     | Non                                                                                                                                                                                                  |
| `trigger_phrase`          | Phrase de déclenchement à laquelle Claude répond. Par défaut : `@claude`                                                                                                                         | Non                                                                                                                                                                                                  |
| `use_bedrock`             | Utiliser Amazon Bedrock au lieu de l'API Claude                                                                                                                                                  | Non                                                                                                                                                                                                  |
| `use_vertex`              | Utiliser Google Cloud's Agent Platform au lieu de l'API Claude                                                                                                                                   | Non                                                                                                                                                                                                  |
| `use_foundry`             | Utiliser Microsoft Foundry au lieu de l'API Claude                                                                                                                                               | Non                                                                                                                                                                                                  |

Pour la liste complète des entrées, consultez la [référence de configuration](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) de la GitHub Action Claude Code.

<h3 id="pass-cli-arguments">
  Passer les arguments CLI
</h3>

Le paramètre `claude_args` accepte n'importe quel [argument CLI Claude Code](/docs/fr/cli-reference) :

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Arguments courants :

* `--max-turns` : limiter le nombre de tours de conversation
* `--model` : modèle à utiliser, par exemple `claude-sonnet-5`. Sans cet argument, la GitHub Action Claude Code utilise le [modèle par défaut](/docs/fr/model-config) de Claude Code
* `--mcp-config` : chemin vers la [configuration MCP](/docs/fr/mcp)
* `--allowedTools` : liste séparée par des virgules des outils autorisés. L'alias `--allowed-tools` fonctionne également
* `--debug` : activer la sortie de débogage

<h2 id="upgrade-from-beta">
  Mettre à niveau depuis la version bêta
</h2>

Si vos workflows font toujours référence à `anthropics/claude-code-action@beta`, mettez-les à jour vers v1 :

1. Changez `@beta` en `@v1` dans la ligne `uses`
2. Supprimez l'entrée `mode`, car la GitHub Action Claude Code [détecte maintenant le mode automatiquement](#interactive-and-automation-modes)
3. Remplacez `direct_prompt` par `prompt`
4. Déplacez les options CLI telles que `max_turns` et `model` dans `claude_args`. `custom_instructions` n'a pas de flag de même nom et devient `--append-system-prompt`

Pour le mappage complet des entrées et les exemples avant et après, consultez le [guide de migration](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Utiliser Claude Code GitHub Actions avec les fournisseurs cloud](/docs/fr/github-actions-cloud-providers) : acheminer l'inférence via Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry
* [Référence de configuration](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) : la liste complète des entrées de l'action
* [Répertoire d'exemples](https://github.com/anthropics/claude-code-action/tree/main/examples) : workflows prêts à l'emploi pour plus de scénarios
* [Code Review](/docs/fr/code-review) : révision automatique des pull requests sans maintenir un fichier de workflow
