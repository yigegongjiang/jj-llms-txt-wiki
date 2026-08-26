> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Choisir un mode de permission

> Contrôlez si Claude demande une approbation avant de modifier des fichiers ou d'exécuter des commandes. Basculez entre les modes avec Maj+Tab dans l'interface de ligne de commande ou utilisez le sélecteur de mode dans VS Code, Desktop et claude.ai.

Quand Claude souhaite modifier un fichier, exécuter une commande shell ou effectuer une requête réseau, il s'arrête et vous demande d'approuver l'action. Les modes de permission contrôlent la fréquence à laquelle cette pause se produit. Le mode que vous choisissez façonne le flux d'une session : le mode Manuel vous permet d'examiner chaque action au fur et à mesure, tandis que les modes plus souples permettent à Claude de travailler dans des étapes plus longues et ininterrompues, puis de vous faire un rapport une fois terminé. Choisissez une surveillance plus étroite pour les travaux sensibles, ou moins d'interruptions quand vous faites confiance à la direction.

<h2 id="available-modes">
  Modes disponibles
</h2>

Chaque mode fait un compromis différent entre la commodité et la supervision. Le tableau ci-dessous montre ce que Claude peut faire sans demander de permission dans chaque mode.

| Mode                                                                | Ce qui s'exécute sans demander                                                                                         | Idéal pour                                          |
| :------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `default`                                                           | Lectures uniquement                                                                                                    | Démarrage, travail sensible                         |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lectures, modifications de fichiers et commandes courantes du système de fichiers (`mkdir`, `touch`, `mv`, `cp`, etc.) | Itération sur le code que vous examinez             |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Lectures uniquement                                                                                                    | Exploration d'une base de code avant de la modifier |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Tout, avec des vérifications de sécurité en arrière-plan                                                               | Tâches longues, réduction de la fatigue des invites |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Outils pré-approuvés uniquement                                                                                        | CI verrouillé et scripts                            |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Tout                                                                                                                   | Conteneurs et machines virtuelles isolés uniquement |

Le mode qui examine chaque action s'appelle **Manual** dans la CLI, dans `claude --help`, dans les extensions VS Code et JetBrains, et dans l'application de bureau. Sa valeur de configuration est `default`, ce que les hooks et les intégrations SDK utilisent. La CLI accepte `manual` comme alias partout où vous tapez la valeur, par exemple `claude --permission-mode manual` ou `"defaultMode": "manual"`. L'étiquette Manual et l'alias `manual` nécessitent Claude Code v2.1.200 ou version ultérieure. L'étiquette de l'application de bureau ne dépend pas de votre version CLI.

