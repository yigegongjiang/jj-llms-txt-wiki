> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration réseau d'entreprise

> Configurez Claude Code pour les environnements d'entreprise avec des serveurs proxy, des autorités de certification (CA) personnalisées et l'authentification mutuelle Transport Layer Security (mTLS).

Claude Code prend en charge diverses configurations réseau et de sécurité d'entreprise via des variables d'environnement. Cela inclut le routage du trafic via des serveurs proxy d'entreprise, la confiance envers des autorités de certification (CA) personnalisées et l'authentification avec des certificats mTLS (Transport Layer Security mutuel) pour une sécurité renforcée.

Définissez ces variables d'environnement avant de lancer Claude Code. Les variables exportées dans votre shell sont lues une seule fois au démarrage, donc une session en cours ne récupère pas les modifications ultérieures de votre environnement shell.

<Note>
  Toutes les variables d'environnement affichées sur cette page peuvent également être configurées dans [`settings.json`](/docs/fr/settings).
</Note>

<h2 id="proxy-configuration">
  Configuration du proxy
</h2>

<h3 id="environment-variables">
  Variables d'environnement
</h3>

Claude Code respecte les variables d'environnement proxy standard. Dans les sessions Claude Desktop où l'application gère la connexion du fournisseur, Claude Code les lit uniquement à partir des paramètres gérés et de `~/.claude/settings.json` ; voir [authentification mTLS](#mtls-authentication) pour les règles de portée.

```bash theme={null}
# Proxy HTTPS (recommandé)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (si HTTPS non disponible)
export HTTP_PROXY=http://proxy.example.com:8080

# Contourner le proxy pour des requêtes spécifiques - format séparé par des espaces
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Contourner le proxy pour des requêtes spécifiques - format séparé par des virgules
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Contourner le proxy pour toutes les requêtes
export NO_PROXY="*"
```

Les variantes en minuscules fonctionnent également, et Claude Code utilise la première qui est définie dans l'ordre `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY`.

Claude Code n'envoie jamais ses connexions WebSocket à `localhost`, `::1`, ou `127.0.0.0/8` via le proxy, vous n'avez donc pas besoin d'une entrée de boucle locale dans `NO_PROXY` pour eux.

<Note>
  Claude Code ne prend pas en charge les proxies SOCKS.
</Note>

<h3 id="basic-authentication">
  Authentification basique
</h3>

Si votre proxy nécessite une authentification basique, incluez les identifiants dans l'URL du proxy :

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Évitez de coder en dur les mots de passe dans les scripts. Utilisez plutôt des variables d'environnement ou un stockage sécurisé des identifiants.
</Warning>

<Tip>
  Pour les proxies nécessitant une authentification avancée (NTLM, Kerberos, etc.), envisagez d'utiliser un service LLM Gateway qui prend en charge votre méthode d'authentification.
</Tip>

<h2 id="ca-certificate-store">
  Magasin de certificats CA
</h2>

Par défaut, Claude Code fait confiance à la fois aux certificats CA Mozilla fournis avec le produit et au magasin de certificats de votre système d'exploitation. La lecture du magasin du système d'exploitation nécessite un runtime avec `tls.getCACertificates` : l'installateur natif l'a toujours, et les installations npm nécessitent Node 22.15 ou une version ultérieure. Sur les versions plus anciennes de Node, seul l'ensemble fourni et `NODE_EXTRA_CA_CERTS` s'appliquent. Les proxies d'inspection TLS d'entreprise fonctionnent sans configuration supplémentaire lorsque leur certificat racine est installé dans le magasin de confiance du système d'exploitation et que le runtime peut le lire.

`CLAUDE_CODE_CERT_STORE` accepte une liste séparée par des virgules de sources. Les valeurs reconnues sont `bundled` pour l'ensemble de certificats CA Mozilla fourni avec Claude Code et `system` pour le magasin de confiance du système d'exploitation. La valeur par défaut est `bundled,system`.

Pour approuver uniquement l'ensemble de certificats CA Mozilla fourni :

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Pour approuver uniquement le magasin de certificats du système d'exploitation :

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` n'a pas de clé de schéma dédiée dans `settings.json`. Définissez-la via le bloc `env` dans `~/.claude/settings.json` ou directement dans l'environnement du processus.
</Note>

<h2 id="custom-ca-certificates">
  Certificats CA personnalisés
</h2>

Si votre environnement d'entreprise utilise une CA personnalisée, configurez Claude Code pour la faire confiance directement :

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Authentification mTLS
</h2>

Pour les environnements d'entreprise nécessitant une authentification par certificat client :

```bash theme={null}
# Certificat client pour l'authentification
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Clé privée du client
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Optionnel : phrase de passe pour la clé privée chiffrée
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code lit les fichiers de certificat et de clé au démarrage et les relit chaque fois qu'il applique les paramètres, par exemple lorsque votre organisation modifie le bloc `env` dans les [paramètres gérés](/docs/fr/server-managed-settings) en cours de session.

Pour faire tourner le certificat et la clé, remplacez les fichiers aux mêmes chemins. Claude Code récupère le remplacement dans une session en cours sans redémarrage. Lorsqu'une demande d'API échoue avec une erreur au niveau de la connexion, comme une réinitialisation de connexion ou une erreur de négociation TLS, il relit les deux fichiers et réessaie la demande avec la nouvelle paire. Avant la v2.1.232, Claude Code ne relisait pas en cas d'erreurs de connexion, il conservait donc la paire qu'il avait déjà chargée jusqu'à la prochaine application des paramètres ou jusqu'à votre redémarrage.

Claude Code relit les fichiers en réponse aux demandes échouées, pas en les surveillant pour détecter les modifications :

* **Timing** : Claude Code ne fait rien au moment où vous remplacez les fichiers. Il présente la nouvelle paire lors de la nouvelle tentative après un échec admissible, ou lors de la prochaine demande après application des paramètres, selon ce qui se produit en premier.
* **Rejets de passerelle** : Claude Code relit lorsque votre passerelle réinitialise la connexion ou rejette la négociation TLS après avoir cessé d'accepter l'ancienne paire. Il ne relit pas lorsque la passerelle termine la négociation et répond avec une erreur HTTP. Dans ce cas, Claude Code charge la nouvelle paire lors de la prochaine application des paramètres ou lorsque vous le redémarrez.
* **Rotations partiellement écrites** : lorsque Claude Code relit pendant que votre rotation est en cours d'écriture, par exemple en lisant un certificat et une clé qui ne correspondent pas l'un à l'autre, il conserve la paire précédente et relit lors de l'échec suivant.
* **Exportateurs de télémétrie OTLP** : Claude Code conserve le certificat que les [exportateurs](/docs/fr/monitoring-usage#mtls-authentication) ont chargé à la première utilisation, redémarrez donc Claude Code pour qu'un certificat rotatif atteigne votre collecteur de télémétrie.
* **Désactiver le rechargement** : définissez [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/fr/env-vars#variables) pour désactiver la relecture en cas d'erreur de connexion. Claude Code récupère ensuite les fichiers rotatifs uniquement lors de la prochaine application des paramètres ou au prochain démarrage.

Pour confirmer que Claude Code a récupéré une rotation, [démarrez la session avec la journalisation de débogage](#verify-your-configuration) et recherchez `Stale connection — reloaded rotated mTLS client material` dans le journal. Claude Code ne journalise pas cette ligne lorsqu'il récupère la rotation lors de l'application des paramètres à la place, donc une ligne manquante seule ne signifie pas que la rotation a échoué.

Remplacez les fichiers avant l'expiration de la paire actuelle afin que Claude Code ne charge pas une paire déjà expirée au prochain démarrage.

Dans les [sessions cloud](/docs/fr/claude-code-on-the-web), l'environnement d'hébergement gère la connexion à l'API, donc Claude Code ignore les variables suivantes lorsqu'elles proviennent d'un bloc `env` du fichier de paramètres :

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code note chaque clé ignorée dans le journal de débogage de la session.

Dans les sessions [Claude Desktop](/docs/fr/desktop) où l'application gère la connexion du fournisseur, comme l'onglet Code sur un [fournisseur tiers](/docs/fr/third-party-integrations) et les sessions Cowork, Claude Code lit ces variables et les variables proxy `HTTP_PROXY`, `HTTPS_PROXY` et `NO_PROXY` uniquement à partir des [paramètres gérés](/docs/fr/managed-settings) et `~/.claude/settings.json` : il les ignore dans les fichiers de paramètres propres d'un référentiel, donc un référentiel extrait ne peut pas rediriger le chemin TLS ou proxy d'une session dont les identifiants proviennent de l'application. Dans une session d'onglet Code local, SSH ou WSL connectée via claude.ai, l'application ne gère pas la connexion, et Claude Code lit ces variables à partir de chaque portée de paramètres, comme n'importe quelle session de terminal ; les [sessions cloud](/docs/fr/claude-code-on-the-web) suivent les règles de session cloud ci-dessus où que vous les démarriez. Avant la v2.1.217, Claude Code ignorait ces variables dans chaque fichier de paramètres lorsque l'application gérait la connexion.

<h2 id="verify-your-configuration">
  Vérifier votre configuration
</h2>

Vous découvrez généralement une adresse proxy incorrecte ou un mauvais chemin de certificat à partir d'une [erreur de connexion ou de certificat](/docs/fr/errors#network-and-connection-errors) lors d'une demande ultérieure, car Claude Code ne valide pas la plupart de ces paramètres lors de leur lecture. Le seul paramètre qu'il vérifie au démarrage est l'URL du proxy : quand il ne peut pas analyser la valeur, par exemple une valeur manquant le schéma `http://`, Claude Code arrête le lancement avec une erreur nommant la variable à corriger.

Pour confirmer que votre configuration a été chargée avant d'envoyer une demande, démarrez Claude Code avec la journalisation de débogage :

```bash theme={null}
claude --debug
```

La sortie de débogage va dans `~/.claude/debug/<session-id>.txt` plutôt que dans le terminal, ou dans un chemin que vous définissez avec `--debug-file <path>`. Dans le journal, recherchez les lignes qui confirment le chargement de chaque fichier :

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Si Claude Code ne peut pas lire l'un de ces fichiers, le journal affiche une ligne `Failed to read` ou `Failed to load` avec la raison à la place.

Vous pouvez également exécuter `/status` dans une session interactive et vérifier ces lignes :

* **Proxy** : affiche l'URL du proxy actif et marque une valeur qu'il ne peut pas analyser comme invalide et ignorée.
* **mTLS client cert** et **mTLS client key** : n'apparaissent que lorsque les fichiers sont chargés, donc une ligne manquante signifie que le chargement a échoué et le journal de débogage contient la raison.
* **Additional CA cert(s)** : affiche le chemin `NODE_EXTRA_CA_CERTS` sans vérifier que le fichier a été chargé, donc confirmez celui-ci dans le journal de débogage.

<h2 id="apply-network-settings-to-background-agents">
  Appliquer les paramètres réseau aux agents en arrière-plan
</h2>

[Les agents en arrière-plan](/docs/fr/agent-view) ne s'exécutent pas dans le terminal qui les a lancés. Un processus superviseur par utilisateur démarre à la demande, survit à votre shell et héberge chaque session `claude agents`, `--bg` et `/background`. Voir [Comment les sessions en arrière-plan sont hébergées](/docs/fr/agent-view#how-background-sessions-are-hosted). Cela change la façon dont la configuration de cette page atteint ces sessions.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Définir les variables réseau dans les paramètres, pas dans le shell
</h3>

Le superviseur est un processus unique partagé par chaque terminal. Il hérite de l'environnement du shell qui le démarre en premier, et un superviseur installé par le système d'exploitation ne reçoit aucun environnement shell du tout. Si vous exportez une variable proxy, chemin CA ou mTLS uniquement dans votre shell, elle atteint les agents en arrière-plan quand ce shell a démarré à froid le superviseur, et silencieusement ne le fait pas quand un shell différent l'a fait.

Mettez plutôt les mêmes variables dans le bloc `env` de `~/.claude/settings.json` ou [paramètres gérés](/docs/fr/settings). Chaque variable de cette page peut y être définie, et les paramètres sont la seule configuration qui atteint chaque session en arrière-plan sur chaque machine.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Configurer un lanceur d'entreprise comme paramètre
</h3>

Certaines organisations exigent que chaque processus Claude Code démarre via un lanceur d'entreprise qui applique le sandboxing, les contrôles réseau ou l'injection de credentials. Le superviseur et ses workers démarrent Claude Code à partir d'un chemin fixe plutôt que de chercher `claude` sur `PATH`, donc chaque agent en arrière-plan contourne un wrapper que vous placez plus tôt sur `PATH`.

Définissez le paramètre [`processWrapper`](/docs/fr/settings-reference#processwrapper) pour préfixer le superviseur, ses workers et les autres processus en arrière-plan listés sous [Ce que le lanceur couvre](/docs/fr/corporate-launcher#what-the-launcher-covers) avec votre lanceur. La variable d'environnement [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/fr/env-vars) équivalente prend la priorité quand les deux sont définis, et elle est soumise à la même règle : livrez-la via les paramètres gérés ou `~/.claude/settings.json`, pas une export shell. [Exécuter Claude Code derrière un lanceur d'entreprise](/docs/fr/corporate-launcher) couvre le contrat que le lanceur doit satisfaire, ce qu'il fait et ne fait pas, et comment le déployer.

<Note>
  Un superviseur déjà en cours d'exécution conserve la configuration de lancement avec laquelle il a démarré. Après le déploiement du paramètre lanceur, exécutez [`claude daemon stop --any`](/docs/fr/agent-view#the-supervisor-process) pour que le prochain `claude agents` ou `--bg` démarre un superviseur qui le respecte. Un service installé prend `claude daemon stop` sans `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Chiens de garde d'inactivité du streaming
</h2>

Claude Code exécute quatre minuteurs indépendants qui interrompent une réponse de modèle en streaming lorsqu'elle devient silencieuse, de sorte qu'une connexion morte échoue et réessaie au lieu de rester bloquée. La limite du premier octet couvre l'attente des en-têtes de réponse, avant l'arrivée d'une partie de la réponse. Chacun des trois autres surveille une réponse active pour un signal différent.

| Minuteur                                | Interrompt quand                                                                                                                                                                                                                                                                     | S'exécute sur                                                                                                                                                                                                                                                                                                                                                                    | Délai d'expiration par défaut                                                                                   |
| :-------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| Limite du premier octet                 | Aucun en-tête de réponse n'arrive après que Claude Code envoie la demande                                                                                                                                                                                                            | API Anthropic directe et [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), y compris via un proxy HTTPS, mais pas quand `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL` les acheminent via une [passerelle](/docs/fr/gateways). Opt-in sur Amazon Bedrock avec `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1` ; ne s'exécute pas sur Google Cloud's Agent Platform ou Microsoft Foundry | 180 secondes sur l'API Anthropic directe, 300 secondes ailleurs, plus une seconde par 32 Ko de corps de demande |
| Chien de garde au niveau des événements | Aucun événement de réponse ne s'analyse. Sur les connexions où le chien de garde au niveau des octets s'exécute, les octets arrivants, y compris les pings de maintien de connexion, réinitialisent également ce chien de garde, pendant environ cinq minutes sans événement analysé | Tous les fournisseurs                                                                                                                                                                                                                                                                                                                                                            | 300 secondes                                                                                                    |
| Chien de garde au niveau des octets     | Aucun octet n'arrive sur le fil, y compris les pings de maintien de connexion SSE                                                                                                                                                                                                    | API Anthropic directe, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), et [passerelle](/docs/fr/gateways) connexions, y compris un `ANTHROPIC_BASE_URL` personnalisé. Opt-in sur les réponses Amazon Bedrock `vnd.amazon.eventstream` avec `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1` ; ne s'exécute pas sur Google Cloud's Agent Platform ou Microsoft Foundry                    | 180 secondes sur l'API Anthropic directe, 300 secondes ailleurs                                                 |
| Délai d'inactivité du corps             | Aucun octet n'arrive pendant 5 minutes                                                                                                                                                                                                                                               | Fournisseurs autres que l'API Anthropic directe et Claude Platform on AWS, sauf si [`API_FORCE_IDLE_TIMEOUT`](/docs/fr/env-vars) change cela                                                                                                                                                                                                                                          | 5 minutes                                                                                                       |

Configurez les minuteurs avec ces variables, chacune détaillée dans la [référence des variables d'environnement](/docs/fr/env-vars) :

* `CLAUDE_ENABLE_STREAM_WATCHDOG` et `CLAUDE_ENABLE_BYTE_WATCHDOG` forcent le chien de garde correspondant activé avec `1` ou désactivé avec `0`, dans les connexions que le tableau énumère ; aucune variable n'étend un chien de garde à un type de connexion qu'il ne couvre pas. `CLAUDE_ENABLE_BYTE_WATCHDOG` défini à `0` désactive également la limite du premier octet.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` définit le délai d'expiration des deux chiens de garde. Claude Code augmente les valeurs inférieures à 5 minutes à 5 minutes, et plafonne la valeur à 30 minutes pour le chien de garde au niveau des octets.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` définit le délai d'expiration du chien de garde au niveau des octets sans modifier celui du chien de garde au niveau des événements, limité entre 10 secondes et 30 minutes, et prend précédence sur `CLAUDE_STREAM_IDLE_TIMEOUT_MS` pour ce chien de garde.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` définit la limite du premier octet directement. Laissez-la non définie et Claude Code utilise le délai d'expiration du chien de garde au niveau des octets, de sorte que `CLAUDE_STREAM_IDLE_TIMEOUT_MS` et `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` modifient également la limite. Pour les limites, l'allocation de téléchargement, le plafond `API_TIMEOUT_MS`, et la durée d'attente de la nouvelle tentative après un abandon sans réponse, voir [Aucune réponse de l'API](/docs/fr/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` défini à `0` désactive le délai d'inactivité du corps, et défini à `1` l'active pour tous les fournisseurs. Les chiens de garde s'exécutent indépendamment de celui-ci, donc pour permettre à un flux de s'interrompre plus longtemps que leurs seuils, augmentez-les ou désactivez-les également.

Quand un chien de garde interrompt un flux bloqué, Claude Code traite l'interruption comme une défaillance en milieu de flux, et ce que vous voyez dépend de la distance parcourue par la réponse. Claude Code réessaie la demande ou termine le tour avec une erreur, conserve la sortie complétée et affiche un [avis de réponse incomplète](/docs/fr/errors#the-response-above-may-be-incomplete), ou termine le tour normalement. [Les nouvelles tentatives automatiques](/docs/fr/errors#automatic-retries) indiquent où chaque résultat s'applique.

Dans une [session non interactive](/docs/fr/headless), et pour la réponse d'un sous-agent dans n'importe quelle session, Claude Code peut d'abord inviter Claude à continuer la réponse coupée ; [l'entrée de cet avis](/docs/fr/errors#the-response-above-may-be-incomplete) indique quand il le fait et quand vous voyez toujours l'avis.

Quand la limite du premier octet se déclenche, aucune réponse n'a commencé, il n'y a donc pas de sortie partielle à conserver. Pour savoir comment Claude Code renvoie la demande et quand le tour se termine à la place, voir [Aucune réponse de l'API](/docs/fr/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Exigences d'accès réseau
</h2>

Claude Code nécessite un accès aux URL suivantes. Ajoutez ces URL à la liste blanche de votre configuration proxy et de vos règles de pare-feu, en particulier dans les environnements réseau conteneurisés ou restreints. La vérification de connectivité de la configuration de première exécution pointe vers ces URL lorsqu'elle ne peut pas atteindre `api.anthropic.com` ou `platform.claude.com` ; consultez [Impossible de se connecter aux services Anthropic](/docs/fr/errors#unable-to-connect-to-anthropic-services) pour les messages de vérification et les étapes de récupération.

| URL                                  | Requis pour                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Requêtes API Claude, y compris la vérification de sécurité du domaine WebFetch [domain safety check](/docs/fr/data-usage#webfetch-domain-safety-check), les récupérations de drapeaux de fonctionnalités et la journalisation des événements de télémétrie                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude.ai`                          | Authentification du compte claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `claude.com`                         | La connexion au compte claude.ai ouvre une page `claude.com` dans le navigateur, qui redirige vers `claude.ai` ; les recherches de documentation WebFetch pré-approuvées atteignent également cet hôte depuis l'interface de ligne de commande                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `platform.claude.com`                | Authentification du compte Anthropic Console. L'échange, l'actualisation et la révocation des jetons OAuth vont également à cet hôte pour les comptes claude.ai, donc les connexions à la fois à Console et à claude.ai le nécessitent                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcp-proxy.anthropic.com`            | [Connecteurs MCP depuis claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai), y compris les connecteurs qu'un administrateur d'organisation configure. Le trafic des connecteurs est acheminé via ce proxy ; les connecteurs sont activés par défaut pour les utilisateurs authentifiés par claude.ai. Pour empêcher Claude Code de les récupérer, définissez [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/fr/env-vars) ou le paramètre [`disableClaudeAiConnectors`](/docs/fr/settings-reference#disableclaudeaiconnectors)                                                                                                                                                                                      |
| `downloads.claude.ai`                | Téléchargements d'exécutables de plugins ; programme d'installation natif, mise à jour automatique native et vérifications de version de mise à jour                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `storage.googleapis.com`             | Comptages d'installation de plugins et métadonnées affichées dans `/plugin`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `storage.googleapis.com`             | Programme d'installation natif et mise à jour automatique native sur les versions antérieures à 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `registry.npmjs.org`                 | Installations de plugins (récupération de packages de plugins source npm et installation des dépendances de packages Node.js des plugins), serveurs MCP lancés avec `npx` et le registre de packages pour les installations npm et bun de Claude Code lui-même                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `bridge.claudeusercontent.com`       | Extension [Claude in Chrome](/docs/fr/chrome) WebSocket bridge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `*.frame.claudeusercontent.com`      | Lectures de contenu [Artifact](/docs/fr/artifacts). L'interface de ligne de commande récupère les fichiers d'un artifact depuis cet hôte lorsque Claude en ouvre un, et uniquement lorsque l'outil Artifact est [disponible](/docs/fr/artifacts#availability) pour votre compte. Pour désactiver l'outil et supprimer cette exigence, définissez [`"enableArtifact": false`](/docs/fr/settings-reference#enableartifact) ou [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/fr/env-vars) ; Claude Code honore également le paramètre [`disableArtifact`](/docs/fr/settings-reference#disableartifact) déprécié. Consultez [Désactiver les artifacts](/docs/fr/artifacts#disable-artifacts) pour voir comment ces paramètres interagissent |
| `github.com`                         | Clonage des [marketplaces de plugins](/docs/fr/plugins/overview) et des plugins hébergés sur GitHub, y compris la marketplace officielle Anthropic, via HTTPS ou SSH. Pour cloner les sources GitHub `owner/repo` via HTTPS uniquement, définissez [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/fr/env-vars)                                                                                                                                                                                                                                                                                                                                                                                                    |
| `raw.githubusercontent.com`          | Flux de changelog pour [`/release-notes`](/docs/fr/commands). Dans les sessions interactives, Claude Code le récupère également en arrière-plan au démarrage lorsque son changelog en cache ne couvre pas encore la version en cours d'exécution, par exemple au premier démarrage après une mise à jour ; les sessions non interactives et cloud ne le récupèrent jamais                                                                                                                                                                                                                                                                                                                                 |
| `*-review.googlesource.com`          | Recherche de changement Gerrit sur les checkouts `googlesource.com`. Lorsqu'une session d'onglet Claude Desktop Code démarre ou reprend sur un checkout [approuvé](/docs/fr/permissions#project-allow-rules-and-workspace-trust) dont l'`origin` est un hôte `googlesource.com`, Claude Code demande anonymement au serveur `-review` de cet hôte le changement ouvert correspondant au `Change-Id` de HEAD, une fois par démarrage ou reprise. Les autres types de sessions ignorent la recherche, et aucun autre hôte Gerrit n'est contacté. Facultatif : désactiver avec [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars)                                                                    |
| `http-intake.logs.us5.datadoghq.com` | Événements de télémétrie opérationnelle, envoyés uniquement lorsque l'interface de ligne de commande utilise directement l'API Anthropic, jamais pour Amazon Bedrock, la plateforme d'agent de Google Cloud ou Microsoft Foundry. Facultatif : désactiver avec [`DISABLE_TELEMETRY`](/docs/fr/data-usage#telemetry-services) ou `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                                                            |
| `browser-intake-us5-datadoghq.com`   | Rapports d'erreurs opérationnels, envoyés lorsque l'interface de ligne de commande utilise directement l'API Anthropic et qu'une porte de déploiement côté serveur les active. Facultatif : désactiver avec `DISABLE_ERROR_REPORTING` ou `DISABLE_TELEMETRY` ; consultez [Services de télémétrie](/docs/fr/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                                                                 |
| `formulae.brew.sh`                   | Vérifications de version de mise à jour sur les installations Homebrew. Les autres méthodes d'installation ne contactent pas cet hôte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `code.claude.com`                    | Recherches de documentation Claude Code par l'agent claude-code-guide intégré et les requêtes WebFetch pré-approuvées. Le blocage de cet hôte affecte uniquement les recherches de documentation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

Si vous installez Claude Code via npm ou gérez votre propre distribution binaire, les utilisateurs finaux n'ont pas besoin des utilisations du programme d'installation natif et de la mise à jour automatique de `downloads.claude.ai`, mais les installations npm et bun ont besoin de leur registre de packages, `registry.npmjs.org`, sauf si votre organisation le met en miroir. Les autres utilisations du tableau s'appliquent indépendamment de la méthode d'installation.

Les deux hôtes d'entrée Datadog ne transportent que la télémétrie opérationnelle facultative, et la définition de [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars) désactive les deux. Les sessions sur les fournisseurs tiers n'envoient jamais à ces hôtes, même lorsqu'une plateforme définit [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars) et que les métriques de télémétrie sont activées par défaut. Consultez [Services de télémétrie](/docs/fr/data-usage#telemetry-services) pour tout ce que Claude Code envoie et comment le désactiver avant de finaliser votre liste blanche.

Lors de l'utilisation d'[Amazon Bedrock](/docs/fr/amazon-bedrock), de la [plateforme d'agent de Google Cloud](/docs/fr/google-vertex-ai), de [Microsoft Foundry](/docs/fr/microsoft-foundry) ou d'une session de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectée, le trafic du modèle et l'authentification vont à votre fournisseur ou passerelle au lieu de `api.anthropic.com`, `claude.ai` ou `platform.claude.com`. L'outil WebFetch appelle toujours `api.anthropic.com` pour sa [vérification de sécurité du domaine](/docs/fr/data-usage#webfetch-domain-safety-check) sauf si vous définissez `skipWebFetchPreflight: true` dans [paramètres](/docs/fr/settings).

Lors du routage via une [passerelle LLM](/docs/fr/llm-gateway) avec [`ANTHROPIC_BASE_URL`](/docs/fr/llm-gateway-connect#set-the-base-url-and-credential), la vérification de disponibilité du [mode rapide](/docs/fr/fast-mode) appelle toujours `api.anthropic.com` plutôt que l'URL de base de la passerelle. La vérification honore un proxy HTTP configuré, donc lorsqu'un bloc réseau est la cause, une entrée de liste blanche pour `api.anthropic.com` dans le proxy est la solution. Un bloc réseau échoue la vérification uniquement lorsque l'hôte est inaccessible même via le proxy, et le mode rapide signale alors une erreur de connectivité. La même erreur de connectivité apparaît lorsque la vérification présente une credential émise par la passerelle qu'Anthropic rejette ; la mise en liste blanche n'aide pas là, puisque rien n'est bloqué. Consultez [utiliser le mode rapide derrière les proxies et les passerelles LLM](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) pour les variables qui le restaurent.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Listes blanches IP d'organisation et sortie proxy
</h3>

Si votre organisation a [la mise en liste blanche IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) activée pour Claude, acheminez `bridge.claudeusercontent.com` via la même sortie proxy que `claude.ai` et `api.anthropic.com`, par exemple en le plaçant dans le même segment d'application Zscaler ou la même politique de direction Netskope. Si vous ne pouvez pas l'acheminer de cette façon, ajoutez l'adresse de sortie que votre proxy utilise pour cet hôte à la liste blanche IP de votre organisation, mais uniquement lorsque cette adresse est dédiée à votre organisation : une plage de sortie proxy partagée admet également les autres clients du fournisseur de proxy.

Anthropic vérifie les connexions à `bridge.claudeusercontent.com` par rapport à la liste blanche IP de votre organisation en utilisant l'adresse à partir de laquelle elles arrivent. Si votre proxy envoie le trafic pour cet hôte via une adresse qui n'est pas sur cette liste blanche, Claude Code ne peut pas se connecter à l'extension [Claude in Chrome](/docs/fr/chrome) même si le reste de Claude Code fonctionne.

<h3 id="github-allow-lists-and-firewalls">
  Listes blanches GitHub et pare-feu
</h3>

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) dans les environnements hébergés par Anthropic et [Code Review](/docs/fr/code-review) se connectent à vos référentiels depuis l'infrastructure gérée par Anthropic ; les sessions dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments) se connectent depuis l'intérieur de votre réseau, sauf si le runner opte pour la [passerelle git Anthropic](/docs/fr/self-hosted-environments-deploy#use-the-anthropic-git-proxy), qui récupère depuis le côté Anthropic.

Si votre organisation GitHub Enterprise Cloud restreint l'accès par adresse IP, activez [l'héritage de la liste blanche IP pour les applications GitHub installées](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) et [ajoutez également une entrée de liste blanche](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) pour les [adresses IP sortantes](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) d'Anthropic. L'héritage couvre uniquement les requêtes que l'application GitHub Claude effectue en tant qu'installation, pas les requêtes qu'elle effectue au nom de vos utilisateurs. Pour les autres pare-feu, consultez les [adresses IP de l'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Pour les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) auto-hébergées derrière un pare-feu, mettez en liste blanche les [adresses IP sortantes](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) d'Anthropic afin que l'infrastructure Anthropic puisse atteindre votre hôte GHES pour cloner les référentiels et publier les commentaires d'examen. Les sessions dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments-deploy#configure-git) atteignent votre hôte GHES depuis l'intérieur de votre réseau à la place, donc cette exposition s'applique uniquement aux sessions hébergées par Anthropic, aux flux de pré-session hébergés tels que le sélecteur de référentiel, et aux runners auto-hébergés qui optent pour la [passerelle git Anthropic](/docs/fr/self-hosted-environments-deploy#use-the-anthropic-git-proxy), qui récupère depuis le côté Anthropic. Pour un hôte GHES qui n'est routable que dans votre réseau, le [connecteur SCM](/docs/fr/self-hosted-environments-reference#scm-connector-flags) porte les flux de pré-session hébergés sur une connexion sortante à la place, donc la liste blanche n'est pas nécessaire pour eux.

<h3 id="desktop-and-claude-ai">
  Bureau et claude.ai
</h3>

Le tableau précédent couvre l'interface de ligne de commande autonome. L'application Claude Desktop et claude.ai dans un navigateur chargent leur code d'application et le contenu utilisateur à partir d'hôtes CDN Anthropic supplémentaires, y compris `assets-proxy.anthropic.com` et les autres origines `*.claudeusercontent.com` qui servent les [artifacts](/docs/fr/artifacts) dans ces applications. Autoriser `claude.ai` tout en bloquant ces hôtes produit une page vierge plutôt qu'une erreur. Consultez [exigences d'accès réseau](/docs/fr/desktop#network-access-requirements) sur la page Bureau.

Un [artifact](/docs/fr/artifacts) qui charge une police de caractères à partir de [Google Fonts](/docs/fr/artifacts#improve-the-visual-design) demande également `fonts.googleapis.com` et `fonts.gstatic.com`. Les deux hôtes sont facultatifs. Si vous les bloquez, les artifacts s'affichent dans les polices de secours. Bloquez avec un rejet rapide plutôt qu'une suppression silencieuse afin que la requête de police échoue immédiatement au lieu de retarder le premier rendu de la page.

Les artifacts peuvent également charger des bibliothèques JavaScript, telles que React ou un package de graphique, à partir de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` et `unpkg.com`, et d'aucun autre hôte externe. Si vous bloquez ces hôtes, les parties d'un artifact qui dépendent d'une bibliothèque ne fonctionnent pas, et contrairement à une police bloquée, une bibliothèque bloquée n'a pas de secours. Bloquez avec un rejet rapide ici aussi, afin qu'une requête de bibliothèque bloquée échoue immédiatement plutôt que de rester suspendue jusqu'à l'expiration du délai d'attente.

<h2 id="additional-resources">
  Ressources supplémentaires
</h2>

* [Fichiers de configuration et ordre de priorité](/docs/fr/settings)
* [Référence des variables d'environnement](/docs/fr/env-vars)
* [Guide de dépannage](/docs/fr/troubleshooting)
