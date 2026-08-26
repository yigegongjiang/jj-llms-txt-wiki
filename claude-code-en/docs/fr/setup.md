> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration avancée

> Configuration requise, installation spécifique à la plateforme, gestion des versions et désinstallation pour Claude Code.

Cette page couvre la configuration requise, les détails d'installation spécifiques à la plateforme, les mises à jour et la désinstallation. Pour une présentation guidée de votre première session, consultez le [démarrage rapide](/docs/fr/quickstart). Si vous n'avez jamais utilisé un terminal auparavant, consultez le [guide du terminal](/docs/fr/terminal-guide).

<h2 id="system-requirements">
  Configuration requise
</h2>

Claude Code s'exécute sur les plateformes et configurations suivantes :

* **Système d'exploitation** :
  * macOS 13.0+
  * Windows 10 1809+ ou Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **Matériel** : 4 Go+ de RAM, processeur x64 ou ARM64
* **Réseau** : connexion Internet requise. Consultez la [configuration réseau](/docs/fr/network-config#network-access-requirements).
* **Shell** : Bash, Zsh, PowerShell ou CMD.
* **Localisation** : [pays supportés par Anthropic](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  Dépendances supplémentaires
</h3>

* **ripgrep** : généralement inclus avec Claude Code. Si la recherche échoue, consultez le [dépannage de la recherche](/docs/fr/troubleshooting#search-and-discovery-issues).

<h2 id="install-claude-code">
  Installer Claude Code
</h2>

<Tip>
  Préférez une interface graphique ? L'[application de bureau](/docs/fr/desktop-quickstart) vous permet d'utiliser Claude Code sans le terminal. Téléchargez-la pour [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) ou [Linux](/docs/fr/desktop-linux).

  Nouveau sur le terminal ? Consultez le [guide du terminal](/docs/fr/terminal-guide) pour des instructions étape par étape.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

Une fois l'installation terminée, ouvrez un terminal dans le projet sur lequel vous souhaitez travailler et démarrez Claude Code :

```bash theme={null}
claude
```

Si vous rencontrez des problèmes lors de l'installation, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

<h3 id="set-up-on-windows">
  Configuration sur Windows
</h3>

Vous pouvez exécuter Claude Code nativement sur Windows ou à l'intérieur de WSL. Choisissez en fonction de l'endroit où vos projets sont situés et des fonctionnalités dont vous avez besoin :

| Option        | Nécessite                                                                  | [Sandboxing](/docs/fr/sandboxing) | Quand l'utiliser                                            |
| ------------- | -------------------------------------------------------------------------- | ---------------------------- | ----------------------------------------------------------- |
| Windows natif | Aucun ; [Git for Windows](https://git-scm.com/downloads/win) est optionnel | Non supporté                 | Projets et outils Windows natifs                            |
| WSL 2         | WSL 2 activé                                                               | Supporté                     | Chaînes d'outils Linux ou exécution de commandes en sandbox |
| WSL 1         | WSL 1 activé                                                               | Non supporté                 | Si WSL 2 n'est pas disponible                               |

**Option 1 : Windows natif**

Exécutez la commande d'installation à partir de PowerShell ou CMD. Vous n'avez pas besoin d'exécuter en tant qu'administrateur. L'installation de [Git for Windows](https://git-scm.com/downloads/win) est optionnelle. Elle active l'[outil Bash](/docs/fr/tools-reference#bash-tool-behavior) en fournissant Git Bash.

Que vous installiez à partir de PowerShell ou CMD affecte uniquement la commande d'installation que vous exécutez. Votre invite affiche `PS C:\Users\VotreNom>` dans PowerShell et `C:\Users\VotreNom>` sans le `PS` dans CMD. Si vous êtes nouveau sur le terminal, le [guide du terminal](/docs/fr/terminal-guide#windows) vous guide à travers chaque étape.

Après l'installation, lancez `claude` à partir de n'importe quel terminal.

* **Sans Git for Windows**, Claude Code exécute les commandes shell via l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool).
* **Avec Git for Windows**, Claude Code utilise Git Bash pour l'[outil Bash](/docs/fr/tools-reference#bash-tool-behavior). Si Claude Code ne trouve pas Git Bash, définissez le chemin dans votre [fichier settings.json](/docs/fr/settings) :

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Lorsque Git for Windows est installé, l'outil PowerShell est déployé progressivement en tant qu'option supplémentaire aux côtés de Bash. Définissez `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` pour l'activer ou `0` pour le désactiver. Consultez [outil PowerShell](/docs/fr/tools-reference#powershell-tool) pour la configuration et les limitations.

**Option 2 : WSL**

Ouvrez votre distribution WSL et exécutez le programme d'installation Linux à partir des [instructions d'installation](#install-claude-code) ci-dessus. Vous installez et lancez `claude` à l'intérieur du terminal WSL, pas à partir de PowerShell ou CMD.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux et distributions basées sur musl
</h3>

L'installateur natif sur Alpine et autres distributions basées sur musl/uClibc nécessite `libgcc`, `libstdc++` et `ripgrep`. Installez-les à l'aide du gestionnaire de paquets de votre distribution, puis définissez `USE_BUILTIN_RIPGREP=0`.

Cet exemple installe les paquets requis sur Alpine :

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

Ensuite, définissez `USE_BUILTIN_RIPGREP` à `0` dans votre fichier [`settings.json`](/docs/fr/settings#available-settings) :

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  Vérifier votre installation
</h2>

Après l'installation, confirmez que Claude Code fonctionne :

```bash theme={null}
claude --version
```

Si cela échoue avec `command not found` ou une autre erreur, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

Pour une vérification plus détaillée de votre installation et configuration, exécutez [`claude doctor`](/docs/fr/troubleshooting#get-more-help) :

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  S'authentifier
</h2>

Claude Code nécessite un compte Pro, Max, Team, Enterprise ou Console. Le plan gratuit Claude.ai n'inclut pas l'accès à Claude Code. Vous pouvez également utiliser Claude Code avec un fournisseur d'API tiers comme [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou [Microsoft Foundry](/docs/fr/microsoft-foundry).

Après l'installation, connectez-vous en exécutant `claude` et en suivant les invites du navigateur. Consultez [Authentification](/docs/fr/authentication) pour tous les types de comptes et les options de configuration d'équipe.

<h2 id="update-claude-code">
  Mettre à jour Claude Code
</h2>

Les installations natives se mettent à jour automatiquement en arrière-plan. Vous pouvez [configurer le canal de version](#configure-release-channel) pour contrôler si vous recevez les mises à jour immédiatement ou selon un calendrier stable retardé, ou [désactiver les mises à jour automatiques](#disable-auto-updates) entièrement. Les installations Homebrew, WinGet et [gestionnaire de paquets Linux](#install-with-linux-package-managers) nécessitent des mises à jour manuelles par défaut.

<h3 id="auto-updates">
  Mises à jour automatiques
</h3>

Claude Code vérifie les mises à jour au démarrage et périodiquement pendant l'exécution. Les mises à jour se téléchargent et s'installent en arrière-plan, puis prennent effet la prochaine fois que vous démarrez Claude Code.

Exécutez `claude doctor` pour voir le résultat de la tentative de mise à jour la plus récente.

Sur macOS et Linux, le programme d'installation natif gère le lanceur à `~/.local/bin/claude` en tant que lien symbolique vers `~/.local/share/claude/versions/`. Si vous remplacez ce lanceur par votre propre script ou lien symbolique, la mise à jour automatique et `claude update` le laissent en place : les nouvelles versions s'installent toujours sous le répertoire `versions/`, et votre lanceur décide quelle version s'exécute. Avant la v2.1.207, le programme de mise à jour automatique remplaçait un lanceur personnalisé à ce chemin par son propre lien symbolique à chaque mise à jour.

Avec un lanceur personnalisé, Claude Code conserve également chaque version installée sur le disque car il ne peut pas déterminer quelle version le lanceur nécessite. `claude doctor` signale un lanceur que le programme d'installation natif n'a pas créé.

Pour laisser Claude Code gérer à nouveau le lanceur, supprimez `~/.local/bin/claude` et exécutez `claude update`.

Si une installation npm globale ne peut pas se mettre à jour automatiquement car le répertoire npm global n'est pas accessible en écriture, Claude Code affiche un avis unique au démarrage, et `claude doctor` répertorie les correctifs disponibles. Consultez [erreurs de permission lors de l'installation](/docs/fr/troubleshoot-install#permission-errors-during-installation) pour plus de détails.

<Note>
  Les installations Homebrew, WinGet, apt, dnf et apk ne se mettent pas à jour automatiquement par défaut ; consultez ci-dessous pour opter pour Homebrew et WinGet. Pour mettre à niveau Homebrew manuellement, exécutez `brew upgrade claude-code` ou `brew upgrade claude-code@latest`, selon le cask que vous avez installé. Pour WinGet, exécutez `winget upgrade Anthropic.ClaudeCode`. Pour les gestionnaires de paquets Linux, consultez les commandes de mise à niveau dans [Installer avec les gestionnaires de paquets Linux](#install-with-linux-package-managers).

  Pour que Claude Code exécute la commande de mise à niveau pour vous sur Homebrew ou WinGet, définissez [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/fr/env-vars) à `1`. Claude Code exécute ensuite la mise à niveau en arrière-plan lorsqu'une nouvelle version est disponible et affiche une invite de redémarrage en cas de succès. La mise à niveau cible uniquement le paquet Claude Code et n'affecte pas les autres logiciels que vous avez installés.

  Sur WinGet, la mise à niveau peut échouer pendant que Claude Code s'exécute car Windows verrouille l'exécutable. Dans ce cas, Claude Code affiche la commande manuelle à la place. apt, dnf et apk continuent à nécessiter une mise à niveau manuelle car ces commandes ont besoin de privilèges élevés.

  **Problème connu :** Claude Code peut vous notifier des mises à jour avant que la nouvelle version soit disponible dans ces gestionnaires de paquets. Si une mise à niveau échoue, attendez et réessayez plus tard.

  Homebrew conserve les anciennes versions sur le disque après les mises à niveau. Exécutez `brew cleanup` périodiquement pour récupérer de l'espace disque.
</Note>

<h3 id="configure-release-channel">
  Configurer le canal de version
</h3>

Contrôlez le canal de version que Claude Code suit pour les mises à jour automatiques et `claude update` avec le paramètre `autoUpdatesChannel` :

* `"latest"`, la valeur par défaut : recevez les nouvelles fonctionnalités dès qu'elles sont publiées
* `"stable"` : utilisez une version qui a généralement environ une semaine, en ignorant les versions avec des régressions majeures

Configurez ceci via `/config` → **Canal de mise à jour automatique**, ou ajoutez-le à votre [fichier settings.json](/docs/fr/settings) :

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Pour les déploiements d'entreprise, vous pouvez appliquer un canal de version cohérent dans votre organisation à l'aide des [paramètres gérés](/docs/fr/permissions#managed-settings).

Les installations Homebrew choisissent un canal par nom de cask au lieu de ce paramètre : `claude-code` suit stable et `claude-code@latest` suit latest.

<h3 id="pin-a-minimum-version">
  Épingler une version minimale
</h3>

Le paramètre `minimumVersion` établit un plancher. Les mises à jour automatiques en arrière-plan et `claude update` refusent d'installer toute version inférieure à cette valeur, donc passer au canal `"stable"` ne vous rétrograde pas si vous êtes déjà sur une version `"latest"` plus récente.

Passer de `"latest"` à `"stable"` via `/config` vous invite à rester sur la version actuelle ou à autoriser la rétrogradation. Choisir de rester définit `minimumVersion` à cette version. Revenir à `"latest"` l'efface.

Ajoutez-le à votre [fichier settings.json](/docs/fr/settings) pour épingler un plancher explicitement :

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Dans les [paramètres gérés](/docs/fr/permissions#managed-settings), cela applique un minimum à l'échelle de l'organisation que les paramètres utilisateur et projet ne peuvent pas remplacer.

Le pin `minimumVersion` ne contraint que les mises à jour. Pour faire refuser à Claude Code de démarrer en dehors d'une plage de versions, utilisez plutôt les paramètres gérés `requiredMinimumVersion` et `requiredMaximumVersion`. Les mises à jour respectent également le plafond `requiredMaximumVersion`. Consultez [paramètres disponibles](/docs/fr/settings#available-settings).

<h3 id="disable-auto-updates">
  Désactiver les mises à jour automatiques
</h3>

Définissez `DISABLE_AUTOUPDATER` à `"1"` dans la clé `env` de votre fichier [`settings.json`](/docs/fr/settings#available-settings) :

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` arrête uniquement la vérification en arrière-plan ; `claude update` et `claude install` fonctionnent toujours. Pour bloquer tous les chemins de mise à jour, y compris les mises à jour manuelles, définissez [`DISABLE_UPDATES`](/docs/fr/env-vars) à la place. Utilisez ceci lorsque vous distribuez Claude Code via vos propres canaux et que vous avez besoin que les utilisateurs restent sur la version que vous fournissez.

<h3 id="update-manually">
  Mettre à jour manuellement
</h3>

Pour appliquer une mise à jour immédiatement sans attendre la prochaine vérification en arrière-plan, exécutez :

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  Options d'installation avancées
</h2>

Ces options sont destinées à l'épinglage de version, aux gestionnaires de paquets Linux, à npm et à la vérification de l'intégrité des binaires.

<h3 id="install-a-specific-version">
  Installer une version spécifique
</h3>

L'installateur natif accepte soit un numéro de version spécifique, soit un canal de version (`latest` ou `stable`). Le canal que vous choisissez au moment de l'installation devient votre valeur par défaut pour les mises à jour automatiques. Consultez [configurer le canal de version](#configure-release-channel) pour plus d'informations.

Pour installer la dernière version (par défaut) :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

Pour installer la version stable :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

Pour installer un numéro de version spécifique :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Installer avec les gestionnaires de paquets Linux
</h3>

Claude Code publie des dépôts apt, dnf et apk signés. Chaque dépôt offre deux canaux : `stable` fournit une version généralement une semaine ancienne, en ignorant les versions avec des régressions majeures, et `latest` fournit chaque version dès qu'elle est publiée. Les commandes ci-dessous configurent le canal `stable`, qui convient à la plupart des utilisateurs ; chaque onglet affiche également l'URL du dépôt `latest`. Les installations du gestionnaire de paquets ne se mettent pas à jour automatiquement via Claude Code ; les mises à jour arrivent via votre flux de mise à niveau système normal.

Tous les dépôts sont signés avec la [clé de signature de version Claude Code](#binary-integrity-and-code-signing). Avant de faire confiance à la clé, vérifiez-la comme décrit dans chaque onglet.

<Tabs>
  <Tab title="apt">
    Pour Debian et Ubuntu. Les commandes d'installation ci-dessous téléchargent la clé de signature avec `curl`, que les installations fraîches de Debian et Ubuntu peuvent ne pas inclure. Si le téléchargement échoue avec `sudo: curl: command not found`, installez d'abord curl :

    ```bash theme={null}
    sudo apt install curl
    ```

    Les commandes suivantes configurent le canal `stable` :

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    Pour utiliser le canal `latest` à la place, le chemin d'URL et le nom de la suite changent tous les deux. Utilisez cette ligne `deb` :

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    Vérifiez l'empreinte digitale de la clé GPG avant de lui faire confiance : `gpg --show-keys /etc/apt/keyrings/claude-code.asc` devrait signaler `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.

    Pour mettre à jour ultérieurement, exécutez `sudo apt update && sudo apt upgrade claude-code`.
  </Tab>

  <Tab title="dnf">
    Pour Fedora et RHEL. Les commandes suivantes configurent le canal `stable` :

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    Pour utiliser le canal `latest` à la place, définissez `baseurl` sur le dépôt `latest` :

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf télécharge la clé lors de la première installation et vous demande de confirmer l'empreinte digitale. Vérifiez qu'elle correspond à `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` avant d'accepter.

    Pour mettre à jour ultérieurement, exécutez `sudo dnf upgrade claude-code`.
  </Tab>

  <Tab title="apk">
    Pour Alpine Linux. Les commandes suivantes configurent le canal `stable` :

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    Pour basculer vers le canal `latest`, supprimez la ligne du dépôt `stable` et ajoutez le dépôt `latest` :

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    Vérifiez la clé téléchargée avec `sha256sum /etc/apk/keys/claude-code.rsa.pub`, qui devrait signaler `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`.

    Pour mettre à jour ultérieurement, exécutez `apk update && apk upgrade claude-code`.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  Installer avec npm
</h3>

Vous pouvez également installer Claude Code en tant que paquet npm global. À partir de la v2.1.198, le paquet npm nécessite [Node.js 22 ou ultérieur](https://nodejs.org/en/download). Sur une version antérieure de Node.js, npm affiche un avertissement `EBADENGINE` lors de l'installation plutôt que d'échouer ; l'installation se termine et `claude` s'exécute toujours, car le paquet télécharge un binaire natif qui n'utilise pas votre Node.js à l'exécution.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

Le paquet npm installe le même binaire natif que l'installateur autonome. npm récupère le binaire via une dépendance optionnelle par plateforme telle que `@anthropic-ai/claude-code-darwin-arm64`, et une étape postinstallation le lie en place. Le binaire `claude` installé n'invoque pas lui-même Node.

Les plateformes d'installation npm supportées sont `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` et `win32-arm64`. Votre gestionnaire de paquets doit autoriser les dépendances optionnelles. Consultez le [dépannage](/docs/fr/troubleshoot-install#native-binary-not-found-after-npm-install) si le binaire est manquant après l'installation.

Pour mettre à niveau une installation npm, exécutez `npm install -g @anthropic-ai/claude-code@latest`. Évitez `npm update -g`, qui respecte la plage semver de l'installation d'origine et peut ne pas vous amener à la version la plus récente.

<Warning>
  N'utilisez PAS `sudo npm install -g` car cela peut entraîner des problèmes de permissions et des risques de sécurité. Si vous rencontrez des erreurs de permissions, consultez le [dépannage des erreurs de permissions](/docs/fr/troubleshoot-install#permission-errors-during-installation).
</Warning>

<h3 id="binary-integrity-and-code-signing">
  Intégrité des binaires et signature du code
</h3>

Chaque version publie un `manifest.json` contenant les sommes de contrôle SHA256 pour chaque binaire de plateforme. Le manifeste est signé avec une clé GPG Anthropic, donc vérifier la signature sur le manifeste vérifie transitivement chaque binaire qu'il répertorie.

<h4 id="verify-the-manifest-signature">
  Vérifier la signature du manifeste
</h4>

Les étapes 1 à 3 nécessitent un shell POSIX avec `gpg` et `curl`. Sur Windows, exécutez-les dans Git Bash ou WSL. L'étape 4 inclut une option PowerShell.

<Steps>
  <Step title="Télécharger et importer la clé publique">
    La clé de signature de version est publiée à une URL fixe.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    Afficher l'empreinte digitale de la clé importée.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    Confirmez que la sortie inclut cette empreinte digitale :

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="Télécharger le manifeste et la signature">
    Définissez `VERSION` sur la version que vous souhaitez vérifier.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="Vérifier la signature">
    Vérifiez la signature détachée par rapport au manifeste.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    Un résultat valide signale `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`.

    `gpg` imprime également `WARNING: This key is not certified with a trusted signature!` pour toute clé nouvellement importée. C'est attendu. La ligne `Good signature` confirme que la vérification cryptographique a réussi. La comparaison d'empreinte digitale à l'étape 1 confirme que la clé elle-même est authentique.
  </Step>

  <Step title="Vérifier le binaire par rapport au manifeste">
    Comparez la somme de contrôle SHA256 du binaire avec la valeur répertoriée sous `platforms.<platform>.checksum` dans `manifest.json`. Les commandes ci-dessous supposent un binaire `claude` dans le répertoire courant. Pour vérifier un binaire natif installé à la place, exécutez la commande par rapport à `~/.local/share/claude/versions/VERSION`, en remplaçant VERSION par la version que vous avez définie à l'étape 2.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  Les signatures de manifeste sont disponibles pour les versions à partir de `2.1.89`. Les versions antérieures publient les sommes de contrôle dans `manifest.json` sans signature détachée.
</Note>

<h4 id="platform-code-signatures">
  Signatures de code de plateforme
</h4>

En plus du manifeste signé, les binaires individuels portent des signatures de code natives de plateforme où supportées.

* **macOS** : signé par « Anthropic PBC » et notarié par Apple. Vérifiez avec `codesign --verify --verbose ./claude`.
* **Windows** : signé par « Anthropic, PBC ». Vérifiez avec `Get-AuthenticodeSignature .\claude.exe`.
* **Linux** : les binaires ne sont pas individuellement signés en code. Si vous téléchargez directement depuis le bucket `claude-code-releases` ou utilisez l'installateur natif, vérifiez l'intégrité avec la signature de manifeste ci-dessus. Si vous installez avec [apt, dnf ou apk](#install-with-linux-package-managers), votre gestionnaire de paquets vérifie automatiquement les signatures en utilisant la clé de signature du dépôt.

<h2 id="uninstall-claude-code">
  Désinstaller Claude Code
</h2>

Pour supprimer Claude Code, suivez les instructions correspondant à votre méthode d'installation. Si `claude` s'exécute toujours après cela, vous avez probablement une deuxième installation ou un alias shell résiduel d'un ancien installateur. Consultez [Vérifier les installations en conflit](/docs/fr/troubleshoot-install#check-for-conflicting-installations) pour le trouver et le supprimer.

<h3 id="native-installation">
  Installation native
</h3>

Supprimez le binaire Claude Code et les fichiers de version :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Installation Homebrew
</h3>

Supprimez le cask Homebrew que vous avez installé. Si vous avez installé le cask stable :

```bash theme={null}
brew uninstall --cask claude-code
```

Si vous avez installé le cask latest :

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  Installation WinGet
</h3>

Supprimez le paquet WinGet :

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

Supprimez le paquet et la configuration du référentiel :

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

Supprimez le paquet npm global :

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  Supprimer les fichiers de configuration
</h3>

<Warning>
  La suppression des fichiers de configuration supprimera tous vos paramètres, outils autorisés, configurations de serveur MCP et historique de session.
</Warning>

L'extension VS Code, le plugin JetBrains et l'application de bureau écrivent également dans `~/.claude/`. Si l'un d'eux est toujours installé, le répertoire est recréé la prochaine fois qu'il s'exécute. Pour supprimer Claude Code complètement, désinstallez l'[extension VS Code](/docs/fr/vs-code#uninstall-the-extension), le plugin JetBrains et l'application de bureau avant de supprimer ces fichiers.

Pour supprimer les paramètres et données en cache de Claude Code :

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # Supprimer les paramètres utilisateur et l'état
    rm -rf ~/.claude
    rm ~/.claude.json

    # Supprimer les paramètres spécifiques au projet (exécutez depuis votre répertoire de projet)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # Supprimer les paramètres utilisateur et l'état
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # Supprimer les paramètres spécifiques au projet (exécutez depuis votre répertoire de projet)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