Dans tous les modes sauf `bypassPermissions`, les écritures vers les [chemins protégés](#protected-paths) ne sont jamais auto-approuvées, protégeant l'état du référentiel et la configuration de Claude contre la corruption accidentelle.

Les modes définissent la ligne de base. Superposez les [règles de permission](/docs/fr/permissions#manage-permissions) sur le dessus pour pré-approuver ou bloquer des outils spécifiques. Les règles de refus, les règles de demande explicite, le [paramètre `ask` de l'organisation sur les outils de connecteur](/docs/fr/mcp#organization-controls-on-connector-tools), et le marqueur [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) s'appliquent dans tous les modes, y compris `bypassPermissions`. Les règles d'autorisation n'ont aucun effet dans ce mode car tout le reste est déjà approuvé.

<h2 id="switch-permission-modes">
  Changer les modes de permission
</h2>

Vous pouvez changer de mode en cours de session, au démarrage ou comme paramètre par défaut persistant. Le mode est défini par ces contrôles, pas en demandant à Claude dans le chat. Sélectionnez votre interface ci-dessous pour voir comment le modifier.

<Tabs>
  <Tab title="CLI">
    **En cours de session** : appuyez sur `Shift+Tab` pour parcourir `default` → `acceptEdits` → `plan`. Le mode actuel s'affiche dans la barre d'état. Le mode manuel, `default` dans ce cycle, affiche un badge gris `⏸ manual mode on`. Avant la v2.1.203, la barre d'état n'affichait aucun badge en mode Manuel.

    Tous les modes ne sont pas dans le cycle par défaut :

    * `auto` : apparaît lorsque votre compte répond aux [conditions du mode auto](#eliminate-prompts-with-auto-mode) ; le basculer vers celui-ci change de mode sans invite de confirmation
    * `bypassPermissions` : apparaît après que vous ayez démarré avec `--permission-mode bypassPermissions`, `--dangerously-skip-permissions` ou `--allow-dangerously-skip-permissions` ; la variante `--allow-` ajoute le mode au cycle sans l'activer
    * `dontAsk` : n'apparaît jamais dans le cycle ; définissez-le avec `--permission-mode dontAsk`

    Les modes optionnels activés s'insèrent après `plan`, avec `bypassPermissions` en premier et `auto` en dernier. Si vous avez les deux activés, vous parcourrez `bypassPermissions` en allant vers `auto`.

    **Au démarrage** : passez le mode en tant que drapeau.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Par défaut** : définissez `defaultMode` dans [settings](/docs/fr/settings#settings-files).

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    Le même drapeau `--permission-mode` fonctionne avec `-p` pour les [exécutions non interactives](/docs/fr/headless).
  </Tab>

  <Tab title="VS Code">
    **En cours de session** : cliquez sur l'indicateur de mode en bas de la zone de saisie.

    **Par défaut** : définissez `claudeCode.initialPermissionMode` dans les paramètres VS Code, ou utilisez le panneau des paramètres de l'extension Claude Code.

    L'indicateur de mode affiche ces étiquettes, mappées au mode auquel chacune s'applique :

    | Étiquette UI       | Mode                |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    Avant la v2.1.205, l'extension étiquetait `plan` comme Plan mode et `auto` comme Auto mode.

    Le mode Auto apparaît dans l'indicateur de mode lorsque votre compte répond à tous les critères énumérés dans la [section mode auto](#eliminate-prompts-with-auto-mode). Le paramètre `claudeCode.initialPermissionMode` n'accepte pas `auto`. Pour démarrer en mode auto par défaut, définissez `defaultMode` dans vos [paramètres utilisateur](/docs/fr/settings#settings-files) à la place. Claude Code ignore `defaultMode: "auto"` dans les paramètres de projet et locaux.

    Bypass permissions nécessite le bouton bascule **Allow dangerously skip permissions** dans les paramètres de l'extension avant qu'il n'apparaisse dans l'indicateur de mode.

    Consultez le [guide VS Code](/docs/fr/vs-code) pour les détails spécifiques à l'extension.
  </Tab>

  <Tab title="JetBrains">
    Le plugin JetBrains exécute Claude Code dans le terminal de l'IDE, donc le changement de mode fonctionne de la même manière que dans la CLI : appuyez sur `Shift+Tab` pour parcourir, ou passez `--permission-mode` lors du lancement.
  </Tab>

  <Tab title="Desktop">
    **En cours de session** : utilisez le sélecteur de mode à côté du bouton d'envoi. Tous les modes n'apparaissent pas dans le sélecteur :

    * **Auto** : apparaît lorsque votre compte répond aux [conditions du mode auto](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions** : nécessite le bouton bascule **Allow bypass permissions mode** dans les paramètres Desktop sur les plans Pro et Max ; sur les plans Team et Enterprise, la politique organisationnelle le contrôle à la place

    Pour les détails spécifiques au bureau, consultez [Choose a permission mode](/docs/fr/desktop#choose-a-permission-mode) dans le guide Desktop.

    **Par défaut** : définissez `defaultMode` dans [settings](/docs/fr/settings#settings-files). L'application de bureau lit les mêmes fichiers de paramètres que la CLI et applique le mode aux nouvelles sessions locales.

    Un mode que vous choisissez dans le sélecteur de mode est mémorisé par dossier et prend précédence sur `defaultMode` pour ce dossier. Plan est l'exception : le choisir s'applique à la session actuelle uniquement.

    Cet exemple définit le mode Plan comme paramètre par défaut pour les nouvelles sessions locales :

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    Utilisez la liste déroulante de mode à côté de la zone de saisie sur [claude.ai/code](https://claude.ai/code) ou dans l'application mobile. Les invites de permission apparaissent dans claude.ai pour approbation. Les modes qui apparaissent dépendent de l'endroit où la session s'exécute :

    * **Sessions cloud** sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : Accept edits, Plan et Auto. Accept edits correspond au mode `default` : l'environnement cloud pré-approuve les modifications de fichiers quel que soit le mode, donc la liste déroulante affiche Accept edits au lieu de Manual. Les sessions cloud honorent toujours `defaultMode: "acceptEdits"` à partir des paramètres. Le mode Auto n'apparaît que lorsque votre organisation le permet et que le modèle sélectionné le supporte. Bypass permissions n'est pas disponible.
    * **Sessions [Remote Control](/docs/fr/remote-control)** sur votre machine locale : Manual, Accept edits et Plan. Vous ne pouvez pas sélectionner Auto ou Bypass permissions à partir de l'application. La liste déroulante affiche le mode dans lequel se trouve la session locale, y compris un mode défini à partir du terminal, et se met à jour lorsque le mode change dans l'application ou dans le terminal. La seule exception est Bypass permissions : la session ne signale jamais ce mode à claude.ai, donc le basculer à partir du terminal ne change pas ce que la liste déroulante affiche. Avant la v2.1.202, les sessions connectées avec `/remote-control` ou `claude --remote-control` ne signalaient pas du tout leur mode, donc claude.ai et l'application mobile pouvaient afficher un mode dans lequel la session n'était pas. L'inadéquation affectait uniquement l'étiquette : Claude Code générait des invites de permission à partir du mode réel de la session, et elles apparaissaient toujours dans l'application pour approbation.

    Pour Remote Control, vous pouvez également définir le mode de démarrage lors du lancement de l'hôte :

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approuver les modifications de fichiers avec le mode acceptEdits
</h2>

Le mode `acceptEdits` permet à Claude de créer et modifier des fichiers dans votre répertoire de travail sans demander de confirmation. La barre d'état affiche `⏵⏵ accept edits on` lorsque ce mode est actif.

En plus des modifications de fichiers, le mode `acceptEdits` auto-approuve les commandes Bash courantes du système de fichiers : `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, et `sed`. Ces commandes sont également auto-approuvées lorsqu'elles sont préfixées par des variables d'environnement sûres telles que `LANG=C` ou `NO_COLOR=1`, ou des wrappers de processus tels que `timeout`, `nice`, ou `nohup`. Comme pour les modifications de fichiers, l'auto-approbation s'applique uniquement aux chemins à l'intérieur de votre répertoire de travail ou `additionalDirectories`. Les chemins en dehors de cette portée, les écritures vers les [chemins protégés](#protected-paths), et toutes les autres commandes Bash sauf l'[ensemble intégré en lecture seule](/docs/fr/permissions#read-only-commands) demandent toujours une confirmation.

Lorsque l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) est activé, le mode `acceptEdits` auto-approuve également `Set-Content`, `Add-Content`, `Clear-Content`, et `Remove-Item` sur les chemins dans la portée, ainsi que leurs alias courants. Les mêmes règles de portée et de chemin protégé s'appliquent.

Utilisez `acceptEdits` lorsque vous souhaitez examiner les modifications dans votre éditeur ou via `git diff` après coup plutôt que d'approuver chaque modification en ligne.

Appuyez sur `Shift+Tab` une fois à partir du mode Manuel pour y accéder, ou démarrez directement avec celui-ci :

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analysez avant de modifier avec le mode plan
</h2>

Le mode plan indique à Claude de rechercher et de proposer des modifications sans les effectuer. Claude lit les fichiers, exécute des commandes shell pour explorer et rédige un plan, mais ne modifie pas votre source. Les invites de permission s'appliquent comme en mode Manuel, sauf si le [mode auto](/docs/fr/auto-mode-config) est disponible et que `useAutoModeDuringPlan` est activé, ce qui est le paramètre par défaut. Avec le mode auto actif, le classificateur approuve les commandes en lecture seule telles que les recherches et les lectures de fichiers sans demander de confirmation. Les modifications restent bloquées de toute façon jusqu'à ce que vous approuviez le plan.

Entrez en mode plan en appuyant sur `Shift+Tab` ou en préfixant une seule invite avec `/plan`. Vous pouvez également démarrer en mode plan à partir de la CLI :

```bash theme={null}
claude --permission-mode plan
```

Appuyez à nouveau sur `Shift+Tab` pour quitter le mode plan sans approuver un plan.

<h3 id="review-and-approve-a-plan">
  Examinez et approuvez un plan
</h3>

Lorsque le plan est prêt, Claude le présente et demande comment procéder. À partir de cette invite, vous pouvez :

* Approuver et démarrer en mode auto
* Approuver et accepter les modifications
* Approuver et examiner chaque modification manuellement
* Continuer la planification avec des commentaires
* Affiner avec [Ultraplan](/docs/fr/ultraplan) pour un examen basé sur le navigateur

L'approbation d'un plan quitte le mode plan et bascule la session vers le mode de permission que chaque option d'approbation décrit, de sorte que Claude commence à modifier. Pour planifier à nouveau, revenez au mode plan avec `Shift+Tab`, ou préfixez votre prochaine invite avec `/plan`.

Appuyez sur `Ctrl+G` pour ouvrir le plan proposé dans votre éditeur de texte par défaut et le modifier directement avant que Claude ne procède. Lorsque [`showClearContextOnPlanAccept`](/docs/fr/settings#available-settings) est activé, chaque option d'approbation offre également la possibilité d'effacer d'abord le contexte de planification.

L'acceptation d'un plan nomme également automatiquement la session à partir du contenu du plan, sauf si vous avez déjà défini un nom avec `--name` ou `/rename`.

<h3 id="set-plan-mode-as-the-default">
  Définissez le mode plan comme paramètre par défaut
</h3>

Pour faire du mode plan le paramètre par défaut d'un projet, définissez `defaultMode` dans `.claude/settings.json` :

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  Éliminer les invites de permission avec le mode auto
</h2>

Le mode auto permet à Claude d'exécuter sans invites de permission routinières. Un modèle classificateur distinct examine les actions avant leur exécution, bloquant tout ce qui dépasse votre demande, cible une infrastructure non reconnue, ou semble provoqué par du contenu hostile que Claude a lu. Les [règles ask](/docs/fr/permissions#manage-permissions) explicites forcent toujours une invite.

Les suppressions ciblant la racine du système de fichiers ou le répertoire personnel, telles que `rm -rf /` et `rm -rf ~`, demandent une approbation au lieu d'aller au classificateur. Cette invite se déclenche également quand la commande contient une substitution de commande avec `$(...)` ou des backticks, ou une substitution de processus avec `<(...)`, que la suppression se trouve à l'intérieur de la substitution, comme dans `echo "$(rm -rf ~)"`, ou ailleurs dans la même commande. Avant v2.1.208, les commandes contenant ces formes allaient au classificateur au lieu de demander.

Le mode auto encourage également Claude à continuer à travailler sans s'arrêter pour des questions de clarification, bien que Claude demande toujours quand votre invite ou une skill le nécessite explicitement. Pour un comportement plus autonome tout en conservant les invites de permission, définissez plutôt le [style de sortie Proactive](/docs/fr/output-styles).

<Warning>
  Le mode auto réduit les invites de permission mais ne garantit pas la sécurité. Utilisez-le pour les tâches où vous faites confiance à la direction générale, pas comme remplacement pour l'examen des opérations sensibles.
</Warning>

Le mode auto est disponible uniquement quand votre compte répond à tous ces critères :

* **Plan** : Tous les plans.
* **Propriétaire** : sur Team et Enterprise, un propriétaire doit l'activer dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code) avant que les utilisateurs puissent l'activer. Les administrateurs peuvent également désactiver le mode auto en définissant `permissions.disableAutoMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/permissions#managed-settings). Pour l'onglet Code de l'application de bureau, `disableAutoMode` est le contrôle au niveau de l'organisation, et le bouton des paramètres d'administration ne s'applique pas.
* **Modèle** : sur l'API Anthropic, Claude Opus 4.6 ou ultérieur, ou Sonnet 4.6 ou ultérieur. Sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées, uniquement Claude Sonnet 5, Opus 4.7, et Opus 4.8. Les modèles plus anciens, y compris Sonnet 4.5, Opus 4.5, Haiku, et les modèles claude-3, ne sont pas supportés sur aucun fournisseur.
* **Fournisseur** : disponible par défaut sur l'API Anthropic, Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de passerelle d'applications Claude connectées. Dans les versions v2.1.158 à v2.1.206, le mode auto était désactivé sur tous ces fournisseurs sauf l'API Anthropic jusqu'à ce que vous définissiez `CLAUDE_CODE_ENABLE_AUTO_MODE=1` ; v2.1.207 a supprimé cette exigence.

Si Claude Code signale que le mode auto est indisponible, l'une de ces exigences n'est pas satisfaite ; ce n'est pas une panne transitoire. Un message distinct qui nomme un modèle et dit que le mode auto « ne peut pas déterminer la sécurité » d'une action est une panne transitoire du classificateur ; voir la [référence d'erreur](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action).

Si vous définissez `defaultMode: "auto"` dans les [paramètres](/docs/fr/settings#available-settings) et que la session démarre en mode `default` sans erreur, le paramètre se trouve probablement dans `.claude/settings.json` ou `.claude/settings.local.json`. Claude Code v2.1.142 et ultérieur ignorent `auto` de ces fichiers afin qu'un référentiel ne puisse pas s'accorder le mode auto. Déplacez-le vers `~/.claude/settings.json`.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Mode auto sur Bedrock, Agent Platform, ou Foundry
</h3>

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), la [plateforme Agent de Google Cloud](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry), et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées, le mode auto apparaît dans le cycle `Shift+Tab` par défaut. L'apparition dans le cycle ne change pas le mode dans lequel une session démarre : les sessions démarrent toujours dans votre [`defaultMode`](/docs/fr/settings#available-settings), qui est Manual sauf si vous le changez. Seuls Claude Sonnet 5, Opus 4.7, et Opus 4.8 sont supportés sur ces fournisseurs.

Pour faire du mode auto le mode de démarrage par défaut, définissez `"permissions": {"defaultMode": "auto"}` dans les paramètres utilisateur ou gérés.

Pour empêcher les développeurs d'utiliser le mode auto, définissez `disableAutoMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/permissions#managed-settings). Cela supprime `auto` du cycle `Shift+Tab` et rejette `--permission-mode auto` au démarrage.

Dans les versions v2.1.158 à v2.1.206, le mode auto était désactivé sur ces fournisseurs jusqu'à ce que vous définissiez `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, et Claude Code ignorait `defaultMode: "auto"` sur ces fournisseurs sauf si la variable était également définie. La variable est toujours acceptée pour la compatibilité et n'a aucun effet à partir de v2.1.207.

<h3 id="what-the-classifier-blocks-by-default">
  Ce que le classificateur bloque par défaut
</h3>

Le classificateur fait confiance à votre répertoire de travail et aux remotes qui ont été configurés pour celui-ci au démarrage de la session. Un remote ajouté ou réorienté pendant la session avec `git remote add` ou `git remote set-url` n'est pas approuvé, et tout le reste est traité comme externe jusqu'à ce que vous [configuriez l'infrastructure approuvée](/docs/fr/auto-mode-config). Avant v2.1.200, les remotes ajoutés en milieu de session étaient également approuvés.

**Bloqué par défaut** :

* Téléchargement et exécution de code, comme `curl | bash`
* Envoi de données sensibles à des points de terminaison externes
* Déploiements et migrations en production
* Suppression en masse sur le stockage cloud
* Octroi de permissions IAM ou de référentiel
* Modification d'une infrastructure partagée
* Destruction irréversible de fichiers qui existaient avant la session
* Force push
* Poussée vers la branche par défaut du référentiel quand la poussée contient du contenu sensible tel que des secrets ou des données personnelles ou confiées, contient des modifications dissimulées ou mal décrites par rapport à ce que vous avez demandé, contient du contenu porté ou d'abord lu de l'extérieur du référentiel, ou contourne une demande de tirage, un examen, ou une vérification que vous avez demandée. Une simple poussée vers la branche par défaut n'est pas bloquée en elle-même, et l'effacement d'une poussée signalée nécessite de nommer le contenu signalé ou l'examen contourné, pas seulement la poussée. Le classificateur est une couche : les [règles `permissions.deny`](/docs/fr/permissions#manage-permissions) s'appliquent dans tous les modes et peuvent bloquer les poussées vers la branche par défaut complètement, et la protection de branche du référentiel s'applique toujours. Avant v2.1.203, toute poussée directe vers la branche par défaut était bloquée
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop`, ou `git stash clear`, que le classificateur présume éliminerait les modifications non validées
* `git commit --amend` quand le commit à HEAD n'a pas été créé dans cette session
* À partir de v2.1.198, `git commit --amend` quand le commit à HEAD a déjà été poussé. Une reformulation de message uniquement n'est pas bloquée : `--amend -m` sans rien de nouvellement préparé, sur un commit que Claude a créé pendant cette session
* `terraform destroy`, `pulumi destroy`, `cdk destroy`, ou `terragrunt destroy`, et l'application d'un plan qui détruit des ressources

Claude Code v2.1.195 et ultérieur bloquent plus de catégories par défaut. Plusieurs dépendent d'entrées [environment](/docs/fr/auto-mode-config#define-trusted-infrastructure), telles que les cibles de remote sensibles et les portées IaC protégées, que vous pouvez affiner à des noms concrets.

* Écriture dans un gestionnaire de secrets, ou modification des enregistrements DNS ou des certificats TLS
* Fusion d'une demande de tirage qu'aucun humain n'a approuvée, approbation de la propre demande de tirage de Claude, ou désactivation des vérifications CI
* Publication d'un commentaire qui est lui-même une commande pour l'automatisation, comme `atlantis apply` ou le `/deploy` ou `/merge` d'un bot
* Basculement, augmentation, ou suppression d'un drapeau de fonctionnalité en production
* Application de modifications d'infrastructure à une portée IaC protégée, ou drainage et suppression de nœuds de cluster
* Écritures dans un cluster de calcul partagé qui vont au-delà de la ressource que vous avez nommée, comme un sélecteur d'étiquette ou `--all` qui capture les tâches d'autres utilisateurs
* Création de ressources Kubernetes qui s'exécutent sur chaque nœud ou interceptent le trafic du cluster, comme les DaemonSets et les webhooks d'admission
* Shells interactifs ou port-forwards vers une cible de remote sensible
* Ouverture d'un tunnel ou d'un shell inverse qui rend un service local accessible depuis l'internet public
* Impression d'une credential ou d'un token en direct dans la transcription ou un fichier
* Accès à un emplacement répertorié comme emplacement de données sensibles dans votre [environment](/docs/fr/auto-mode-config#define-trusted-infrastructure), ou copie de données hors de celui-ci. À partir de v2.1.198, cela bloque également l'envoi de données d'un vers un public que l'entrée exclut
* Routage d'une installation de package autour de votre registre de package interne vers un registre public. À partir de v2.1.198, cela s'applique également quand vous avez dit à Claude qu'un registre interne ou un miroir existe dans la conversation, pas seulement quand un est répertorié dans votre environment
* Exécution d'une commande avec un drapeau qui désarme une garde de sécurité, comme `--insecure`
* Lancement d'une boucle d'agent autonome qui s'exécute sans approbation humaine ou sandbox, comme une lancée avec `--dangerously-skip-permissions` ou `--no-sandbox`. À partir de v2.1.198, cela couvre également l'exécution d'un agent tiers ou d'un harnais d'évaluation avec isolation et approbation par action désactivées, comme un runner lancé avec `--yes-always`
* Les actions du navigateur [Claude in Chrome](/docs/fr/chrome) qui pourraient envoyer le contenu de la page, les cookies, ou les credentials hors-origine

Claude Code v2.1.198 et ultérieur bloquent également ceux-ci par défaut :

* Suppression de fichiers dans `/tmp`, `$TMPDIR`, ou un autre répertoire de scratch ou de cache partagé par wildcard, glob, ou filtre d'âge plutôt que par un chemin nommé spécifique
* Inclusion de détails sensibles dans le contenu envoyé, téléchargé, publié, ou écrit à d'autres personnes ou systèmes partagés, quand votre propre message n'a pas autorisé ces détails pour ce destinataire. Les corps de PR et de problème, les messages de commit, et les commentaires comptent comme ce type de contenu sortant quand le référentiel est en dehors de la limite de confiance ou public, y compris les référentiels publics de votre propre organisation ; les chemins de fichiers internes, les noms de code, les données de réponse API en direct telles que les e-mails ou les identifiants de compte, et les identifiants d'infrastructure comptent comme des détails sensibles. La portée PR, problème, et message de commit nécessite Claude Code v2.1.200 ou ultérieur. Les données personnelles en direct d'une réponse API dans un corps de PR ou de problème, comme une adresse e-mail, un identifiant de compte ou d'organisation, ou une métrique d'utilisation, vous obligent à nommer ces détails et le destinataire indépendamment de la visibilité du référentiel ou de la limite de confiance. Cette vérification nécessite Claude Code v2.1.203 ou ultérieur
* Envoi de frappes au volet tmux de Claude Code lui-même pour piloter sa propre interface, que le classificateur traite comme Claude changeant ses propres permissions ou surveillance

Claude Code v2.1.200 et ultérieur bloquent également ceux-ci par défaut :

* Commentaire, suppression, ou passage forcé d'un test ou d'une assertion qui protège le comportement de sécurité, comme l'authentification, le contrôle d'accès, la validation d'entrée, ou le sandboxing
* Suppression ou démantèlement d'une ressource avec état que Claude n'a pas créée dans la session, quand aucune règle de suppression plus spécifique ne s'applique et vous n'avez pas nommé cette ressource
* Réorientation d'une URL de base API, d'un point de terminaison proxy, d'un récepteur webhook, ou d'un miroir de registre vers un hôte tiers qui ne correspond pas à la tâche, y compris dans les fichiers d'exemple comme `.env.example`
* Modification de la destination des poussées avec `git remote set-url` ou `git remote add`, sauf si vous avez nommé le nouveau remote
* Poussée de secrets ou de données personnelles ou confiées vers un référentiel connu pour être public, ou poussée de matériel confidentiel là-bas qui ne fait pas partie du travail propre de ce référentiel. La matière propre d'un référentiel de dotfiles est la seule exception pour les données personnelles ou confiées, et le contenu d'un référentiel privé atteignant toute surface publique est bloqué de la même manière ; les deux raffinements nécessitent Claude Code v2.1.203 ou ultérieur. Avant v2.1.203, les données personnelles étaient groupées avec le matériel confidentiel et bloquées uniquement quand elles ne faisaient pas partie du travail propre de ce référentiel. Quand la visibilité d'un référentiel n'est pas établie, le classificateur ne bloque pas sur cela seul ; il juge le contenu par rapport aux autres règles à la place
* Ouverture d'une demande de tirage contre un référentiel ou une organisation différente, fork avec `gh repo fork`, ou poussée vers un référentiel tiers, sauf si vous avez nommé cette cible externe

Claude Code v2.1.203 et ultérieur bloquent également ceux-ci par défaut :

* Contenu d'un magasin local sensible, ou d'un fichier dont le nom, le chemin, ou le type le marque comme sensible, entrant dans un commit, une poussée, un texte de PR ou de problème, une gist ou un paste, ou une publication de package, sauf si vous avez nommé à la fois la source et la destination. Les transcriptions de session et les journaux de conversation, les dossiers de points de credential et de configuration tels que les clés SSH, les credentials cloud, les profils de navigateur, et l'historique du shell, et les exports de données utilisateur comptent tous, et le référentiel étant privé ne l'efface pas

Claude Code v2.1.205 et ultérieur bloquent également ceux-ci par défaut :

* Écriture dans les transcriptions de session Claude Code, les fichiers d'historique `.jsonl` sous `~/.claude/projects/` ou votre répertoire de configuration configuré, directement ou via une commande shell. La règle couvre également les lignes de métadonnées que Claude Code ajoute à chaque entrée de transcription pour ses propres vérifications. Une transcription est l'état de la session que Claude Code écrit, pas un fichier de travail, et une entrée falsifiée atteint chaque vérification ultérieure une fois que vous reprenez la session, donc le mode auto bloque ces écritures comme défense en profondeur. La lecture d'une transcription n'est pas bloquée
* Une suppression forcée récursive comme `rm -rf "$VAR"` ou `Remove-Item -Recurse -Force $dir` dont la cible est une variable shell, ou un glob enraciné à une, qui n'est assignée nulle part dans la conversation que le classificateur voit. La valeur provenait uniquement de la sortie de commande antérieure, que le classificateur ne reçoit jamais, donc le classificateur ne peut pas vérifier la cible de suppression par rapport aux autres règles de suppression. Le classificateur lit la conversation plutôt que la sortie de commande par conception, donc il bloque l'appel au lieu de deviner la cible. Le bloc s'efface quand vous nommez le chemin exact en cours de suppression, ou quand Claude réexécute la suppression avec le chemin littéral résolu écrit dans la commande. Les suppressions dont la cible le classificateur peut résoudre ne sont pas affectées

**Autorisé par défaut** :

* Opérations de fichiers locaux dans votre répertoire de travail
* Installation de dépendances déclarées dans vos fichiers de verrouillage ou manifestes
* Lecture de `.env` et envoi de credentials à leur API correspondante
* Requêtes HTTP en lecture seule
* Poussée vers la branche sur laquelle vous avez commencé ou une que Claude a créée
* Poussées routinières vers la branche par défaut du référentiel. Avant v2.1.203, toute poussée directe vers la branche par défaut était bloquée

Claude Code v2.1.195 et ultérieur autorisent également ceux-ci par défaut :

* Suppression des tâches exactes que Claude a créées plus tôt dans la même session
* Lecture, examen, ou écriture de code, configs, et modèles de menace liés à la sécurité dans le cadre de votre tâche
* Messages entre agents travaillant ensemble dans la même session multi-agent
* Envoi de données aux domaines approuvés, buckets, et services que vous répertoriez dans [`environment`](/docs/fr/auto-mode-config#define-trusted-infrastructure). Cela couvre le flux de données uniquement, pas les opérations destructrices ou de credential sur la même infrastructure
* [Claude in Chrome](/docs/fr/chrome) navigation vers un domaine interne approuvé, localhost, ou une URL que vous avez nommée

Les demandes d'accès réseau Sandbox sont acheminées via le classificateur plutôt que d'être autorisées par défaut. À partir de v2.1.198, le classificateur réutilise son verdict pour un hôte et un port réseau au lieu de réexécuter à chaque connexion :

* Un allow est réutilisé jusqu'à ce que du nouveau contenu entre dans la conversation, auquel point cet hôte est vérifié à nouveau
* Dans le CLI interactif, un deny est supprimé quand le tour se termine
* En [mode non-interactif](/docs/fr/headless) et les sessions Agent SDK il n'y a pas de limite de tour, donc un deny est réutilisé pour le reste de l'exécution
* Changer votre mode de permission ou vos règles supprime tous les verdicts en cache

Exécutez `claude auto-mode defaults` pour voir les listes de règles complètes. Si les actions routinières sont bloquées, un administrateur peut ajouter des référentiels approuvés, des buckets, et des services via le paramètre `autoMode.environment` : voir [Configurer le mode auto](/docs/fr/auto-mode-config).

Poussée vers votre branche de travail, poussée routinière vers la branche par défaut du référentiel, et création d'une demande de tirage qui correspond à votre demande s'exécutent tous sans invite. Le classificateur bloque une poussée uniquement quand elle porte un risque, comme une force push ou du contenu qui contourne un examen que vous avez mis en place. Pour exiger un point de contrôle humain avant ces actions tout en restant en mode auto, ajoutez des règles `permissions.ask` : voir [Limites communes](/docs/fr/auto-mode-config#common-boundaries).

<h3 id="boundaries-you-state-in-conversation">
  Limites que vous énoncez dans la conversation
</h3>

Le classificateur traite les limites que vous énoncez dans la conversation comme un signal de blocage. Si vous dites à Claude « ne pousse pas » ou « attends que j'examine avant de déployer », le classificateur bloque les actions correspondantes même quand les règles par défaut les autoriseraient. Une limite reste en vigueur jusqu'à ce que vous la leviez dans un message ultérieur. Le propre jugement de Claude qu'une condition a été remplie ne la lève pas.

Les limites ne sont pas stockées comme des règles. Le classificateur les relit à partir de la transcription à chaque vérification, donc une limite peut être perdue si la [compaction de contexte](/docs/fr/costs#reduce-token-usage) supprime le message qui l'a énoncée. Pour une garantie ferme, ajoutez plutôt une [règle deny](/docs/fr/permissions#permission-rule-syntax).

<h3 id="when-auto-mode-falls-back">
  Quand le mode auto se replie
</h3>

Chaque action refusée affiche une notification et apparaît dans `/permissions` sous l'onglet Recently denied, où vous pouvez appuyer sur `r` pour la réessayer avec une approbation manuelle.

Si le classificateur bloque une action 3 fois de suite ou 20 fois au total, le mode auto s'interrompt et Claude Code reprend l'invite. L'approbation de l'action invitée reprend le mode auto. Ces seuils ne sont pas configurables. Toute action autorisée réinitialise le compteur consécutif, tandis que le compteur total persiste pour la session et se réinitialise uniquement quand sa propre limite déclenche un repli.

En [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, les blocs répétés abandonnent la session puisqu'il n'y a pas d'utilisateur à inviter.

Les blocs répétés signifient généralement que le classificateur manque de contexte sur votre infrastructure. Utilisez `/feedback` pour signaler les faux positifs, ou demandez à un administrateur de [configurer l'infrastructure approuvée](/docs/fr/auto-mode-config).

<AccordionGroup>
  <Accordion title="Comment le classificateur évalue les actions">
    Chaque action passe par un ordre de décision fixe. La première étape correspondante gagne :

    1. Les actions correspondant à vos [règles allow, ask, ou deny](/docs/fr/permissions#manage-permissions) se résolvent immédiatement. Les écritures vers les [chemins protégés](#protected-paths) sont acheminées vers le classificateur même quand une règle allow correspond. Les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) vous invitent directement même quand une règle allow correspond. Les règles ask limitées au contenu se replient sur une invite de permission
    2. Les actions en lecture seule et les éditions de fichiers dans votre répertoire de travail sont auto-approuvées, sauf les écritures vers les [chemins protégés](#protected-paths)
    3. Tout le reste va au classificateur. Un outil connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) saute le classificateur et vous invite directement, donc une approbation requise par l'organisation n'est jamais auto-approuvée. À partir de v2.1.199, un outil MCP marqué avec [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) saute également le classificateur et vous invite directement, donc une étape de consentement n'est jamais auto-approuvée au nom de l'auteur de l'outil
    4. Si le classificateur bloque, Claude reçoit la raison et essaie une alternative

    En entrant en mode auto, les règles allow larges qui accordent l'exécution de code arbitraire sont supprimées :

    * Blanket `Bash(*)` ou `PowerShell(*)`
    * Interpréteurs avec wildcard comme `Bash(python*)`
    * Commandes d'exécution du gestionnaire de packages
    * Règles `Agent` allow

    Les règles étroites comme `Bash(npm test)` sont conservées. Les règles supprimées sont restaurées quand vous quittez le mode auto.

    Le classificateur voit les messages utilisateur, les appels d'outils, et votre contenu CLAUDE.md. Les résultats d'outils sont supprimés, donc le contenu hostile dans un fichier ou une page web ne peut pas le manipuler directement. Une sonde côté serveur distincte analyse les résultats d'outils entrants et signale le contenu suspect avant que Claude ne le lise. Pour plus d'informations sur la façon dont ces couches fonctionnent ensemble, voir l'[annonce du mode auto](https://claude.com/blog/auto-mode) et la [plongée technique d'ingénierie](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Comment le mode auto gère les sous-agents">
    Le classificateur vérifie le travail des [sous-agents](/docs/fr/sub-agents) à trois points :

    1. Avant qu'un sous-agent ne démarre, la description de la tâche déléguée est évaluée, donc une tâche qui semble dangereuse est bloquée au moment du lancement.
    2. Pendant que le sous-agent s'exécute, chacune de ses actions passe par le classificateur avec les mêmes règles que la session parent, et tout `permissionMode` dans le frontmatter du sous-agent est ignoré.
    3. Quand le sous-agent se termine, le classificateur examine son historique d'action complet ; si cette vérification de retour signale une préoccupation, un avertissement de sécurité est ajouté au début des résultats du sous-agent.

    L'étape 1 nécessite Claude Code v2.1.178 ou ultérieur. Les versions antérieures appliquaient le classificateur aux étapes 2 et 3, mais n'évaluaient pas la description de la tâche avant le démarrage du sous-agent.
  </Accordion>

  <Accordion title="Coût et latence">
    Le classificateur s'exécute sur un modèle configuré par le serveur qui est indépendant de votre sélection `/model`, donc changer de modèle ne change pas la disponibilité du classificateur. Les appels du classificateur comptent vers votre utilisation de tokens. Chaque vérification envoie une portion de la transcription plus l'action en attente, ajoutant un aller-retour avant l'exécution. Les lectures et les éditions de répertoire de travail en dehors des chemins protégés sautent le classificateur, donc la surcharge provient principalement des commandes shell et des opérations réseau. À partir de v2.1.198, un verdict réseau sandbox pour un hôte et un port est réutilisé au lieu d'être reclassifié à chaque connexion, donc les connexions répétées au même hôte n'ajoutent pas chacune une vérification. [Ce que le classificateur bloque par défaut](#what-the-classifier-blocks-by-default) décrit combien de temps un allow et un deny durent.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Autoriser uniquement les outils pré-approuvés avec le mode dontAsk
</h2>

Si vous définissez le mode `dontAsk`, Claude Code refuse automatiquement chaque appel d'outil qui déclencherait autrement une invite. Claude exécute uniquement les actions correspondant à vos règles `permissions.allow`, les [commandes Bash en lecture seule](/docs/fr/permissions#read-only-commands) et les appels approuvés par un [hook PreToolUse](/docs/fr/permissions#extend-permissions-with-hooks). Utilisez ce mode pour les pipelines CI ou les environnements restreints où vous prédéfinissez exactement ce que Claude peut faire ; la session n'attend jamais d'entrée. La barre d'état affiche `⏵⏵ don't ask on` tandis que ce mode est actif.

Claude Code refuse les appels correspondant à vos [règles `ask` explicites](/docs/fr/permissions#manage-permissions) plutôt que de déclencher une invite. Il refuse également l'outil intégré `AskUserQuestion` et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), même si vos règles allow les correspondent. Il refuse les outils MCP marqués [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) de la même manière, car leur carte d'approbation nécessite une réponse que ce mode ne collecte jamais ; cela nécessite Claude Code v2.1.199 ou ultérieur.

Les sessions cloud sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ignorent `defaultMode: "dontAsk"` ; consultez [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) pour plus de détails.

Définissez-le au démarrage avec le drapeau :

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Ignorer tous les contrôles avec le mode bypassPermissions
</h2>

Le mode `bypassPermissions` désactive les invites de permission et les contrôles de sécurité afin que les appels d'outils s'exécutent immédiatement, y compris les écritures vers les [chemins protégés](#protected-paths). Avant la v2.1.126, les écritures vers les chemins protégés invitaient toujours dans ce mode.

Les [règles ask](/docs/fr/permissions#manage-permissions) explicites et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) forcent toujours une invite dans ce mode. Les outils MCP marqués avec [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) invitent également toujours ; cela nécessite Claude Code v2.1.199 ou une version ultérieure.

Les suppressions ciblant la racine du système de fichiers ou le répertoire personnel, telles que `rm -rf /` et `rm -rf ~`, invitent toujours comme disjoncteur contre les erreurs du modèle. Le disjoncteur s'active également lorsque la commande contient une substitution de commande avec `$(...)` ou des backticks, ou une substitution de processus avec `<(...)`, que la suppression se trouve à l'intérieur de la substitution, comme dans `echo "$(rm -rf ~)"`, ou ailleurs dans la même commande. La forme simple, tapée comme sa propre commande, a invité dans ce mode depuis l'introduction du disjoncteur ; avant la v2.1.208, les commandes contenant ces formes n'invitaient pas.

<Warning>
  Utilisez ce mode uniquement dans des environnements isolés comme les conteneurs, les machines virtuelles ou les dev containers sans accès à Internet, où Claude Code ne peut pas endommager votre système hôte.
</Warning>

Vous ne pouvez pas entrer dans `bypassPermissions` à partir d'une session qui a été démarrée sans l'un des drapeaux d'activation ; redémarrez avec l'un d'eux pour l'activer :

```bash theme={null}
claude --permission-mode bypassPermissions
```

Le drapeau `--dangerously-skip-permissions` est équivalent.

Sur Linux et macOS, Claude Code refuse de démarrer dans ce mode lors de l'exécution en tant que root ou sous `sudo` :

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

La vérification est ignorée automatiquement à l'intérieur d'un sandbox reconnu. Pour s'exécuter de manière autonome dans un conteneur, utilisez la configuration du [dev container](/docs/fr/devcontainer), qui exécute Claude Code en tant qu'utilisateur non-root.

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) n'honore pas `defaultMode: "bypassPermissions"` ou `"dontAsk"` de vos fichiers de paramètres, donc les paramètres archivés d'un référentiel ne peuvent pas démarrer une session cloud en mode bypass-permissions. Le paramètre est ignoré silencieusement et la session démarre dans le mode affiché dans la liste déroulante des modes à la place. Consultez [Changer les modes de permission](#switch-permission-modes) pour connaître les modes que les sessions cloud proposent.

<Warning>
  `bypassPermissions` n'offre aucune protection contre l'injection de prompt ou les actions involontaires. Pour les contrôles de sécurité en arrière-plan avec beaucoup moins d'invites de permission, utilisez le [mode auto](#eliminate-prompts-with-auto-mode) à la place. Les administrateurs peuvent bloquer ce mode en définissant `permissions.disableBypassPermissionsMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/permissions#managed-settings).
</Warning>

<h2 id="protected-paths">
  Chemins protégés
</h2>

Les écritures vers un petit ensemble de chemins ne sont jamais approuvées automatiquement, dans tous les modes sauf `bypassPermissions`. Cela empêche la corruption accidentelle de l'état du référentiel et de la configuration propre de Claude.

| Mode                             | Écritures de chemins protégés   |
| :------------------------------- | :------------------------------ |
| `default`, `acceptEdits`, `plan` | Invité                          |
| `auto`                           | Acheminé vers le classificateur |
| `dontAsk`                        | Refusé                          |
| `bypassPermissions`              | Autorisé                        |

Les règles [`permissions.allow`](/docs/fr/permissions#manage-permissions) dans les fichiers de paramètres n'approuvent pas préalablement les écritures de chemins protégés. La vérification de sécurité s'exécute avant que Claude Code n'évalue les règles allow des paramètres, donc une entrée telle que `Edit(.claude/**)` dans `~/.claude/settings.json` ou `.claude/settings.json` ne change pas le résultat par mode dans le tableau ci-dessus. Dans les modes qui invitent, l'invite pour une écriture `.claude/` offre **Oui, et autoriser Claude à modifier ses propres paramètres pour cette session**, ce qui approuve les écritures `.claude/` ultérieures dans cette session sans inviter à nouveau.

Répertoires protégés :

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, sauf pour `.claude/worktrees` où Claude stocke ses propres git worktrees

Fichiers protégés :

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="see-also">
  Voir aussi
</h2>

* [Permissions](/docs/fr/permissions) : règles allow, ask et deny ; politiques gérées
* [Configurer le mode auto](/docs/fr/auto-mode-config) : indiquez au classificateur l'infrastructure de confiance de votre organisation
* [Hooks](/docs/fr/hooks) : logique de permission personnalisée via les hooks `PreToolUse` et `PermissionRequest`
* [Ultraplan](/docs/fr/ultraplan) : exécutez le mode plan dans une session Claude Code sur le web avec révision basée sur le navigateur
* [Sécurité](/docs/fr/security) : protections et bonnes pratiques
* [Sandboxing](/docs/fr/sandboxing) : isolation du système de fichiers et du réseau pour les commandes Bash
* [Mode non-interactif](/docs/fr/headless) : exécutez Claude Code avec le flag `-p`
