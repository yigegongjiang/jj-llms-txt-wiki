> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déployer les paramètres gérés

> Déployez les paramètres gérés sur la machine de chaque développeur : mécanismes de livraison par système d'exploitation, comment Claude Code combine les sources gérées, et comment vérifier l'application.

Les paramètres gérés sont les paramètres que votre organisation déploie sur la machine de chaque développeur. Claude Code les applique au-dessus de tous les autres niveaux, donc aucune valeur utilisateur, projet, locale ou `--settings` ne peut les remplacer, à l'exception de quelques [exceptions sensibles à la sécurité](/docs/fr/settings#exceptions-to-managed-settings-precedence) où une valeur plus stricte d'un niveau inférieur compte toujours.

Cette page s'adresse à l'administrateur qui déploie les paramètres gérés ou qui débogue pourquoi l'un d'eux ne s'applique pas. Pour décider ce qu'il faut appliquer, commencez par le tableau [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce). Pour le chemin de la console claude.ai, consultez [Paramètres gérés par le serveur](/docs/fr/server-managed-settings). Pour savoir dans quel fichier les propres valeurs d'un développeur vont, consultez [Paramètres](/docs/fr/settings).

<h2 id="deploy-a-managed-settings-file">
  Déployer un fichier de paramètres gérés
</h2>

C'est le moyen le plus rapide de mettre une politique sur chaque machine : un fichier `managed-settings.json`. Si vous n'avez pas encore choisi comment livrer les paramètres gérés, ou si vos appareils sont sous MDM ou si les développeurs exécutent des sessions cloud, lisez d'abord [Choisir un mécanisme de livraison](#choose-a-delivery-mechanism).

