> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer l'outil Bash en sandbox

> Découvrez comment l'outil Bash en sandbox de Claude Code fournit une isolation du système de fichiers et du réseau pour une exécution d'agent plus sûre et plus autonome.

Le sandbox Bash permet à Claude d'exécuter la plupart des commandes shell sans s'arrêter pour demander une permission. Au lieu d'approuver chaque commande, vous définissez quels fichiers et domaines réseau les commandes peuvent toucher, et le système d'exploitation applique cette limite pour chaque commande Bash, PowerShell ou Monitor et ses processus enfants.

<Note>
  Pour comparer d'autres approches d'isolation telles que les dev containers, les conteneurs personnalisés et les machines virtuelles, consultez [Environnements sandbox](/docs/fr/sandbox-environments). Pour réduire les invites de permission pour les outils autres que Bash, consultez [modes de permission](/docs/fr/permission-modes).
</Note>

<h2 id="get-started">
  Démarrage
</h2>

Le sandbox est intégré à Claude Code et s'exécute sur macOS, Linux et WSL2. Windows natif n'est pas supporté. Sur Windows, exécutez Claude Code à l'intérieur d'une distribution WSL2.

Sur macOS, il n'y a rien à installer : le sandboxing utilise le framework Seatbelt intégré. Sur Linux et WSL2, le sandbox dépend de deux packages, couverts dans [Configurer Linux et WSL2](#set-up-linux-and-wsl2). Même si vous ne les avez pas encore installés, vous pouvez commencer avec `/sandbox`, car son panneau montre si quelque chose manque.

