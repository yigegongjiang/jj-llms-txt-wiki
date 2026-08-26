> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dépanner l'installation et la connexion

> Corrigez les erreurs de commande introuvable, PATH, permission, réseau et authentification lors de l'installation ou de la connexion à Claude Code.

Si l'installation échoue ou que vous ne pouvez pas vous connecter, trouvez votre erreur ci-dessous. Pour les problèmes d'exécution après que Claude Code fonctionne, consultez [Dépannage](/docs/fr/troubleshooting). Pour les problèmes de configuration tels que les paramètres qui ne s'appliquent pas ou les hooks qui ne se déclenchent pas, consultez [Déboguer votre configuration](/docs/fr/debug-your-config).

<h2 id="find-your-error">
  Trouvez votre erreur
</h2>

Faites correspondre le message d'erreur ou le symptôme que vous voyez à une solution :

| Ce que vous voyez                                                                                                | Solution                                                                                                                                      |
| :--------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` ou `'claude' is not recognized`                                                      | [Corrigez votre PATH](#command-not-found-claude-after-installation)                                                                           |
| `syntax error near unexpected token '<'`                                                                         | [Le script d'installation retourne du HTML](#install-script-returns-html-instead-of-a-shell-script)                                           |
| `curl: (22) The requested URL returned error: 403`                                                               | [Le script d'installation a retourné 403](#install-script-returns-html-instead-of-a-shell-script)                                             |
| `curl: (23)` ou `curl: (56) Failure writing output to destination`                                               | [Vérifiez la connectivité ou utilisez un programme d'installation alternatif](#curl-56-failure-writing-output-to-destination)                 |
| `Killed` pendant l'installation sur Linux, ou `Installation was killed before it could finish (exit code 137)`   | [Libérez de la mémoire ou ajoutez de l'espace d'échange](#install-killed-on-low-memory-linux-servers)                                         |
| `TLS connect error` ou `SSL/TLS secure channel`                                                                  | [Mettez à jour les certificats CA](#tls-or-ssl-connection-errors)                                                                             |
| `Failed to fetch version` ou impossible d'atteindre le serveur de téléchargement                                 | [Vérifiez les paramètres réseau et proxy](#check-network-connectivity)                                                                        |
| `irm is not recognized` ou `&& is not valid`                                                                     | [Utilisez la bonne commande pour votre shell](#wrong-install-command-on-windows)                                                              |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                               | [Mettez à jour Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                              |
| `'bash' is not recognized as the name of a cmdlet`                                                               | [Utilisez la commande du programme d'installation Windows](#wrong-install-command-on-windows)                                                 |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                                 | [Utilisez la commande du programme d'installation Windows](#wrong-install-command-on-windows)                                                 |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                                | [Installez un shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                          |
| `Claude Code does not support 32-bit Windows`                                                                    | [Ouvrez Windows PowerShell, pas l'entrée x86](#claude-code-does-not-support-32-bit-windows)                                                   |
| `The process cannot access the file ... because it is being used by another process`                             | [Videz le dossier des téléchargements et réessayez](#the-process-cannot-access-the-file-during-windows-install)                               |
| `Error loading shared library`                                                                                   | [Mauvaise variante binaire pour votre système](#linux-musl-or-glibc-binary-mismatch)                                                          |
| `Illegal instruction`                                                                                            | [Incompatibilité d'architecture ou d'ensemble d'instructions CPU](#illegal-instruction)                                                       |
| `cannot execute binary file: Exec format error` dans WSL                                                         | [Régression binaire native WSL1](#exec-format-error-on-wsl1)                                                                                  |
| Le programme d'installation PowerShell se termine mais `claude` n'est pas trouvé ou affiche une ancienne version | [Ajoutez le répertoire d'installation à votre PATH](#verify-your-path), puis ouvrez un nouveau terminal                                       |
| `dyld: cannot load`, `dyld: Symbol not found`, ou `Abort trap` sur macOS                                         | [Incompatibilité binaire](#dyld-cannot-load-on-macos)                                                                                         |
| `Invoke-Expression: Missing argument in parameter list`                                                          | [Le script d'installation retourne du HTML](#install-script-returns-html-instead-of-a-shell-script)                                           |
| `App unavailable in region`                                                                                      | Claude Code n'est pas disponible dans votre pays. Consultez [les pays pris en charge](https://www.anthropic.com/supported-countries).         |
| `unable to get local issuer certificate`                                                                         | [Configurez les certificats CA d'entreprise](#tls-or-ssl-connection-errors)                                                                   |
| `OAuth error` ou `403 Forbidden`                                                                                 | [Corrigez l'authentification](#login-and-authentication)                                                                                      |
| `Could not load the default credentials` ou `Could not load credentials from any providers`                      | [Identifiants Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` ou `CredentialUnavailableError`                                   | [Identifiants Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429`, ou autres erreurs 4xx et 5xx non listées ci-dessus                    | Consultez la [Référence des erreurs](/docs/fr/errors)                                                                                              |

Si votre problème n'est pas listé, travaillez à travers les vérifications de diagnostic ci-dessous pour affiner la cause.

<Tip>
  Si vous préférez ignorer complètement le terminal, l'[application Claude Code Desktop](/docs/fr/desktop-quickstart) vous permet d'installer et d'utiliser Claude Code via une interface graphique. Téléchargez-la pour [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) ou [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) et commencez à coder sans aucune configuration en ligne de commande. Sur Linux, installez l'application avec apt en suivant les [instructions d'installation Linux](/docs/fr/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Exécutez les vérifications de diagnostic
</h2>

<h3 id="check-network-connectivity">
  Vérifiez la connectivité réseau
</h3>

Le programme d'installation télécharge depuis `downloads.claude.ai`. Vérifiez que vous pouvez l'atteindre :

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

Dans PowerShell, exécutez `curl.exe -sI` à la place. PowerShell crée un alias `curl` vers `Invoke-WebRequest`, qui rejette les drapeaux `-sI`.

Une ligne `HTTP/2 200` signifie que vous avez atteint le serveur. Si vous ne voyez aucune sortie, `Could not resolve host`, ou un délai d'expiration de connexion, votre réseau bloque la connexion. Les causes courantes incluent :

* Les pare-feu d'entreprise ou les proxies bloquant `downloads.claude.ai`
* Les restrictions réseau régionales : essayez un VPN ou un réseau alternatif
* Les problèmes TLS/SSL : mettez à jour les certificats CA de votre système, ou vérifiez si `HTTPS_PROXY` est configuré

Si vous êtes derrière un proxy d'entreprise, définissez `HTTPS_PROXY` et `HTTP_PROXY` à l'adresse de votre proxy avant d'installer. Demandez à votre équipe informatique l'URL du proxy si vous ne la connaissez pas, ou vérifiez les paramètres proxy de votre navigateur.

Cet exemple définit les deux variables de proxy, puis exécute le programme d'installation via votre proxy :

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Vérifiez votre PATH
</h3>

Si l'installation a réussi mais que vous obtenez une erreur `command not found` ou `not recognized` lors de l'exécution de `claude`, le répertoire d'installation n'est pas dans votre PATH. Votre shell recherche les programmes dans les répertoires listés dans PATH, et le programme d'installation place `claude` à `~/.local/bin/claude` sur macOS/Linux ou `%USERPROFILE%\.local\bin\claude.exe` sur Windows.

<Note>
  L'[extension VS Code](/docs/fr/vs-code) ne place pas `claude` à cet emplacement. Elle regroupe une copie privée de la CLI à l'intérieur du répertoire d'extension pour son propre panneau de chat et ne l'ajoute pas à PATH. Si vous avez uniquement installé l'extension, `~/.local/bin/claude` n'existera pas. Exécutez l'[installation autonome](/docs/fr/setup) pour utiliser `claude` à partir d'un terminal, puis continuez ci-dessous.
</Note>

Vérifiez si le répertoire d'installation est dans votre PATH en listant vos entrées PATH et en filtrant pour `local/bin` :

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Si cela affiche `/Users/you/.local/bin` ou `/home/you/.local/bin`, le répertoire est dans votre PATH et vous pouvez passer à [Vérifiez les installations en conflit](#check-for-conflicting-installations). S'il n'y a pas de sortie, ajoutez-le à votre configuration shell.

    Pour Zsh, la valeur par défaut sur macOS :

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Pour Bash, la valeur par défaut sur la plupart des distributions Linux :

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Alternativement, fermez et rouvrez votre terminal.

    Pour les autres shells tels que fish ou Nushell, ajoutez `~/.local/bin` à votre PATH en utilisant la syntaxe de configuration propre à votre shell, puis redémarrez votre terminal.

    Vérifiez que la correction a fonctionné :

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    S'il n'y a pas de sortie, ajoutez le répertoire d'installation à votre PATH utilisateur :

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Redémarrez votre terminal pour que la modification prenne effet.

    Vérifiez que la correction a fonctionné :

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    S'il n'y a pas de sortie, ouvrez Paramètres système, allez à Variables d'environnement, et ajoutez `%USERPROFILE%\.local\bin` à votre variable PATH utilisateur. Redémarrez votre terminal.

    Vérifiez que la correction a fonctionné :

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Vérifiez les installations en conflit
</h3>

Plusieurs installations de Claude Code peuvent causer des incompatibilités de version ou un comportement inattendu. Vérifiez ce qui est installé :

<Tabs>
  <Tab title="macOS/Linux">
    Listez tous les binaires `claude` trouvés dans votre PATH :

    ```bash theme={null}
    which -a claude
    ```

    Si cela n'affiche rien, aucun `claude` n'est encore sur votre PATH. Retournez à [Vérifiez votre PATH](#verify-your-path).

    Vérifiez les trois emplacements d'où un binaire `claude` peut provenir. `~/.local/bin/claude` est le programme d'installation natif, `~/.claude/local/` est une installation npm locale héritée créée par les anciennes versions de Claude Code, et la liste npm globale affiche une installation `-g` :

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Une installation native affiche un lien symbolique dans `~/.local/share/claude/versions/`. Un script ou un lien symbolique que vous avez créé vous-même à ce chemin est un lanceur personnalisé, que la [mise à jour automatique laisse en place](/docs/fr/setup#auto-updates).

    Si l'une ou l'autre commande `ls` affiche `No such file or directory`, ce n'est pas une erreur. Cela signifie que rien n'est installé à cet emplacement, alors passez à la vérification suivante.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Listez tous les binaires `claude` trouvés dans votre PATH :

    ```powershell theme={null}
    where.exe claude
    ```

    Vérifiez si le programme d'installation natif a placé un binaire :

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Si vous trouvez plusieurs installations, conservez-en une seule. L'installation native à `~/.local/bin/claude` sur macOS/Linux ou `%USERPROFILE%\.local\bin\claude.exe` sur Windows est recommandée. Supprimez les extras :

Désinstallez une installation npm globale :

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Supprimez l'installation npm locale héritée :

```bash theme={null}
rm -rf ~/.claude/local
```

Sur Windows, utilisez PowerShell :

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Supprimez une installation Homebrew sur macOS. Si vous avez installé le cask `claude-code@latest`, remplacez ce nom :

```bash theme={null}
brew uninstall --cask claude-code
```

Supprimez une installation WinGet sur Windows :

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Vérifiez les permissions des répertoires
</h3>

Le programme d'installation a besoin d'accès en écriture à `~/.local/bin/` et `~/.claude/` sur macOS et Linux. Sur Windows, l'emplacement d'installation est sous `%USERPROFILE%`, qui est accessible en écriture par votre utilisateur par défaut, donc cette section s'applique rarement là.

Vérifiez si les répertoires sont accessibles en écriture :

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Si l'un des répertoires n'est pas accessible en écriture, créez le répertoire d'installation et définissez votre utilisateur comme propriétaire :

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Vérifiez que le binaire fonctionne
</h3>

Si `claude --version` affiche une version mais que `claude` plante ou se fige au démarrage, exécutez ces vérifications pour affiner la cause. Si `claude --version` dit commande introuvable, allez d'abord à [Vérifiez votre PATH](#verify-your-path) ; les commandes ci-dessous supposent que `claude` est sur votre PATH.

Confirmez que le binaire existe et est exécutable :

```bash theme={null}
ls -la "$(command -v claude)"
```

Sur Windows, utilisez PowerShell :

```powershell theme={null}
Get-Command claude | Select-Object Source
```

Sur Linux, vérifiez les bibliothèques partagées manquantes. Si `ldd` affiche des bibliothèques manquantes, vous devrez peut-être installer des paquets système. Sur Alpine Linux et autres distributions basées sur musl, consultez [Configuration Alpine Linux](/docs/fr/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Confirmez que le binaire peut s'exécuter :

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Problèmes d'installation courants
</h2>

Ce sont les problèmes d'installation les plus fréquemment rencontrés et leurs solutions.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Le script d'installation retourne du HTML au lieu d'un script shell
</h3>

Lors de l'exécution de la commande d'installation, vous pouvez voir l'une de ces erreurs :

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

Sur PowerShell, le même problème apparaît comme :

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

Selon la façon dont la demande a été routée, vous pouvez à la place voir un 403 sans corps HTML :

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Tout cela signifie que l'URL d'installation a retourné une page HTML ou un statut d'erreur au lieu du script d'installation. Si la page HTML dit « App unavailable in region », Claude Code n'est pas disponible dans votre pays. Consultez [les pays pris en charge](https://www.anthropic.com/supported-countries).

Un 403 nu sans corps a souvent la même cause, mais il peut aussi provenir d'un proxy d'entreprise ou d'un pare-feu bloquant le téléchargement. Si vous êtes dans un pays pris en charge et voyez toujours le 403, parcourez [Vérifiez la connectivité réseau](#check-network-connectivity) avant d'essayer les programmes d'installation alternatifs ci-dessous, car ceux-ci atteignent les mêmes hôtes.

Sinon, cela peut se produire en raison de problèmes réseau, de routage régional ou d'une interruption de service temporaire.

**Solutions :**

1. **Utilisez une méthode d'installation alternative** :

   Sur macOS, installez via Homebrew :

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Sur Windows, installez via WinGet :

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Réessayez après quelques minutes** : le problème est souvent temporaire. Attendez et réessayez la commande d'origine.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` après l'installation
</h3>

L'installation s'est terminée mais `claude` ne fonctionne pas. L'erreur exacte varie selon la plateforme :

| Plateforme  | Message d'erreur                                                       |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Cela signifie que le répertoire d'installation n'est pas dans le chemin de recherche de votre shell. Consultez [Vérifiez votre PATH](#verify-your-path) pour la correction sur chaque plateforme.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

La commande `curl ... | bash` télécharge le script et le transmet à Bash pour exécution. Cette erreur, et l'erreur associée `curl: (23) Failure writing output to destination`, signifie que Bash n'a pas reçu le script complet. Le code de sortie 56 indique que le téléchargement lui-même a été interrompu, et le code de sortie 23 indique que curl n'a pas pu écrire ce qu'il a reçu dans le tuyau, généralement parce que Bash s'est fermé prématurément.

**Solutions :**

1. **Vérifiez la stabilité du réseau** : les binaires Claude Code sont hébergés sur `downloads.claude.ai`. Testez que vous pouvez l'atteindre :
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Une ligne `HTTP/2 200` signifie que vous avez atteint le serveur et l'échec d'origine était probablement intermittent ; réessayez la commande d'installation. Si vous voyez `Could not resolve host` ou un délai d'expiration de connexion, votre réseau bloque le téléchargement.

2. **Essayez une méthode d'installation alternative** :

   Sur macOS :

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Sur Windows :

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Cask Homebrew indisponible ou obsolète
</h3>

Homebrew signale `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` lorsque votre copie locale de l'index cask Homebrew est antérieure à la publication du cask. Actualisez l'index et réessayez :

```bash theme={null}
brew update
brew install --cask claude-code
```

Si Homebrew installe une version de Claude Code plus ancienne que celle que vous attendez, le même index obsolète en est généralement la cause. Le cask `claude-code` suit le canal stable et est généralement environ une semaine en retard sur la dernière version ; pour la version la plus récente, exécutez `brew install --cask claude-code@latest` à la place. Consultez [Configurer le canal de version](/docs/fr/setup#configure-release-channel) pour la différence entre les deux casks.

<h3 id="tls-or-ssl-connection-errors">
  Erreurs de connexion TLS ou SSL
</h3>

Les erreurs comme `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, ou le `Could not establish trust relationship for the SSL/TLS secure channel` de PowerShell indiquent des échecs de négociation TLS.

**Solutions :**

1. **Mettez à jour vos certificats CA système** :

   Sur Ubuntu/Debian :

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   Sur macOS, le curl système utilise le magasin de confiance Keychain ; la mise à jour de macOS lui-même met à jour les certificats racine.

2. **Sur Windows, activez TLS 1.2** dans PowerShell avant d'exécuter le programme d'installation :
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Vérifiez l'interférence du proxy ou du pare-feu** : les proxies d'entreprise qui effectuent une inspection TLS peuvent causer ces erreurs, y compris `unable to get local issuer certificate` et `SELF_SIGNED_CERT_IN_CHAIN`. Pour l'étape d'installation, pointez curl vers votre bundle CA d'entreprise avec `--cacert` :
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Pour Claude Code lui-même une fois installé, définissez `NODE_EXTRA_CA_CERTS` pour que les demandes API fassent confiance au même bundle :
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Demandez à votre équipe informatique le fichier de certificat si vous ne l'avez pas. Vous pouvez également essayer sur une connexion directe pour confirmer que le proxy est la cause.

4. **Sur Windows, contournez les vérifications de révocation de certificat** si vous voyez `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` ou `CRYPT_E_REVOCATION_OFFLINE (0x80092013)`. Ceux-ci signifient que curl a atteint le serveur mais votre réseau bloque la recherche de révocation de certificat, ce qui est courant derrière les pare-feu d'entreprise. Ajoutez le drapeau `--ssl-revoke-best-effort` de curl ne corrige pas ceci : le drapeau s'applique uniquement au téléchargement de `install.cmd` lui-même, et les téléchargements du script s'exécutent sans lui, donc l'installation échoue avec la même erreur. Utilisez une méthode d'installation qui tolère la recherche bloquée à la place. Ouvrez PowerShell et exécutez le programme d'installation PowerShell, qui télécharge via .NET et ne échoue pas lorsque le serveur de révocation est inaccessible :
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Vous pouvez également installer avec `winget install Anthropic.ClaudeCode`, qui évite curl entièrement.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Le programme d'installation n'a pas pu atteindre le serveur de téléchargement. Cela signifie généralement que `downloads.claude.ai` est bloqué sur votre réseau.

**Solutions :**

1. **Testez la connectivité directement** :
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Si derrière un proxy**, définissez `HTTPS_PROXY` pour que le programme d'installation puisse le router. Consultez [configuration du proxy](/docs/fr/network-config#proxy-configuration) pour plus de détails.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Si sur un réseau restreint**, essayez un réseau différent ou un VPN, ou utilisez une méthode d'installation alternative :

   Sur macOS :

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Sur Windows :

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Mauvaise commande d'installation sur Windows
</h3>

Si vous voyez `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'`, ou `'bash' is not recognized as the name of a cmdlet`, vous avez copié la commande d'installation pour un shell ou un système d'exploitation différent.

* **`irm` non reconnu** : vous êtes dans CMD, pas PowerShell. Vous avez deux options :

  Ouvrez PowerShell en recherchant « PowerShell » dans le menu Démarrer, puis exécutez la commande d'installation d'origine :

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Ou restez dans CMD et utilisez le programme d'installation CMD à la place :

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` non valide** : vous êtes dans PowerShell mais avez exécuté la commande du programme d'installation CMD. Utilisez le programme d'installation PowerShell :
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`** : vous avez exécuté le programme d'installation macOS/Linux `curl -fsSL ... | bash` dans Windows PowerShell, où `curl` est un alias pour `Invoke-WebRequest` et rejette les drapeaux `-fsSL`. Utilisez le programme d'installation PowerShell à la place :
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` non reconnu** : vous avez exécuté le programme d'installation macOS/Linux sur Windows. Utilisez le programme d'installation PowerShell à la place :
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` pendant l'installation Windows
</h3>

Si le programme d'installation PowerShell échoue avec `Failed to download binary: The process cannot access the file ... because it is being used by another process`, le programme d'installation n'a pas pu écrire dans `%USERPROFILE%\.claude\downloads`. Cela signifie généralement qu'une tentative d'installation précédente est toujours en cours d'exécution, ou que le logiciel antivirus analyse un binaire partiellement téléchargé dans ce dossier.

Fermez toutes les autres fenêtres PowerShell exécutant le programme d'installation et attendez que les analyses antivirus libèrent le fichier. Ensuite, supprimez le dossier des téléchargements et exécutez le programme d'installation à nouveau :

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Installation interrompue sur les serveurs Linux à faible mémoire
</h3>

Un message `Killed` pendant l'installation signifie généralement que le tueur OOM (out-of-memory) Linux a terminé l'étape `claude install` car le système a manqué de mémoire libre. Cela est courant sur les petits VPS et instances cloud. Le script d'installation signale la cause et se termine avec le code 137 :

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Avant v2.1.200, le script s'est terminé avec seulement la ligne `Killed` nue du shell et aucune explication.

L'installation nécessite environ 512 Mo de mémoire libre, et l'exécution de Claude Code en nécessite davantage. Consultez les [exigences système](/docs/fr/setup#system-requirements).

**Solutions :**

1. **Ajoutez de l'espace d'échange** si votre serveur a une RAM limitée. L'échange utilise l'espace disque comme mémoire de débordement, permettant à l'installation de se terminer même avec une RAM physique faible.

   Créez un fichier d'échange de 2 Go et activez-le :

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Puis réessayez l'installation :

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Fermez les autres processus** pour libérer de la mémoire avant d'installer.

3. **Utilisez une instance plus grande** si possible. Claude Code nécessite au moins 4 Go de RAM.

<h3 id="install-hangs-in-docker">
  L'installation se fige dans Docker
</h3>

Lors de l'installation de Claude Code dans un conteneur Docker, l'installation en tant que root dans `/` peut causer des blocages.

**Solutions :**

1. **Définissez un répertoire de travail** avant d'exécuter le programme d'installation. Lorsqu'il est exécuté depuis `/`, le programme d'installation analyse l'ensemble du système de fichiers, ce qui provoque une utilisation excessive de la mémoire. La définition de `WORKDIR` limite l'analyse à un petit répertoire :
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Augmentez les limites de mémoire Docker** si vous utilisez Docker Desktop :
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop remplace la commande `claude` sur Windows
</h3>

Si vous avez installé une version antérieure de Claude Desktop, elle peut enregistrer un `Claude.exe` dans le répertoire `WindowsApps` qui prend la priorité PATH sur Claude Code CLI. L'exécution de `claude` ouvre l'application Desktop au lieu de la CLI.

Mettez à jour Claude Desktop vers la dernière version pour corriger ce problème.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code sur Windows nécessite Git pour Windows (pour bash) ou PowerShell
</h3>

Git pour Windows est optionnel. Claude Code utilise l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) en l'absence de Git Bash, donc cette erreur signifie qu'aucun shell n'a été trouvé.

**Si PowerShell manque de votre PATH**, son emplacement par défaut est `C:\Windows\System32\WindowsPowerShell\v1.0\`. Ajoutez ce répertoire à votre `PATH`, ou installez [PowerShell 7](https://aka.ms/powershell), qui fournit `pwsh`.

**Pour installer Git pour Windows à la place**, téléchargez-le depuis [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Pendant la configuration, sélectionnez « Add to PATH ». Redémarrez votre terminal après l'installation. L'installation active l'outil Bash, utile lorsque vous travaillez avec des scripts et des outils basés sur Bash.

**Si Git est déjà installé** mais que Claude Code ne peut pas le trouver, définissez le chemin dans votre [fichier settings.json](/docs/fr/settings) :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Si votre Git est installé ailleurs, trouvez le chemin en exécutant `where.exe git` dans PowerShell et utilisez le chemin `bin\bash.exe` de ce répertoire.

**Si le chemin est correct et le fichier existe** mais que Claude Code signale toujours qu'il n'est pas trouvé, un logiciel de sécurité des points de terminaison tel que AppLocker, les stratégies de restriction logicielle de la stratégie de groupe ou les agents EDR peuvent interférer. Sur les versions antérieures à v2.1.116, Claude Code générait un processus enfant (`cmd.exe`) pour vérifier le chemin, que ces stratégies peuvent bloquer — un signal courant est que `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` fonctionne lorsque vous l'exécutez directement dans PowerShell mais échoue silencieusement lorsqu'il est lancé par `claude.exe`.

Claude Code v2.1.116 et versions ultérieures vérifient le système de fichiers directement, donc mettez à jour d'abord. Si l'erreur persiste sur une version actuelle, demandez à votre équipe informatique de mettre en liste blanche `claude.exe` et les processus qu'il génère, y compris `cmd.exe` et `bash.exe`, dans votre stratégie de protection des points de terminaison.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code ne supporte pas Windows 32 bits
</h3>

Windows inclut deux entrées PowerShell dans le menu Démarrer : `Windows PowerShell` et `Windows PowerShell (x86)`. L'entrée x86 s'exécute en tant que processus 32 bits et déclenche cette erreur même sur une machine 64 bits. Pour vérifier quel cas vous êtes, exécutez ceci dans la même fenêtre qui a produit l'erreur :

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Si cela affiche `True`, votre système d'exploitation est correct. Fermez la fenêtre, ouvrez `Windows PowerShell` sans le suffixe x86, et réexécutez la commande d'installation.

Si cela affiche `False`, vous êtes sur une édition 32 bits de Windows. Claude Code nécessite un système d'exploitation 64 bits. Consultez les [exigences système](/docs/fr/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Incompatibilité binaire musl ou glibc Linux
</h3>

Si vous voyez des erreurs concernant les bibliothèques partagées manquantes comme `libstdc++.so.6` ou `libgcc_s.so.1` après l'installation, le programme d'installation a peut-être téléchargé la mauvaise variante binaire pour votre système.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Cela peut se produire sur les systèmes basés sur glibc qui ont des paquets de compilation croisée musl installés, ce qui amène le programme d'installation à mal détecter le système comme musl.

**Solutions :**

1. **Vérifiez quelle libc votre système utilise** :
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   La sortie mentionnant `GNU libc` ou `GLIBC` signifie glibc. La sortie mentionnant `musl` signifie musl.

2. **Si vous êtes sur glibc mais avez obtenu le binaire musl**, supprimez l'installation et réinstallez. Vous pouvez également télécharger manuellement le binaire correct en utilisant le manifeste à `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Déposez un [problème GitHub](https://github.com/anthropics/claude-code/issues) avec la sortie de `ldd --version` et `ls /lib/libc.musl*`.

3. **Si vous êtes réellement sur musl**, comme Alpine Linux, installez les paquets requis :
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Si l'exécution de `claude` ou du programme d'installation affiche `Illegal instruction`, le binaire natif utilise des instructions CPU que votre processeur ne supporte pas. Il y a deux causes distinctes.

**Incompatibilité d'architecture.** Le programme d'installation a téléchargé le mauvais binaire, par exemple x86 sur un serveur ARM. Vérifiez avec `uname -m` sur macOS ou Linux, ou `$env:PROCESSOR_ARCHITECTURE` dans PowerShell. Si le résultat ne correspond pas au binaire que vous avez reçu, [déposez un problème GitHub](https://github.com/anthropics/claude-code/issues) avec la sortie.

**Ensemble d'instructions manquant sur les anciens processeurs.** Si votre architecture est correcte mais que vous voyez toujours `Illegal instruction`, votre processeur manque probablement d'AVX ou d'une autre instruction que le binaire nécessite. Cela affecte environ les processeurs Intel et AMD antérieurs à 2013, et les machines virtuelles où l'hyperviseur ne transmet pas AVX à l'invité.

Sur un VPS ou une VM, exécutez `grep -m1 -ow avx /proc/cpuinfo` ; un résultat vide signifie qu'AVX n'est pas disponible pour l'invité.

Il n'y a pas de solution de contournement binaire native ; suivez [le problème #50384](https://github.com/anthropics/claude-code/issues/50384) pour le statut, et incluez votre modèle de processeur depuis `grep -m1 "model name" /proc/cpuinfo` sur Linux ou `sysctl -n machdep.cpu.brand_string` sur macOS lors du signalement.

Les méthodes d'installation alternatives téléchargent le même binaire natif et ne résoudront aucune des deux causes.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` sur macOS
</h3>

Si vous voyez `dyld: cannot load`, `dyld: Symbol not found`, ou `Abort trap: 6` pendant l'installation, le binaire est incompatible avec votre version ou matériel macOS.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Une erreur `Symbol not found` qui référence `libicucore` indique également que votre version macOS est plus ancienne que celle que le binaire supporte :

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Solutions :**

1. **Vérifiez votre version macOS** : Claude Code nécessite macOS 13.0 ou ultérieur. Ouvrez le menu Apple et sélectionnez À propos de ce Mac pour vérifier votre version.

2. **Mettez à jour macOS** si vous êtes sur une version plus ancienne. Le binaire utilise des commandes de chargement et des bibliothèques système que les anciennes versions de macOS ne supportent pas. Les méthodes d'installation alternatives comme Homebrew téléchargent le même binaire et ne résoudront pas cette erreur.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` sur WSL1
</h3>

Si l'exécution de `claude` dans WSL affiche `cannot execute binary file: Exec format error`, vous êtes sur WSL1 et vous rencontrez une régression binaire native connue suivie dans [le problème #38788](https://github.com/anthropics/claude-code/issues/38788). Les en-têtes de programme du binaire ont changé d'une manière que le chargeur WSL1 ne peut pas gérer.

La correction la plus propre est de convertir votre distribution en WSL2 depuis PowerShell :

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Si vous devez rester sur WSL1, invoquez le binaire via l'éditeur de liens dynamique. Ajoutez cette fonction à `~/.bashrc` dans WSL, en remplaçant le chemin si votre répertoire personnel diffère :

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Puis exécutez `source ~/.bashrc` et réessayez `claude`.

<h3 id="npm-install-errors-in-wsl">
  Erreurs d'installation npm dans WSL
</h3>

Ces problèmes s'appliquent si vous avez installé Claude Code avec `npm install -g` dans WSL. Si vous avez utilisé le [programme d'installation natif](/docs/fr/setup), ignorez cette section.

**Problèmes de détection d'OS ou de plateforme.** Si npm signale une incompatibilité de plateforme pendant l'installation, WSL utilise probablement le `npm` Windows. Exécutez d'abord `npm config set os linux`, puis installez avec `npm install -g @anthropic-ai/claude-code --force`. N'utilisez pas `sudo`.

**`exec: node: not found` lors de l'exécution de `claude`.** Votre environnement WSL utilise probablement l'installation Windows de Node.js. Confirmez avec `which npm` et `which node` : les chemins commençant par `/mnt/c/` sont des binaires Windows, tandis que les chemins Linux commencent par `/usr/`. Pour corriger cela, installez Node via le gestionnaire de paquets de votre distribution Linux ou via [`nvm`](https://github.com/nvm-sh/nvm).

**Conflits de version nvm.** Si vous avez nvm installé à la fois dans WSL et Windows, basculer les versions de Node dans WSL peut casser car WSL importe le PATH Windows par défaut et le nvm Windows prend la priorité. La cause la plus courante est que nvm n'est pas chargé dans votre shell. Ajoutez le chargeur nvm à `~/.bashrc` ou `~/.zshrc` :

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Ou chargez-le dans votre session actuelle :

```bash theme={null}
source ~/.nvm/nvm.sh
```

Si nvm est chargé mais que les chemins Windows prennent toujours la priorité, prépendez explicitement votre chemin Node Linux :

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Évitez de désactiver l'importation du PATH Windows via `appendWindowsPath = false` car cela casse la capacité à appeler les exécutables Windows depuis WSL. De même, évitez de désinstaller Node.js de Windows si vous l'utilisez pour le développement Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Erreurs de permission pendant l'installation
</h3>

Si le programme d'installation natif échoue avec des erreurs de permission, le répertoire cible peut ne pas être accessible en écriture. Consultez [Vérifiez les permissions des répertoires](#check-directory-permissions).

Si vous avez précédemment installé avec npm et rencontrez des erreurs de permission spécifiques à npm, passez au programme d'installation natif :

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Binaire natif non trouvé après l'installation npm
</h3>

Le paquet npm `@anthropic-ai/claude-code` récupère le binaire natif via une dépendance optionnelle par plateforme comme `@anthropic-ai/claude-code-darwin-arm64`. Si l'exécution de `claude` après l'installation affiche `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, vérifiez les causes suivantes :

* **Les dépendances optionnelles sont désactivées.** Supprimez `--omit=optional` de votre commande d'installation npm, `--no-optional` de pnpm, ou `--ignore-optional` de yarn, et vérifiez que `.npmrc` ne définit pas `optional=false`. Puis réinstallez. Le binaire natif est livré uniquement en tant que dépendance optionnelle, donc il n'y a pas de secours JavaScript s'il est ignoré.
* **Plateforme non supportée.** Les binaires précompilés sont publiés pour `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, et `win32-arm64`. Claude Code ne livre pas de binaire pour d'autres plateformes ; consultez les [exigences système](/docs/fr/setup#system-requirements). Sur FreeBSD, le programme d'installation signale la plateforme comme non supportée. Avant v2.1.205, il traitait FreeBSD comme Linux et téléchargeait un binaire qui ne pouvait pas s'exécuter.
* **Le miroir npm d'entreprise manque les paquets de plateforme.** Assurez-vous que votre registre reflète les huit paquets `@anthropic-ai/claude-code-*` de plateforme en plus du paquet méta.

L'installation avec `--ignore-scripts` ne déclenche pas cette erreur. L'étape postinstall qui lie le binaire en place est ignorée, donc Claude Code revient à un wrapper qui localise et génère le binaire de plateforme à chaque lancement. Cela fonctionne mais démarre plus lentement ; réinstallez avec les scripts activés pour l'exécution directe.

<h2 id="login-and-authentication">
  Connexion et authentification
</h2>

Ces sections traitent des échecs de connexion, des erreurs OAuth et des problèmes de jeton.

<h3 id="reset-your-login">
  Réinitialisez votre connexion
</h3>

Lorsque la connexion échoue et que la cause n'est pas évidente, une ré-authentification propre résout la plupart des cas :

1. Exécutez `/logout` pour vous déconnecter complètement
2. Fermez Claude Code
3. Redémarrez avec `claude` et complétez le processus d'authentification à nouveau

Si le navigateur ne s'ouvre pas automatiquement pendant la connexion, appuyez sur `c` pour copier l'URL OAuth dans votre presse-papiers, puis collez-la dans un navigateur manuellement. Cela fonctionne également lorsque l'URL s'enroule sur plusieurs lignes dans un terminal étroit ou SSH et ne peut pas être cliquée directement.

<h3 id="oauth-error-invalid-code">
  Erreur OAuth : Code invalide
</h3>

Si vous voyez `OAuth error: Invalid code. Please make sure the full code was copied`, le code de connexion a expiré ou a été tronqué lors du copier-coller.

**Solutions :**

* Appuyez sur Entrée pour réessayer et complétez la connexion rapidement après l'ouverture du navigateur
* Tapez `c` pour copier l'URL complète si le navigateur ne s'ouvre pas automatiquement
* Si vous utilisez une session distante/SSH, le navigateur peut s'ouvrir sur la mauvaise machine. Copiez l'URL affichée dans le terminal et ouvrez-la dans votre navigateur local à la place.

<h3 id="403-forbidden-after-login">
  403 Forbidden après la connexion
</h3>

Si vous voyez `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` après la connexion :

* **Utilisateurs Claude Pro/Max** : vérifiez que votre abonnement est actif sur [claude.ai/settings](https://claude.ai/settings)
* **Utilisateurs Anthropic Console** : confirmez que votre compte a le rôle « Claude Code » ou « Developer ». Les administrateurs l'attribuent dans la console Anthropic sous Paramètres → Membres.
* **Derrière un proxy** : les proxies d'entreprise peuvent interférer avec les demandes API. Consultez [configuration réseau](/docs/fr/network-config) pour la configuration du proxy.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  Cette organisation a été désactivée avec un abonnement actif
</h3>

Si vous voyez `API Error: 400 ... "This organization has been disabled"` malgré un abonnement Claude actif, une variable d'environnement `ANTHROPIC_API_KEY` remplace vos identifiants OAuth d'abonnement. Cela se produit couramment lorsqu'une ancienne clé API d'un employeur ou d'un projet précédent est toujours définie dans votre profil shell.

Lorsque `ANTHROPIC_API_KEY` est présente et que vous l'avez approuvée, Claude Code utilise cette clé au lieu des identifiants OAuth de votre abonnement. En mode non interactif avec le drapeau `-p`, la clé est toujours utilisée lorsqu'elle est présente. Consultez [précédence d'authentification](/docs/fr/authentication#authentication-precedence) pour l'ordre de résolution complet.

Pour utiliser votre abonnement à la place, défiez la variable d'environnement et supprimez-la de votre profil shell :

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Vérifiez `~/.zshrc`, `~/.bashrc`, ou `~/.profile` pour les lignes `export ANTHROPIC_API_KEY=...` et supprimez-les pour rendre le changement permanent. Sur Windows, vérifiez votre profil PowerShell à `$PROFILE` et vos variables d'environnement utilisateur pour `ANTHROPIC_API_KEY`. Exécutez `/status` dans Claude Code pour confirmer quelle méthode d'authentification est active.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  La connexion OAuth échoue dans WSL2, SSH ou conteneurs
</h3>

Lorsque Claude Code s'exécute dans WSL2, sur une machine distante via SSH ou à l'intérieur d'un conteneur, le navigateur s'ouvre généralement sur un hôte différent et sa redirection ne peut pas atteindre le serveur de rappel local de Claude Code. Après vous être connecté, le navigateur affiche un code de connexion au lieu de rediriger automatiquement. Collez ce code dans le terminal à l'invite `Paste code here if prompted` pour terminer la connexion.

Si le navigateur ne s'ouvre pas du tout depuis WSL2, définissez la variable d'environnement `BROWSER` sur le chemin de votre navigateur Windows :

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Sinon, appuyez sur `c` à l'invite de connexion interactive pour copier l'URL OAuth, ou copiez l'URL que `claude auth login` affiche, et ouvrez-la dans un navigateur sur votre machine locale.

Si coller le code dans l'invite interactive ne fait rien, le raccourci de collage de votre terminal n'atteint probablement pas le champ de saisie. Essayez le raccourci de collage alternatif de votre terminal, souvent clic droit ou Maj+Insérer dans Windows Terminal, ou utilisez `claude auth login` à la place, qui lit le code collé à partir de l'entrée standard :

```bash theme={null}
claude auth login
```

Ce secours s'applique également sur Windows natif ou tout terminal où coller le code dans l'invite interactive échoue.

<h3 id="not-logged-in-or-token-expired">
  Non connecté ou jeton expiré
</h3>

Si Claude Code vous demande de vous connecter à nouveau après une session, votre jeton OAuth a peut-être expiré.

Exécutez `/login` pour vous ré-authentifier. Si cela se produit fréquemment, vérifiez que votre horloge système est exacte, car la validation du jeton dépend des horodatages corrects.

Sur macOS, la connexion peut également échouer lorsque le Keychain est verrouillé ou que son mot de passe est désynchronisé avec votre mot de passe de compte, ce qui empêche Claude Code de sauvegarder les identifiants. Exécutez `claude doctor` pour vérifier l'accès au Keychain. Pour déverrouiller le Keychain manuellement, exécutez `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Si le déverrouillage n'aide pas, ouvrez Keychain Access, sélectionnez le Keychain `login`, et choisissez Édition > Changer le mot de passe du Keychain « login » pour le resynchroniser avec votre mot de passe de compte.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Les identifiants Bedrock, Agent Platform ou Foundry ne se chargent pas
</h3>

Si vous avez configuré Claude Code pour utiliser un fournisseur cloud et voyez `Could not load credentials from any providers` sur Amazon Bedrock, `Could not load the default credentials` sur Google Cloud's Agent Platform, ou `ChainedTokenCredential authentication failed` sur Microsoft Foundry, votre CLI du fournisseur cloud n'est probablement pas authentifiée dans le shell actuel.

Pour Amazon Bedrock, confirmez que vos identifiants AWS sont valides :

```bash theme={null}
aws sts get-caller-identity
```

Pour Google Cloud's Agent Platform, confirmez que `ANTHROPIC_VERTEX_PROJECT_ID` et `CLOUD_ML_REGION` sont définis dans votre shell, puis définissez les identifiants par défaut de l'application :

```bash theme={null}
gcloud auth application-default login
```

Pour Microsoft Foundry, confirmez que `ANTHROPIC_FOUNDRY_API_KEY` est défini, ou connectez-vous avec l'interface de ligne de commande Azure pour que la chaîne d'identifiants par défaut puisse trouver votre compte :

```bash theme={null}
az login
```

Si les identifiants fonctionnent dans votre terminal mais pas dans l'extension VS Code ou JetBrains, le processus IDE n'a probablement pas hérité de votre environnement shell. Définissez les variables d'environnement du fournisseur dans les paramètres de l'IDE lui-même, ou lancez l'IDE depuis un terminal où elles sont déjà exportées.

Consultez [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry) pour la configuration complète du fournisseur.

<h2 id="still-stuck">
  Toujours bloqué
</h2>

Si aucune des solutions ci-dessus ne résout votre problème :

1. Vérifiez le [référentiel GitHub](https://github.com/anthropics/claude-code/issues) pour les problèmes connus, ou ouvrez-en un nouveau avec votre système d'exploitation, la commande d'installation que vous avez exécutée, et la sortie d'erreur complète
2. Si `claude --version` fonctionne mais quelque chose d'autre ne va pas, exécutez `claude doctor` pour un rapport de diagnostic automatisé
3. Si vous pouvez démarrer une session, utilisez `/feedback` dans Claude Code pour signaler le problème
