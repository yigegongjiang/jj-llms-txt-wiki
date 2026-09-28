> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop sur Linux (bêta)

> Installez et mettez à jour l'application de bureau Claude sur Ubuntu et Debian

<Note>
  Le support Linux pour l'application de bureau Claude est en bêta.
</Note>

L'application de bureau sur Linux vous offre la même expérience Chat, Cowork et Claude Code que sur macOS et Windows : sessions parallèles, examen des différences visuelles, un terminal et un éditeur intégrés, et un aperçu en direct de l'application. Consultez [Utiliser Claude Code Desktop](/docs/fr/desktop) pour la référence des fonctionnalités.

<h2 id="requirements">
  Configuration requise
</h2>

* Une distribution basée sur Debian : Ubuntu 22.04 ou version ultérieure, ou Debian 12 ou version ultérieure
* x86\_64 ou arm64

Les autres distributions basées sur Debian qui répondent à ces exigences peuvent fonctionner mais ne sont pas officiellement testées. Sur les distributions qui ne sont pas basées sur Debian, comme Fedora ou Arch, exécutez plutôt l'[interface de ligne de commande](/docs/fr/setup#system-requirements). Si vous travaillez sur Windows avec WSL 2, installez l'application de bureau Windows et exécutez les sessions dans votre distribution ; consultez [Claude Code Desktop dans WSL](/docs/fr/desktop-wsl).

<h3 id="cowork-requirements">
  Exigences de Cowork
</h3>

Cowork est l'onglet de bureau pour [Dispatch et les travaux agentiques plus longs](https://claude.com/docs/cowork/overview). Sur Linux, Cowork exécute ces tâches dans une machine virtuelle que l'application de bureau héberge avec QEMU et KVM. Pour utiliser Cowork, votre machine a besoin de :

* **Virtualisation matérielle** : activée dans les paramètres de votre firmware. Sans cela, l'onglet Cowork signale « Cowork requires hardware virtualization (KVM) ».
* **Firmware QEMU et UEFI** : `qemu-system-x86`, `ovmf` et `virtiofsd` sur x86\_64, ou `qemu-system-arm`, `qemu-efi-aarch64` et `virtiofsd` sur arm64. `apt install claude-desktop` les installe par défaut en tant que paquets recommandés. Si vous avez installé avec `--no-install-recommends`, ou si votre système est une image minimale qui ignore les paquets recommandés, l'onglet Cowork signale « Cowork requires QEMU » et affiche la commande `apt install` à exécuter. Ubuntu 22.04 n'a pas de paquet `virtiofsd` ; l'application utilise une copie fournie à cet endroit.
* **Accès à `/dev/kvm`** : ajoutez votre utilisateur au groupe `kvm` avec `sudo usermod -aG kvm $USER`, puis déconnectez-vous et reconnectez-vous. Certains environnements de bureau accordent à l'utilisateur connecté l'accès à `/dev/kvm` sans le groupe, mais Cowork a également besoin de `/dev/vhost-vsock`, que seuls les membres du groupe `kvm` peuvent ouvrir. Rejoignez le groupe même si `/dev/kvm` fonctionne déjà pour vous.

L'application vérifie ces exigences une fois au lancement : redémarrez-la après l'installation des paquets, et déconnectez-vous et reconnectez-vous après avoir rejoint le groupe. Si `/dev/vhost-vsock` est manquant et que votre noyau en cours d'exécution n'a pas de répertoire de module sous `/lib/modules`, l'onglet Cowork signale que le noyau n'inclut pas le support de virtualisation dont Cowork a besoin et qu'il ne peut pas être ajouté manuellement. Cette combinaison est courante sur ChromeOS et dans les environnements Linux basés sur des conteneurs.

<h2 id="install">
  Installation
</h2>

Installez à partir du référentiel apt d'Anthropic afin que les mises à jour arrivent via les mises à jour régulières des paquets de votre système. Ouvrez un terminal et exécutez les commandes de chaque étape.

