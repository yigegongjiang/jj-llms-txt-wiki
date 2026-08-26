> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code avec GitHub Enterprise Server

> Connectez Claude Code à votre instance GitHub Enterprise Server auto-hébergée pour les sessions web, la révision de code et les marketplaces de plugins.

<Note>
  Le support de GitHub Enterprise Server est disponible pour les plans Team et Enterprise.
</Note>

Le support de GitHub Enterprise Server (GHES) permet à votre organisation d'utiliser Claude Code avec des dépôts hébergés sur votre instance GitHub auto-gérée au lieu de github.com. Une fois qu'un propriétaire connecte votre instance GHES, les développeurs peuvent exécuter des sessions web et obtenir des révisions de code automatisées sans aucune configuration par dépôt. Les marketplaces de plugins hébergées sur votre instance sont également pris en charge ; les exigences en matière d'identifiants varient selon la surface, comme décrit dans [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes).

Pour les dépôts sur github.com, consultez [Claude Code sur le web](/docs/fr/claude-code-on-the-web) et [Révision de code](/docs/fr/code-review). Pour exécuter Claude dans votre propre infrastructure CI, consultez [GitHub Actions](/docs/fr/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  Ce qui fonctionne avec GitHub Enterprise Server
</h2>

Le tableau ci-dessous montre quelles fonctionnalités de Claude Code supportent GHES et les différences éventuelles par rapport au comportement de github.com.

| Fonctionnalité            | Support GHES   | Notes                                                                                                                                                        |
| :------------------------ | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code sur le web    | ✅ Supporté     | Un propriétaire connecte l'instance GHES une fois ; les développeurs utilisent `claude --cloud` ou [claude.ai/code](https://claude.ai/code) comme d'habitude |
| Révision de code          | ✅ Supporté     | Mêmes révisions de PR automatisées que github.com                                                                                                            |
| Claude Security           | ✅ Supporté     | Disponible en bêta publique pour les plans Enterprise à [claude.ai/security](https://claude.ai/security)                                                     |
| Sessions Teleport         | ✅ Supporté     | Déplacez les sessions entre le web et le terminal avec `--teleport`                                                                                          |
| Marketplaces de plugins   | ✅ Supporté     | Les exigences en matière d'identifiants varient selon la surface. Voir [Marketplaces de plugins sur GHES](#plugin-marketplaces-on-ghes)                      |
| Métriques de contribution | ✅ Supporté     | Livrées via webhooks au [tableau de bord d'analyse](/docs/fr/analytics)                                                                                           |
| GitHub Actions            | ✅ Supporté     | Nécessite une configuration manuelle du workflow ; `/install-github-app` est github.com uniquement                                                           |
| Serveur GitHub MCP        | ❌ Non supporté | Le serveur GitHub MCP ne fonctionne pas avec les instances GHES                                                                                              |

<h2 id="admin-setup">
  Configuration administrateur
</h2>

Un propriétaire connecte votre instance GHES à Claude Code une seule fois. Après cela, les développeurs de votre organisation peuvent utiliser les dépôts GHES sans aucune configuration supplémentaire. Vous avez besoin du rôle Propriétaire ou Propriétaire principal dans votre organisation Claude et de la permission de créer des GitHub Apps sur votre instance GHES.

La configuration guidée génère un manifeste GitHub App et vous redirige vers votre instance GHES pour créer l'application en un clic. Si votre environnement bloque le flux de redirection, une [configuration manuelle alternative](#manual-setup) est disponible.

<Steps>
  <Step title="Ouvrir les paramètres administrateur de Claude Code">
    Allez à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) et trouvez la section GitHub Enterprise Server.
  </Step>

  <Step title="Démarrer la configuration guidée">
    Cliquez sur **Connecter**. Entrez un nom d'affichage pour la connexion et le nom d'hôte de votre GHES, par exemple `github.example.com`. Si votre instance GHES utilise un certificat auto-signé ou une autorité de certification privée, collez le certificat CA dans le champ optionnel.
  </Step>

  <Step title="Créer la GitHub App">
    Cliquez sur **Continuer vers GitHub Enterprise**. Votre navigateur vous redirige vers votre instance GHES avec un manifeste d'application pré-rempli. Vérifiez la configuration et cliquez sur **Créer une GitHub App**. GHES vous redirige vers Claude avec les identifiants de l'application stockés automatiquement.
  </Step>

  <Step title="Installer l'application sur vos dépôts">
    À partir de la page GitHub App sur votre instance GHES, installez l'application sur les dépôts ou organisations auxquels vous souhaitez que Claude accède. Vous pouvez commencer par un sous-ensemble et en ajouter d'autres plus tard.
  </Step>

  <Step title="Activer les fonctionnalités">
    Retournez à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) et activez [Révision de code](/docs/fr/code-review#set-up-code-review), Claude Security et [métriques de contribution](/docs/fr/analytics#enable-contribution-metrics) pour vos dépôts GHES en utilisant la même configuration que github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  Permissions de la GitHub App
</h3>

Le manifeste configure la GitHub App avec les permissions et les événements webhook dont Claude a besoin pour les sessions web, la révision de code, Claude Security et les métriques de contribution :

| Permission       | Accès               | Utilisé pour                                                   |
| :--------------- | :------------------ | :------------------------------------------------------------- |
| Contents         | Lecture et écriture | Clonage de dépôts et envoi de branches                         |
| Pull requests    | Lecture et écriture | Création de PR et publication de commentaires de révision      |
| Issues           | Lecture et écriture | Réponse aux mentions de problèmes                              |
| Checks           | Lecture et écriture | Publication des exécutions de vérification de révision de code |
| Actions          | Lecture             | Lecture du statut CI pour la correction automatique            |
| Repository hooks | Lecture et écriture | Réception des webhooks pour les métriques de contribution      |
| Metadata         | Lecture             | Requis par GitHub pour toutes les applications                 |

L'application s'abonne aux événements `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review` et `check_run`.

<h3 id="manual-setup">
  Configuration manuelle
</h3>

Si le flux de redirection guidé est bloqué par votre configuration réseau, cliquez sur **Ajouter manuellement** au lieu de Connecter. Créez une GitHub App sur votre instance GHES avec les [permissions et événements ci-dessus](#github-app-permissions), puis entrez les identifiants de l'application dans le formulaire : nom d'hôte, ID client OAuth et secret, ID de GitHub App, ID client, secret client, secret webhook et clé privée.

<h3 id="network-requirements">
  Exigences réseau
</h3>

Votre instance GHES doit être accessible à partir de l'infrastructure Anthropic pour que Claude puisse cloner les dépôts et publier des commentaires de révision. Si votre instance GHES est derrière un pare-feu, mettez en liste blanche les [adresses IP de l'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

<h2 id="developer-workflow">
  Flux de travail des développeurs
</h2>

Une fois que votre administrateur a connecté l'instance GHES, aucune configuration côté développeur n'est nécessaire. Claude Code détecte automatiquement le nom d'hôte de votre GHES à partir de la télécommande git dans votre répertoire de travail.

Clonez un dépôt à partir de votre instance GHES comme vous le feriez normalement :

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Ensuite, démarrez une session web. Claude détecte l'hôte GHES à partir de votre télécommande git et achemine la session via votre instance configurée de l'organisation :

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

La session s'exécute sur l'infrastructure Anthropic, clone votre dépôt à partir de GHES et repousse les modifications vers une branche. Surveillez la progression avec `/tasks` ou à [claude.ai/code](https://claude.ai/code). Consultez [Claude Code sur le web](/docs/fr/claude-code-on-the-web) pour le flux de travail complet de la session distante, y compris la révision des différences, la correction automatique et les routines.

<h3 id="teleport-sessions-to-your-terminal">
  Téléporter les sessions vers votre terminal
</h3>

Tirez une session web dans votre terminal local avec `claude --teleport`. Teleport vérifie que vous êtes dans une extraction du même dépôt GHES avant de récupérer la branche et de charger l'historique de la session. Consultez [les exigences de téléportation](/docs/fr/claude-code-on-the-web#teleport-requirements) pour plus de détails.

<h2 id="plugin-marketplaces-on-ghes">
  Marketplaces de plugins sur GHES
</h2>

Hébergez des marketplaces de plugins sur votre instance GHES pour distribuer les outils internes dans votre organisation. La structure de la marketplace est identique aux marketplaces hébergées sur github.com, mais l'installation fonctionne différemment selon l'endroit où vous ajoutez la marketplace, et les identifiants varient selon les surfaces :

| Surface                                          | Fonctionnement de l'installation                                                                                                                                                                                                                                         | Ce dont chaque utilisateur a besoin                                                                                                                                                                                                            |
| :----------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code CLI et bureau                        | Claude Code clone le dépôt de la marketplace en utilisant les identifiants git existants de la machine                                                                                                                                                                   | Accès Git à votre hôte GHES depuis leur machine                                                                                                                                                                                                |
| Paramètres gérés (`extraKnownMarketplaces`)      | Claude Code enregistre l'entrée et clone le dépôt en utilisant les identifiants git existants de la machine                                                                                                                                                              | Accès Git à votre hôte GHES depuis leur machine                                                                                                                                                                                                |
| Paramètres de plugin de l'organisation claude.ai | Un propriétaire sélectionne l'instance GHES comme source ; le backend d'Anthropic récupère et synchronise le dépôt en utilisant l'application GitHub de la [configuration d'administration](#admin-setup)                                                                | Rien par utilisateur une fois ajouté. Le propriétaire qui l'ajoute doit avoir son propre compte GitHub Enterprise connecté comme vérification d'accès, et l'application GitHub doit être installée sur le dépôt de la marketplace              |
| Paramètres utilisateur claude.ai                 | Le backend d'Anthropic récupère le dépôt en utilisant la connexion GitHub Enterprise de l'utilisateur qui le soumet                                                                                                                                                      | Son propre compte GitHub Enterprise connecté à Claude                                                                                                                                                                                          |
| Claude Code sur le web                           | Les sessions cloud clonent les marketplaces à l'intérieur du sandbox de la session. Le sandbox ne peut atteindre votre instance GHES que lorsque le dépôt de la session se trouve sur cette même instance, et ses identifiants git sont limités aux dépôts de la session | Non fiable pour les marketplaces hébergées sur GHES : un hôte différent du dépôt de la session n'est pas accessible, et même les installations sur la même instance peuvent échouer. Utilisez plutôt la CLI, les paramètres gérés ou claude.ai |

<Warning>
  Les connexions GitHub Enterprise sur claude.ai sont par utilisateur lorsqu'une marketplace est ajoutée à partir des paramètres utilisateur. La [configuration d'administration](#admin-setup) connecte votre instance GHES à votre organisation, mais elle ne connecte pas les comptes utilisateur individuels : chaque utilisateur qui ajoute une marketplace GHES à partir de ses propres paramètres doit d'abord connecter son propre compte GitHub Enterprise, et la connexion d'un utilisateur, y compris celle du propriétaire, ne couvre personne d'autre. Les marketplaces ajoutées par un propriétaire dans les paramètres de plugin de l'organisation ne mettent pas cette exigence sur les utilisateurs, car les récupérations continues utilisent l'application GitHub de l'organisation. Le propriétaire qui ajoute la marketplace doit toujours avoir son propre compte GitHub Enterprise connecté au moment de l'ajout.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Ajouter une marketplace GHES
</h3>

Le raccourci `owner/repo` se résout toujours en github.com. Pour les marketplaces hébergées sur GHES, utilisez l'URL git complète. Les URL HTTPS sont recommandées :

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

Les URL SSH fonctionnent si la machine fait déjà confiance à votre hôte GHES :

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code exécute git de manière non-interactive et rejette les connexions SSH aux hôtes qui ne figurent pas dans le fichier `known_hosts` de la machine. Une URL HTTPS avec un assistant d'identifiants git évite l'exigence `known_hosts`.

Consultez [Créer et distribuer une marketplace de plugins](/docs/fr/plugin-marketplaces) pour le guide complet de la création de marketplaces.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Pré-enregistrer les marketplaces GHES avec les paramètres gérés
</h3>

Le paramètre `extraKnownMarketplaces` pré-enregistre une marketplace afin que les développeurs l'obtiennent sans configuration manuelle. Il fonctionne à partir de [n'importe quel fichier de paramètres](/docs/fr/settings#extraknownmarketplaces), y compris le `.claude/settings.json` d'un dépôt ; les paramètres gérés le livrent à l'échelle de l'organisation :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code installe ces marketplaces localement : il enregistre chaque entrée et clone le dépôt avec les identifiants git existants de la machine. Ce chemin ne passe pas par claude.ai, donc la connexion GitHub Enterprise par utilisateur n'est pas requise. Pour un déploiement réussi :

* **Utilisez une URL git complète.** Le raccourci `owner/repo` se résout toujours en github.com et ne peut pas référencer un hôte GHES.
* **Préférez les URL HTTPS.** Les clones SSH échouent sur les machines qui ne font pas déjà confiance à votre clé d'hôte GHES. Une URL HTTPS avec l'assistant d'identifiants git standard de votre organisation fonctionne sur n'importe quelle machine avec des identifiants configurés.
* **Confirmez que chaque machine peut cloner à partir de votre hôte GHES.** Si une machine manque d'identifiants, la marketplace est enregistrée mais jamais installée, et ses plugins signalent qu'ils ne sont pas trouvés au lieu de demander des identifiants.
* **Confirmez que le paramètre atteint chaque machine.** Un fichier de paramètres gérés ne prend effet que sur les machines sur lesquelles il est déployé, par exemple via votre système de gestion des appareils. Consultez [paramètres gérés](/docs/fr/settings#settings-files) pour les emplacements des fichiers.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Mettre en liste blanche les marketplaces GHES dans les paramètres gérés
</h3>

Si votre organisation utilise les [paramètres gérés](/docs/fr/settings) pour restreindre les marketplaces que les développeurs peuvent ajouter, utilisez le type de source `hostPattern` pour autoriser toutes les marketplaces de votre instance GHES sans énumérer chaque dépôt :

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Consultez la référence des paramètres [strictKnownMarketplaces](/docs/fr/settings#strictknownmarketplaces) et [extraKnownMarketplaces](/docs/fr/settings#extraknownmarketplaces) pour le schéma complet.

<h2 id="limitations">
  Limitations
</h2>

Quelques fonctionnalités se comportent différemment sur GHES que sur github.com. Le [tableau des fonctionnalités](#what-works-with-github-enterprise-server) résume le support ; cette section couvre les solutions de contournement.

* **Commande `/install-github-app`** : suivez le flux de [configuration administrateur](#admin-setup) sur claude.ai à la place. Si vous souhaitez également des workflows GitHub Actions sur GHES, adaptez manuellement l'[exemple de workflow](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml).
* **Serveur GitHub MCP** : utilisez plutôt la CLI `gh` configurée pour votre hôte GHES. Exécutez `gh auth login --hostname github.example.com` pour vous authentifier, puis Claude peut utiliser les commandes `gh` dans les sessions.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="web-session-fails-to-clone-repository">
  La session web échoue à cloner le dépôt
</h3>

Si `claude --cloud` échoue avec une erreur de clonage, vérifiez qu'un propriétaire a terminé la configuration de votre instance GHES et que la GitHub App est installée sur le dépôt sur lequel vous travaillez. Demandez au propriétaire qui a connecté l'instance de confirmer que le nom d'hôte enregistré dans les paramètres Claude correspond au nom d'hôte de votre télécommande git.

<h3 id="marketplace-add-fails-with-a-policy-error">
  L'ajout de marketplace échoue avec une erreur de politique
</h3>

Si `/plugin marketplace add` est bloqué pour votre URL GHES, votre organisation a restreint les sources de marketplace. Demandez à votre administrateur d'ajouter une entrée `hostPattern` pour le nom d'hôte de votre GHES dans les [paramètres gérés](#allowlist-ghes-marketplaces-in-managed-settings).

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  L'ajout de marketplace sur claude.ai échoue avec une erreur d'accès GitHub
</h3>

Si l'ajout d'un marketplace GHES à partir de vos paramètres utilisateur échoue avec une erreur générique comme « Marketplace n'a pas pu être ajouté », vérifiez d'abord votre connexion GitHub Enterprise. C'est ce qui s'affiche lorsque votre propre compte GitHub Enterprise n'est pas connecté à Claude, même si l'instance GHES de votre organisation est configurée et que d'autres utilisateurs sont connectés. La boîte de dialogue ne pointe pas vers le flux de connexion GitHub Enterprise, et l'option « Se connecter à GitHub » sur l'onglet Parcourir se connecte à github.com, ce qui n'accorde pas l'accès aux dépôts GHES.

Pour connecter votre compte GitHub Enterprise : le sélecteur de dépôt sur [claude.ai/code](https://claude.ai/code) offre une option de connexion pour chaque instance GHES configurée, et les propriétaires peuvent également se connecter à partir de la section GitHub Enterprise des [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code). Ensuite, ajoutez le marketplace à nouveau. Vous pouvez également demander à un propriétaire d'ajouter le marketplace dans les paramètres de plugin de l'organisation, ce qui supprime l'exigence de connexion par utilisateur.

Sur d'autres surfaces claude.ai, une erreur « Dépôt non trouvé. S'il est privé, l'accès GitHub est requis » sur un marketplace GHES indique généralement la même connexion manquante. Connectez votre compte GitHub Enterprise via l'un des chemins ci-dessus, puis réessayez.

<h3 id="ghes-instance-not-reachable">
  Instance GHES non accessible
</h3>

Si les révisions ou les sessions web expirent, votre instance GHES peut ne pas être accessible à partir de l'infrastructure Anthropic. Confirmez que votre pare-feu autorise les connexions entrantes à partir des [adresses IP de l'API Anthropic](https://platform.claude.com/docs/fr/api/ip-addresses).

<h2 id="related-resources">
  Ressources connexes
</h2>

Ces pages couvrent les fonctionnalités référencées dans ce guide en plus de détails :

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : exécutez les sessions Claude Code sur l'infrastructure cloud
* [Révision de code](/docs/fr/code-review) : révisions de PR automatisées
* [Marketplaces de plugins](/docs/fr/plugin-marketplaces) : créer et distribuer des catalogues de plugins
* [Analyse](/docs/fr/analytics) : suivre l'utilisation et les métriques de contribution
* [Paramètres gérés](/docs/fr/settings) : configuration de politique à l'échelle de l'organisation
* [Configuration réseau](/docs/fr/network-config) : exigences de pare-feu et de liste blanche IP