<Steps>
  <Step title="Exécuter /sandbox">
    Démarrez une session Claude Code et exécutez la commande `/sandbox` :

    ```text theme={null}
    /sandbox
    ```

    Cela ouvre le panneau sandbox avec trois onglets, plus un onglet Dependencies sur Linux lorsque le filtre seccomp optionnel manque :

    * **Mode** : choisissez comment les commandes sandboxées sont approuvées, couvert à l'étape suivante
    * **Overrides** : choisissez si les commandes qui échouent sous le sandbox peuvent revenir à une exécution non sandboxée. C'est le paramètre [`allowUnsandboxedCommands`](/docs/fr/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config** : affichage les paramètres de sandbox résolus

    Si le panneau affiche uniquement un onglet Dependencies, un package requis manque. Installez-le comme décrit dans [Configurer Linux et WSL2](#set-up-linux-and-wsl2), redémarrez Claude Code et exécutez `/sandbox` à nouveau.
  </Step>

  <Step title="Choisir un mode">
    Sur l'onglet Mode, sélectionnez auto-allow ou permissions régulières. Auto-allow exécute les commandes sandboxées sans invite, et les permissions régulières conservent les invites de permission régulières même lorsque les commandes sont sandboxées. Consultez [Modes sandbox](#sandbox-modes) pour voir quelles commandes invitent toujours en mode auto-allow.
  </Step>

  <Step title="Exécuter une commande Bash">
    Demandez à Claude d'exécuter une commande, comme une compilation ou une suite de tests. Par défaut, les commandes à l'intérieur du sandbox peuvent écrire dans le répertoire de travail, le répertoire temporaire de la session et tout [répertoire que vous avez ajouté](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) avec `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`.

    La première fois qu'une commande a besoin d'un nouveau domaine réseau, Claude Code demande une approbation ; en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), Claude nomme plutôt les hôtes dont une commande a besoin [sur la commande elle-même](#per-command-allowed-domains-in-auto-mode) pour que le classificateur les examine avec elle.

    Les commandes qui ne peuvent pas s'exécuter sandboxées reviennent au flux de permission régulier. Claude Code intitule leur invite de permission « Bash command (unsandboxed) » au lieu de « Bash command », afin que vous puissiez voir quelles commandes se sont exécutées en dehors du sandbox. Pour élargir ou réduire ce que le sandbox autorise, consultez [Configurer le sandboxing](#configure-sandboxing).

    Si les commandes sandboxées échouent avec `Operation not permitted` à l'intérieur d'un conteneur, consultez l'entrée Bubblewrap sous [Dépannage](#troubleshooting).
  </Step>
</Steps>

Lorsque vous sélectionnez un mode dans le panneau, Claude Code l'enregistre dans les paramètres locaux de votre projet à `.claude/settings.local.json`, qui s'appliquent au projet actuel. Claude Code ajoute ce fichier à votre gitignore global lorsqu'il y enregistre un paramètre. Pour activer le sandbox dans tous vos projets, définissez [`sandbox.enabled`](/docs/fr/settings-reference#sandbox-enabled) sur `true` dans vos paramètres utilisateur à `~/.claude/settings.json`. Pour appliquer le sandboxing pour chaque développeur dans une organisation, utilisez [paramètres gérés](#enforce-sandboxing-with-managed-settings).

Pour modifier le sandbox pour une seule session sans écrire dans un fichier de paramètres, démarrez Claude Code avec [`--settings`](/docs/fr/settings#change-a-setting-for-one-session). Par exemple, cette commande démarre une session sandboxée dans laquelle Claude ne peut pas réessayer une commande bloquée en dehors du sandbox :

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  Par défaut, si le sandbox ne peut pas démarrer parce que les dépendances manquent ou que la plateforme n'est pas supportée, Claude Code affiche un avertissement et exécute les commandes sans sandboxing. Pour en faire un échec dur à la place, définissez [`sandbox.failIfUnavailable`](/docs/fr/settings-reference#sandbox-failifunavailable) sur `true`. Ceci est destiné aux déploiements gérés qui nécessitent le sandboxing comme porte de sécurité.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Configurer Linux et WSL2
</h3>

Sur Linux et WSL2, le sandbox dépend de deux packages :

* [`bubblewrap`](https://github.com/containers/bubblewrap) : l'outil de sandboxing sans privilèges qui applique l'isolation du système de fichiers
* [`socat`](http://www.dest-unreach.org/socat/) : le relais utilisé pour acheminer le trafic réseau via le proxy sandbox

Installez-les avec le gestionnaire de packages de votre distribution :

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Lorsqu'une dépendance manque, l'onglet Dependencies dans `/sandbox` liste lequel de `ripgrep`, `bubblewrap`, `socat` et le filtre seccomp votre plateforme manque. Si vous ne voyez pas l'onglet après l'installation et le redémarrage de Claude Code, toutes les dépendances sont présentes.

Ripgrep est fourni avec le binaire natif Claude Code. Le filtre seccomp est optionnel et ajoute le blocage des sockets de domaine Unix. Installez-le avec `npm install -g @anthropic-ai/sandbox-runtime` s'il manque.

Lorsqu'une dépendance requise manque, l'onglet Dependencies est le seul onglet affiché jusqu'à ce que vous l'installiez. Lorsque seul le filtre seccomp optionnel manque, l'onglet Dependencies apparaît aux côtés des autres onglets. La vérification des dépendances s'exécute au démarrage, donc redémarrez Claude Code après l'installation des packages pour que `/sandbox` les détecte.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 et versions ultérieures : autoriser bubblewrap à créer des espaces de noms utilisateur">
    Sur Ubuntu 24.04 et versions ultérieures, la politique AppArmor par défaut empêche bubblewrap de créer les espaces de noms utilisateur dont il a besoin pour l'isolation.

    Pour vérifier si votre environnement applique cette restriction, y compris à l'intérieur de WSL2, exécutez `sysctl kernel.apparmor_restrict_unprivileged_userns`. Si la commande retourne `0`, ignorez cette étape. Si elle affiche une erreur `No such file or directory`, la clé n'existe pas et vous pouvez ignorer cette étape. Si elle retourne `1`, ajoutez un profil AppArmor qui accorde à `bwrap` cette capacité :

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    Le profil s'applique uniquement à `bwrap` lui-même, pas aux commandes qu'il exécute à l'intérieur du sandbox. Rechargez AppArmor pour l'appliquer :

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="Notes WSL2">
    Vérifiez votre version WSL avec `wsl -l -v` à partir de PowerShell. Si vous voyez `Sandboxing requires WSL2`, votre distribution exécute WSL1. Mettez-la à niveau vers WSL2 ou exécutez Claude Code sans sandboxing.

    Sur WSL2, WSL transmet le lancement d'un binaire Windows tel que `cmd.exe`, `powershell.exe` ou quoi que ce soit sous `/mnt/c/` à l'hôte Windows via un socket Unix, donc le fait qu'une commande sandboxée puisse en lancer un suit les [paramètres Unix-socket](/docs/fr/settings-reference#sandbox-network-allowunixsockets) du sandbox : le filtre seccomp optionnel doit être installé pour bloquer le socket en premier lieu. Pour autoriser ces lancements, définissez `allowAllUnixSockets` ; pour les garder en dehors du sandbox entièrement, ajoutez la commande à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands).
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Modes sandbox
</h3>

Claude Code offre deux modes sandbox. Dans les deux, le sandbox applique les mêmes restrictions de système de fichiers et de réseau ; la différence réside uniquement dans le fait que les commandes sandboxées sont auto-approuvées ou nécessitent une permission explicite.

<h4 id="auto-allow-mode">
  Mode auto-allow
</h4>

Lorsqu'une commande peut être sandboxée, Claude Code l'exécute à l'intérieur du sandbox et l'approuve automatiquement, sans vous demander votre permission. Les commandes qui ne peuvent pas être sandboxées, comme celles nécessitant un accès réseau à des hôtes non autorisés, reviennent au flux de permission régulier, où Claude Code vérifie vos [règles de permission](/docs/fr/permissions) et bloque toute commande que ces règles n'autorisent pas déjà, avec une invite en mode Manuel.

Même en mode auto-allow, les éléments suivants s'appliquent toujours :

* Les [règles de refus](/docs/fr/permissions) explicites sont toujours respectées
* Les commandes `rm` ou `rmdir` qui ciblent un [chemin critique](/docs/fr/permission-modes#critical-paths) passent toujours par le flux de permission régulier
* Les [règles ask](/docs/fr/permissions) délimitées par le contenu comme `Bash(git push *)` forcent toujours une invite même pour les commandes sandboxées
* Une règle ask `Bash` simple, ou la forme équivalente `Bash(*)`, est ignorée pour les commandes qui s'exécutent sandboxées ; elle s'applique toujours aux commandes qui reviennent au flux de permission régulier. En [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), la règle n'est pas ignorée : elle invite pour les commandes sandboxées aussi, y compris les commandes en lecture seule. Avant v2.1.212, l'ignorance s'appliquait aussi en mode plan

<Info>
  Le mode auto-allow fonctionne indépendamment de votre paramètre de mode de permission, sauf en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), pour une commande auto mode qui porte des [domaines autorisés par commande](#per-command-allowed-domains-in-auto-mode), et pour l'[examen du classificateur côté serveur](/docs/fr/permission-modes#how-the-classifier-evaluates-actions) des commandes sandboxées en mode auto. Même si vous n'êtes pas en mode « accepter les modifications », les commandes Bash sandboxées s'exécutent automatiquement lorsque l'auto-allow est activé. Cela signifie que les commandes Bash qui modifient les fichiers dans les limites du sandbox s'exécutent sans invite, même en mode Manuel, où les outils de modification de fichiers inviteraient.

  En mode plan, l'auto-allow n'élargit pas les approbations ; consultez [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour voir comment Claude Code bloque les commandes pendant que vous planifiez. Avant v2.1.212, l'auto-allow exécutait les commandes sandboxées sans invite en mode plan aussi.
</Info>

<h4 id="regular-permissions-mode">
  Mode permissions régulières
</h4>

Toutes les commandes Bash passent par le flux de permission régulier, même lorsqu'elles sont sandboxées. Cela offre plus de contrôle mais nécessite plus d'approbations.

<h4 id="the-unsandboxed-retry-escape-hatch">
  La trappe d'échappement de réessai non sandboxé
</h4>

Certaines commandes ne peuvent pas s'exécuter à l'intérieur du sandbox du tout, comme les outils qui sont incompatibles avec lui ou qui ont besoin d'un hôte que vous n'avez pas autorisé. Claude Code signale les violations du sandbox dans le résultat de la commande bloquée, en nommant le chemin ou l'hôte que le sandbox a refusé, afin que Claude voie ce que le sandbox a bloqué. Plutôt que d'échouer la tâche ou de vous demander d'éteindre le sandboxing, Claude Code inclut une trappe d'échappement : Claude analyse la violation et peut réessayer la commande avec le paramètre `dangerouslyDisableSandbox`.

La commande réessayée s'exécute en dehors du sandbox, elle passe donc par le flux de permission régulier. En mode Manuel vous obtenez une invite de confirmation. En [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), le classificateur évalue la commande sous-jacente. Pendant que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) est activé, un réessai qui a besoin d'approbation pour s'exécuter en dehors du sandbox vous invite à la place. Pour être invité à chaque réessai non sandboxé même en mode auto, ajoutez une [règle ask](/docs/fr/permissions#match-by-input-parameter) pour `Bash(dangerouslyDisableSandbox:true)`.

Vous pouvez désactiver cette trappe d'échappement en définissant `"allowUnsandboxedCommands": false` dans vos [paramètres de sandbox](/docs/fr/settings-reference#sandbox-settings). Avec la trappe d'échappement désactivée, Claude Code ignore le paramètre `dangerouslyDisableSandbox`, et chaque commande que Claude exécute doit s'exécuter sandboxée à moins que vous l'ayez listée dans `excludedCommands`. L'onglet **Overrides** de `/sandbox` affiche ce paramètre comme **Mode sandbox strict**.

Le mode sandbox strict s'applique aux commandes que Claude exécute. Les commandes que vous tapez vous-même à l'[invite shell-mode `!`](/docs/fr/interactive-mode#shell-mode-with-prefix) s'exécutent en dehors du sandbox à moins que la session soit l'une de celles-ci :

* **Une [session en arrière-plan](/docs/fr/agent-view)** : le mode sandbox strict couvre aussi les commandes shell-mode
* **Une session Linux avec [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/fr/env-vars#variables) défini** : chaque commande s'exécute sandboxée, commandes shell-mode incluses

Avant v2.1.260, le mode sandbox strict sandboxait les commandes shell-mode dans chaque session.

<h4 id="temporary-directories">
  Répertoires temporaires
</h4>

Le répertoire temporaire de la session est inscriptible à l'intérieur du sandbox par défaut, aux côtés du répertoire de travail. À moins que vous [désactiviez l'isolation du système de fichiers](#disable-filesystem-isolation), Claude Code définit `$TMPDIR` sur ce répertoire pour les commandes sandboxées, de sorte que les outils qui écrivent des fichiers temporaires fonctionnent sans configuration supplémentaire.

Les commandes non sandboxées héritent de votre `$TMPDIR` shell lorsqu'il est défini, donc pendant que l'isolation du système de fichiers est activée, les commandes sandboxées et non sandboxées résolvent `$TMPDIR` à des répertoires différents. Si votre shell laisse `$TMPDIR` indéfini ou vide, une commande non sandboxée qui référence `$TMPDIR` reçoit votre remplacement [`CLAUDE_CODE_TMPDIR`](/docs/fr/env-vars) ou le répertoire temporaire du système d'exploitation lorsque vous n'en avez pas défini un ou que le remplacement est un chemin long, de sorte que la variable ne se développe pas en une chaîne vide. Pour transmettre des fichiers temporaires entre les deux, écrivez-les plutôt sous le répertoire de travail.

<h2 id="configure-sandboxing">
  Configurer le sandboxing
</h2>

Personnalisez le comportement du sandbox via votre fichier `settings.json`. Consultez [Paramètres](/docs/fr/settings-reference#sandbox-settings) pour la référence de configuration complète.

Par défaut, les commandes sandboxées peuvent écrire dans le répertoire de travail actuel, le répertoire temporaire de la session, et tous les [répertoires que vous avez ajoutés](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) avec `--add-dir`, `/add-dir`, ou `permissions.additionalDirectories`. Si les commandes de sous-processus comme `kubectl`, `terraform` ou `npm` doivent écrire en dehors de ces répertoires, utilisez `sandbox.filesystem.allowWrite` pour accorder l'accès à des chemins spécifiques :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Ces chemins sont appliqués au niveau du système d'exploitation, donc toutes les commandes s'exécutant à l'intérieur du sandbox, y compris leurs processus enfants, les respectent. C'est l'approche recommandée lorsqu'un outil a besoin d'un accès en écriture à un emplacement spécifique, plutôt que d'exclure complètement l'outil du sandbox avec `excludedCommands`.

Lorsque vous définissez le même tableau de système de fichiers dans plusieurs [portées de paramètres](/docs/fr/settings#settings-precedence), Claude Code les fusionne, combinant les chemins de chaque portée plutôt que de remplacer le tableau d'une portée par celui d'une autre.

Si vous excluez une source avec [`--setting-sources`](/docs/fr/cli-reference) sur la CLI ou [`settingSources`](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) dans l'Agent SDK, Claude Code ignore ses entrées `sandbox.filesystem`, ses règles de permission `Edit`, et ses règles de refus `Read` lors de la construction de la configuration du sandbox. Nécessite Claude Code v2.1.246 ou ultérieur.

Lorsque vous modifiez ces listes de système de fichiers pendant une session, Claude Code [applique la modification à la session en cours d'exécution](/docs/fr/settings#when-edits-take-effect), donc la prochaine commande sandboxée s'exécute sous les nouveaux chemins.

Les préfixes de chemin contrôlent la façon dont les chemins sont résolus :

| Préfixe                | Signification                                                                                                 | Exemple                                                                      |
| :--------------------- | :------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------- |
| `/`                    | Chemin absolu à partir de la racine du système de fichiers                                                    | `/tmp/build` reste `/tmp/build`                                              |
| `~/`                   | Relatif au répertoire personnel                                                                               | `~/.kube` devient `$HOME/.kube`                                              |
| `./` ou pas de préfixe | Relatif à la racine du projet pour les paramètres du projet, ou à `~/.claude` pour les paramètres utilisateur | `./output` dans `.claude/settings.json` se résout en `<project-root>/output` |

Cette syntaxe diffère des [règles de permission Read et Edit](/docs/fr/permissions#read-and-edit), qui utilisent `//path` pour absolu et `/path` pour relatif au projet. Les chemins du système de fichiers du sandbox utilisent les conventions standard : `/tmp/build` est absolu. Pour savoir comment Claude Code traite une barre oblique finale ou un caractère générique dans ces chemins, consultez [Préfixes de chemin du sandbox](/docs/fr/settings-reference#sandbox-path-prefixes).

Vous pouvez également refuser l'accès en écriture ou en lecture en utilisant `sandbox.filesystem.denyWrite` et `sandbox.filesystem.denyRead`, et réautoriser des chemins spécifiques dans une région refusée en utilisant `sandbox.filesystem.allowRead`. Lorsque les règles de lecture se chevauchent, le chemin le plus spécifique gagne :

| Règles d'exemple                                        | Résultat                                                                                                                                                                                                                                                 |
| :------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` avec `"allowRead": ["~/projects"]` | `~/projects` est lisible et le reste du répertoire personnel reste bloqué. L'autorisation plus étroite réouvre cette partie de la région refusée                                                                                                         |
| `"allowRead": ["~/"]` avec `"denyRead": ["~/.env"]`     | `~/.env` reste bloqué et le reste du répertoire personnel est lisible. Le refus tient à l'intérieur d'une autorisation plus large, donc une autorisation large ne peut pas réexposer silencieusement un secret                                           |
| `"allowRead": ["~/"]` avec `"denyRead": ["~/**/.env"]`  | Chaque `.env` sous le répertoire personnel reste bloqué et le reste est lisible. Un [refus avec caractère générique](/docs/fr/settings-reference#sandbox-path-prefixes) tient à l'intérieur d'une autorisation plus large de la même façon qu'un chemin exact |

L'exemple ci-dessous bloque la lecture de l'ensemble du répertoire personnel tout en autorisant toujours les lectures du projet actuel. Placez-le dans le `.claude/settings.json` de votre projet, car le chemin relatif `.` se résout à la racine du projet uniquement lorsque la configuration se trouve dans les paramètres du projet :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Si vous aviez placé la même configuration dans `~/.claude/settings.json`, `.` se résoudrait à `~/.claude` à la place, et les fichiers du projet resteraient bloqués par la règle `denyRead`.

Pour refuser aux commandes sandboxées l'accès en lecture aux répertoires personnels et aux volumes montés tout en gardant les répertoires de travail lisibles, définissez [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) à la place d'écrire des règles de chemin.

<h3 id="disable-filesystem-isolation">
  Désactiver l'isolation du système de fichiers
</h3>

Définissez `sandbox.filesystem.disabled` à `true` pour ignorer l'isolation du système de fichiers tout en conservant l'isolation réseau. L'exemple ci-dessous désactive l'isolation du système de fichiers tout en conservant une liste d'autorisation de domaines réseau :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Le sandbox a deux couches indépendantes : [l'isolation du système de fichiers](#filesystem-isolation) contrôle les chemins que les commandes sandboxées peuvent lire et écrire, et [l'isolation réseau](#network-isolation) contrôle les domaines qu'elles peuvent atteindre. Avec la couche du système de fichiers désactivée, les commandes sandboxées obtiennent un accès en lecture et écriture sans restriction au système de fichiers hôte, tandis que leur sortie réseau reste limitée à vos domaines autorisés. Désactivez la couche lorsque vous sandboxez pour contrôler où les commandes se connectent plutôt que ce qu'elles écrivent.

Le paramètre est désactivé par défaut et s'applique sur les plateformes où le sandbox s'exécute : macOS, Linux et WSL2. Nécessite Claude Code v2.1.216 ou ultérieur.

<Warning>
  Avec l'isolation du système de fichiers désactivée et les commandes auto-autorisées, une commande sandboxée peut écrire des fichiers que les commandes ultérieures exécutent ou lisent, comme les fichiers de démarrage du shell, les exécutables sur `$PATH`, ou `~/.claude/settings.json`, et les utiliser pour élargir son propre accès à la prochaine exécution. Définissez `filesystem.disabled` à `true` uniquement pour les charges de travail auxquelles vous faites confiance pour ne pas escalader leur propre accès. Verrouiller les domaines réseau avec [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) réduit le risque mais ne l'élimine pas, car ce verrouillage s'applique uniquement aux commandes s'exécutant à l'intérieur du sandbox.
</Warning>

<h4 id="which-settings-can-disable-it">
  Quels paramètres peuvent le désactiver
</h4>

Parce que désactiver l'isolation du système de fichiers élargit ce que les commandes sandboxées peuvent faire, Claude Code honore `filesystem.disabled` uniquement à partir de ces sources de paramètres :

* Les paramètres utilisateur, les paramètres gérés et l'indicateur CLI `--settings` peuvent le définir. Les paramètres du projet dans `.claude/settings.json` et `.claude/settings.local.json` ne peuvent pas, donc un projet extrait ne peut pas désactiver l'isolation du système de fichiers.
* Lorsque les paramètres gérés configurent `sandbox.filesystem` du tout, ou listent une entrée `sandbox.credentials.files` avec `"mode": "deny"`, seuls les paramètres gérés peuvent définir la clé. Cela maintient les restrictions du système de fichiers déployées par l'administrateur en vigueur ; pour assouplir un tel déploiement, définissez `"disabled": true` dans les paramètres gérés.
* Lorsque [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/fr/env-vars) est défini, Claude Code ignore `filesystem.disabled` de chaque source, y compris les paramètres gérés, et maintient l'isolation du système de fichiers activée.

Qu'une entrée `credentials.files` gérée épingle `filesystem.disabled`, verrouillant la clé aux paramètres gérés pour que les développeurs ne puissent pas désactiver l'isolation du système de fichiers, dépend du `mode` de l'entrée et de ce qui se passe à l'entrée lorsque le sandbox démarre :

| Entrée gérée                                                                                                  | Épingle `filesystem.disabled`  | Ce qui protège le fichier lorsque l'isolation est désactivée                                                                                           |
| ------------------------------------------------------------------------------------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `"mode": "deny"`                                                                                              | Oui                            | Rien : le bloc de lecture fait partie de la couche du système de fichiers                                                                              |
| `"mode": "mask"`, appliqué comme un masque                                                                    | Non                            | Le masquage lui-même : la [copie sentinelle et le proxy](#mask-credential-files) sur Linux et WSL2, les propres règles de lecture du sandbox sur macOS |
| `"mode": "mask"`, [revenu à `deny`](#mask-credential-files) à la configuration                                | Non                            | Rien, comme `deny`. Listez un chemin qui ne peut pas être masqué, comme un répertoire, comme une entrée `deny` explicite, qui épingle la clé           |
| `"mode": "mask"`, [dégradé à `deny` par validation](/docs/fr/managed-settings#invalid-entries-in-managed-settings) | Oui, comme un `deny` explicite | Rien, comme `deny`                                                                                                                                     |

Un retour se produit lorsque le sandbox démarre, après que Claude Code ait déjà lu les paramètres sur lesquels la vérification de l'épingle s'exécute, donc une entrée revenue n'épingle jamais. La validation réécrit une entrée invalide en `deny` pendant le chargement des paramètres, donc une entrée dégradée épingle comme celle que vous avez écrite en `deny`.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  Ce qui change lorsque l'isolation du système de fichiers est désactivée
</h4>

Définir `filesystem.disabled` lève les protections que la couche du système de fichiers elle-même applique. Les protections que d'autres couches appliquent continuent de s'appliquer :

| Protection                                                                                   | Avec l'isolation du système de fichiers désactivée                                                                                                                                                  |
| -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` et [`credentials.files`](#protect-credentials) blocs de lecture `deny` | Non appliqué. La couche du système de fichiers applique les deux                                                                                                                                    |
| `credentials.envVars` entrées `deny` et `mask`                                               | Appliqué. Le nettoyage des variables d'environnement est indépendant de la couche du système de fichiers                                                                                            |
| [`credentials.files` entrées `mask`](#mask-credential-files) appliquées comme des masques    | Appliqué : le masquage est indépendant de la couche du système de fichiers. Une entrée qui [est revenue à `deny`](#mask-credential-files) n'est pas appliquée, comme n'importe quelle entrée `deny` |

Deux autres choses changent :

* Les commandes sandboxées héritent du `$TMPDIR` de votre shell au lieu du répertoire temporaire de la session, car chaque répertoire temporaire est inscriptible et Claude Code ne redirige plus les commandes vers celui de la session.

  Sur Linux, la variable est souvent non définie dans le shell parent. Le guide de l'outil Bash indique à Claude de créer des répertoires de travail avec `mktemp -d` au lieu de compter sur `$TMPDIR`.
* [`autoAllowBashIfSandboxed`](/docs/fr/settings-reference#sandbox-autoallowbashifsandboxed) continue de défaut à `true`, donc les commandes sandboxées continuent de s'exécuter sans invites. Définissez-le à `false` pour inviter les commandes sandboxées.

<h3 id="protect-credentials">
  Protéger les identifiants
</h3>

Le paramètre `sandbox.credentials` déclare les fichiers d'identifiants et les variables d'environnement à protéger des commandes sandboxées. Chaque entrée nomme un chemin de fichier ou une variable d'environnement et un `mode`. Le bloc `credentials` dédié maintient les règles d'identifiants groupées ensemble et séparées des règles générales du système de fichiers.

Pour les entrées avec `"mode": "deny"`, les chemins de fichiers sont refusés pour les lectures à l'intérieur du sandbox, la même restriction que celle appliquée par `filesystem.denyRead`, et les variables d'environnement sont supprimées avant chaque exécution de commande sandboxée. La protection des fichiers fait partie de la couche du système de fichiers, donc elle ne s'applique pas si vous [désactivez l'isolation du système de fichiers](#disable-filesystem-isolation) ; la protection des variables d'environnement continue de s'appliquer.

L'exemple ci-dessous bloque les lectures du fichier d'identifiants AWS et du répertoire SSH et supprime `GITHUB_TOKEN` et `NPM_TOKEN` de l'environnement des commandes sandboxées :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Les entrées de variables d'environnement et les entrées de fichiers acceptent également `"mode": "mask"`, décrit sous [Masquer les identifiants](#mask-credentials).

Les chemins de fichiers suivent les mêmes [règles de préfixe](/docs/fr/settings-reference#sandbox-path-prefixes) que les paramètres `sandbox.filesystem.*`.

Claude Code fusionne les entrées `deny` de chaque [portée de paramètres](/docs/fr/settings#settings-precedence) que la session charge. Une entrée `deny` ne fait que réduire l'accès, donc n'importe quelle portée peut en ajouter une, mais aucune portée ne peut en supprimer une qu'une autre portée a ajoutée.

Lorsque vous [excluez une source de paramètres](#configure-sandboxing) :

* **Paramètres du projet ou locaux** : Claude Code n'applique aucune de leurs entrées `credentials`. Nécessite Claude Code v2.1.246 ou ultérieur.
* **Paramètres utilisateur** : Claude Code applique toujours les entrées `deny` dans `~/.claude/settings.json` et maintient ses [entrées `mask` de fichier](#mask-credential-files) comme des restrictions, mais supprime ses [entrées `mask` de variable d'environnement](#mask-environment-variables).

Il n'y a pas de liste de refus d'identifiants intégrée, donc seuls les fichiers et les variables que vous listez sont restreints.

`sandbox.credentials` affecte uniquement les commandes Bash sandboxées. Pour supprimer les identifiants de tous les sous-processus indépendamment du sandboxing, définissez [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/fr/env-vars).

<h3 id="mask-credentials">
  Masquer les identifiants
</h3>

Le masquage va plus loin qu'une entrée `deny` sous [Protéger les identifiants](#protect-credentials). Au lieu de bloquer un identifiant, Claude Code montre aux commandes sandboxées un espace réservé, la sentinelle, et le [proxy sandbox](#network-isolation) échange la vraie valeur sur les requêtes sortantes vers les hôtes que vous autorisez. Pour les fichiers, la substitution est le comportement de Linux et WSL2 ; [macOS bloque le fichier à la place](#mask-credential-files).

<h4 id="mask-environment-variables">
  Masquer les variables d'environnement
</h4>

`"mode": "mask"` protège un identifiant tout en gardant les outils qui s'authentifient avec lui fonctionnels. `deny` supprime complètement la variable, ce qui casse également les outils qui en ont besoin, comme `gh` ou `npm`. Nécessite Claude Code v2.1.199 ou ultérieur.

Avec `mask`, la commande sandboxée voit une valeur sentinelle par session au lieu de la vraie. Chaque entrée `mask` peut lister `injectHosts`, les hôtes auxquels la vraie valeur est autorisée à atteindre. Lorsqu'une requête quitte le sandbox pour l'un d'eux, le [proxy sandbox](#network-isolation) remplace la sentinelle par la vraie valeur. La commande et tout ce qu'elle enregistre ne détiennent jamais l'identifiant réel, mais ses requêtes s'authentifient toujours.

Le proxy substitue l'identifiant à l'intérieur du contenu des requêtes, donc il doit les voir. Définissez [`network.tlsTerminate`](/docs/fr/settings-reference#sandbox-network-tlsterminate) pour que le proxy termine TLS lui-même.

Sans cela, le masquage échoue fermé : la commande voit toujours seulement la sentinelle, mais la sentinelle atteint le serveur inchangée et l'authentification échoue. Claude Code signale cette mauvaise configuration au démarrage.

La substitution couvre les en-têtes et les corps de requête. Les requêtes qui s'authentifient avec une signature dérivée de l'identifiant, plutôt que l'identifiant lui-même, ont besoin d'une re-signature au proxy ; [Re-signer les requêtes AWS](#re-sign-aws-requests) couvre comment cela fonctionne pour AWS.

Le proxy injecte uniquement sur les connexions que la [liste d'autorisation de domaines](#network-isolation) admet, donc chaque destination `injectHosts` doit également être accessible via `network.allowedDomains`.

L'exemple ci-dessous masque deux tokens. `GH_TOKEN` est substitué uniquement sur les requêtes à `api.github.com`, tandis que `NPM_TOKEN` n'a pas de `injectHosts` et est substitué sur les requêtes à chaque hôte dans `network.allowedDomains`.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

<span id="ipv6-destinations-in-injecthosts" />Épellez une destination IPv6 différemment dans les deux listes, car chaque liste a son propre matcher :

* **`network.allowedDomains`** : la [forme entre crochets que les listes de domaines utilisent](#ipv6-addresses-in-domain-lists), comme `"[::1]"`. Le proxy vérifie cette liste pour admettre la connexion.
* **`injectHosts`** : l'adresse nue dans sa forme canonique compressée, comme `"::1"` ou `"2001:db8::1"`. Le proxy compare chaque entrée à l'adresse de destination nue de la connexion, en ignorant les ports, donc une épellation entre crochets, avec ID de zone, ou compressée différemment ne correspond jamais et le proxy n'injecte jamais l'identifiant là.

`claude doctor` signale les entrées `injectHosts` qui ne peuvent jamais correspondre avec l'avertissement `Sandbox credential injectHosts entries can never match their destination`. Cette vérification nécessite Claude Code v2.1.229 ou ultérieur.

Contrairement à `deny`, le masquage autorise le proxy à envoyer votre identifiant réel aux hôtes listés, donc Claude Code l'honore uniquement à partir des paramètres que vous ou votre administrateur contrôlez : les paramètres utilisateur, les paramètres gérés et l'indicateur CLI `--settings`. Claude Code ignore les entrées `mask` dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un référentiel. Dans ces fichiers, il ignore également `network.tlsTerminate` et [`credentials.allowPlaintextInject`](/docs/fr/settings-reference#sandbox-credentials-allowplaintextinject), le paramètre qui permet au proxy d'injecter des identifiants dans les requêtes non chiffrées. Si vous [excluez les paramètres utilisateur](#configure-sandboxing), Claude Code supprime également les entrées `mask` de variable d'environnement dans `~/.claude/settings.json`.

Lorsque votre administrateur fournit des entrées `mask`, `network.tlsTerminate`, ou `credentials.allowPlaintextInject` via les paramètres gérés par le serveur, ils comptent comme [paramètres qui nécessitent une approbation](/docs/fr/server-managed-settings#security-approval-dialogs).

Lorsque la même variable est listée avec `deny` dans n'importe quelle portée, `deny` prend la priorité.

Le masquage remplace la valeur entière de la variable par défaut, ce qui convient à un token nu. Les champs d'entrée optionnels, qui nécessitent Claude Code v2.1.224 ou ultérieur, gèrent les valeurs avec structure :

* `extract` : une expression régulière que Claude Code applique sur la valeur, en remplaçant uniquement le texte capturé par le groupe 1 de chaque correspondance, donc un outil qui analyse la valeur, comme une chaîne de connexion `DATABASE_URL`, continue de fonctionner à l'intérieur du sandbox. Le motif doit contenir au moins un groupe de capture.
* `onExtractNoMatch` contrôle ce qui se passe lorsque le motif ne correspond à rien :
  * `warn`, la valeur par défaut, avertit et transmet la variable sans masque
  * `deny` désactive la variable à l'intérieur du sandbox
  * `error` arrête la configuration du sandbox jusqu'à ce que vous corrigiez la configuration
* `decode: "jwt"` : pour une variable contenant un JSON Web Token (JWT). Claude Code vérifie que la valeur est un JWT et la remplace par un faux token structurellement valide, donc le code à l'intérieur du sandbox qui décode le token continue de fonctionner. Ajoutez `maskClaims` pour lister les revendications de charge utile de haut niveau à masquer individuellement au lieu de remplacer le token entier ; les autres revendications restent lisibles. Lorsque la valeur ne se vérifie pas comme un JWT, ou qu'aucune revendication listée ne correspond, Claude Code transmet la variable sans masque avec un avertissement. `decode` ne peut pas être combiné avec `extract`.

Consultez les [lignes `credentials.envVars[]` dans la référence des paramètres](/docs/fr/settings-reference#sandbox-settings) pour la liste complète des champs.

<h4 id="re-sign-aws-requests">
  Re-signer les requêtes AWS
</h4>

Les requêtes AWS portent des signatures SigV4 sur le contenu de la requête, donc masquez `AWS_ACCESS_KEY_ID` et `AWS_SECRET_ACCESS_KEY` ensemble. Le proxy détecte une requête SigV4 par la sentinelle de la clé d'accès et la re-signe après substitution des vraies valeurs. Masquer uniquement le secret laisse les requêtes signées avec l'espace réservé, que le proxy ne peut pas détecter, donc elles échouent à AWS ; Claude Code avertit à ce sujet au démarrage, mais pas lorsque seul l'ID de clé d'accès est masqué. Une requête détectée que le proxy ne peut pas re-signer, comme une requête manquant son en-tête `x-amz-date`, échoue avec une erreur de proxy au lieu d'atteindre le serveur avec une signature cassée.

Claude Code lie les variables conventionnelles `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` et `AWS_SESSION_TOKEN` en un identifiant automatiquement lorsque vous masquez leurs valeurs entières. Si votre identifiant AWS se trouve dans des variables avec d'autres noms, groupez-les vous-même avec [`credentials.awsPairs`](/docs/fr/settings-reference#sandbox-credentials-awspairs), qui nécessite Claude Code v2.1.224 ou ultérieur. Cet exemple ajoute l'appairage à une configuration qui masque déjà `MY_KEY_ID`, `MY_SECRET_KEY` et `MY_SESSION_TOKEN` en valeur entière, comme dans la [configuration de masquage ci-dessus](#mask-environment-variables) :

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Chaque entrée suit ces règles :

* `accessKeyIdVar` et `secretAccessKeyVar` nomment les entrées `envVars` masquées contenant l'ID de clé d'accès et la clé secrète. Le `sessionTokenVar` optionnel nomme l'entrée contenant le token de session pour les identifiants temporaires ; lorsqu'il est défini, le proxy envoie le vrai token comme `x-amz-security-token` sur les requêtes re-signées.
* Chaque variable nommée doit être une entrée `mask` qui masque sa valeur entière, sans `extract` ou `decode`.
* Le proxy re-signe les requêtes sur les hôtes listés dans `injectHosts` de l'entrée d'ID de clé d'accès.
* Nommer l'une des variables conventionnelles dans une paire remplace l'appairage automatique.

Comme les entrées `mask`, `awsPairs` est honoré uniquement à partir des paramètres utilisateur, des paramètres gérés et de l'indicateur CLI `--settings`.

Trois formes de requête AWS portent des signatures que le proxy ne peut pas recalculer. Lorsqu'une telle requête est signée avec l'espace réservé d'une paire masquée, le proxy l'échoue plutôt que de transférer une signature cassée ; les requêtes signées avec des identifiants non masqués ne sont jamais affectées. Le paramètre [`credentials.sigv4`](/docs/fr/settings-reference#sandbox-credentials-sigv4), qui nécessite Claude Code v2.1.224 ou ultérieur, assouplit cela par forme : définir la clé d'une forme à `passthrough` transfère la requête avec sa signature dérivée de l'espace réservé, donc l'outil appelant reçoit la propre réponse de rejet d'AWS au lieu d'une erreur de proxy. Comme `awsPairs`, `sigv4` est honoré uniquement à partir des paramètres utilisateur, des paramètres gérés et de l'indicateur CLI `--settings`.

| Forme de requête                         | Clé `sigv4` | Pourquoi le proxy ne peut pas la re-signer                                                                                        |
| :--------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| Téléchargements de streaming aws-chunked | `streaming` | Les signatures par chunk s'enchaînent à partir de la signature de départ, donc la re-signature nécessiterait de réécrire le corps |
| URLs pré-signées                         | `presigned` | La signature se trouve dans l'URL elle-même, sans en-tête `Authorization`                                                         |
| Signatures asymétriques SigV4A           | `sigv4a`    | Il n'y a pas de HMAC à clé partagée à recalculer                                                                                  |

<h4 id="mask-credential-files">
  Masquer les fichiers d'identifiants
</h4>

Les entrées de fichiers acceptent également `"mode": "mask"`, qui nécessite Claude Code v2.1.221 ou ultérieur. Ce qu'une commande sandboxée voit dépend de la plateforme :

* **Linux et WSL2** : les commandes sandboxées lisent une copie sentinelle du fichier, un substitut dont le secret est remplacé par une valeur d'espace réservé, et le [proxy sandbox](#network-isolation) substitue la vraie valeur à la sortie.
* **macOS** : les commandes sandboxées ne peuvent pas lire le fichier listé du tout. Claude Code ne construit pas de copie sentinelle et ne substitue rien à la sortie, donc les outils qui s'authentifient avec le fichier ne fonctionnent pas à l'intérieur du sandbox, le même effet que `deny`. Contrairement à une entrée `deny`, le bloc de lecture tient même lorsque vous [désactivez l'isolation du système de fichiers](#disable-filesystem-isolation).

Sur chaque plateforme, Claude Code applique le besoin [`network.tlsTerminate`](/docs/fr/settings-reference#sandbox-network-tlsterminate) et `injectHosts` de la même façon que pour les [variables d'environnement masquées](#mask-environment-variables), et ignore les paramètres du référentiel de la même façon. Si vous [excluez les paramètres utilisateur](#configure-sandboxing), Claude Code maintient les entrées `mask` de fichier dans `~/.claude/settings.json` comme des restrictions, mais les entrées n'autorisent plus le proxy à substituer la vraie valeur.

L'exemple ci-dessous masque un token GitHub stocké dans `~/.config/gh/hosts.yml` ; le motif `extract`, couvert ci-dessous, indique à Claude Code quelle partie du fichier est le secret. Sur Linux et WSL2, les commandes sandboxées qui lisent le fichier obtiennent une sentinelle à la place du token, et le proxy substitue le vrai token sur les requêtes à `api.github.com` :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Pour confirmer que le masque est actif, demandez à Claude d'exécuter `cat ~/.config/gh/hosts.yml` dans une commande sandboxée : sur Linux et WSL2, la sortie montre une valeur sentinelle à la place du token, et sur macOS, la lecture échoue à la place.

Sur Linux et WSL2, le motif `extract` est ce qui garde le reste de `hosts.yml` lisible. Claude Code applique l'expression régulière sur le fichier entier et remplace uniquement le texte capturé par le groupe 1 de chaque correspondance, donc `gh` analyse toujours sa configuration et seul le token est un espace réservé. Utilisez `extract` pour tout fichier structuré que les outils analysent, comme `.netrc`, JSON ou YAML ; le motif doit contenir au moins un groupe de capture. Sans `extract`, Claude Code remplace le contenu entier du fichier par une valeur sentinelle, ce qui convient à un fichier qui contient un seul secret nu et rien d'autre.

Pour un fichier qui contient un JSON Web Token (JWT), définissez `decode: "jwt"` à la place de, ou ensemble avec, `extract`. `decode` nécessite Claude Code v2.1.224 ou ultérieur. Claude Code trouve les candidats JWT avec un motif intégré, ou avec votre motif `extract` lorsqu'il est défini, vérifie que chaque candidat est un JWT, et le remplace par un faux token structurellement valide, donc le code qui décode le token à l'intérieur du sandbox continue de fonctionner. Ajoutez `maskClaims` pour masquer uniquement les revendications de charge utile de haut niveau nommées à l'intérieur de chaque token vérifié et laissez les autres revendications lisibles. Lorsqu'aucun candidat ne se vérifie, ou qu'aucune revendication nommée ne correspond, le champ `onExtractNoMatch` ci-dessous gouverne le résultat, comme il le fait pour un motif qui ne correspond à rien.

Deux champs optionnels affinent le comportement de la correspondance. Les deux s'appliquent uniquement lorsque `mode` est `mask` et que `extract` ou `decode` est défini. Sur macOS, Claude Code applique les entrées `mask` comme `deny` avant que le motif s'exécute chaque fois que l'isolation du système de fichiers est activée, donc ces champs, et les résultats de non-correspondance ci-dessous, prennent effet là uniquement lorsque [l'isolation du système de fichiers est désactivée](#disable-filesystem-isolation) :

* `onExtractNoMatch` contrôle ce qui se passe lorsque la correspondance ne trouve rien à masquer dans le fichier :

  * `warn`, la valeur par défaut, avertit et ignore l'entrée, donc les commandes sandboxées peuvent lire le fichier réel sans masque. La valeur par défaut convient aux identifiants qui peuvent être légitimement absents ; si le secret pourrait être présent mais le motif pourrait le manquer, utilisez `deny`
  * `deny` rend le fichier illisible à la place
  * `error` arrête la configuration du sandbox jusqu'à ce que vous corrigiez la configuration

  Claude Code traite `deny` comme `error` chaque fois que le bloc de lecture ne serait pas appliqué : lorsque vous [désactivez l'isolation du système de fichiers](#disable-filesystem-isolation), et lorsqu'une entrée `filesystem.allowRead` de n'importe quelle source de paramètres réouvre le chemin du fichier.
* `maskDuplicates` remplace également les copies verbatim de chaque valeur d'identifiant masqué, une capture `extract` ou un token vérifié `decode`, trouvé en dehors des portées correspondantes, pour un secret répété où la correspondance ne s'étend pas. Il correspond à des sous-chaînes brutes, donc une valeur courte ou commune serait remplacée partout où elle apparaît ; réservez-la aux secrets longs et à haute entropie. Valeur par défaut : false.

`mask` s'applique à un seul fichier, donc listez chaque fichier d'identifiant individuellement. Claude Code revient à `deny` pour une entrée `mask` qu'il ne peut pas masquer en toute sécurité : un chemin de répertoire, un motif glob, un fichier plus grand que 8 MiB, ou un fichier qui n'est pas du texte UTF-8. Écrivez les répertoires comme des entrées `deny` explicites à la place ; le tableau sous [Quels paramètres peuvent le désactiver](#which-settings-can-disable-it) couvre si chaque forme épingle `filesystem.disabled` et comment elle se comporte avec l'isolation du système de fichiers désactivée.

<h2 id="how-sandboxing-works">
  Comment fonctionne le sandboxing
</h2>

<h3 id="filesystem-isolation">
  Isolation du système de fichiers
</h3>

L'outil Bash en sandbox restreint l'accès au système de fichiers à des répertoires spécifiques :

* **Comportement d'écriture par défaut** : accès en lecture et écriture au répertoire de travail actuel et à ses sous-répertoires, tous les répertoires que vous avez ajoutés avec `--add-dir`, `/add-dir`, ou [`permissions.additionalDirectories`](/docs/fr/settings-reference#permissions-additionaldirectories), plus le répertoire temporaire de session vers lequel `$TMPDIR` pointe
* **Comportement de lecture par défaut** : accès en lecture à l'ensemble de l'ordinateur, sauf certains répertoires refusés. Notez que ce comportement par défaut permet toujours de lire les fichiers d'identifiants tels que `~/.aws/credentials` et `~/.ssh/`. Utilisez [`sandbox.credentials`](#protect-credentials) pour bloquer les lectures de ces fichiers et désactiver les variables d'environnement secrètes, ou ajoutez les chemins à `denyRead`.
* **Accès bloqué** : impossible de modifier les fichiers en dehors du répertoire de travail, des répertoires ajoutés et du répertoire temporaire de session sans permission explicite, y compris les fichiers de configuration shell tels que `~/.bashrc` et les binaires système dans `/bin/`
* **Git worktrees** : lorsque le répertoire de travail est un [linked git worktree](/docs/fr/worktrees), le sandbox permet également les écritures dans le répertoire `.git` partagé du référentiel principal afin que les commandes telles que `git commit` puissent mettre à jour les références et l'index. Les écritures dans `hooks/` et `config` à l'intérieur de ce répertoire restent refusées.
* **Configurable** : définissez des chemins autorisés et refusés personnalisés via les paramètres

Pour ignorer complètement l'isolation du système de fichiers tout en conservant l'isolation réseau, définissez [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Chemins protégés
</h3>

À l'intérieur des répertoires dans lesquels les commandes en sandbox peuvent écrire, le sandbox refuse toujours les écritures dans les fichiers à partir desquels Claude Code charge la configuration et le code. Une commande qui pourrait modifier ces fichiers pourrait s'accorder des permissions, ou ajouter un hook ou un serveur MCP que Claude Code exécute en dehors du sandbox. Le système de permissions a ses propres [chemins protégés](/docs/fr/permission-modes#protected-paths), qui contrôlent ce que Claude Code approuve avant qu'un outil ne s'exécute ; la liste du sandbox s'applique à une commande qui est déjà en cours d'exécution. Elle couvre quatre groupes de chemins :

* **Dans votre répertoire de travail et les répertoires au-dessus** : les fichiers de paramètres `.claude`, les répertoires `.claude/skills`, `.claude/agents`, `.claude/commands`, et `.claude/hooks`, `.mcp.json`, et les fichiers que Claude Code exécute de lui-même, tels que `.claude/workflows` et `.claude/scheduled_tasks.json`
* **Dans votre répertoire de travail uniquement** : les fichiers de démarrage shell tels que `.bashrc` et `.zshrc`, `.gitconfig`, les répertoires `.vscode` et `.idea`, et `hooks` et `config` à l'intérieur de `.git`
* **Les fichiers qui transformeraient votre répertoire de travail en référentiel git nu** : `HEAD`, `objects`, et `refs` au niveau supérieur, plus `config` et `hooks` là-bas quand un `HEAD` les accompagne. Un fichier nommé `config` est refusé même sans `HEAD`. Sur Linux et WSL2, le sandbox supprime un fichier `HEAD` de niveau supérieur ou un répertoire `objects` ou `refs` qui apparaît pendant qu'une commande en sandbox s'exécute
* **Dans `~/.claude`, ou le répertoire vers lequel `CLAUDE_CONFIG_DIR` pointe** : la plupart de son contenu, plus `~/.claude.json` et le magasin d'identifiants `.credentials.json`

Si un lien symbolique apparaît au chemin d'un fichier de paramètres protégés pendant la session, le sandbox refuse également les écritures dans le fichier vers lequel il pointe, à partir de la commande suivante.

Il n'y a aucun moyen d'exempter l'un de ces chemins : une entrée `allowWrite` ou une règle d'autorisation `Edit` qui couvre le chemin ne lève pas la protection. La seule façon de désactiver la protection est [`filesystem.disabled`](#disable-filesystem-isolation), qui désactive l'isolation du système de fichiers pour chaque chemin. Pour voir la plupart de ces chemins résolus pour votre machine, exécutez `/sandbox` et ouvrez l'onglet **Config**, qui les répertorie sous **Denied within allowed**, mélangés avec vos propres entrées `denyWrite`.

Si `git merge` ou `git checkout` échoue avec `unable to unlink old` sur l'un de ces chemins, consultez [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Isolation réseau
</h3>

L'accès réseau est contrôlé via un serveur proxy s'exécutant en dehors du sandbox :

* **Restrictions de domaine** : Claude Code ne pré-autorise aucun domaine par défaut. La première fois qu'une commande a besoin d'un nouveau domaine, Claude Code demande une approbation ; en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), Claude nomme plutôt les hôtes qu'une commande nécessite sur la commande elle-même, selon [Domaines autorisés par commande](#per-command-allowed-domains-in-auto-mode).
* **Choix d'approbation** : si vous choisissez Oui quand vous êtes invité, Claude Code autorise l'hôte pour le reste de la session actuelle et ne demande plus pour les connexions ultérieures au même hôte. Si vous choisissez « Oui, et ne me demander plus », Claude Code enregistre une règle d'autorisation `WebFetch(domain:...)` dans vos [paramètres locaux](/docs/fr/permissions#permission-system), afin que l'hôte reste autorisé dans les sessions futures.
* **Domaines pré-autorisés** : pré-autorisez les domaines avec [`allowedDomains`](/docs/fr/settings-reference#sandbox-network-alloweddomains) pour éviter complètement l'invite. Claude Code pré-autorise également les domaines à partir des règles d'autorisation `WebFetch(domain:...)`, comme décrit dans [Règles de permissions](#permission-rules).
* **Liste d'autorisation stricte** : si vous définissez [`strictAllowlist`](/docs/fr/settings-reference#sandbox-network-strictallowlist) à `true` dans les paramètres utilisateur, gérés ou CLI `--settings`, Claude Code refuse aux commandes en sandbox l'accès à tout hôte en dehors de la liste d'autorisation au lieu de demander. La liste d'autorisation est la même contre laquelle le sandbox demande autrement : `allowedDomains` plus les domaines des règles d'autorisation `WebFetch(domain:...)`, ou uniquement les entrées de paramètres gérés quand `allowManagedDomainsOnly` est défini. Claude Code applique cela uniquement aux commandes en sandbox ; les outils en processus tels que `WebFetch` suivent toujours leurs [règles de permissions](#permission-rules). Le définir dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un référentiel n'a aucun effet. Nécessite Claude Code v2.1.219 ou ultérieur.
* **Verrouillage géré** : si [`allowManagedDomainsOnly`](/docs/fr/settings-reference#sandbox-network-allowmanageddomainsonly) est défini dans les paramètres gérés, les domaines non autorisés sont bloqués automatiquement au lieu de demander, et seuls `allowedDomains` et les règles d'autorisation `WebFetch(domain:...)` des paramètres gérés sont honorés.
* **Proxy d'entreprise** : quand votre réseau nécessite que le trafic sortant passe par un proxy d'entreprise, définissez `HTTPS_PROXY`, `HTTP_PROXY`, et `NO_PROXY` comme [configuration de proxy](/docs/fr/network-config#proxy-configuration) le décrit, dans le bloc `env` de vos paramètres afin que les [agents en arrière-plan](/docs/fr/network-config#set-network-variables-in-settings-not-the-shell) les obtiennent aussi, ou dans l'environnement à partir duquel vous lancez Claude Code. Claude Code applique la liste d'autorisation de domaine et tunnelise ensuite les connexions autorisées via ce proxy en amont.
* **Support de proxy personnalisé** : les utilisateurs avancés peuvent implémenter des règles personnalisées sur le trafic sortant
* **Couverture complète** : les restrictions s'appliquent à tous les scripts, programmes et sous-processus générés par les commandes

Dans une règle `WebFetch(domain:...)`, le sandbox honore deux formes de caractères génériques : un `*.` initial, tel que `*.example.com`, et un `*` nu. La forme `*` nu nécessite Claude Code v2.1.186 ou ultérieur. Un caractère générique dans toute autre position, tel que `WebFetch(domain:example.*)`, correspond toujours aux récupérations mais n'a aucun effet sur les commandes en sandbox.

<Note>
  Le proxy intégré applique la liste d'autorisation en fonction du nom d'hôte demandé et, par défaut, ne termine pas ou n'inspecte pas le trafic TLS. Le paramètre expérimental [`network.tlsTerminate`](/docs/fr/settings-reference#sandbox-network-tlsterminate), disponible dans Claude Code v2.1.199 et ultérieur, fait que le proxy intégré termine lui-même TLS, ce que les entrées d'identifiants [`mask`](#mask-credentials) nécessitent. Consultez [Limitations de sécurité](#security-limitations) pour les implications de la conception par défaut, et [Configuration de proxy personnalisée](#custom-proxy-configuration) si votre modèle de menace nécessite l'inspection TLS.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Domaines autorisés par commande en mode auto
</h4>

En [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) avec sandboxing activé, Claude nomme les hôtes qu'une commande nécessite sur la commande elle-même au lieu de déclencher une approbation réseau pour chaque connexion. Chaque commande Bash, PowerShell, ou [Monitor](/docs/fr/tools-reference#monitor-tool) qui s'exécute dans le sandbox peut porter une liste d'hôtes au-delà de la liste d'autorisation du sandbox : un domaine tel que `registry.npmjs.org`, un caractère générique tel que `*.pythonhosted.org`, ou une adresse IP, chacun avec un `:port` optionnel. Le classificateur examine les hôtes ensemble avec la commande. Nécessite Claude Code v2.1.271 ou ultérieur.

Une liste approuvée ouvre ces hôtes pour cette seule commande, aussi longtemps qu'elle s'exécute. Rien n'est ajouté aux hôtes autorisés de votre session ou à vos paramètres ; la commande suivante nomme ses propres hôtes.

Une commande qui porte des hôtes va au classificateur au lieu d'être approuvée par une règle de permission ou le [mode auto-autorisation](#sandbox-modes) du sandbox. Si une [règle ask](/docs/fr/permissions#manage-permissions) force une invite pour la commande, la boîte de dialogue de permission dans votre terminal répertorie les hôtes à côté, et approuver là couvre les deux.

Une liste par commande élargit uniquement ce que le sandbox refuse par défaut. Les entrées [`deniedDomains`](/docs/fr/settings-reference#sandbox-network-denieddomains) bloquent toujours. Quand [`strictAllowlist`](/docs/fr/settings-reference#sandbox-network-strictallowlist) ou [`allowManagedDomainsOnly`](/docs/fr/settings-reference#sandbox-network-allowmanageddomainsonly) verrouille la liste d'autorisation, Claude Code refuse les listes par commande.

Pendant que les listes par commande s'appliquent, Claude Code refuse une connexion à un hôte qu'aucune commande approuvée n'a répertorié, sans invite ou vérification du classificateur. Le refus nomme l'hôte dans le résultat de la commande, et Claude réexécute la commande avec l'hôte ajouté.

<h4 id="ipv6-addresses-in-domain-lists">
  Adresses IPv6 dans les listes de domaines
</h4>

Les listes de domaines du sandbox sont `allowedDomains`, `deniedDomains`, et les règles `WebFetch(domain:...)` qui les alimentent. Pour correspondre à une adresse IPv6 dans l'une d'elles, écrivez le littéral entre crochets : `"[::1]"` correspond à cette adresse sur chaque port, et `"[::1]:443"` la correspond sur le port 443 uniquement. Écrivez le port comme un nombre de 1 à 65535 sans zéros non significatifs. La forme entre crochets nécessite Claude Code v2.1.229 ou ultérieur. Avant v2.1.229, quand le texte après le dernier deux-points d'une entrée non entre crochets était un numéro de port, Claude Code le lisait comme tel, donc `::1:443` nommait l'adresse `::1` sur le port 443.

Quand vous choisissez « Oui, et ne me demander plus » à l'invite d'approbation réseau pour une adresse IPv6, Claude Code enregistre la règle `WebFetch(domain:...)` avec l'adresse entre crochets, afin que la règle continue de correspondre à l'adresse dans les sessions futures.

Une entrée non entre crochets avec deux deux-points ou plus est ambiguë : `::1:443` est à la fois une adresse IPv6 complète et une adresse suivie d'un port. Claude Code applique les orthographes ambiguës de manière conservatrice au lieu de deviner quelle lecture vous aviez l'intention :

* **Listes de refus** : Claude Code refuse chaque lecture que l'entrée analyse, donc quelle que soit la lecture que vous aviez l'intention, elle est bloquée. Pour une entrée sans lecture analysable, Claude Code ne bloque rien.
* **Listes d'autorisation** : Claude Code n'autorise jamais plus que ce que vous avez écrit. Il réécrit une entrée ambiguë à sa lecture hôte-et-port quand cette lecture analyse proprement, et peut supprimer l'entrée entièrement plutôt que d'élargir la liste d'autorisation.

Exécutez `claude doctor` dans votre terminal pour trouver les entrées affectées : l'avertissement `Sandbox network domain entries have unreliable spellings` nomme jusqu'à trois d'entre elles et compte le reste. Réécrivez chacune dans la forme entre crochets pour effacer l'avertissement. L'avertissement nomme également les entrées dont l'orthographe est peu fiable pour d'autres raisons, telles que `@`, les caractères de chemin ou de requête, ou les caractères génériques à l'intérieur des crochets.

<h3 id="os-level-enforcement">
  Application au niveau du système d'exploitation
</h3>

L'outil Bash en sandbox utilise les primitives de sécurité du système d'exploitation :

* **macOS** : utilise Seatbelt pour l'application du sandbox
* **Linux** : utilise [bubblewrap](https://github.com/containers/bubblewrap) pour l'isolation
* **WSL2** : utilise bubblewrap, comme Linux

WSL1 n'est pas supporté car bubblewrap nécessite des fonctionnalités du noyau uniquement disponibles dans WSL2.

Ces mêmes primitives sont disponibles en tant que package autonome [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), que la page [Environnements sandbox](/docs/fr/sandbox-environments#sandbox-runtime) couvre comme une approche distincte pour envelopper l'ensemble du processus Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Comment le sandboxing se rapporte aux permissions et aux modes de permission
</h2>

Le sandboxing, les [règles de permission](/docs/fr/permissions), et les [modes de permission](/docs/fr/permission-modes) sont des couches complémentaires. Les sections ci-dessous couvrent comment le sandbox interagit avec chacun.

<h3 id="permission-rules">
  Règles de permission
</h3>

Les règles de permission et le sandboxing contrôlent des choses différentes :

* **Les règles de permission** contrôlent quels outils Claude Code peut utiliser et sont évaluées avant l'exécution de tout outil. Elles s'appliquent à chaque outil : Bash, Read, Edit, WebFetch, MCP, et autres, sauf qu'une règle de refus ou de demande ne peut pas bloquer [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant qu'un autre outil reste.
* **Le sandboxing** fournit une application au niveau du système d'exploitation qui restreint ce que les commandes shell peuvent accéder au niveau du système de fichiers et du réseau. Il s'applique uniquement aux commandes Bash, PowerShell, et [Monitor](/docs/fr/tools-reference#monitor-tool) et à leurs processus enfants.

Les deux couches diffèrent également dans la façon dont elles sont appliquées. Claude Code évalue les décisions de permission avant l'exécution d'une commande, en fonction de la chaîne de commande et, en mode auto, du jugement d'un classificateur distinct sur la sécurité de la commande. Le système d'exploitation applique la limite du sandbox au processus en cours d'exécution, donc elle tient indépendamment de ce que le modèle a choisi d'exécuter et même si une commande autorisée fait plus que son nom ne le suggère.

Les restrictions du système de fichiers et du réseau sont configurées via les paramètres du sandbox et les règles de permission :

| Paramètre ou règle                                              | Ce qu'il fait                                                                                                             |
| :-------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `sandbox.filesystem.allowWrite`                                 | Accorde l'accès en écriture du sous-processus aux chemins en dehors du répertoire de travail                              |
| `sandbox.filesystem.denyWrite` et `sandbox.filesystem.denyRead` | Bloquent l'accès du sous-processus à des chemins spécifiques                                                              |
| `sandbox.filesystem.allowRead`                                  | Réautorise la lecture de chemins spécifiques dans une région `denyRead`                                                   |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)  | Désactive entièrement la couche du système de fichiers tout en conservant l'isolation du réseau                           |
| Règles d'autorisation `Edit`                                    | Accordent l'accès en écriture à des chemins spécifiques, de la même manière que `sandbox.filesystem.allowWrite`           |
| Règles de refus `Read` et `Edit`                                | Bloquent l'accès à des fichiers ou répertoires spécifiques                                                                |
| Règles d'autorisation et de refus `WebFetch(domain:...)`        | Contrôlent l'accès au domaine                                                                                             |
| `allowedDomains` du sandbox                                     | Contrôle les domaines que les commandes Bash peuvent atteindre                                                            |
| `deniedDomains` du sandbox                                      | Bloque les domaines spécifiques même lorsqu'un caractère générique `allowedDomains` plus large les autoriserait autrement |

Les chemins et domaines des paramètres du sandbox et des règles de permission sont fusionnés dans la configuration finale du sandbox.

Le [répertoire d'exemples du référentiel claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) inclut des configurations de paramètres de démarrage pour les scénarios de déploiement courants, y compris des exemples spécifiques au sandbox. Utilisez-les comme points de départ et ajustez-les selon vos besoins.

<h3 id="permission-modes">
  Modes de permission
</h3>

`/sandbox` n'est pas un [mode de permission](/docs/fr/permission-modes). Les modes de permission décident si un appel d'outil s'exécute et si vous êtes d'abord invité, tandis que le sandbox restreint ce qu'une commande Bash peut accéder une fois qu'elle s'exécute. Ils diffèrent dans ce qu'ils contrôlent et ce qui remplace l'invite par action :

|                                                                    | Ce qu'il contrôle                                               | Ce qui remplace l'invite                                                                                                                                                                                                          |
| :----------------------------------------------------------------- | :-------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                         | Ce qu'une commande Bash peut accéder une fois qu'elle s'exécute | La limite du sandbox elle-même, en [mode auto-allow](#sandbox-modes)                                                                                                                                                              |
| [Mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) | Si chaque appel d'outil s'exécute                               | Un classificateur qui examine les actions                                                                                                                                                                                         |
| `--dangerously-skip-permissions`                                   | Si chaque appel d'outil s'exécute                               | Rien. Les vérifications de [chemin protégé](/docs/fr/permission-modes#protected-paths) sont également ignorées ; les [actions qu'aucun mode n'auto-approuve](/docs/fr/permission-modes#actions-no-mode-auto-approves) s'appliquent toujours |

Le [mode auto-allow](#sandbox-modes) du sandbox est séparé du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : auto-allow approuve les commandes Bash parce que la limite du sandbox les contient, tandis que le mode auto utilise un classificateur pour examiner les actions. Les deux fonctionnent indépendamment et peuvent être combinés, avec les exceptions énumérées sous [Modes du sandbox](#sandbox-modes). Pour choisir une limite d'isolation pour les exécutions sans surveillance, voir [Environnements du sandbox](/docs/fr/sandbox-environments#how-isolation-relates-to-permission-modes). Pour un tableau des appairages courants de mode de permission et de sandbox avec les drapeaux qui démarrent chacun, voir [Configurations courantes](/docs/fr/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Configurer le sandbox pour votre organisation
</h2>

Les administrateurs peuvent exiger le sandboxing pour chaque utilisateur, empêcher les développeurs d'élargir la politique et acheminer le trafic sandbox via un proxy d'entreprise.

<h3 id="enforce-sandboxing-with-managed-settings">
  Appliquer le sandboxing avec les paramètres gérés
</h3>

Pour exiger le sandbox pour chaque développeur, livrez les clés `sandbox` via [paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), soit en tant que fichier géré par votre MDM, soit via [paramètres gérés par serveur](/docs/fr/server-managed-settings) sur claude.ai.

La configuration de paramètres gérés suivante active le sandbox, refuse de démarrer Claude Code si le sandbox ne peut pas s'initialiser et empêche le modèle de réessayer les commandes en dehors du sandbox :

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

Les deux clés au-delà de `enabled` contrôlent ce qui se passe lorsque le sandbox ne peut pas exécuter une commande :

* **`failIfUnavailable`** : une dépendance manquante telle que bubblewrap sur Linux empêche Claude Code de démarrer plutôt que d'afficher un avertissement et de revenir à une exécution non sandboxée
* **`allowUnsandboxedCommands: false`** : Claude Code ignore la trappe d'échappement `dangerouslyDisableSandbox`, donc lorsqu'une commande échoue sous le sandbox, Claude ne peut pas la réessayer en dehors du sandbox

Deux ajouts valent la peine d'être envisagés aux côtés d'eux. Ajoutez `excludedCommands` pour tous les outils approuvés par l'organisation qui doivent s'exécuter sans isolation. Ajoutez des entrées [`sandbox.credentials`](#protect-credentials) pour les répertoires d'identifiants tels que `~/.aws` et `~/.ssh` et pour les variables d'environnement secrètes, car la politique de lecture par défaut les autorise toujours.

Cette configuration sandboxe les commandes que Claude exécute. Un développeur peut toujours taper une commande à l'[invite shell-mode `!`](/docs/fr/interactive-mode#shell-mode-with-prefix) et l'exécuter en dehors du sandbox, avec le même accès qu'il a déjà dans n'importe quel terminal en dehors de Claude Code. Consultez [La trappe d'échappement de réessai non sandboxé](#the-unsandboxed-retry-escape-hatch) pour les sessions où les commandes tapées s'exécutent en sandboxé.

Le sandbox ne s'exécute pas sur Windows natif, donc si votre flotte inclut des hôtes Windows, limitez cette configuration à macOS et Linux ou demandez à ces utilisateurs d'exécuter Claude Code à l'intérieur de WSL2 ou d'un conteneur.

<h3 id="keep-developers-from-widening-the-policy">
  Empêcher les développeurs d'élargir la politique
</h3>

Pour les clés booléennes telles que `enabled` et `failIfUnavailable`, Claude Code utilise la valeur gérée et ignore tout ce qu'un développeur définit localement. Pour les clés de tableau telles que `excludedCommands` et `allowRead`, Claude Code fusionne les entrées de chaque portée que la session charge, un développeur peut donc ajouter des entrées qui élargissent la politique.

Définissez `allowManagedReadPathsOnly` sur `true` dans les paramètres gérés pour que seules les entrées `allowRead` des paramètres gérés soient honorées. Cela empêche les développeurs d'élargir l'accès en lecture au-delà des chemins approuvés par l'organisation. Pour verrouiller les domaines réseau aux valeurs gérées de la même manière, définissez [`allowManagedDomainsOnly`](/docs/fr/settings-reference#sandbox-network-allowmanageddomainsonly).

Lorsque les paramètres gérés configurent `sandbox.filesystem` ou listent une entrée `sandbox.credentials.files` avec `"mode": "deny"`, seuls les paramètres gérés peuvent définir [`filesystem.disabled`](#disable-filesystem-isolation), les développeurs ne peuvent donc pas désactiver les restrictions de filesystem déployées par l'administrateur. Le fait qu'une entrée `mask` épingle la clé dépend de la façon dont elle se résout ; le tableau sous [Quels paramètres peuvent le désactiver](#which-settings-can-disable-it) couvre les quatre cas.

`excludedCommands` n'a pas d'équivalent de verrouillage géré uniquement, un développeur peut donc toujours ajouter des entrées qui exécutent des commandes supplémentaires en dehors du sandbox. Gardez la liste gérée étroite.

<h3 id="custom-proxy-configuration">
  Configuration de proxy personnalisée
</h3>

Pour les organisations nécessitant une sécurité réseau avancée, vous pouvez implémenter un proxy personnalisé pour :

* Déchiffrer et inspecter le trafic HTTPS
* Appliquer des règles de filtrage personnalisées
* Enregistrer toutes les demandes réseau
* Intégrer avec l'infrastructure de sécurité existante

Pour pointer Claude Code vers votre proxy, définissez les ports proxy dans [paramètres de sandbox](/docs/fr/settings-reference#sandbox-settings) :

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Dépannage
</h2>

Certaines commandes échouent à l'intérieur du sandbox même si elles fonctionnent en dehors. Les correctifs ci-dessous couvrent les cas les plus courants.

* **Les commandes échouent avec une erreur host-not-allowed** : de nombreux outils CLI doivent atteindre des hôtes spécifiques. Accorder la permission lorsque vous y êtes invité ajoute l'hôte à votre liste autorisée pour que l'outil s'exécute à l'intérieur du sandbox à l'avenir.
* **`jest` se bloque ou échoue** : `watchman` est incompatible avec le sandbox. Exécutez `jest --no-watchman` à la place.
* **Les CLI basés sur Go échouent la vérification TLS sur macOS** : les outils tels que `gh`, `gcloud` et `terraform` peuvent échouer la vérification TLS sous Seatbelt. Listez ces outils dans [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands). Si vous utilisez `httpProxyPort` avec un proxy MITM et une CA personnalisée, définissez [`enableWeakerNetworkIsolation`](/docs/fr/settings-reference#sandbox-enableweakernetworkisolation) sur `true` à la place.
* **Les commandes `open`, `osascript` ou les flux d'authentification basés sur un navigateur échouent avec l'erreur `-600` sur macOS** : le sandbox bloque les Apple Events par défaut. Définissez [`allowAppleEvents`](/docs/fr/settings-reference#sandbox-allowappleevents) sur `true` dans vos paramètres utilisateur, gérés ou CLI pour les autoriser. Les paramètres du projet sont ignorés pour cette clé. L'activation supprime l'isolation de l'exécution du code, car les commandes sandboxées peuvent alors lancer d'autres applications non sandboxées sans invite utilisateur et envoyer des commandes AppleScript aux applications en cours d'exécution, sous réserve de l'invite de consentement à l'automatisation macOS (TCC). Vous pouvez également ajouter la commande à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands).
* **Les commandes `docker` échouent** : `docker` est incompatible avec le sandbox. Ajoutez `docker *` à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands).
* **`pbcopy`, `xclip` ou `wl-copy` ne met pas à jour le presse-papiers** : ces utilitaires de presse-papiers peuvent échouer à atteindre le presse-papiers système de l'intérieur du sandbox, auquel cas le texte qui leur est envoyé par pipe n'arrive pas.

  Pour mettre la sortie de Claude sur votre presse-papiers, demandez à Claude de l'imprimer dans sa réponse, puis exécutez [`/copy`](/docs/fr/commands). `/copy` écrit dans le presse-papiers à partir du processus Claude Code plutôt qu'à partir d'une commande sandboxée.

  Lorsque Claude envoie du texte par pipe à l'un de ces outils, ajouter l'outil à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands) ne retire pas cet appel du sandbox en soi.
* **Une commande git échoue avec `unable to unlink old`** : `git merge`, `git checkout` et les commandes similaires échouent de cette façon lorsqu'elles doivent remplacer un fichier auquel le sandbox refuse les écritures, que ce fichier soit sous un [chemin protégé](#protected-paths) tel que `.claude/skills`, sous l'une de vos entrées `denyWrite` ou en dehors des répertoires dans lesquels le sandbox permet aux commandes d'écrire. Sur Linux et WSL2, l'erreur se termine par `Read-only file system`.

  Après l'échec, Claude peut [proposer de réexécuter la commande en dehors du sandbox](#the-unsandboxed-retry-escape-hatch) ; approuvez cette nouvelle tentative ou exécutez la commande git vous-même dans un autre terminal. Si vous avez défini `allowUnsandboxedCommands` sur `false`, Claude ne peut pas proposer la nouvelle tentative, alors exécutez la commande vous-même. Si la même commande git échoue souvent, ajoutez-la à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands).
* **Bubblewrap échoue à démarrer à l'intérieur d'un conteneur** : dans un conteneur sans privilèges, bubblewrap ne peut pas monter un système de fichiers `/proc` frais, donc les commandes sandboxées échouent avec une erreur `bwrap` telle que `Can't mount proc on /newroot/proc: Operation not permitted`. Définissez [`enableWeakerNestedSandbox`](/docs/fr/settings-reference#sandbox-enableweakernestedsandbox) sur `true` pour que le sandbox interne bind-monte le `/proc` existant du conteneur à la place. Utilisez ce paramètre uniquement lorsque le conteneur externe fournit déjà la limite d'isolation dont vous avez besoin, car il expose les informations de processus aux commandes sandboxées qu'un montage `/proc` frais cacherait.
* **Les fichiers en lecture seule de 0 octet apparaissent aux chemins des paramètres `.claude`, et « Oui, et ne pas demander à nouveau » ne sauvegarde pas** : sur Linux et WSL2, le sandbox maintient un refus d'écriture sur un fichier qui n'existe pas encore en créant un espace réservé en lecture seule de 0 octet là-bas pendant qu'une commande sandboxée s'exécute. Le sandbox supprime l'espace réservé après. Si une session est tuée avant que ce nettoyage ne s'exécute, par exemple par SIGKILL, les espaces réservés restent. Les sessions ultérieures les lient en lecture seule à nouveau à chaque démarrage, donc une écriture de paramètres telle que l'enregistrement d'un choix de permission échoue là où l'un se trouve.

  Exécutez `claude doctor` pour lister les fichiers d'espace réservé restants. L'avertissement [`Stale sandbox mask files left by a killed session`](/docs/fr/errors#stale-sandbox-mask-files-left-by-a-killed-session) nomme jusqu'à trois d'entre eux et compte le reste. Supprimez chaque fichier avec `rm` tandis qu'aucune autre session Claude Code ne s'exécute dans ce projet. Avant v2.1.257, Claude Code laissait les mêmes espaces réservés derrière sans les signaler.
* **`--dangerously-skip-permissions` échoue en tant que root** : cet indicateur est bloqué lors de l'exécution en tant que root ou via sudo sur Linux et macOS, car l'accès root combiné à aucune invite de permission peut modifier n'importe quel fichier ou service sur le système. La vérification est ignorée automatiquement à l'intérieur d'un sandbox reconnu. Pour exécuter de manière autonome dans un conteneur, utilisez la configuration [dev container](/docs/fr/devcontainer), qui exécute Claude Code en tant qu'utilisateur non-root.

<h2 id="limitations">
  Limitations
</h2>

Le sandboxing réduit le risque mais n'est pas une limite d'isolation complète. Examinez les limitations ci-dessous avant de vous y fier comme contrôle de sécurité dur.

<h3 id="security-limitations">
  Limitations de sécurité
</h3>

* **Filtrage réseau** : le sandbox restreint les domaines auxquels les processus peuvent se connecter. Par défaut, le proxy intégré ne termine pas ou n'inspecte pas TLS sur le trafic sortant, le contenu des connexions chiffrées n'est donc pas examiné. Le paramètre expérimental [`network.tlsTerminate`](/docs/fr/settings-reference#sandbox-network-tlsterminate) termine TLS au proxy pour la [substitution de credentials `mask`](#mask-credentials) mais n'ajoute pas de filtrage de contenu. Vous êtes responsable de vous assurer que seuls les domaines de confiance sont autorisés dans votre politique.

<Warning>
  Autoriser des domaines larges tels que `github.com` peut créer des chemins pour l'exfiltration de données. Parce que le proxy prend sa décision d'autorisation à partir du nom d'hôte fourni par le client sans inspecter TLS, le code s'exécutant à l'intérieur du sandbox peut potentiellement utiliser [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) ou des techniques similaires pour atteindre des hôtes en dehors de la liste d'autorisation. Si votre modèle de menace nécessite des garanties plus fortes, configurez un [proxy personnalisé](#custom-proxy-configuration) qui termine TLS et inspecte le trafic, et installez son certificat CA à l'intérieur du sandbox. L'isolation réseau plus forte consciente de TLS est un domaine actif de développement.
</Warning>

* **Escalade de privilèges via les sockets de domaine Unix** : la configuration `allowUnixSockets` peut accorder involontairement l'accès à des services système qui pourraient entraîner des contournements du sandbox. Par exemple, autoriser l'accès à `/var/run/docker.sock` accorde effectivement l'accès au système hôte via le socket Docker. Considérez attentivement tous les sockets Unix que vous autorisez via le sandbox.
* **Escalade de permissions du système de fichiers** : les permissions d'écriture du système de fichiers trop larges peuvent permettre des attaques d'escalade de privilèges. Autoriser les écritures dans les répertoires contenant des exécutables dans `$PATH`, les répertoires de configuration système ou les fichiers de configuration shell utilisateur tels que `.bashrc` ou `.zshrc` peut entraîner l'exécution de code dans différents contextes de sécurité lorsque d'autres utilisateurs ou processus système accèdent à ces fichiers.
* **Force du sandbox Linux** : l'implémentation Linux fournit une isolation forte du système de fichiers et du réseau mais inclut un mode `enableWeakerNestedSandbox` qui lui permet de fonctionner à l'intérieur des environnements Docker sans espaces de noms privilégiés, ou sur les hôtes Linux où les espaces de noms utilisateur sans privilèges sont désactivés par sysctl. Cette option affaiblit considérablement la sécurité et ne doit être utilisée que lorsqu'une isolation supplémentaire est autrement appliquée.
* **Apple Events sur macOS** : le sandbox macOS bloque les Apple Events par défaut. Le paramètre `allowAppleEvents` lève cette restriction afin que les outils tels que `open` et `osascript` fonctionnent, mais il supprime l'isolation de l'exécution du code : les commandes sandboxées peuvent lancer d'autres applications sans sandbox sans invite utilisateur, et peuvent envoyer des commandes AppleScript aux applications en cours d'exécution, sous réserve de l'invite de consentement à l'automatisation macOS par application (TCC). Il n'est honoré que par les paramètres utilisateur, gérés ou CLI. Les paramètres de projet ne peuvent pas l'activer.

<h3 id="platform-and-tool-compatibility">
  Compatibilité de plateforme et d'outil
</h3>

* **Support de plateforme** : supporte macOS, Linux et WSL2. WSL1 et Windows natif ne sont pas supportés.
* **Surcharge de performance** : minimale, mais certaines opérations du système de fichiers peuvent être légèrement plus lentes.
* **Compatibilité d'outil** : certains outils qui nécessitent des modèles d'accès système spécifiques peuvent nécessiter des ajustements de configuration, ou peuvent avoir besoin d'être exécutés en dehors du sandbox.

<h3 id="scope">
  Portée
</h3>

Le sandbox isole les sous-processus Bash. Les autres outils fonctionnent sous des limites différentes :

* **Outils de fichiers intégrés** : Read, Edit et Write utilisent le système de permissions directement plutôt que de s'exécuter via le sandbox. Consultez [permissions](/docs/fr/permissions).
* **Utilisation de l'ordinateur** : lorsque Claude ouvre des applications et contrôle votre écran, il s'exécute sur votre bureau réel plutôt que dans un environnement isolé. Les invites de permission par application contrôlent chaque application. Consultez [utilisation de l'ordinateur dans la CLI](/docs/fr/computer-use) ou [utilisation de l'ordinateur dans Desktop](/docs/fr/desktop#let-claude-use-your-computer).
* **Variables d'environnement** : les commandes Bash sandboxées héritent de l'environnement du processus parent par défaut, y compris les identifiants définis là-bas. Utilisez [`sandbox.credentials`](#protect-credentials) pour supprimer ou masquer les variables spécifiques pour les commandes sandboxées, ou définissez [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/fr/env-vars) pour supprimer les identifiants de tous les sous-processus.
* **Sous-agents** : les [sous-agents](/docs/fr/sub-agents) s'exécutent dans le même processus que la session parent et utilisent la même configuration de sandbox. Les commandes Bash à l'intérieur d'un sous-agent sont sandboxées lorsque le sandboxing est activé dans la session parent.

<Warning>
  Un sandboxing efficace nécessite à la fois l'isolation du système de fichiers et du réseau. Sans isolation réseau, un agent compromis pourrait exfiltrer des fichiers sensibles comme les clés SSH. Sans isolation du système de fichiers, qu'elle provienne d'une politique permissive ou de la [désactivation de la couche système de fichiers](#disable-filesystem-isolation), un agent compromis pourrait installer une porte dérobée sur les ressources système pour accéder au réseau. Lorsque vous élargissez les valeurs par défaut, vérifiez qu'un chemin `allowWrite`, une entrée `allowedDomains` large ou une exception `excludedCommands` ne défait pas une restriction de l'autre côté.
</Warning>

<h2 id="see-also">
  Voir aussi
</h2>

* [Environnements sandbox](/docs/fr/sandbox-environments) : comparez le sandbox intégré avec les dev containers, les conteneurs et les machines virtuelles
* [Sécurité](/docs/fr/security) : fonctionnalités de sécurité complètes et meilleures pratiques
* [Permissions](/docs/fr/permissions) : configuration des permissions et contrôle d'accès
* [Tous les paramètres](/docs/fr/settings-reference) : chaque clé de paramètres
* [Référence CLI](/docs/fr/cli-reference) : options de ligne de commande