<Steps>
  <Step title="Ajouter le référentiel apt d'Anthropic">
    Cette étape télécharge la clé de signature avec `curl` et la vérifie avec `gpg`, que les installations fraîches de Debian et Ubuntu peuvent ne pas inclure. Si l'une ou l'autre commande signale `command not found`, installez d'abord les deux :

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Téléchargez la clé de signature d'Anthropic :

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    La commande n'affiche rien en cas de succès et une erreur `curl:` en cas d'échec. Une clé manquante ou incorrecte fait échouer `apt update` plus tard avec `NO_PUBKEY BAA929FF1A7ECACE`, donc confirmez que la clé a été téléchargée et appartient à Anthropic avant de continuer :

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    L'empreinte digitale que gpg affiche doit être `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Si gpg signale que le fichier ne peut pas être ouvert ou ne contient pas de données OpenPGP valides, le téléchargement a échoué ou a renvoyé le mauvais contenu : confirmez que votre réseau peut atteindre `downloads.claude.ai`, puis réexécutez la commande de téléchargement.

    Enregistrez le référentiel :

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Installer le paquet">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Lancer et se connecter">
    Lancez **Claude** à partir de votre lanceur d'applications, ou exécutez `claude-desktop` à partir d'un terminal, et connectez-vous avec votre compte Anthropic.

    L'application Linux se connecte de la même manière que sur macOS et Windows : avec un abonnement claude.ai, ou via l'authentification unique de votre organisation. Desktop n'accepte pas directement une clé API Claude Console ; utilisez l'[interface de ligne de commande](/docs/fr/quickstart) pour l'authentification par clé API. Pour les déploiements d'entreprise qui acheminent Desktop vers la plateforme Agent de Google Cloud ou une passerelle LLM, consultez [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) et la [configuration réseau](/docs/fr/network-config).
  </Step>
</Steps>

<h3 id="install-from-a-downloaded-file">
  Installer à partir d'un fichier téléchargé
</h3>

Si vous ne pouvez pas installer via le référentiel apt, téléchargez le paquet `.deb` directement à partir du pool de paquets du référentiel. Cette commande recherche le paquet le plus récent pour votre architecture dans l'index du référentiel, puis le télécharge dans le répertoire courant :

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Si la commande échoue avec `Remote file name has no length`, la recherche n'a renvoyé aucun chemin de paquet. Cela peut signifier que l'index du référentiel n'a pas pu être récupéré, par exemple lorsque votre réseau bloque `downloads.claude.ai`, ou qu'aucun paquet n'existe pour votre architecture. Confirmez que votre réseau peut atteindre `downloads.claude.ai` et que `dpkg --print-architecture` affiche `amd64` ou `arm64` ; le référentiel ne publie pas de paquets pour d'autres architectures.

