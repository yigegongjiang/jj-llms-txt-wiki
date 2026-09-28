> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dépanner le SDK Agent

> Corrigez les erreurs du SDK Agent lorsque la CLI Claude Code ne démarre pas, le processus CLI se termine ou un résultat réussi arrive sans sortie structurée.

Cette page couvre les erreurs du SDK Agent au démarrage de la CLI, à la sortie du processus CLI et aux sorties structurées. Les entrées de cette page sont indexées selon l'erreur que vous voyez. Chacune indique la cause et ce qu'il faut faire.

Les symptômes liés à une fonctionnalité, comme un hook qui ne se déclenche pas ou une skill qui n'est pas utilisée, ont une section de dépannage sur la page de cette fonctionnalité. Le tableau nomme la section ou la page qui couvre chaque symptôme :

| Symptôme                                                                                                                                                                                                                                                                                                                                                      | Aller à                                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Skills non trouvées, une skill non utilisée, erreur `Invalid skill name`                                                                                                                                                                                                                                                                                      | [Dépannage des skills](/docs/fr/agent-sdk/skills#troubleshooting)                                                                                   |
| Le serveur MCP affiche le statut `failed`, les outils ne sont pas appelés, les délais d'expiration de la connexion, la sortie de l'outil qui dépasse le nombre maximum de tokens autorisés                                                                                                                                                                    | [Dépannage MCP](/docs/fr/agent-sdk/mcp#troubleshooting)                                                                                             |
| Plugin ne se charge pas, les skills du plugin n'apparaissent pas                                                                                                                                                                                                                                                                                              | [Dépannage des plugins](/docs/fr/agent-sdk/plugins#troubleshooting)                                                                                 |
| Claude ne délègue pas aux sous-agents, les agents basés sur le système de fichiers ne se chargent pas                                                                                                                                                                                                                                                         | [Dépannage des sous-agents](/docs/fr/agent-sdk/subagents#troubleshooting)                                                                           |
| Les options de checkpointing ne sont pas reconnues, les messages utilisateur sans UUID, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                              | [Dépannage du checkpointing de fichiers](/docs/fr/agent-sdk/file-checkpointing#troubleshooting)                                                     |
| Hook ne se déclenche pas, le matcher ne filtre pas comme prévu, délai d'expiration du hook, outil bloqué de manière inattendue, entrée modifiée non appliquée, hooks de session non disponibles en Python, invites de permission de sous-agent se multipliant, boucles de hook récursives avec sous-agents, `systemMessage` n'apparaissant pas dans la sortie | [Corriger les problèmes courants](/docs/fr/agent-sdk/hooks#fix-common-issues) sur la page des hooks                                                 |
| Un agent qui fonctionne sur votre machine échoue dans un service déployé ou un conteneur                                                                                                                                                                                                                                                                      | [Dépanner les défaillances de déploiement](/docs/fr/agent-sdk/hosting#troubleshoot-deployment-failures)                                             |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                                                            | [Référence des erreurs](/docs/fr/errors#find-your-error)                                                                                            |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` est `None`                                                                                                                                                                                    | [Démarrage de la CLI](#cli-startup), [Sortie du processus CLI](#cli-process-exit) et [Sorties structurées](#structured-outputs) sur cette page |

<h2 id="cli-startup">
  Démarrage du CLI
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError : Claude Code introuvable
</h3>

Le SDK Python lance le CLI Claude Code en tant que sous-processus. Quand il ne peut pas trouver un exécutable `claude`, la connexion échoue avec une `CLINotFoundError` :

```
Claude Code not found at: /your/configured/path
```

Le message inclut le chemin configuré quand vous définissez `ClaudeAgentOptions(cli_path=...)` et qu'il pointe vers un fichier manquant. Sans `cli_path`, le SDK recherche dans votre `PATH` et les emplacements d'installation courants, et le message inclut les instructions d'installation pour votre plateforme.

Pour corriger cela :

* Installez Claude Code s'il n'est pas installé. Consultez [Installer Claude Code](/docs/fr/setup#install-claude-code) pour la commande sur votre plateforme.
* Si vous avez défini `cli_path`, confirmez que le fichier existe et qu'il s'agit de l'exécutable `claude`.
* Si vous comptez sur la résolution `PATH`, confirmez que `claude --version` fonctionne dans le même environnement que celui dans lequel votre application s'exécute. Les processus que vous lancez en dehors de votre shell, par exemple à partir d'un IDE ou d'un gestionnaire de services, s'exécutent souvent avec un `PATH` différent.

Le SDK TypeScript recherche le CLI dans son paquet de plateforme fourni et le chemin que vous avez défini dans `pathToClaudeCodeExecutable`. Faites correspondre le message que vous voyez :

* `Native CLI binary for <platform>-<arch> not found` : le paquet de plateforme fourni est manquant, le plus souvent parce que l'installation a ignoré les dépendances optionnelles. Réinstallez `@anthropic-ai/claude-agent-sdk` sans ignorer les dépendances optionnelles, ou pointez `pathToClaudeCodeExecutable` vers une [installation native](/docs/fr/setup#install-claude-code). Dans un exécutable monofichier construit avec `bun build --compile`, le même message a une cause et une correction différentes. Consultez [Compiler en un seul exécutable](/docs/fr/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` ou `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?` : le fichier au chemin résolu est manquant, ou le processus ne peut pas y accéder. Confirmez que le fichier existe à ce chemin et que le processus peut y accéder.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError : Refus d'exécuter un script batch
</h3>

Sur Windows, la connexion échoue avec une `CLIConnectionError` quand le chemin du CLI que le SDK Python utilise est un script batch `.bat` ou `.cmd`, y compris le shim `claude.cmd` qu'une installation npm crée :

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

Le refus est un durcissement de sécurité délibéré, pas une installation cassée. Windows exécute les scripts batch en réécrivant le spawn en une invocation `cmd.exe /c`, et `cmd.exe` réanalyse toute la ligne de commande au moment de l'exécution, donc une valeur d'argument peut exécuter des commandes injectées.

La plupart des installations Windows ne rencontrent jamais cette erreur. La wheel Windows x64 de `claude-agent-sdk` inclut un `claude.exe`, et le SDK préfère le CLI fourni, puis tout `claude.exe` natif qu'il peut découvrir, avant de revenir à un shim batch. Vous voyez le refus dans deux cas :

* Vous avez défini `ClaudeAgentOptions(cli_path=...)` vers un fichier `.bat` ou `.cmd`, comme le shim `claude.cmd` de npm.
* Votre installation n'a pas de `claude.exe` fourni ou natif, par exemple une installation source sur ARM64 Windows où le seul `claude` sur votre `PATH` est le shim npm.

Pour corriger cela, donnez au SDK un exécutable natif au lieu d'un script batch :

* Si vous avez défini `ClaudeAgentOptions(cli_path=...)`, pointez-le vers un `claude.exe` ou supprimez l'option. Le SDK ignore la découverte tant que `cli_path` est défini, donc une installation native seule ne peut pas prendre effet.
* Installez Claude Code nativement dans PowerShell : `irm https://claude.ai/install.ps1 | iex`
* Sur Windows x64, installez la wheel `claude-agent-sdk`, qui inclut `claude.exe`.

Avant `claude-agent-sdk` 0.2.124, le SDK Python lançait les scripts batch via `cmd.exe` sans cette vérification.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError : Impossible de démarrer Claude Code
</h3>

Le SDK a trouvé un fichier au chemin résolu mais n'a pas pu le lancer. Python lève ces défaillances en tant que `CLIConnectionError`. TypeScript rejette l'itération de message avec une erreur ne portant aucune classe SDK. Le tableau ci-dessous mappe chaque message à ce qu'il vous dit. Faites correspondre le message que vous voyez :

| Message                                                           | SDK        | Ce qu'il vous dit                                                              |
| ----------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------ |
| `Failed to start Claude Code: <detail>`                           | Python     | Le reste du message est l'erreur propre du système d'exploitation              |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | Le script au chemin configuré ne peut pas s'exécuter                           |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | Le binaire ne peut pas s'exécuter, avec une suggestion libc ajoutée au message |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Toute autre défaillance de lancement                                           |

Dans les deux SDK, la cause habituelle est un chemin résolu qui pointe vers quelque chose qui ne peut pas s'exécuter, comme un fichier texte, un répertoire ou un fichier sans permission d'exécution. Lisez la suggestion libc du message binaire natif comme une cause possible.

Pour corriger cela dans l'un ou l'autre SDK :

* Confirmez que le chemin configuré pointe vers l'exécutable `claude` lui-même et que le fichier a la permission d'exécution.
* Si vous n'avez pas besoin d'un chemin personnalisé, supprimez `cli_path` en Python ou `pathToClaudeCodeExecutable` en TypeScript pour que le SDK trouve un CLI par lui-même, en préférant sa copie fournie.
* Quand le binaire défaillant est la copie fournie du SDK dans une image conteneur, réinstallez le SDK pendant la construction de l'image pour que le binaire fourni corresponde à la plateforme du conteneur, ou reconstruisez l'image pour l'architecture sur laquelle elle s'exécute. La cause habituelle est un binaire qui ne correspond pas à l'architecture ou à la libc du conteneur, ou un qui a perdu sa permission d'exécution dans la construction de l'image.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError : Non connecté
</h3>

Appeler une méthode `ClaudeSDKClient` en Python avant que le client se soit connecté, ou après qu'il se soit déconnecté, lève une `CLIConnectionError` avec ce message :

```
Not connected. Call connect() first.
```

Faites ce que le message dit. Appelez soit `await client.connect()` avant toute autre méthode client, soit ouvrez le client avec `async with ClaudeSDKClient() as client:`, qui se connecte à l'entrée.

<h2 id="cli-process-exit">
  Sortie du processus CLI
</h2>

Les entrées de cette section signifient que le processus Claude Code s'est terminé pendant que votre application l'utilisait. L'erreur que vous voyez dépend du langage SDK et du fait que le CLI ait signalé un résultat d'erreur avant sa sortie.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError : Commande échouée avec le code de sortie
</h3>

Le SDK Python lève une `ProcessError` quand le processus Claude Code se termine avec un code non nul :

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

Le message indique le code de sortie deux fois, et la ligne `Error output` est du texte fixe plutôt que la sortie d'erreur de votre processus. Le même texte fixe remplit l'attribut `stderr` de l'exception. L'attribut `exit_code` de l'exception porte le code. Pour capturer ce que le CLI a réellement écrit sur stderr, passez un callback `stderr` dans `ClaudeAgentOptions` et enregistrez ce qu'il reçoit.

Une `ProcessError` nue signifie que le CLI s'est terminé sans signaler un résultat d'erreur. Quand le CLI en a signalé un, le SDK lève [`ResultError`](/docs/fr/agent-sdk/python#resulterror) à la place, couvert dans [Claude Code a retourné un résultat d'erreur](#claude-code-returned-an-error-result). `ResultError` est une sous-classe de `ProcessError`, donc `except ProcessError` capture les deux. Pour les gérer différemment, mettez la clause `except ResultError` en premier.

Avant `claude-agent-sdk` 0.2.140, le SDK Python levait les sorties de résultat d'erreur en tant qu'une `Exception` simple plutôt qu'une `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Le processus Claude Code s'est terminé avec le code N
</h3>

Les wrappers IDE impriment aussi ce message, et la [référence d'erreur](/docs/fr/errors#claude-code-process-exited-with-code-n) la couvre pour VS Code et d'autres lanceurs. Cette entrée couvre ce que votre code SDK TypeScript reçoit. Le SDK surface une sortie CLI non nulle en tant qu'une `Error` simple qui rejette la boucle `for await` sur les messages de `query()`. Il n'y a pas de classe d'erreur SDK à capturer, donc enveloppez la boucle dans `try`/`catch` et faites correspondre le message :

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Quand le CLI a écrit sur stderr, le message se termine par la fin de celui-ci. Pour capturer le flux complet, passez un callback `stderr` dans les options de requête. Un processus tué par un signal signale `Claude Code process terminated by signal <name>` de la même forme.

<h3 id="claude-code-returned-an-error-result">
  Claude Code a retourné un résultat d'erreur
</h3>

Les deux SDK remplacent l'erreur de sortie du processus par ce message quand le CLI a signalé un résultat d'erreur avant de se terminer :

```
Claude Code returned an error result: <the CLI's own error report>
```

Le texte après les deux points est le rapport du CLI sur ce qui s'est mal passé, donc commencez par là plutôt que par la sortie elle-même. Python lève cela en tant qu'une [`ResultError`](/docs/fr/agent-sdk/python#resulterror), dont l'attribut `data` porte le résultat d'erreur complet. TypeScript rejette la boucle de message avec une `Error` simple portant la même forme de message.

<h2 id="structured-outputs">
  Sorties structurées
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output est None mais le résultat dit succès
</h3>

Un message de résultat peut se terminer par `subtype: "success"` tandis que `structured_output` est `None` en Python ou `undefined` en TypeScript. L'exécution se termine, mais aucune sortie validée n'existe. Une façon de rencontrer cela est un schéma qu'aucune sortie ne peut satisfaire, par exemple des contraintes de longueur conflictuelles. L'exécution se termine sans erreur de validation, et le seul signal est le `structured_output` manquant.

Traitez ce résultat comme un échec dans le code d'application. Vérifiez à la fois que `subtype` est `success` et que `structured_output` est présent avant de l'utiliser. La section [Gestion des erreurs](/docs/fr/agent-sdk/structured-outputs#error-handling) montre ce modèle pour les deux SDK.

Si cela se produit à plusieurs reprises avec un schéma que vous croyez correct, vérifiez que le schéma est satisfaisable, puis simplifiez-le jusqu'à ce que les sorties se valident, et réintroduisez les contraintes une à la fois.

<h2 id="report-a-new-issue">
  Signaler un nouveau problème
</h2>

Si votre erreur n'est pas couverte ici, vérifiez les problèmes ouverts ou déposez-en un nouveau dans les référentiels SDK : [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) ou [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Incluez le texte d'erreur complet et votre version SDK.
