> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code avec Chrome

> Connectez Claude Code à votre navigateur Chrome pour tester des applications web, déboguer avec les journaux de console, automatiser le remplissage de formulaires et extraire des données des pages web.

Claude Code s'intègre à l'[extension Claude in Chrome du navigateur](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) pour vous offrir des capacités d'automatisation du navigateur depuis la CLI ou l'[extension VS Code](/docs/fr/vs-code#automate-browser-tasks-with-chrome). Créez votre code, puis testez et déboguez dans le navigateur sans changer de contexte.

Claude ouvre de nouveaux onglets pour les tâches du navigateur et partage l'état de connexion de votre navigateur, ce qui lui permet d'accéder à n'importe quel site auquel vous êtes déjà connecté. Les actions du navigateur s'exécutent en temps réel dans une fenêtre Chrome visible. Lorsque Claude rencontre une page de connexion ou un CAPTCHA, il s'arrête et vous demande de le gérer manuellement.

L'extension regroupe les onglets que Claude ouvre dans un groupe d'onglets Chrome lié à votre session. Dans les sessions locales, le fait que Claude Code ferme ce groupe à la fin de la session dépend de la façon dont elle se termine :

* Lorsque vous tapez `/clear`, Claude Code ferme le groupe, y compris les pages ouvertes, sauf si le travail qui survit au clear est toujours en cours d'exécution
* Lorsque vous changez de session avec une commande comme `/resume`, quittez Claude Code, ou exécutez un `/clear` tandis que le travail qui le survit est toujours en cours d'exécution, Claude Code ferme le groupe uniquement s'il ne contient que des nouveaux onglets vides, de sorte que les pages que vous lisez peut-être encore restent ouvertes

<Note>
  L'intégration Chrome fonctionne avec Google Chrome et Microsoft Edge. Claude Code détecte également l'extension et configure la connexion dans d'autres navigateurs basés sur Chromium, notamment Brave, Arc, Vivaldi et Opera. L'intégration Chrome n'est pas prise en charge dans Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Capacités
</h2>

Avec Chrome connecté, vous pouvez enchaîner les actions du navigateur avec les tâches de codage dans un seul flux de travail :

* **Débogage en direct** : lisez les erreurs de console et l'état du DOM directement, puis corrigez le code qui les a causées
* **Vérification de la conception** : créez une interface utilisateur à partir d'une maquette Figma, puis ouvrez-la dans le navigateur pour vérifier qu'elle correspond
* **Test d'application web** : testez la validation des formulaires, vérifiez les régressions visuelles ou vérifiez les flux utilisateur
* **Applications web authentifiées** : interagissez avec Google Docs, Gmail, Notion ou n'importe quelle application à laquelle vous êtes connecté sans connecteurs API
* **Extraction de données** : extrayez des informations structurées des pages web et enregistrez-les localement
* **Automatisation des tâches** : automatisez les tâches répétitives du navigateur comme la saisie de données, le remplissage de formulaires ou les flux multi-sites
* **Enregistrement de session** : enregistrez les interactions du navigateur sous forme de GIF pour documenter ou partager ce qui s'est passé

<h2 id="prerequisites">
  Prérequis
</h2>

Avant d'utiliser Claude Code avec Chrome, vous avez besoin de :

* [Google Chrome](https://www.google.com/chrome/), [Microsoft Edge](https://www.microsoft.com/edge) ou un autre navigateur basé sur Chromium tel que Brave, Arc, Vivaldi ou Opera
* Extension [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) version 1.0.36 ou supérieure, disponible dans le Chrome Web Store
* [Claude Code](/docs/fr/quickstart#step-1-install-claude-code)
* Un plan Anthropic direct (Pro, Max, Team ou Enterprise)

L'intégration Chrome nécessite également de se connecter avec `/login`. Si vous vous authentifiez avec une clé API ou un jeton de longue durée de [`claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token), Claude Code désactive l'intégration Chrome, même si vous passez `--chrome`, car l'extension de navigateur ne peut pas s'authentifier avec ces identifiants. Avant la v2.1.216, ces sessions pouvaient activer l'intégration Chrome, mais chaque tentative de connexion à l'extension de navigateur échouait avec une erreur 403.

<Note>
  L'intégration Chrome n'est pas disponible via des fournisseurs tiers comme Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Si vous accédez à Claude exclusivement via un fournisseur tiers, vous avez besoin d'un compte claude.ai séparé pour utiliser cette fonctionnalité.
</Note>

<h2 id="get-started-in-the-cli">
  Démarrer dans la CLI
</h2>

<Steps>
  <Step title="Lancer Claude Code avec Chrome">
    Démarrez Claude Code avec le drapeau `--chrome` :

    ```bash theme={null}
    claude --chrome
    ```

    La première fois que vous lancez avec Chrome, Claude Code affiche une boîte de dialogue unique qui présente l'intégration et explique comment fonctionnent les autorisations de site. Appuyez sur Entrée pour continuer.

    Pour activer Chrome pour les sessions futures sans le drapeau, consultez [Activer Chrome par défaut](#enable-chrome-by-default).
  </Step>

  <Step title="Demander à Claude d'utiliser le navigateur">
    Cet exemple accède à une page, interagit avec elle et rapporte ce qu'il trouve, le tout depuis votre terminal ou éditeur :

    ```text wrap theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Si Claude Code demande une permission avant une action du navigateur, approuvez-la. La boîte de dialogue commence par `Claude in Chrome wants to` et offre une option pour autoriser toutes les actions sur ce site pour la session. Claude ouvre un nouvel onglet et commence la tâche.
  </Step>
</Steps>

Exécutez `/chrome` à tout moment pour vérifier l'état de la connexion, gérer les autorisations, reconnecter l'extension ou choisir quel navigateur connecté utiliser. L'intégration fonctionne lorsque le panneau d'état affiche « Status: Enabled » et « Extension: Installed ».

Si plusieurs navigateurs sont connectés, vous choisissez lequel Claude utilise. Lorsqu'une action de navigateur commence avant que vous ayez choisi, Claude vous invite à en sélectionner un. Pour basculer entre les navigateurs ultérieurement, exécutez `/chrome` et sélectionnez **Select browser…**. Claude continue à utiliser votre choix même lorsqu'un autre navigateur se connecte.

Pour VS Code, consultez [l'automatisation du navigateur dans VS Code](/docs/fr/vs-code#automate-browser-tasks-with-chrome).

<h3 id="install-the-extension-when-claude-asks">
  Installer l'extension quand Claude le demande
</h3>

Lorsque Claude a besoin de votre navigateur dans une session interactive et que Claude Code ne détecte pas l'extension, Claude Code affiche une invite d'installation intitulée « Claude wants to use your browser ». Claude Code demande au maximum une fois par session.

L'invite offre trois choix :

* **Install extension** : ouvre la page d'installation de l'extension dans votre navigateur et démarre une configuration guidée. Claude Code attend l'installation, connecte l'extension et active les outils du navigateur dans la même session. Lorsque la connexion est prête, sélectionnez « Continue with browser tools » et Claude reprend la tâche dans votre navigateur. Vous pouvez quitter la configuration en sélectionnant « Continue without browser tools » et terminer plus tard avec `/chrome`.
* **Not now** : continue la tâche sans outils du navigateur. Claude Code peut demander à nouveau dans une session ultérieure.
* **Don't ask again** : arrête l'invite dans les sessions futures. Vous pouvez toujours configurer l'intégration à tout moment avec `/chrome`.

Si votre organisation bloque le serveur MCP `claude-in-chrome` avec le [paramètre géré `deniedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists), Claude Code n'affiche pas l'invite d'installation.

<h3 id="enable-chrome-by-default">
  Activer Chrome par défaut
</h3>

Pour éviter de passer `--chrome` à chaque session, exécutez `/chrome` et sélectionnez « Enabled by default ».

Claude Code démarre normalement lorsque Chrome n'est pas en cours d'exécution. Avant la v2.1.211, le démarrage pouvait se bloquer lorsque l'intégration Chrome était activée mais que Chrome n'était pas en cours d'exécution.

Dans l'[extension VS Code](/docs/fr/vs-code#automate-browser-tasks-with-chrome), Chrome est disponible chaque fois que l'extension Chrome est installée. Aucun drapeau supplémentaire n'est nécessaire.

<Note>
  L'activation de Chrome par défaut dans la CLI augmente l'utilisation du contexte puisque les outils du navigateur sont toujours chargés. Si vous remarquez une augmentation de la consommation de contexte, désactivez ce paramètre et utilisez `--chrome` uniquement si nécessaire.
</Note>

<h3 id="manage-site-permissions">
  Gérer les autorisations du site
</h3>

Les autorisations au niveau du site sont héritées de l'extension Chrome. Gérez les autorisations dans les paramètres de l'extension Chrome pour contrôler les sites que Claude peut parcourir, cliquer et taper.

<h3 id="browser-tools-in-plan-mode">
  Outils du navigateur en mode plan
</h3>

En [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), une invite de permission apparaît avant que Claude n'enregistre un GIF, n'ouvre un nouvel onglet ou n'exécute un raccourci. Si le [mode de contournement des permissions est disponible](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) dans votre session et que la [récupération des drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching) est désactivée, ces appels s'exécutent sans invite.

Un appel `tabs_context_mcp` demande également une invite lorsqu'il définit `createIfEmpty`, et il en va de même pour un appel `browser_batch` qui inclut l'une de ces actions.

<h2 id="example-workflows">
  Exemples de flux de travail
</h2>

Ces exemples montrent les façons courantes de combiner les actions du navigateur avec les tâches de codage. Exécutez `/mcp`, sélectionnez `claude-in-chrome`, puis sélectionnez **Afficher les outils** pour voir la liste complète des outils de navigateur disponibles.

<h3 id="test-a-local-web-application">
  Tester une application web locale
</h3>

Lors du développement d'une application web, demandez à Claude de vérifier que vos modifications fonctionnent correctement :

```text wrap theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude accède à votre serveur local, interagit avec le formulaire et rapporte ce qu'il observe.

<h3 id="debug-with-console-logs">
  Déboguer avec les journaux de console
</h3>

Claude peut lire la sortie de la console pour aider à diagnostiquer les problèmes. Dites à Claude quels modèles rechercher plutôt que de demander toute la sortie de la console, car les journaux peuvent être verbeux :

```text wrap theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude lit les messages de la console et peut filtrer les modèles ou types d'erreurs spécifiques.

<h3 id="automate-form-filling">
  Automatiser le remplissage de formulaires
</h3>

Accélérez les tâches répétitives de saisie de données :

```text wrap theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude lit votre fichier local, navigue dans l'interface web et saisit les données pour chaque enregistrement.

<h3 id="upload-files-to-web-pages">
  Télécharger des fichiers sur des pages web
</h3>

Claude peut joindre des fichiers de votre machine aux champs de téléchargement sur une page. Claude Code lit le fichier et envoie son contenu au navigateur, de sorte que les téléchargements fonctionnent dans les sessions locales et distantes. Nécessite Claude Code v2.1.211 ou version ultérieure.

Cet exemple joint un fichier journal à un formulaire :

```text wrap theme={null}
Open the bug tracker at bugs.example.com, create a new issue,
and attach logs/session.log to it
```

Trois restrictions s'appliquent aux téléchargements :

* **Autorisations** : Claude ne peut télécharger un fichier que lorsque la session est autorisée à le lire, donc les [règles de permission](/docs/fr/settings-reference#permission-settings) qui refusent l'accès `Read` à un fichier bloquent également son téléchargement.
* **Taille** : un seul téléchargement peut inclure jusqu'à 10 Mo de fichiers au total.
* **Liens physiques** : Claude refuse les fichiers qui ont plusieurs liens physiques, ce qui est courant dans les magasins de gestionnaires de paquets comme `node_modules`. Copiez le fichier et téléchargez la copie.

<h3 id="draft-content-in-google-docs">
  Rédiger du contenu dans Google Docs
</h3>

Utilisez Claude pour écrire directement dans vos documents sans configuration d'API :

```text wrap theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude ouvre le document, clique dans l'éditeur et tape le contenu. Cela fonctionne avec n'importe quelle application web à laquelle vous êtes connecté : Gmail, Notion, Sheets, et plus.

<h3 id="extract-data-from-web-pages">
  Extraire des données des pages web
</h3>

Extrayez des informations structurées des sites web :

```text wrap theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude accède à la page, lit le contenu et compile les données dans un format structuré.

<h3 id="run-multi-site-workflows">
  Exécuter des flux de travail multi-sites
</h3>

Coordonnez les tâches sur plusieurs sites web :

```text wrap theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude travaille sur plusieurs onglets pour rassembler les informations et terminer le flux de travail.

<h3 id="record-a-demo-gif">
  Enregistrer un GIF de démonstration
</h3>

Créez des enregistrements partageables des interactions du navigateur :

```text wrap theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude enregistre la séquence d'interaction et l'enregistre sous forme de fichier GIF. L'enregistrement capture tout ce qui est visible dans le navigateur, y compris les détails du compte sur les pages connectées, alors examinez-le avant de le partager en dehors de votre équipe.

<h3 id="save-screenshots-to-disk">
  Enregistrer des captures d'écran sur le disque
</h3>

Demandez à Claude de conserver une capture d'écran en tant que fichier :

```text wrap theme={null}
Take a screenshot of the checkout page and save it to disk
```

Claude enregistre l'image sur le disque et rapporte le chemin du fichier. Avant v2.1.211, l'option `save_to_disk` de l'outil de capture d'écran n'écrivait pas de fichier.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="extension-not-detected">
  Extension non détectée
</h3>

Si Claude Code ne peut pas détecter l'extension Chrome :

1. Vérifiez que l'extension Chrome est installée et activée dans `chrome://extensions`
2. Vérifiez que Claude Code est à jour en exécutant `claude --version`
3. Vérifiez que Chrome est en cours d'exécution
4. Exécutez `/chrome` et sélectionnez « Reconnect extension » pour rétablir la connexion
5. Si le problème persiste, redémarrez Claude Code et Chrome

La première fois que vous activez l'intégration Chrome, Claude Code installe un fichier de configuration d'hôte de messagerie native. Chrome lit ce fichier au démarrage, donc si l'extension n'est pas détectée à votre première tentative, redémarrez Chrome pour récupérer la nouvelle configuration.

Claude Code ouvre un onglet du navigateur vous invitant à connecter l'extension uniquement lors de cette première installation. Claude Code ne le rouvre pas lorsqu'une session ultérieure réécrit le fichier de configuration, par exemple après le changement de versions ou de répertoires de configuration.

Si la connexion échoue toujours, vérifiez que le fichier de configuration d'hôte existe à :

Pour Chrome :

* **macOS** : `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux** : `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows** : vérifiez `HKCU\Software\Google\Chrome\NativeMessagingHosts\` dans le Registre Windows

Pour Edge :

* **macOS** : `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux** : `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows** : vérifiez `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` dans le Registre Windows

Les autres navigateurs basés sur Chromium lisent le même fichier à partir de leur propre répertoire de configuration, nommé d'après le navigateur. Par exemple, Brave sur macOS utilise `~/Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts/`, et sur Windows, chaque navigateur a sa propre clé de registre, comme `HKCU\Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\`.

<h3 id="browser-not-responding">
  Le navigateur ne répond pas
</h3>

Si les commandes du navigateur de Claude cessent de fonctionner :

1. Vérifiez si une boîte de dialogue modale (alerte, confirmation, invite) bloque la page. Les boîtes de dialogue JavaScript bloquent les événements du navigateur et empêchent Claude de recevoir des commandes. Fermez la boîte de dialogue manuellement, puis demandez à Claude de continuer.
2. Demandez à Claude de créer un nouvel onglet et réessayez
3. Redémarrez l'extension Chrome en la désactivant et en la réactivant dans `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  La connexion s'interrompt lors de longues sessions
</h3>

Le service worker de l'extension Chrome peut devenir inactif lors de sessions prolongées, ce qui rompt la connexion. Si les outils du navigateur cessent de fonctionner après une période d'inactivité, exécutez `/chrome` et sélectionnez « Reconnect extension ».

<h3 id="windows-specific-issues">
  Problèmes spécifiques à Windows
</h3>

Sous Windows, vous pouvez rencontrer :

* **Conflits de tuyau nommé (EADDRINUSE)** : si un autre processus utilise le même tuyau nommé, redémarrez Claude Code. Fermez toute autre session Claude Code qui pourrait utiliser Chrome.
* **Erreurs d'hôte de messagerie native** : si l'hôte de messagerie native plante au démarrage, essayez de réinstaller Claude Code pour régénérer la configuration d'hôte.
* **Les pages de configuration ne s'ouvrent pas** : mettez à jour Claude Code. Avant la v2.1.211, l'onglet du navigateur vous invitant à connecter l'extension pouvait ne pas s'ouvrir sur Windows.

<h3 id="common-error-messages">
  Messages d'erreur courants
</h3>

Ce sont les erreurs les plus fréquemment rencontrées et comment les résoudre :

| Erreur                                      | Cause                                                                                                                                                                           | Solution                                                                                                                                                                                                                                                                                                             |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| « Browser extension is not connected »      | L'hôte de messagerie native ne peut pas atteindre l'extension, ou la liste d'adresses IP autorisées de votre organisation rejette la connexion à `bridge.claudeusercontent.com` | Redémarrez Chrome et Claude Code, puis exécutez `/chrome` pour reconnecter. Si votre organisation utilise une liste d'adresses IP autorisées et que l'erreur persiste, consultez [Listes d'adresses IP autorisées et sortie proxy de l'organisation](/docs/fr/network-config#organization-ip-allowlists-and-proxy-egress) |
| Extension shows "Not detected" in `/chrome` | L'extension Chrome n'est pas installée ou est désactivée                                                                                                                        | Installez ou activez l'extension dans `chrome://extensions`                                                                                                                                                                                                                                                          |
| « No tab available »                        | Claude a tenté d'agir avant qu'un onglet soit prêt                                                                                                                              | Demandez à Claude de créer un nouvel onglet et réessayez                                                                                                                                                                                                                                                             |
| « Receiving end does not exist »            | Le service worker de l'extension est devenu inactif                                                                                                                             | Exécutez `/chrome` et sélectionnez « Reconnect extension »                                                                                                                                                                                                                                                           |

<h2 id="see-also">
  Voir aussi
</h2>

* [Utilisation informatique](/docs/fr/computer-use) : contrôlez les applications macOS natives quand une tâche ne peut pas être effectuée dans un navigateur
* [Utiliser Claude Code dans VS Code](/docs/fr/vs-code#automate-browser-tasks-with-chrome) : automatisation du navigateur dans l'extension VS Code
* [Référence CLI](/docs/fr/cli-reference) : drapeaux de ligne de commande incluant `--chrome`
* [Flux de travail courants](/docs/fr/common-workflows) : plus de façons d'utiliser Claude Code
* [Données et confidentialité](/docs/fr/data-usage) : comment Claude Code gère vos données
* [Démarrer avec Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome) : documentation complète pour l'extension Chrome, incluant les raccourcis, la planification et les autorisations