Pour installer sans enregistrer le référentiel apt d'Anthropic, créez d'abord `/etc/default/claude-desktop` avec la ligne `CLAUDE_DESKTOP_ADD_REPO="false"`. Sans le référentiel, apt ne livre pas les nouvelles versions ; pour mettre à jour, réexécutez la commande de téléchargement et réinstallez, ou [enregistrez le référentiel](#install) plus tard.

Ensuite, ouvrez le fichier téléchargé avec votre installateur de logiciels, tel que GNOME Software, ou installez-le avec apt à partir du répertoire qui contient le fichier téléchargé :

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Si apt signale `E: Unsupported file ./claude-desktop_*.deb given on commandline`, le motif ne correspondait pas à un fichier `.deb` dans le répertoire courant. Confirmez que le téléchargement s'est terminé, puis exécutez à nouveau la commande à partir du répertoire qui contient le fichier.

L'installation du `.deb` enregistre également le référentiel apt d'Anthropic à `/etc/apt/sources.list.d/claude-desktop.list`, donc les futures mises à jour arrivent avec les [mises à jour régulières des paquets](#update) de votre système.

<h2 id="update">
  Mise à jour
</h2>

L'application de bureau ne se met pas à jour elle-même sur Linux. Les mises à jour arrivent avec les mises à jour régulières des paquets de votre système :

```bash theme={null}
sudo apt update && sudo apt upgrade
```

Le gestionnaire de logiciels graphique de votre distribution détectera également les nouvelles versions.

<h2 id="uninstall">
  Désinstallation
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

La désinstallation du paquet supprime également l'entrée du référentiel et la clé de signature qu'il a enregistrées. Si vous avez ajouté l'entrée du référentiel vous-même lors de l'étape [Ajouter le référentiel apt d'Anthropic](#install), supprimez-la également :

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Dépannage
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Impossible de localiser le paquet claude-desktop
</h3>

Si `sudo apt install claude-desktop` échoue avec `E: Unable to locate package claude-desktop`, apt n'a pas trouvé le référentiel que vous avez ajouté. Vérifiez les points suivants :

* Exécutez `sudo apt update` après avoir ajouté le référentiel. `apt install` seul ne voit pas un référentiel que vous avez ajouté après la dernière fois que vous avez exécuté `apt update`.
* Confirmez que l'entrée du référentiel a été écrite. `cat /etc/apt/sources.list.d/claude-desktop.list` devrait afficher la ligne `deb` de l'étape [Ajouter le référentiel apt d'Anthropic](#install). Si le fichier est vide ou manquant, exécutez cette étape à nouveau.
* Confirmez que votre architecture est prise en charge. `dpkg --print-architecture` devrait afficher `amd64` ou `arm64`. Le référentiel ne publie pas de paquets pour d'autres architectures.
* Exécutez `sudo apt update` à nouveau et vérifiez sa sortie pour les erreurs liées à `downloads.claude.ai`. Une erreur de réseau ou de clé à cet endroit signifie que le référentiel a été ajouté mais n'a pas pu être atteint ou vérifié.

Si le référentiel est en place et accessible et que le paquet n'est toujours pas trouvé, [installez à partir d'un fichier téléchargé](#install-from-a-downloaded-file) à la place.

<h3 id="unmet-dependencies">
  Dépendances non satisfaites
</h3>

Si `apt` s'arrête avec `The following packages have unmet dependencies` ou `Unsatisfied dependencies`, lisez la dépendance qu'il nomme :

* `libc6 (>= 2.34)` : votre distribution est plus ancienne que celle prise en charge par le paquet. Ubuntu 20.04 est livré avec `libc6` 2.31. Mettez à niveau vers Ubuntu 22.04 ou ultérieur, ou Debian 12 ou ultérieur.
* Toutes les dépendances manquantes affichent `not installable` avec un suffixe `:amd64` ou `:arm64` : vous avez téléchargé le `.deb` pour une architecture différente de celle de votre machine. Exécutez `dpkg --print-architecture` et téléchargez le `.deb` correspondant, ou [installez à partir du référentiel apt](#install), qui sélectionne le paquet pour votre architecture.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  L'exécution en tant que root sans --no-sandbox n'est pas prise en charge
</h3>

Si `claude-desktop` se ferme avec ce message, vous l'avez lancé en tant que root. Connectez-vous en tant qu'utilisateur régulier et lancez-le à partir de là.

<h3 id="cowork-isn’t-available">
  Cowork n'est pas disponible
</h3>

Si l'onglet Cowork affiche l'un de ces messages, corrigez l'exigence qu'il nomme, puis redémarrez l'application :

* **Cowork nécessite QEMU** : installez les [paquets QEMU et firmware UEFI](#cowork-requirements) que le message énumère.
* **Cowork nécessite la virtualisation matérielle (KVM)** : activez la [virtualisation matérielle](#cowork-requirements) dans les paramètres de votre firmware.
* **Claude n'a pas la permission d'utiliser la virtualisation (/dev/kvm)** : ajoutez votre utilisateur au [groupe `kvm`](#cowork-requirements), puis déconnectez-vous et reconnectez-vous.
* **Cowork nécessite le module noyau `vhost_vsock`** : exécutez `sudo modprobe vhost_vsock`, puis redémarrez l'application. Cela charge le module pour le démarrage actuel uniquement. Pour le charger à chaque démarrage, exécutez `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Ce qui n'est pas encore dans la bêta Linux
</h2>

* **Computer Use** : [le contrôle des applications et de l'écran](/docs/fr/desktop#let-claude-use-your-computer) n'est pas disponible sur Linux.
* **Dictation** : l'entrée vocale n'est pas disponible dans l'application de bureau Linux. Utilisez plutôt la [dictation vocale](/docs/fr/voice-dictation) dans l'interface de ligne de commande.
* **Raccourci global Quick Entry** : fonctionne sur X11. Sur Wayland natif, cela nécessite le portail GlobalShortcuts de votre environnement de bureau.
* **Fedora et RHEL** : seules les distributions basées sur Debian sont prises en charge aujourd'hui. Le support pour des distributions supplémentaires arrivera à l'avenir.

Pour tout ce qui n'est pas encore disponible dans l'application de bureau, l'[interface de ligne de commande](/docs/fr/quickstart) exécute le même moteur Claude Code et prend en charge une gamme plus large de distributions Linux ; consultez la [configuration requise du système](/docs/fr/setup#system-requirements).
