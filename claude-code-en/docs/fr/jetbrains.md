> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Utilisez Claude Code avec les IDEs JetBrains, notamment IntelliJ, PyCharm, WebStorm et bien d'autres

Claude Code s'intègre aux IDEs JetBrains via un plugin dédié, offrant des fonctionnalités telles que l'affichage interactif des différences, le partage du contexte de sélection, et bien d'autres.

<h2 id="supported-ides">
  IDEs supportés
</h2>

Le plugin Claude Code fonctionne avec la plupart des IDEs JetBrains, notamment :

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Fonctionnalités
</h2>

* **Lancement rapide** : Utilisez `Cmd+Esc` (Mac) ou `Ctrl+Esc` (Windows/Linux) pour ouvrir Claude Code directement depuis votre éditeur, ou cliquez sur le bouton Claude Code dans l'interface utilisateur
* **Affichage des différences** : Les modifications de code peuvent être affichées directement dans la visionneuse de différences de l'IDE au lieu du terminal
* **Contexte de sélection** : La sélection actuelle ou l'onglet dans l'IDE est automatiquement partagé avec Claude Code. Les [règles de refus `Read`](/docs/fr/permissions#read-and-edit) bloquent ce partage pour les fichiers correspondants
* **Raccourcis de référence de fichier** : Utilisez `Cmd+Option+K` (Mac) ou `Alt+Ctrl+K` (Linux/Windows) pour insérer des références de fichier telles que `@src/auth.ts#L1-99`
* **Partage des diagnostics** : Les erreurs de diagnostic de l'IDE, telles que les erreurs de lint et de syntaxe, sont automatiquement partagées avec Claude au fur et à mesure que vous travaillez

<h2 id="installation">
  Installation
</h2>

Le plugin exécute la commande `claude` dans le terminal intégré de votre IDE et s'y connecte. Il ne regroupe pas sa propre copie de la CLI, donc installez les deux éléments :

<Steps>
  <Step title="Installer la CLI Claude Code">
    Suivez le [guide de démarrage rapide](/docs/fr/quickstart) pour installer la CLI si vous ne l'avez pas déjà fait. Le plugin affiche une notification ' Impossible de lancer Claude Code ' lorsque `claude` ne se trouve pas sur votre PATH.
  </Step>

  <Step title="Installer le plugin JetBrains">
    Installez le [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) depuis la Marketplace JetBrains et redémarrez votre IDE.
  </Step>
</Steps>

Si `claude` est installé quelque part que votre IDE ne peut pas trouver, définissez le chemin complet dans les [paramètres de commande Claude](#general-settings) du plugin.

Claude Code fonctionne avec n'importe quel abonnement Claude payant (Pro, Max, Team ou Enterprise) ou un compte Claude Console, et aucune clé API n'est requise. Vous serez invité à [vous connecter](/docs/fr/authentication#log-in-to-claude-code) la première fois que vous exécutez `claude`.

<Note>
  Après l'installation du plugin, vous devrez peut-être redémarrer complètement votre IDE pour que les modifications prennent effet.
</Note>

<h2 id="usage">
  Utilisation
</h2>

<h3 id="from-your-ide">
  Depuis votre IDE
</h3>

Exécutez `claude` depuis le terminal intégré de votre IDE, et toutes les fonctionnalités d'intégration seront actives.

<h3 id="from-external-terminals">
  Depuis des terminaux externes
</h3>

Utilisez la commande `/ide` dans n'importe quel terminal externe pour connecter Claude Code à votre IDE JetBrains et activer toutes les fonctionnalités :

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Si vous souhaitez que Claude ait accès aux mêmes fichiers que votre IDE, démarrez Claude Code à partir du même répertoire que la racine du projet de votre IDE.

<h2 id="configuration">
  Configuration
</h2>

<h3 id="claude-code-settings">
  Paramètres de Claude Code
</h3>

Configurez l'intégration de l'IDE via les paramètres de Claude Code :

1. Exécutez `claude`
2. Entrez la commande `/config`
3. Définissez l'outil de différence sur `auto` pour afficher les différences dans l'IDE, ou `terminal` pour les conserver dans le terminal

<h3 id="plugin-settings">
  Paramètres du plugin
</h3>

Configurez le plugin Claude Code en accédant à **Paramètres → Outils → Claude Code \[Beta]** :

<h4 id="general-settings">
  Paramètres généraux
</h4>

* **Commande Claude** : Spécifiez une commande personnalisée pour exécuter Claude, par exemple `claude`, `/usr/local/bin/claude`, ou `npx @anthropic-ai/claude-code`
* **Supprimer la notification pour la commande Claude non trouvée** : Ignorez les notifications concernant la non-détection de la commande Claude
* **Activer l'utilisation d'Option+Entrée pour les invites multi-lignes** : Sur macOS uniquement. Lorsqu'elle est activée, Option+Entrée insère de nouvelles lignes dans les invites Claude Code. Désactivez si la touche Option est capturée de manière inattendue. Nécessite un redémarrage du terminal.
* **Activer les mises à jour automatiques** : Vérifiez automatiquement et installez les mises à jour du plugin, appliquées au redémarrage

<Tip>
  Pour les utilisateurs WSL : Définissez `wsl -d Ubuntu -- bash -lic "claude"` comme votre commande Claude (remplacez `Ubuntu` par le nom de votre distribution WSL)
</Tip>

<h4 id="esc-key-configuration">
  Configuration de la touche ESC
</h4>

Si la touche ESC n'interrompt pas les opérations Claude Code dans les terminaux JetBrains :

1. Accédez à **Paramètres → Outils → Terminal**
2. Soit :
   * Décochez « Déplacer le focus vers l'éditeur avec Échap », soit
   * Cliquez sur « Configurer les raccourcis clavier du terminal » et supprimez le raccourci « Basculer le focus vers l'éditeur »
3. Appliquez les modifications

Cela permet à la touche ESC d'interrompre correctement les opérations Claude Code.

<h2 id="special-configurations">
  Configurations spéciales
</h2>

<h3 id="remote-development">
  Développement à distance
</h3>

<Warning>
  Lors de l'utilisation du développement à distance JetBrains, vous devez installer le plugin sur l'hôte distant via **Paramètres → Plugin (Hôte)**.
</Warning>

Le plugin doit être installé sur l'hôte distant, et non sur votre machine cliente locale.

<h3 id="wsl-configuration">
  Configuration WSL
</h3>

Si vous utilisez Claude Code sur WSL2 avec un IDE JetBrains et que vous voyez « Aucun IDE disponible détecté », la cause est généralement le réseau NAT de WSL2 ou le Pare-feu Windows bloquant la connexion entre WSL2 et l'IDE s'exécutant sur l'hôte Windows. WSL1 utilise directement le réseau de l'hôte et n'est pas affecté.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  Autoriser le trafic WSL2 via le Pare-feu Windows
</h4>

Ceci est la correction recommandée car elle conserve votre mode de mise en réseau WSL2 existant.

<Steps>
  <Step title="Trouvez votre adresse IP WSL2">
    Depuis votre shell WSL, exécutez :

    ```bash theme={null}
    hostname -I
    ```

    Notez le sous-réseau, par exemple `172.21.123.45` se trouve dans `172.21.0.0/16`.
  </Step>

  <Step title="Créez une règle de pare-feu">
    Ouvrez PowerShell en tant qu'administrateur et exécutez ce qui suit, en ajustant la plage d'adresses IP pour correspondre à votre sous-réseau :

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Redémarrez votre IDE et Claude Code">
    Fermez et rouvrez les deux pour que la nouvelle règle prenne effet.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Basculer WSL2 vers la mise en réseau en miroir
</h4>

La mise en réseau en miroir nécessite Windows 11 22H2 ou version ultérieure. Si vous êtes sur Windows 10, utilisez la règle de pare-feu ci-dessus à la place.

Ajoutez ceci à `.wslconfig` dans votre répertoire utilisateur Windows :

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Ensuite, redémarrez WSL avec `wsl --shutdown` depuis PowerShell.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="plugin-not-working">
  Le plugin ne fonctionne pas
</h3>

Si le plugin est installé mais que les fonctionnalités Claude Code n'apparaissent pas dans votre IDE :

* Assurez-vous que vous exécutez Claude Code à partir du répertoire racine du projet
* Vérifiez que le plugin JetBrains est activé dans les paramètres de l'IDE
* Redémarrez complètement l'IDE (vous devrez peut-être le faire plusieurs fois)
* Pour le développement à distance, assurez-vous que le plugin est installé sur l'hôte distant

<h3 id="ide-not-detected">
  IDE non détecté
</h3>

Si l'exécution de `claude` affiche « Aucun IDE disponible détecté » :

* Vérifiez que le plugin est installé et activé
* Redémarrez complètement l'IDE
* Vérifiez que vous exécutez Claude Code à partir du terminal intégré
* Pour les utilisateurs WSL, consultez la [configuration WSL](#wsl-configuration) ci-dessus

<h3 id="command-not-found">
  Commande non trouvée
</h3>

Si cliquer sur l'icône Claude affiche « commande non trouvée » :

1. Vérifiez que Claude Code est installé en exécutant `claude --version` dans un terminal
2. Configurez le chemin de la commande Claude dans les paramètres du plugin
3. Pour les utilisateurs WSL, utilisez le format de commande WSL mentionné dans la section configuration

<h2 id="security-considerations">
  Considérations de sécurité
</h2>

Lorsque Claude Code s'exécute dans un IDE JetBrains en mode de permission [`acceptEdits`](/docs/fr/permission-modes#auto-approve-file-edits-with-acceptedits-mode), il peut être en mesure de modifier les fichiers de configuration de l'IDE qui peuvent être exécutés automatiquement par votre IDE. Cela peut augmenter le risque d'exécution de Claude Code en mode `acceptEdits` et permettre de contourner les invites de permission de Claude Code pour l'exécution bash.

Lors de l'exécution dans les IDEs JetBrains, considérez :

* L'utilisation du mode d'approbation manuelle pour les modifications
* La prise de précautions supplémentaires pour vous assurer que Claude n'est utilisé qu'avec des invites de confiance
* La sensibilisation aux fichiers auxquels Claude Code a accès pour les modifier

Pour les problèmes d'installation ou de connexion de Claude Code en dehors de l'IDE, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  Le serveur MCP intégré à l'IDE
</h3>

Lorsque le plugin est actif, il exécute un serveur MCP local auquel le CLI se connecte automatiquement. C'est ainsi que le CLI ouvre les diffs dans la visionneuse de diffs native de l'IDE, lit votre sélection actuelle pour les mentions `@`, et récupère les diagnostics d'inspection dans la conversation.

Le serveur s'appelle `ide` et est masqué de `/mcp` car il n'y a rien à configurer. Si votre organisation utilise un [hook `PreToolUse`](/docs/fr/hooks#pretooluse) pour créer une liste d'autorisation des outils MCP, cependant, vous devrez savoir qu'il existe.

**Contexte de sélection et de fichier ouvert.** Lors de la connexion, le CLI inclut votre sélection d'éditeur actuelle et le chemin du fichier actif comme contexte sur chaque invite que vous envoyez. La transcription affiche une ligne `⧉ Selected N lines from <file>` lorsque cela se produit. Pour exclure un fichier sensible tel que `.env`, ajoutez une [règle de refus `Read`](/docs/fr/permissions#read-and-edit) pour son chemin. Une règle de refus correspondante empêche à la fois le texte sélectionné et l'avis de fichier ouvert pour ce fichier d'atteindre Claude.

**Transport et authentification.** Le serveur écoute sur un port éphémère attribué par le système d'exploitation, et le port n'est pas configurable. Le transport est un `ws://` non chiffré ; sur la boucle locale, tout processus qui pourrait capturer le trafic peut également lire le token du fichier de verrouillage, donc TLS n'ajouterait pas de protection contre un attaquant local. Chaque démarrage d'IDE génère un token d'authentification aléatoire frais, l'écrit dans un fichier de verrouillage à `~/.claude/ide/<port>.lock`, et le CLI doit le présenter comme l'en-tête `X-Claude-Code-Ide-Authorization` pour se connecter. Si `CLAUDE_CONFIG_DIR` est défini, le fichier de verrouillage est écrit à `$CLAUDE_CONFIG_DIR/ide/` à la place.

**Outils exposés au modèle.** Le serveur héberge plusieurs outils, mais un seul est visible au modèle. Le reste est un RPC interne que le CLI utilise pour sa propre interface utilisateur, comme l'ouverture de diffs et la lecture de sélections, et est filtré avant que la liste des outils n'atteigne Claude.

| Nom de l'outil (tel que vu par les hooks) | Ce qu'il fait                                                                                                                               | Lecture seule |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `mcp__ide__getDiagnostics`                | Retourne les diagnostics d'inspection de l'IDE, les erreurs et avertissements affichés dans l'éditeur. Optionnellement limité à un fichier. | Oui           |

Le plugin JetBrains n'expose pas d'outil d'exécution de code au modèle.

**Interface d'écoute.** L'interface réseau sur laquelle le serveur se lie est contrôlée par **Accept connections from all network interfaces** sous **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)**. Avec le paramètre désactivé, le serveur écoute uniquement sur `127.0.0.1` et n'est pas accessible depuis d'autres hôtes. Avec le paramètre activé, le port est accessible depuis votre réseau local. Le paramètre existe pour les cas où le CLI ne peut pas atteindre l'IDE sur la boucle locale, comme WSL2 avec la mise en réseau NAT par défaut ou une configuration d'IDE distant ; consultez [Configuration WSL](#wsl-configuration) pour ce scénario.

<Warning>
  L'activation de **Accept connections from all network interfaces** rend le port MCP de l'IDE accessible depuis votre réseau local. Les connexions nécessitent toujours le token d'authentification du fichier de verrouillage, mais comme le transport est un `ws://` non chiffré, à la fois le trafic de session et ce token traversent le réseau en texte clair lorsque le paramètre est activé. Activez-le uniquement lorsque la boucle locale ne peut vraiment pas fonctionner. Pour WSL2, préférez [la mise en réseau en miroir](#switch-wsl2-to-mirrored-networking) afin que l'interface de boucle locale Windows soit partagée avec la VM Linux et que le socket puisse rester sur la boucle locale.
</Warning>
