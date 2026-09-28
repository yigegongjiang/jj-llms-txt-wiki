> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les environnements cloud

> Configurez les environnements cloud pour les sessions Claude Code cloud : niveaux d'accès réseau, variables d'environnement, scripts de configuration et mise en cache d'environnement.

<Note>
  Les environnements cloud s'appliquent aux [sessions cloud](/docs/fr/claude-code-on-the-web), qui sont disponibles sur les plans Pro, Max et Team, et pour les utilisateurs Enterprise disposant de [sièges premium ou de sièges Chat + Claude Code](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Chaque [session cloud](/docs/fr/claude-code-on-the-web) s'exécute dans un environnement cloud. Vous pouvez configurer un environnement pour autoriser ou refuser l'[accès réseau](#access-levels), [définir des variables d'environnement](#set-environment-variables) pour la session, sur les plans Pro et Max stocker des [identifiants API](#add-api-credentials) que les sessions utilisent sans les voir, et exécuter un [script de configuration](#setup-scripts) avant que Claude ne commence à travailler.

Les mêmes environnements s'appliquent partout où vous démarrez une session cloud : l'[application de bureau](/docs/fr/desktop), l'[application mobile Claude](/docs/fr/mobile), votre navigateur sur [claude.ai/code](https://claude.ai/code), le terminal avec [`claude --cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud), les [routines](/docs/fr/routines) et [Claude Tag](https://claude.com/docs/claude-tag/overview). Chacune de ces surfaces peut également router vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments). La section [Disponibilité et limitations](/docs/fr/self-hosted-environments#availability-and-limitations) couvre ce que Claude ne peut pas encore utiliser quand une session Claude Tag s'exécute dans un.

<Info>
  Les sessions [Remote Control](/docs/fr/remote-control) connectent les interfaces web et mobile à une session sur votre propre machine, qui utilise le réseau et les fichiers de votre machine, et non un environnement cloud. Les sessions de canal Claude Tag utilisent uniquement des environnements au niveau de l'organisation, soit des [environnements partagés](#organization-shared-environments), soit des [environnements auto-hébergés](/docs/fr/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  L'environnement par défaut
</h2>

Si vous n'avez pas encore d'environnement, l'intégration configure l'environnement **Default** pour vous. Cela dépend de l'endroit où vous vous intégrez :

* **Flux CLI tels que `/web-setup`** : créent **Default** pour vous
* **Intégration web sur Pro et Max** : crée **Default** pour vous
* **Intégration web sur Team et Enterprise** : affiche un formulaire **Créer votre premier environnement cloud** sauf si un propriétaire a activé la [Configuration web rapide](/docs/fr/claude-code-on-the-web#github-authentication-options) ; conservez les valeurs par défaut du formulaire et cliquez sur **Créer et terminer** pour obtenir le même environnement **Default**

**Default** n'a aucune configuration propre :

* [Accès réseau **Trusted**](#access-levels) : les sessions atteignent les registres de paquets et autres [domaines autorisés](#default-allowed-domains), et rien d'autre via le réseau de la session.
* Aucune autre configuration : **Default** ne définit aucune variable d'environnement ou script de configuration, donc les sessions commencent avec juste les [outils pré-installés](#installed-tools).

Avec seulement **Default** disponible, chaque session s'exécute dedans. Quand vous avez plus d'un environnement, les sessions en choisissent un par surface :

* Sur l'application Desktop, l'application mobile et sur claude.ai/code, les sessions que vous démarrez vous-même utilisent l'environnement affiché dans le [sélecteur](#configure-your-environment). Un [environnement par défaut](#organization-shared-environments) défini par un propriétaire remplit la sélection quand vous n'en avez pas choisi un. Les threads dans un [projet](/docs/fr/claude-projects#project-settings-reference) utilisent l'environnement défini dans les paramètres du projet à la place.
* Depuis le CLI, Claude Code utilise votre choix [`/remote-env`](#select-an-environment-from-the-cli), ou revient à l'environnement hébergé par Anthropic quand votre liste en a un, et sinon au premier environnement de votre liste qui n'est pas un environnement bridge, une entrée [Remote Control](/docs/fr/remote-control) que vous enregistrez pour représenter votre propre machine plutôt qu'un environnement cloud. Pour un [environnement auto-hébergé](/docs/fr/self-hosted-environments), passer `--environment <environment-id>` avec son ID `ccpool_` [quand vous lancez une session](/docs/fr/self-hosted-environments-testing#run-the-test-loop) remplace le choix `/remote-env` et le fallback pour cet appel. Claude Code rejette les IDs `env_` hébergés par Anthropic passés au drapeau, donc utilisez `/remote-env` pour cibler ceux-ci. Le drapeau nécessite Claude Code v2.1.224 ou ultérieur.

Configurez un environnement quand le défaut ne suffit pas : quand Claude doit atteindre des domaines en dehors de la [liste d'autorisation par défaut](#default-allowed-domains), a besoin de variables d'environnement définies pour ses sessions, ou a besoin de dépendances installées avant de commencer à travailler.

<h2 id="configure-your-environment">
  Configurer votre environnement
</h2>

Créez, modifiez et archivez des environnements à partir du sélecteur d'environnement, accessible sur [claude.ai/code](https://claude.ai/code) après [l'intégration web](/docs/fr/web-quickstart), ou à partir de la zone de message dans l'[application de bureau](/docs/fr/desktop#cloud-sessions). Les environnements que vous créez sont personnels à votre compte ; les [environnements partagés](#organization-shared-environments) créés par un propriétaire apparaissent dans le même sélecteur. Consultez [Outils installés](#installed-tools) pour voir ce qui est disponible sans aucune configuration.

<Steps>
  <Step title="Ouvrir le sélecteur d'environnement">
    Sur [claude.ai/code](https://claude.ai/code), sélectionnez l'icône cloud affichant le nom de l'environnement actuel, dans la ligne au-dessus de la zone de message. Il n'y a pas de page de paramètres ou d'URL directe pour le sélecteur.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="Le sélecteur d'environnement ouvert au-dessus de la zone de message sur claude.ai/code. Le bouton cloud affichant le nom de l'environnement Default se trouve dans la ligne au-dessus de la zone de message. Le menu ouvert liste une ligne Local avec les étiquettes Download et Desktop only, une section Cloud où l'environnement Default est sélectionné avec une coche et affiche une icône d'engrenage de paramètres au survol, une option Add cloud environment, et une section Remote Control avec les instructions de configuration." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Ajouter ou modifier un environnement">
    Sélectionnez **Add cloud environment**, ou survolez un environnement existant et sélectionnez l'icône de paramètres qui apparaît à droite. La boîte de dialogue inclut le nom, le niveau d'accès réseau, les variables d'environnement et le script de configuration. Lorsque vous modifiez un environnement cloud existant sur un plan Pro ou Max, la boîte de dialogue inclut également les [identifiants API](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="La boîte de dialogue New cloud environment. Un champ Name avec le texte d'espace réservé Default, un sélecteur Network access défini sur Trusted avec des liens vers la politique réseau et les niveaux d'accès, une zone Environment variables affichant un texte d'espace réservé au format .env avec une note indiquant que les valeurs sont visibles pour quiconque utilise l'environnement, une zone Setup script décrite comme un script Bash qui s'exécute au démarrage d'une nouvelle session avant le lancement de Claude Code, et les boutons Cancel et Create environment." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Définir les variables d'environnement
</h3>

Les variables d'environnement utilisent le format `.env`, une paire `KEY=value` par ligne. Les valeurs simples n'ont pas besoin de guillemets, et si vous mettez une valeur entre guillemets avec une paire correspondante, les guillemets ne font pas partie de la valeur. Mettez entre guillemets une valeur qui s'étend sur plusieurs lignes ou contient un `#` : dans une valeur sans guillemets, `#` démarre un commentaire et le reste de la ligne est supprimé.

L'exemple suivant définit trois variables.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Chaque session copie les valeurs de l'environnement une fois, au démarrage, dans des variables d'environnement ordinaires que toute commande exécutée par Claude peut lire. Comme les sessions en cours d'exécution ne relisent pas la configuration, la modification ou l'ajout de variables affecte les sessions que vous démarrez par la suite ; les sessions déjà en cours d'exécution conservent les valeurs avec lesquelles elles ont démarré.

Une session cloud définit également certaines variables elle-même au démarrage. Pour [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/fr/claude-code-on-the-web#manage-context), la valeur que la session définit remplace celle que vous ajoutez ici, donc ajouter cette clé ici n'a aucun effet.

Quiconque utilise l'environnement peut lire les valeurs. Sur les plans Pro et Max, utilisez plutôt une [identifiant API](#add-api-credentials) pour une clé que le proxy d'agent peut joindre à une demande. Les [demandes qui ne reçoivent jamais d'identifiant](#requests-that-never-get-the-credential) sont listées là.

<h3 id="add-api-credentials">
  Ajouter des identifiants API
</h3>

Un identifiant API est une clé API ou un jeton que vous stockez sur un environnement cloud afin que Claude puisse appeler cette API à partir de n'importe quelle session dans l'environnement sans voir la clé. Le proxy d'agent d'Anthropic ajoute la clé aux demandes pour les hôtes que vous listez, après que chaque demande quitte la VM de la session. La clé n'atteint jamais Claude, les commandes qu'il exécute, ou les variables d'environnement de la session.

Les identifiants API sont disponibles sur les plans Pro et Max. Ils ne sont pas encore disponibles sur les plans Team ou Enterprise, donc la section **API credentials** n'apparaît pas dans la boîte de dialogue d'environnement sur ces plans.

<h4 id="requirements">
  Exigences
</h4>

Deux d'entre elles décident si vous pouvez ajouter un identifiant, et deux décident si le proxy d'agent peut l'utiliser une fois ajouté :

* **Rôle** : un rôle d'administrateur d'organisation dans votre organisation claude.ai
  * Sur Team et Enterprise, les propriétaires le détiennent et les administrateurs ne le détiennent pas
  * Sur Pro et Max, vous le détenez dans votre propre organisation
  * Sans lui, vous voyez une note au lieu de la liste des identifiants, même sur vos propres environnements. Demandez à un propriétaire d'ajouter l'identifiant à un environnement partagé et d'exécuter vos sessions là
* **Type d'environnement** : un environnement cloud hébergé par Anthropic qui existe déjà. Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) n'a pas d'identifiants API
* **Accessibilité de l'API** : l'API accepte les connexions depuis Internet, car les demandes partent du réseau d'Anthropic
* **Clés de chiffrement** : si votre organisation utilise des clés de chiffrement gérées par le client, vous ne pouvez pas enregistrer les identifiants

<h4 id="add-a-credential">
  Ajouter un identifiant
</h4>

Vous ajoutez les identifiants un à la fois à partir de l'éditeur d'un environnement qui existe déjà. La boîte de dialogue pour un nouvel environnement ne les propose pas. Il n'y a pas non plus de modification. Pour modifier les hôtes ou la valeur d'un identifiant, supprimez-le et ajoutez-le à nouveau.

<Steps>
  <Step title="Ouvrir les identifiants API de l'environnement">
    [Ouvrez l'environnement pour modification](#configure-your-environment) sur [claude.ai/code](https://claude.ai/code). Dans la boîte de dialogue **Update cloud environment**, trouvez **API credentials** sous **Environment variables**. Vous voyez les identifiants déjà sur l'environnement, chacun avec les hôtes auxquels il s'applique.
  </Step>

  <Step title="Ajouter l'identifiant">
    Sélectionnez **Add credential** et remplissez le formulaire. Conservez le **Credential type** par défaut, **Bearer**, pour une clé API qui voyage dans un en-tête de demande, et remplissez ces champs :

    * **Name** : une étiquette pour l'identifiant, comme `Internal billing API`
    * **Allowed websites** : les hôtes de l'API, comme `api.example.com`. Un `*.` initial correspond à chaque sous-domaine
    * **Custom headers** : une ligne pour l'en-tête qui porte la clé. La ligne commence par `Authorization` comme **Name** de l'en-tête et `Bearer` comme son **Prefix** ; collez la clé elle-même comme **Value**. Pour un en-tête comme `X-Api-Key` qui prend la valeur brute, changez le nom et effacez le préfixe

    Pour une API qui s'authentifie d'une autre manière, choisissez un **Credential type** différent. La liste est la même que celle que [Claude Tag](https://claude.com/docs/claude-tag/overview), l'intégration Slack pour les plans Team et Enterprise, propose pour les [connexions](https://claude.com/docs/claude-tag/admins/add-connections).
  </Step>

  <Step title="Enregistrer l'identifiant">
    Sélectionnez **Connect**. L'identifiant apparaît dans la liste avec ses hôtes, enregistré sans le bouton **Save changes** de la boîte de dialogue. Vous ne pouvez pas afficher la valeur à nouveau après l'enregistrement.
  </Step>
</Steps>

Pour confirmer que l'identifiant fonctionne, démarrez une session dans l'environnement et demandez à Claude d'appeler l'API, par exemple avec `curl`. L'API répond comme si la clé était dans la demande, et la clé n'apparaît pas dans les variables d'environnement de la session ou dans aucun fichier. Si la liste marque un identifiant **Not sent** à la place, la note sous celui-ci explique pourquoi et quoi faire. Deux identifiants dont les hôtes se chevauchent sans correspondre exactement ne reçoivent aucun marqueur, et le proxy d'agent n'en envoie qu'un.

<h4 id="which-requests-get-the-credential">
  Quelles demandes reçoivent l'identifiant
</h4>

Le proxy d'agent joint un identifiant à une demande lorsque l'hôte de la demande correspond à l'un de ceux que vous avez listés sur cet identifiant. Les sessions peuvent atteindre ces hôtes même lorsque le [niveau d'accès réseau](#access-levels) de l'environnement ne le permettrait pas autrement, sauf les [hôtes qui ne reçoivent jamais l'identifiant](#requests-that-never-get-the-credential). L'identifiant s'applique dans chaque session qui s'exécute dans l'environnement, peu importe qui l'a démarrée, jusqu'à ce que vous le supprimiez.

<h4 id="requests-that-never-get-the-credential">
  Demandes qui ne reçoivent jamais l'identifiant
</h4>

Le proxy d'agent ne joint jamais un identifiant que vous ajoutez à ces demandes :

* **GitHub** : le [proxy GitHub](#github-proxy) authentifie les demandes à GitHub à la place, donc vous n'avez pas besoin d'un identifiant API pour cela
* **L'API Anthropic et les registres de paquets publics** : `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io`, et `proxy.golang.org`
* **Demandes de script de configuration** : Claude Code se connecte au proxy d'agent au lancement, après l'exécution du [script de configuration](#setup-scripts)

<h3 id="select-an-environment-from-the-cli">
  Sélectionner un environnement à partir de la CLI
</h3>

Exécutez `/remote-env` dans votre terminal pour choisir l'environnement par défaut pour les sessions cloud que vous créez à partir de la CLI, comme [`claude --cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud). La commande ouvre un sélecteur de vos environnements existants et enregistre votre choix dans la clé `remote.defaultEnvironmentId` dans vos [paramètres utilisateur](/docs/fr/settings#where-settings-live), donc cela s'applique dans chaque projet sur votre machine jusqu'à ce que vous le changiez, sauf si la même clé est définie à une [couche de paramètres](#configure-your-environment) de précédence plus élevée, comme les paramètres de projet d'un dépôt.

Un ID [d'environnement auto-hébergé](/docs/fr/self-hosted-environments), qui a la forme `ccpool_...`, suit une règle de source plus stricte. Consultez [`remote.defaultEnvironmentId`](/docs/fr/settings-reference#remote-defaultenvironmentid) pour les couches de paramètres que Claude Code honore.

`/remote-env` définit uniquement la valeur par défaut : il ne démarre pas une session, et il ne peut pas ajouter ou modifier des environnements. Gérez-les à partir du [sélecteur d'environnement](#configure-your-environment).

<h3 id="archive-an-environment">
  Archiver un environnement
</h3>

Pour archiver l'un de vos propres environnements, ouvrez-le pour modification et sélectionnez **Archive**. Un propriétaire archive un [environnement partagé](#organization-shared-environments) à partir de la page **Cloud environments** dans les paramètres d'administration. Vous ne pouvez pas supprimer un environnement, seulement l'archiver.

L'archivage affecte les nouvelles sessions, pas les sessions en cours d'exécution :

* Les sessions déjà en cours d'exécution dans l'environnement continuent de fonctionner.
* L'environnement disparaît du sélecteur et de `/remote-env`, donc vous ne pouvez pas le choisir pour les nouvelles sessions.
* Les identifiants API sur l'environnement restent attachés dans ses sessions en cours d'exécution. Supprimez ceux que vous ne voulez plus avant d'archiver.
* Aucune nouvelle session ne peut démarrer dans un environnement archivé, sur aucune surface. Si l'environnement était votre [défaut CLI](#select-an-environment-from-the-cli) enregistré, Claude Code démarre les sessions cloud CLI dans l'environnement hébergé par Anthropic lorsque votre liste en a un, et sinon dans le premier environnement de votre liste qui n'est pas un [environnement de pont Remote Control](#the-default-environment). Tout ce qui est configuré avec l'environnement explicitement, comme une [routine](/docs/fr/routines#environments-and-network-access), ne peut pas démarrer de nouvelles sessions dedans. Pointez-le vers un autre environnement.

<h3 id="organization-shared-environments">
  Environnements partagés par l'organisation
</h3>

Sur les plans Team et Enterprise, un propriétaire peut créer des environnements cloud qui sont partagés avec chaque membre de l'organisation. Le même rôle gère tout le reste sur la page d'administration **Cloud environments**, y compris les [environnements auto-hébergés](/docs/fr/self-hosted-environments) ; le rôle Admin ne peut pas ouvrir la page. La liste complète des rôles qui peuvent l'ouvrir est celle pour [gérer les paramètres gérés par le serveur](/docs/fr/server-managed-settings#access-control).

Les environnements partagés apparaissent dans le [sélecteur d'environnement](#configure-your-environment) de chaque membre sous un en-tête **Organization**, après les environnements du membre sous **Personal**, donc une équipe peut standardiser sur une configuration au lieu que chaque membre la recréé. Sélectionner l'icône de paramètres d'un environnement partagé là ouvre un résumé en lecture seule de sa configuration pour chaque membre, propriétaires inclus.

Un propriétaire rend un environnement disponible pour l'organisation de l'une de deux façons :

* **Créer un environnement partagé** : utilisez la page **Cloud environments** dans les [paramètres d'administration](https://claude.ai/admin-settings), qui est aussi où les propriétaires modifient et archivez les environnements partagés. Chacun a un nom, un [niveau d'accès réseau](#access-levels), des [variables d'environnement](#set-environment-variables) au format `.env`, et un [script de configuration](#setup-scripts).
* **Partager un environnement personnel** : ouvrez l'un de vos propres environnements pour modification dans le sélecteur d'environnement, puis partagez-le à partir de la ligne **Who can use it**. L'environnement conserve son ID, donc les sessions et routines qui l'utilisent déjà ne sont pas affectées, et chaque membre peut alors le voir et démarrer des sessions dedans.

Les propriétaires choisissent l'[environnement par défaut](#the-default-environment) de l'organisation séparément, sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Les sessions de chaque membre dans un environnement partagé lisent ses variables, donc n'incluez pas de secrets dedans. Les [identifiants API](#add-api-credentials), qui donnent aux sessions une clé qu'elles ne peuvent pas lire, ne sont pas encore disponibles sur les plans Team ou Enterprise.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Définir l'environnement qu'un canal Claude Tag utilise
</h3>

Dans les canaux [Claude Tag](https://claude.com/docs/claude-tag/overview), Claude fonctionne comme l'identité partagée de votre organisation, pas comme un membre, donc les sessions de canal utilisent uniquement les environnements au niveau de l'organisation, soit les environnements partagés, soit les [environnements auto-hébergés](/docs/fr/self-hosted-environments). Pour donner à un canal une chaîne d'outils qui n'est pas [pré-installée](#installed-tools), comme .NET, un propriétaire peut créer un [environnement partagé](#organization-shared-environments) à partir de la page d'administration **Cloud environments** avec un [script de configuration](#setup-scripts) qui l'installe. Pointez le canal vers un environnement de l'une de deux façons :

* Définissez un environnement partagé ou auto-hébergé comme l'[environnement par défaut](#the-default-environment) de l'organisation sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Épinglez-en un à un canal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) dans les paramètres d'administration Claude Tag.

<h2 id="network-access">
  Accès réseau
</h2>

Chaque environnement définit un niveau d'accès réseau, qui contrôle les connexions sortantes que ses sessions peuvent établir. Le niveau par défaut, **Trusted**, autorise les registres de paquets et autres [domaines autorisés](#default-allowed-domains) ; **Custom** utilise votre propre liste de domaines.

Pour modifier l'accès réseau d'un environnement, [ouvrez-le pour l'édition](#configure-your-environment) et utilisez le sélecteur **Network access** dans la boîte de dialogue. Un [environnement partagé](#organization-shared-environments) s'ouvre en lecture seule là, donc un propriétaire modifie son accès réseau à partir de la page **Cloud environments** dans les [paramètres d'administration](https://claude.ai/admin-settings) à la place. L'icône cloud qui ouvre le sélecteur apparaît sur les surfaces de l'application listées sous [The Default environment](#the-default-environment) et dans l'[éditeur de routines](/docs/fr/routines#environments-and-network-access) ; les environnements personnels n'ont pas de page séparée dans les paramètres de votre compte claude.ai.

<Note>
  Les connecteurs MCP que vous activez sur une session ou une routine fonctionnent sans ajouter leurs hôtes à **Allowed domains**, car le trafic des connecteurs transite par les serveurs d'Anthropic plutôt que par le réseau de la session. Cela s'appuie sur le même canal lié à Anthropic noté sous [Security and isolation](/docs/fr/claude-code-on-the-web#security-and-isolation). Désactivez tout connecteur dont vous n'avez pas besoin pour limiter les outils que Claude peut atteindre.
</Note>

<h3 id="access-levels">
  Niveaux d'accès
</h3>

Le champ **Network access** dans la [boîte de dialogue d'environnement](#configure-your-environment) prend l'un des quatre niveaux suivants :

| Niveau      | Connexions sortantes                                                                                |
| :---------- | :-------------------------------------------------------------------------------------------------- |
| **None**    | Aucun accès réseau sortant via le réseau de la session                                              |
| **Trusted** | [Domaines autorisés](#default-allowed-domains) uniquement : registres de paquets, GitHub, SDK cloud |
| **Full**    | N'importe quel domaine                                                                              |
| **Custom**  | Votre propre liste d'autorisation, incluant optionnellement les domaines par défaut                 |

Quel que soit le niveau que vous choisissez, les sessions peuvent toujours atteindre ceux-ci, car chacun emprunte un chemin qui ne passe pas par la liste d'autorisation réseau de la session :

* GitHub, via son [proxy séparé](#github-proxy)
* Les [connecteurs MCP](#network-access) que vous activez, dont le trafic transite par les serveurs d'Anthropic
* Les hôtes que vous avez listés sur les [identifiants API](#add-api-credentials) de l'environnement, sauf les [hôtes qui ne reçoivent jamais l'identifiant](#requests-that-never-get-the-credential)
* L'API Anthropic, pour les propres requêtes de Claude Code, même au niveau **None**, comme noté sous [Security and isolation](/docs/fr/claude-code-on-the-web#security-and-isolation)

<h3 id="allow-specific-domains">
  Autoriser des domaines spécifiques
</h3>

Pour autoriser des domaines qui ne figurent pas dans la liste Trusted, sélectionnez **Custom** dans les paramètres d'accès réseau de l'environnement, puis listez un domaine par ligne dans le champ **Allowed domains**. Cet exemple autorise trois hôtes qu'un projet interne pourrait nécessiter.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Les sessions dans cet environnement peuvent maintenant atteindre `api.example.com`, n'importe quel sous-domaine de `internal.example.com`, et `registry.example.com`, et aucun autre domaine via le réseau de la session. Le [trafic GitHub](#github-proxy), le [trafic des connecteurs MCP](#network-access), et les requêtes vers les hôtes des [identifiants API](#add-api-credentials) de l'environnement, autres que les [hôtes qui ne reçoivent jamais l'identifiant](#requests-that-never-get-the-credential), ne passent pas par cette liste d'autorisation. Un `*.` au début correspond à tous les sous-domaines. Pour conserver également les [domaines Trusted](#default-allowed-domains), cochez **Also include default list of common package managers** ; laissez-le décoché pour autoriser uniquement ce que vous listez.

Si votre organisation utilise les [artifacts](/docs/fr/artifacts#availability), vous n'avez pas besoin de `*.frame.claudeusercontent.com` dans la liste pour que les sessions les lisent. Lorsque la liste omet cet hôte, Claude Code lit le contenu des artifacts via la connexion de la session à Anthropic à la place. Conservez l'hôte dans une liste d'autorisation dans deux situations :

* **Les sessions dans cet environnement ouvrent les artifacts publics d'une autre organisation** : Claude Code les récupère directement depuis l'hôte, donc ajoutez-le à cette liste.
* **Vous configurez le CLI local ou un runner auto-hébergé** : conservez l'hôte dans cette liste d'autorisation. Voir [network access requirements](/docs/fr/network-config#network-access-requirements) et les [network requirements](/docs/fr/self-hosted-environments-deploy#network-requirements) auto-hébergées.

Chaque environnement a sa propre liste de domaines autorisés ; il n'y a pas de liste d'autorisation au niveau de l'organisation que les administrateurs peuvent pousser aux environnements de chaque membre. Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) s'appliquent toujours dans les sessions cloud, mais aucun d'eux n'ajoute de domaines à la liste d'autorisation réseau de l'environnement. Pour donner à une équipe une liste standard, un propriétaire peut créer un [environnement partagé au niveau de l'organisation](#organization-shared-environments) avec un accès réseau **Custom** et cette liste.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Dans les environnements hébergés par Anthropic, toutes les opérations GitHub passent par un proxy dédié qui garde vos véritables identifiants GitHub en dehors de la VM de la session, indépendamment du [niveau d'accès](#access-levels) de l'environnement. Les sessions dans un environnement auto-hébergé authentifient les opérations git avec les identifiants que votre déploiement fournit ; [Configure git](/docs/fr/self-hosted-environments-deploy#configure-git) couvre les options, y compris les identifiants émis par session et un opt-in pour ce même proxy. Le proxy fournit :

* **Identifiants Git** : le client git à l'intérieur de la VM utilise un identifiant limité en portée, que le proxy vérifie et échange contre votre véritable token GitHub.
* **Requêtes API** : les requêtes des outils GitHub intégrés, et de `gh` sous l'[espace réservé `proxy-injected`](#work-with-github-issues-and-pull-requests), sortent avec vos véritables identifiants substitués.
* **Protection contre les push** : `git push` fonctionne uniquement contre la branche de travail actuelle de la session ; le clonage, la récupération et les opérations PR fonctionnent normalement.
* **Portée du référentiel** : les requêtes API GitHub et les requêtes d'actifs de version n'atteignent que les référentiels attachés à la session, donc un script de configuration qui télécharge des actifs de version à partir d'un référentiel non attaché reçoit un 403.
* **Restrictions GraphQL** : le proxy ne sert qu'un ensemble épinglé d'opérations GraphQL pour les flux de travail de pull-request. Le proxy rejette tout le reste sur le point de terminaison GraphQL avec un 403 qui dit `This GraphQL query is not enabled for this session` et nomme le fallback REST, `gh api repos/{owner}/{repo}/...`. La restriction s'applique à chaque requête via le proxy quel que soit l'identifiant que vous fournissez, donc un `GH_TOKEN` que vous définissez reçoit le même 403. Claude ne peut pas atteindre les API GitHub qui n'existent que dans GraphQL, comme Projects v2, via le proxy.

Les fichiers validés des référentiels publics arrivent via `raw.githubusercontent.com`, que le [proxy de sécurité](#security-proxy) gère à la place. Ce domaine figure dans la liste [Trusted](#default-allowed-domains) par défaut, donc ces fichiers restent accessibles sauf si le [niveau d'accès](#access-levels) de l'environnement l'exclut.

<h3 id="security-proxy">
  Proxy de sécurité
</h3>

Les sessions cloud dans les environnements hébergés par Anthropic s'exécutent derrière un proxy réseau HTTP/HTTPS à des fins de sécurité et de prévention des abus ; dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments-deploy#default-deny-egress), le trafic sortant quitte via votre propre limite réseau à la place. Tout le trafic Internet sortant d'une session hébergée par Anthropic passe par ce proxy, qui fournit :

* Protection contre les requêtes malveillantes
* Limitation de débit et prévention des abus
* Filtrage de contenu pour une sécurité renforcée
* Un journal d'audit au niveau DNS des noms d'hôtes demandés

<h2 id="what’s-available-in-cloud-sessions">
  Ce qui est disponible dans les sessions cloud
</h2>

Dans les environnements hébergés par Anthropic, chaque session obtient une machine virtuelle (VM) fraîche exécutant Ubuntu 24.04 sur x86\_64, quel que soit votre propre système d'exploitation et architecture CPU, avec votre dépôt cloné et les chaînes d'outils courantes pré-installées. Quand une dépendance fournit des binaires précompilés, comme les gems Ruby avec des extensions natives ou les roues Python précompilées, utilisez sa compilation Linux x86\_64 pour correspondre à la VM. Cette section couvre les défauts hébergés par Anthropic, les outils GitHub intégrés, comment [exécuter des tests et des services](#run-tests-start-services-and-add-packages), et les [limites de ressources](#resource-limits) que chaque VM obtient.

<Note>
  Les sessions que votre organisation route vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments) s'exécutent sur vos propres exécuteurs à la place, avec les outils que votre image d'exécuteur fournit.
</Note>

<h3 id="what-carries-over-from-your-setup">
  Ce qui est reporté de votre configuration
</h3>

Les sessions cloud commencent à partir d'un clone frais de votre dépôt. Tout ce que vous validez dans le dépôt est disponible. Tout ce que vous avez installé ou configuré seulement sur votre propre machine n'est pas disponible dans la session. La politique de votre organisation arrive séparément via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings).

|                                                                                                                                                                                                       | Disponible dans les sessions cloud                                            | Pourquoi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Votre `CLAUDE.md` du dépôt                                                                                                                                                                            | Oui                                                                           | Partie du clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Vos crochets `.claude/settings.json` du dépôt et règles de permission                                                                                                                                 | Oui, dans une session avec un seul dépôt                                      | Partie du clone. Une session avec plusieurs dépôts, incluant un fil de [projet](/docs/fr/claude-projects#what-threads-pick-up-from-your-repositories), démarre au-dessus des clones et ne les lit pas                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Vos serveurs MCP `.mcp.json` du dépôt                                                                                                                                                                 | Oui, dans une session avec un seul dépôt                                      | Partie du clone, trouvé à partir du répertoire de travail de la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Votre `.claude/rules/` du dépôt                                                                                                                                                                       | Oui                                                                           | Partie du clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Votre `.claude/skills/`, `.claude/agents/`, `.claude/commands/` du dépôt                                                                                                                              | Oui                                                                           | Partie du clone                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Plugins et places de marché déclarés dans le `.claude/settings.json` de votre dépôt                                                                                                                   | Non                                                                           | Une session cloud n'installe pas les plugins qu'un dépôt active sous [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins), y compris ceux des places de marché qu'il liste sous [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces)                                                                                                                                                                                                                                                                                                                                                                                                       |
| Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de votre organisation                                                                                                              | Oui                                                                           | Récupérés des serveurs d'Anthropic quand la session démarre. Consultez [Couverture de surface](/docs/fr/model-config#surface-coverage) pour savoir comment `availableModels` est appliqué dans les sessions cloud. Les paramètres déployés sur votre appareil via MDM ou des fichiers de paramètres gérés ne s'appliquent pas, car la session s'exécute sur une VM gérée par Anthropic ; dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments), les sessions lisent également le fichier de paramètres gérés dans l'image d'exécuteur, selon [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) |
| Votre `~/.claude/CLAUDE.md` utilisateur                                                                                                                                                               | Non                                                                           | Vit sur votre machine, pas dans le dépôt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Vos `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` utilisateur                                                                                                                       | Non                                                                           | Vivent sur votre machine, pas dans le dépôt. Validez-les dans le répertoire `.claude/` du dépôt à la place. Les sessions cloud chargent automatiquement les compétences que vous activez sur claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Plugins activés seulement dans vos paramètres utilisateur                                                                                                                                             | Non                                                                           | L'`enabledPlugins` au niveau utilisateur vit dans `~/.claude/settings.json` sur votre machine                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Serveurs MCP que vous avez ajoutés avec `claude mcp add` à la portée locale par défaut ou à la portée utilisateur                                                                                     | Non                                                                           | Ceux-ci écrivent dans `~/.claude.json` sur votre machine, pas le dépôt. Ajoutez le serveur avec `claude mcp add --scope project`, qui écrit le [`.mcp.json`](/docs/fr/mcp#project-scope) du dépôt, et validez ce fichier. Une session avec un seul dépôt le charge                                                                                                                                                                                                                                                                                                                                                                                                        |
| Variables de transport dans le bloc `env` de `.claude/settings.json` de votre dépôt, comme `NODE_EXTRA_CA_CERTS` et les [variables de certificat client mTLS](/docs/fr/network-config#mtls-authentication) | Non                                                                           | L'environnement d'hébergement gère la connexion API de la session, donc Claude Code ignore ces clés et note chaque clé ignorée dans le journal de débogage de la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Clés API et jetons pour les services que Claude appelle                                                                                                                                               | Sur les plans Pro et Max, en tant qu'[identifiants API](#add-api-credentials) | Vous ajoutez la clé une fois sur l'environnement et le proxy d'agent la joint aux requêtes pour les hôtes que vous listez. Une clé que le proxy d'agent [ne peut pas joindre](#requests-that-never-get-the-credential), ou n'importe quelle clé sur un plan Team ou Enterprise, reste dans une variable d'environnement                                                                                                                                                                                                                                                                                                                                              |
| Authentification interactive comme AWS SSO                                                                                                                                                            | Non                                                                           | Non supporté. SSO nécessite une connexion basée sur un navigateur qui ne peut pas s'exécuter dans une session cloud                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

Pour rendre votre propre configuration disponible dans les sessions cloud, validez-la dans le dépôt.

Quiconque utilise l'environnement peut lire ses variables d'environnement et son script de configuration. La note de la boîte de dialogue sous **Environment variables** le dit et avertit contre l'ajout de secrets dedans. Sur les plans Pro et Max, stockez une clé que le proxy d'agent peut joindre en tant qu'[identifiant API](#add-api-credentials) à la place.

<h3 id="installed-tools">
  Outils installés
</h3>

Les sessions cloud sont livrées avec les runtimes de langage courants, les outils de construction et les bases de données pré-installés. Le tableau ci-dessous résume ce qui est inclus par catégorie.

| Catégorie            | Inclus                                                                   |
| :------------------- | :----------------------------------------------------------------------- |
| **Python**           | Python 3.x avec pip, poetry, uv, black, mypy, pytest, ruff               |
| **Node.js**          | 20, 21 et 22, avec npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**             | 3.1, 3.2, 3.3 avec gem, bundler, rbenv                                   |
| **PHP**              | 8.3 avec Composer                                                        |
| **Java**             | OpenJDK 21 avec Maven et Gradle                                          |
| **Go**               | Go avec support des modules                                              |
| **Rust**             | rustc et cargo                                                           |
| **C/C++**            | GCC, Clang, cmake, ninja, conan                                          |
| **Docker**           | docker, dockerd, docker compose                                          |
| **Bases de données** | PostgreSQL 16, Redis 7.0                                                 |
| **Utilitaires**      | git, gh, jq, yq, ripgrep, tmux, vim, nano                                |

¹ Bun est installé mais a des [problèmes de compatibilité proxy](#install-dependencies-with-a-sessionstart-hook) connus pour la récupération de paquets.

Pour obtenir les versions de la plupart des outils de ce tableau, demandez à Claude d'exécuter `check-tools` dans une session cloud. C'est une commande shell installée sur la VM de la session, pas une commande que vous tapez avec `/` ; vous demandez à Claude car [Claude exécute toutes les commandes VM pour vous](#run-tests-start-services-and-add-packages). Pour un outil qu'il ne rapporte pas, comme Ruby, PHP, bun, PostgreSQL ou Redis, demandez à Claude d'exécuter la propre commande de version de l'outil, par exemple `psql --version`.

Les versions de Node.js sont installées sur `/opt/node20`, `/opt/node21` et `/opt/node22`, avec 22 sur `PATH` par défaut. Pour travailler avec une version différente, demandez à Claude de préfixer le répertoire `bin` de cette version, comme `/opt/node20/bin`, à `PATH`.

Les chaînes d'outils en dehors de cette liste, comme le SDK .NET, ne sont pas pré-installées même quand leurs registres de paquets sont sur la [liste d'autorisation par défaut](#default-allowed-domains). Installez-les avec un [script de configuration](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Travaillez avec les problèmes et les demandes de tirage GitHub
</h3>

Les sessions cloud incluent des outils GitHub intégrés qui permettent à Claude de lire les problèmes, de lister les demandes de tirage, de récupérer les diffs et de publier des commentaires sans aucune configuration. Ces outils s'authentifient via le [proxy GitHub](#github-proxy) en utilisant la méthode que vous avez configurée sous [Options d'authentification GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options), donc votre jeton ne pénètre jamais dans le conteneur.

Vous pouvez définir `GH_TOKEN` ou `GITHUB_TOKEN` vous-même dans les [paramètres d'environnement](#set-environment-variables), ou laisser les deux non définis et laisser le [proxy GitHub](#github-proxy) s'authentifier pour vous :

* Si vous définissez un jeton, il passe au conteneur inchangé, donc vos scripts et le [`gh` CLI](https://cli.github.com) de GitHub l'utilisent directement.
* Si vous ne définissez ni l'un ni l'autre et que le [proxy GitHub](#github-proxy) gère l'authentification pour votre session, les deux variables lisent comme la chaîne d'espace réservé `proxy-injected` dans les commandes que Claude exécute, et le proxy substitue vos vrais identifiants sur les requêtes GitHub sortantes. `gh` fonctionne sans un jeton de votre côté, mais un script qui lit `GITHUB_TOKEN` directement obtient l'espace réservé, pas un jeton utilisable.

Un jeton que vous définissez est une variable d'environnement ordinaire, donc quiconque utilise l'environnement peut le lire ; le chemin du proxy garde l'identifiant hors de la configuration de l'environnement et de la VM de la session.

Pour vérifier quel cas s'applique à votre session, demandez à Claude d'exécuter `echo $GH_TOKEN`.

Le [`gh` CLI](https://cli.github.com) de GitHub est pré-installé. Si vous avez besoin d'une commande `gh` que les outils intégrés ne couvrent pas, comme `gh release` ou `gh workflow run`, demandez à Claude de l'exécuter. `gh` lit `GH_TOKEN` automatiquement, donc vous n'avez pas besoin d'exécuter `gh auth login`.

<h3 id="link-output-back-to-the-session">
  Liez la sortie à la session
</h3>

Chaque session cloud a une URL de transcription sur claude.ai, et la session peut lire son propre ID à partir de la variable d'environnement `CLAUDE_CODE_REMOTE_SESSION_ID`. Utilisez ceci pour mettre un lien traçable dans les corps PR, les messages de validation, les publications Slack ou les rapports générés afin qu'un examinateur puisse ouvrir l'exécution qui les a produits.

Les validations que Claude crée dans une session cloud incluent une remorque git `Claude-Session: <url>`, et les corps PR incluent l'URL de la session sur sa propre ligne. Pour omettre la remorque et le lien du corps PR, définissez [`attribution.sessionUrl`](/docs/fr/settings-reference#attribution-sessionurl) sur `false`.

Pour inclure le lien de session dans quelque chose d'autre qu'une validation ou une PR, comme un message Slack que Claude publie ou un fichier de rapport qu'il écrit, demandez à Claude d'exécuter la commande suivante et d'utiliser sa sortie. La commande convertit le préfixe `cse_` dans la valeur de la variable d'environnement au préfixe `session_` que l'URL de transcription attend :

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Exécutez des tests, démarrez des services et ajoutez des paquets
</h3>

Vous n'avez pas d'accès shell à la VM de la session. Claude exécute chaque commande pour vous, donc formulez les tâches de cette section comme des demandes dans votre invite.

<h4 id="run-tests">
  Exécutez des tests
</h4>

Claude exécute les tests dans le cadre du travail sur une tâche. Demandez-le dans votre invite, comme « corriger les tests échoués dans `tests/` » ou « exécuter pytest après chaque modification ». Les exécuteurs de tests qui viennent avec les [chaînes d'outils pré-installées](#installed-tools), comme pytest et cargo test, fonctionnent sans configuration supplémentaire. Un exécuteur que votre projet déclare comme dépendance, comme jest, s'installe avec vos dépendances.

<h4 id="start-services">
  Démarrez des services
</h4>

PostgreSQL et Redis sont pré-installés mais ne s'exécutent pas par défaut. Demandez à Claude de démarrer celui dont vous avez besoin ; les commandes qu'il exécute sont :

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker est disponible pour exécuter des services conteneurisés. Demandez à Claude d'exécuter `docker compose up` pour démarrer les services de votre projet. L'accès réseau pour extraire les images suit le [niveau d'accès](#access-levels) de votre environnement, et les [défauts Trusted](#default-allowed-domains) incluent Docker Hub et d'autres registres courants.

Si vos images sont grandes ou lentes à extraire, ajoutez `docker compose pull` ou `docker compose build` à votre [script de configuration](#setup-scripts). Le [cache d'environnement](#environment-caching) conserve les images extraites, donc chaque nouvelle session les a sur le disque. Le cache stocke seulement les fichiers, pas les processus en cours d'exécution, donc Claude démarre toujours les conteneurs chaque session.

<h4 id="add-packages">
  Ajoutez des paquets
</h4>

Pour ajouter des paquets qui ne sont pas pré-installés, utilisez un [script de configuration](#setup-scripts). Le [cache d'environnement](#environment-caching) conserve ce que le script installe, donc les paquets que vous installez là sont disponibles au démarrage de chaque session sans réinstallation à chaque fois. Vous pouvez aussi demander à Claude d'installer des paquets en milieu de session, mais ces installations ne se reportent pas à d'autres sessions.

<h3 id="resource-limits">
  Limites de ressources
</h3>

Les sessions cloud dans les environnements hébergés par Anthropic s'exécutent avec des plafonds de ressources approximatifs qui peuvent changer au fil du temps :

* 4 vCPU
* 16 Go de RAM
* 30 Go de disque

La VM peut arrêter les tâches qui ont besoin de beaucoup plus de mémoire, comme les gros travaux de construction ou les tests gourmands en mémoire. Pour les charges de travail au-delà de ces limites, utilisez [Remote Control](/docs/fr/remote-control) pour exécuter Claude Code sur votre propre matériel, ou exécutez les sessions cloud dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments) sur le calcul que votre organisation exploite.

<h2 id="setup-scripts">
  Scripts de configuration
</h2>

Un script de configuration est un script Bash qui s'exécute quand une nouvelle session cloud démarre, avant le lancement de Claude Code. Utilisez les scripts de configuration pour installer les dépendances, configurer les outils ou récupérer tout ce dont la session a besoin qui n'est pas pré-installé.

Les scripts s'exécutent en tant que root sur Ubuntu 24.04, donc `apt install` et la plupart des gestionnaires de paquets de langage fonctionnent.

Pour ajouter un script de configuration, ouvrez la boîte de dialogue des paramètres d'environnement et entrez votre script dans le champ **Setup script**.

Cet exemple installe [ShellCheck](https://www.shellcheck.net/), qui n'est pas pré-installé.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Exigences du script
</h3>

Un script de configuration a trois contraintes à contourner :

* **Quitter zéro** : si le script quitte non-zéro, la session échoue à démarrer. Ajoutez `|| true` aux commandes non critiques afin qu'une défaillance d'installation intermittente ne bloque pas la session.
* **Terminer en cinq minutes** : gardez le temps d'exécution total du script sous environ cinq minutes afin que le [cache d'environnement](#environment-caching) puisse se construire. Exécutez les installations indépendantes en parallèle avec `&` et `wait`, et déplacez tout téléchargement unique qui ne rentre pas dans un crochet [SessionStart](#setup-scripts-vs-sessionstart-hooks) qui le lance en arrière-plan.
* **Accès réseau pour les installations** : les installations de paquets doivent atteindre les registres. Le niveau **Trusted** par défaut couvre les [registres de paquets courants](#default-allowed-domains) incluant npm, PyPI, RubyGems et crates.io ; avec un accès réseau **None**, les installations échouent.

<h3 id="environment-caching">
  Mise en cache d'environnement
</h3>

Le script de configuration s'exécute la première fois que vous démarrez une session dans un environnement. Après sa fin, Anthropic crée un instantané du système de fichiers et réutilise cet instantané comme point de départ pour les sessions ultérieures. Les nouvelles sessions commencent avec vos dépendances, outils et images Docker déjà sur le disque, et sautent l'étape du script de configuration. Cela garde le démarrage rapide même quand le script installe de grandes chaînes d'outils ou extrait des images de conteneur.

Le cache est un instantané du système de fichiers, donc il conserve ce que le script de configuration écrit sur le disque et perd tout ce qui était seulement en cours d'exécution. Les paquets que vous installez, les images Docker que vous extrayez et les fichiers que vous écrivez se reportent tous. Une base de données que le script a démarrée, une pile `docker compose up`, ou tout autre processus en arrière-plan ne le fait pas ; démarrez-les par session en demandant à Claude ou avec un crochet [SessionStart](#setup-scripts-vs-sessionstart-hooks).

Le script de configuration s'exécute à nouveau pour reconstruire le cache quand vous modifiez le script de configuration de l'environnement ou les hôtes réseau autorisés, et quand le cache atteint son expiration après environ sept jours. Reprendre une session existante ne réexécute jamais le script de configuration.

Vous n'avez pas besoin d'activer la mise en cache ou de gérer les instantanés vous-même.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuration vs. crochets SessionStart
</h3>

Utilisez un script de configuration pour provisionner la VM elle-même : les chaînes d'outils et les outils CLI qui ne sont pas [pré-installés](#installed-tools). Utilisez un crochet [SessionStart](/docs/fr/hooks#sessionstart) pour la configuration du projet qui devrait s'exécuter partout, cloud et local, comme `npm install`.

Les scripts de configuration et les crochets SessionStart s'exécutent dans un ordre fixe quand une session cloud démarre. Le tableau compare où vous les configurez, quand ils s'exécutent et où ils s'exécutent.

|                            | Scripts de configuration                                                                                                                                                                                      | Crochets SessionStart                                                                                                                                                                                                                                            |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Où vous les configurez** | La boîte de dialogue d'environnement sur [claude.ai/code](https://claude.ai/code), plus la page d'administration **Cloud environments** pour les [environnements partagés](#organization-shared-environments) | Un [fichier de paramètres](/docs/fr/settings#where-settings-live) comme le `.claude/settings.json` de votre dépôt ; consultez [Ce qui est reporté de votre configuration](#what-carries-over-from-your-setup) pour savoir quels fichiers atteignent une session cloud |
| **Quand ils s'exécutent**  | Avant le lancement de Claude Code, sautés quand un [environnement en cache](#environment-caching) existe                                                                                                      | Après le lancement de Claude Code, sur chaque session incluant la reprise                                                                                                                                                                                        |
| **Où ils s'exécutent**     | Sessions cloud uniquement                                                                                                                                                                                     | Sessions locales et cloud                                                                                                                                                                                                                                        |

Si vous avez des crochets SessionStart dans votre `~/.claude/settings.json` au niveau utilisateur, ne vous attendez pas à les voir dans le cloud : les paramètres au niveau utilisateur restent sur votre machine. Quels autres crochets s'exécutent dépend de l'endroit où la session s'exécute :

* **Environnement hébergé par Anthropic** : Claude Code exécute les crochets du dépôt et de vos [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de l'organisation.
* **[Environnement auto-hébergé](/docs/fr/self-hosted-environments-configuration#permissions-and-tool-approval)** : Claude Code exécute également les crochets que l'opérateur a ensemencés à partir de `~/.claude/` de l'hôte d'exécuteur, et les crochets dans le fichier de paramètres gérés de l'image d'exécuteur quand ce fichier est l'une des [sources gérées que Claude Code applique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Installez les dépendances avec un crochet SessionStart
</h3>

Pour installer les dépendances seulement dans les sessions cloud, associez un crochet SessionStart avec un script qui vérifie où il s'exécute.

Tout d'abord, ajoutez un crochet SessionStart au `.claude/settings.json` de votre dépôt. Cette configuration dit à Claude Code d'exécuter `scripts/install_pkgs.sh` à partir de votre dépôt chaque fois qu'une session démarre ou reprend :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Le `matcher` limite le crochet aux événements `startup` et `resume`, et `$CLAUDE_PROJECT_DIR` se résout à la racine du dépôt, donc le crochet trouve le script quel que soit le répertoire de travail de la session.

Ensuite, créez le script sur `scripts/install_pkgs.sh`. Il quitte immédiatement en dehors du cloud, puis installe vos dépendances :

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

La vérification `CLAUDE_CODE_REMOTE` est ce qui limite l'installation aux sessions cloud : la VM de la session porte cette variable comme `true`, elle n'est jamais `true` localement, donc sur votre ordinateur portable le script quitte avant d'installer quoi que ce soit.

Ensemble, les deux fichiers donnent à chaque session cloud un `npm install` et `pip install` frais au démarrage tout en laissant les sessions locales intactes.

<h4 id="limitations-in-cloud-sessions">
  Limitations dans les sessions cloud
</h4>

Les crochets SessionStart se comportent de la même manière dans le cloud qu'en local, avec ces mises en garde :

* **Un dépôt par session** : une session avec plusieurs dépôts ne charge pas les crochets du `.claude/settings.json` d'aucun dépôt, donc un crochet SessionStart que vous définissez là ne s'exécute pas. Installez les dépendances pour ces sessions avec un [script de configuration](#setup-scripts) à la place.
* **Pas de limitation au cloud uniquement** : les crochets s'exécutent dans les sessions locales et cloud. Pour ignorer l'exécution locale, quittez tôt sauf si la variable d'environnement `CLAUDE_CODE_REMOTE` est `true`, de la même manière que le [script d'installation de dépendances](#install-dependencies-with-a-sessionstart-hook) le fait.
* **Nécessite un accès réseau** : les commandes d'installation doivent atteindre les registres de paquets. Si votre environnement utilise un accès réseau **None**, ces crochets échouent. La [liste d'autorisation par défaut](#default-allowed-domains) sous **Trusted** couvre npm, PyPI, RubyGems et crates.io.
* **Compatibilité proxy** : dans les environnements hébergés par Anthropic, tout le trafic sortant passe par un [proxy de sécurité](#security-proxy), et certains gestionnaires de paquets ne fonctionnent pas correctement avec lui ; Bun est un exemple connu. Dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments-deploy#default-deny-egress), le trafic sortant va via votre propre limite réseau à la place.
* **Ajoute une latence de démarrage** : les crochets s'exécutent chaque fois qu'une session démarre ou reprend, contrairement aux scripts de configuration qui bénéficient de la [mise en cache d'environnement](#environment-caching). Gardez les scripts d'installation rapides en vérifiant si les dépendances sont déjà présentes avant de réinstaller.

Pour personnaliser l'image de base, utilisez un script de configuration pour installer ce dont vous avez besoin en haut de l'[image fournie](#installed-tools), ou exécutez votre propre image en tant que conteneur à côté de Claude avec `docker compose`. Remplacer l'image de base entièrement n'est pas encore supporté.

<h2 id="default-allowed-domains">
  Domaines autorisés par défaut
</h2>

Avec un accès réseau **Trusted**, les sessions peuvent atteindre les domaines suivants par défaut. Les domaines marqués avec `*` indiquent une correspondance de sous-domaine générique, donc `*.gcr.io` autorise n'importe quel sous-domaine de `gcr.io`.

<AccordionGroup>
  <Accordion title="Services Anthropic">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Contrôle de version">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registres de conteneurs">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Plateformes cloud">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="Gestionnaires de paquets JavaScript et Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Gestionnaires de paquets Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Gestionnaires de paquets Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Gestionnaires de paquets Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Gestionnaires de paquets Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Gestionnaires de paquets JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Autres gestionnaires de paquets">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distributions Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Outils de développement et plateformes">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Services cloud et surveillance">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Livraison de contenu et miroirs">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schéma et configuration">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Référence des sessions cloud](/docs/fr/claude-code-on-the-web) : démarrez, gérez et partagez les sessions cloud
* [Démarrage rapide des sessions cloud](/docs/fr/web-quickstart) : connectez GitHub et démarrez votre première session cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview) : les sessions que Claude démarre à partir de Slack s'exécutent dans les mêmes environnements
* [Routines](/docs/fr/routines) : les exécutions programmées utilisent les mêmes environnements et niveaux d'accès réseau
* [Remote Control](/docs/fr/remote-control) : exécutez les sessions sur le réseau et les fichiers de votre propre machine à la place
* [Environnements auto-hébergés](/docs/fr/self-hosted-environments) : exécutez les sessions cloud sur l'infrastructure propre de votre organisation
* [Crochets SessionStart](/docs/fr/hooks#sessionstart) : configuration validée dans le dépôt qui s'exécute dans les sessions locales et cloud
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : politique organisationnelle qui atteint les sessions cloud
