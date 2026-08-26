> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lancer des sessions à partir de liens

> Ouvrir une session de terminal Claude Code à partir d'une URL. Intégrez des liens `claude-cli://` dans les runbooks, les alertes et les tableaux de bord pour qu'un clic ouvre Claude Code dans le bon dépôt avec la bonne invite.

Un lien profond est une URL `claude-cli://` qui ouvre Claude Code dans une nouvelle fenêtre de terminal. L'URL peut contenir un répertoire de travail et une invite pour le pré-remplissage.

Cela vous permet de partager un point de départ en un clic pour une tâche : toute personne ayant Claude Code installé qui clique sur le lien voit une session s'ouvrir avec l'invite déjà tapée. L'invite est remplie mais n'est pas envoyée tant que vous n'appuyez pas sur Entrée.

Parce qu'un lien profond est une URL, vous pouvez le placer n'importe où où un lien peut aller :

* Une étape de runbook d'incident qui ouvre le dépôt du service affecté avec une invite de diagnostic
* Une alerte de surveillance ou un tableau de bord qui renvoie à une invite d'investigation pour une métrique spécifique
* Une page README ou wiki qui ouvre le projet avec une invite d'intégration
* Une notification d'échec CI qui pré-remplit le nom du travail défaillant

Cette page explique comment [créer un lien](#build-a-link), [l'intégrer dans un runbook ou le déclencher à partir du shell](#examples), et [gérer ou désactiver l'enregistrement du gestionnaire](#registration-and-supported-platforms) sur chaque plateforme.

<h2 id="how-it-works">
  Fonctionnement
</h2>

Le préfixe `claude-cli://` est un schéma d'URL personnalisé que Claude Code enregistre auprès de votre système d'exploitation, similaire à la façon dont les liens `mailto:` ouvrent votre client de messagerie. Le lien peut se trouver sur une page web, dans un wiki, dans un message Slack ou dans n'importe quelle application qui affiche des liens. Lorsque vous cliquez dessus :

1. Le navigateur ou l'application transmet l'URL à votre système d'exploitation.
2. Le système d'exploitation reconnaît le préfixe `claude-cli://` et démarre Claude Code sur votre machine.
3. Une nouvelle fenêtre de terminal s'ouvre avec Claude Code s'exécutant dans le répertoire spécifié par le lien, et le texte d'invite du lien déjà dans la zone de saisie.
4. Vous lisez l'invite, la modifiez si vous le souhaitez, et appuyez sur Entrée pour l'envoyer.

Le lien lui-même peut être hébergé n'importe où, mais la session s'ouvre toujours localement sur l'ordinateur où vous avez cliqué. Consultez [Enregistrement et plateformes prises en charge](#registration-and-supported-platforms) pour savoir quel émulateur de terminal s'ouvre sur chaque système d'exploitation.

<Note>
  La plateforme qui affiche le lien doit autoriser les schémas d'URL personnalisés. Le Markdown rendu par GitHub autorise `http` et `https` mais supprime les schémas comme `claude-cli://` dans les README, les problèmes, les demandes de tirage et les wikis. Seul le texte du lien s'affiche, sans lien derrière et l'URL masquée. Consultez [Dépannage](#the-link-renders-as-plain-text-instead-of-being-clickable) pour une solution de contournement.
</Note>

<h3 id="what-a-launched-session-shows">
  Ce qu'une session lancée affiche
</h3>

Un lien profond n'exécute jamais rien de lui-même. Le lien choisit uniquement un répertoire et remplit la zone d'invite. Si vous cliquez sur un lien d'une page en laquelle vous n'avez pas confiance, l'invite reste inerte : rien n'atteint le modèle tant que vous n'avez pas lu ce qui a été rempli et appuyé sur Entrée.

Lorsque la session s'ouvre, une ligne d'avertissement sous la zone de saisie indique « Invite d'un lien externe » et reste visible jusqu'à ce que vous envoyiez ou effaciez l'invite. Pour les invites de plus de 1 000 caractères, l'avertissement inclut le nombre de caractères et vous indique de faire défiler et d'examiner le texte complet avant d'appuyer sur Entrée, car les longues invites peuvent repousser les instructions hors de l'écran. Les règles de permission, `CLAUDE.md` et les invites de confiance pour le répertoire sélectionné s'appliquent de la même manière que pour toute autre session.

<h2 id="build-a-link">
  Créer un lien
</h2>

Chaque lien profond commence par `claude-cli://open`, qui est le seul chemin que le gestionnaire accepte, suivi de paramètres de requête optionnels. La forme minimale ouvre Claude Code dans votre répertoire personnel avec une invite vide :

```text theme={null}
claude-cli://open
```

Ajoutez des paramètres pour contrôler où la session démarre et ce que contient la zone d'invite :

| Paramètre | Description                                                                                                                                                                                                                                                                 |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Texte à pré-remplir dans la zone d'invite. [Encodez en URL](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) la valeur. Utilisez `%0A` pour les sauts de ligne dans les invites multi-lignes. Maximum 5 000 caractères. |
| `cwd`     | Chemin absolu à utiliser comme répertoire de travail. Les chemins réseau et UNC sont rejetés, tout comme les chemins qui contiennent des caractères de contrôle invisibles ou bidirectionnels.                                                                              |
| `repo`    | Un slug GitHub `owner/name`. Claude Code le résout en un clone local qu'il a déjà vu et démarre là. Si vous n'avez pas de clone correspondant, la session s'ouvre dans votre répertoire personnel à la place.                                                               |

`cwd` et `repo` sont [deux façons de définir le répertoire de travail](#choose-between-cwd-and-repo). Si vous passez les deux, `cwd` a la priorité et `repo` est ignoré, même si le chemin `cwd` n'existe pas.

Le lien suivant pointe vers un dépôt appelé `acme/payments` avec une invite de diagnostic sur deux lignes. Remplacez `acme/payments` par le slug `owner/name` de votre dépôt lorsque vous créez le vôtre :

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

En cliquant dessus, une nouvelle fenêtre de terminal s'ouvre, Claude Code démarre dans votre clone local de `acme/payments`, et la zone d'invite est remplie avec le texte décodé :

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Vous pouvez modifier l'invite avant d'appuyer sur Entrée pour l'envoyer. Si vous n'avez pas de clone local du dépôt, la session s'ouvre dans votre répertoire personnel à la place. Consultez [Choisir entre `cwd` et `repo`](#choose-between-cwd-and-repo) pour savoir comment le chemin local est sélectionné lorsque vous avez plusieurs clones ou worktrees.

<h3 id="choose-between-cwd-and-repo">
  Choisir entre `cwd` et `repo`
</h3>

Utilisez `cwd` lorsque tous ceux qui cliquent sur le lien ont le projet au même chemin absolu, comme un devcontainer standardisé ou une image de machine virtuelle.

Utilisez `repo` lorsque le lien est partagé et que chaque personne clone à un emplacement différent. Claude Code résout le slug en chemin local comme suit :

* Chaque fois que vous exécutez `claude` dans un dépôt Git, le chemin du système de fichiers de ce répertoire est enregistré par rapport au slug GitHub `owner/name` du dépôt.
* Lorsqu'un lien profond arrive, `repo` ouvre le chemin correspondant que vous avez utilisé le plus récemment. Les clones multiples et les worktrees sont suivis séparément, donc il choisit celui dans lequel vous avez travaillé en dernier.
* La recherche ne trouve que les chemins où vous avez déjà exécuté Claude Code au moins une fois.
* Le lien ne change pas la branche qui est extraite. La session s'ouvre dans l'état actuel de ce répertoire.

L'en-tête de bienvenue affiche le chemin qu'il a choisi afin que vous puissiez confirmer que le bon clone s'est ouvert.

<h2 id="examples">
  Exemples
</h2>

Les sections ci-dessous montrent deux façons courantes d'utiliser un lien profond : comme un lien Markdown dans un document et comme une commande dans un script ou un alias shell.

<h3 id="embed-a-link-in-a-runbook">
  Intégrer un lien dans un runbook
</h3>

Un lien profond dans un runbook donne à celui qui trie un moyen en un clic de commencer à enquêter dans le bon dépôt avec une invite préparée. La plateforme qui affiche le runbook doit autoriser les schémas d'URL personnalisés. Le Markdown rendu par GitHub n'autorise pas `claude-cli://`, donc un lien profond dans un README, un problème ou un wiki GitHub affiche uniquement son étiquette sans lien cliquable. Consultez [la note de dépannage](#the-link-renders-as-plain-text-instead-of-being-clickable) pour une solution de contournement.

L'invite fait partie de l'URL et doit être encodée en URL. Pour produire la valeur encodée, passez votre texte d'invite par `encodeURIComponent` dans une console de navigateur ou n'importe quel encodeur d'URL.

L'exemple ci-dessous ajoute un point d'entrée d'investigation à un runbook d'incident pour un service appelé `web-gateway` :

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Pour utiliser ceci dans votre propre runbook, remplacez `acme/web-gateway` par le slug du dépôt de votre service. Cela permet aux ingénieurs ayant Claude Code installé et un clone local de ce dépôt de cliquer sur l'étape 2 et de commencer à enquêter avec l'invite prête à être envoyée.

<h3 id="open-a-link-from-the-shell">
  Ouvrir un lien à partir du shell
</h3>

Vous pouvez également ouvrir un lien profond à partir d'un script shell, d'un alias ou d'une automatisation plutôt que de cliquer dessus. Appelez la commande d'ouverture d'URL de votre système d'exploitation avec le lien comme argument.

<Tabs>
  <Tab title="macOS">
    La commande `open` intégrée transmet l'URL au gestionnaire `claude-cli://` enregistré :

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    La plupart des environnements de bureau fournissent `xdg-open`, qui transmet l'URL au gestionnaire enregistré :

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    Dans PowerShell, `Start-Process` transmet l'URL au gestionnaire enregistré :

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    Dans `cmd.exe`, `start` traite son premier argument entre guillemets comme un titre de fenêtre, donc passez un titre vide avant l'URL :

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Enregistrement et plateformes prises en charge
</h2>

Claude Code enregistre le gestionnaire `claude-cli://` auprès de votre système d'exploitation la première fois que vous démarrez une session interactive sur macOS, Linux et Windows. Vous n'exécutez pas de commande d'installation séparée. L'enregistrement écrit uniquement dans des emplacements au niveau de l'utilisateur :

| Plateforme | Emplacement du gestionnaire                                                                                    |
| ---------- | -------------------------------------------------------------------------------------------------------------- |
| macOS      | `~/Applications/Claude Code URL Handler.app`                                                                   |
| Linux      | `claude-code-url-handler.desktop` sous `$XDG_DATA_HOME/applications`, par défaut `~/.local/share/applications` |
| Windows    | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                |

Le gestionnaire lance Claude Code dans un émulateur de terminal détecté. Sur macOS, Claude Code se souvient du terminal de votre session interactive la plus récente et le réutilise, en prenant en charge iTerm2, Ghostty, kitty, Alacritty, WezTerm et Terminal.app. Sur Linux, il honore la variable d'environnement `$TERMINAL`, puis `x-terminal-emulator`, puis une liste d'émulateurs courants. Sur Windows, il préfère Windows Terminal, puis PowerShell, puis `cmd.exe`.

Pour empêcher complètement l'enregistrement, définissez [`disableDeepLinkRegistration`](/docs/fr/settings) sur `"disable"` dans `settings.json`. Pour appliquer ceci dans toute une organisation afin que les utilisateurs ne puissent pas le réactiver, définissez-le dans [les paramètres gérés](/docs/fr/server-managed-settings) à la place.

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Ouvrir un onglet VS Code au lieu d'un terminal
</h2>

L'extension VS Code enregistre son propre gestionnaire à `vscode://anthropic.claude-code/open`, qui ouvre un onglet de l'éditeur Claude Code plutôt qu'une fenêtre de terminal. Consultez [Lancer un onglet VS Code à partir d'autres outils](/docs/fr/vs-code#launch-a-vs-code-tab-from-other-tools) pour les paramètres de cette URL.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="clicking-the-link-does-nothing">
  Cliquer sur le lien ne fait rien
</h3>

Le gestionnaire n'est probablement pas encore enregistré. Démarrez une session `claude` interactive une fois sur cette machine, quittez, et réessayez le lien. Si vous êtes sur Linux sans environnement de bureau, `xdg-open` n'a peut-être rien à dispatcher.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  Le lien s'affiche en tant que texte brut au lieu d'être cliquable
</h3>

Certains moteurs de rendu Markdown n'autorisent que les liens `http` et `https` et suppriment les autres schémas d'URL. GitHub le fait dans les README, les problèmes, les demandes de tirage et les wikis : `[label](claude-cli://...)` s'affiche simplement comme `label`, sans lien et l'URL supprimée. Sur ces plateformes, mettez le lien profond dans un bloc de code afin que les lecteurs puissent voir l'URL et la coller dans la barre d'adresse de leur navigateur.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  La session s'ouvre dans mon répertoire personnel au lieu du dépôt
</h3>

Le paramètre `repo` ne résout que les clones que Claude Code a déjà vus. Exécutez `claude` à l'intérieur du clone une fois pour que son chemin soit enregistré, ou basculez le lien pour utiliser `cwd` avec un chemin absolu.

<h3 id="the-link-opens-the-wrong-terminal">
  Le lien ouvre le mauvais terminal
</h3>

Sur macOS, démarrez `claude` dans votre terminal préféré une fois et le prochain lien profond l'utilisera. Sur Linux, définissez la variable d'environnement `$TERMINAL` sur le nom de commande de votre émulateur préféré. Sur Windows, l'ordre est fixe : installez Windows Terminal si vous voulez que les liens s'ouvrent là au lieu d'une fenêtre PowerShell ou `cmd.exe`.

<h2 id="learn-more">
  En savoir plus
</h2>

Ces pages couvrent les façons connexes de lancer ou d'étendre les sessions Claude Code :

* [Skills](/docs/fr/skills) : stockez une longue invite de runbook en tant que `/skill` dans le dépôt afin que le paramètre `q` du lien profond n'ait qu'à la nommer
* [Mode non interactif](/docs/fr/headless) : exécutez Claude à partir d'un script et capturez la sortie sans ouvrir de terminal
