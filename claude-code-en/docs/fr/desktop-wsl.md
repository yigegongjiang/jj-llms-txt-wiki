> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop dans WSL

> Exécuter des sessions Code dans une distribution WSL 2 sur Windows

Sur Windows, l'onglet Code peut exécuter une session dans une distribution WSL 2 au lieu de sur Windows lui-même. Le processus Claude Code de la session, ses outils et git s'exécutent tous à l'intérieur de la distribution, en utilisant sa chaîne d'outils Linux et les chemins Linux natifs, le même environnement que celui ciblé par votre projet.

Utilisez une session WSL lorsque votre référentiel se trouve à l'intérieur du système de fichiers de la distribution. Travailler sur ces fichiers à partir de Windows passe par un système de fichiers réseau, ce qui est lent et casse la surveillance des fichiers ; exécuter la session à l'intérieur de la distribution évite les deux.

<h2 id="requirements">
  Conditions requises
</h2>

* Windows 10 ou 11 avec [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 n'est pas pris en charge.
* Au moins une distribution installée (par exemple, Ubuntu).
* `git` installé à l'intérieur de la distribution.

<h2 id="start-a-wsl-session">
  Démarrer une session WSL
</h2>

<Steps>
  <Step title="Choisir une distribution">
    Démarrez une nouvelle session dans l'onglet Code et ouvrez le sélecteur d'environnement. Vos distributions WSL 2 installées apparaissent dans une section **WSL**. Choisissez-en une.
  </Step>

  <Step title="Choisir un dossier">
    La session démarre dans le répertoire personnel de la distribution. Utilisez le sélecteur de dossier pour choisir un dossier de projet. La navigation se fait à l'intérieur de la distribution, avec des chemins Linux comme `/home/you/project`.
  </Step>

  <Step title="Approuver le dossier">
    La première session dans un dossier affiche la boîte de dialogue de confiance de l'espace de travail. La confiance est accordée par distribution et par dossier ; approuver un dossier dans une distribution ne s'applique pas à une autre distribution ou au même chemin sur Windows.
  </Step>
</Steps>

La première session dans une distribution prend un peu plus de temps pendant que Claude se configure à l'intérieur. Vous pouvez également ouvrir un dossier `\\wsl.localhost\...` à partir du sélecteur de dossier normal, et il se réouvre à l'intérieur de cette distribution.

Les dossiers que vous avez utilisés récemment apparaissent dans le sélecteur par distribution, donc se reconnecter à un projet ne prend qu'un clic.

<h2 id="what-works-in-a-wsl-session">
  Ce qui fonctionne dans une session WSL
</h2>

Les sessions parallèles, les chats latéraux, l'examen des différences visuelles, l'état des branches et des demandes de tirage, et les worktrees fonctionnent tous, soutenus par git et la chaîne d'outils à l'intérieur de la distribution. « Ouvrir dans l'éditeur » ouvre VS Code connecté à la distribution via [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl).

Quelques fonctionnalités ne sont pas encore disponibles dans les sessions WSL : le terminal intégré, les connecteurs et les plugins, la bifurcation de session, le volet du navigateur de fichiers et les suggestions de fichiers lorsque vous tapez `@` dans le compositeur.

<h2 id="managed-devices">
  Appareils gérés
</h2>

Sur les appareils gérés par une organisation, les sessions WSL peuvent ne pas être disponibles. Si le démarrage de la session échoue avec un message indiquant que l'appareil est géré, cela est contrôlé par votre administrateur. Administrateurs : consultez [comment les paramètres atteignent les appareils](/docs/fr/admin-setup#decide-how-settings-reach-devices) dans le guide de déploiement.
