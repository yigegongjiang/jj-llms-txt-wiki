> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Laisser Claude utiliser votre ordinateur depuis la CLI

> Activez l'utilisation de l'ordinateur dans la CLI Claude Code pour que Claude puisse ouvrir des applications, cliquer, taper et voir votre écran sur macOS. Testez les applications natives, déboguez les problèmes visuels et automatisez les outils GUI uniquement sans quitter votre terminal.

<Note>
  L'utilisation de l'ordinateur est un aperçu de recherche sur macOS qui nécessite un plan Pro ou Max. Elle n'est pas disponible sur les plans Team ou Enterprise. Elle nécessite une session interactive, elle n'est donc pas disponible en mode non-interactif avec le drapeau `-p`.
</Note>

L'utilisation de l'ordinateur permet à Claude d'ouvrir des applications, de contrôler votre écran et de travailler sur votre machine comme vous le feriez. Depuis la CLI, Claude peut compiler une application Swift, la lancer, cliquer sur chaque bouton et faire une capture d'écran du résultat, tout dans la même conversation où il a écrit le code.

Cette page explique comment fonctionne l'utilisation de l'ordinateur dans la CLI. Pour l'application Desktop sur macOS ou Windows, consultez [utilisation de l'ordinateur dans Desktop](/docs/fr/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  Ce que vous pouvez faire avec l'utilisation de l'ordinateur
</h2>

L'utilisation de l'ordinateur gère les tâches qui nécessitent une GUI : tout ce que vous devriez normalement faire en quittant le terminal et en le faisant manuellement.

* **Créer et valider des applications natives** : demandez à Claude de créer une application de barre de menu macOS. Claude écrit le Swift, le compile, lance l'application et clique sur chaque contrôle pour vérifier qu'il fonctionne avant que vous ne l'ouvriez jamais.
* **Tests d'interface utilisateur de bout en bout** : pointez Claude vers une application Electron locale et dites « teste le flux d'intégration ». Claude ouvre l'application, clique sur l'inscription et fait une capture d'écran de chaque étape. Pas de configuration Playwright, pas de harnais de test.
* **Déboguer les problèmes visuels et de mise en page** : dites à Claude « la modale se coupe sur les petites fenêtres ». Claude redimensionne la fenêtre, reproduit le bogue, en fait une capture d'écran, corrige le CSS et vérifie la correction. Claude voit ce que vous voyez.
* **Piloter les outils GUI uniquement** : interagissez avec les outils de conception, les panneaux de contrôle du matériel, le simulateur iOS ou les applications propriétaires qui n'ont pas de CLI ou d'API.

<h2 id="when-computer-use-applies">
  Quand l'utilisation de l'ordinateur s'applique
</h2>

Claude a plusieurs façons d'interagir avec une application ou un service. L'utilisation de l'ordinateur est la plus large et la plus lente, donc Claude essaie d'abord l'outil le plus précis :

* Si vous avez un [serveur MCP](/docs/fr/mcp) pour le service, Claude l'utilise.
* Si la tâche est une commande shell, Claude utilise Bash.
* Si la tâche est du travail de navigateur et que vous avez [Claude dans Chrome](/docs/fr/chrome) configuré, Claude l'utilise.
* Si aucun de ces éléments ne s'applique, Claude utilise l'utilisation de l'ordinateur.

Le contrôle de l'écran est réservé aux choses que rien d'autre ne peut atteindre : les applications natives, les simulateurs et les outils sans API.

<h2 id="enable-computer-use">
  Activer l'utilisation de l'ordinateur
</h2>

L'utilisation de l'ordinateur est disponible en tant que serveur MCP intégré appelé `computer-use`. Elle est désactivée par défaut jusqu'à ce que vous l'activiez.

<Steps>
  <Step title="Ouvrir le menu MCP">
    Dans une session Claude Code interactive, exécutez :

    ```text theme={null}
    /mcp
    ```

    Trouvez `computer-use` dans la liste des serveurs. Il s'affiche comme désactivé.
  </Step>

  <Step title="Activer le serveur">
    Sélectionnez `computer-use` et choisissez **Activer**. Le paramètre persiste par projet, vous ne le faites donc qu'une fois pour chaque projet où vous souhaitez utiliser l'ordinateur.
  </Step>

  <Step title="Accorder les autorisations macOS">
    La première fois que Claude essaie d'utiliser votre ordinateur, vous verrez une invite pour accorder deux autorisations macOS :

    * **Accessibilité** : permet à Claude de cliquer, taper et faire défiler
    * **Enregistrement d'écran** : permet à Claude de voir ce qui est sur votre écran

    L'invite inclut des liens pour ouvrir le volet Paramètres système pertinent. Accordez les deux, puis sélectionnez **Réessayer** dans l'invite. macOS peut vous demander de redémarrer Claude Code après avoir accordé l'enregistrement d'écran.
  </Step>
</Steps>

Après la configuration, demandez à Claude de faire quelque chose qui nécessite la GUI :

```text theme={null}
Créez la cible d'application, lancez-la et cliquez sur chaque onglet pour
vous assurer que rien ne plante. Faites une capture d'écran de tout état
d'erreur que vous trouvez.
```

<h2 id="approve-apps-per-session">
  Approuver les applications par session
</h2>

L'activation du serveur `computer-use` ne donne pas à Claude accès à chaque application sur votre machine. La première fois que Claude a besoin d'une application spécifique dans une session, une invite apparaît dans votre terminal montrant :

* Quelles applications Claude souhaite contrôler
* Toute autorisation supplémentaire demandée, comme l'accès au presse-papiers
* Combien d'autres applications seront masquées pendant que Claude travaille

Choisissez **Autoriser pour cette session** ou **Refuser**. Les approbations durent pour la session actuelle. Vous pouvez approuver plusieurs applications à la fois lorsque Claude les demande ensemble.

Les applications avec une large portée affichent un avertissement supplémentaire dans l'invite pour que vous sachiez ce que l'approbation leur accorde :

| Avertissement                              | S'applique à                                              |
| :----------------------------------------- | :-------------------------------------------------------- |
| Équivalent à l'accès shell                 | Terminal, iTerm, VS Code, Warp et autres terminaux et IDE |
| Peut lire ou écrire n'importe quel fichier | Finder                                                    |
| Peut modifier les paramètres système       | Paramètres système                                        |

Ces applications ne sont pas bloquées. L'avertissement vous permet de décider si la tâche justifie ce niveau d'accès.

Le niveau de contrôle de Claude varie également selon la catégorie d'application : les navigateurs et les plateformes de trading sont en lecture seule, les terminaux et les IDE sont en clic uniquement, et tout le reste obtient un contrôle complet. Consultez [autorisations des applications dans Desktop](/docs/fr/desktop#app-permissions) pour la répartition complète des niveaux.

<h2 id="how-claude-works-on-your-screen">
  Comment Claude fonctionne sur votre écran
</h2>

Comprendre le flux vous aide à anticiper ce que Claude fera et comment intervenir.

<h3 id="one-session-at-a-time">
  Une session à la fois
</h3>

L'utilisation de l'ordinateur maintient un verrou à l'échelle de la machine à partir de la première action d'utilisation de l'ordinateur jusqu'à ce que la session qui l'a acquis se termine. À partir de la v2.1.195, terminer la tâche ne libère pas le verrou ; seule la fermeture de la session le fait. Si une autre session Claude Code utilise déjà votre ordinateur, les nouvelles tentatives échouent avec un message vous indiquant quelle session détient le verrou. Quittez d'abord cette session.

<h3 id="apps-are-hidden-while-claude-works">
  Les applications sont masquées pendant que Claude travaille
</h3>

Lorsque Claude commence à contrôler votre écran, les autres applications visibles sont masquées pour que Claude n'interagisse qu'avec les applications approuvées. Votre fenêtre de terminal reste visible et est exclue des captures d'écran, vous pouvez donc regarder la session et Claude ne voit jamais sa propre sortie.

Lorsque Claude termine le tour, les applications masquées sont restaurées automatiquement.

<h3 id="screenshots-are-downscaled-automatically">
  Les captures d'écran sont réduites automatiquement
</h3>

Claude Code réduit chaque capture d'écran avant de l'envoyer au modèle. Vous n'avez pas besoin de réduire votre résolution d'affichage ou de redimensionner les fenêtres sur les écrans Retina ou autres écrans haute résolution. Un MacBook Pro 16 pouces à résolution Retina native capture à 3456×2234 et réduit à environ 1372×887, en préservant le rapport d'aspect.

Il n'y a pas de paramètre pour modifier la taille cible. Si le texte ou les contrôles à l'écran sont trop petits pour que Claude les lise après la réduction, augmentez leur taille dans l'application plutôt que de modifier votre résolution d'affichage.

<h3 id="stop-at-any-time">
  Arrêter à tout moment
</h3>

Lorsque Claude acquiert le verrou, une notification macOS apparaît : « Claude utilise votre ordinateur · appuyez sur Échap pour arrêter ». Appuyez sur `Échap` n'importe où pour abandonner l'action actuelle immédiatement, ou appuyez sur `Ctrl+C` dans le terminal. De toute façon, Claude s'arrête, affiche vos applications et vous rend le contrôle. La session conserve le [verrou d'utilisation de l'ordinateur](#one-session-at-a-time) jusqu'à ce qu'elle se termine.

Une deuxième notification apparaît lorsque Claude a terminé.

<h2 id="safety-and-the-trust-boundary">
  Sécurité et limite de confiance
</h2>

<Warning>
  Contrairement à l'[outil Bash en bac à sable](/docs/fr/sandboxing), l'utilisation de l'ordinateur s'exécute sur votre vrai bureau avec accès aux applications que vous approuvez. Claude vérifie chaque action et signale les injections de requête potentielles du contenu à l'écran, mais la limite de confiance est différente. Consultez le [guide de sécurité de l'utilisation de l'ordinateur](https://support.claude.com/en/articles/14128542) pour les meilleures pratiques.
</Warning>

Les garde-fous intégrés réduisent le risque sans nécessiter de configuration :

* **Approbation par application** : Claude ne peut contrôler que les applications que vous avez approuvées dans la session actuelle.
* **Avertissements sentinelles** : les applications qui accordent l'accès shell, système de fichiers ou paramètres système sont signalées avant que vous les approuviez.
* **Terminal exclu des captures d'écran** : Claude ne voit jamais votre fenêtre de terminal, donc les invites à l'écran dans votre session ne peuvent pas être renvoyées au modèle.
* **Échappement global** : la touche `Échap` abandonne l'utilisation de l'ordinateur de n'importe où, et la pression sur la touche est consommée pour que l'injection de requête ne puisse pas l'utiliser pour fermer les dialogues.
* **Fichier de verrou** : une seule session peut contrôler votre machine à la fois.

<h2 id="example-workflows">
  Exemples de flux de travail
</h2>

Ces exemples montrent les façons courantes de combiner l'utilisation de l'ordinateur avec les tâches de codage.

<h3 id="validate-a-native-build">
  Valider une compilation native
</h3>

Après avoir apporté des modifications à une application macOS ou iOS, demandez à Claude de compiler et vérifier en une seule passe :

```text theme={null}
Créez la cible MenuBarStats, lancez-la, ouvrez la fenêtre des préférences
et vérifiez que le curseur d'intervalle met à jour l'étiquette. Faites une
capture d'écran de la fenêtre des préférences lorsque vous avez terminé.
```

Claude exécute `xcodebuild`, lance l'application, interagit avec l'interface utilisateur et rapporte ce qu'il trouve.

<h3 id="reproduce-a-layout-bug">
  Reproduire un bogue de mise en page
</h3>

Lorsqu'un bogue visuel n'apparaît qu'à certaines tailles de fenêtre, laissez Claude le trouver :

```text theme={null}
La modale des paramètres coupe son pied de page sur les fenêtres étroites.
Redimensionnez la fenêtre de l'application jusqu'à ce que vous puissiez la
reproduire, faites une capture d'écran de l'état coupé, puis vérifiez le
CSS du conteneur modal.
```

Claude redimensionne la fenêtre, capture l'état cassé et lit les feuilles de style pertinentes.

<h3 id="test-a-simulator-flow">
  Tester un flux de simulateur
</h3>

Pilotez le simulateur iOS sans écrire XCTest :

```text theme={null}
Ouvrez le simulateur iOS, lancez l'application, appuyez sur les écrans
d'intégration et dites-moi si un écran prend plus d'une seconde à charger.
```

Claude contrôle le simulateur de la même manière que vous le feriez avec une souris.

<h2 id="differences-from-the-desktop-app">
  Différences par rapport à l'application Desktop
</h2>

Les surfaces CLI et Desktop partagent le même moteur d'utilisation de l'ordinateur, avec quelques différences :

| Fonctionnalité                   | Desktop                                                                           | CLI                                |
| :------------------------------- | :-------------------------------------------------------------------------------- | :--------------------------------- |
| Plateformes                      | macOS et Windows                                                                  | macOS uniquement                   |
| Activer                          | Basculer dans **Paramètres > Général** (sous **Application Desktop**)             | Activer `computer-use` dans `/mcp` |
| Liste des applications refusées  | Configurable dans les paramètres                                                  | Pas encore disponible              |
| Basculer l'affichage automatique | Optionnel                                                                         | Toujours activé                    |
| Intégration Dispatch             | Les sessions générées par Dispatch peuvent utiliser l'utilisation de l'ordinateur | Non applicable                     |

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  « L'utilisation de l'ordinateur est utilisée par une autre session Claude »
</h3>

Une autre session Claude Code détient le verrou, qu'elle conserve jusqu'à sa fermeture. Quittez cette session. Si l'autre session a planté, le verrou est libéré automatiquement lorsque Claude détecte que le processus n'est plus en cours d'exécution.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  L'invite de permissions macOS continue de réapparaître
</h3>

macOS nécessite parfois un redémarrage du processus demandeur après avoir accordé l'enregistrement d'écran. Quittez complètement Claude Code et démarrez une nouvelle session. Si l'invite persiste, ouvrez **Paramètres système > Confidentialité et sécurité > Enregistrement d'écran** et confirmez que votre application de terminal est répertoriée et activée.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` n'apparaît pas dans `/mcp`
</h3>

Le serveur n'apparaît que sur les configurations éligibles. Vérifiez que :

* Vous êtes sur macOS. L'utilisation de l'ordinateur dans la CLI n'est pas disponible sur Linux ou Windows. Sur Windows, utilisez [utilisation de l'ordinateur dans Desktop](/docs/fr/desktop#let-claude-use-your-computer) à la place.
* Vous êtes sur un plan Pro ou Max. Exécutez `/status` pour confirmer votre abonnement.
* Vous êtes authentifié via claude.ai. L'utilisation de l'ordinateur n'est pas disponible avec les fournisseurs tiers comme Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Si vous accédez à Claude exclusivement via un fournisseur tiers, vous avez besoin d'un compte claude.ai séparé pour utiliser cette fonctionnalité.
* Vous êtes dans une session interactive. L'utilisation de l'ordinateur n'est pas disponible en mode non-interactif avec le drapeau `-p`.

<h2 id="see-also">
  Voir aussi
</h2>

* [Utilisation de l'ordinateur dans Desktop](/docs/fr/desktop#let-claude-use-your-computer) : la même capacité avec une page de paramètres graphique
* [Claude dans Chrome](/docs/fr/chrome) : automatisation du navigateur pour les tâches basées sur le web
* [MCP](/docs/fr/mcp) : connectez Claude à des outils et des API structurés
* [Bac à sable](/docs/fr/sandboxing) : comment l'outil Bash de Claude isole l'accès au système de fichiers et au réseau
* [Guide de sécurité de l'utilisation de l'ordinateur](https://support.claude.com/en/articles/14128542) : meilleures pratiques pour une utilisation sécurisée de l'ordinateur
