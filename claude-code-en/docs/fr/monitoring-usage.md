> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Surveillance

> Découvrez comment activer et configurer OpenTelemetry pour Claude Code.

Suivez l'utilisation de Claude Code, les coûts et l'activité des outils dans votre organisation en exportant les données de télémétrie via OpenTelemetry (OTel). Claude Code exporte les métriques sous forme de données de séries chronologiques via le protocole de métriques standard, les événements via le protocole de journaux/événements, et optionnellement les traces distribuées via le [protocole de traces](#traces-beta). Configurez vos backends de métriques, de journaux et de traces pour qu'ils correspondent à vos exigences de surveillance.

<h2 id="quick-start">
  Démarrage rapide
</h2>

Configurez OpenTelemetry à l'aide de variables d'environnement :

```bash theme={null}
# 1. Activer la télémétrie
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Choisir les exportateurs (les deux sont facultatifs - configurez uniquement ce dont vous avez besoin)
export OTEL_METRICS_EXPORTER=otlp       # Options : otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Options : otlp, console, none

# 3. Configurer le point de terminaison OTLP (pour l'exportateur OTLP)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Définir l'authentification (si nécessaire)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Pour le débogage : réduire les intervalles d'export
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 secondes (par défaut : 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 secondes (par défaut : 5000ms)

# 6. Exécuter Claude Code
claude
```

<Note>
  Les intervalles d'export par défaut sont de 60 secondes pour les métriques et de 5 secondes pour les journaux. Lors de la configuration, vous pouvez utiliser des intervalles plus courts à des fins de débogage. N'oubliez pas de les réinitialiser pour une utilisation en production.
</Note>

Pour les options de configuration complètes, consultez la [spécification OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuration de l'administrateur
</h2>

Les administrateurs peuvent configurer les paramètres OpenTelemetry pour tous les utilisateurs via le [fichier de paramètres gérés](/docs/fr/settings#settings-files). Cela permet un contrôle centralisé des paramètres de télémétrie dans toute une organisation. Consultez la [précédence des paramètres](/docs/fr/settings#settings-precedence) pour plus d'informations sur la façon dont les paramètres sont appliqués.

Exemple de configuration des paramètres gérés :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  Les paramètres gérés peuvent être distribués via MDM (Mobile Device Management) ou d'autres solutions de gestion d'appareils. Les variables d'environnement définies dans le fichier de paramètres gérés ont une haute priorité et ne peuvent pas être remplacées par les utilisateurs.
</Note>

Claude Code ne transmet pas les variables d'environnement `OTEL_*` aux sous-processus qu'il génère, y compris l'outil Bash, les hooks, les serveurs MCP et les serveurs de langage. Une application instrumentée par OpenTelemetry que vous exécutez via l'outil Bash n'hérite pas du point de terminaison de l'exportateur ou des en-têtes de Claude Code, donc définissez ces variables directement dans la commande si cette application doit exporter sa propre télémétrie.

<h2 id="configuration-details">
  Détails de la configuration
</h2>

<h3 id="common-configuration-variables">
  Variables de configuration courantes
</h3>

| Variable d'environnement                            | Description                                                                                                                                                                                                                                                                                                                                                                                                                  | Exemples de valeurs                                                                                                                               |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Active la collecte de télémétrie (obligatoire)                                                                                                                                                                                                                                                                                                                                                                               | `1`                                                                                                                                               |
| `OTEL_METRICS_EXPORTER`                             | Types d'exportateur de métriques, séparés par des virgules. Utilisez `none` pour désactiver                                                                                                                                                                                                                                                                                                                                  | `console`, `otlp`, `prometheus`, `none`                                                                                                           |
| `OTEL_LOGS_EXPORTER`                                | Types d'exportateur de journaux/événements, séparés par des virgules. Utilisez `none` pour désactiver                                                                                                                                                                                                                                                                                                                        | `console`, `otlp`, `none`                                                                                                                         |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocole pour l'exportateur OTLP, s'applique à tous les signaux                                                                                                                                                                                                                                                                                                                                                             | `grpc`, `http/json`, `http/protobuf`                                                                                                              |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Point de terminaison du collecteur OTLP pour tous les signaux                                                                                                                                                                                                                                                                                                                                                                | `http://localhost:4317`                                                                                                                           |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocole pour les métriques, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                  | `grpc`, `http/json`, `http/protobuf`                                                                                                              |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Point de terminaison des métriques OTLP, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                       | `http://localhost:4318/v1/metrics`                                                                                                                |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocole pour les journaux, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                   | `grpc`, `http/json`, `http/protobuf`                                                                                                              |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Point de terminaison des journaux OTLP, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                        | `http://localhost:4318/v1/logs`                                                                                                                   |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | En-têtes d'authentification pour OTLP                                                                                                                                                                                                                                                                                                                                                                                        | `Authorization=Bearer token`                                                                                                                      |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalle d'export en millisecondes (par défaut : 60000)                                                                                                                                                                                                                                                                                                                                                                    | `5000`, `60000`                                                                                                                                   |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalle d'export des journaux en millisecondes (par défaut : 5000)                                                                                                                                                                                                                                                                                                                                                        | `1000`, `10000`                                                                                                                                   |
| `OTEL_LOG_USER_PROMPTS`                             | Activer la journalisation du contenu des invites utilisateur (par défaut : désactivé)                                                                                                                                                                                                                                                                                                                                        | `1` pour activer                                                                                                                                  |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Activer la journalisation du texte de réponse de l'assistant sur les événements `assistant_response` (par défaut : désactivé). Lorsque non défini, revient à la valeur de `OTEL_LOG_USER_PROMPTS`. Nécessite Claude Code v2.1.193 ou version ultérieure                                                                                                                                                                      | `1` pour activer, `0` pour garder masqué                                                                                                          |
| `OTEL_LOG_TOOL_DETAILS`                             | Activer la journalisation des paramètres d'outil et des arguments d'entrée dans les événements d'outil et les attributs d'intervalle de trace : commandes Bash, noms de serveur MCP et d'outil, noms de compétences et entrée d'outil. Active également les noms de commandes personnalisées, de plugin et MCP sur les événements `user_prompt` (par défaut : désactivé)                                                     | `1` pour activer                                                                                                                                  |
| `OTEL_LOG_TOOL_CONTENT`                             | Activer la journalisation du contenu d'entrée et de sortie d'outil dans les événements d'intervalle (par défaut : désactivé). Nécessite [traçage](#traces-beta). Le contenu est tronqué à 60 Ko                                                                                                                                                                                                                              | `1` pour activer                                                                                                                                  |
| `OTEL_LOG_RAW_API_BODIES`                           | Émettre les corps JSON complets de la demande et de la réponse de l'API Messages d'Anthropic sous forme d'événements de journaux `api_request_body` / `api_response_body` (par défaut : désactivé). Les corps incluent l'historique complet de la conversation. L'activation de cette option implique le consentement à tout ce que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` et `OTEL_LOG_TOOL_CONTENT` révèleraient | `1` pour les corps en ligne tronqués à 60 Ko, ou `file:<dir>` pour les corps non tronqués sur disque avec un pointeur `body_ref` dans l'événement |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Préférence de temporalité des métriques (par défaut : `delta`). Définissez sur `cumulative` si votre backend attend une temporalité cumulative                                                                                                                                                                                                                                                                               | `delta`, `cumulative`                                                                                                                             |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalle d'actualisation des en-têtes dynamiques (par défaut : 1740000ms / 29 minutes)                                                                                                                                                                                                                                                                                                                                     | `900000`                                                                                                                                          |

<h3 id="mtls-authentication">
  Authentification mTLS
</h3>

La façon dont vous configurez les certificats clients pour l'exportateur OTLP dépend du protocole OTLP utilisé pour ce signal, défini via `OTEL_EXPORTER_OTLP_PROTOCOL` ou le remplacement par signal. La même configuration s'applique aux métriques, journaux et traces.

| Protocole                    | Variables de certificat client                                                                                                                                                                              | Faire confiance au CA du collecteur avec |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, et optionnellement `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Voir [Configuration réseau](/docs/fr/network-config#mtls-authentication)                            | `NODE_EXTRA_CA_CERTS`                    |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` et `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, ou les variantes par signal telles que `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` pour utiliser un certificat différent par signal | `OTEL_EXPORTER_OTLP_CERTIFICATE`         |

Pour `grpc`, le SDK OpenTelemetry lit les variables OTLP standard directement, donc les configurations existantes qui définissent les variables de métriques par signal continuent de fonctionner.

<h3 id="metrics-cardinality-control">
  Contrôle de la cardinalité des métriques
</h3>

Les variables d'environnement suivantes contrôlent les attributs inclus dans les métriques pour gérer la cardinalité :

| Variable d'environnement                   | Description                                                                                           | Valeur par défaut | Exemple pour désactiver |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------- | ----------------- | ----------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Inclure l'attribut session.id dans les métriques                                                      | `true`            | `false`                 |
| `OTEL_METRICS_INCLUDE_VERSION`             | Inclure l'attribut app.version dans les métriques                                                     | `false`           | `true`                  |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Inclure les attributs user.account\_uuid et user.account\_id dans les métriques                       | `true`            | `false`                 |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Inclure l'attribut app.entrypoint dans les métriques                                                  | `false`           | `true`                  |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Inclure les clés de `OTEL_RESOURCE_ATTRIBUTES` comme attributs sur les points de données de métriques | `true`            | `false`                 |

Ces variables aident à contrôler la cardinalité des métriques, ce qui affecte les exigences de stockage et les performances des requêtes dans votre backend de métriques. Une cardinalité plus faible signifie généralement de meilleures performances et des coûts de stockage plus bas, mais des données moins granulaires pour l'analyse.

<h3 id="traces-beta">
  Traces (bêta)
</h3>

Le traçage distribué exporte des intervalles qui lient chaque invite utilisateur aux demandes d'API et aux exécutions d'outils qu'elle déclenche, afin que vous puissiez afficher une demande complète sous forme de trace unique dans votre backend de traçage.

Le traçage est désactivé par défaut. Pour l'activer, définissez à la fois `CLAUDE_CODE_ENABLE_TELEMETRY=1` et `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, puis définissez `OTEL_TRACES_EXPORTER` pour choisir où les intervalles sont envoyés. Les traces réutilisent la [configuration OTLP courante](#common-configuration-variables) pour le point de terminaison, le protocole, les en-têtes et [mTLS](#mtls-authentication).

| Variable d'environnement              | Description                                                                                           | Exemples de valeurs                  |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Activer le traçage d'intervalle (obligatoire). `ENABLE_ENHANCED_TELEMETRY_BETA` est également accepté | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Types d'exportateur de traces, séparés par des virgules. Utilisez `none` pour désactiver              | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocole pour les traces, remplace `OTEL_EXPORTER_OTLP_PROTOCOL`                                     | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Point de terminaison des traces OTLP, remplace `OTEL_EXPORTER_OTLP_ENDPOINT`                          | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalle d'export par lot d'intervalles en millisecondes (par défaut : 5000)                        | `1000`, `10000`                      |

Les intervalles masquent le texte de l'invite utilisateur, les détails d'entrée d'outil et le contenu d'outil par défaut. Définissez `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` et `OTEL_LOG_TOOL_CONTENT=1` pour les inclure.

Lorsque le traçage est actif, les sous-processus Bash et PowerShell héritent automatiquement d'une variable d'environnement `TRACEPARENT` contenant le contexte de trace W3C de l'intervalle d'exécution d'outil actif. Cela permet à tout sous-processus qui lit `TRACEPARENT` de placer ses propres intervalles sous la même trace, permettant le traçage distribué de bout en bout via les scripts et les commandes que Claude exécute.

Lorsque le traçage est actif et que Claude Code est connecté directement à l'API Anthropic, chaque demande de modèle porte un en-tête W3C `traceparent` défini au contexte de l'intervalle `claude_code.llm_request`, et l'en-tête `traceresponse` de l'API est enregistré comme un lien d'intervalle. Ensemble, ceux-ci connectent les intervalles côté client de Claude Code à la trace côté serveur via tout intermédiaire conforme. Les demandes HTTP MCP sortantes portent `traceparent` de la même manière. L'en-tête n'est pas envoyé aux fournisseurs tiers.

Par défaut, l'en-tête `traceparent` sur les demandes de modèle et MCP HTTP est envoyé uniquement lorsque `ANTHROPIC_BASE_URL` n'est pas défini ou pointe vers l'API Anthropic, car certains proxies rejettent les en-têtes non reconnus. La variable `TRACEPARENT` du sous-processus est contrôlée par le même commutateur pour la cohérence. Si vous exécutez Claude Code via un proxy `ANTHROPIC_BASE_URL` personnalisé et souhaitez que le contexte de trace soit propagé, définissez `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

Dans le SDK Agent et les sessions non interactives démarrées avec `-p`, Claude Code lit également `TRACEPARENT` et `TRACESTATE` de son propre environnement au démarrage de chaque intervalle d'interaction. Cela permet à un processus d'intégration de transmettre son contexte de trace W3C actif au sous-processus afin que les intervalles de Claude Code apparaissent comme des enfants de la trace distribuée de l'appelant. Les sessions interactives ignorent `TRACEPARENT` entrant pour éviter d'hériter accidentellement des valeurs ambiantes des environnements CI ou conteneur.

<h4 id="span-hierarchy">
  Hiérarchie des intervalles
</h4>

Chaque invite utilisateur démarre un intervalle racine `claude_code.interaction`. Les appels d'API, les appels d'outils et les exécutions de hooks sont enregistrés comme ses enfants. Les intervalles d'outils ont deux intervalles enfants : un pour le temps passé à attendre une décision de permission et un pour l'exécution elle-même. Lorsque l'outil Agent ou l'outil Task hérité génère un sous-agent, les intervalles d'API et d'outils du sous-agent se placent sous l'intervalle `claude_code.tool` du parent.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (nécessite un traçage bêta détaillé)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (outil Agent) intervalles claude_code.llm_request / claude_code.tool du sous-agent
```

Dans les sessions du SDK Agent et `claude -p`, `claude_code.interaction` lui-même devient un enfant de l'intervalle de l'appelant lorsque `TRACEPARENT` est défini dans l'environnement.

<h4 id="span-attributes">
  Attributs des intervalles
</h4>

Chaque intervalle porte les [attributs standard](#standard-attributes) plus un attribut `span.type` correspondant à son nom. Les tableaux ci-dessous listent les attributs supplémentaires définis sur chaque intervalle. Les intervalles `llm_request`, `tool.execution` et `hook` définissent le statut OpenTelemetry `ERROR` lorsqu'ils enregistrent un échec ; les autres intervalles se terminent toujours avec le statut `UNSET`.

**`claude_code.interaction`**

| Attribut                  | Description                                                                | Contrôlé par            |
| ------------------------- | -------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Texte de l'invite. La valeur est `<REDACTED>` sauf si la porte est définie | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Longueur de l'invite en caractères                                         |                         |
| `interaction.sequence`    | Compteur basé sur 1 des interactions dans cette session                    |                         |
| `interaction.duration_ms` | Durée murale du tour                                                       |                         |

**`claude_code.llm_request`**

| Attribut                         | Description                                                                                                                                    | Contrôlé par            |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Identifiant du modèle                                                                                                                          |                         |
| `gen_ai.system`                  | Toujours `anthropic`. Convention sémantique GenAI OpenTelemetry                                                                                |                         |
| `gen_ai.request.model`           | Même valeur que `model`. Convention sémantique GenAI OpenTelemetry                                                                             |                         |
| `query_source`                   | Sous-système qui a émis la demande, tel que `repl_main_thread` ou un nom de sous-agent                                                         |                         |
| `agent_id`                       | Identifiant du sous-agent ou du coéquipier qui a émis la demande. Absent dans la session principale                                            |                         |
| `parent_agent_id`                | Identifiant de l'agent qui a généré celui-ci. Absent pour la session principale et pour les agents générés directement à partir de celle-ci    |                         |
| `workflow.run_id`                | Identifiant d'exécution de l'exécution de l'outil [Workflow](/docs/fr/workflows), préfixé `wf_`. Absent pour les agents non générés par un workflow |                         |
| `workflow.name`                  | Nom du workflow qui a généré cet agent. Les noms créés par l'utilisateur sont remplacés par `custom` sauf si la porte est définie              | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` ou `normal`                                                                                                                             |                         |
| `llm_request.context`            | `interaction`, `tool`, ou `standalone` selon l'intervalle parent                                                                               |                         |
| `duration_ms`                    | Durée murale incluant les tentatives                                                                                                           |                         |
| `ttft_ms`                        | Temps jusqu'au premier jeton en millisecondes                                                                                                  |                         |
| `input_tokens`                   | Nombre de jetons d'entrée du bloc d'utilisation de l'API                                                                                       |                         |
| `output_tokens`                  | Nombre de jetons de sortie                                                                                                                     |                         |
| `cache_read_tokens`              | Jetons lus à partir du cache de prompt                                                                                                         |                         |
| `cache_creation_tokens`          | Jetons écrits dans le cache de prompt                                                                                                          |                         |
| `request_id`                     | ID de demande d'API Anthropic de l'en-tête de réponse `request-id`                                                                             |                         |
| `gen_ai.response.id`             | Même valeur que `request_id`. Convention sémantique GenAI OpenTelemetry                                                                        |                         |
| `client_request_id`              | `x-client-request-id` généré par le client de la tentative finale                                                                              |                         |
| `attempt`                        | Nombre total de tentatives effectuées pour cette demande                                                                                       |                         |
| `success`                        | `true` ou `false`                                                                                                                              |                         |
| `status_code`                    | Code de statut HTTP lorsque la demande a échoué                                                                                                |                         |
| `error`                          | Message d'erreur lorsque la demande a échoué                                                                                                   |                         |
| `response.has_tool_call`         | `true` lorsque la réponse contenait des blocs tool-use                                                                                         |                         |
| `stop_reason`                    | `stop_reason` de la réponse API, tel que `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, ou `refusal`                     |                         |
| `gen_ai.response.finish_reasons` | Même valeur que `stop_reason`, enveloppée dans un tableau de chaînes. Convention sémantique GenAI OpenTelemetry                                |                         |

Chaque tentative de nouvelle tentative est également enregistrée comme un événement d'intervalle `gen_ai.request.attempt` avec les attributs `attempt` et `client_request_id`.

**`claude_code.tool`**

| Attribut              | Description                                                                                                                                                                                                                                                                          | Contrôlé par            |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `tool_name`           | Nom de l'outil                                                                                                                                                                                                                                                                       |                         |
| `duration_ms`         | Durée murale incluant l'attente de permission et l'exécution                                                                                                                                                                                                                         |                         |
| `result_tokens`       | Taille approximative en jetons du résultat de l'outil                                                                                                                                                                                                                                |                         |
| `agent_id`            | Identifiant du sous-agent ou du coéquipier qui a exécuté l'outil. Absent dans la session principale                                                                                                                                                                                  |                         |
| `parent_agent_id`     | Identifiant de l'agent qui a généré celui-ci. Absent pour la session principale et pour les agents générés directement à partir de celle-ci                                                                                                                                          |                         |
| `workflow.run_id`     | Identifiant d'exécution de l'exécution de l'outil Workflow qui a généré cet agent, préfixé `wf_`. Absent pour les agents non générés par un workflow                                                                                                                                 |                         |
| `workflow.name`       | Nom du workflow qui a généré cet agent. Les noms créés par l'utilisateur sont remplacés par `custom` sauf si la porte est définie                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | L'ID du bloc `tool_use` du modèle pour cet appel. Correspond au `tool_use_id` sur les événements [tool\_result](#tool-result-event) et [tool\_decision](#tool-decision-event) et dans les charges utiles de hooks, afin que vous puissiez joindre l'intervalle à ces enregistrements |                         |
| `gen_ai.tool.call.id` | Même valeur que `tool_use_id`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                             |                         |
| `file_path`           | Chemin de fichier cible pour les outils Read, Edit et Write                                                                                                                                                                                                                          | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Chaîne de commande pour l'outil Bash                                                                                                                                                                                                                                                 | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nom de la compétence pour l'outil Skill                                                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Type de sous-agent pour l'outil Agent ou l'outil Task hérité                                                                                                                                                                                                                         | `OTEL_LOG_TOOL_DETAILS` |

Lorsque `OTEL_LOG_TOOL_CONTENT=1`, cet intervalle enregistre également un événement d'intervalle `tool.output` dont les attributs contiennent les corps d'entrée et de sortie de l'outil, tronqués à 60 Ko par attribut.

**`claude_code.tool.blocked_on_user`**

| Attribut      | Description                                                                                    | Contrôlé par |
| ------------- | ---------------------------------------------------------------------------------------------- | ------------ |
| `duration_ms` | Temps passé à attendre la décision de permission                                               |              |
| `decision`    | `accept` ou `reject`                                                                           |              |
| `source`      | Source de la décision, correspondant à l'événement [Tool decision event](#tool-decision-event) |              |

**`claude_code.tool.execution`**

| Attribut              | Description                                                                                                                                                                       | Contrôlé par            |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Temps passé à exécuter le corps de l'outil                                                                                                                                        |                         |
| `tool_use_id`         | Même valeur que sur l'intervalle parent `claude_code.tool`                                                                                                                        |                         |
| `gen_ai.tool.call.id` | Même valeur que `tool_use_id`. Convention sémantique GenAI OpenTelemetry                                                                                                          |                         |
| `success`             | `true` ou `false`                                                                                                                                                                 |                         |
| `error`               | Chaîne de catégorie d'erreur lorsque l'exécution a échoué, telle que `Error:ENOENT` ou `ShellError`. Contient le message d'erreur complet à la place lorsque la porte est définie | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Cet intervalle est émis uniquement lorsque le traçage bêta détaillé est actif, ce qui nécessite `ENABLE_BETA_TRACING_DETAILED=1` et `BETA_TRACING_ENDPOINT` en plus de la configuration de l'exportateur de trace ci-dessus. Dans les sessions CLI interactives, cela nécessite également que votre organisation soit sur liste blanche pour la fonctionnalité. Les sessions du SDK Agent et non interactives `-p` ne sont pas contrôlées. Il n'est pas émis lorsque seul `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` est défini.

| Attribut                 | Description                                              | Contrôlé par            |
| ------------------------ | -------------------------------------------------------- | ----------------------- |
| `hook_event`             | Type d'événement hook, tel que `PreToolUse`              |                         |
| `hook_name`              | Nom complet du hook, tel que `PreToolUse:Write`          |                         |
| `num_hooks`              | Nombre de commandes hook correspondantes exécutées       |                         |
| `hook_definitions`       | Configuration du hook sérialisée en JSON                 | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Durée murale de tous les hooks correspondants            |                         |
| `num_success`            | Nombre de hooks qui se sont terminés avec succès         |                         |
| `num_blocking`           | Nombre de hooks qui ont retourné une décision de blocage |                         |
| `num_non_blocking_error` | Nombre de hooks qui ont échoué sans bloquer              |                         |
| `num_cancelled`          | Nombre de hooks annulés avant la fin                     |                         |

<Note>
  Les attributs supplémentaires porteurs de contenu tels que `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` et `response.model_output` sont émis uniquement lorsque le traçage bêta détaillé est actif. Ils ne font pas partie du schéma d'intervalle stable. `user_system_prompt` nécessite également `OTEL_LOG_USER_PROMPTS=1`. Il porte uniquement le texte du prompt système que vous fournissez via l'option SDK `systemPrompt` ou les drapeaux `--system-prompt` et `--append-system-prompt`, tronqué à 60 Ko, et est émis une fois par session plutôt que par demande.
</Note>

<h3 id="dynamic-headers">
  En-têtes dynamiques
</h3>

Pour les environnements d'entreprise qui nécessitent une authentification dynamique, vous pouvez configurer un script pour générer des en-têtes dynamiquement. Les en-têtes dynamiques s'appliquent uniquement aux protocoles `http/protobuf` et `http/json`. L'exportateur `grpc` utilise uniquement la valeur statique `OTEL_EXPORTER_OTLP_HEADERS`.

<h4 id="settings-configuration">
  Configuration des paramètres
</h4>

Ajoutez à votre `.claude/settings.json` :

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

La valeur peut être le chemin d'un fichier exécutable, y compris un chemin contenant des espaces, ou une ligne de commande shell avec des arguments. Sur Windows, la valeur s'exécute toujours via le shell, donc mettez entre guillemets un chemin contenant des espaces à l'intérieur de la valeur JSON.

<h4 id="script-requirements">
  Exigences du script
</h4>

Le script doit générer du JSON valide avec des paires clé-valeur de chaînes représentant les en-têtes HTTP :

```bash theme={null}
#!/bin/bash
# Exemple : plusieurs en-têtes
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Si le script d'aide échoue ou imprime une sortie qui ne répond pas à ces exigences, Claude Code signale l'erreur dans :

* La sortie `/status`
* Le journal de débogage, lors de l'exécution avec [`--debug`](/docs/fr/cli-reference#cli-flags) ou après l'exécution de `/debug` dans la session
* stderr, dans les sessions non interactives démarrées avec `-p`

<h4 id="refresh-behavior">
  Comportement d'actualisation
</h4>

Le script d'aide des en-têtes s'exécute au démarrage et périodiquement par la suite pour prendre en charge l'actualisation des jetons. Par défaut, le script s'exécute toutes les 29 minutes. Personnalisez l'intervalle avec la variable d'environnement `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`.

<h3 id="multi-team-organization-support">
  Support des organisations multi-équipes
</h3>

Les organisations avec plusieurs équipes ou départements peuvent ajouter des attributs personnalisés pour distinguer les différents groupes à l'aide de la variable d'environnement `OTEL_RESOURCE_ATTRIBUTES` :

```bash theme={null}
# Ajouter des attributs personnalisés pour l'identification de l'équipe
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Ces attributs personnalisés seront inclus dans toutes les métriques et tous les événements, ce qui vous permet de :

* Filtrer les métriques par équipe ou département
* Suivre les coûts par centre de coûts
* Créer des tableaux de bord spécifiques à l'équipe
* Configurer des alertes pour des équipes spécifiques

Claude Code attache ces valeurs comme attributs sur chaque point de données de métriques et enregistrement d'événement, en plus de les envoyer dans le bloc de ressources OTLP. Parce que la plupart des backends de métriques exposent les attributs de point de données comme des étiquettes interrogeables, vous pouvez regrouper et filtrer les métriques par vos clés personnalisées directement. Les clés personnalisées ne remplacent jamais les [attributs standard](#standard-attributes) tels que `user.id` ou `session.id` : lorsqu'une clé entre en collision, Claude Code conserve la valeur intégrée.

Chaque clé personnalisée devient une étiquette sur chaque série de métriques, donc les valeurs de haute cardinalité augmentent le coût de stockage dans votre backend de métriques. Pour envoyer des attributs personnalisés dans le bloc de ressources uniquement et les omettre des étiquettes de point de données, définissez `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Voir [Contrôle de la cardinalité des métriques](#metrics-cardinality-control).

<Warning>
  La variable d'environnement `OTEL_RESOURCE_ATTRIBUTES` utilise des paires clé=valeur séparées par des virgules avec des exigences de formatage strictes :

  * **Aucun espace autorisé** : Les valeurs ne peuvent pas contenir d'espaces. Par exemple, `user.organizationName=My Company` est invalide
  * **Format** : Doit être des paires clé=valeur séparées par des virgules : `key1=value1,key2=value2`
  * **Caractères autorisés** : Uniquement les caractères US-ASCII à l'exclusion des caractères de contrôle, des espaces, des guillemets doubles, des virgules, des points-virgules et des barres obliques inverses
  * **Caractères spéciaux** : Les caractères en dehors de la plage autorisée doivent être codés en pourcentage

  Pour une valeur qui aurait besoin d'un espace, utilisez des traits de soulignement ou camelCase à la place. Les exemples suivants définissent `org.name` avec chaque forme :

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Vous pouvez coder en pourcentage n'importe quel caractère, pas seulement les caractères exclus. Cet exemple code à la fois l'espace et l'apostrophe :

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Entourer les valeurs de guillemets n'échappe pas aux espaces. Par exemple, `org.name="My Company"` donne la valeur littérale `"My Company"` avec les guillemets inclus, pas `My Company`.
</Warning>

<h3 id="example-configurations">
  Exemples de configurations
</h3>

Définissez ces variables d'environnement avant d'exécuter `claude`. Chaque bloc montre une configuration complète pour un exportateur ou un scénario de déploiement différent :

```bash theme={null}
# Débogage de console (intervalles de 1 seconde)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# Plusieurs exportateurs
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Points de terminaison/backends différents pour les métriques et les journaux
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Métriques uniquement (pas d'événements/journaux)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Événements/journaux uniquement (pas de métriques)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Métriques et événements disponibles
</h2>

<h3 id="standard-attributes">
  Attributs standard
</h3>

Toutes les métriques et tous les événements partagent ces attributs standard :

| Attribut                           | Description                                                                                                                                                                                                                                                                            | Contrôlé par                                                   |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `session.id`                       | Identifiant de session unique                                                                                                                                                                                                                                                          | `OTEL_METRICS_INCLUDE_SESSION_ID` (par défaut : true)          |
| `app.version`                      | Version actuelle de Claude Code                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_VERSION` (par défaut : false)            |
| `app.entrypoint`                   | Comment la session a été lancée, par exemple `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, ou `claude-vscode`                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (par défaut : false)         |
| `organization.id`                  | UUID de l'organisation (si authentifié)                                                                                                                                                                                                                                                | Toujours inclus si disponible                                  |
| `user.account_uuid`                | UUID du compte (si authentifié)                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (par défaut : true)        |
| `user.account_id`                  | ID du compte au format balisé correspondant aux API d'administration Anthropic (si authentifié), tel que `user_01BWBeN28...`                                                                                                                                                           | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (par défaut : true)        |
| `user.id`                          | Identifiant anonyme aléatoire généré à la première exécution et conservé dans `~/.claude.json`. Il ne contient aucune information personnelle et n'est pas dérivé de votre compte Claude. La suppression du fichier produit une nouvelle valeur sans rapport à la prochaine exécution. | Toujours inclus                                                |
| `user.email`                       | Adresse e-mail de l'utilisateur (si authentifié via OAuth)                                                                                                                                                                                                                             | Toujours inclus si disponible                                  |
| `terminal.type`                    | Type de terminal, tel que `iTerm.app`, `vscode`, `cursor`, ou `tmux`                                                                                                                                                                                                                   | Toujours inclus si détecté                                     |
| Clés de `OTEL_RESOURCE_ATTRIBUTES` | Attributs personnalisés que vous définissez, tels que `department` ou `team.id`. Voir [Support des organisations multi-équipes](#multi-team-organization-support)                                                                                                                      | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (par défaut : true) |

Lorsque Claude Code est connecté à une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), la CLI marque les exportations avec l'identité authentifiée de la session de la passerelle : `user.id` est le sujet IdP plutôt qu'un identifiant d'installation anonyme, `user.email` est l'e-mail connecté, et `user.groups` porte l'appartenance au groupe IdP sous forme de chaîne séparée par des virgules. Chaque exportation porte également `identity.source: gateway-oidc`. L'identité de la passerelle est appliquée en dernier, donc les clés `user.*` et `identity.*` définies via `OTEL_RESOURCE_ATTRIBUTES` sont ignorées sur les sessions de passerelle.

Les événements incluent en outre les attributs suivants. Ceux-ci ne sont jamais attachés aux métriques car ils causeraient une cardinalité non bornée :

* `prompt.id` : UUID corrélant une invite utilisateur avec tous les événements suivants jusqu'à l'invite suivante. Voir [Attributs de corrélation d'événements](#event-correlation-attributes).
* `workspace.host_paths` : répertoires d'espace de travail hôte sélectionnés dans l'application de bureau, sous forme de tableau de chaînes
* `workflow.run_id` : identifiant d'exécution, préfixé `wf_`, sur les événements d'API et d'outil émis par les agents qui appartiennent à une exécution d'outil [Workflow](/docs/fr/workflows). Le filtrage des événements par un `workflow.run_id` reconstruit les demandes d'API et les résultats d'outil de cette exécution. L'identifiant couvre les agents que le script de flux de travail génère et tous les agents que ceux-ci génèrent à leur tour, tels que les invocations de compétences. Il correspond à l'identifiant d'exécution signalé dans le résultat de l'outil Workflow. Absent sur tous les autres événements. Nécessite Claude Code v2.1.202 ou ultérieur
* `workflow.name` : nom du flux de travail, le `meta.name` de son script, émis aux côtés de `workflow.run_id`. Les noms de flux de travail intégrés apparaissent textuellement lorsque l'exécution exécute le script intégré non modifié. Les noms créés par l'utilisateur, y compris les copies modifiées de scripts intégrés, sont remplacés par `custom` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Nécessite Claude Code v2.1.202 ou ultérieur

<h3 id="metrics">
  Métriques
</h3>

Claude Code exporte les métriques suivantes :

| Nom de la métrique                    | Description                                                    | Unité  |
| ------------------------------------- | -------------------------------------------------------------- | ------ |
| `claude_code.session.count`           | Nombre de sessions CLI démarrées                               | count  |
| `claude_code.lines_of_code.count`     | Nombre de lignes de code modifiées                             | count  |
| `claude_code.pull_request.count`      | Nombre de demandes de tirage créées                            | count  |
| `claude_code.commit.count`            | Nombre de commits git créés                                    | count  |
| `claude_code.cost.usage`              | Coût de la session Claude Code                                 | USD    |
| `claude_code.token.usage`             | Nombre de jetons utilisés                                      | tokens |
| `claude_code.code_edit_tool.decision` | Nombre de décisions de permission de l'outil d'édition de code | count  |
| `claude_code.active_time.total`       | Temps actif total en secondes                                  | s      |

<h3 id="metric-details">
  Détails des métriques
</h3>

Chaque métrique inclut les attributs standard listés ci-dessus. Les métriques avec des attributs supplémentaires spécifiques au contexte sont notées ci-dessous.

<h4 id="session-counter">
  Compteur de sessions
</h4>

Incrémenté au début de chaque session.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `start_type` : Comment la session a été démarrée. L'un de `"fresh"`, `"resume"`, `"continue"`, ou `"agents_view"`. La valeur `"agents_view"` identifie le processus du tableau de bord `claude agents`, une interface utilisateur locale lancée par l'utilisateur plutôt qu'une session conversationnelle. Filtrez sur cette valeur pour séparer les lancements de processus d'interface utilisateur des sessions conversationnelles dans vos tableaux de bord.

<h4 id="lines-of-code-counter">
  Compteur de lignes de code
</h4>

Incrémenté lorsque du code est ajouté ou supprimé.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : (`"added"`, `"removed"`)
* `model` : Identifiant du modèle pour le modèle qui a effectué la modification (par exemple, « claude-sonnet-5 »)

<h4 id="pull-request-counter">
  Compteur de demandes de tirage
</h4>

Incrémenté lors de la création de demandes de tirage ou de demandes de fusion via une commande shell ou un outil MCP.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)

<h4 id="commit-counter">
  Compteur de commits
</h4>

Incrémenté lors de la création de commits git via Claude Code.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)

<h4 id="cost-counter">
  Compteur de coûts
</h4>

Incrémenté après chaque demande d'API.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `query_source` : Catégorie du sous-système qui a émis la demande. L'un de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed` : `"fast"` lorsque la demande a utilisé le mode rapide. Absent sinon
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande : `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Absent lorsque le modèle ne supporte pas l'effort.
* `agent.name` : Type de sous-agent qui a émis la demande. Les noms d'agents intégrés et les agents des plugins de la place de marché officielle apparaissent textuellement. Les autres noms d'agents définis par l'utilisateur sont remplacés par `"custom"`. Absent lorsque la demande n'a pas été émise par un type de sous-agent nommé.
* `skill.name` : Compétence active pour la demande, définie par l'outil Skill, une commande `/`, ou héritée par un sous-agent généré. Les noms de compétences intégrées, groupées, définies par l'utilisateur et de plugin de place de marché officielle apparaissent textuellement. Les noms de compétences de plugin tiers sont remplacés par `"third-party"`. Absent lorsqu'aucune compétence n'est active.
* `plugin.name` : Plugin propriétaire lorsque la compétence active ou le sous-agent est fourni par un plugin. Les noms de plugins de place de marché officielle apparaissent textuellement. Les noms de plugins tiers sont remplacés par `"third-party"`. Absent lorsque ni la compétence ni le sous-agent n'a de plugin propriétaire.
* `marketplace.name` : Place de marché à partir de laquelle le plugin propriétaire a été installé. Émis uniquement pour les plugins de place de marché officielle. Absent sinon.
* `mcp_server.name` : Serveur MCP dont l'outil a été exécuté dans le tour qui a produit cette demande. Les noms de serveurs intégrés, proxifiés par claude.ai et de registre officiel apparaissent textuellement. Les noms de serveurs configurés par l'utilisateur sont remplacés par `"custom"`. Absent lorsqu'aucun outil MCP n'a été exécuté.
* `mcp_tool.name` : Outil MCP qui a été exécuté dans le tour qui a produit cette demande, avec la même rédaction que `mcp_server.name`. Absent lorsqu'aucun outil MCP n'a été exécuté.

<h4 id="token-counter">
  Compteur de jetons
</h4>

Incrémenté après chaque demande d'API.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `query_source` : Catégorie du sous-système qui a émis la demande. L'un de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed` : `"fast"` lorsque la demande a utilisé le mode rapide. Absent sinon
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Voir [Compteur de coûts](#cost-counter) pour les détails.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coûts](#cost-counter) pour les définitions et le comportement de masquage.

<h4 id="code-edit-tool-decision-counter">
  Compteur de décisions de l'outil d'édition de code
</h4>

Incrémenté lorsque l'utilisateur accepte ou rejette l'utilisation de l'outil Edit, Write ou NotebookEdit.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `tool_name` : Nom de l'outil (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision` : Décision de l'utilisateur (`"accept"`, `"reject"`)
* `source` : Source de la décision. L'un de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, ou `"user_reject"`. Voir l'[Événement de décision d'outil](#tool-decision-event) pour savoir ce que chaque valeur signifie.
* `language` : Langage de programmation du fichier édité, tel que `"TypeScript"`, `"Python"`, `"JavaScript"`, ou `"Markdown"`. Retourne `"unknown"` pour les extensions de fichier non reconnues.

<h4 id="active-time-counter">
  Compteur de temps actif
</h4>

Suit le temps réel passé à utiliser activement Claude Code, excluant le temps d'inactivité. Cette métrique est incrémentée lors des interactions utilisateur (saisie, lecture des réponses) et lors du traitement CLI (exécution d'outils, génération de réponses IA).

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : `"user"` pour les interactions au clavier, `"cli"` pour l'exécution d'outils et les réponses IA

<h3 id="events">
  Événements
</h3>

Claude Code exporte les événements suivants via les journaux/événements OpenTelemetry (lorsque `OTEL_LOGS_EXPORTER` est configuré) :

<h4 id="event-correlation-attributes">
  Attributs de corrélation d'événements
</h4>

Lorsqu'un utilisateur soumet une invite, Claude Code peut effectuer plusieurs appels d'API et exécuter plusieurs outils. L'attribut `prompt.id` vous permet de lier tous ces événements à l'invite unique qui les a déclenchés.

| Attribut    | Description                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------- |
| `prompt.id` | Identifiant UUID v4 liant tous les événements produits lors du traitement d'une invite utilisateur unique |

Pour tracer toute l'activité déclenchée par une invite unique, filtrez vos événements par une valeur `prompt.id` spécifique. Cela retourne l'événement user\_prompt, tous les événements api\_request, et tous les événements tool\_result qui se sont produits lors du traitement de cette invite.

<Note>
  `prompt.id` est intentionnellement exclu des métriques car chaque invite génère un ID unique, ce qui créerait un nombre toujours croissant de séries chronologiques. Utilisez-le uniquement pour l'analyse au niveau des événements et les pistes d'audit.
</Note>

<h4 id="user-prompt-event">
  Événement d'invite utilisateur
</h4>

Enregistré lorsqu'un utilisateur soumet une invite.

**Nom de l'événement** : `claude_code.user_prompt`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"user_prompt"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `prompt_length` : Longueur de l'invite
* `prompt` : Contenu de l'invite. Masqué par défaut. Définissez `OTEL_LOG_USER_PROMPTS=1` pour l'inclure
* `command_name` : Nom de la commande lorsque l'invite en invoque une. Les noms de commandes intégrées et groupées tels que `compact` ou `debug` sont émis tels quels ; les alias tels que `reset` émettent tels que tapés plutôt que le nom canonique. Les noms de commandes personnalisées, de plugin et MCP s'effondrent en `custom` ou `mcp` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini
* `command_source` : Origine de la commande lorsqu'elle est présente : `builtin`, `custom`, ou `mcp`. Les commandes fournies par les plugins signalent comme `custom`

<h4 id="assistant-response-event">
  Événement de réponse d'assistant
</h4>

Enregistré après chaque demande d'API qui retourne du contenu textuel du modèle. Seuls les blocs de texte de la réponse sont inclus ; les blocs de réflexion et les blocs d'utilisation d'outil sont exclus. Nécessite Claude Code v2.1.193 ou ultérieur.

**Nom de l'événement** : `claude_code.assistant_response`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"assistant_response"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `response_length` : Longueur du texte de réponse en caractères
* `response` : Texte de réponse, tronqué à 60 Ko. Masqué à `<REDACTED>` par défaut. Définissez `OTEL_LOG_ASSISTANT_RESPONSES=1` pour l'inclure. Lorsque `OTEL_LOG_ASSISTANT_RESPONSES` n'est pas défini, `OTEL_LOG_USER_PROMPTS` le contrôle à la place, donc définissez `OTEL_LOG_ASSISTANT_RESPONSES=0` pour garder les réponses masquées tandis que la journalisation des invites est activée
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `request_id` : ID de demande d'API Anthropic de l'en-tête `request-id` de la réponse. Présent uniquement lorsque l'API en retourne un
* `query_source` : Sous-système qui a émis la demande, tel que `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent

<h4 id="tool-result-event">
  Événement de résultat d'outil
</h4>

Enregistré lorsqu'un outil termine son exécution. Non émis si l'appel d'outil a été rejeté ; voir l'[Événement de décision d'outil](#tool-decision-event) pour les rejets.

**Nom de l'événement** : `claude_code.tool_result`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"tool_result"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `tool_name` : Nom de l'outil
* `tool_use_id` : Identifiant unique pour cette invocation d'outil. Correspond au `tool_use_id` passé aux hooks, permettant la corrélation entre les événements OTel et les données capturées par les hooks.
* `success` : `"true"` ou `"false"`
* `duration_ms` : Temps d'exécution en millisecondes
* `error_type` : Chaîne de catégorie d'erreur lorsque l'outil a échoué, telle que `"Error:ENOENT"` ou `"ShellError"`
* `error` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Message d'erreur complet lorsque l'outil a échoué
* `decision_type` : Toujours `"accept"`, puisque cet événement n'est émis qu'après l'exécution de l'outil. Les appels rejetés ne produisent pas de résultat d'outil
* `decision_source` : Source de la décision de permission. L'un de `"config"`, `"hook"`, `"user_permanent"`, ou `"user_temporary"`. Voir l'[Événement de décision d'outil](#tool-decision-event) pour savoir ce que chaque valeur signifie. Les sources de rejet uniquement `"user_abort"` et `"user_reject"` n'apparaissent jamais sur cet événement.
* `tool_input_size_bytes` : Taille de l'entrée d'outil sérialisée en JSON en octets
* `tool_result_size_bytes` : Taille du résultat de l'outil en octets
* `mcp_server_scope` : Identifiant de portée du serveur MCP (pour les outils MCP)
* `tool_parameters` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Chaîne JSON contenant les paramètres spécifiques à l'outil :
  * Pour l'outil Bash : inclut `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`, et `git_commit_id` (le SHA du commit, lorsqu'une commande `git commit` réussit)
  * Pour l'outil WorkspaceBash : inclut `bash_command`, `full_command`, `timeout`
  * Pour les outils MCP : inclut `mcp_server_name`, `mcp_tool_name`
  * Pour l'outil Skill : inclut `skill_name`
  * Pour l'outil Agent ou l'outil Task hérité : inclut `subagent_type`
* `tool_input` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Arguments d'outil sérialisés en JSON. Les valeurs individuelles dépassant 512 caractères sont tronquées, et la charge utile complète est limitée à environ 4 K caractères. S'applique à tous les outils, y compris les outils MCP.

<h4 id="api-request-event">
  Événement de demande d'API
</h4>

Enregistré pour chaque demande d'API à Claude.

**Nom de l'événement** : `claude_code.api_request`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_request"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `model` : Modèle utilisé (par exemple, « claude-sonnet-5 »)
* `cost_usd` : Coût estimé en USD
* `duration_ms` : Durée de la demande en millisecondes
* `input_tokens` : Nombre de jetons d'entrée
* `output_tokens` : Nombre de jetons de sortie
* `cache_read_tokens` : Nombre de jetons lus à partir du cache
* `cache_creation_tokens` : Nombre de jetons utilisés pour la création du cache
* `request_id` : ID de demande d'API Anthropic de l'en-tête `request-id` de la réponse, tel que `"req_011..."`. Présent uniquement lorsque l'API en retourne un.
* `speed` : `"fast"` ou `"normal"`, indiquant si le mode rapide était actif
* `query_source` : Sous-système qui a émis la demande, tel que `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande : `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Absent lorsque le modèle ne supporte pas l'effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coûts](#cost-counter) pour les définitions et le comportement de masquage.

<h4 id="api-error-event">
  Événement d'erreur d'API
</h4>

Enregistré lorsqu'une demande d'API à Claude échoue.

**Nom de l'événement** : `claude_code.api_error`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_error"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `model` : Modèle utilisé (par exemple, « claude-sonnet-5 »)
* `error` : Message d'erreur
* `status_code` : Code de statut HTTP sous forme de nombre. Absent pour les erreurs non-HTTP telles que les défaillances de connexion.
* `duration_ms` : Durée de la demande en millisecondes
* `attempt` : Nombre total de tentatives effectuées, y compris la demande initiale (`1` signifie qu'aucune nouvelle tentative ne s'est produite)
* `request_id` : ID de demande d'API Anthropic de l'en-tête `request-id` de la réponse, tel que `"req_011..."`. Présent uniquement lorsque l'API en retourne un.
* `speed` : `"fast"` ou `"normal"`, indiquant si le mode rapide était actif
* `query_source` : Sous-système qui a émis la demande, tel que `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Absent lorsque le modèle ne supporte pas l'effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coûts](#cost-counter) pour les définitions et le comportement de masquage.

<h4 id="api-refusal-event">
  Événement de refus d'API
</h4>

Enregistré lorsqu'une demande d'API retourne `stop_reason: "refusal"`. Les refus arrivent sur un flux de réponse réussi plutôt que comme une erreur HTTP, donc l'événement `api_error` ne se déclenche pas pour eux. Cet événement vous permet de suivre la fréquence des refus et de regrouper les refus par les mêmes attributs que `api_request` et `api_error`.

**Nom de l'événement** : `claude_code.api_refusal`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_refusal"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `model` : Identifiant du modèle de la demande
* `request_id` : ID de demande d'API Anthropic de l'en-tête `request-id` de la réponse, tel que `"req_011..."`. Présent uniquement lorsque l'API en retourne un.
* `query_source` : Sous-système qui a émis la demande, tel que `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent. Voir [`api_request`](#api-request-event) pour les définitions.
* `speed` : Soit `"fast"` lorsque le [Mode rapide](/docs/fr/fast-mode) est actif, soit `"normal"`
* `attempt` : Numéro de tentative de nouvelle tentative. La première tentative est `1`.
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Absent lorsque le modèle ne supporte pas l'effort.
* `server_fallback_hop` : `true` lorsque le basculement de modèle côté serveur de l'API a déjà réessayé ce refus sur un modèle différent, donc l'utilisateur n'a pas vu ce refus particulier. `false` lorsque la demande s'est terminée par un refus. Un seul tour peut émettre à la fois un événement `true` hop et un événement final `false` ultérieur lorsque le modèle de secours refuse également.
* `has_category` : `true` lorsque la réponse de l'API contenait une `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. `false` lorsque la réponse ne contenait aucune catégorie ou une valeur en dehors de cet ensemble. Absent lorsque `server_fallback_hop` est `true`, car les blocs hop ne portent pas `stop_details`.
* `has_explanation` : `true` lorsque la réponse de l'API contenait une `stop_details.explanation`, sinon `false`. Absent lorsque `server_fallback_hop` est `true`.
* `category` : La valeur `stop_details.category` de la réponse de l'API. L'un de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. Présent uniquement lorsque `OTEL_LOG_TOOL_DETAILS=1` est défini et `has_category` est `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coûts](#cost-counter) pour les définitions et le comportement de masquage.

<h4 id="api-request-body-event">
  Événement de corps de demande d'API
</h4>

Enregistré pour chaque tentative de demande d'API lorsque `OTEL_LOG_RAW_API_BODIES` est défini. Un événement est émis par tentative, donc les nouvelles tentatives avec des paramètres ajustés produisent chacune leur propre événement.

**Nom de l'événement** : `claude_code.api_request_body`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_request_body"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `body` : Paramètres de demande de l'API Messages sérialisés en JSON (invite système, messages, outils, etc.), tronqués à 60 Ko. Le contenu de la réflexion étendue dans les tours d'assistant antérieurs est masqué. Émis uniquement en mode en ligne (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref` : Chemin absolu vers un fichier `<dir>/<uuid>.request.json` contenant le corps non tronqué. Émis uniquement en mode fichier (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length` : Longueur du corps non tronqué. Octets UTF-8 lorsque `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unités de code UTF-16 lorsque `=1`
* `body_truncated` : `"true"` lorsque la troncature en ligne s'est produite. Absent en mode fichier et lorsqu'aucune troncature ne s'est produite.
* `model` : Identifiant du modèle à partir des paramètres de demande
* `query_source` : Sous-système qui a émis la demande (par exemple, `"compact"`)

<h4 id="api-response-body-event">
  Événement de corps de réponse d'API
</h4>

Enregistré pour chaque réponse d'API réussie lorsque `OTEL_LOG_RAW_API_BODIES` est défini.

**Nom de l'événement** : `claude_code.api_response_body`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_response_body"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `body` : Réponse de l'API Messages sérialisée en JSON (id, blocs de contenu, utilisation, raison d'arrêt), tronquée à 60 Ko. Le contenu de la réflexion étendue est masqué. Émis uniquement en mode en ligne (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref` : Chemin absolu vers un fichier `<dir>/<request_id>.response.json` contenant le corps non tronqué. Émis uniquement en mode fichier (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length` : Longueur du corps non tronqué. Octets UTF-8 lorsque `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unités de code UTF-16 lorsque `=1`
* `body_truncated` : `"true"` lorsque la troncature en ligne s'est produite. Absent en mode fichier et lorsqu'aucune troncature ne s'est produite.
* `model` : Identifiant du modèle
* `query_source` : Sous-système qui a émis la demande
* `request_id` : ID de demande d'API Anthropic de l'en-tête `request-id` de la réponse, tel que `"req_011..."`. Présent uniquement lorsque l'API en retourne un.

<h4 id="tool-decision-event">
  Événement de décision d'outil
</h4>

Enregistré lorsqu'une décision de permission d'outil est prise (accepter/rejeter).

**Nom de l'événement** : `claude_code.tool_decision`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"tool_decision"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `tool_name` : Nom de l'outil (par exemple, « Read », « Edit », « Write », « NotebookEdit »)
* `tool_use_id` : Identifiant unique pour cette invocation d'outil. Correspond au `tool_use_id` passé aux hooks, permettant la corrélation entre les événements OTel et les données capturées par les hooks.
* `decision` : Soit `"accept"` soit `"reject"`
* `source` : Source de la décision :
  * `"config"` : Décidé automatiquement sans invite, basé sur les paramètres du projet, les règles d'autorisation dans les paramètres personnels de l'utilisateur, la politique gérée par l'entreprise, les drapeaux `--allowedTools` ou `--disallowedTools`, le mode de permission actif, une autorisation limitée à la session d'une invite antérieure dans la même session CLI interactive, ou parce que l'outil est intrinsèquement sûr. L'événement n'indique pas laquelle de ces sources a correspondu.
  * `"hook"` : Un hook `PreToolUse` ou `PermissionRequest` a retourné la décision.
  * `"user_permanent"` : Émis lorsque l'utilisateur a choisi « Oui, et ne me demande plus pour ... » à une invite de permission, ce qui enregistre une règle d'autorisation dans ses paramètres personnels. Dans la CLI interactive, ceci est émis uniquement pour ce choix lui-même ; les appels ultérieurs qui correspondent à la règle enregistrée émettent `"config"` à la place. Dans le SDK Agent ou les sessions `-p` non-interactives, à la fois le choix initial et les correspondances de règles ultérieures émettent `"user_permanent"`. Traité comme une acceptation.
  * `"user_temporary"` : Émis lorsque l'utilisateur a choisi « Oui » à une invite de permission pour une approbation unique, ou a choisi l'une des options « ... pendant cette session » sur une invite d'édition ou de lecture de fichier. Dans la CLI interactive, ceci est émis uniquement pour le choix lui-même ; les appels ultérieurs autorisés par cette autorisation limitée à la session émettent `"config"` à la place. Dans le SDK Agent ou les sessions `-p` non-interactives, à la fois le choix et les correspondances ultérieures émettent `"user_temporary"`. Traité comme une acceptation.
  * `"user_abort"` : Émis lorsque l'utilisateur a fermé l'invite de permission sans répondre. Traité comme un rejet.
  * `"user_reject"` : Émis lorsque l'utilisateur a choisi « Non » lorsqu'il a été invité. Dans la CLI interactive, ceci est émis uniquement pour ce choix lui-même ; les appels qui correspondent à une règle de refus dans les paramètres personnels de l'utilisateur émettent `"config"` à la place. Dans le SDK Agent ou les sessions `-p` non-interactives, les appels qui correspondent à une règle de refus dans les paramètres personnels émettent `"user_reject"`. Traité comme un rejet.
* `tool_parameters` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Chaîne JSON contenant les paramètres spécifiques à l'outil. Même forme que l'[Événement de résultat d'outil](#tool-result-event), moins les champs post-exécution tels que `git_commit_id`. Les valeurs peuvent différer de `tool_result` pour un appel accepté si la décision de permission réécrit l'entrée d'outil via `updatedInput`. Utilisez cet attribut pour voir quelle commande a été rejetée lorsque `decision` est `"reject"`.
  * Pour l'outil Bash : inclut `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Pour l'outil WorkspaceBash : inclut `bash_command`, `full_command`, `timeout`
  * Pour les outils MCP : inclut `mcp_server_name`, `mcp_tool_name`
  * Pour l'outil Skill : inclut `skill_name`
  * Pour l'outil Agent ou l'outil Task hérité : inclut `subagent_type`

<h4 id="permission-mode-changed-event">
  Événement de changement de mode de permission
</h4>

Enregistré lorsque le mode de permission change, par exemple à partir du cycle Shift+Tab, de la sortie du mode plan ou d'une vérification de porte en mode automatique.

**Nom de l'événement** : `claude_code.permission_mode_changed`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"permission_mode_changed"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `from_mode` : Le mode de permission précédent, par exemple `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, ou `"bypassPermissions"`
* `to_mode` : Le nouveau mode de permission
* `trigger` : Ce qui a causé le changement. L'un de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, ou `"auto_opt_in"`. Absent lorsque la transition provient du SDK ou du pont

<h4 id="auth-event">
  Événement d'authentification
</h4>

Enregistré lorsque `/login` ou `/logout` se termine.

**Nom de l'événement** : `claude_code.auth`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"auth"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `action` : `"login"` ou `"logout"`
* `success` : `"true"` ou `"false"`
* `auth_method` : Méthode d'authentification, telle que `"oauth"`
* `error_category` : Type d'erreur catégorique lorsque l'action a échoué. Le message d'erreur brut n'est jamais inclus
* `status_code` : Code de statut HTTP sous forme de chaîne lorsque l'action a échoué avec une erreur HTTP

<h4 id="mcp-server-connection-event">
  Événement de connexion du serveur MCP
</h4>

Enregistré lorsqu'un serveur MCP se connecte, se déconnecte ou échoue à se connecter.

**Nom de l'événement** : `claude_code.mcp_server_connection`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"mcp_server_connection"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `status` : `"connected"`, `"failed"`, ou `"disconnected"`
* `transport_type` : Transport du serveur, tel que `"stdio"`, `"sse"`, ou `"http"`
* `server_scope` : Portée à laquelle le serveur est configuré, telle que `"user"`, `"project"`, ou `"local"`
* `duration_ms` : Durée de la tentative de connexion en millisecondes
* `error_code` : Code d'erreur lorsque la connexion a échoué
* `is_plugin` : `true` lorsque le serveur est fourni par un plugin, `false` sinon
* `plugin_id_hash` (lorsque `is_plugin` est `true`) : Hash stable du nom du plugin et de la place de marché, pour regrouper les événements par plugin sans exposer le nom
* `plugin.name` (lorsque `is_plugin` est `true`) : Nom du plugin qui fournit le serveur. Pour les plugins tiers, ceci est la chaîne littérale `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1` ; cela protège les noms de plugins tiers d'apparaître dans les journaux par défaut. Les plugins provenant de sources officielles d'Anthropic sont toujours identifiés par nom. Les attributs `plugin_id_hash` et `plugin.name` circulent vers votre propre backend de surveillance et ne sont pas envoyés à Anthropic
* `server_name` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Nom du serveur configuré
* `error` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : Message d'erreur complet lorsque la connexion a échoué

<h4 id="internal-error-event">
  Événement d'erreur interne
</h4>

Enregistré lorsque Claude Code détecte une erreur interne inattendue. Seul le nom de la classe d'erreur et un code de style errno sont enregistrés. Le message d'erreur et la trace de pile ne sont jamais inclus. Cet événement n'est pas émis lors de l'exécution sur Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry, ou lorsque `DISABLE_ERROR_REPORTING` est défini.

**Nom de l'événement** : `claude_code.internal_error`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"internal_error"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `error_name` : Nom de la classe d'erreur, tel que `"TypeError"` ou `"SyntaxError"`
* `error_code` : Code errno Node.js tel que `"ENOENT"` lorsqu'il est présent sur l'erreur

<h4 id="plugin-installed-event">
  Événement de plugin installé
</h4>

Enregistré lorsqu'un plugin termine l'installation, à partir de la commande CLI `claude plugin install` et de l'interface utilisateur interactive `/plugin`.

**Nom de l'événement** : `claude_code.plugin_installed`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"plugin_installed"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `marketplace.is_official` : `"true"` si la place de marché est une place de marché officielle d'Anthropic, `"false"` sinon
* `install.trigger` : `"cli"` ou `"ui"`
* `plugin.name` : Nom du plugin installé. Pour les places de marché tierces, ceci est inclus uniquement lorsque `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version` : Version du plugin lorsqu'elle est déclarée dans l'entrée de la place de marché. Pour les places de marché tierces, ceci est inclus uniquement lorsque `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name` : Place de marché à partir de laquelle le plugin a été installé. Pour les places de marché tierces, ceci est inclus uniquement lorsque `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Événement de plugin chargé
</h4>

Enregistré une fois par plugin activé au démarrage de la session. Utilisez cet événement pour inventorier les plugins actifs dans votre flotte, en complément de `plugin_installed` qui enregistre l'action d'installation elle-même.

**Nom de l'événement** : `claude_code.plugin_loaded`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"plugin_loaded"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `plugin.name` : nom du plugin. Pour les plugins en dehors de la place de marché officielle et du bundle intégré, la valeur est `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name` : place de marché à partir de laquelle le plugin a été installé, lorsqu'elle est connue. Masquée à `"third-party"` sous la même condition que `plugin.name`
* `plugin.version` : version du manifeste du plugin. Inclus uniquement lorsque le nom n'est pas masqué et que le manifeste déclare une version
* `plugin.scope` : catégorie de provenance du plugin : `"official"`, `"org"`, `"user-local"`, ou `"default-bundle"`
* `enabled_via` : comment le plugin en est venu à être activé : `"default-enable"`, `"org-policy"`, `"seed-mount"`, ou `"user-install"`
* `plugin_id_hash` : hash déterministe du nom du plugin et de la place de marché, envoyé uniquement à votre exportateur configuré. Vous permet de compter combien de plugins tiers distincts sont chargés dans votre flotte sans enregistrer leurs noms
* `has_hooks` : si le plugin contribue des hooks
* `has_mcp` : si le plugin contribue des serveurs MCP
* `host_owned_mcp` : `true` lorsque l'hôte SDK gère les connexions MCP de ce plugin et Claude Code a ignoré la lecture de la configuration du serveur MCP du plugin, `false` sinon. Nécessite Claude Code v2.1.172 ou ultérieur
* `skill_path_count` : nombre de répertoires de compétences que le plugin déclare
* `command_path_count` : nombre de répertoires de commandes que le plugin déclare
* `agent_path_count` : nombre de répertoires d'agents que le plugin déclare
* `safe_mode` : `"true"` lorsque la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. En mode sûr, cet événement rapporte l'inventaire configuré uniquement ; les commandes, compétences, hooks et serveurs MCP du plugin ne se chargent pas. Nécessite Claude Code v2.1.169 ou ultérieur

<h4 id="skill-activated-event">
  Événement de compétence activée
</h4>

Enregistré lorsqu'une compétence est invoquée, que Claude l'appelle via l'outil Skill ou que vous l'exécutiez en tant que commande `/`.

**Nom de l'événement** : `claude_code.skill_activated`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"skill_activated"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `skill.name` : Nom de la compétence. Pour les compétences définies par l'utilisateur et les compétences de plugin tiers, la valeur est l'espace réservé `"custom_skill"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger` : Comment la compétence a été déclenchée (`"user-slash"`, `"claude-proactive"`, ou `"nested-skill"`)
* `skill.source` : D'où la compétence a été chargée (par exemple, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind` : `"workflow"` lorsque la compétence est une compétence de flux de travail. Absent sinon
* `plugin.name` (lorsque `OTEL_LOG_TOOL_DETAILS=1` ou le plugin provient d'une place de marché officielle) : Nom du plugin propriétaire lorsque la compétence est fournie par un plugin
* `marketplace.name` (lorsque `OTEL_LOG_TOOL_DETAILS=1` ou le plugin provient d'une place de marché officielle) : Place de marché du plugin propriétaire, lorsque la compétence est fournie par un plugin

<h4 id="at-mention-event">
  Événement de mention @
</h4>

Enregistré lorsque Claude Code résout une mention `@` dans une invite. Pas chaque mention n'émet un événement : les chemins de sortie anticipée tels que les refus de permission, les fichiers surdimensionnés, les pièces jointes de référence PDF et les défaillances de listage de répertoires retournent sans enregistrement.

**Nom de l'événement** : `claude_code.at_mention`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"at_mention"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `mention_type` : Type de mention (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success` : Si la mention a été résolue avec succès (`"true"` ou `"false"`)

<h4 id="api-retries-exhausted-event">
  Événement de tentatives d'API épuisées
</h4>

Enregistré une fois lorsqu'une demande d'API échoue après plus d'une tentative. Émis aux côtés de l'événement `api_error` final.

**Nom de l'événement** : `claude_code.api_retries_exhausted`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_retries_exhausted"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `model` : Modèle utilisé
* `error` : Message d'erreur final
* `status_code` : Code de statut HTTP sous forme de nombre. Absent pour les erreurs non-HTTP.
* `total_attempts` : Nombre total de tentatives effectuées
* `total_retry_duration_ms` : Temps mural total sur toutes les tentatives
* `speed` : `"fast"` ou `"normal"`

<h4 id="hook-registered-event">
  Événement de hook enregistré
</h4>

Enregistré une fois par hook configuré au démarrage de la session. Utilisez cet événement pour inventorier les hooks actifs dans votre flotte, en complément des événements `hook_execution_start` et `hook_execution_complete` par exécution.

**Nom de l'événement** : `claude_code.hook_registered`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_registered"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `hook_event` : type d'événement hook, tel que `"PreToolUse"` ou `"PostToolUse"`
* `hook_type` : type d'implémentation du hook : `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, ou `"agent"`
* `hook_source` : où le hook est défini : `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, ou `"pluginHook"`
* `safe_mode` : `"true"` lorsque la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_matcher` (lorsque `OTEL_LOG_TOOL_DETAILS=1`) : la chaîne matcher de la configuration du hook, lorsqu'elle est définie
* `plugin.name` (lorsque `hook_source` est `"pluginHook"`) : nom du plugin contributeur. Pour les plugins en dehors de la place de marché officielle et du bundle intégré, la valeur est `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (lorsque `hook_source` est `"pluginHook"`) : hash déterministe du nom du plugin et de la place de marché, envoyé uniquement à votre exportateur configuré. Vous permet de compter les plugins contributeurs distincts sans enregistrer leurs noms

<h4 id="hook-execution-start-event">
  Événement de démarrage d'exécution de hook
</h4>

Enregistré lorsqu'un ou plusieurs hooks commencent à s'exécuter pour un événement de hook.

**Nom de l'événement** : `claude_code.hook_execution_start`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_execution_start"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `hook_event` : Type d'événement hook, tel que `"PreToolUse"` ou `"PostToolUse"`
* `hook_name` : Nom complet du hook incluant le matcher, tel que `"PreToolUse:Write"`
* `num_hooks` : Nombre de commandes hook correspondantes
* `managed_only` : `"true"` lorsque seuls les hooks de politique gérée sont autorisés
* `hook_source` : `"policySettings"` ou `"merged"`
* `safe_mode` : `"true"` lorsque la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_definitions` : Configuration du hook sérialisée en JSON. Inclus uniquement lorsque le traçage bêta détaillé et `OTEL_LOG_TOOL_DETAILS=1` sont tous deux activés

<h4 id="hook-execution-complete-event">
  Événement de fin d'exécution de hook
</h4>

Enregistré lorsque tous les hooks pour un événement de hook ont terminé.

**Nom de l'événement** : `claude_code.hook_execution_complete`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_execution_complete"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `hook_event` : Type d'événement hook
* `hook_name` : Nom complet du hook incluant le matcher
* `num_hooks` : Nombre de commandes hook correspondantes
* `num_success` : Nombre qui se sont terminées avec succès
* `num_blocking` : Nombre qui ont retourné une décision de blocage
* `num_non_blocking_error` : Nombre qui ont échoué sans bloquer
* `num_cancelled` : Nombre annulé avant la fin
* `total_duration_ms` : Durée murale de tous les hooks correspondants
* `managed_only` : `"true"` lorsque seuls les hooks de politique gérée sont autorisés
* `hook_source` : `"policySettings"` ou `"merged"`
* `safe_mode` : `"true"` lorsque la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_definitions` : Configuration du hook sérialisée en JSON. Inclus uniquement lorsque le traçage bêta détaillé et `OTEL_LOG_TOOL_DETAILS=1` sont tous deux activés

<h4 id="hook-plugin-metrics-event">
  Événement de métriques de plugin hook
</h4>

Enregistré lorsqu'un hook de plugin de place de marché officielle émet des métriques par invocation. Seuls les plugins installés à partir d'une place de marché officielle d'Anthropic peuvent émettre ces métriques. Les plugins de place de marché tiers et les hooks configurés par l'utilisateur n'émettent pas vers cet événement. Utilisez cet événement pour surveiller le comportement des plugins tels que les taux de découverte, les coûts et les durées à partir de votre propre pile d'observabilité.

**Nom de l'événement** : `claude_code.hook_plugin_metrics`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_plugin_metrics"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `plugin_id` : identifiant du plugin sous la forme `<name>@<marketplace>`
* `hook_event` : type d'événement hook qui a émis les métriques
* Jusqu'à 20 clés de métriques émises par le plugin. Les noms correspondent à `^[a-z][a-z0-9_]{0,39}$`. Les valeurs sont booléennes ou numériques.

<h4 id="compaction-event">
  Événement de compaction
</h4>

Enregistré lorsque la compaction de conversation se termine.

**Nom de l'événement** : `claude_code.compaction`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"compaction"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `trigger` : `"auto"` ou `"manual"`
* `success` : `"true"` ou `"false"`
* `duration_ms` : Durée de la compaction
* `pre_tokens` : Nombre approximatif de jetons avant la compaction
* `post_tokens` : Nombre approximatif de jetons après la compaction
* `error` : Message d'erreur lorsque la compaction a échoué
* `precompute_reuse` : Défini uniquement lorsque `trigger` est `"manual"`. La compaction automatique peut préparer un résumé en arrière-plan avant que la fenêtre de contexte ne se remplisse, et cet attribut enregistre si `/compact` a réutilisé ce résumé préparé. `"hit"` signifie qu'il a été réutilisé ; `"miss_custom_instructions"`, `"miss_hook"`, et `"miss_not_ready"` donnent la raison pour laquelle un résumé frais a été calculé à la place. Nécessite Claude Code v2.1.153 ou ultérieur

<h4 id="feedback-survey-event">
  Événement de sondage de rétroaction
</h4>

Enregistré lorsqu'un sondage de qualité de session est affiché ou auquel il est répondu. Voir [Sondages de qualité de session](/docs/fr/data-usage#session-quality-surveys) pour savoir ce que les sondages collectent et comment les contrôler.

**Nom de l'événement** : `claude_code.feedback_survey`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"feedback_survey"`
* `event.timestamp` : Horodatage ISO 8601
* `event.sequence` : Compteur monotone croissant pour ordonner les événements au sein d'une session
* `event_type` : Événement du cycle de vie du sondage, par exemple `"appeared"`, `"responded"`, ou `"transcript_prompt_appeared"`
* `appearance_id` : ID unique liant les événements émis pour une instance de sondage
* `survey_type` : Quel sondage a produit l'événement. `"session"` est l'invite d'évaluation « Comment Claude se débrouille-t-il ? »
* `response` : La sélection de l'utilisateur sur les événements `responded`
* `enabled_via_override` : `true` lorsque [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/fr/env-vars) est défini. Émis en tant que booléen, pas une chaîne. Présent sur les événements de sondage `session`. Filtrez sur cet attribut pour confirmer que le remplacement est appliqué dans votre flotte

<h2 id="interpret-metrics-and-events-data">
  Interpréter les données de métriques et d'événements
</h2>

Les métriques et événements exportés prennent en charge une gamme d'analyses :

<h3 id="usage-monitoring">
  Surveillance de l'utilisation
</h3>

| Métrique                                                      | Opportunité d'analyse                                                                                          |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Ventiler par `type` (entrée/sortie), utilisateur, équipe, modèle, `skill.name`, `plugin.name`, ou `agent.name` |
| `claude_code.session.count`                                   | Suivre l'adoption et l'engagement au fil du temps                                                              |
| `claude_code.lines_of_code.count`                             | Mesurer la productivité en suivant les ajouts et suppressions de code, ventilés par modèle                     |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Comprendre l'impact sur les flux de travail de développement                                                   |

<h3 id="cost-monitoring">
  Surveillance des coûts
</h3>

La métrique `claude_code.cost.usage` aide à :

* Suivre les tendances d'utilisation entre les équipes ou les individus
* Identifier les sessions à utilisation élevée pour l'optimisation
* Attribuer les dépenses à des compétences, des plugins ou des types de sous-agents spécifiques via les attributs `skill.name`, `plugin.name`, et `agent.name`

<Note>
  Les métriques de coûts sont des approximations. Pour les données de facturation officielles, consultez votre fournisseur d'API (Claude Console, Amazon Bedrock ou Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Alertes et segmentation
</h3>

Les alertes courantes à considérer :

* Pics de coûts
* Consommation de jetons inhabituelle
* Volume de session élevé d'utilisateurs spécifiques

Toutes les métriques peuvent être segmentées par les [attributs standard](#standard-attributes). L'attribut `model` est disponible sur `claude_code.token.usage`, `claude_code.cost.usage`, et à partir de la v2.1.172, `claude_code.lines_of_code.count`.

Les ventilations par modèle des commits ne peuvent être approximées que en joignant les métriques de jetons ou de coûts sur `session.id`, puisqu'une session peut s'étendre sur plusieurs modèles. Filtrez le côté jetons ou coûts pour les lignes où `query_source` est `"main"` afin que les demandes auxiliaires et de sous-agents n'attribuent pas les commits de la session à un modèle qui ne les a pas effectués.

<h3 id="detect-retry-exhaustion">
  Détecter l'épuisement des tentatives
</h3>

Claude Code réessaie les demandes d'API échouées en interne et n'émet un seul événement `claude_code.api_error` qu'après avoir abandonné, donc l'événement lui-même est le signal terminal pour cette demande. Les tentatives de nouvelle tentative intermédiaires ne sont pas enregistrées comme des événements séparés.

L'attribut `attempt` sur l'événement enregistre le nombre total de tentatives effectuées. `CLAUDE_CODE_MAX_RETRIES` est par défaut `10` et plafonné à `15` ; à partir de la v2.1.199, `CLAUDE_CODE_RETRY_WATCHDOG` augmente la valeur par défaut et supprime le plafond. Lorsque la demande épuise toutes les tentatives sur une erreur transitoire, `attempt` est égal à un de plus que cette limite effective : 11 par défaut, et jamais plus de 16 sauf si le watchdog est défini. Une valeur inférieure indique une erreur non réessayable telle qu'une réponse `400`.

Pour distinguer une session qui s'est rétablie d'une qui s'est bloquée, groupez les événements par `session.id` et vérifiez si un événement `api_request` ultérieur existe après l'erreur.

<h3 id="event-analysis">
  Analyse des événements
</h3>

Les données d'événements fournissent des informations détaillées sur les interactions de Claude Code :

**Modèles d'utilisation des outils** : analyser les événements de résultat d'outil pour identifier :

* Les outils les plus fréquemment utilisés
* Les taux de réussite des outils
* Les temps d'exécution moyens des outils
* Les modèles d'erreur par type d'outil

**Surveillance des performances** : suivre les durées des demandes d'API et les temps d'exécution des outils pour identifier les goulots d'étranglement de performance.

<h2 id="audit-security-events">
  Audit des événements de sécurité
</h2>

Les événements OpenTelemetry sont la source de données d'audit pour l'activité de Claude Code. Chaque événement porte des attributs d'identité qui lient les appels d'outils, l'activité MCP et les décisions de permission à l'utilisateur qui les a déclenchés. L'exportateur de journaux OTLP peut livrer ces événements à n'importe quelle plateforme SIEM (Security Information and Event Management) avec un récepteur OTLP, ou à un collecteur OpenTelemetry qui transfère vers votre SIEM.

<h3 id="attribute-actions-to-users">
  Attribuer les actions aux utilisateurs
</h3>

Les [attributs standard](#standard-attributes) sur chaque événement incluent l'identité de l'utilisateur authentifié : `user.email`, `user.account_uuid`, `user.account_id`, et `organization.id` lorsqu'il est connecté avec un compte Claude, plus `user.id` et le per-session `session.id`. `user.id` est un identifiant limité à l'installation, sauf sur les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), où il s'agit du sujet IdP du jeton émis par la passerelle.

Les appels d'outils MCP, les commandes Bash et les éditions de fichiers sont donc attribués au développeur qui a démarré la session. Claude Code n'agit pas sous un compte de service distinct ; l'identité enregistrée sur chaque événement est le propre compte Claude du développeur, ou l'identité IdP du développeur sur une session de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway).

Lorsque Claude Code s'authentifie avec une clé API directe, ou contre Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, il n'y a pas de compte Claude dans la session et seuls `user.id` et `session.id` sont remplis. Dans ces déploiements, attachez l'identité utilisateur vous-même avec `OTEL_RESOURCE_ATTRIBUTES`, défini par utilisateur via le fichier [paramètres gérés](#administrator-configuration) ou un wrapper de lancement. Les sessions de passerelle d'applications Claude n'ont besoin d'aucune de ces opérations : le CLI horodate l'identité IdP automatiquement, comme décrit dans [Attributs standard](#standard-attributes).

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Audit de l'activité MCP
</h3>

Pour capturer l'activité du serveur MCP avec tous les détails d'appel, activez l'exportateur de journaux et définissez `OTEL_LOG_TOOL_DETAILS=1`. Chaque opération MCP produit alors des événements structurés qui portent le nom du serveur, le nom de l'outil et les arguments d'appel aux côtés des attributs d'identité standard :

| Événement               | Ce qu'il enregistre pour MCP                                                                                                                                                                                         |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | Connexion du serveur, déconnexion et défaillance de connexion avec `server_name`, `transport_type`, `server_scope`, et détail d'erreur                                                                               |
| `tool_result`           | Chaque appel d'outil MCP avec `tool_name` et `mcp_server_scope`, une charge utile `tool_parameters` contenant `mcp_server_name` et `mcp_tool_name`, et une charge utile `tool_input` contenant les arguments d'appel |
| `tool_decision`         | Si l'appel a été autorisé ou refusé, si la décision provenait de la configuration, d'un hook ou de l'utilisateur, et une charge utile `tool_parameters` contenant `mcp_server_name` et `mcp_tool_name`               |

Sans `OTEL_LOG_TOOL_DETAILS`, ces événements suppriment le détail d'identification :

* `tool_result` : conserve `tool_name` et `mcp_server_scope`, omet `mcp_server_name`, `mcp_tool_name`, et les arguments
* `tool_decision` : conserve `tool_name`, omet `tool_parameters`
* `mcp_server_connection` : omet `server_name` et le message d'erreur, mais conserve `is_plugin`, `plugin_id_hash`, et `plugin.name`, avec les noms de plugins non-Anthropic redactés au littéral `"third-party"`, de sorte que les serveurs fournis par les plugins restent distinguables sans journalisation détaillée

<h3 id="map-security-questions-to-events">
  Mapper les questions de sécurité aux événements
</h3>

Lors de la création de règles de détection, recherchez le signal que vous souhaitez surveiller et interrogez votre backend pour l'événement correspondant et les attributs :

| Signal                                                   | Événement                                                                          | Attributs clés                                               |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Appel d'outil autorisé ou refusé, et par quoi            | `tool_decision`                                                                    | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Escalade du mode de permission                           | `permission_mode_changed`                                                          | `from_mode`, `to_mode`, `trigger`                            |
| Hook de politique a bloqué une action                    | `hook_execution_complete`                                                          | `hook_event`, `num_blocking`                                 |
| Connexion, déconnexion et défaillance d'authentification | `auth`                                                                             | `action`, `success`, `error_category`                        |
| Connexion du serveur MCP ou défaillance                  | `mcp_server_connection`                                                            | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin installé et sa source                             | `plugin_installed`                                                                 | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Commandes exécutées et fichiers touchés                  | `tool_result` (exécuté) ou `tool_decision` (rejeté) avec `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters` ; `tool_input` (`tool_result` uniquement)  |

Claude Code émet uniquement le flux d'événements brut. La détection d'anomalies, l'établissement de lignes de base, la corrélation entre les sessions et les alertes sont la responsabilité de votre SIEM ou backend d'observabilité.

<h3 id="send-events-to-a-siem">
  Envoyer les événements à un SIEM
</h3>

Pointez `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` vers le récepteur OTLP de votre SIEM, ou vers un collecteur OpenTelemetry qui transfère vers l'API d'ingestion native de votre SIEM. L'exemple de paramètres gérés suivant exporte uniquement les événements, avec tous les détails d'outil activés pour l'audit MCP et Bash :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  Considérations relatives aux backends
</h2>

Votre choix de backends de métriques, de journaux et de traces détermine les types d'analyses que vous pouvez effectuer :

<h3 id="for-metrics">
  Pour les métriques
</h3>

* **Bases de données de séries chronologiques (par exemple, Prometheus)** : Calculs de taux, métriques agrégées
* **Magasins colonnaires (par exemple, ClickHouse)** : Requêtes complexes, analyse d'utilisateurs uniques
* **Plates-formes d'observabilité complètes (par exemple, Honeycomb, Datadog, Grafana Cloud)** : Requêtes avancées, visualisation, alertes

<h3 id="for-events/logs">
  Pour les événements/journaux
</h3>

* **Systèmes d'agrégation de journaux (par exemple, Elasticsearch, Loki)** : Recherche en texte intégral, analyse de journaux
* **Magasins colonnaires (par exemple, ClickHouse)** : Analyse d'événements structurés
* **Plates-formes d'observabilité complètes (par exemple, Honeycomb, Datadog, Grafana Cloud)** : Corrélation entre les métriques et les événements

<h3 id="for-traces">
  Pour les traces
</h3>

Choisissez un backend qui prend en charge le stockage de traces distribuées et la corrélation d'intervalles :

* **Systèmes de traçage distribué (par exemple, Jaeger, Zipkin, Grafana Tempo)** : Visualisation d'intervalles, cascades de demandes, analyse de latence
* **Plates-formes d'observabilité complètes (par exemple, Honeycomb, Datadog, Grafana Cloud)** : Recherche de traces et corrélation avec les métriques et les journaux

Pour les organisations nécessitant des métriques d'utilisateurs actifs quotidiens/hebdomadaires/mensuels (DAU/WAU/MAU), envisagez des backends qui prennent en charge les requêtes de valeurs uniques efficaces.

<h2 id="service-information">
  Informations sur le service
</h2>

Toutes les métriques et tous les événements sont exportés avec les attributs de ressource suivants :

* `service.name` : `claude-code`
* `service.version` : Version actuelle de Claude Code
* `os.type` : Type de système d'exploitation (par exemple, `linux`, `darwin`, `windows`)
* `os.version` : Chaîne de version du système d'exploitation
* `host.arch` : Architecture de l'hôte (par exemple, `amd64`, `arm64`)
* `wsl.version` : Numéro de version WSL (présent uniquement lors de l'exécution sur Windows Subsystem for Linux)
* Nom du compteur : `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  Ressources de mesure du ROI
</h2>

Pour un guide complet sur la mesure du retour sur investissement pour Claude Code, y compris la configuration de la télémétrie, l'analyse des coûts, les métriques de productivité et les rapports automatisés, consultez le [Guide de mesure du ROI de Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Ce référentiel fournit des configurations Docker Compose prêtes à l'emploi, des configurations Prometheus et OpenTelemetry, et des modèles pour générer des rapports de productivité intégrés à des outils comme Linear.

<h2 id="security-and-privacy">
  Sécurité et confidentialité
</h2>

* L'export OpenTelemetry vers votre backend est opt-in et nécessite une configuration explicite. Pour la télémétrie opérationnelle distincte d'Anthropic et comment la désactiver, consultez [Utilisation des données](/docs/fr/data-usage#telemetry-services)
* Les contenus de fichiers bruts et les extraits de code ne sont pas inclus dans les métriques ou les événements. Les intervalles de trace constituent un chemin de données distinct : voir la puce `OTEL_LOG_TOOL_CONTENT` ci-dessous
* Lorsqu'authentifié via OAuth, `user.email` est inclus dans les attributs de télémétrie. Si cela pose un problème pour votre organisation, travaillez avec votre backend de télémétrie pour filtrer ou masquer ce champ
* Le contenu des invites utilisateur n'est pas collecté par défaut. Seule la longueur de l'invite est enregistrée. Pour inclure le contenu de l'invite, définissez `OTEL_LOG_USER_PROMPTS=1`
* Le texte de réponse de l'assistant n'est pas collecté par défaut. Seule la longueur de la réponse est enregistrée. Pour inclure le texte de réponse, définissez `OTEL_LOG_ASSISTANT_RESPONSES=1`. Comme toutes les données OpenTelemetry de Claude Code, le texte de réponse est envoyé uniquement au point de terminaison OTel que vous configurez, jamais à Anthropic. Lorsque cette variable n'est pas définie, `OTEL_LOG_USER_PROMPTS` est utilisé comme solution de secours, donc définissez `OTEL_LOG_ASSISTANT_RESPONSES=0` si vous souhaitez le contenu de l'invite sans contenu de réponse
* Les arguments d'entrée d'outil et les paramètres ne sont pas enregistrés par défaut. Pour les inclure, définissez `OTEL_LOG_TOOL_DETAILS=1`. Ces données sont envoyées uniquement au point de terminaison OTEL que vous configurez, jamais à Anthropic. Les arguments peuvent toujours contenir des valeurs sensibles, donc configurez votre backend de télémétrie pour filtrer ou masquer ces attributs selon les besoins. Lorsqu'activé :
  * Les événements `tool_result` et `tool_decision` incluent un attribut `tool_parameters` avec les commandes Bash, les noms de serveur MCP et d'outil, et les noms de compétences. Les champs tels que `full_command` sont émis sans troncature
  * Les événements `tool_result` incluent également un attribut `tool_input` avec les chemins de fichiers, les URL, les modèles de recherche et d'autres arguments. Les valeurs individuelles dépassant 512 caractères sont tronquées et le total est limité à environ 4 K caractères
  * Les événements `user_prompt` incluent le `command_name` verbatim pour les commandes personnalisées, de plugin et MCP
  * Les intervalles de trace incluent le même attribut `tool_input` et les attributs dérivés de l'entrée tels que `file_path`, avec la même troncature que `tool_input`
* Le contenu d'entrée et de sortie d'outil n'est pas enregistré dans les intervalles de trace par défaut. Pour l'inclure, définissez `OTEL_LOG_TOOL_CONTENT=1`. Lorsqu'activé, les événements d'intervalle incluent le contenu complet d'entrée et de sortie d'outil tronqué à 60 Ko par intervalle. Cela peut inclure les contenus de fichiers bruts des résultats de l'outil Read et la sortie de commande Bash. Configurez votre backend de télémétrie pour filtrer ou masquer ces attributs selon les besoins
* Les corps bruts de la demande et de la réponse de l'API Messages d'Anthropic ne sont pas enregistrés par défaut. Pour les inclure, définissez `OTEL_LOG_RAW_API_BODIES`. Avec `=1`, chaque appel d'API émet des événements de journaux `api_request_body` et `api_response_body` dont l'attribut `body` est la charge utile sérialisée en JSON, tronquée à 60 Ko. Avec `=file:<dir>`, les corps non tronqués sont écrits dans les fichiers `.request.json` et `.response.json` sous ce répertoire et les événements portent un chemin `body_ref` à la place du corps en ligne. Livrez le répertoire avec un collecteur de journaux ou un sidecar plutôt que via le flux de télémétrie. Dans les deux modes, les corps contiennent l'historique complet de la conversation (invite système, chaque tour d'utilisateur et d'assistant antérieur, résultats d'outils), donc l'activation de cette option implique le consentement à tout ce que les autres drapeaux de contenu `OTEL_LOG_*` révèleraient. Le contenu de réflexion étendue de Claude est toujours masqué de ces corps indépendamment des autres paramètres

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Surveiller Claude Code sur Amazon Bedrock
</h2>

Pour des conseils détaillés sur la surveillance de l'utilisation de Claude Code pour Amazon Bedrock, consultez [Implémentation de la surveillance de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