<Steps>
  <Step title="Écrire managed-settings.json">
    Écrivez un `managed-settings.json` qui contient les clés que vous avez décidé d'appliquer, dans la même forme JSON que `settings.json`. Le tableau [Décider ce qu'il faut appliquer](/docs/fr/admin-setup#decide-what-to-enforce) énumère les clés derrière chaque contrôle, et chaque entrée dans la [référence des paramètres](/docs/fr/settings-reference) indique si une source gérée peut la définir. Ce fichier bloque deux lectures de fichiers, désactive le mode de contournement, et fait que Claude Code ignore les règles de permission des fichiers utilisateur, projet et local et de `--allowedTools` :

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Pour un exemple plus complet qui montre la forme de plus de clés gérées, y compris la méthode de connexion, les modèles, les serveurs MCP et les places de marché, consultez [Les paramètres gérés d'une organisation](/docs/fr/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Placer le fichier sur chaque machine">
    Enregistrez le fichier sous le nom `managed-settings.json` dans le répertoire système du système d'exploitation, en utilisant les outils que vous utilisez déjà pour placer des fichiers sur votre parc :

    * **macOS** : `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux et WSL** : `/etc/claude-code/managed-settings.json`
    * **Windows** : `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Confirmer que la politique a été appliquée">
    Sur une machine, exécutez `/status` dans Claude Code. La ligne `Setting sources` affiche `Enterprise managed settings (file)`. Déployez sur le reste du parc après cela ; [Vérifier qu'une politique est en vigueur](#check-that-a-policy-is-in-force) couvre ce qu'il faut regarder quand la ligne est manquante.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Choisir un mécanisme de livraison
</h2>

Le fichier dans les étapes ci-dessus est l'un des quatre moyens de mettre les paramètres gérés sur une machine. Chaque mécanisme porte les mêmes clés de politique qu'un fichier `settings.json`, donc la [référence des paramètres](/docs/fr/settings-reference) s'applique à tous. Quelques clés sont liées à des sources particulières, et la ligne Scope de chaque entrée indique lesquelles :

* **Contrôles de livraison** : [`policyHelper`](/docs/fr/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings), et [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior)
* **Clés de connexion à la passerelle** : [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/fr/settings-reference#gatewayinternalnetworks), et la valeur `"gateway"` de [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod)

Un fichier de paramètres gérés, un profil MDM, ou la console claude.ai applique une politique à tous ceux qu'il atteint. Pour donner à un groupe de développeurs une politique différente, déployez un fichier ou un profil différent à ce groupe ; la console claude.ai [ne peut pas encore cibler un groupe](/docs/fr/server-managed-settings#current-limitations), tandis qu'une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée livre les paramètres gérés par groupe IdP.

Quand plus d'un mécanisme livre une politique à la même machine, Claude Code utilise par défaut l'un et ignore les autres. [Comment Claude Code combine les sources gérées](#how-claude-code-combines-managed-sources) donne l'ordre et l'opt-in qui s'applique à chaque source.

Les lignes MDM et fichier sont ensemble appelées paramètres gérés par le point de terminaison, car la politique est stockée sur l'appareil du développeur, par opposition à la ligne gérée par le serveur, où Claude Code la récupère.

Choisissez un mécanisme selon la façon dont vous gérez déjà les appareils, en utilisant le tableau ci-dessous.

| Mécanisme                                                      | Comment vous le livrez                                                                                                                                                                                                                | Quand Claude Code le lit                                                                                                                                                                                                                                                               | Utilisez-le quand                                                                                                   |
| :------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) | Dans la console d'administration claude.ai, ou sur une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée                                                                                                      | Récupérés au démarrage et interrogés toutes les heures ; consultez [les modifications qui nécessitent une approbation](#where-and-when-a-policy-applies)                                                                                                                               | Vous voulez un seul endroit pour changer la politique pour une organisation claude.ai sans toucher à chaque machine |
| Politique MDM ou au niveau du système d'exploitation           | Comme un profil de configuration macOS ou une valeur de registre Windows `HKLM`, via Jamf, Intune, Group Policy, ou un outil similaire ; consultez [où chaque mécanisme stocke la politique](#where-each-mechanism-stores-the-policy) | Lus au démarrage et vérifiés pour les modifications toutes les 30 minutes                                                                                                                                                                                                              | Vous gérez déjà les appareils avec MDM ou Group Policy                                                              |
| Basé sur fichier                                               | Comme `managed-settings.json` dans un répertoire système sur chaque machine ; consultez [où chaque mécanisme stocke la politique](#where-each-mechanism-stores-the-policy)                                                            | Lus au démarrage et rechargés quand un fichier change                                                                                                                                                                                                                                  | Machines sans MDM, hôtes Linux, ou images que vous construisez vous-même                                            |
| Registre HKCU, Windows et WSL                                  | Comme une valeur de registre Windows `HKCU` ; consultez [où chaque mécanisme stocke la politique](#where-each-mechanism-stores-the-policy)                                                                                            | Lus au démarrage et vérifiés pour les modifications toutes les 30 minutes ; Claude Code ne l'utilise que quand aucune autre source gérée ne livre une clé de politique et aucun [paramètre parent fourni par l'hôte](#let-an-embedding-host-add-policy) ne fournit une clé restrictive | Vous ne pouvez pas écrire la clé au niveau de la machine `HKLM`                                                     |

Les modèles de démarrage pour Jamf, Iru, Intune et Group Policy se trouvent dans le [référentiel d'exemples MDM](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Pour les serveurs MCP gérés, que vous déployez aux côtés de l'un de ceux-ci via `managed-mcp.json` ou que vous fournissez via la clé [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers), consultez [Configuration MCP gérée](/docs/fr/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Où et quand une politique s'applique
</h3>

Une politique déployée atteint les sessions du développeur comme suit :

* **Surfaces** : sur la machine du développeur, le terminal, les extensions VS Code et JetBrains, l'onglet Code de l'application de bureau, et les sessions [Agent SDK](/docs/fr/agent-sdk/typescript) lisent toutes ces sources. Les sessions Agent SDK chargent les paramètres gérés même quand `settingSources` exclut les fichiers utilisateur, projet et local.
* **Sessions cloud** : une session dans un environnement hébergé par Anthropic ne lit pas un profil MDM ou un fichier d'appareil, donc la politique pour cela doit provenir des paramètres gérés par le serveur. Une session dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments) lit également le fichier de paramètres gérés dans son image de runner, par défaut uniquement quand les paramètres gérés par le serveur ne livrent aucune clé de politique, à l'exception des [clés que Claude Code lit de chaque source d'administration](#keys-read-from-every-admin-source). [Comment Claude Code combine les sources gérées](#how-claude-code-combines-managed-sources) couvre l'opt-in qui s'applique aux deux.
* **Sessions Cowork** : [Cowork](https://claude.com/docs/cowork/overview) dans l'application Claude Desktop exécute ses sessions sur Claude Code. Dans une session Cowork, Claude Code ne récupère jamais les paramètres gérés par le serveur de la console d'administration claude.ai, même quand l'utilisateur se connecte avec un compte Team ou Enterprise, donc la politique qui s'applique dépend de l'endroit où la session s'exécute :

  * **Sur la machine de l'utilisateur** : par défaut, Claude Code dans une session Cowork lit la politique MDM ou au niveau du système d'exploitation et le fichier de paramètres gérés sur cet appareil, donc déployez la politique là.
  * **Dans un sandbox VM complet** : quand votre configuration gérée Claude Desktop définit [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox), Claude Code s'exécute à l'intérieur d'une machine virtuelle où la politique MDM de l'appareil et le fichier de paramètres gérés ne sont pas présents.
  * **Sessions Cowork distantes** : celles-ci s'exécutent sur des machines virtuelles gérées par Anthropic, où Claude Code n'a pas de politique d'appareil à lire.

  Où que la session s'exécute, claude.ai applique les listes [`strictKnownMarketplaces`](/docs/fr/settings-reference#strictknownmarketplaces) et [`blockedMarketplaces`](/docs/fr/settings-reference#blockedmarketplaces) de la console d'administration elle-même quand quelqu'un ajoute une marketplace à partir d'un référentiel git sur claude.ai ou à partir de **Personnaliser** dans l'onglet Cowork. [Comment fonctionnent les restrictions](/docs/fr/plugins/org#restrict-what-users-can-install) décrit cette vérification. Le tableau [couverture de surface](/docs/fr/model-config#surface-coverage) compare Cowork avec les autres surfaces.
* **Sessions en cours d'exécution** : la plupart des modifications atteignent une session en cours d'exécution selon le calendrier du tableau de [mécanisme de livraison](#choose-a-delivery-mechanism), sans redémarrage.
  * Les modifications apportées à [`forceRemoteSettingsRefresh`](/docs/fr/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/fr/settings-reference#requiredminimumversion), et [certaines clés modifiables par l'utilisateur](/docs/fr/settings#when-edits-take-effect) prennent effet au prochain démarrage de session.
  * Une entrée [`policyHelper`](/docs/fr/settings-reference#policyhelper) nouvelle ou modifiée prend effet au prochain lancement. Si les paramètres gérés par le serveur masquent l'assistant à ce lancement, l'assistant s'exécute dès qu'une récupération signale que ces paramètres ont été supprimés.
* **Modifications qui nécessitent une approbation** : à part les [mises à jour qui attendent le prochain lancement](/docs/fr/server-managed-settings#fetch-and-caching-behavior), une modification gérée par le serveur d'un paramètre qui [nécessite une approbation](/docs/fr/server-managed-settings#security-approval-dialogs), comme un hook ou une variable `env`, attend que le développeur accepte la boîte de dialogue dans une session interactive, et s'applique pour l'exécution actuelle dans une session qu'une extension IDE ou l'Agent SDK héberge. Les autres modifications gérées par le serveur s'appliquent au prochain sondage.
* **Sessions longue durée** : une session laissée ouverte pendant des semaines peut toujours être en retard sur un déploiement. [`requiredMinimumVersion`](/docs/fr/settings-reference#requiredminimumversion) bloque un binaire obsolète de démarrer et ne termine pas une session qui s'exécute déjà.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Où chaque mécanisme stocke la politique
</h3>

Les clés sont les mêmes partout, mais chaque mécanisme les stocke dans un endroit et une forme différents :

* **Gérée par le serveur** : les serveurs d'Anthropic, ou votre passerelle, détiennent la politique. Claude Code conserve un cache local qu'il applique au démarrage et [remplace à chaque récupération réussie](/docs/fr/server-managed-settings#security-considerations).
* **Profil de configuration macOS** : le domaine des préférences gérées `com.anthropic.claudecode`. Utilisez les mêmes clés de niveau supérieur que `managed-settings.json`, avec les paramètres imbriqués comme dictionnaires et les listes comme tableaux plist.
* **Registre Windows HKLM** : le JSON comme valeur `REG_SZ` ou `REG_EXPAND_SZ` nommée `Settings` sous `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **Basé sur fichier** : `managed-settings.json`, un répertoire optionnel `managed-settings.d/`, et `managed-mcp.json` dans le répertoire système : `/Library/Application Support/ClaudeCode/` sur macOS, `/etc/claude-code/` sur Linux et WSL, et `C:\Program Files\ClaudeCode\` sur Windows. Claude Code ne lit pas le chemin Windows hérité `C:\ProgramData\ClaudeCode\managed-settings.json`.
* **Registre Windows HKCU** : la même valeur `Settings` sous `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Diviser une politique basée sur fichier entre les équipes
</h3>

Si plusieurs équipes possèdent des parties d'une politique, mettez chaque partie dans son propre fichier dans `managed-settings.d/`, à côté de `managed-settings.json` dans le même répertoire système, au lieu de modifier un fichier partagé.

Claude Code fusionne d'abord `managed-settings.json`, puis chaque fichier `*.json` du répertoire dans l'ordre alphabétique. Nommez les fichiers avec des préfixes numériques pour contrôler l'ordre, comme `10-telemetry.json` et `20-security.json`. Claude Code ignore les fichiers cachés et les fichiers qui ne se terminent pas par `.json`.

Quand deux fichiers définissent la même clé, Claude Code les combine selon ces règles :

* **Valeurs uniques**, comme `"model": "opus"` ou `"cleanupPeriodDays": 7` : la valeur du fichier ultérieur remplace celle du fichier antérieur
* **Listes**, comme `permissions.deny` ou `sandbox.network.allowedDomains` : les deux listes se combinent, avec les doublons supprimés
* **Blocs imbriqués**, comme `env` ou `sandbox` : les deux blocs fusionnent clé par clé, et chaque clé à l'intérieur suit ces mêmes règles
* **`fallbackModel`** : la chaîne ultérieure remplace entièrement la chaîne antérieure
* **[`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) et [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers)** : une entrée ultérieure avec le même nom remplace entièrement celle antérieure
* **[`modelPicker`](/docs/fr/settings-reference#modelpicker)** : la lineup ultérieure remplace entièrement la lineup antérieure

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Comment Claude Code combine les sources gérées
</h2>

Quand votre organisation livre plus d'une source gérée à la même machine, la clé [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) décide ce que Claude Code fait avec les autres :

* **`"first-wins"`, la valeur par défaut** : Claude Code utilise la source la mieux classée qui livre au moins une clé de politique et ignore le reste plutôt que de les fusionner, à l'exception des clés dans [Clés lues de chaque source d'administration](#keys-read-from-every-admin-source). Claude Code n'affiche aucun avertissement pour les sources qu'il ignore ; `/status` [nomme la source qu'il a utilisée et celles qu'il a ignorées](#read-the-source-in-/status).
* **`"merge"`** : Claude Code applique chaque source d'administration qui livre une clé de politique et les combine par type de clé : sur la plupart des clés, la valeur de la source la mieux classée s'applique, les listes s'unissent, et les verrous prennent la valeur la plus stricte. [Composer chaque source gérée](#compose-every-managed-source) dit où définir la clé et comment chaque type de clé se combine. Nécessite Claude Code v2.1.242 ou ultérieur.

Les deux paramètres classent les sources de la même manière. Deux termes reviennent dans cette section :

* **Clé de politique** : toute clé de paramètres autre que les deux clés de contrôle, [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings) et [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior). Un fichier de paramètres gérés ou une politique MDM qui contient uniquement ceux-ci ne compte pas, et Claude Code passe à la source suivante.
* **Source d'administration** : l'une des trois premières sources ci-dessous. Le registre HKCU modifiable par l'utilisateur n'en est pas une.

Claude Code vérifie les sources dans cet ordre, priorité la plus élevée en premier :

1. Paramètres distants, livrés de claude.ai comme [paramètres gérés par le serveur](/docs/fr/server-managed-settings) ou par une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway). Claude Code récupère cette source uniquement quand la session s'authentifie à l'API d'Anthropic directement avec une [connexion ou clé éligible](/docs/fr/server-managed-settings#platform-availability), ou se connecte à une passerelle avec `/login`. Sur d'autres fournisseurs, ou quand `ANTHROPIC_BASE_URL` pointe ailleurs que l'API d'Anthropic, il commence à la source suivante
2. Politiques MDM ou au niveau du système d'exploitation : le plist macOS ou la clé de registre HKLM
3. Fichiers de paramètres gérés, `managed-settings.d/*.json` et `managed-settings.json` fusionnés ensemble
4. Le registre HKCU, sur Windows, et sur WSL une fois que le registre HKLM ou le fichier de paramètres gérés Windows active [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings) et la valeur HKCU le définit également. Claude Code ne le lit que quand aucune source au-dessus ne livre une clé de politique et aucun [paramètre parent fourni par l'hôte](#let-an-embedding-host-add-policy) ne fournit une clé restrictive

Ce diagramme montre le classement, avec des exemples des clés inter-sources que Claude Code lit des trois premières sources sous l'un ou l'autre paramètre :

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagramme montrant les quatre sources de paramètres gérés classées des paramètres distants en haut jusqu'à MDM, fichiers de paramètres gérés, et le registre HKCU en bas. Par défaut, la première source avec une clé de politique fournit la politique et le reste est ignoré ; avec managedSourcesBehavior défini sur merge, chaque source d'administration avec une clé de politique contribue, combinée par type de clé, et le registre HKCU reste en dehors. Un panneau latéral montre que les clés inter-sources telles que les verrous sandbox, forceRemoteSettingsRefresh, et la fusion env par variable sont lues de chaque source d'administration, ce qui exclut le registre HKCU." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagramme montrant les quatre sources de paramètres gérés classées des paramètres distants en haut jusqu'à MDM, fichiers de paramètres gérés, et le registre HKCU en bas. Par défaut, la première source avec une clé de politique fournit la politique et le reste est ignoré ; avec managedSourcesBehavior défini sur merge, chaque source d'administration avec une clé de politique contribue, combinée par type de clé, et le registre HKCU reste en dehors. Un panneau latéral montre que les clés inter-sources telles que les verrous sandbox, forceRemoteSettingsRefresh, et la fusion env par variable sont lues de chaque source d'administration, ce qui exclut le registre HKCU." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Clés lues de chaque source d'administration
</h3>

Sous le paramètre par défaut `"first-wins"`, Claude Code lit la plupart des clés uniquement de la [source qu'il a sélectionnée](#how-claude-code-combines-managed-sources), et ignore une valeur dans une source de rang inférieur même quand la source sélectionnée laisse cette clé non définie.

Quelques clés fonctionnent différemment. Claude Code les lit de chaque source d'administration, donc une politique MDM ou un fichier de paramètres gérés de rang inférieur peut toujours les définir quand la source sélectionnée ne le fait pas. Claude Code laisse le registre HKCU modifiable par l'utilisateur en dehors de cette analyse ; quand HKCU est la seule source et qu'aucun hôte ne fournit de paramètres parent, HKCU s'applique comme n'importe quelle source sélectionnée.

Les clés inter-sources incluent :

* `sandbox.network.allowManagedDomainsOnly` et `sandbox.filesystem.allowManagedReadPathsOnly` : un `true` dans n'importe quelle source d'administration active le verrou. Pendant qu'un verrou est actif, Claude Code unit la liste d'autorisation qu'il verrouille, `sandbox.network.allowedDomains` ensemble avec les règles d'autorisation `WebFetch(domain:...)`, ou `sandbox.filesystem.allowRead`, de chaque source d'administration. Sans le verrou, Claude Code traite la liste d'autorisation comme n'importe quelle autre clé, donc sous `"first-wins"` la liste d'autorisation d'une source d'administration non sélectionnée est ignorée
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly` : un `true` dans n'importe quelle source d'administration active le verrou de liste d'autorisation MCP. Pendant que le verrou est actif, la liste `allowedMcpServers` gérée provient de la source d'administration la mieux classée qui en définit une. Une liste gérée par le serveur remplace la liste d'une source inférieure plutôt que de se combiner avec elle.

  Si aucune source d'administration ne définit une liste, chaque serveur qui passe la liste de refus se charge, à moins que les [paramètres parent](#let-an-embedding-host-add-policy) ne fournissent une liste.

  Sans le verrou, Claude Code lit `allowedMcpServers` de la source gérée qu'il applique, donc sous `"first-wins"` la liste d'une source d'administration non sélectionnée est ignorée. Nécessite Claude Code v2.1.273 ou ultérieur
* `deniedMcpServers` et [`disableClaudeAiConnectors`](/docs/fr/settings-reference#disableclaudeaiconnectors) : une entrée ou un `true` dans n'importe quelle source d'administration s'applique. Nécessite Claude Code v2.1.273 ou ultérieur
* Les chemins binaires sandbox `sandbox.bwrapPath` et `sandbox.socatPath`
* Le binaire sandbox `ripgrep`, [`sandbox.ripgrep`](/docs/fr/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` et `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/fr/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/fr/settings-reference#syncclaudeaiskills), et [`syncClaudeAiPlugins`](/docs/fr/settings-reference#syncclaudeaiplugins), où un `false` de n'importe quelle source d'administration désactive le comportement. Un `false` dans les paramètres utilisateur ou local du développeur le désactive également ; chaque clé ne peut que refuser
* [`enableArtifact`](/docs/fr/settings-reference#enableartifact), où un `false` de n'importe quelle source d'administration désactive l'[outil Artifact](/docs/fr/artifacts). Un `false` dans les paramètres utilisateur, projet ou local du développeur le désactive également, et aucune source ne le réactive ; consultez [quelles valeurs de niveau inférieur comptent toujours](/docs/fr/settings#exceptions-to-managed-settings-precedence). Nécessite Claude Code v2.1.242 ou ultérieur
* [`maxEffortLevel`](/docs/fr/settings-reference#maxeffortlevel), où le plafond le plus bas dans n'importe quelle source d'administration s'applique. Si un développeur définit un plafond plus bas dans ses propres paramètres ou avec `--settings`, Claude Code applique celui-ci ; aucune source ne peut augmenter le plafond. Nécessite Claude Code v2.1.267 ou ultérieur
* Une opt-out de commit-trailer dans `attribution`, ou dans le `includeCoAuthoredBy` déprécié, de n'importe quel niveau
* [`forceRemoteSettingsRefresh`](/docs/fr/server-managed-settings)
* `env`, fusionné par variable de chaque source d'administration : chaque variable provient de la source de priorité la plus élevée qui la définit, donc les sources inférieures remplissent les variables que les sources supérieures laissent non définies. Quelques variables suivent leurs propres règles ; [Exceptions par clé de chaque source gérée](/docs/fr/server-managed-settings#per-key-exceptions-across-managed-sources) nomme chacune. Nécessite Claude Code v2.1.223 ou ultérieur. Avant v2.1.223, Claude Code appliquait uniquement le bloc `env` entier de la source sélectionnée

Les [clés de connexion à la passerelle](#choose-a-delivery-mechanism) suivent une règle séparée. Claude Code ne les lit jamais à partir des paramètres gérés par le serveur, donc pendant que les paramètres gérés par le serveur sont la source sélectionnée, la source d'administration la mieux classée sur la machine qui porte une clé de politique les fournit toujours. Une valeur dans une source d'administration classée en dessous de celle-ci, ou dans le registre HKCU, est ignorée.

Quand une source d'administration définit `allowManagedMcpServersOnly` ou une liste `allowedMcpServers` et que cette valeur n'est pas celle en vigueur, `/status` et `claude doctor` nomment cette source et cette clé.

<h3 id="compose-every-managed-source">
  Composer chaque source gérée
</h3>

Pour que Claude Code applique chaque source d'administration que votre organisation livre, définissez [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) sur `"merge"` dans la source la mieux classée que vous déployez. Claude Code lit la clé uniquement de la source la mieux classée qui porte soit la clé soit une clé de politique, donc une source inférieure ne peut pas se faire fusionner avec la source au-dessus, et une machine qui ne reçoit jamais les paramètres gérés par le serveur a besoin de la clé dans son profil MDM également. Le registre HKCU modifiable par l'utilisateur ne fusionne jamais avec une autre source. Nécessite Claude Code v2.1.242 ou ultérieur.

Sous `"merge"`, Claude Code ajoute les entrées de liste d'une source inférieure, comme les règles `permissions.allow` et les hooks, à la politique, donc activez-le uniquement quand chaque source classée en dessous de votre source la plus élevée est sous le contrôle d'un administrateur.

Ce tableau montre comment Claude Code combine chaque type de clé sous `"merge"`. L'entrée [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) nomme chaque clé dans trois des lignes : listes d'autorisation de restriction, valeurs prises entièrement, et clés lues uniquement de la source la mieux classée.

| Type de clé                                        | Comment Claude Code la combine                                                                                                                                                            | Exemples                                                                                                                                       |
| :------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Listes                                             | Combine les entrées de chaque source                                                                                                                                                      | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                                             |
| Verrous                                            | Applique la valeur la plus stricte que n'importe quelle source définit ; une valeur plus souple s'applique uniquement de la source la mieux classée                                       | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                                                     |
| Listes d'autorisation de restriction               | Prend la liste entière de la source la mieux classée qui la définit, sans ajouter d'entrées de sources inférieures                                                                        | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins`, et la chaîne `fallbackModel`                       |
| Valeurs prises entièrement                         | Prend la valeur entière de la source la mieux classée qui la définit, sans combiner d'entrées ou de champs de sources inférieures                                                         | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                                              |
| Serveurs MCP fournis                               | Combine les noms de serveur de chaque source ; quand deux sources définissent le même nom, applique l'entrée entière de la source la mieux classée                                        | `managedMcpServers`                                                                                                                            |
| Clés lues uniquement de la source la mieux classée | Ignore la clé dans chaque source inférieure, même quand la source la mieux classée la laisse non définie                                                                                  | Les aides aux identifiants comme `apiKeyHelper`, les épingles de connexion comme `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode` |
| `env`                                              | Fusionne par variable de chaque source d'administration sous l'un ou l'autre paramètre, comme [Clés lues de chaque source d'administration](#keys-read-from-every-admin-source) le décrit |                                                                                                                                                |
| Chaque autre clé                                   | Prend la valeur de la source la mieux classée qui la définit                                                                                                                              | `model`, `cleanupPeriodDays`                                                                                                                   |

Pour confirmer quelles sources se sont combinées sur une machine, [lisez la ligne `Setting sources` dans `/status`](#read-the-source-in-/status) ; cette section dit ce que chaque étiquette signifie.

<h3 id="compute-the-policy-with-a-helper-program">
  Calculer la politique avec un programme d'aide
</h3>

Un [`policyHelper`](/docs/fr/settings-reference#policyhelper) est un exécutable que votre politique MDM ou fichier de paramètres gérés nomme, et Claude Code l'exécute pour calculer les paramètres gérés au démarrage. Quand la source sélectionnée en configure un et que l'aide émet un objet `managedSettings`, cette sortie change ce que Claude Code lit :

* **L'objet `managedSettings` émis est le seul paramètre géré pour la session**, y compris pour les [clés qu'il lit autrement de chaque source d'administration](#keys-read-from-every-admin-source), à l'exception de [`forceRemoteSettingsRefresh`, qui a sa propre règle de démarrage](/docs/fr/settings-reference#forceremotesettingsrefresh)

Pour savoir quels aides échouent, et ce que Claude Code fait quand l'une le fait, consultez [Défaillances d'aide](/docs/fr/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Laisser un hôte d'intégration ajouter une politique
</h3>

Quand une autre application lance Claude Code, comme Claude Desktop, une extension IDE, ou une application Agent SDK, cet hôte peut passer ses propres paramètres gérés via l'option SDK `managedSettings`. Claude Code appelle ces paramètres parent.

Par défaut, Claude Code ignore les paramètres parent chaque fois qu'une source d'administration est présente : paramètres gérés par le serveur, une politique MDM ou au niveau du système d'exploitation, ou un fichier de paramètres gérés.

Pour que Claude Code fusionne les paramètres parent aux côtés d'une source d'administration, définissez [`parentSettingsBehavior`](/docs/fr/settings-reference#parentsettingsbehavior) sur `"merge"` dans la source gérée de priorité la plus élevée ; Claude Code lit la clé de cette source uniquement.

Claude Code conserve alors uniquement les valeurs de l'hôte qui restreignent ce que Claude peut faire, avec une lacune à connaître : à moins que vous ne définissiez également les verrous `allowManaged*Only`, les règles d'autorisation de permission de l'hôte et les listes d'autorisation sandbox s'appliquent toujours. Consultez [Restreindre les paramètres parent](/docs/fr/claude-apps-gateway#restrict-parent-settings) pour les verrous.

Un [`policyHelper`](/docs/fr/settings-reference#policyhelper) peut désactiver la fusion parent indépendamment de cette clé ; son entrée dit quand.

Claude Code applique également ces vérifications aux valeurs fournies par le parent d'elles-mêmes :

* Quand n'importe quelle source d'administration définit `allowManagedPermissionRulesOnly`, Claude Code supprime les [règles d'autorisation de permission fournies par le parent](/docs/fr/claude-apps-gateway#restrict-parent-settings) et `additionalDirectories` au fur et à mesure qu'il les lit, même quand une source de priorité plus élevée laisse la clé non définie. L'effet de la clé sur vos propres règles de permission provient des paramètres gérés que Claude Code applique, ou des paramètres parent que vous avez choisi de fusionner
* Claude Code applique la valeur `forceLoginOrgUUID` ou `allowedMcpServers` dans les paramètres gérés qu'il applique et bloque une valeur fournie par le parent. Une valeur dans une source d'administration inférieure que Claude Code n'applique pas ne s'applique ni ne bloque celle du parent.

  Sur Claude Code v2.1.273 ou ultérieur, pendant que `allowManagedMcpServersOnly` est actif, la liste `allowedMcpServers` de la source d'administration la mieux classée qui en définit une s'applique et bloque celle du parent, comme une [clé inter-source](#keys-read-from-every-admin-source). La liste du parent s'applique uniquement quand aucune source d'administration n'en définit une. L'entrée [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) dit quelle source fournit chaque clé sous `"merge"`. Avant v2.1.223, une valeur dans n'importe quelle source d'administration bloquait celle du parent
* Pour `availableModels`, Claude Code applique la valeur dans les paramètres gérés qu'il applique et bloque une liste fournie par le parent
* Pour `strictKnownMarketplaces`, Claude Code applique de la même manière la liste dans les paramètres gérés qu'il applique et bloque une liste fournie par le parent. La liste du parent s'applique uniquement quand aucune source gérée appliquée ne la définit. Nécessite Claude Code v2.1.282 ou ultérieur
* Un `blockedMarketplaces` fourni par le parent s'applique en plus de toute liste de blocage qu'une source gérée définit. Nécessite Claude Code v2.1.282 ou ultérieur

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Garder l'accès au dossier Cowork quand seules les règles gérées s'appliquent
</h4>

[Cowork](https://claude.com/docs/cowork/overview) dans l'application Claude Desktop exécute ses sessions sur Claude Code et accorde à chaque session l'accès à ses dossiers de travail, comme le dossier que l'utilisateur connecte, via les règles d'autorisation qu'il fournit quand il lance la session. Quand votre politique gérée définit [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly), Claude Code conserve uniquement les règles d'autorisation dans la politique gérée : il supprime les règles d'autorisation qu'un hôte fournit comme paramètres parent, comme `--allowedTools`, ou dans un fichier de paramètres, donc les écritures dans ces dossiers perdent leur pré-approbation. Dans une session Cowork qui demande avant les modifications, Cowork ne peut pas afficher l'invite, et Claude signale chaque écriture comme bloquée car le chemin se résout à un emplacement protégé ou un chemin en dehors du dossier connecté.

Pour restaurer les écritures, ajoutez des règles d'autorisation pour ces dossiers à la source gérée que Claude Code [sélectionne](#precedence-within-the-managed-tier) sur ces machines : sur une flotte gérée par MDM, c'est la politique MDM plutôt qu'un fichier de paramètres gérés séparé. Cet exemple utilise la forme de fichier, et une politique MDM prend les mêmes clés. Il garde `allowManagedPermissionRulesOnly` défini et permet les modifications sous un dossier `CoworkProjects` dans le répertoire personnel de chaque utilisateur ; remplacez le chemin par les dossiers que vos utilisateurs connectent :

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Après avoir déployé la politique, Claude peut enregistrer les fichiers sous ce dossier dans une nouvelle session Cowork. [Règles Read et Edit](/docs/fr/permissions#read-and-edit) couvrent la syntaxe du chemin, y compris la forme `//` pour les chemins absolus.

<h3 id="what-a-developer-can-change">
  Ce qu'un développeur peut changer
</h3>

Les fichiers de paramètres propres d'un développeur, les valeurs `--settings`, et les fichiers de projet ne remplacent jamais une valeur gérée ; les [exceptions](/docs/fr/settings#exceptions-to-managed-settings-precedence) permettent uniquement à une valeur plus stricte de niveau inférieur de compter. Ces cas se situent en dehors de cette règle :

* **Le modèle pour une session** : un `model` géré est une valeur par défaut, pas un verrou. `--model` et `ANTHROPIC_MODEL` choisissent toujours le modèle pour cette session, donc déployez [`availableModels`](/docs/fr/settings-reference#availablemodels) pour restreindre le choix.
* **Droits d'administrateur local** : un développeur qui est administrateur sur la machine peut modifier la source gérée elle-même, c'est pourquoi les outils MDM peuvent redéployer le profil ou le fichier selon un calendrier et pourquoi le registre HKLM et le domaine des préférences gérées macOS existent.
* **Le cache géré par le serveur** : les paramètres gérés par le serveur proviennent des serveurs d'Anthropic, et une modification du cache local [dure uniquement jusqu'à la prochaine récupération réussie](/docs/fr/server-managed-settings#security-considerations).
* **Autres outils** : les paramètres gérés lient uniquement Claude Code. Un développeur qui appelle l'API à partir d'un autre outil n'est pas sous eux.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Vérifier qu'une politique est en vigueur
</h2>

Un développeur signale qu'une politique ne s'applique pas, ou vous voulez confirmer qu'un déploiement a atterri avant de le pousser à la flotte. Deux commandes sur cette machine répondent : `/status` montre quelle source gérée Claude Code a sélectionnée, et `claude doctor` énumère ce qu'il a supprimé.

<h3 id="read-the-source-in-/status">
  Lire la source dans /status
</h3>

Sur la machine du développeur, exécutez `/status` dans Claude Code et lisez la ligne `Setting sources`. Quand une source gérée est en vigueur, la ligne énumère `Enterprise managed settings` avec la source que Claude Code a sélectionnée entre parenthèses :

* `(remote)` : paramètres gérés par le serveur de claude.ai ou une passerelle
* `(plist)` ou `(HKLM)` : une politique MDM ou au niveau du système d'exploitation
* `(file)`, `(drop-ins)`, ou `(file + drop-ins)` : `managed-settings.json`, le répertoire drop-in, ou les deux
* `(remote + file, merged)`, ou une autre liste se terminant par `, merged` : votre organisation [compose chaque source gérée](#compose-every-managed-source), et Claude Code a fusionné les sources énumérées dans la politique. Une source inférieure peut toujours fournir des variables `env` sans apparaître dans la liste. Nécessite Claude Code v2.1.242 ou ultérieur
* `(HKCU)` : le registre modifiable par l'utilisateur fallback
* `(parent process)` : un [hôte d'intégration](#let-an-embedding-host-add-policy) a fourni des paramètres restrictifs
* `(helper)` : un [`policyHelper`](/docs/fr/settings-reference#policyhelper) configuré par la source MDM ou fichier sélectionnée

Quand Claude Code a trouvé une source gérée sur la machine et ne l'a pas sélectionnée, une deuxième ligne, `Skipped sources`, nomme chaque source de ce type. Lisez-la pour distinguer une politique qui n'a jamais atteint la machine d'une qui l'a atteinte et qu'une source de priorité plus élevée a remplacée. Nécessite Claude Code v2.1.242 ou ultérieur.

Quand la politique ne s'applique pas, la ligne `Setting sources` vous dit lequel de deux problèmes vous avez :

* **La ligne est manquante** : Claude Code n'a trouvé aucune source gérée qui livre une clé de politique.

  Si vous avez déployé un fichier de paramètres gérés, vérifiez qu'il se trouve au chemin du système d'exploitation et qu'il contient une [clé de politique](#how-claude-code-combines-managed-sources) plutôt que uniquement les clés de contrôle. Un fichier qui n'est pas un JSON valide ne produit pas cet état ; Claude Code [refuse de démarrer](#find-entries-claude-code-dropped) à la place.

  Quand vous avez déployé via les paramètres gérés par le serveur à la place, exécutez `claude doctor`, qui signale le [résultat de la récupération](/docs/fr/server-managed-settings#verify-settings-delivery).
* **La ligne nomme une source autre que celle que vous avez déployée** : une source de priorité plus élevée est présente et Claude Code a ignoré la vôtre, et `Skipped sources` l'énumère. [Comment Claude Code combine les sources gérées](#how-claude-code-combines-managed-sources) donne l'ordre.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Trouver les entrées que Claude Code a supprimées
</h3>

Quand un fichier de paramètres gérés, un profil MDM, une valeur de registre, ou une charge utile gérée par le serveur échoue la validation du schéma, Claude Code ignore d'abord les entrées individuelles qu'il peut réparer, comme une règle de permission invalide, avec un avertissement pour chacune, puis supprime toute clé de niveau supérieur dont la valeur échoue toujours et continue à appliquer chaque clé valide restante.

Claude Code est plus strict avec le `managedSettings` qu'un [`policyHelper`](/docs/fr/settings-reference#policyhelper) émet : il fait les mêmes réparations d'entrée, mais toute violation de schéma qui survit échoue l'exécution entière de l'aide, et au démarrage Claude Code refuse de démarrer, de la même manière que pour une aide qui se termine avec un code non nul.

Quand un fichier de paramètres gérés, un fichier drop-in, un plist MDM, ou une valeur de registre HKLM est présent mais ne peut pas être analysé comme un objet JSON, Claude Code refuse de démarrer et imprime [une erreur nommant la source](/docs/fr/errors#managed-settings-document-could-not-be-parsed), même quand une autre source d'administration livre une politique valide. Chaque source échoue de cette manière quand :

* **Fichier de paramètres gérés ou fichier drop-in** : le fichier n'est pas un JSON valide, ou son niveau supérieur n'est pas un objet
* **Plist MDM** : le `plutil` de macOS signale le plist malformé, ou son contenu converti n'est pas un objet JSON
* **Valeur de registre HKLM** : la valeur `Settings` n'est pas une chaîne, est vide, ou ne contient pas un objet JSON

Trois états de source ne causent pas ce refus :

* Un fichier, profil, ou valeur de registre absent n'est pas une défaillance ; Claude Code s'exécute sans cette source.
* Un fichier de paramètres gérés vide compte comme `{}`.
* Une valeur malformée dans la clé de registre HKCU modifiable par l'utilisateur ne bloque jamais le lancement. Claude Code la signale comme un avis dans `/status` et `claude doctor` à la place.

Si un fichier de paramètres gérés, un fichier drop-in, ou un répertoire `managed-settings.d/` ne peut pas être lu et qu'aucune source d'administration ne fournit une politique, les sessions connectées avec des identifiants claude.ai ou Claude Console se terminent au démarrage avec un message pour contacter un administrateur.

Pour trouver une entrée supprimée, regardez dans l'un de trois endroits :

* Les sessions interactives affichent une boîte de dialogue au démarrage énumérant les entrées invalides.
* Les exécutions non interactives avec `-p` impriment un résumé sur stderr.
* [`claude doctor`](/docs/fr/debug-your-config) énumère chaque entrée invalide avec sa source et son champ.

<h4 id="keys-that-fail-closed">
  Clés qui échouent fermées
</h4>

Quelques clés d'application ne sont pas supprimées quand elles sont invalides. Claude Code applique un fallback plus strict jusqu'à ce que la valeur soit corrigée ; le tableau montre ce qu'il applique pour chaque clé :

| Champ                         | Comportement quand présent mais invalide                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Appliquée comme une liste d'autorisation vide jusqu'à ce que la valeur soit corrigée, donc aucun serveur MCP que les utilisateurs ajoutent n'est admis. Les serveurs que votre organisation livre via [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers) se chargent toujours, et les serveurs `managed-mcp.json` se chargent selon [Comment un serveur est évalué](/docs/fr/managed-mcp#how-a-server-is-evaluated). Une entrée invalide individuelle est supprimée et le sous-ensemble valide est appliqué.                                     |
| `allowedHttpHookUrls`         | Claude Code applique une [liste d'autorisation](/docs/fr/settings-reference#allowedhttphookurls) gérée vide jusqu'à ce que vous corrigiez la valeur, donc un hook HTTP s'exécute uniquement si un autre fichier de paramètres énumère son URL. Si seule une entrée individuelle est invalide, Claude Code supprime cette entrée et applique le reste.                                                                                                                                                                                                         |
| `httpHookAllowedEnvVars`      | Claude Code applique une [liste d'autorisation](/docs/fr/settings-reference#httphookallowedenvvars) gérée vide jusqu'à ce que vous corrigiez la valeur, donc une variable d'en-tête est interpolée uniquement si un autre fichier de paramètres la nomme. Si seule une entrée individuelle est invalide, Claude Code supprime cette entrée et applique le reste.                                                                                                                                                                                              |
| `allowedChannelPlugins`       | Claude Code applique une liste d'autorisation vide jusqu'à ce que vous corrigiez la valeur, donc aucun plugin de canal passé à `--channels` n'est admis. Si seule une entrée individuelle est invalide, il supprime cette entrée et applique le reste.                                                                                                                                                                                                                                                                                                   |
| `strictKnownMarketplaces`     | Appliquée comme une liste d'autorisation vide jusqu'à ce que la valeur soit corrigée, donc aucune [source de marketplace](/docs/fr/plugins/org#restrict-what-users-can-install) n'est admise. Une entrée individuelle qui est invalide ou ne peut pas être appliquée, comme une regex `hostPattern` qui ne compile pas, est supprimée et le sous-ensemble valide est appliqué.                                                                                                                                                                                |
| `allowManagedHooksOnly`       | Traitée comme `true` jusqu'à correction : les [restrictions de hook](/docs/fr/settings-reference#allowmanagedhooksonly) s'appliquent et, à moins que `disableCommandPluginSources` ne soit explicitement `false`, les plugins sourced par commande sont désactivés.                                                                                                                                                                                                                                                                                           |
| `allowManagedMcpServersOnly`  | Traitée comme `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disableCommandPluginSources` | Traitée comme `true`, donc les plugins sourced par commande restent désactivés jusqu'à ce que la valeur soit corrigée.                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disableSideloadFlags`        | Traitée comme `true` jusqu'à ce que la valeur soit corrigée, avec les effets énumérés pour [`disableSideloadFlags`](/docs/fr/settings-reference#disablesideloadflags).                                                                                                                                                                                                                                                                                                                                                                                        |
| `availableModels`             | Appliquée comme une liste d'autorisation vide jusqu'à correction, donc seul le modèle par défaut est disponible ; une entrée non-chaîne est supprimée et le sous-ensemble valide est appliqué.                                                                                                                                                                                                                                                                                                                                                           |
| `enforceAvailableModels`      | Traitée comme `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `syncClaudeAiPlugins`         | Traitée comme `false`, donc la synchronisation des [plugins claude.ai](/docs/fr/settings-reference#syncclaudeaiplugins) est désactivée jusqu'à ce que la valeur soit corrigée.                                                                                                                                                                                                                                                                                                                                                                                |
| `forceLoginOrgUUID`           | Aucune organisation n'est autorisée à se connecter jusqu'à ce que la valeur soit corrigée.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `gatewayInternalNetworks`     | Quand la valeur invalide provient de la source gérée la plus élevée sur la machine, `/login` refuse chaque nouvelle connexion [passerelle cloud](/docs/fr/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) sur cette machine jusqu'à ce que la valeur soit corrigée.                                                                                                                                                                                                                                                                      |
| `crossSessionInbound`         | Traitée comme `refuse`, la valeur la plus restrictive, donc les [messages inter-sessions](/docs/fr/cross-session-messaging#control-inbound-messages) entrants sont refusés jusqu'à ce que la valeur soit corrigée. Le développeur voit [un avertissement](/docs/fr/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                                                                  |
| `deniedMcpServers`            | Une entrée invalide individuelle est supprimée et le sous-ensemble valide est appliqué. Une valeur entièrement invalide est supprimée avec un avertissement, car refuser chaque serveur bloquerait les serveurs que la politique n'a jamais nommés.                                                                                                                                                                                                                                                                                                      |
| `blockedMarketplaces`         | Une entrée invalide individuelle est supprimée et le sous-ensemble valide est appliqué. Une entrée qui analyse mais ne peut jamais correspondre, comme une regex `hostPattern` qui ne compile pas, est conservée avec un avertissement. Elle ne bloque rien jusqu'à correction, mais les [restrictions de marketplace](/docs/fr/plugins/org#restrict-what-users-can-install) restent actives. Une valeur entièrement invalide est supprimée avec un avertissement, car bloquer chaque marketplace bloquerait les sources que la politique n'a jamais nommées. |
| `sandbox.credentials`         | Une entrée invalide récupérable est dégradée à `mode: "deny"` avec un avertissement ; une irrécupérable est supprimée ; les entrées valides restent appliquées. Consultez [entrées de credential invalides](/docs/fr/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                                                                       |

`allowedHttpHookUrls` et `httpHookAllowedEnvVars` fusionnent entre les fichiers de paramètres, donc les entrées dans vos paramètres utilisateur, projet ou local s'appliquent toujours tandis que la liste gérée est vide.

Les fallbacks pour ces deux clés et pour `allowedChannelPlugins` nécessitent Claude Code v2.1.267 ou ultérieur ; les versions antérieures suppriment la clé entière quand sa valeur ou une entrée est invalide. Les fallbacks pour `strictKnownMarketplaces`, `blockedMarketplaces`, et `disableSideloadFlags` nécessitent Claude Code v2.1.277 ou ultérieur ; les versions antérieures suppriment la clé entière quand sa valeur ou une entrée est invalide.

`requiredMinimumVersion` et `requiredMaximumVersion` échouent ouvertes par conception : une valeur invalide est supprimée plutôt qu'appliquée.

Cette tolérance s'applique uniquement aux paramètres gérés. Les fichiers de paramètres utilisateur, projet et local restent stricts : un fichier dont le JSON ou la forme de niveau supérieur échoue la validation est rejeté entièrement et signalé, et une entrée individuelle qui échoue, comme une règle de permission malformée, est ignorée avec un avertissement tandis que le reste du fichier s'applique.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Clés qu'une source gérée seule peut définir
</h2>

Claude Code lit les clés suivantes uniquement d'une source gérée ; les placer dans les fichiers de paramètres utilisateur ou projet n'a aucun effet.

La plupart d'entre elles sont des verrous : la valeur qu'un verrou gouverne, comme les règles de permission ou `sandbox.network.allowedDomains`, est une clé ordinaire que n'importe quel niveau peut définir, et le verrou dit à Claude Code d'honorer uniquement la valeur gérée.

Le tableau couvre les contrôles de permission, plugin et livraison. Pour toute clé non énumérée ici, la colonne Scope de l'[index de référence des paramètres](/docs/fr/settings-reference#all-settings) dit si elle est gérée uniquement ; les clés gérées uniquement restantes là incluent l'URL de connexion à la passerelle, la version, le navigateur, le simulateur mobile, l'hôte SSH, la session locale Desktop, le chemin binaire sandbox, la tarification du modèle, et les contrôles CLAUDE.md.

| Paramètre                                                                                                             | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/fr/settings-reference#allowallclaudeaimcps)                                                 | Charger les connecteurs claude.ai que Claude Code récupère lui-même aux côtés d'un `managed-mcp.json` déployé au lieu de les supprimer                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`allowedChannelPlugins`](/docs/fr/settings-reference#allowedchannelplugins)                                               | Liste d'autorisation des plugins de canal qui peuvent pousser des messages. Remplace la liste d'autorisation Anthropic par défaut quand défini. Nécessite `channelsEnabled: true`. Consultez [Restreindre quels plugins de canal peuvent s'exécuter](/docs/fr/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                                   |
| [`allowManagedHooksOnly`](/docs/fr/settings-reference#allowmanagedhooksonly)                                               | Quand `true`, restreint quels hooks s'exécutent ; consultez [ce qui s'exécute sous `allowManagedHooksOnly`](/docs/fr/settings-reference#what-runs-under-allowmanagedhooksonly) pour la liste complète des effets                                                                                                                                                                                                                                                                                                                                                                 |
| [`allowManagedMcpServersOnly`](/docs/fr/settings-reference#allowmanagedmcpserversonly)                                     | Quand `true`, seul `allowedMcpServers` des paramètres gérés est respecté. `deniedMcpServers` fusionne toujours de toutes les sources. Consultez [Clés lues de chaque source d'administrateur](#keys-read-from-every-admin-source) pour savoir quelles sources gérées peuvent le définir, et [Configuration MCP gérée](/docs/fr/managed-mcp)                                                                                                                                                                                                                                      |
| [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly)                           | Rend les paramètres gérés la seule source de paramètres des règles de permission. L'entrée énumère chaque source qu'elle ignore                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`blockedMarketplaces`](/docs/fr/settings-reference#blockedmarketplaces)                                                   | Liste de blocage des sources de place de marché. Les sources bloquées sont vérifiées avant le téléchargement, donc elles ne touchent jamais le système de fichiers. Consultez [restrictions de place de marché gérée](/docs/fr/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                      |
| [`channelsEnabled`](/docs/fr/settings-reference#channelsenabled)                                                           | Autoriser les [canaux](/docs/fr/channels) pour l'organisation. Consultez [contrôles d'entreprise](/docs/fr/channels#enterprise-controls) pour la valeur par défaut sur chaque plan                                                                                                                                                                                                                                                                                                                                                                                                    |
| [`disableCommandPluginSources`](/docs/fr/settings-reference#disablecommandpluginsources)                                   | Quand `true`, bloque entièrement les [sources de plugin `command`](/docs/fr/plugins/marketplace-reference#command-plugin-source), donc la commande déclarée par la place de marché ne s'exécute jamais. Bloque également les commandes [`headersHelper`](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) de la place de marché, sauf pour une place de marché que les paramètres gérés eux-mêmes déclarent. Quand non défini, suit `allowManagedHooksOnly`. Nécessite Claude Code v2.1.229 ou ultérieur, et le bloc `headersHelper` nécessite v2.1.238 ou ultérieur |
| [`disableSideloadFlags`](/docs/fr/settings-reference#disablesideloadflags)                                                 | Rejeter les drapeaux `--plugin-dir`, `--plugin-url`, `--agents`, et `--mcp-config` au démarrage. Dans les sessions cloud, Claude Code supprime les serveurs MCP que le serveur a livrés via `--mcp-config`, autres que les entrées `type: "sdk"` en processus, et démarre la session. Nécessite Claude Code v2.1.193 ou ultérieur                                                                                                                                                                                                                                           |
| [`forceRemoteSettingsRefresh`](/docs/fr/settings-reference#forceremotesettingsrefresh)                                     | Quand `true`, bloque le démarrage CLI jusqu'à ce que les paramètres gérés distants soient fraîchement récupérés et se termine si la récupération échoue. Consultez [application fail-closed](/docs/fr/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                       |
| [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers)                                                       | Serveurs MCP distants fournis à chaque utilisateur aux côtés des leurs. Il fournit des serveurs plutôt que de verrouiller quoi que ce soit. Consultez [Fournir des serveurs via les paramètres gérés](/docs/fr/managed-mcp#provide-servers-through-managed-settings). Nécessite Claude Code v2.1.259 ou ultérieur                                                                                                                                                                                                                                                                |
| [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior)                                             | Si Claude Code applique uniquement la source gérée de priorité la plus élevée ou [compose chacune d'elles](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`parentSettingsBehavior`](/docs/fr/settings-reference#parentsettingsbehavior)                                             | Si les paramètres parent fournis par l'hôte fusionnent sous la politique gérée                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`pluginSuggestionMarketplaces`](/docs/fr/settings-reference#pluginsuggestionmarketplaces)                                 | Places de marché dont les plugins Claude Code peut suggérer aux utilisateurs                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [`pluginTrustMessage`](/docs/fr/settings-reference#plugintrustmessage)                                                     | Message personnalisé ajouté à l'avertissement de confiance du plugin affiché avant l'installation                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`policyHelper`](/docs/fr/settings-reference#policyhelper)                                                                 | Exécutable qui calcule les paramètres gérés au démarrage ; consultez [Calculer les paramètres gérés avec un aide de politique](/docs/fr/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                                                         |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/fr/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Quand `true`, seuls les chemins `filesystem.allowRead` des paramètres gérés sont respectés. `denyRead` fusionne toujours de toutes les sources                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/fr/settings-reference#sandbox-network-allowmanageddomainsonly)           | Honorer uniquement les `allowedDomains` gérés et les règles d'autorisation `WebFetch(domain:...)` ; bloquer les autres domaines sans demander                                                                                                                                                                                                                                                                                                                                                                                                                               |
| [`strictKnownMarketplaces`](/docs/fr/settings-reference#strictknownmarketplaces)                                           | Contrôle quelles sources de place de marché de plugin les utilisateurs peuvent ajouter et installer des plugins. Consultez [restrictions de place de marché gérée](/docs/fr/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                         |
| [`strictPluginOnlyCustomization`](/docs/fr/settings-reference#strictpluginonlycustomization)                               | Bloquer les skills, agents, hooks, et serveurs MCP des sources utilisateur et projet ; `true` verrouille les quatre, un tableau nomme lesquels                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`wslInheritsWindowsSettings`](/docs/fr/settings-reference#wslinheritswindowssettings)                                     | Quand défini dans le registre HKLM ou un fichier sous `C:\Program Files\ClaudeCode`, faire que WSL lise la chaîne de politique Windows, et lire `/etc/claude-code` uniquement quand aucun fichier de paramètres gérés ou drop-in sous ce répertoire ne livre une [clé de politique](#how-claude-code-combines-managed-sources) ; l'entrée donne l'ordre                                                                                                                                                                                                                     |

<Note>
  Sur les plans Team et Enterprise, un Owner active ou désactive [Remote Control](/docs/fr/remote-control) et [sessions web](/docs/fr/claude-code-on-the-web) à l'échelle de l'organisation dans [les paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control peut en outre être désactivé par appareil avec le paramètre [`disableRemoteControl`](/docs/fr/settings-reference#disableremotecontrol). Les sessions web n'ont pas de clé de paramètres gérés par appareil.

  Pour vérifier si ces paramètres d'organisation ont atteint une machine donnée, exécutez `claude doctor` là et lisez la ligne `Organization policy`, qui dit où Claude Code a chargé la politique ou pourquoi il ne l'a pas chargée. Nécessite Claude Code v2.1.261 ou ultérieur. Dans une session en cours d'exécution, `/status` affiche la même ligne quand la politique n'a pas été chargée.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Désactiver la télémétrie pour votre organisation
</h2>

Claude Code envoie la [télémétrie](/docs/fr/data-usage#telemetry-services) opérationnelle d'Anthropic par défaut sur les sessions qui utilisent l'API Anthropic, directement, via une passerelle LLM, ou via un `ANTHROPIC_BASE_URL` personnalisé ; [Comportements par défaut par fournisseur d'API](/docs/fr/data-usage#default-behaviors-by-api-provider) dit quels fournisseurs l'envoient. Pour la désactiver pour chaque développeur sans compter sur la shell de chaque personne, livrez `DISABLE_TELEMETRY` via le bloc `env` de vos paramètres gérés. Cet exemple définit `DISABLE_TELEMETRY` pour tous ceux que la politique atteint :

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code applique une valeur de `1` sans afficher à l'utilisateur la [boîte de dialogue d'approbation](/docs/fr/server-managed-settings#environment-variables-and-the-approval-dialog).

Si vous désactivez la télémétrie, Claude Code arrête d'envoyer les données d'utilisation qui alimentent le [tableau de bord d'analyse](/docs/fr/analytics) de votre organisation pour les développeurs que la politique atteint. La variable désactive également la récupération des drapeaux de fonctionnalité, ce qui rend Remote Control, le mode auto par défaut, et les autres [fonctionnalités qui nécessitent la récupération des drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching) indisponibles pour ces développeurs.

[Où et quand une politique s'applique](#where-and-when-a-policy-applies) dit quel mécanisme de livraison atteint chaque surface, et [Disponibilité de la plateforme](/docs/fr/server-managed-settings#platform-availability) dit quelles sessions ignorent la récupération des paramètres gérés par le serveur.

Si votre organisation utilise des clés de chiffrement gérées par le client et achemine Claude Code via une passerelle, [Configurer les proxies et les passerelles](/docs/fr/third-party-integrations#configure-proxies-and-gateways) dit pourquoi ces sessions ont besoin de cette variable.

<h2 id="see-also">
  Voir aussi
</h2>

* [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup) : décider ce qu'il faut appliquer et comment
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrer la politique de la console claude.ai ou une passerelle
* [Configuration MCP gérée](/docs/fr/managed-mcp) : contrôler quels serveurs MCP les développeurs peuvent utiliser
* [Tous les paramètres](/docs/fr/settings-reference) : chaque clé, avec si une source gérée peut la définir
* [Fichiers de paramètres d'exemple](/docs/fr/settings-example#an-organizations-managed-settings) : un `managed-settings.json` complet montrant la forme des clés gérées
