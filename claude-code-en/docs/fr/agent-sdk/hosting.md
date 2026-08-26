> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Héberger l'Agent SDK

> Déployer l'Agent SDK en production : architecture de sous-processus, persistance des sessions, mise à l'échelle, observabilité et isolation multi-locataire pour Docker, Kubernetes et fournisseurs de sandbox.

L'Agent SDK crée et supervise un sous-processus CLI `claude` qui possède un shell, un répertoire de travail et des fichiers de session sur le disque. L'héberger n'est pas comme héberger un wrapper API sans état. Chaque agent en cours d'exécution est un processus de longue durée lié à l'état local, ce qui façonne la façon dont vous allouez les ressources, persistez les sessions et mettez à l'échelle entre les locataires.

Cette page couvre l'auto-hébergement sur votre propre infrastructure : comprenez [le modèle de sous-processus](#the-subprocess-model), [choisissez un modèle de session](#choose-a-session-pattern), [provisionnez le conteneur](#provision-the-container) et [gérez les préoccupations de production](#handle-production-concerns) comme la persistance, l'observabilité, l'authentification et l'isolation multi-locataire. Pour les Dockerfiles déployables et les manifestes Kubernetes, consultez le [guide d'hébergement](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Si vous n'avez pas besoin de contrôle d'infrastructure, d'isolation personnalisée ou de votre propre plan de données, envisagez plutôt les [Agents gérés](https://platform.claude.com/docs/fr/managed-agents/overview) : une API REST hébergée où Anthropic exécute l'agent et le sandbox, de sorte que votre application envoie des événements et reçoit les résultats en streaming sans infrastructure d'hébergement à exploiter.

<Info>
  Pour le renforcement de la sécurité au-delà du sandboxing basique, y compris les contrôles réseau, la gestion des identifiants et les options d'isolation, consultez [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  Le modèle de sous-processus
</h2>

Chaque décision d'hébergement sur cette page découle de la façon dont le SDK exécute l'agent. Lorsque votre code appelle `query()`, le SDK lance un processus CLI `claude` séparé et communique avec lui via stdio. Ce sous-processus possède le shell, le répertoire de travail et les transcriptions de session JSONL sur le disque local.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Flux de requête : client vers votre application, qui lance un sous-processus CLI claude via stdio à l'intérieur du conteneur ; le sous-processus écrit sur le disque local et appelle api.anthropic.com via HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Une session d'agent correspond à un sous-processus. L'exécution de N sessions concurrentes signifie N sous-processus, chacun avec son propre arborescence de processus et son fichier de transcription. Par défaut, ils héritent tous du répertoire de travail de votre application, donc passez `cwd` sur chaque appel `query()` lorsque les sessions ont besoin de systèmes de fichiers séparés :

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  État qui réside sur le disque local
</h3>

Trois types d'état d'agent résident sur le système de fichiers du conteneur par défaut. Aucun d'entre eux ne survit à un redémarrage de conteneur, une réduction d'échelle ou un déplacement vers un nœud différent.

| État                               | Emplacement par défaut                                                                                           |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Transcriptions de session          | `~/.claude/projects/`, ou le répertoire `projects/` sous `CLAUDE_CONFIG_DIR` s'il est défini                     |
| Fichiers mémoire `CLAUDE.md`       | `~/.claude/CLAUDE.md` pour le niveau utilisateur et le répertoire de travail de la session pour le niveau projet |
| Artefacts du répertoire de travail | Le répertoire de travail de la session                                                                           |

Pour persister les transcriptions entre les hôtes, configurez un adaptateur [`SessionStore`](/docs/fr/agent-sdk/session-storage). Les fichiers mémoire et autres artefacts du répertoire de travail ont besoin de leur propre stratégie de stockage, comme un volume monté ou une synchronisation de magasin d'objets.

Pour savoir comment les sessions, la reprise et la bifurcation fonctionnent au niveau de l'API, consultez [Sessions](/docs/fr/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Choisir un modèle de session
</h2>

Ces quatre modèles couvrent le cycle de vie de la session : la durée de vie d'un conteneur par rapport aux sessions qu'il dessert. Pour savoir où le conteneur s'exécute, le [guide d'hébergement](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) contient du [code déployable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) pour Docker local, Modal et Kubernetes. Choisissez un modèle de session ici et une cible de déploiement à partir du guide.

<h3 id="ephemeral-sessions">
  Sessions éphémères
</h3>

Créez un conteneur pour chaque tâche utilisateur et détruisez-le lorsque la tâche est terminée. Idéal pour les tâches ponctuelles. L'utilisateur peut toujours interagir avec l'IA pendant que la tâche s'exécute, mais une fois terminée, le conteneur est détruit.

Les charges de travail d'exemple incluent l'investigation et la correction de bogues, l'extraction de factures et de reçus, la traduction de documents et la transformation de médias.

Le conteneur exécute un point d'entrée unique qui appelle le SDK et se termine. L'exemple ci-dessous montre une version TypeScript minimale. Enregistrez-la sous `entrypoint.mts` ou définissez `"type": "module"` dans `package.json` pour que `await` au niveau supérieur soit disponible.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Sessions de longue durée
</h3>

Exécutez des instances de conteneur persistantes, souvent hébergeant plusieurs processus SDK par conteneur, pour servir le travail continu. Idéal pour les agents qui prennent des mesures autonomes, servent du contenu ou gèrent des flux de messages à haut volume.

Les charges de travail d'exemple incluent un agent de messagerie qui trie et répond aux messages entrants, un générateur de site qui héberge un site modifiable par utilisateur via les ports du conteneur, et un chatbot qui gère le trafic continu d'une plateforme comme Slack.

Le conteneur expose un point de terminaison HTTP ou WebSocket et mappe chaque session active à une requête de longue durée et au sous-processus derrière elle. En TypeScript, utilisez [`streamInput()`](/docs/fr/agent-sdk/typescript#query-object) pour ajouter des tours à une session active et [`startup()`](/docs/fr/agent-sdk/typescript#startup) pour préchauffer les sous-processus avant le trafic entrant. En Python, utilisez [`ClaudeSDKClient`](/docs/fr/agent-sdk/python#claudesdkclient) pour maintenir une session ouverte entre les tours. Dimensionnez le conteneur pour qu'il puisse contenir le nombre maximum de sessions simultanées en mémoire.

<h3 id="hybrid-sessions">
  Sessions hybrides
</h3>

Conteneurs éphémères qui s'hydratent à partir d'un [`SessionStore`](/docs/fr/agent-sdk/session-storage) au démarrage et persistent les mises à jour en retour. Idéal pour les sessions qui s'étendent sur de nombreuses interactions mais restent inactives entre elles. Le conteneur s'arrête pendant les périodes d'inactivité et redémarre lorsque l'utilisateur revient.

Les charges de travail d'exemple incluent un gestionnaire de projet personnel avec des vérifications intermittentes, une recherche approfondie qui s'interrompt et reprend sur des heures, et un agent d'assistance client qui charge l'historique des tickets entre les interactions.

Ajustez le délai d'inactivité de votre fournisseur à la fréquence à laquelle vous vous attendez à ce que les utilisateurs reviennent. L'arrêt d'un conteneur sans `SessionStore` configuré perd la transcription avec lui, donc le magasin est requis pour ce modèle, pas optionnel.

Le modèle repose sur la reprise d'une session par ID avec un magasin partagé attaché :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

Consultez [Stockage de session](/docs/fr/agent-sdk/session-storage) pour l'interface `SessionStore` complète et les adaptateurs de référence.

<h3 id="multi-agent-container">
  Conteneur multi-agent
</h3>

Exécutez plusieurs sous-processus SDK à l'intérieur d'un conteneur. Idéal pour les agents qui doivent collaborer étroitement, par exemple les simulations multi-agents où les agents interagissent les uns avec les autres dans un environnement partagé.

Donnez à chaque agent son propre répertoire de travail pour qu'ils ne se réécrivent pas les fichiers les uns des autres, et isolez le chargement des paramètres pour que les fichiers `CLAUDE.md` par agent ne fuient pas entre les agents. Consultez [Isolation multi-locataire](#multi-tenant-isolation) pour les options spécifiques.

<h2 id="provision-the-container">
  Provisionner le conteneur
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing basé sur conteneur
</h3>

Exécutez le SDK à l'intérieur d'un conteneur sécurisé pour l'isolation des processus, les limites de ressources, le contrôle du réseau et un système de fichiers éphémère. Plusieurs fournisseurs se spécialisent dans les environnements de conteneurs sécurisés qui correspondent au modèle du SDK Agent.

Questions à répondre lors du choix d'un fournisseur :

* **Qui exécute le sandbox** : un fournisseur sandbox-as-a-service exploite l'infrastructure pour vous, tandis que les options auto-hébergées vous donnent un logiciel à exécuter sur votre propre infrastructure.
* **Latence de démarrage à froid** : combien de temps entre « créer un sandbox » et « prêt à accepter la première requête ». Les modèles éphémères nécessitent des démarrages infra-seconde. Les modèles de longue durée tolèrent davantage.
* **Stockage persistant** : si le fournisseur offre des volumes durables ou seulement un disque éphémère. Le modèle hybride a besoin de stockage durable quelque part, que ce soit dans le sandbox ou à côté.
* **Modèle de tarification** : facturation à la seconde, à la requête ou forfaitaire horaire. La tarification à la seconde convient aux charges de travail éphémères par rafales. La tarification horaire convient aux sessions de longue durée.
* **Réseau** : support des règles de sortie personnalisées, des proxies sortants et de l'appairage VPC privé pour les environnements réglementés.

Fournisseurs à évaluer :

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), avec une [implémentation de démonstration](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Pour les options auto-hébergées telles que Docker, gVisor et Firecracker, et la configuration détaillée de l'isolation, voir [Technologies d'isolation](/docs/fr/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dépendances d'exécution
</h3>

Le conteneur a besoin uniquement du runtime de langage de votre SDK :

* Python 3.10+ pour le SDK Python, ou Node.js 18+ pour le SDK TypeScript
* Les deux packages SDK incluent un binaire Claude Code natif pour la plateforme hôte, donc aucune installation séparée de Claude Code ou Node.js n'est nécessaire pour le CLI généré

Le binaire inclus est épinglé à la version du package SDK, donc la mise à jour du SDK est la façon de mettre à jour le CLI. Le SDK suit semver : prenez les versions de correctif en continu et consultez le changelog [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) ou [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) avant de prendre une version mineure.

<h3 id="resources">
  Ressources
</h3>

1 GiB de RAM, 5 GiB de disque et 1 CPU par agent est un point de départ raisonnable pour une instance fraîchement démarrée. L'utilisation de la mémoire augmente avec la durée de la session et l'activité des outils, donc dimensionnez pour les durées de session et la concurrence que vous avez réellement besoin plutôt que la ligne de base inactive. Voir [Mise à l'échelle et concurrence](#scaling-and-concurrency) pour savoir comment calculer les agents par hôte.

<h3 id="network">
  Réseau
</h3>

Le SDK a besoin d'un accès HTTPS sortant à `api.anthropic.com`, ou au point de terminaison régional de votre fournisseur lors de l'exécution sur Amazon Bedrock ou Google Cloud's Agent Platform. Si vos agents utilisent des [serveurs MCP](/docs/fr/agent-sdk/mcp) ou des outils externes, ils ont besoin d'un accès sortant à ces points de terminaison également. Pour la production, acheminez le trafic sortant via un proxy de sortie qui applique les listes blanches de domaines, injecte les identifiants et enregistre les requêtes. Voir [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) pour le modèle complet.

Pour le trafic entrant, exposez un port HTTP ou WebSocket sur le conteneur. Votre application gère les requêtes des clients sur ce port et appelle le SDK en interne ; le sous-processus lui-même n'écoute pas sur le réseau.

<h2 id="handle-production-concerns">
  Gérer les préoccupations de production
</h2>

Travaillez à travers ces décisions avant de déployer un agent auto-hébergé.

<h3 id="session-and-state-persistence">
  Persistance des sessions et de l'état
</h3>

Le disque local par défaut est perdu au redémarrage, à la réduction d'échelle ou à un déplacement vers un nœud différent. Pour toute session qu'un utilisateur s'attend à reprendre, mettez en miroir la transcription vers un stockage durable avec un adaptateur [`SessionStore`](/docs/fr/agent-sdk/session-storage). Consultez [Implémentations de référence](/docs/fr/agent-sdk/session-storage#reference-implementations) pour les adaptateurs S3, Redis et Postgres et une suite de conformité pour la vôtre.

Trois choses à savoir sur le comportement de `SessionStore` :

* **Transcriptions uniquement** : `SessionStore` met en miroir les transcriptions, pas les fichiers mémoire `CLAUDE.md` ou autres artefacts du répertoire de travail. Montez un volume partagé ou synchronisez-les séparément.
* **Miroir, pas remplacement** : le sous-processus écrit d'abord sur le disque local, et le magasin reçoit une copie de chaque lot. Les écritures locales restent faisant autorité.
* **Messages `mirror_error`** : un lot que le magasin rejette est envoyé jusqu'à trois fois au total, avec un court délai d'attente avant chaque nouvelle tentative ; un appel qui expire n'est pas réessayé. Si le lot échoue toujours, le SDK le supprime, émet un message `{ type: "system", subtype: "mirror_error" }`, et continue la requête. Alertez sur ceux-ci si la durabilité du magasin est importante.

<h3 id="observability">
  Observabilité
</h3>

Les agents du SDK Agent sont des processus de longue durée qui génèrent des appels d'outils sur de nombreux allers-retours API. Sans télémétrie, vous ne pouvez pas voir quels outils ont été exécutés, combien de temps ils ont pris ou où une session s'est bloquée.

Le SDK hérite la configuration OpenTelemetry de l'environnement. Définissez les variables d'environnement OTEL au niveau du conteneur ou de l'orchestrateur afin que chaque appel `query()` exporte des spans, des métriques et des événements de journal vers votre collecteur. L'exemple ci-dessous active l'export OTLP pour les trois signaux. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` est requis uniquement pour les traces ; omettez-le si vous exportez uniquement les métriques et les journaux.

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Le texte d'invite et les entrées d'outils ne sont pas inclus dans les exports par défaut. Consultez [Contrôler les données sensibles dans les exports](/docs/fr/agent-sdk/observability#control-sensitive-data-in-exports) pour les indicateurs d'acceptation et [Observabilité](/docs/fr/agent-sdk/observability) pour le catalogue complet des signaux.

<h3 id="auth-and-secrets">
  Authentification et secrets
</h3>

Trois préoccupations d'authentification sont importantes au moment de l'hébergement :

* **API Anthropic** : le sous-processus lit `ANTHROPIC_API_KEY` de son environnement. Fournissez-le à partir de votre gestionnaire de secrets, ou définissez `ANTHROPIC_BASE_URL` pour acheminer les appels de modèle via un proxy qui injecte la clé en dehors du conteneur. Consultez [Gestion des identifiants](/docs/fr/agent-sdk/secure-deployment#credential-management) pour le modèle de proxy et [Aperçu du SDK](/docs/fr/agent-sdk/overview#get-started) pour les méthodes d'authentification prises en charge.
* **Entrant** : mettez l'authentification à une passerelle devant le conteneur de l'agent. L'agent doit recevoir des requêtes pré-authentifiées et ne doit pas être le composant qui valide les jetons utilisateur.
* **Outils sortants** : gardez les identifiants d'outils en dehors de l'environnement de l'agent. Acheminez les appels sortants via un proxy qui injecte les clés API après que la requête quitte le conteneur. L'agent effectue l'appel ; le proxy ajoute l'identifiant.

<h3 id="scaling-and-concurrency">
  Mise à l'échelle et concurrence
</h3>

Chaque session s'exécute dans son propre sous-processus, donc la concurrence sur un hôte est limitée par le nombre de sous-processus que sa RAM peut contenir.

Dimensionnez chaque hôte avec cette formule :

```text theme={null}
agents par hôte = (RAM hôte - surcharge) / (plafond RAM par session)
```

Mesurez le plafond par session en exécutant une session représentative jusqu'à votre longueur cible sous votre charge d'outils attendue et en enregistrant le pic RSS. Le point de départ de 1 GiB dans [Ressources](#resources) est un plancher, pas le plafond.

Le routage à l'échelle horizontale dépend de votre modèle. Pour les sessions de longue durée, où les conteneurs contiennent de nombreuses sessions, exécutez un pool de conteneurs derrière un équilibreur de charge et épinglez chaque session à un conteneur en utilisant un hachage cohérent sur `sessionId`. Une session épinglée continue à frapper le même conteneur, et donc le même sous-processus en cours d'exécution, jusqu'à ce qu'il soit évincé ou que le conteneur redémarre.

Les grands appels de fan-out concurrents de [sous-agents](/docs/fr/agent-sdk/subagents) à partir d'une seule session peuvent atteindre les limites de taux API. Divisez le travail en lots plus petits plutôt que d'émettre une seule distribution large.

<h3 id="cost">
  Coût
</h3>

Le coût des jetons Anthropic domine généralement le coût de l'infrastructure du conteneur d'un ordre de grandeur ou plus. Un conteneur minimalement provisionné coûte environ 0,05 \$ par heure, tandis qu'une seule session d'agent longue peut dépenser des dollars en jetons. Consultez [Suivi des coûts](/docs/fr/agent-sdk/cost-tracking) pour la comptabilité des jetons par session.

<h3 id="multi-tenant-isolation">
  Isolation multi-locataire
</h3>

Le comportement par défaut du SDK lit les paramètres et les fichiers mémoire `CLAUDE.md` à partir du système de fichiers. Dans un conteneur partagé qui sert plusieurs locataires, ces fichiers peuvent fuir le contexte d'un locataire dans la session d'un autre locataire.

Pour isoler les locataires à l'intérieur d'un conteneur partagé :

* Passez `settingSources: []` en TypeScript ou `setting_sources=[]` en Python afin qu'aucun paramètre du système de fichiers ne se charge.
* Définissez `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` dans `env`. [Auto memory](/docs/fr/memory#auto-memory) à `~/.claude/projects/<project>/memory/` se charge dans l'invite système indépendamment de `settingSources`. Consultez [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les autres entrées qui se chargent sans condition.
* Pointez `CLAUDE_CONFIG_DIR` vers un répertoire par locataire afin que les locataires ne partagent pas la configuration globale `~/.claude.json`.
* Utilisez un répertoire de travail par locataire. Passez `cwd` explicitement à chaque appel `query()`.
* Appliquez des règles de sortie par locataire à votre proxy, telles que des adresses IP sortantes distinctes, des identifiants ou des listes blanches de domaines, afin qu'un locataire compromis ne puisse pas exfiltrer les données via la politique sortante d'un autre locataire.

L'exemple ci-dessous applique les quatre options au niveau du SDK ensemble. Construisez `tenantDir` et `configDir` afin que chaque locataire obtienne un chemin qu'aucun autre locataire ne peut lire. En TypeScript, `env` remplace l'environnement du sous-processus, donc propagez `...process.env` pour conserver les variables héritées comme `PATH` et `ANTHROPIC_API_KEY`. En Python, `env` est fusionné au-dessus de l'environnement hérité.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=prompt,
      options=ClaudeAgentOptions(
          cwd=tenant_dir,
          setting_sources=[],
          env={
              "CLAUDE_CONFIG_DIR": config_dir,
              "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
          },
      ),
  ):
      ...
  ```
</CodeGroup>

Pour les contrôles réseau par locataire, consultez [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Limitations connues
</h2>

Planifiez autour de celles-ci dans votre conception de déploiement.

| Limitation                                                                               | Que faire                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pas de délai d'expiration de session au niveau supérieur                                 | Une session n'expire pas d'elle-même. Définissez `maxTurns` dans `Options` pour limiter le nombre de cycles d'utilisation d'outils que l'agent effectue avant de s'arrêter.                                                                                                                                                                                     |
| Croissance de la mémoire au cours de longues sessions                                    | Limitez la durée de la session ou recyclez les sous-processus périodiquement. Voir [Mise à l'échelle et concurrence](#scaling-and-concurrency).                                                                                                                                                                                                                 |
| Les grands déploiements parallèles de sous-agents peuvent atteindre les limites de débit | Divisez le travail en lots plus petits plutôt que d'émettre un seul envoi large.                                                                                                                                                                                                                                                                                |
| Pas de délai limite d'horloge murale par sous-agent                                      | Limitez chaque [sous-agent](/docs/fr/agent-sdk/subagents) avec `maxTurns` dans sa `AgentDefinition`. Pour les sous-agents d'arrière-plan uniquement, `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` définit un chien de garde de stagnation qui se déclenche lorsqu'un sous-agent `run_in_background` cesse de produire une sortie ; ce n'est pas un délai d'exécution total. |

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Carnet de recettes d'hébergement](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) : parcours du carnet avec [code déployable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) pour Docker, Modal et Kubernetes.
* [Stockage des sessions](/docs/fr/agent-sdk/session-storage) : persistez les transcriptions entre les hôtes avec un adaptateur `SessionStore`.
* [Observabilité](/docs/fr/agent-sdk/observability) : exportez les traces OTEL, les métriques et les journaux vers votre collecteur.
* [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) : contrôles réseau, gestion des identifiants et renforcement de l'isolation.
* [Suivi des coûts](/docs/fr/agent-sdk/cost-tracking) : comptabilité des jetons et des coûts par session.
