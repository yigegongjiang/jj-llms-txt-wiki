> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrage rapide

> Bienvenue dans Claude Code !

Ce guide de démarrage rapide vous permettra d'utiliser l'assistance au codage alimentée par l'IA en quelques minutes. À la fin, vous comprendrez comment utiliser Claude Code pour les tâches de développement courantes.

<h2 id="before-you-begin">
  Avant de commencer
</h2>

Assurez-vous que vous avez :

* Un terminal ou une invite de commande ouvert
  * Si vous n'avez jamais utilisé le terminal auparavant, consultez le [guide du terminal](/docs/fr/terminal-guide)
* Un projet de code avec lequel travailler
* Un [abonnement Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team ou Enterprise), un compte [Claude Console](https://platform.claude.com/), ou un accès via un [fournisseur cloud pris en charge](/docs/fr/third-party-integrations)

<Note>
  Ce guide couvre le CLI du terminal. Claude Code est également disponible sur le [web](https://claude.ai/code), en tant qu'[application de bureau](/docs/fr/desktop), dans [VS Code](/docs/fr/vs-code) et [les IDE JetBrains](/docs/fr/jetbrains), dans [Slack](/docs/fr/slack), et en CI/CD avec [GitHub Actions](/docs/fr/github-actions) et [GitLab](/docs/fr/gitlab-ci-cd). Voir [toutes les interfaces](/docs/fr/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Étape 1 : Installer Claude Code
</h2>

Pour installer Claude Code, utilisez l'une des méthodes suivantes :

<Tabs>
  <Tab title="Installation native (recommandée)">
    **macOS, Linux, WSL :**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell :**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD :**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    Si vous voyez `The token '&&' is not a valid statement separator`, vous êtes dans PowerShell, pas dans CMD. Si vous voyez `'irm' is not recognized as an internal or external command`, vous êtes dans CMD, pas dans PowerShell. Votre invite affiche `PS C:\` quand vous êtes dans PowerShell et `C:\` sans le `PS` quand vous êtes dans CMD.

    Si la commande d'installation échoue avec `syntax error near unexpected token '<'`, un `403`, ou une autre erreur curl, consultez [Dépannage de l'installation](/docs/fr/troubleshoot-install#find-your-error) pour faire correspondre l'erreur à une solution et pour connaître les méthodes d'installation alternatives.

    [Git for Windows](https://git-scm.com/downloads/win) est recommandé sur Windows natif afin que Claude Code puisse utiliser l'outil Bash. Si Git for Windows n'est pas installé, Claude Code utilise PowerShell comme outil shell à la place. Les configurations WSL n'ont pas besoin de Git for Windows.

    <Info>
      Les installations natives se mettent à jour automatiquement en arrière-plan pour vous maintenir à jour avec la dernière version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew propose deux casks. `claude-code` suit le canal de version stable, qui est généralement environ une semaine en retard et ignore les versions avec des régressions majeures. `claude-code@latest` suit le canal le plus récent et reçoit les nouvelles versions dès qu'elles sont publiées.

    <Info>
      Les installations Homebrew ne se mettent pas à jour automatiquement. Exécutez `brew upgrade claude-code` ou `brew upgrade claude-code@latest`, selon le cask que vous avez installé, pour obtenir les dernières fonctionnalités et correctifs de sécurité.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      Les installations WinGet ne se mettent pas à jour automatiquement. Exécutez `winget upgrade Anthropic.ClaudeCode` périodiquement pour obtenir les dernières fonctionnalités et correctifs de sécurité.
    </Info>
  </Tab>
</Tabs>

Vous pouvez également installer avec [apt, dnf, ou apk](/docs/fr/setup#install-with-linux-package-managers) sur Debian, Fedora, RHEL et Alpine.

Pour confirmer que l'installation a fonctionné, exécutez :

```bash theme={null}
claude --version
```

La commande affiche un numéro de version suivi de `(Claude Code)`.

<h2 id="step-2-log-in-to-your-account">
  Étape 2 : Se connecter à votre compte
</h2>

Claude Code nécessite un compte pour être utilisé. Démarrez une session interactive avec la commande `claude` et vous serez invité à vous connecter lors de la première utilisation :

```bash theme={null}
claude
```

Pour les comptes Claude abonnement ou Console, suivez les invites pour terminer l'authentification dans votre navigateur. Si vous avez défini la variable d'environnement `ANTHROPIC_API_KEY`, Claude Code ignore l'invite de connexion et vous demande d'approuver la clé à la place. Pour changer de compte ultérieurement ou vous réauthentifier, tapez `/login` dans la session en cours :

```text wrap theme={null}
/login
```

Vous pouvez vous connecter en utilisant l'un de ces types de compte :

* [Claude Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (recommandé)
* [Claude Console](https://platform.claude.com/) (accès API avec crédits prépayés). Lors de la première connexion, un espace de travail « Claude Code » est automatiquement créé dans la Console pour un suivi centralisé des coûts.
* [Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/fr/third-party-integrations) (fournisseurs cloud d'entreprise)
* Une passerelle [Claude apps gateway](/docs/fr/claude-apps-gateway) auto-hébergée, si votre organisation en exécute une : votre administrateur préconfigure l'URL de la passerelle, et `/login` ouvre directement l'écran **Cloud gateway** pour que vous vous connectiez avec l'authentification unique d'entreprise

Une fois connecté, vos identifiants sont stockés et vous n'aurez pas besoin de vous reconnecter. En savoir plus dans [Gestion des identifiants](/docs/fr/authentication#credential-management).

<h2 id="step-3-start-your-first-session">
  Étape 3 : Démarrer votre première session
</h2>

Ouvrez votre terminal dans n'importe quel répertoire de projet et démarrez Claude Code :

```bash theme={null}
cd /path/to/your/project
claude
```

Remplacez `/path/to/your/project` par le chemin du projet sur lequel vous souhaitez travailler.

Vous verrez l'invite de Claude Code avec la version, le modèle actuel et le répertoire de travail affichés au-dessus. Tapez `/help` pour les commandes disponibles ou `/resume` pour continuer une conversation précédente.

<h2 id="step-4-ask-your-first-question">
  Étape 4 : Posez votre première question
</h2>

Commençons par comprendre votre base de code. Essayez l'une de ces commandes :

```text wrap theme={null}
what does this project do?
```

Claude analysera vos fichiers et fournira un résumé. Vous pouvez également poser des questions plus spécifiques :

```text wrap theme={null}
what technologies does this project use?
```

```text wrap theme={null}
where is the main entry point?
```

```text wrap theme={null}
explain the folder structure
```

Vous pouvez également demander à Claude ses propres capacités :

```text wrap theme={null}
what can Claude Code do?
```

```text wrap theme={null}
how do I create custom skills in Claude Code?
```

```text wrap theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code lit vos fichiers de projet selon les besoins. Vous n'avez pas à ajouter manuellement du contexte.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Étape 5 : Effectuez votre première modification de code
</h2>

Maintenant, faisons en sorte que Claude Code fasse du vrai codage. Essayez une tâche simple :

```text wrap theme={null}
add a hello world function to the main file
```

Claude Code trouve le fichier approprié et vous montre la modification. S'il vous demande avant d'effectuer la modification, sélectionnez **Oui** pour approuver.

Le mode Auto est le [mode de permission de démarrage intégré](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour les sessions de terminal interactives sur les plans Pro, Max et Team : un classificateur examine les actions au lieu de vous, et Claude modifie la plupart des fichiers et exécute la plupart des commandes sans vous demander. Sur les autres plans, le mode Manuel est le mode de permission de démarrage intégré. Pour la session que vous démarrez juste après l'installation, consultez [Première session après une installation ou une mise à jour](/docs/fr/env-vars#first-session-after-an-install-or-upgrade).

<Note>
  Vos paramètres ou votre organisation peuvent définir un mode de permission de démarrage différent. [Quel mode de permission une session démarre](/docs/fr/permission-modes#which-mode-a-session-starts-in) énumère ce qui le fait. Appuyez sur `Shift+Tab` à tout moment pour basculer le mode de permission de la session dans laquelle vous vous trouvez.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Étape 6 : Utiliser Git avec Claude Code
</h2>

Claude Code rend les opérations Git conversationnelles :

```text wrap theme={null}
what files have I changed?
```

```text wrap theme={null}
commit my changes with a descriptive message
```

Vous pouvez également demander des opérations Git plus complexes :

```text wrap theme={null}
create a new branch called feature/quickstart
```

```text wrap theme={null}
show me the last 5 commits
```

```text wrap theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Étape 7 : Corriger un bug ou ajouter une fonctionnalité
</h2>

Claude est compétent pour le débogage et l'implémentation de fonctionnalités.

Décrivez ce que vous voulez en langage naturel :

```text wrap theme={null}
add input validation to the user registration form
```

Ou corrigez les problèmes existants :

```text wrap theme={null}
there's a bug where users can submit empty forms - fix it
```

Claude Code va :

* Localiser le code pertinent
* Comprendre le contexte
* Implémenter une solution
* Exécuter les tests si disponibles

<h2 id="step-8-test-out-other-common-workflows">
  Étape 8 : Testez d'autres flux de travail courants
</h2>

Il existe plusieurs façons de travailler avec Claude :

**Refactoriser le code**

```text wrap theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Écrire des tests**

```text wrap theme={null}
write unit tests for the calculator functions
```

**Mettre à jour la documentation**

```text wrap theme={null}
update the README with installation instructions
```

**Révision de code**

```text wrap theme={null}
review my changes and suggest improvements
```

<Tip>
  Parlez à Claude comme vous le feriez avec un collègue utile. Décrivez ce que vous voulez réaliser, et il vous aidera à y arriver.
</Tip>

<h2 id="essential-commands">
  Commandes essentielles
</h2>

Voici les commandes les plus importantes pour l'utilisation quotidienne. Les commandes shell s'exécutent depuis votre terminal pour démarrer ou reprendre Claude Code. Les commandes de session s'exécutent à l'intérieur de Claude Code après son démarrage.

**Commandes shell**

| Commande            | Ce qu'elle fait                                                     | Exemple                             |
| ------------------- | ------------------------------------------------------------------- | ----------------------------------- |
| `claude`            | Démarrer le mode interactif                                         | `claude`                            |
| `claude "task"`     | Démarrer le mode interactif avec une invite initiale                | `claude "fix the build error"`      |
| `claude -p "query"` | Exécuter une requête unique, puis quitter                           | `claude -p "explain this function"` |
| `claude -c`         | Continuer la conversation la plus récente dans le répertoire actuel | `claude -c`                         |
| `claude -r`         | Reprendre une conversation précédente                               | `claude -r`                         |

**Commandes de session**

| Commande                    | Ce qu'elle fait                        | Exemple  |
| --------------------------- | -------------------------------------- | -------- |
| `/clear`                    | Effacer l'historique des conversations | `/clear` |
| `/help`                     | Afficher les commandes disponibles     | `/help`  |
| `/exit` ou Ctrl+D deux fois | Quitter Claude Code                    | `/exit`  |

Voir la [référence CLI](/docs/fr/cli-reference) pour la liste complète des commandes shell et la [référence des commandes](/docs/fr/commands) pour la liste complète des commandes de session.

<h2 id="pro-tips-for-beginners">
  Conseils professionnels pour les débutants
</h2>

Pour plus d'informations, voir [les meilleures pratiques](/docs/fr/best-practices) et [les flux de travail courants](/docs/fr/common-workflows).

<AccordionGroup>
  <Accordion title="Soyez spécifique dans vos demandes">
    Au lieu de : ' corriger le bug '

    Essayez : ' corriger le bug de connexion où les utilisateurs voient un écran vide après avoir entré des identifiants incorrects '
  </Accordion>

  <Accordion title="Utilisez des instructions étape par étape">
    Divisez les tâches complexes en étapes :

    ```text wrap theme={null}
    1. créer une nouvelle table de base de données pour les profils utilisateur
    2. créer un endpoint API pour obtenir et mettre à jour les profils utilisateur
    3. construire une page web qui permet aux utilisateurs de voir et modifier leurs informations
    ```
  </Accordion>

  <Accordion title="Laissez Claude explorer d'abord">
    Avant de faire des modifications, laissez Claude comprendre votre code :

    ```text wrap theme={null}
    analyser le schéma de la base de données
    ```

    ```text wrap theme={null}
    construire un tableau de bord montrant les produits les plus fréquemment retournés par nos clients au Royaume-Uni
    ```
  </Accordion>

  <Accordion title="Gagnez du temps avec les raccourcis">
    * Tapez `/` pour voir les commandes et skills disponibles
    * Utilisez Tab pour la complétion des commandes
    * Appuyez sur ↑ pour l'historique des commandes
    * Appuyez sur `Shift+Tab` pour parcourir les modes de permission
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Prochaines étapes
</h2>

Maintenant que vous avez appris les bases, explorez des fonctionnalités plus avancées :

<CardGroup cols={2}>
  <Card title="Comment fonctionne Claude Code" icon="microchip" href="/docs/fr/how-claude-code-works">
    Comprendre la boucle agentique, les outils intégrés et comment Claude Code interagit avec votre projet
  </Card>

  <Card title="Meilleures pratiques" icon="star" href="/docs/fr/best-practices">
    Obtenez de meilleurs résultats avec un prompting efficace et une configuration de projet appropriée
  </Card>

  <Card title="Flux de travail courants" icon="graduation-cap" href="/docs/fr/common-workflows">
    Guides étape par étape pour les tâches courantes
  </Card>

  <Card title="Étendre Claude Code" icon="puzzle-piece" href="/docs/fr/features-overview">
    Personnalisez avec CLAUDE.md, skills, hooks, MCP et bien plus
  </Card>
</CardGroup>

<h2 id="getting-help">
  Obtenir de l'aide
</h2>

* **Dans Claude Code** : Tapez `/help` ou demandez « comment faire... »
* **Documentation** : Vous êtes ici ! Parcourez les autres guides
* **Cours** : Suivez [Claude Code 101](https://academy.claude.com/courses/claude-code-101) et d'autres cours gratuits à votre rythme sur [Claude Academy](https://academy.claude.com/)
* **Communauté** : Rejoignez notre [Discord](https://www.anthropic.com/discord) pour des conseils et du support
