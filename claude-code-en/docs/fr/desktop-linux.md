> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop sur Linux (bêta)

> Installez et mettez à jour l'application de bureau Claude sur Ubuntu et Debian

<Note>
  Le support Linux pour l'application de bureau Claude est en bêta. Les onglets Chat, Cowork et Code sont tous disponibles.
</Note>

L'application de bureau sur Linux vous offre la même expérience Chat, Cowork et Claude Code que macOS et Windows : sessions parallèles, examen des différences visuelles, un terminal et un éditeur intégrés, et un aperçu en direct de l'application. Consultez [Utiliser Claude Code Desktop](/docs/fr/desktop) pour la référence complète des fonctionnalités.

<h2 id="requirements">
  Configuration requise
</h2>

* Ubuntu 22.04 ou version ultérieure, ou Debian 12 ou version ultérieure
* x86\_64 ou arm64

Les autres distributions basées sur Debian qui répondent à ces exigences peuvent fonctionner mais ne sont pas officiellement testées.

<h2 id="install">
  Installation
</h2>

Installez à partir du référentiel apt d'Anthropic afin que les mises à jour arrivent via les mises à jour régulières des paquets de votre système. Ouvrez un terminal et exécutez les commandes de chaque étape.

<Steps>
  <Step title="Ajouter le référentiel apt d'Anthropic">
    Cette étape télécharge la clé de signature avec `curl`, que les installations fraîches de Debian et Ubuntu peuvent ne pas inclure. Si la commande de téléchargement échoue avec `sudo: curl: command not found`, installez d'abord curl :

    ```bash theme={null}
    sudo apt install curl
    ```

    Téléchargez la clé de signature d'Anthropic :

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

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

<Accordion title="Vérifier la clé de signature">
  Vous pouvez confirmer que la clé de signature téléchargée appartient à Anthropic :

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  L'empreinte digitale doit être `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Installer à partir d'un fichier téléchargé
</h3>

Si vous ne pouvez pas installer via le référentiel apt, téléchargez le paquet `.deb` directement à partir du pool de paquets du référentiel. Cette commande recherche le paquet le plus récent pour votre architecture dans l'index du référentiel, puis le télécharge dans le répertoire courant :

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Si la commande échoue avec `Remote file name has no length`, la recherche n'a renvoyé aucun chemin de paquet. Cela peut signifier que l'index du référentiel n'a pas pu être récupéré, par exemple lorsque votre réseau bloque `downloads.claude.ai`, ou qu'aucun paquet n'existe pour votre architecture. Confirmez que votre réseau peut atteindre `downloads.claude.ai` et que `dpkg --print-architecture` affiche `amd64` ou `arm64` ; le référentiel ne publie pas de paquets pour d'autres architectures.

Ensuite, ouvrez le fichier téléchargé avec votre installateur de logiciels, tel que GNOME Software, ou installez-le avec apt à partir du répertoire qui contient le fichier téléchargé :

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Si apt signale `E: Unsupported file ./claude-desktop_*.deb given on commandline`, le motif ne correspondait pas à un fichier `.deb` dans le répertoire courant. Confirmez que le téléchargement s'est terminé, puis exécutez à nouveau la commande à partir du répertoire qui contient le fichier.

Un `.deb` installé de cette manière ne reçoit pas de mises à jour. Pour obtenir les mises à jour via apt, enregistrez le référentiel à partir de l'étape [Ajouter le référentiel apt d'Anthropic](#install). Le paquet écrit également une entrée de référentiel commentée dans `/etc/apt/sources.list.d/claude-desktop.list` ; décommenter sa ligne `deb` est équivalent.

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

Cela supprime la clé de signature ainsi que l'application, donc si vous avez ajouté l'entrée du référentiel lors de l'installation, supprimez-la également :

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

* Confirmez que l'entrée du référentiel a été écrite. `cat /etc/apt/sources.list.d/claude-desktop.list` devrait afficher la ligne `deb` de l'étape [Ajouter le référentiel apt d'Anthropic](#install). Si le fichier est vide ou manquant, exécutez cette étape à nouveau.
* Confirmez que votre architecture est prise en charge. `dpkg --print-architecture` devrait afficher `amd64` ou `arm64`. Le référentiel ne publie pas de paquets pour d'autres architectures.
* Exécutez `sudo apt update` à nouveau et vérifiez sa sortie pour les erreurs liées à `downloads.claude.ai`. Une erreur de réseau ou de clé à cet endroit signifie que le référentiel a été ajouté mais n'a pas pu être atteint ou vérifié.

Si le référentiel est en place et accessible et que le paquet n'est toujours pas trouvé, [installez à partir d'un fichier téléchargé](#install-from-a-downloaded-file) à la place.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Ce qui n'est pas encore dans la bêta Linux
</h2>

* **Computer Use** : [le contrôle des applications et de l'écran](/docs/fr/desktop#let-claude-use-your-computer) n'est pas disponible sur Linux.
* **Dictation** : l'entrée vocale n'est pas disponible dans l'application de bureau Linux. Utilisez plutôt la [dictation vocale](/docs/fr/voice-dictation) dans l'interface de ligne de commande.
* **Raccourci global Quick Entry** : fonctionne sur X11. Sur Wayland natif, cela nécessite le portail GlobalShortcuts de votre environnement de bureau.
* **Fedora et RHEL** : seules les distributions basées sur Debian sont prises en charge aujourd'hui. Le support pour des distributions supplémentaires arrivera à l'avenir.

Pour tout ce qui n'est pas encore disponible dans l'application de bureau, l'[interface de ligne de commande](/docs/fr/quickstart) exécute le même moteur Claude Code et prend en charge une gamme plus large de distributions Linux ; consultez la [configuration requise du système](/docs/fr/setup#system-requirements).
