> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Surveillance

> Découvrez comment activer et configurer OpenTelemetry pour Claude Code.

Suivez l'utilisation de Claude Code, les coûts et l'activité des outils dans votre organisation en exportant les données de télémétrie via OpenTelemetry (OTel). Claude Code exporte les métriques sous forme de données de séries chronologiques via le protocole de métriques standard, les événements via le protocole de journaux/événements, et optionnellement les traces distribuées via le [protocole de traces](#traces-beta).

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

# 5. Pour le débogage : réduire les intervalles d'export, et les réinitialiser pour une utilisation en production
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 secondes (par défaut : 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 secondes (par défaut : 5000ms)

# 6. Exécuter Claude Code
claude
```

Pour vérifier une configuration qui exporte des métriques, vérifiez votre backend pour la métrique `claude_code.session.count`, que Claude Code émet au démarrage d'une session. Pour vérifier une configuration réservée aux journaux, soumettez une invite et vérifiez l'événement `claude_code.user_prompt`.

Si rien n'arrive, exécutez `claude --debug` et vérifiez le journal de débogage. Claude Code signale les défaillances des exportateurs que vous configurez en tant qu'erreurs `[3P telemetry]`, où 3P signifie tiers. Les lignes préfixées par `[Anthropic telemetry]` décrivent la [télémétrie opérationnelle distincte d'Anthropic](/docs/fr/data-usage#telemetry-services) et n'indiquent pas un problème avec votre configuration.

Pour les options de configuration complètes, consultez la [spécification OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuration de l'administrateur
</h2>

Les administrateurs peuvent configurer les paramètres OpenTelemetry pour tous les utilisateurs via le [fichier de paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms). Consultez la [précédence des paramètres](/docs/fr/settings#settings-precedence) pour plus d'informations sur la façon dont les paramètres sont appliqués.

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

Claude Code ignore les [variables d'exportateur OpenTelemetry](/docs/fr/settings-reference#variables-claude-code-ignores-in-env) dans le `.claude/settings.json` et `.claude/settings.local.json` d'un référentiel, donc un référentiel ne peut pas les utiliser pour activer la télémétrie, choisir où elle va ou capturer du contenu. Définissez-les dans les paramètres gérés, ou laissez chaque développeur les définir dans son shell ou `~/.claude/settings.json`. Un référentiel peut toujours désactiver un signal en définissant son sélecteur d'exportateur, tel que `OTEL_LOGS_EXPORTER`, sur `none`, sauf si les paramètres gérés, un fichier `--settings` ou l'environnement à partir duquel vous lancez Claude Code définit cette variable.

Claude Code ne transmet pas les variables d'environnement `OTEL_*` aux sous-processus qu'il génère, y compris l'outil Bash, les hooks, les serveurs MCP et les serveurs de langage. Une application instrumentée par OpenTelemetry que vous exécutez via l'outil Bash n'hérite pas du point de terminaison de l'exportateur ou des en-têtes de Claude Code, donc définissez ces variables directement dans la commande si cette application doit exporter sa propre télémétrie.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  Comment les paramètres gérés verrouillent la destination OTLP
</h3>

Lorsque vous définissez une variable `OTEL_EXPORTER_OTLP_*` dans les paramètres gérés, Claude Code supprime les variables conflictuelles définies par les développeurs au démarrage et enregistre un avertissement que vous pouvez voir avec `claude --debug`. Ce qu'il supprime dépend de la variable que vous définissez :

* **Points de terminaison** : lorsque vous définissez `OTEL_EXPORTER_OTLP_ENDPOINT`, Claude Code supprime tous les points de terminaison par signal définis par les développeurs. Les développeurs ne peuvent pas pointer un signal vers un collecteur différent, donc vous n'avez pas besoin de définir également les variables de point de terminaison par signal dans les paramètres gérés.
* **Protocoles** : lorsque vous définissez `OTEL_EXPORTER_OTLP_PROTOCOL`, Claude Code supprime tous les protocoles par signal définis par les développeurs.
* **Identifiants** : lorsque vous définissez `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY` ou `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, Claude Code supprime les versions par signal de cette variable définies par les développeurs, plus toutes les variables de point de terminaison définies par les développeurs, génériques ou par signal, car ces identifiants atteindraient autrement un collecteur que les paramètres gérés n'ont pas choisi.
* **Sélecteurs d'exportateur** : `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` et le `OTEL_TRACES_EXPORTER` bêta suivent la précédence normale par clé. Un paramètre de développeur peut toujours désactiver un signal ou le basculer vers l'exportateur de console, donc définissez également les sélecteurs dans les paramètres gérés si vous avez besoin qu'ils soient verrouillés. Parmi les [sources d'administrateur](/docs/fr/managed-settings#precedence-within-the-managed-tier), `OTEL_LOGS_EXPORTER` suit l'[unité de télémétrie](/docs/fr/server-managed-settings#per-key-exceptions-across-managed-sources) tandis que les deux autres sélecteurs fusionnent par clé. Nécessite Claude Code v2.1.223 ou version ultérieure.
* **Points de terminaison de traçage bêta** : avec le [traçage bêta détaillé](#traces-beta) actif, Claude Code exporte les journaux et les traces vers `BETA_TRACING_ENDPOINT` au lieu de passer par les exportateurs de journaux et de traces. Claude Code supprime donc un `BETA_TRACING_ENDPOINT` défini par le développeur chaque fois que l'un de ces paramètres gérés décide de la destination de l'un ou l'autre signal :

  * Un point de terminaison ou un identifiant générique ou de journaux/traces
  * Un [`otelHeadersHelper`](/docs/fr/settings-reference#otelheadershelper)
  * Un sélecteur d'exportateur de journaux ou de traces défini sur `none`, `console` ou vide, des valeurs qui maintiennent le signal hors d'un collecteur
  * `CLAUDE_CODE_ENABLE_TELEMETRY` désactivé

  Un point de terminaison ou un identifiant réservé aux métriques ne le supprime pas. Avant v2.1.251, un `BETA_TRACING_ENDPOINT` défini par le développeur redirigait les journaux et les traces que le traçage bêta détaillé exporte même lorsque les paramètres gérés épinglaient le collecteur.

Claude Code ne supprime pas les variables par signal que vous définissez dans les paramètres gérés eux-mêmes, donc vous pouvez router un signal vers un collecteur différent en définissant sa variable là, comme le fait l'[exemple SIEM](#send-events-to-a-siem). Si vous définissez un identifiant par signal là, Claude Code supprime le point de terminaison défini par le développeur pour ce signal.

Ce comportement de suppression change où la télémétrie est livrée, pas ce que Claude Code collecte.

Avant v2.1.217, chaque variable suivait la précédence des paramètres par clé indépendamment, donc un point de terminaison spécifique au signal défini dans les paramètres utilisateur ou le shell redirigait ce signal loin du collecteur géré.

Lorsque l'application de bureau ou un exécuteur d'[environnement auto-hébergé](/docs/fr/self-hosted-environments) lance Claude Code et nomme un point de terminaison OTLP dans l'environnement qu'il fournit, Claude Code épingle la destination de la même manière : les variables de télémétrie du lanceur suppriment les variables définies par les développeurs exactement comme le font les paramètres gérés. Claude Code ne supprime pas les variables que le lanceur lui-même a définies. Nécessite Claude Code v2.1.251 ou version ultérieure.

<h2 id="configuration-details">
  Détails de la configuration
</h2>

<h3 id="common-configuration-variables">
  Variables de configuration courantes
</h3>

Ces variables configurent les exportateurs, les points de terminaison et le comportement d'export pour tous les déploiements.

Si vous définissez une variable de point de terminaison ou de protocole par signal, telle que `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, Claude Code l'utilise à la place de la variable générique pour ce signal. Si vous définissez une variable d'en-têtes par signal, telle que `OTEL_EXPORTER_OTLP_METRICS_HEADERS`, Claude Code la fusionne avec la variable générique `OTEL_EXPORTER_OTLP_HEADERS` pour ce signal.

Sur les machines avec des paramètres gérés, voir [Comment les paramètres gérés verrouillent la destination OTLP](#how-managed-settings-lock-the-otlp-destination) pour savoir ce que Claude Code supprime.

| Variable d'environnement                            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Exemples de valeurs                                                                                                                                                                 |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Active la collecte de télémétrie (obligatoire)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `1`                                                                                                                                                                                 |
| `OTEL_METRICS_EXPORTER`                             | Types d'exportateur de métriques, séparés par des virgules. Utilisez `none` pour désactiver                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `console`, `otlp`, `prometheus`, `none`                                                                                                                                             |
| `OTEL_LOGS_EXPORTER`                                | Types d'exportateur de journaux/événements, séparés par des virgules. Utilisez `none` pour désactiver                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `console`, `otlp`, `none`                                                                                                                                                           |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocole pour l'exportateur OTLP, s'applique à tous les signaux. Claude Code n'a pas de protocole par défaut, donc définissez ceci ou la variable de protocole spécifique au signal pour chaque exportateur `otlp` que vous activez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `grpc`, `http/json`, `http/protobuf`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Point de terminaison du collecteur OTLP pour tous les signaux                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `http://localhost:4317`                                                                                                                                                             |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocole pour les métriques, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `grpc`, `http/json`, `http/protobuf`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Point de terminaison des métriques OTLP, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `http://localhost:4318/v1/metrics`                                                                                                                                                  |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocole pour les journaux, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `grpc`, `http/json`, `http/protobuf`                                                                                                                                                |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Point de terminaison des journaux OTLP, remplace le paramètre général                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `http://localhost:4318/v1/logs`                                                                                                                                                     |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | En-têtes d'authentification pour OTLP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `Authorization=Bearer token`                                                                                                                                                        |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | En-têtes d'authentification pour les métriques, fusionnés avec les en-têtes généraux                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `Authorization=Bearer token`                                                                                                                                                        |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | En-têtes d'authentification pour les journaux, fusionnés avec les en-têtes généraux                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `Authorization=Bearer token`                                                                                                                                                        |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalle d'export en millisecondes (par défaut : 60000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `5000`, `60000`                                                                                                                                                                     |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalle d'export des journaux en millisecondes (par défaut : 5000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `1000`, `10000`                                                                                                                                                                     |
| `OTEL_LOG_USER_PROMPTS`                             | Activer la journalisation du contenu des invites utilisateur (par défaut : désactivé)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `1` pour activer                                                                                                                                                                    |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Activer la journalisation du texte de réponse de l'assistant sur les événements `assistant_response` (par défaut : désactivé). Lorsque non défini, revient à la valeur de `OTEL_LOG_USER_PROMPTS`. Nécessite Claude Code v2.1.193 ou version ultérieure                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `1` pour activer, `0` pour garder masqué                                                                                                                                            |
| `OTEL_LOG_TOOL_DETAILS`                             | Activer la journalisation des paramètres d'outil et des arguments d'entrée dans les événements d'outil et les attributs d'intervalle de trace : commandes Bash, noms de serveur MCP et d'outil, noms de compétences, noms de workflow créés par l'utilisateur et entrée d'outil. Active également les noms de commandes personnalisées, de plugin et MCP sur les événements `user_prompt` (par défaut : désactivé). Pour les serveurs intégrés de Claude Desktop, dans les sessions que Claude Desktop possède, `mcp_server_name`/`mcp_tool_name` sont émis sur `tool_decision`/`tool_result` même avec l'indicateur désactivé. L'exception nécessite Claude Code v2.1.214 ou version ultérieure                                                                                                                                                      | `1` pour activer                                                                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT`                             | Activer la journalisation du contenu d'outil dans l'[événement d'intervalle `tool.output`](#tool-output-span-event) (par défaut : désactivé). Les attributs d'intervalle portent le contenu d'outil sous [leurs propres portes](#new-context-gates). Nécessite [traçage](#traces-beta). Le contenu est tronqué à la limite de contenu (60 Ko par défaut)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `1` pour activer                                                                                                                                                                    |
| `OTEL_LOG_MANAGED_SETTINGS`                         | Ajouter les paramètres gérés masqués et un résumé SHA-256 des paramètres avant masquage aux événements [managed settings resolved](#managed-settings-resolved-event) (par défaut : désactivé). Une valeur dans les paramètres de projet ou locaux ne l'active pas. Nécessite Claude Code v2.1.274 ou version ultérieure                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `1` pour activer                                                                                                                                                                    |
| `OTEL_LOG_RAW_API_BODIES`                           | Émettre les corps JSON complets de la demande et de la réponse de l'API Messages d'Anthropic sous forme d'événements de journaux `api_request_body` / `api_response_body` (par défaut : désactivé). Les corps incluent l'historique complet de la conversation. L'activation de cette option implique le consentement à tout ce que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` et `OTEL_LOG_TOOL_CONTENT` révèleraient                                                                                                                                                                                                                                                                                                                                                                                                                          | `1` pour les corps en ligne tronqués à la limite de contenu (60 Ko par défaut), ou `file:<dir>` pour les corps non tronqués sur disque avec un pointeur `body_ref` dans l'événement |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | Limite de contenu : la longueur maximale des attributs porteurs de contenu tels que les réponses du modèle, le contenu d'outil, les invites système et les corps API bruts, marqueur de troncature inclus, en unités de code UTF-16 (par défaut : 61440, c'est-à-dire 60 Ko). La valeur par défaut est dimensionnée pour les backends qui limitent les valeurs d'attribut à 64 Ko ; augmentez-la uniquement si votre backend accepte des valeurs plus grandes, ou diminuez-la pour réduire le volume de télémétrie. Lorsqu'une limite d'attribut du SDK OpenTelemetry, `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` ou l'une de ses variantes logrecord et span, est définie plus bas, Claude Code tronque à cette valeur plus petite afin que le marqueur `[TRUNCATED ...]` reste dans la limite du SDK. Nécessite Claude Code v2.1.214 ou version ultérieure | `262144`                                                                                                                                                                            |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Préférence de temporalité des métriques (par défaut : `delta`). Définissez sur `cumulative` si votre backend attend une temporalité cumulative                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `delta`, `cumulative`                                                                                                                                                               |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalle d'actualisation des en-têtes dynamiques (par défaut : 1740000ms / 29 minutes)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `900000`                                                                                                                                                                            |

Pour les protocoles `http/protobuf` et `http/json`, Claude Code envoie chaque demande d'export avec un en-tête `Content-Length`. Avant v2.1.212, les versions de Claude Code à partir de v2.1.191 envoyaient ces demandes avec un codage de transfert fragmenté ; Azure Monitor et d'autres points de terminaison qui nécessitent une longueur déclarée les rejetaient avec des erreurs `411 Length Required` ou `400`.

<h3 id="mtls-authentication">
  Authentification mTLS
</h3>

La façon dont vous configurez les certificats clients pour l'exportateur OTLP dépend du protocole OTLP utilisé pour ce signal, défini via `OTEL_EXPORTER_OTLP_PROTOCOL` ou le remplacement par signal. La même configuration s'applique aux métriques, journaux et traces.

| Protocole                    | Variables de certificat client                                                                                                                                                                              | Faire confiance au CA du collecteur avec |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, et optionnellement `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Voir [Configuration réseau](/docs/fr/network-config#mtls-authentication)                            | `NODE_EXTRA_CA_CERTS`                    |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` et `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, ou les variantes par signal telles que `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` pour utiliser un certificat différent par signal | `OTEL_EXPORTER_OTLP_CERTIFICATE`         |

Pour `grpc`, le SDK OpenTelemetry lit les variables OTLP standard directement, donc les configurations existantes qui définissent les variables de métriques par signal continuent de fonctionner. Sur les machines avec des paramètres gérés, Claude Code [peut supprimer les identifiants et points de terminaison par signal définis par le développeur](#how-managed-settings-lock-the-otlp-destination) au démarrage.

<h3 id="metrics-cardinality-control">
  Contrôle de la cardinalité des métriques
</h3>

Les variables d'environnement suivantes contrôlent les attributs inclus dans les métriques pour gérer la cardinalité :

| Variable d'environnement                   | Description                                                                                                                                                             | Valeur par défaut | Exemple pour désactiver |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- | ----------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Inclure l'attribut session.id dans les métriques                                                                                                                        | `true`            | `false`                 |
| `OTEL_METRICS_INCLUDE_VERSION`             | Inclure l'attribut app.version dans les métriques                                                                                                                       | `false`           | `true`                  |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Inclure les attributs user.account\_uuid et user.account\_id dans les métriques                                                                                         | `true`            | `false`                 |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Inclure l'attribut app.entrypoint dans les métriques                                                                                                                    | `false`           | `true`                  |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Inclure les clés de `OTEL_RESOURCE_ATTRIBUTES` comme attributs sur les points de données de métriques                                                                   | `true`            | `false`                 |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | Inclure les [attributs d'identité de référentiel](#repository-attributes) `vcs.*` sur les métriques et événements. Nécessite Claude Code v2.1.269 ou version ultérieure | `false`           | `true`                  |

Une cardinalité plus faible signifie généralement de meilleures performances et des coûts de stockage plus bas, mais des données moins granulaires pour l'analyse.

<h3 id="traces-beta">
  Traces (bêta)
</h3>

Le traçage distribué exporte des intervalles qui lient chaque invite utilisateur aux demandes d'API et aux exécutions d'outils qu'elle déclenche, afin que vous puissiez afficher une demande complète sous forme de trace unique dans votre backend de traçage.

Le traçage est désactivé par défaut. Pour l'activer, définissez à la fois `CLAUDE_CODE_ENABLE_TELEMETRY=1` et `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, puis définissez `OTEL_TRACES_EXPORTER` pour choisir où les intervalles sont envoyés. Les traces réutilisent la [configuration OTLP courante](#common-configuration-variables) pour le point de terminaison, le protocole, les en-têtes et [mTLS](#mtls-authentication). Sur les machines avec des paramètres gérés, Claude Code [peut supprimer les identifiants et points de terminaison par signal définis par le développeur](#how-managed-settings-lock-the-otlp-destination) au démarrage.

| Variable d'environnement              | Description                                                                                           | Exemples de valeurs                  |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Activer le traçage d'intervalle (obligatoire). `ENABLE_ENHANCED_TELEMETRY_BETA` est également accepté | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Types d'exportateur de traces, séparés par des virgules. Utilisez `none` pour désactiver              | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocole pour les traces, remplace `OTEL_EXPORTER_OTLP_PROTOCOL`                                     | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Point de terminaison des traces OTLP, remplace `OTEL_EXPORTER_OTLP_ENDPOINT`                          | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | En-têtes d'authentification pour les traces, fusionnés avec `OTEL_EXPORTER_OTLP_HEADERS`              | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalle d'export par lot d'intervalles en millisecondes (par défaut : 5000)                        | `1000`, `10000`                      |

Les intervalles masquent le texte de l'invite utilisateur, les détails d'entrée d'outil et le contenu d'outil par défaut. Définissez `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` et `OTEL_LOG_TOOL_CONTENT=1` pour les inclure.

Lorsque le traçage est actif, les sous-processus Bash et PowerShell héritent automatiquement d'une variable d'environnement `TRACEPARENT` contenant le contexte de trace W3C de l'intervalle d'exécution d'outil actif. Cela permet à tout sous-processus qui lit `TRACEPARENT` de placer ses propres intervalles sous la même trace, permettant le traçage distribué de bout en bout via les scripts et les commandes que Claude exécute.

Lorsque le traçage est actif et que Claude Code est connecté directement à l'API Anthropic, chaque demande de modèle porte un en-tête W3C `traceparent` défini au contexte de l'intervalle `claude_code.llm_request`, et l'en-tête `traceresponse` de l'API est enregistré comme un lien d'intervalle. Ensemble, ceux-ci connectent les intervalles côté client de Claude Code à la trace côté serveur via tout intermédiaire conforme. Les demandes HTTP MCP sortantes portent `traceparent` de la même manière. L'en-tête n'est pas envoyé aux fournisseurs tiers.

Par défaut, l'en-tête `traceparent` sur les demandes de modèle et MCP HTTP est envoyé uniquement lorsque `ANTHROPIC_BASE_URL` n'est pas défini ou pointe vers l'API Anthropic, car certains proxies rejettent les en-têtes non reconnus. La variable `TRACEPARENT` du sous-processus est contrôlée par le même commutateur pour la cohérence. Si vous exécutez Claude Code via un proxy `ANTHROPIC_BASE_URL` personnalisé et souhaitez que le contexte de trace soit propagé, définissez `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

Dans le SDK Agent et les sessions non interactives démarrées avec `-p`, Claude Code lit également `TRACEPARENT` et `TRACESTATE` de son propre environnement au démarrage de chaque intervalle d'interaction. Cela permet à un processus d'intégration de transmettre son contexte de trace W3C actif au sous-processus afin que les intervalles de Claude Code apparaissent comme des enfants de la trace distribuée de l'appelant. Les sessions interactives ignorent `TRACEPARENT` entrant pour éviter d'hériter accidentellement des valeurs ambiantes des environnements CI ou conteneur.

Le contexte de trace entrant s'applique également aux [événements](#events). Dans les sessions du SDK Agent et `-p` avec `TRACEPARENT` défini, chaque enregistrement de journal d'événement OTLP porte les valeurs `trace_id` et `span_id` qui le joignent à la trace de votre application, même lorsque l'exportateur de traces n'est pas configuré, afin que votre backend de journalisation puisse corréler les événements avec le reste de la trace.

Un enregistrement émis pendant qu'une interaction est active porte les ID de l'intervalle d'interaction, même lorsque Claude Code l'émet en dehors du contexte asynchrone de l'intervalle, tel que dans un rappel d'invite de permission ou pour un enregistrement mis en mémoire tampon au démarrage et exporté ultérieurement. Un enregistrement émis sans intervalle d'interaction actif porte directement les ID `TRACEPARENT` entrants. Avant v2.1.214, les enregistrements émis en dehors du contexte asynchrone de l'intervalle portaient les ID `TRACEPARENT` entrants à la place des ID de l'intervalle. Avant v2.1.212, les enregistrements d'événements émis en dehors d'un intervalle actif ne portaient pas `trace_id` ou `span_id`.

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

Lorsqu'un hook `PreToolUse` [reporte un appel d'outil](/docs/fr/hooks#defer-a-tool-call-for-later), Claude Code enregistre le contexte de trace du tour qui l'a reporté. Lorsque vous reprenez la session et que l'outil s'exécute à nouveau, les intervalles de l'outil rejoignent la trace de ce tour antérieur en tant qu'enfants de l'intervalle `claude_code.interaction` du tour.

<h4 id="span-attributes">
  Attributs des intervalles
</h4>

Chaque intervalle porte les [attributs standard](#standard-attributes) plus un attribut `span.type` correspondant à son nom. Les tableaux ci-dessous listent les attributs supplémentaires définis sur chaque intervalle. Les intervalles `llm_request`, `tool.execution` et `hook` définissent le statut OpenTelemetry `ERROR` lorsqu'ils enregistrent un échec ; les autres intervalles se terminent toujours avec le statut `UNSET`.

**`claude_code.interaction`**

| Attribut                  | Description                                                                                                                                                                                                      | Contrôlé par            |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Texte de l'invite. La valeur est `<REDACTED>` sauf si la porte est définie                                                                                                                                       | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Longueur de l'invite en caractères                                                                                                                                                                               |                         |
| `interaction.sequence`    | Compteur basé sur 1 des interactions, compté par processus Claude Code plutôt que par session, comme décrit pour [`event.sequence`](#event-correlation-attributes)                                               |                         |
| `parent.source`           | Comment l'intervalle a obtenu son parent de trace : `env` lorsqu'il a été parent sous un `TRACEPARENT` entrant, `none` lorsqu'il a démarré sa propre trace. Nécessite Claude Code v2.1.268 ou version ultérieure |                         |
| `interaction.duration_ms` | Durée murale du tour                                                                                                                                                                                             |                         |

**`claude_code.llm_request`**

| Attribut                         | Description                                                                                                                                                                                                                                                                                                             | Contrôlé par                   |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | Identifiant du modèle                                                                                                                                                                                                                                                                                                   |                                |
| `gen_ai.system`                  | Toujours `anthropic`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                                                                         |                                |
| `gen_ai.request.model`           | Même valeur que `model`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                                                                      |                                |
| `query_source`                   | Sous-système qui a émis la demande, tel que `repl_main_thread` ou un nom de sous-agent                                                                                                                                                                                                                                  | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | Forme bornée de `query_source`, émise que le traçage bêta détaillé soit actif ou non, avec des valeurs telles que `repl_main_thread` ou `agent.builtin.general-purpose`. `:` devient `.` et les agents nommés par l'utilisateur apparaissent comme `agent.custom`. Nécessite Claude Code v2.1.268 ou version ultérieure |                                |
| `agent_id`                       | Identifiant du sous-agent ou du coéquipier qui a émis la demande. Absent dans la session principale                                                                                                                                                                                                                     |                                |
| `parent_agent_id`                | Identifiant de l'agent qui a généré celui-ci. Absent pour la session principale et pour les agents générés directement à partir de celle-ci                                                                                                                                                                             |                                |
| `workflow.run_id`                | Identifiant d'exécution de l'exécution de l'outil [Workflow](/docs/fr/workflows), préfixé `wf_`. Absent pour les agents non générés par un workflow                                                                                                                                                                          |                                |
| `workflow.name`                  | Nom du workflow qui a généré cet agent. Les noms créés par l'utilisateur sont remplacés par `custom` sauf si la porte est définie                                                                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` ou `normal`                                                                                                                                                                                                                                                                                                      |                                |
| `effort`                         | [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande : `low`, `medium`, `high`, `xhigh`, ou `max`. Absent lorsque Claude Code n'envoie pas de niveau d'effort, par exemple sur un modèle qui ne supporte pas l'effort. Nécessite Claude Code v2.1.274 ou version ultérieure                    |                                |
| `llm_request.context`            | `interaction`, `tool`, ou `standalone` selon l'intervalle parent                                                                                                                                                                                                                                                        |                                |
| `duration_ms`                    | Durée murale incluant les tentatives                                                                                                                                                                                                                                                                                    |                                |
| `ttft_ms`                        | Temps jusqu'au premier jeton en millisecondes                                                                                                                                                                                                                                                                           |                                |
| `first_content_ms`               | Temps du début de la demande au premier bloc de contenu de la tentative réussie, en millisecondes. Absent sur les demandes qui sont revenues au chemin non-streaming. Nécessite Claude Code v2.1.268 ou version ultérieure                                                                                              |                                |
| `input_tokens`                   | Nombre de jetons d'entrée du bloc d'utilisation de l'API                                                                                                                                                                                                                                                                |                                |
| `output_tokens`                  | Nombre de jetons de sortie                                                                                                                                                                                                                                                                                              |                                |
| `cache_read_tokens`              | Jetons lus à partir du cache de prompt                                                                                                                                                                                                                                                                                  |                                |
| `cache_creation_tokens`          | Jetons écrits dans le cache de prompt                                                                                                                                                                                                                                                                                   |                                |
| `request_id`                     | ID de demande d'API Anthropic de l'en-tête de réponse `request-id`                                                                                                                                                                                                                                                      |                                |
| `gen_ai.response.id`             | Même valeur que `request_id`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                                                                 |                                |
| `client_request_id`              | `x-client-request-id` généré par le client de la tentative finale                                                                                                                                                                                                                                                       |                                |
| `attempt`                        | Nombre total de tentatives effectuées pour cette demande                                                                                                                                                                                                                                                                |                                |
| `success`                        | `true` ou `false`                                                                                                                                                                                                                                                                                                       |                                |
| `status_code`                    | Code de statut HTTP lorsque la demande a échoué                                                                                                                                                                                                                                                                         |                                |
| `error`                          | Message d'erreur lorsque la demande a échoué                                                                                                                                                                                                                                                                            |                                |
| `error_class`                    | Jeton de classe d'erreur court lorsque la demande a échoué, tel que `api_timeout` ou `server_overload`. Nécessite Claude Code v2.1.268 ou version ultérieure                                                                                                                                                            |                                |
| `response.has_tool_call`         | `true` lorsque la réponse contenait des blocs tool-use                                                                                                                                                                                                                                                                  |                                |
| `stop_reason`                    | `stop_reason` de la réponse API, tel que `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, ou `refusal`                                                                                                                                                                                              |                                |
| `gen_ai.response.finish_reasons` | Même valeur que `stop_reason`, enveloppée dans un tableau de chaînes. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                         |                                |

Chaque tentative de nouvelle tentative est également enregistrée comme un événement d'intervalle `gen_ai.request.attempt` avec les attributs `attempt` et `client_request_id`.

**`claude_code.tool`**

| Attribut              | Description                                                                                                                                                                                                                                                                                                                                                            | Contrôlé par            |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nom de l'outil                                                                                                                                                                                                                                                                                                                                                         |                         |
| `tool_name_safe`      | Forme de `tool_name` qui ne porte aucun nom choisi par l'utilisateur. Les noms d'outils intégrés passent verbatim. Les noms d'outils MCP apparaissent comme `mcp_other`, sauf les noms d'outils correspondant à quelques formes fixes, tels que les outils `playwright` nommés `browser_*`, qui passent verbatim. Nécessite Claude Code v2.1.268 ou version ultérieure |                         |
| `bash_command_class`  | Pour l'outil Bash : catégorie du premier programme de la commande à partir d'une liste fixe, telle que `vcs` ou `package_manager`. `other` pour un programme en dehors de la liste, `unparsed` lorsque la ligne ne peut pas être analysée. Nécessite Claude Code v2.1.268 ou version ultérieure                                                                        |                         |
| `bash_argv0`          | Pour l'outil Bash : le premier programme de la commande lorsqu'il est sur la même liste fixe, tel que `git` ou `npm`. `other` pour tout programme en dehors de la liste. Nécessite Claude Code v2.1.268 ou version ultérieure                                                                                                                                          |                         |
| `duration_ms`         | Durée murale incluant l'attente de permission et l'exécution                                                                                                                                                                                                                                                                                                           |                         |
| `result_tokens`       | Taille approximative en jetons du résultat de l'outil                                                                                                                                                                                                                                                                                                                  |                         |
| `agent_id`            | Identifiant du sous-agent ou du coéquipier qui a exécuté l'outil. Absent dans la session principale                                                                                                                                                                                                                                                                    |                         |
| `parent_agent_id`     | Identifiant de l'agent qui a généré celui-ci. Absent pour la session principale et pour les agents générés directement à partir de celle-ci                                                                                                                                                                                                                            |                         |
| `workflow.run_id`     | Identifiant d'exécution de l'exécution de l'outil Workflow qui a généré cet agent, préfixé `wf_`. Absent pour les agents non générés par un workflow                                                                                                                                                                                                                   |                         |
| `workflow.name`       | Nom du workflow qui a généré cet agent. Les noms créés par l'utilisateur sont remplacés par `custom` sauf si la porte est définie                                                                                                                                                                                                                                      | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | L'ID du bloc `tool_use` du modèle pour cet appel. Correspond au `tool_use_id` sur les événements [tool\_result](#tool-result-event) et [tool\_decision](#tool-decision-event) et dans les charges utiles de hooks, afin que vous puissiez joindre l'intervalle à ces enregistrements                                                                                   |                         |
| `gen_ai.tool.call.id` | Même valeur que `tool_use_id`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                                                                                                               |                         |
| `file_path`           | Chemin de fichier cible pour les outils Read, Edit et Write                                                                                                                                                                                                                                                                                                            | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Chaîne de commande pour l'outil Bash                                                                                                                                                                                                                                                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nom de la compétence pour l'outil Skill                                                                                                                                                                                                                                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Type de sous-agent pour l'outil Agent ou l'outil Task hérité                                                                                                                                                                                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**Événement d'intervalle `tool.output` sur `claude_code.tool`**

Si vous définissez `OTEL_LOG_TOOL_CONTENT=1`, les appels Read et Bash peuvent enregistrer un événement d'intervalle `tool.output` sur l'intervalle `claude_code.tool`. Les appels Edit et Write enregistrent un événement uniquement lorsque vous définissez également `OTEL_LOG_TOOL_DETAILS=1`. Cette variable n'est pas limitée à ces deux outils, donc vérifiez sa [ligne dans le tableau de configuration](#common-configuration-variables) pour les arguments qu'elle ajoute ailleurs.

Claude Code écrit cet événement à partir du retour réussi d'un appel d'outil, donc un appel qui lève une erreur n'enregistre rien, quel que soit l'outil. Parmi les appels qui retournent, il n'enregistre pas d'événement `tool.output` pour :

* Un appel à tout outil autre que Read, Edit, Write et Bash, y compris les outils MCP et WebFetch
* Un Read qui retourne autre chose que du texte de fichier, tel qu'une image, un PDF ou une relecture d'un fichier dont le contenu n'a pas changé
* Un appel Edit ou Write, sauf si vous définissez également `OTEL_LOG_TOOL_DETAILS=1`

L'événement porte ces attributs, chacun tronqué à la limite de contenu (60 Ko par défaut). `Contrôlé par` nomme la variable qu'un attribut nécessite en plus de `OTEL_LOG_TOOL_CONTENT=1`, et pour Edit et Write cette variable contrôle l'événement lui-même plutôt que l'attribut.

| Attribut       | Description                                                                                               | Contrôlé par                               |
| -------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| `content`      | Texte que l'outil Read a retourné, ou le texte qu'un appel Write a été demandé d'écrire                   | `OTEL_LOG_TOOL_DETAILS` pour l'outil Write |
| `output`       | Sortie combinée d'une commande Bash, avec stderr entrelacé dans stdout                                    |                                            |
| `diff`         | Correctif structuré que l'outil Edit a appliqué                                                           | `OTEL_LOG_TOOL_DETAILS`                    |
| `file_path`    | Chemin de fichier cible pour les outils Read, Edit et Write, répétant l'attribut d'intervalle du même nom | `OTEL_LOG_TOOL_DETAILS`                    |
| `bash_command` | Chaîne de commande pour l'outil Bash                                                                      | `OTEL_LOG_TOOL_DETAILS`                    |

L'attribut `tool_name` de l'intervalle parent vous indique quel outil un événement provient. Un attribut coupé à la limite de contenu est accompagné de `<attribute>_truncated` et `<attribute>_original_length`.

**`claude_code.tool.blocked_on_user`**

| Attribut      | Description                                                                                    | Contrôlé par |
| ------------- | ---------------------------------------------------------------------------------------------- | ------------ |
| `duration_ms` | Temps passé à attendre la décision de permission                                               |              |
| `decision`    | `accept` ou `reject`                                                                           |              |
| `source`      | Source de la décision, correspondant à l'événement [Tool decision event](#tool-decision-event) |              |

**`claude_code.tool.execution`**

| Attribut              | Description                                                                                                                                                                                                                                                                                                              | Contrôlé par            |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `duration_ms`         | Temps passé à exécuter le corps de l'outil                                                                                                                                                                                                                                                                               |                         |
| `tool_use_id`         | Même valeur que sur l'intervalle parent `claude_code.tool`                                                                                                                                                                                                                                                               |                         |
| `gen_ai.tool.call.id` | Même valeur que `tool_use_id`. Convention sémantique GenAI OpenTelemetry                                                                                                                                                                                                                                                 |                         |
| `success`             | `true` ou `false`                                                                                                                                                                                                                                                                                                        |                         |
| `error`               | Chaîne de catégorie d'erreur lorsque l'exécution a échoué, telle que `Error:ENOENT` ou `ShellError`. Contient le message d'erreur complet à la place lorsque la porte est définie                                                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | La catégorie d'erreur sous forme d'identifiant, avec les caractères en dehors des lettres, des chiffres et des traits de soulignement remplacés par `_`, tels que `Error_ENOENT` ou `ShellError`. Porte la catégorie même lorsque `error` porte le message complet. Nécessite Claude Code v2.1.268 ou version ultérieure |                         |

**`claude_code.hook`**

Cet intervalle n'apparaît que lorsque le traçage bêta détaillé est actif, ce qui nécessite `ENABLE_BETA_TRACING_DETAILED=1` et `BETA_TRACING_ENDPOINT`, une paire qui [change également où vos journaux et traces vont](/docs/fr/env-vars#variables). Définissez la paire dans votre shell, vos paramètres utilisateur ou vos paramètres gérés ; les deux variables sont ignorées dans les [paramètres de projet et locaux](/docs/fr/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` seul ne le produit pas.

Dans les sessions CLI interactives, le traçage bêta détaillé nécessite également que votre organisation soit sur liste blanche pour la fonctionnalité. Les sessions du SDK Agent et non interactives `-p` ne nécessitent pas de liste blanche.

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

<span id="new-context-gates" />

<Note>
  Les attributs supplémentaires porteurs de contenu tels que `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` et `response.model_output` sont émis uniquement lorsque le traçage bêta détaillé est actif. Ils ne font pas partie du schéma d'intervalle stable.

  La porte sur `new_context` dépend de l'intervalle qui le porte, et chaque copie est tronquée à la limite de contenu (60 Ko par défaut). Sur l'intervalle `claude_code.tool` il porte le résultat de cet appel d'outil, quel que soit l'outil, et nécessite `OTEL_LOG_TOOL_CONTENT=1`. Sur l'intervalle `claude_code.interaction` il porte l'invite utilisateur, et sur l'intervalle `claude_code.llm_request` les nouveaux messages utilisateur et résultats d'outils de cette demande. Les deux nécessitent `OTEL_LOG_USER_PROMPTS=1`.

  `user_system_prompt` nécessite également `OTEL_LOG_USER_PROMPTS=1`. Il porte uniquement le texte du prompt système que vous fournissez via l'option SDK `systemPrompt` ou les drapeaux `--system-prompt` et `--append-system-prompt`, tronqué à la limite de contenu (60 Ko par défaut), et est émis une fois par session plutôt que par demande.
</Note>

<h3 id="dynamic-headers">
  En-têtes dynamiques
</h3>

Pour les environnements d'entreprise qui nécessitent une authentification dynamique, vous pouvez configurer un script pour générer des en-têtes dynamiquement. Les en-têtes dynamiques s'appliquent uniquement aux protocoles `http/protobuf` et `http/json`. Avec le protocole `grpc`, Claude Code utilise uniquement les variables d'en-têtes statiques, `OTEL_EXPORTER_OTLP_HEADERS` et ses variantes par signal.

<h4 id="settings-configuration">
  Configuration des paramètres
</h4>

Ajoutez à votre `.claude/settings.json`, en remplaçant le chemin par votre propre script :

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

Si le script d'aide échoue ou imprime une sortie qui ne répond pas à ces exigences, les exports échouent et votre backend de télémétrie ne reçoit rien de la session jusqu'à ce que le script fonctionne à nouveau. Claude Code signale l'erreur dans :

* Une notification d'avertissement dans les sessions interactives, [`otelHeadersHelper failed; telemetry is not being exported`](/docs/fr/errors#otelheadershelper-failed), affichée une fois par session lorsque le script d'aide échoue pour la première fois
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

Claude Code attache ces valeurs comme attributs sur chaque point de données de métriques et enregistrement d'événement, en plus de les envoyer dans le bloc de ressources OTLP. Parce que la plupart des backends de métriques exposent les attributs de point de données comme des étiquettes interrogeables, vous pouvez regrouper et filtrer les métriques par vos clés personnalisées directement. Sauf pour les [attributs de référentiel](#repository-attributes) `vcs.*`, les clés personnalisées ne remplacent jamais les [attributs standard](#standard-attributes) tels que `user.id` ou `session.id` : lorsqu'une clé entre en collision, Claude Code conserve la valeur intégrée.

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

Définissez ces variables d'environnement avant d'exécuter `claude`. Chaque scénario ci-dessous montre une configuration complète, et chaque variable est décrite sous [Variables de configuration courantes](#common-configuration-variables). Pour confirmer qu'une configuration a pris effet, vérifiez votre backend pour la métrique `claude_code.session.count` après le démarrage d'une session ; le [Démarrage rapide](#quick-start) couvre la vérification en journaux uniquement et ce qu'il faut vérifier quand rien n'arrive.

Pour le débogage de console avec un intervalle d'export d'1 seconde :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

Pour OTLP sur gRPC :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Pour Prometheus, récupéré depuis `http://localhost:9464/metrics` :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

Sur un [environnement auto-hébergé](/docs/fr/self-hosted-environments-reference#pass-through-session-child-metrics), la session lie le port 9464 uniquement à la capacité par défaut du runner d'un. À une capacité plus élevée, le runner réexpose les compteurs et jauges de session sur son propre point de terminaison `/metrics` à la place.

Pour envoyer des métriques à plusieurs exportateurs :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

Pour envoyer des métriques et des journaux à différents points de terminaison ou backends :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

Pour exporter les métriques uniquement, sans événements ni journaux :

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Pour exporter les événements et journaux uniquement, sans métriques :

```bash theme={null}
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

| Attribut                                                                                | Description                                                                                                                                                                                                                                                                            | Contrôlé par                                                                                        |
| --------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `session.id`                                                                            | Identifiant de session unique                                                                                                                                                                                                                                                          | `OTEL_METRICS_INCLUDE_SESSION_ID` (par défaut : true)                                               |
| `app.version`                                                                           | Version actuelle de Claude Code                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_VERSION` (par défaut : false)                                                 |
| `app.entrypoint`                                                                        | Comment la session a été lancée, par exemple `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, ou `claude-vscode`                                                                                                                                                                                  | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (par défaut : false)                                              |
| `organization.id`                                                                       | UUID de l'organisation (si authentifié)                                                                                                                                                                                                                                                | Toujours inclus quand disponible                                                                    |
| `user.account_uuid`                                                                     | UUID du compte (si authentifié)                                                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (par défaut : true)                                             |
| `user.account_id`                                                                       | ID du compte au format balisé correspondant aux API d'administration Anthropic (si authentifié), par exemple `user_01BWBeN28...`                                                                                                                                                       | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (par défaut : true)                                             |
| `user.id`                                                                               | Identifiant anonyme aléatoire généré à la première exécution et persisté dans `~/.claude.json`. Il ne contient aucune information personnelle et n'est pas dérivé de votre compte Claude. La suppression du fichier produit une nouvelle valeur sans rapport à la prochaine exécution. | Toujours inclus                                                                                     |
| `user.email`                                                                            | Adresse e-mail de l'utilisateur, de votre connexion ou, dans une [session cloud](/docs/fr/claude-code-on-the-web), des identifiants de la session elle-même                                                                                                                                 | Toujours inclus quand disponible                                                                    |
| `terminal.type`                                                                         | Type de terminal, par exemple `iTerm.app`, `vscode`, `cursor`, ou `tmux`                                                                                                                                                                                                               | Toujours inclus quand détecté                                                                       |
| Clés de `OTEL_RESOURCE_ATTRIBUTES`                                                      | Attributs personnalisés que vous définissez, par exemple `department` ou `team.id`. Voir [Support des organisations multi-équipes](#multi-team-organization-support)                                                                                                                   | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (par défaut : true)                                      |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | L'identité du référentiel de la session, dérivée de sa télécommande `origin`. Voir [Attributs du référentiel](#repository-attributes)                                                                                                                                                  | `OTEL_METRICS_INCLUDE_REPOSITORY` (par défaut : false). Nécessite Claude Code v2.1.269 ou ultérieur |

Quand Claude Code est connecté à une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), l'interface de ligne de commande marque les exportations avec l'identité authentifiée de la session de la passerelle : `user.id` est le sujet IdP plutôt qu'un identifiant d'installation anonyme, `user.email` est l'e-mail connecté, et `user.groups` porte l'appartenance au groupe IdP sous forme de chaîne séparée par des virgules. Chaque exportation porte également `identity.source: gateway-oidc`. L'identité de la passerelle est appliquée en dernier, donc les clés `user.*` et `identity.*` définies via `OTEL_RESOURCE_ATTRIBUTES` sont ignorées sur les sessions de passerelle.

Les événements incluent en outre les attributs suivants. Ceux-ci ne sont jamais attachés aux métriques car ils causeraient une cardinalité non bornée :

* `prompt.id` : UUID corrélant une invite utilisateur avec tous les événements suivants jusqu'à l'invite suivante. Voir [Attributs de corrélation d'événements](#event-correlation-attributes).
* `workspace.host_paths` : répertoires d'espace de travail hôte sélectionnés dans l'application de bureau, sous forme de tableau de chaînes
* `workflow.run_id` : identifiant d'exécution, préfixé `wf_`, sur les événements API et d'outils émis par les agents qui appartiennent à une exécution d'outil [Workflow](/docs/fr/workflows). Le filtrage des événements par un `workflow.run_id` reconstruit les demandes API et les résultats d'outils de cette exécution. L'identifiant couvre les agents que le script de flux de travail génère et tous les agents que ceux-ci génèrent à leur tour, par exemple les invocations de compétences. Il correspond à l'identifiant d'exécution signalé dans le résultat de l'outil Workflow. Absent sur tous les autres événements. Nécessite Claude Code v2.1.202 ou ultérieur
* `workflow.name` : nom du flux de travail, le `meta.name` de son script, émis aux côtés de `workflow.run_id`. Les noms de flux de travail intégrés apparaissent textuellement quand l'exécution exécute le script intégré non modifié. Les noms créés par l'utilisateur, y compris les copies modifiées de scripts intégrés, sont remplacés par `custom` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Nécessite Claude Code v2.1.202 ou ultérieur

<h4 id="repository-attributes">
  Attributs du référentiel
</h4>

Définissez `OTEL_METRICS_INCLUDE_REPOSITORY=true` pour balisez les métriques et les événements avec l'identité du référentiel de la session, afin qu'un collecteur partagé puisse attribuer l'utilisation par référentiel. Nécessite Claude Code v2.1.269 ou ultérieur.

Claude Code dérive ces attributs une fois par session à partir de la télécommande `origin` du référentiel. Les télécommandes HTTPS et SSH d'un référentiel produisent des valeurs identiques :

| Attribut                  | Valeur                                                                                                                                                             |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `vcs.repository.url.full` | L'URL du navigateur du référentiel sans `.git`, par exemple `https://github.com/example-org/example-repo`                                                          |
| `vcs.owner.name`          | Le chemin du propriétaire ou du groupe, par exemple `example-org` ; omis quand le chemin de la télécommande a un seul segment                                      |
| `vcs.repository.name`     | Le nom du référentiel nu, par exemple `example-repo`                                                                                                               |
| `vcs.provider.name`       | `github`, `gitlab`, `bitbucket`, ou `gitea` quand Claude Code reconnaît l'hôte de la télécommande ou la forme de l'URL comme l'un de ces fournisseurs ; omis sinon |

Les valeurs sont en minuscules, et les identifiants, chaînes de requête et fragments de l'URL de la télécommande n'apparaissent jamais en eux. Les attributs sont omis quand la session n'a pas de télécommande `origin`, quand la télécommande n'a pas la forme d'une URL, ou quand le seul référentiel englobant est votre répertoire personnel.

Une clé `vcs.*` que vous déclarez dans [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) remplace la valeur dérivée pour cette clé. Si vous déclarez `vcs.repository.url.full`, Claude Code ne lit jamais la télécommande et signale uniquement les clés que vous déclarez.

Les attributs ne circulent que vers vos propres exportateurs ; la télémétrie d'Anthropic supprime chaque clé `vcs.*`.

<h3 id="metrics">
  Métriques
</h3>

Claude Code exporte les métriques suivantes. La colonne Unité affiche la chaîne d'unité OpenTelemetry attachée à chaque métrique ; les métriques de comptage n'en portent aucune.

| Nom de la métrique                    | Description                                                    | Unité  |
| ------------------------------------- | -------------------------------------------------------------- | ------ |
| `claude_code.session.count`           | Nombre de sessions CLI démarrées                               | aucune |
| `claude_code.lines_of_code.count`     | Nombre de lignes de code modifiées                             | aucune |
| `claude_code.pull_request.count`      | Nombre de demandes de tirage créées                            | aucune |
| `claude_code.commit.count`            | Nombre de commits git créés                                    | aucune |
| `claude_code.cost.usage`              | Coût de la session Claude Code                                 | USD    |
| `claude_code.token.usage`             | Nombre de jetons utilisés                                      | tokens |
| `claude_code.code_edit_tool.decision` | Nombre de décisions de permission de l'outil d'édition de code | aucune |
| `claude_code.active_time.total`       | Temps actif total                                              | s      |

Quand `prometheus` est le seul exportateur listé dans `OTEL_METRICS_EXPORTER`, Claude Code omet les unités `USD`, `tokens`, et `s` des métriques exportées afin que le scrape reste au format texte Prometheus valide. Les noms de métriques ne changent pas, et les configurations qui combinent des exportateurs, par exemple `otlp,prometheus`, conservent les unités. Avant v2.1.216, le scrape Prometheus incluait des lignes `# UNIT` uniquement OpenMetrics que certains scrapers rejetaient.

<h3 id="metric-details">
  Détails des métriques
</h3>

Chaque métrique inclut les attributs standard listés ci-dessus. Les métriques avec des attributs supplémentaires spécifiques au contexte sont notées ci-dessous.

<h4 id="session-counter">
  Compteur de session
</h4>

Incrémenté au début de chaque session.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `start_type` : Comment la session a été démarrée. L'une de `"fresh"`, `"resume"`, `"continue"`, ou `"agents_view"`. La valeur `"agents_view"` identifie le processus du tableau de bord `claude agents`, une interface utilisateur locale lancée par l'utilisateur plutôt qu'une session conversationnelle. Filtrez sur cette valeur pour séparer les lancements de processus d'interface utilisateur des sessions conversationnelles dans vos tableaux de bord.

<h4 id="lines-of-code-counter">
  Compteur de lignes de code
</h4>

Incrémenté quand du code est ajouté ou supprimé.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : (`"added"`, `"removed"`)
* `model` : Identifiant du modèle pour le modèle qui a effectué la modification (par exemple, « claude-sonnet-5 »)

<h4 id="pull-request-counter">
  Compteur de demande de tirage
</h4>

Incrémenté quand Claude Code crée une demande de tirage ou de fusion via une commande shell ou un outil MCP.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)

<h4 id="commit-counter">
  Compteur de commit
</h4>

Incrémenté lors de la création de commits git via Claude Code.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)

<h4 id="cost-counter">
  Compteur de coût
</h4>

Incrémenté après chaque demande API.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `query_source` : Catégorie du sous-système qui a émis la demande. L'une de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed` : `"fast"` quand la demande a utilisé le mode rapide. Absent sinon
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande : `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Absent quand Claude Code n'envoie aucun niveau d'effort, par exemple sur un modèle qui ne supporte pas l'effort.
* `agent.name` : Type de sous-agent qui a émis la demande. Les noms d'agents intégrés et les agents des plugins de la place de marché officielle apparaissent textuellement. Les autres noms d'agents définis par l'utilisateur sont remplacés par `"custom"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Absent quand la demande n'a pas été émise par un type de sous-agent nommé.
* `skill.name` : Compétence active pour la demande, définie par l'outil Skill, une commande `/`, ou héritée par un sous-agent généré. Les noms de compétences intégrés, groupés, définis par l'utilisateur et de la place de marché officielle des plugins apparaissent textuellement. Les noms de compétences des plugins tiers sont remplacés par `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Absent quand aucune compétence n'est active.
* `plugin.name` : Plugin propriétaire quand la compétence active ou le sous-agent est fourni par un plugin. Les noms de plugins de la place de marché officielle apparaissent textuellement. Les noms de plugins tiers sont remplacés par `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Absent quand ni la compétence ni le sous-agent n'a de plugin propriétaire.
* `marketplace.name` : Place de marché à partir de laquelle le plugin propriétaire a été installé. Émis uniquement pour les plugins de la place de marché officielle. Absent sinon.
* `mcp_server.name` : Serveur MCP dont le résultat de l'outil cette demande a consommé. Les noms de serveurs intégrés, proxifiés par claude.ai, et de registre officiel apparaissent textuellement. Les noms de serveurs configurés par l'utilisateur sont remplacés par `"custom"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini. Absent quand la demande n'a consommé aucun résultat d'outil MCP. Avant v2.1.222, Claude Code définissait cet attribut sur chaque demande après un appel d'outil MCP, pas seulement sur les demandes qui ont consommé un résultat d'outil, donc les tableaux de bord qui l'agrègent montrent une baisse après la mise à niveau.
* `mcp_tool.name` : Outil MCP dont le résultat cette demande a consommé, avec le même comportement de rédaction et de version que `mcp_server.name`. Absent quand la demande n'a consommé aucun résultat d'outil MCP.

<h4 id="token-counter">
  Compteur de jetons
</h4>

Incrémenté après chaque demande API.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `query_source` : Catégorie du sous-système qui a émis la demande. L'une de `"main"`, `"subagent"`, ou `"auxiliary"`
* `speed` : `"fast"` quand la demande a utilisé le mode rapide. Absent sinon
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Voir [Compteur de coût](#cost-counter) pour les détails.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coût](#cost-counter) pour les définitions et le comportement de rédaction.

<h4 id="code-edit-tool-decision-counter">
  Compteur de décision de l'outil d'édition de code
</h4>

Incrémenté quand l'utilisateur accepte ou rejette l'utilisation de l'outil Edit, Write, ou NotebookEdit.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `tool_name` : Nom de l'outil (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision` : Décision de l'utilisateur (`"accept"`, `"reject"`)
* `source` : D'où provient la décision. L'une de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, ou `"user_reject"`. Voir l'[événement de décision d'outil](#tool-decision-event) pour ce que chaque valeur signifie.
* `language` : Langage de programmation du fichier édité, par exemple `"TypeScript"`, `"Python"`, `"JavaScript"`, ou `"Markdown"`. Retourne `"unknown"` pour les extensions de fichier non reconnues.

<h4 id="active-time-counter">
  Compteur de temps actif
</h4>

Suit le temps réel passé à utiliser activement Claude Code, excluant le temps d'inactivité. Cette métrique est incrémentée lors des interactions utilisateur, par exemple la saisie et la lecture des réponses, et lors du traitement CLI, par exemple l'exécution d'outils et la génération de réponses IA.

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `type` : `"user"` pour les interactions au clavier, `"cli"` pour l'exécution d'outils et les réponses IA

<h3 id="events">
  Événements
</h3>

Claude Code exporte les événements suivants via les journaux/événements OpenTelemetry (quand `OTEL_LOGS_EXPORTER` est configuré) :

<h4 id="event-correlation-attributes">
  Attributs de corrélation d'événements
</h4>

Quand un utilisateur soumet une invite, Claude Code peut effectuer plusieurs appels API et exécuter plusieurs outils. L'attribut `prompt.id` vous permet de lier tous ces événements à l'invite unique qui les a déclenchés.

| Attribut            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id`         | Identifiant UUID v4 liant tous les événements produits lors du traitement d'une invite utilisateur unique                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `event.sequence`    | Compteur basé sur 0 pour ordonner les événements, compté par processus Claude Code plutôt que par session                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `message.uuid`      | UUID du message tel que persisté dans la transcription de session, les fichiers `~/.claude/projects/*/*.jsonl`. Présent sur `assistant_response`, sur `api_response_body`, et sur `user_prompt` sauf pour les dispatches de commande, qui peuvent produire zéro ou plusieurs messages. Sur `assistant_response` et `api_response_body`, c'est l'entrée de transcription finale de la réponse, à partir de laquelle le `parentUuid` du tour suivant s'enchaîne. Nécessite Claude Code v2.1.214 ou ultérieur, ou v2.1.274 ou ultérieur sur `api_response_body`      |
| `client_request_id` | UUID généré par le client envoyé comme en-tête de demande `x-client-request-id`. Présent sur `api_request` et `api_error` sur les connexions API de première partie ; absent sur les backends de fournisseurs tiers et quand la demande a été retentée via le secours non-streaming. Associe une demande à sa réponse et reste disponible pour les défaillances telles que les délais d'expiration qui n'ont jamais produit un `request_id` serveur. Correspond au même attribut sur la plage de trace `llm_request`. Nécessite Claude Code v2.1.214 ou ultérieur |

Pour tracer toute l'activité déclenchée par une invite unique, filtrez vos événements par une valeur `prompt.id` spécifique. Cela retourne l'événement user\_prompt, tous les événements api\_request, et tous les événements tool\_result qui se sont produits lors du traitement de cette invite.

`event.sequence` commence à 0 chaque fois qu'un processus Claude Code démarre et compte jusqu'à la fin de ce processus. Il continue de compter à travers `/clear`, qui assigne un nouveau `session.id`. Si vous [reprenez une session sans la forker](/docs/fr/how-claude-code-works#resume-or-fork-sessions), la session conserve son `session.id` mais prend ses valeurs `event.sequence` du processus qui l'a reprise, donc dans une session un événement ultérieur peut porter une valeur inférieure à celle d'un événement antérieur, ou en répéter une. Pour ordonner les événements d'une session, triez par `event.timestamp` et utilisez `event.sequence` pour ordonner les événements qui partagent un timestamp.

Pour la reconstruction au niveau du message, chaque classe d'événement porte une clé qui correspond à un champ dans la transcription de session. Le format d'entrée de transcription est [interne à Claude Code](/docs/fr/sessions#where-transcripts-are-stored) et change entre les versions, donc un pipeline qui se joint sur ces champs peut se casser à chaque version ; traitez les jointures comme spécifiques à la version plutôt que comme un contrat stable :

* `message.uuid` sur `user_prompt`, `assistant_response`, et `api_response_body`
* `request_id` sur les événements API, persisté comme `requestId` sur les entrées d'assistant de la transcription
* `tool_use_id` sur les événements `tool_result` et `tool_decision`

<h4 id="user-prompt-event">
  Événement d'invite utilisateur
</h4>

Enregistré quand un utilisateur soumet une invite.

**Nom de l'événement** : `claude_code.user_prompt`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"user_prompt"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `prompt_length` : Longueur de l'invite
* `prompt` : Contenu de l'invite. Rédacté par défaut. Définissez `OTEL_LOG_USER_PROMPTS=1` pour l'inclure
* `message.uuid` : UUID du message utilisateur résultant, correspondant à l'entrée de transcription persistée. Absent sur les dispatches de commande, qui peuvent produire zéro ou plusieurs messages. Nécessite Claude Code v2.1.214 ou ultérieur
* `command_name` : Nom de la commande quand l'invite en invoque une. Les noms de commande intégrés et groupés tels que `compact` ou `debug` sont émis tels quels ; les alias tels que `reset` émettent tels que tapés plutôt que le nom canonique. Les noms de commande personnalisés, de plugins et MCP s'effondrent en `custom` ou `mcp` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini
* `command_source` : Origine de la commande quand présente : `builtin`, `custom`, ou `mcp`. Les commandes fournies par les plugins signalent comme `custom`

<h4 id="assistant-response-event">
  Événement de réponse d'assistant
</h4>

Enregistré après chaque demande API qui retourne du contenu textuel du modèle. Seuls les blocs de texte de la réponse sont inclus ; les blocs de réflexion et les blocs d'utilisation d'outils sont exclus. Nécessite Claude Code v2.1.193 ou ultérieur.

**Nom de l'événement** : `claude_code.assistant_response`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"assistant_response"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `response_length` : Longueur du texte de réponse en caractères
* `response` : Texte de réponse, tronqué à la limite de contenu (60 Ko par défaut). Rédacté à `<REDACTED>` par défaut. Définissez `OTEL_LOG_ASSISTANT_RESPONSES=1` pour l'inclure. Quand `OTEL_LOG_ASSISTANT_RESPONSES` n'est pas défini, `OTEL_LOG_USER_PROMPTS` le contrôle à la place, donc définissez `OTEL_LOG_ASSISTANT_RESPONSES=0` pour garder les réponses rédactées tandis que la journalisation des invites est activée
* `model` : Identifiant du modèle (par exemple, « claude-sonnet-5 »)
* `request_id` : ID de demande API Anthropic de l'en-tête `request-id` de la réponse. Présent uniquement quand l'API en retourne un
* `message.uuid` : UUID de l'entrée de transcription finale de la réponse. Une réponse API est persistée comme une entrée de transcription par bloc de contenu ; c'est la dernière, à partir de laquelle le `parentUuid` du tour suivant s'enchaîne. Nécessite Claude Code v2.1.214 ou ultérieur
* `query_source` : Sous-système qui a émis la demande, par exemple `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent

<h4 id="tool-result-event">
  Événement de résultat d'outil
</h4>

Enregistré quand un outil termine son exécution. Non émis si l'appel d'outil a été rejeté ; voir l'[événement de décision d'outil](#tool-decision-event) pour les rejets.

**Nom de l'événement** : `claude_code.tool_result`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"tool_result"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `tool_name` : Nom de l'outil
* `tool_use_id` : Identifiant unique pour cette invocation d'outil. Correspond au `tool_use_id` passé aux hooks, permettant la corrélation entre les événements OTel et les données capturées par les hooks.
* `success` : `"true"` ou `"false"`
* `duration_ms` : Temps d'exécution en millisecondes
* `error_type` : Chaîne de catégorie d'erreur quand l'outil a échoué, par exemple `"Error:ENOENT"` ou `"ShellError"`
* `error` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Message d'erreur complet quand l'outil a échoué
* `decision_type` : Toujours `"accept"`, puisque cet événement n'est émis qu'après l'exécution de l'outil. Les appels rejetés ne produisent pas de résultat d'outil
* `decision_source` : D'où provient la décision de permission. L'une de `"config"`, `"hook"`, `"user_permanent"`, ou `"user_temporary"`. Voir l'[événement de décision d'outil](#tool-decision-event) pour ce que chaque valeur signifie. Les sources réservées au rejet `"user_abort"` et `"user_reject"` n'apparaissent jamais sur cet événement.
* `tool_input_size_bytes` : Taille de l'entrée d'outil sérialisée en JSON en octets
* `tool_result_size_bytes` : Taille du résultat d'outil en octets
* `mcp_server_scope` : Identifiant de portée du serveur MCP (pour les outils MCP)
* `vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (quand `OTEL_LOG_TOOL_DETAILS=1`) : l'identité du commit d'une exécution `git commit` réussie par l'outil Bash ou PowerShell. `vcs.ref.head.revision` est le SHA du commit, `vcs.ref.head.name` est la branche sur laquelle il a été commité, et `vcs.ref.head.type` est `branch`. Le nom et le type sont omis quand le commit a été effectué sur un HEAD détaché. Nécessite Claude Code v2.1.269 ou ultérieur
* `tool_parameters` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Chaîne JSON contenant les paramètres spécifiques à l'outil. Pour les serveurs intégrés de Claude Desktop, dans les sessions que Claude Desktop possède, la paire `mcp_server_name`/`mcp_tool_name` est incluse même avec le drapeau désactivé, la même exception créée par l'hôte que l'[événement de décision d'outil](#tool-decision-event), nécessitant Claude Code v2.1.214 ou ultérieur. Les paramètres varient selon l'outil :
  * Pour l'outil Bash : inclut `bash_command`, `full_command`, `timeout`, `description`, et `dangerouslyDisableSandbox`, plus `git_commit_id` et `git_branch` quand une commande `git commit` réussit. `git_commit_id` est le SHA du commit complet quand le commit est le HEAD du répertoire de travail de la session, et le SHA abrégé de git sinon. `git_branch` est la branche sur laquelle il a été commité, omis sur un HEAD détaché
  * Pour l'outil Bash d'espace de travail de l'application de bureau, qui signale également `tool_name` comme `Bash` : inclut uniquement `bash_command`, `full_command`, et `timeout`
  * Pour les outils MCP : inclut `mcp_server_name`, `mcp_tool_name`
  * Pour l'outil Skill : inclut `skill_name`
  * Pour l'outil Agent ou l'outil Task hérité : inclut `subagent_type`
* `tool_input` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Arguments d'outil sérialisés en JSON. Les valeurs individuelles sur 512 caractères sont tronquées, et la charge utile complète est bornée à environ 4 K caractères. S'applique à tous les outils, y compris les outils MCP.

<h4 id="api-request-event">
  Événement de demande API
</h4>

Enregistré pour chaque demande API à Claude.

**Nom de l'événement** : `claude_code.api_request`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_request"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `model` : Modèle utilisé (par exemple, « claude-sonnet-5 »)
* `cost_usd` : Coût estimé en USD
* `cost_usd_micros` : Coût estimé en millionièmes de dollar américain, émis comme un entier
* `duration_ms` : Durée de la demande en millisecondes
* `input_tokens` : Nombre de jetons d'entrée
* `output_tokens` : Nombre de jetons de sortie
* `cache_read_tokens` : Nombre de jetons lus du cache
* `cache_creation_tokens` : Nombre de jetons utilisés pour la création du cache
* `request_id` : ID de demande API Anthropic de l'en-tête `request-id` de la réponse, par exemple `"req_011..."`. Présent uniquement quand l'API en retourne un.
* `client_request_id` : UUID généré par le client envoyé comme en-tête de demande `x-client-request-id` ; voir le tableau [attributs de corrélation d'événements](#event-correlation-attributes) pour quand il est présent. Nécessite Claude Code v2.1.214 ou ultérieur
* `speed` : `"fast"` ou `"normal"`, indiquant si le mode rapide était actif
* `query_source` : Sous-système qui a émis la demande, par exemple `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande : `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Absent quand Claude Code n'envoie aucun niveau d'effort, par exemple sur un modèle qui ne supporte pas l'effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coût](#cost-counter) pour les définitions et le comportement de rédaction.

<h4 id="api-error-event">
  Événement d'erreur API
</h4>

Enregistré quand une demande API à Claude échoue.

**Nom de l'événement** : `claude_code.api_error`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_error"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `model` : Modèle utilisé (par exemple, « claude-sonnet-5 »)
* `error` : Message d'erreur
* `status_code` : Code de statut HTTP en tant que nombre. Absent pour les erreurs non-HTTP telles que les défaillances de connexion.
* `duration_ms` : Durée de la demande en millisecondes
* `attempt` : Nombre total de tentatives effectuées, y compris la demande initiale (`1` signifie qu'aucune nouvelle tentative ne s'est produite)
* `request_id` : ID de demande API Anthropic de l'en-tête `request-id` de la réponse, par exemple `"req_011..."`. Présent uniquement quand l'API en retourne un.
* `client_request_id` : UUID généré par le client envoyé comme en-tête de demande `x-client-request-id`. Disponible même quand une défaillance telle qu'un délai d'expiration ou une erreur de connexion n'a jamais produit un `request_id` serveur ; voir le tableau [attributs de corrélation d'événements](#event-correlation-attributes) pour quand il est présent. Nécessite Claude Code v2.1.214 ou ultérieur
* `speed` : `"fast"` ou `"normal"`, indiquant si le mode rapide était actif
* `query_source` : Sous-système qui a émis la demande, par exemple `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Absent quand Claude Code n'envoie aucun niveau d'effort, par exemple sur un modèle qui ne supporte pas l'effort.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coût](#cost-counter) pour les définitions et le comportement de rédaction.

<h4 id="api-refusal-event">
  Événement de refus API
</h4>

Enregistré quand une demande API retourne `stop_reason: "refusal"`. Les refus arrivent sur un flux de réponse réussi plutôt que comme une erreur HTTP, donc l'événement `api_error` ne se déclenche pas pour eux. Cet événement vous permet de suivre la fréquence des refus et de regrouper les refus par les mêmes attributs que `api_request` et `api_error`.

**Nom de l'événement** : `claude_code.api_refusal`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_refusal"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `model` : Identifiant du modèle de la demande
* `request_id` : ID de demande API Anthropic de l'en-tête `request-id` de la réponse, par exemple `"req_011..."`. Présent uniquement quand l'API en retourne un.
* `query_source` : Sous-système qui a émis la demande, par exemple `"repl_main_thread"`, `"compact"`, ou un nom de sous-agent. Voir [`api_request`](#api-request-event) pour les définitions.
* `speed` : Soit `"fast"` quand le [Mode rapide](/docs/fr/fast-mode) est actif, soit `"normal"`
* `attempt` : Numéro de tentative de nouvelle tentative. La première tentative est `1`.
* `effort` : [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) appliqué à la demande. Absent quand Claude Code n'envoie aucun niveau d'effort, par exemple sur un modèle qui ne supporte pas l'effort.
* `server_fallback_hop` : `true` quand le secours du modèle côté serveur de l'API a déjà retesté ce refus sur un modèle différent, donc l'utilisateur n'a pas vu ce refus particulier. `false` quand la demande s'est terminée par un refus. Un seul tour peut émettre à la fois un événement `true` hop et un événement `false` final ultérieur quand le modèle de secours refuse également.
* `has_category` : `true` quand la réponse API portait une `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. `false` quand la réponse ne portait aucune catégorie ou une valeur en dehors de cet ensemble. Absent quand `server_fallback_hop` est `true`, car les blocs hop ne portent pas `stop_details`.
* `has_explanation` : `true` quand la réponse API portait une `stop_details.explanation`, sinon `false`. Absent quand `server_fallback_hop` est `true`.
* `category` : La valeur `stop_details.category` de la réponse API. L'une de `"cyber"`, `"bio"`, `"frontier_llm"`, ou `"reasoning_extraction"`. Présent uniquement quand `OTEL_LOG_TOOL_DETAILS=1` est défini et `has_category` est `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name` : Attribution de compétence, plugin, agent et MCP pour la demande. Voir [Compteur de coût](#cost-counter) pour les définitions et le comportement de rédaction.

<h4 id="api-request-body-event">
  Événement de corps de demande API
</h4>

Enregistré pour chaque tentative de demande API quand `OTEL_LOG_RAW_API_BODIES` est défini. Un événement est émis par tentative, donc les nouvelles tentatives avec des paramètres ajustés produisent chacune leur propre événement.

**Nom de l'événement** : `claude_code.api_request_body`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_request_body"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `body` : Paramètres de demande API Messages sérialisés en JSON, par exemple l'invite système, les messages et les outils, tronqués à la limite de contenu (60 Ko par défaut). Le contenu de réflexion étendue dans les tours d'assistant antérieurs est rédacté. Émis uniquement en mode en ligne (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref` : Chemin absolu vers un fichier `<dir>/<uuid>.request.json` contenant le corps non tronqué. Émis uniquement en mode fichier (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length` : Longueur du corps non tronqué. Octets UTF-8 quand `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unités de code UTF-16 quand `=1`
* `body_truncated` : `"true"` quand la troncature en ligne s'est produite. Absent en mode fichier et quand aucune troncature ne s'est produite.
* `model` : Identifiant du modèle à partir des paramètres de demande
* `query_source` : Sous-système qui a émis la demande (par exemple, `"compact"`)
* `request_body_id` : UUID qui identifie le corps de demande de cette tentative. L'[événement `api_response_body`](#api-response-body-event) pour la tentative qui réussit porte la même valeur, donc vous pouvez associer une réponse à la demande exacte qui l'a produite. Nécessite Claude Code v2.1.274 ou ultérieur

<h4 id="api-response-body-event">
  Événement de corps de réponse API
</h4>

Enregistré pour chaque réponse API réussie quand `OTEL_LOG_RAW_API_BODIES` est défini.

En mode fichier (`OTEL_LOG_RAW_API_BODIES=file:<dir>`), Claude Code ajoute également une ligne JSON à `<dir>/index.jsonl` pour chaque réponse réussie, avec les champs `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file`, et `response_file`. Lisez-le pour trouver les fichiers de demande et de réponse derrière un message de transcription donné sans interroger votre backend de télémétrie. Le fichier d'index nécessite Claude Code v2.1.274 ou ultérieur.

**Nom de l'événement** : `claude_code.api_response_body`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_response_body"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `body` : Réponse API Messages sérialisée en JSON, y compris l'id, les blocs de contenu, l'utilisation et la raison d'arrêt, tronquée à la limite de contenu (60 Ko par défaut). Le contenu de réflexion étendue est rédacté. Émis uniquement en mode en ligne (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref` : Chemin absolu vers un fichier `<dir>/<request_id>.response.json` contenant le corps non tronqué. Émis uniquement en mode fichier (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length` : Longueur du corps non tronqué. Octets UTF-8 quand `OTEL_LOG_RAW_API_BODIES=file:<dir>`, ou unités de code UTF-16 quand `=1`
* `body_truncated` : `"true"` quand la troncature en ligne s'est produite. Absent en mode fichier et quand aucune troncature ne s'est produite.
* `model` : Identifiant du modèle
* `query_source` : Sous-système qui a émis la demande
* `request_id` : ID de demande API Anthropic de l'en-tête `request-id` de la réponse, par exemple `"req_011..."`. Présent uniquement quand l'API en retourne un.
* `request_body_id` : Le `request_body_id` de l'[événement `api_request_body`](#api-request-body-event) auquel cette réponse répond. Nécessite Claude Code v2.1.274 ou ultérieur
* `message.id` : ID de message que l'API a assigné à la réponse, le champ `id` du corps de réponse. Nécessite Claude Code v2.1.274 ou ultérieur
* `message.uuid` : UUID de l'entrée de transcription finale de la réponse. Avec `request_body_id`, il lie un message de transcription aux corps de demande et de réponse derrière lui. Nécessite Claude Code v2.1.274 ou ultérieur

<h4 id="tool-decision-event">
  Événement de décision d'outil
</h4>

Enregistré quand une décision de permission d'outil est prise (accepter/rejeter).

**Nom de l'événement** : `claude_code.tool_decision`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"tool_decision"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `tool_name` : Nom de l'outil (par exemple, « Read », « Edit », « Write », « NotebookEdit »)
* `tool_use_id` : Identifiant unique pour cette invocation d'outil. Correspond au `tool_use_id` passé aux hooks, permettant la corrélation entre les événements OTel et les données capturées par les hooks.
* `decision` : Soit `"accept"` soit `"reject"`
* `tool_source` : Toujours présent. La provenance de l'outil, comme un ensemble fermé de valeurs créées par l'interface de ligne de commande. Nécessite Claude Code v2.1.214 ou ultérieur
  * `"builtin"` : les outils de l'interface de ligne de commande elle-même
  * `"mcp"` : serveurs MCP en général
  * `"sdk_host_builtin_mcp"` : un serveur en processus intégré à Claude Desktop lui-même, dans une session que Claude Desktop possède. Claude Desktop possède une session qu'il a démarrée à partir de l'un de ses propres points d'entrée, `claude-desktop`, `claude-desktop-3p`, ou `local-agent`, quand cette session n'est pas un enfant imbriqué ; les sessions imbriquées, y compris les sessions que Claude Code lui-même génère, signalent ces serveurs comme `"mcp"`
* `source` : D'où provient la décision :
  * `"config"` : Décidé automatiquement sans invite, basé sur les paramètres du projet, les règles d'autorisation ou de refus dans les paramètres personnels de l'utilisateur, la politique gérée par l'entreprise, les drapeaux `--allowedTools` ou `--disallowedTools`, le mode de permission actif, une subvention à portée de session d'une invite antérieure dans la même session CLI interactive, ou parce que l'outil est intrinsèquement sûr. L'événement n'indique pas laquelle de ces sources a correspondu. Claude Code signale également `"config"` quand la demande d'invite de permission elle-même échoue, par exemple quand le rappel [`canUseTool`](/docs/fr/agent-sdk/typescript#canusetool) du SDK Agent ou l'outil [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) retourne un résultat invalide, ou quand le flux d'entrée se ferme tandis que la demande est en attente. Avant v2.1.216, Claude Code signalait ces défaillances comme `"user_reject"`.
  * `"hook"` : Un hook `PreToolUse` ou `PermissionRequest` a retourné la décision.
  * `"user_permanent"` : Émis quand l'utilisateur a choisi « Oui, et ne me demande plus pour ... » à une invite de permission, ce qui enregistre une règle d'autorisation dans ses paramètres personnels. Dans l'interface de ligne de commande interactive, ceci n'est émis que pour ce choix lui-même ; les appels ultérieurs qui correspondent à la règle enregistrée émettent `"config"` à la place. Dans les sessions SDK Agent ou non-interactive `-p`, à la fois le choix initial et les correspondances de règles ultérieures émettent `"user_permanent"`. Traité comme une acceptation.
  * `"user_temporary"` : Émis quand l'utilisateur a choisi « Oui » à une invite de permission pour une approbation unique, ou a choisi une option qui accorde l'accès pour le reste de la session sur une invite d'édition ou de lecture de fichier. Dans l'interface de ligne de commande interactive, ceci n'est émis que pour le choix lui-même ; les appels ultérieurs autorisés par cette subvention à portée de session émettent `"config"` à la place. Dans les sessions SDK Agent ou non-interactive `-p`, à la fois le choix et les correspondances ultérieures émettent `"user_temporary"`. Traité comme une acceptation.
  * `"user_abort"` : Émis quand l'utilisateur a fermé l'invite de permission sans répondre. Dans les sessions SDK Agent et non-interactive `-p`, ceci inclut l'interruption du tour tandis qu'une demande de permission `canUseTool` ou `--permission-prompt-tool` est en attente ; avant v2.1.216, Claude Code signalait cette interruption comme `"user_reject"`. Traité comme un rejet.
  * `"user_reject"` : Émis quand l'utilisateur a choisi « Non » quand invité. Dans l'interface de ligne de commande interactive, ceci n'est émis que pour ce choix lui-même ; les appels qui correspondent à une règle de refus dans les paramètres personnels de l'utilisateur émettent `"config"` à la place. Dans les sessions SDK Agent ou non-interactive `-p`, les appels qui correspondent à une règle de refus dans les paramètres personnels émettent `"user_reject"`. Traité comme un rejet.
* `tool_parameters` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Chaîne JSON contenant les paramètres spécifiques à l'outil. Même forme que l'[événement de résultat d'outil](#tool-result-event), moins les champs post-exécution tels que `git_commit_id`. Les valeurs peuvent différer de `tool_result` pour un appel accepté si la décision de permission réécrit l'entrée d'outil via `updatedInput`. Utilisez cet attribut pour voir quelle commande a été rejetée quand `decision` est `"reject"`.
  * Pour les outils `"sdk_host_builtin_mcp"` : `mcp_server_name` et `mcp_tool_name` sont inclus même quand `OTEL_LOG_TOOL_DETAILS` est désactivé, car l'application hôte définit ces noms ; sans eux, un appel rejeté à l'un de ces serveurs intégrés serait non attribuable sur le flux par défaut. Pour les serveurs MCP configurés par l'utilisateur, le `tool_name` de l'événement est toujours le littéral `"mcp_tool"`, et les noms du serveur et de l'outil n'apparaissent que dans `tool_parameters` avec le drapeau activé ; le contenu des arguments nécessite le drapeau partout. Nécessite Claude Code v2.1.214 ou ultérieur
  * Pour l'outil Bash : inclut `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`. L'outil bash d'espace de travail de l'application de bureau signale également `tool_name` comme `Bash`, mais inclut uniquement `bash_command`, `full_command`, et `timeout`
  * Pour les outils MCP : inclut `mcp_server_name`, `mcp_tool_name`
  * Pour l'outil Skill : inclut `skill_name`
  * Pour l'outil Agent ou l'outil Task hérité : inclut `subagent_type`

<h4 id="permission-mode-changed-event">
  Événement de changement de mode de permission
</h4>

Enregistré quand le mode de permission change, par exemple à partir du cycle Maj+Tab, de la sortie du mode plan, ou d'une vérification de porte en mode automatique.

**Nom de l'événement** : `claude_code.permission_mode_changed`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"permission_mode_changed"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `from_mode` : Le mode de permission précédent, par exemple `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, ou `"bypassPermissions"`
* `to_mode` : Le nouveau mode de permission
* `trigger` : Ce qui a causé le changement. L'une de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, ou `"auto_opt_in"`. Absent quand la transition provient du SDK ou du pont

<h4 id="auth-event">
  Événement d'authentification
</h4>

Enregistré quand `/login` ou `/logout` se termine.

**Nom de l'événement** : `claude_code.auth`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"auth"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `action` : `"login"` ou `"logout"`
* `success` : `"true"` ou `"false"`
* `auth_method` : Méthode d'authentification, par exemple `"oauth"`
* `error_category` : Type d'erreur catégorique quand l'action a échoué. Le message d'erreur brut n'est jamais inclus
* `status_code` : Code de statut HTTP en tant que chaîne quand l'action a échoué avec une erreur HTTP

<h4 id="mcp-server-connection-event">
  Événement de connexion du serveur MCP
</h4>

Enregistré quand un serveur MCP se connecte, se déconnecte, ou échoue à se connecter.

**Nom de l'événement** : `claude_code.mcp_server_connection`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"mcp_server_connection"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `status` : `"connected"`, `"failed"`, ou `"disconnected"`
* `transport_type` : Transport du serveur, par exemple `"stdio"`, `"sse"`, ou `"http"`
* `server_scope` : Portée à laquelle le serveur est configuré, par exemple `"user"`, `"project"`, ou `"local"`
* `duration_ms` : Durée de la tentative de connexion en millisecondes
* `error_code` : Code d'erreur quand la connexion a échoué
* `is_plugin` : `true` quand le serveur est fourni par un plugin, `false` sinon
* `plugin_id_hash` (quand `is_plugin` est `true`) : Hash stable du nom du plugin et de la place de marché, pour regrouper les événements par plugin sans exposer le nom. Claude Code le calcule comme décrit sous l'[événement de plugin chargé](#plugin-loaded-event)
* `plugin.name` (quand `is_plugin` est `true`) : Nom du plugin qui fournit le serveur. Pour les plugins tiers, c'est la chaîne littérale `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1` ; ceci protège les noms de plugins tiers d'apparaître dans les journaux par défaut. Les plugins de sources Anthropic officielles sont toujours identifiés par nom. Les attributs `plugin_id_hash` et `plugin.name` circulent vers votre propre backend de surveillance et ne sont pas envoyés à Anthropic
* `server_name` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Nom du serveur configuré
* `error` (quand `OTEL_LOG_TOOL_DETAILS=1`) : Message d'erreur complet quand la connexion a échoué

<h4 id="internal-error-event">
  Événement d'erreur interne
</h4>

Enregistré quand Claude Code capture une erreur interne inattendue. Seul le nom de la classe d'erreur et un code de style errno sont enregistrés. Le message d'erreur et la trace de pile ne sont jamais inclus. Cet événement n'est pas émis lors de l'exécution contre Amazon Bedrock, la plateforme d'agents de Google Cloud, ou Microsoft Foundry, ou quand `DISABLE_ERROR_REPORTING` est défini.

**Nom de l'événement** : `claude_code.internal_error`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"internal_error"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `error_name` : Nom de la classe d'erreur, par exemple `"TypeError"` ou `"SyntaxError"`
* `error_code` : Code errno Node.js tel que `"ENOENT"` quand présent sur l'erreur

<h4 id="plugin-installed-event">
  Événement de plugin installé
</h4>

Enregistré quand un plugin termine son installation, à partir à la fois de la commande CLI `claude plugin install` et de l'interface utilisateur interactive `/plugin`.

**Nom de l'événement** : `claude_code.plugin_installed`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"plugin_installed"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `marketplace.is_official` : `"true"` si la place de marché est une place de marché officielle Anthropic, `"false"` sinon
* `install.trigger` : `"cli"` ou `"ui"`
* `plugin.name` : Nom du plugin installé. Pour les places de marché tiers, ceci n'est inclus que quand `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version` : Version du plugin quand déclarée dans l'entrée de la place de marché. Pour les places de marché tiers, ceci n'est inclus que quand `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name` : Place de marché à partir de laquelle le plugin a été installé. Pour les places de marché tiers, ceci n'est inclus que quand `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Événement de plugin chargé
</h4>

Enregistré une fois par plugin activé au démarrage de la session. Utilisez cet événement pour inventorier quels plugins sont actifs dans votre flotte, en complément de `plugin_installed` qui enregistre l'action d'installation elle-même.

**Nom de l'événement** : `claude_code.plugin_loaded`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"plugin_loaded"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `plugin.name` : nom du plugin. Pour les plugins en dehors de la place de marché officielle et du bundle intégré, la valeur est `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name` : place de marché à partir de laquelle le plugin a été installé, quand connue. Rédacté à `"third-party"` sous la même condition que `plugin.name`
* `plugin.version` : version du manifeste du plugin. Inclus uniquement quand le nom n'est pas rédacté et que le manifeste déclare une version
* `plugin.scope` : catégorie de provenance pour le plugin : `"official"`, `"community"`, `"org"`, `"user-local"`, ou `"default-bundle"`
* `enabled_via` : comment le plugin en est venu à être activé : `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"`, ou `"user-install"`. La valeur `"admin-install"` signifie que le plugin est défini comme obligatoire ou auto-installation pour votre organisation dans [**Paramètres de l'organisation > Plugins et compétences**](https://claude.ai/admin-settings/skills?tab=inventory). Avant v2.1.246, Claude Code signalait ces plugins comme `"user-install"` ou `"seed-mount"`
* `plugin_id_hash` : hash déterministe du nom du plugin et de la place de marché, envoyé uniquement à votre exportateur configuré. Vous permet de compter les plugins tiers distincts chargés dans votre flotte sans enregistrer leurs noms. Pour les [plugins synchronisés à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins), Claude Code hache le nom du plugin avec le nom de la place de marché que claude.ai signale pour le plugin, ou avec `synced` sinon. Avant v2.1.246, Claude Code n'utilisait pas le nom de la place de marché que claude.ai signale dans le hash
* `has_hooks` : si le plugin contribue des hooks
* `has_mcp` : si le plugin contribue des serveurs MCP
* `host_owned_mcp` : `true` quand l'hôte SDK gère les connexions MCP de ce plugin et Claude Code a ignoré la lecture de la configuration du serveur MCP du plugin, `false` sinon. Nécessite Claude Code v2.1.172 ou ultérieur
* `skill_path_count` : nombre de répertoires de compétences que le plugin déclare
* `command_path_count` : nombre de répertoires de commandes que le plugin déclare
* `agent_path_count` : nombre de répertoires d'agents que le plugin déclare
* `safe_mode` : `"true"` quand la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. En mode sûr, cet événement signale uniquement l'inventaire configuré ; les commandes, compétences, hooks et serveurs MCP du plugin ne se chargent pas. Nécessite Claude Code v2.1.169 ou ultérieur

<h4 id="skill-activated-event">
  Événement de compétence activée
</h4>

Enregistré quand une compétence est invoquée, que Claude l'appelle via l'outil Skill ou que vous l'exécutiez en tant que commande `/`.

**Nom de l'événement** : `claude_code.skill_activated`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"skill_activated"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `skill.name` : Nom de la compétence. Pour les compétences définies par l'utilisateur et les plugins tiers, la valeur est l'espace réservé `"custom_skill"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger` : Comment la compétence a été déclenchée (`"user-slash"`, `"claude-proactive"`, ou `"nested-skill"`)
* `skill.source` : D'où la compétence a été chargée (par exemple, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind` : `"workflow"` quand la compétence est une compétence de flux de travail. Absent sinon
* `plugin.name` (quand `OTEL_LOG_TOOL_DETAILS=1` ou le plugin provient d'une place de marché officielle) : Nom du plugin propriétaire quand la compétence est fournie par un plugin
* `marketplace.name` (quand `OTEL_LOG_TOOL_DETAILS=1` ou le plugin provient d'une place de marché officielle) : Place de marché à partir de laquelle le plugin propriétaire a été installé, quand la compétence est fournie par un plugin

<h4 id="at-mention-event">
  Événement de mention @
</h4>

Enregistré quand Claude Code résout une mention `@` dans une invite. Pas chaque mention n'émet un événement : les chemins de sortie anticipée tels que les refus de permission, les fichiers surdimensionnés, les pièces jointes de référence PDF et les défaillances de listage de répertoires retournent sans journalisation.

**Nom de l'événement** : `claude_code.at_mention`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"at_mention"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `mention_type` : Type de mention (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`). La valeur `"peer"` signifie que vous avez mentionné [l'une de vos autres sessions Claude Code](/docs/fr/cross-session-messaging). Nécessite Claude Code v2.1.232 ou ultérieur
* `success` : Si la mention a été résolue avec succès (`"true"` ou `"false"`)

<h4 id="api-retries-exhausted-event">
  Événement de nouvelles tentatives API épuisées
</h4>

Enregistré une fois quand une demande API échoue après plus d'une tentative. Émis aux côtés de l'événement `api_error` final.

**Nom de l'événement** : `claude_code.api_retries_exhausted`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"api_retries_exhausted"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `model` : Modèle utilisé
* `error` : Message d'erreur final
* `status_code` : Code de statut HTTP en tant que nombre. Absent pour les erreurs non-HTTP.
* `total_attempts` : Nombre total de tentatives effectuées
* `total_retry_duration_ms` : Temps mural total à travers toutes les tentatives
* `speed` : `"fast"` ou `"normal"`

<h4 id="hook-registered-event">
  Événement de hook enregistré
</h4>

Enregistré une fois par hook configuré au démarrage de la session. Utilisez cet événement pour inventorier quels hooks sont actifs dans votre flotte, en complément des événements par exécution `hook_execution_start` et `hook_execution_complete`.

**Nom de l'événement** : `claude_code.hook_registered`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_registered"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `hook_event` : type d'événement hook, par exemple `"PreToolUse"` ou `"PostToolUse"`
* `hook_type` : type d'implémentation du hook : `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, ou `"agent"`
* `hook_source` : où le hook est défini : `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, ou `"pluginHook"`
* `safe_mode` : `"true"` quand la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_matcher` (quand `OTEL_LOG_TOOL_DETAILS=1`) : la chaîne de correspondance de la configuration du hook, quand une est définie
* `plugin.name` (quand `hook_source` est `"pluginHook"`) : nom du plugin contributeur. Pour les plugins en dehors de la place de marché officielle et du bundle intégré, la valeur est `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (quand `hook_source` est `"pluginHook"`) : hash déterministe du nom du plugin et de la place de marché, envoyé uniquement à votre exportateur configuré. Vous permet de compter les plugins contributeurs distincts sans enregistrer leurs noms. Claude Code le calcule comme décrit sous l'[événement de plugin chargé](#plugin-loaded-event)

<h4 id="hook-execution-start-event">
  Événement de début d'exécution de hook
</h4>

Enregistré quand un ou plusieurs hooks commencent à s'exécuter pour un événement hook.

**Nom de l'événement** : `claude_code.hook_execution_start`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_execution_start"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `hook_event` : Type d'événement hook, par exemple `"PreToolUse"` ou `"PostToolUse"`
* `hook_name` : Nom complet du hook incluant la correspondance, par exemple `"PreToolUse:Write"`
* `num_hooks` : Nombre de commandes hook correspondantes
* `managed_only` : `"true"` quand seuls les hooks de politique gérée sont autorisés
* `hook_source` : `"policySettings"` ou `"merged"`
* `safe_mode` : `"true"` quand la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_definitions` : Configuration du hook sérialisée en JSON. Inclus uniquement quand le traçage bêta détaillé et `OTEL_LOG_TOOL_DETAILS=1` sont tous deux activés

<h4 id="hook-execution-complete-event">
  Événement de fin d'exécution de hook
</h4>

Enregistré quand tous les hooks pour un événement hook ont terminé.

**Nom de l'événement** : `claude_code.hook_execution_complete`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_execution_complete"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `hook_event` : Type d'événement hook
* `hook_name` : Nom complet du hook incluant la correspondance
* `num_hooks` : Nombre de commandes hook correspondantes
* `num_success` : Nombre qui ont terminé avec succès
* `num_blocking` : Nombre qui ont retourné une décision de blocage
* `num_non_blocking_error` : Nombre qui ont échoué sans bloquer
* `num_cancelled` : Nombre annulé avant la fin
* `total_duration_ms` : Durée mural de tous les hooks correspondants
* `stdout_chars` : Nombre total de caractères de stdout à travers les hooks correspondants qui ont réussi. Nécessite Claude Code v2.1.280 ou ultérieur
* `additional_context_chars` : Nombre total de caractères de `additionalContext` retournés par les hooks correspondants. Nécessite Claude Code v2.1.280 ou ultérieur
* `system_message_chars` : Nombre total de caractères de `systemMessage` retournés par les hooks correspondants. Nécessite Claude Code v2.1.280 ou ultérieur
* `initial_user_message_chars` : Nombre total de caractères de `initialUserMessage` retournés par les hooks correspondants. Nécessite Claude Code v2.1.280 ou ultérieur
* `num_outputs_persisted` : Nombre de sorties de hook au-delà de la [limite de 10 000 caractères](/docs/fr/hooks#json-output) que Claude Code a enregistrées dans un fichier. Nécessite Claude Code v2.1.280 ou ultérieur
* `managed_only` : `"true"` quand seuls les hooks de politique gérée sont autorisés
* `hook_source` : `"policySettings"` ou `"merged"`
* `safe_mode` : `"true"` quand la session a été démarrée avec [`--safe-mode`](/docs/fr/cli-reference), `"false"` sinon. Nécessite Claude Code v2.1.169 ou ultérieur
* `hook_definitions` : Configuration du hook sérialisée en JSON. Inclus uniquement quand le traçage bêta détaillé et `OTEL_LOG_TOOL_DETAILS=1` sont tous deux activés

<h4 id="hook-plugin-metrics-event">
  Événement de métriques de plugin de hook
</h4>

Enregistré quand un hook de plugin de la place de marché officielle émet des métriques par invocation. Seuls les plugins installés à partir d'une place de marché Anthropic officielle peuvent émettre ceci. Les plugins de place de marché tiers et les hooks configurés par l'utilisateur n'émettent pas vers cet événement. Utilisez cet événement pour surveiller le comportement du plugin, par exemple les taux de découverte, les coûts et les durées à partir de votre propre pile d'observabilité.

**Nom de l'événement** : `claude_code.hook_plugin_metrics`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"hook_plugin_metrics"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `plugin_id` : identifiant du plugin sous la forme `<name>@<marketplace>`
* `hook_event` : type d'événement hook qui a émis les métriques
* Jusqu'à 20 clés de métriques émises par le plugin. Les noms correspondent à `^[a-z][a-z0-9_]{0,39}$`. Les valeurs sont booléennes ou numériques.

<h4 id="compaction-event">
  Événement de compaction
</h4>

Enregistré quand la compaction de conversation se termine.

**Nom de l'événement** : `claude_code.compaction`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"compaction"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `trigger` : `"auto"` ou `"manual"`
* `success` : `"true"` ou `"false"`
* `duration_ms` : Durée de compaction
* `pre_tokens` : Nombre approximatif de jetons avant compaction
* `post_tokens` : Nombre approximatif de jetons après compaction
* `error` : Message d'erreur quand la compaction a échoué
* `precompute_reuse` : Défini uniquement quand `trigger` est `"manual"`. La compaction automatique peut préparer un résumé en arrière-plan avant que la fenêtre de contexte se remplisse, et cet attribut enregistre si `/compact` a réutilisé ce résumé préparé. `"hit"` signifie qu'il a été réutilisé ; `"miss_custom_instructions"`, `"miss_hook"`, et `"miss_not_ready"` donnent la raison pour laquelle un résumé frais a été calculé à la place. Nécessite Claude Code v2.1.153 ou ultérieur

<h4 id="subagent-completed-event">
  Événement de sous-agent terminé
</h4>

Enregistré quand un [sous-agent](/docs/fr/sub-agents) se termine et retourne son résultat à la conversation qui l'a démarré. Utilisez-le pour cumuler l'utilisation d'outils et le temps d'exécution par type de sous-agent ; pour les cumuls de jetons ou de coûts, utilisez le [compteur de jetons](#token-counter) et le [compteur de coût](#cost-counter) filtrés sur `query_source` `"subagent"`, puisque le `total_tokens` de cet événement couvre uniquement la demande finale. La catégorie `"subagent"` compte également les demandes des hooks basés sur les agents, qui n'émettent aucun événement de sous-agent.

**Nom de l'événement** : `claude_code.subagent_completed`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"subagent_completed"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `agent_type` : Le type de sous-agent. Les noms d'agents intégrés et les agents des plugins de la place de marché officielle apparaissent textuellement ; les autres noms d'agents sont remplacés par `"custom"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini
* `agent.source` : D'où provient la définition de l'agent : `built-in`, `plugin`, ou la source de paramètres qui a défini un agent personnalisé, par exemple `userSettings` ou `projectSettings`
* `is_built_in` : Si le sous-agent est un type d'agent intégré
* `is_async` : Si le sous-agent s'est exécuté en [arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background)
* `total_tokens` : L'empreinte de jeton de la demande API finale du sous-agent : les jetons d'entrée, de création de cache, de lecture de cache et de sortie de cette seule demande, à peu près la taille du contexte du sous-agent à la fin. Pas une somme à travers l'exécution
* `total_tool_uses` : Nombre d'appels d'outils que le sous-agent a effectués à travers toute l'exécution
* `duration_ms` : Temps d'exécution en millisecondes
* `model` : Le modèle auquel le sous-agent a été résolu pour s'exécuter
* `final_model` : Le modèle qui a produit la réponse finale du sous-agent, qui diffère de `model` après un changement en cours d'exécution tel qu'un secours. Nécessite Claude Code v2.1.212 ou ultérieur
* `model_swapped` : Si plus d'un modèle a servi les demandes du sous-agent. Nécessite Claude Code v2.1.212 ou ultérieur
* `plugin_id_hash`, `plugin.name` : Présent pour les agents fournis par les plugins. Les noms de plugins de la place de marché officielle apparaissent textuellement ; les autres noms de plugins sont remplacés par `"third-party"` sauf si `OTEL_LOG_TOOL_DETAILS=1` est défini

<h4 id="feedback-survey-event">
  Événement d'enquête de rétroaction
</h4>

Enregistré quand une enquête de qualité de session est affichée ou répondue. Voir [Enquêtes de qualité de session](/docs/fr/data-usage#session-quality-surveys) pour ce que les enquêtes collectent et comment les contrôler.

**Nom de l'événement** : `claude_code.feedback_survey`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"feedback_survey"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `event_type` : Événement du cycle de vie de l'enquête, par exemple `"appeared"`, `"responded"`, ou `"transcript_prompt_appeared"`
* `appearance_id` : ID unique liant les événements émis pour une instance d'enquête
* `survey_type` : Quelle enquête a produit l'événement. `"session"` est l'invite d'évaluation « Comment Claude se débrouille-t-il ? »
* `response` : La sélection de l'utilisateur sur les événements `responded`
* `enabled_via_override` : `true` quand [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/fr/env-vars) est défini. Émis comme un booléen, pas une chaîne. Présent sur les événements d'enquête `session`. Filtrez sur cet attribut pour confirmer que le remplacement est appliqué dans une flotte

<h4 id="retention-sweep-event">
  Événement de balayage de rétention
</h4>

Enregistré une fois par exécution du balayage de nettoyage de rétention, qui supprime les [transcriptions de session et autres données d'application](/docs/fr/claude-directory#cleaned-up-automatically) plus anciennes que le paramètre [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays). Claude Code exécute le balayage en arrière-plan au maximum une fois par session, et une exécution qui ne supprime rien émet quand même l'événement. Si Claude Code a exécuté le balayage dans n'importe quelle session sur la même machine au cours des 24 dernières heures, il retarde le balayage de cette session d'au moins 10 minutes, donc une session qui se termine plus tôt n'émet rien. Quand vous exécutez `claude -p` avec `--bare`, Claude Code n'exécute pas le balayage et n'émet rien.

Comme chaque événement OTel sur cette page, il va uniquement au backend de télémétrie que vous configurez. Nécessite Claude Code v2.1.227 ou ultérieur.

Quand Claude Code ne peut pas déterminer avec certitude la période de rétention, il met en pause le balayage et émet l'événement avec `result` défini à `"skipped"` et une `skip_reason`. Quand les [paramètres gérés](/docs/fr/server-managed-settings) définissent `cleanupPeriodDays`, la valeur gérée épingle la période de rétention et le balayage s'exécute même quand un fichier de paramètres dans une portée de priorité inférieure est cassé ou invalide. Quand `managed-settings.json` lui-même ne peut pas être lu, Claude Code met quand même en pause le balayage sauf si le [niveau géré](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) fournit `cleanupPeriodDays` d'ailleurs, par exemple les paramètres gérés par le serveur ou une suppression `managed-settings.d/` à côté du fichier cassé. Les attributs du compteur de suppression sont présents uniquement quand `result` est `"complete"`.

**Nom de l'événement** : `claude_code.retention_sweep`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"retention_sweep"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `result` : `"complete"` quand le balayage s'est exécuté, `"skipped"` quand Claude Code l'a mis en pause
* `period_days` : La valeur `cleanupPeriodDays` des paramètres fusionnés, en jours, ou `30` quand aucune source ne la définit. Sur les événements ignorés, la valeur que le balayage aurait utilisée, calculée à partir des sources de paramètres que Claude Code a pu lire
* `used_default` : `"true"` quand aucune source de paramètres lisible ne définit `cleanupPeriodDays`, `"false"` sinon. Sur les événements complets, `"true"` signifie que la valeur par défaut de 30 jours s'appliquait
* `skip_reason` : Pourquoi Claude Code a mis en pause le balayage. Présent uniquement quand `result` est `"skipped"` :
  * `"user_source_disabled"` : Les paramètres utilisateur sont exclus, par exemple par le drapeau [`--setting-sources`](/docs/fr/cli-reference#cli-flags) ou l'option [`settingSources`](/docs/fr/agent-sdk/typescript#options) du SDK, et aucune source activée ne fournit `cleanupPeriodDays`
  * `"settings_unknowable"` : Un fichier de paramètres n'a pas pu être lu ou analysé, donc `cleanupPeriodDays` ou `desktopSessionCleanupPeriodDays` peut être défini à une valeur que Claude Code ne peut pas voir
  * `"settings_invalid_key_set"` : Les paramètres ont des erreurs de validation et `cleanupPeriodDays` ou `desktopSessionCleanupPeriodDays` est explicitement défini, donc revenir à la valeur par défaut pourrait supprimer ou conserver des fichiers contre ce paramètre
* `transcripts_deleted` : Nombre de transcriptions de session, les fichiers `~/.claude/projects/*/*.jsonl` de niveau supérieur, que le balayage a supprimés
* `transcripts_exempted_desktop` : Nombre de transcriptions au-delà de la période de rétention que le balayage a conservées selon la [règle Claude Desktop et Cowork](/docs/fr/claude-directory#cleaned-up-automatically). Celles-ci ne comptent pas vers `files_past_cutoff`. Nécessite Claude Code v2.1.248 ou ultérieur
* `session_files_deleted` : Nombre d'artefacts que le balayage des fichiers de session a supprimés : transcriptions plus fichiers compagnons par session tels que les barres latérales, les enregistrements et les résultats d'outils
* `artifacts_deleted` : Nombre total d'éléments que le balayage a supprimés à travers les répertoires de données qu'il couvre, y compris les fichiers de session. Certains balayages comptent un arbre de répertoires supprimé entier comme un élément et quelques passes de nettoyage ne contribuent pas au compteur, donc traitez la valeur comme un plancher plutôt qu'un nombre de fichiers exact
* `files_retained_fresh` : Fichiers inspectés et laissés en place car ils sont toujours dans la période de rétention. Seuls les balayages par fichier comptent ceux-ci, donc la valeur est un plancher ; une valeur non nulle est l'état stable normal
* `files_past_cutoff` : Fichiers plus anciens que la période de rétention que le balayage n'a pas pu supprimer, par exemple en raison d'une erreur de permission ou d'un fichier maintenu ouvert. Une valeur supérieure à zéro signifie que les fichiers ont dépassé la période de rétention configurée ; zéro n'est pas la preuve qu'aucun ne l'a fait, car une suppression échouée d'un répertoire entier compte vers `error_count` à la place
* `error_count` : Nombre d'erreurs que le balayage a rencontrées lors de la liste ou de la suppression de fichiers

<h4 id="managed-settings-resolved-event">
  Événement de paramètres gérés résolus
</h4>

Enregistré avec les [paramètres gérés](/docs/fr/managed-settings) qu'une session a résolus : une fois au démarrage de la session, à nouveau quand soit les paramètres gérés soit l'état de l'[assistant de politique](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program) change pendant la session, et quand Claude Code refuse de démarrer ou termine la session pour l'une des raisons que l'attribut `error.type` liste.
Utilisez cet événement pour trouver les machines exécutées sur une source gérée inattendue, les machines dont l'assistant de politique échoue, et la raison pour laquelle une machine a refusé de démarrer.
Nécessite Claude Code v2.1.274 ou ultérieur.

Par défaut, l'événement porte les sources gérées et l'état de l'assistant de politique mais pas les paramètres eux-mêmes. Pour ajouter l'attribut `managed_settings.settings` rédacté et le digest `managed_settings.resolved_sha256`, définissez `OTEL_LOG_MANAGED_SETTINGS=1` :

* Définissez-le dans le bloc `env` des paramètres gérés, des paramètres utilisateur, ou `--settings`, ou dans l'environnement avec lequel vous lancez Claude Code. Une valeur dans les paramètres du projet ou locaux ne l'active pas, car un référentiel cloné peut les écrire.
* Les paramètres gérés par le serveur peuvent le définir sans afficher la [boîte de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs), car la variable ajoute uniquement votre propre politique rédactée de l'organisation à un événement que votre organisation reçoit déjà.

Dans une session interactive dans un dossier que vous n'avez pas [approuvé](/docs/fr/permissions#what-runs-before-you-trust-a-folder), Claude Code n'exporte pas l'événement de refus.

**Nom de l'événement** : `claude_code.managed_settings_resolved`

**Attributs** :

* Tous les [attributs standard](#standard-attributes)
* `event.name` : `"managed_settings_resolved"`
* `event.timestamp` : Timestamp ISO 8601
* `event.sequence` : compteur par processus pour ordonner les événements, décrit sous [Attributs de corrélation d'événements](#event-correlation-attributes)
* `managed_settings.trigger` : `"startup"` pour l'événement de démarrage de session, `"change"` quand les paramètres gérés ou l'état de l'assistant de politique ont changé plus tard dans la session, ou `"refused"` quand une politique de paramètres gérés a arrêté la session. Claude Code envoie un événement `change` uniquement quand un attribut diffère du dernier événement qu'il a envoyé, et une valeur de paramètre modifiée compte même quand `OTEL_LOG_MANAGED_SETTINGS` est désactivé
* `error.type` : pourquoi Claude Code a arrêté la session. Présent uniquement sur les événements `refused` :
  * `"helper_failed"` : une [exécution d'assistant de politique a échoué](/docs/fr/settings-reference#helper-failures)
  * `"policy_invalid"` : les paramètres gérés contiennent une erreur qui arrête Claude Code de démarrer, ou une source d'administration n'a pas pu se charger, donc Claude Code ne peut pas vérifier l'application de la connexion de l'organisation
  * `"consent_rejected"` : l'utilisateur a rejeté la [boîte de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs) pour les paramètres gérés par le serveur
  * `"force_refresh_failed"` : l'extraction de paramètres que [`forceRemoteSettingsRefresh`](/docs/fr/settings-reference#forceremotesettingsrefresh) nécessite a échoué
  * `"gateway_rejected"` : une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) a répondu au chargement des paramètres gérés avec HTTP 403
  * `"version_below_minimum"` : cette version de Claude Code est en dessous de [`requiredMinimumVersion`](/docs/fr/settings-reference#requiredminimumversion) ou au-dessus de [`requiredMaximumVersion`](/docs/fr/settings-reference#requiredmaximumversion)
  * `"_OTHER"` : le chargement des paramètres gérés de la passerelle d'applications Claude a échoué pour une autre raison
* `managed_settings.sources` : chaque source gérée qui fournit au moins une [clé de politique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources), priorité la plus élevée en premier, y compris les sources dont les clés ne prennent pas effet sous `first-wins`. Les valeurs sont `"remote"`, `"plist"` ou `"hklm"` pour la politique MDM ou au niveau du système d'exploitation, `"file"` pour les fichiers de paramètres gérés et les suppressions, `"parent"` quand un [hôte d'intégration](/docs/fr/managed-settings#let-an-embedding-host-add-policy) fournit des paramètres, et `"hkcu"` pour la [valeur du registre Windows HKCU](/docs/fr/managed-settings#where-each-mechanism-stores-the-policy) quand Claude Code la [lit](/docs/fr/managed-settings#how-claude-code-combines-managed-sources). Une source qui porte uniquement des clés de contrôle, ou que Claude Code n'a pas pu lire, n'est pas listée. Émis comme un tableau de chaînes, vide quand aucune source gérée ne fournit une clé de politique
* `managed_settings.source_behavior` : la valeur [`managedSourcesBehavior`](/docs/fr/settings-reference#managedsourcesbehavior) que Claude Code a lue, `"first-wins"` ou `"merge"`. `"first-wins"` quand aucune source ne définit la clé
* `managed_settings.helper.state` : état de l'assistant de politique que la source MDM ou fichier sélectionnée configure :
  * `"ok"` : la sortie de l'assistant sert de paramètres gérés
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"`, ou `"schema_rejected"` : la dernière exécution de l'assistant a échoué. Les [défaillances d'assistant](/docs/fr/settings-reference#helper-failures) décrivent les cas
  * `"none"` : aucun assistant n'est configuré, ou la source qui le configure n'est pas une politique MDM ou un fichier de paramètres gérés
* `managed_settings.helper.applied` : `"output"` tandis que la propre sortie de l'assistant sert de paramètres gérés, `"none"` quand ce n'est pas le cas
* `managed_settings.helper.entry` : `"policyHelper"` quand Claude Code a sélectionné un [`policyHelper`](/docs/fr/settings-reference#policyhelper). Absent quand il n'a sélectionné aucun assistant
* `managed_settings.helper.path` : le [`path`](/docs/fr/settings-reference#policyhelper-path) configuré de l'assistant. Présent chaque fois que Claude Code a sélectionné un assistant, que `OTEL_LOG_MANAGED_SETTINGS` soit défini ou non
* `managed_settings.resolved_sha256` (quand `OTEL_LOG_MANAGED_SETTINGS=1`) : SHA-256 des paramètres gérés résolus avant rédaction, sérialisé en JSON avec les clés triées récursivement et sans espace blanc. Les machines avec le même digest exécutent la même politique. Claude Code envoie le digest uniquement avec l'opt-in car une politique courte peut être récupérée en hachant des suppositions. Absent quand aucun paramètre géré n'a été résolu, et sur les événements `refused`
* `managed_settings.settings` (quand `OTEL_LOG_MANAGED_SETTINGS=1`) : les noms et la forme des paramètres gérés résolus sous forme de chaîne JSON, avec les valeurs rédactées. Absent sur les événements `refused`. Claude Code le construit à partir de son schéma de paramètres :

  * Un nom de paramètre que le schéma déclare est exporté, et une clé qu'il ne déclare pas est laissée de côté
  * Les booléens, les nombres et les valeurs de chaîne que le schéma restreint à un ensemble fixe d'options, par exemple `permissions.defaultMode`, sont exportés tels quels. `sandbox.network.httpProxyPort` et `sandbox.network.socksProxyPort` sont exportés comme `"[REDACTED]"`
  * Chaque autre chaîne, par exemple `model`, `apiKeyHelper`, chaque valeur `env`, chaque URL et chaque commande, est exportée comme `"[REDACTED]"`
  * Les noms d'entrée des cartes, par exemple les noms de variables `env` et les ID de plugins, sont exportés tels quels. Un paramètre dont les entrées le schéma ne tape pas, par exemple `vimInsertModeRemaps`, est exporté comme un seul `"[REDACTED]"`, et `sandbox.ignoreViolations` est exporté comme une liste de ses listes de chemins sans les modèles de commande
  * Une liste conserve sa longueur, avec chaque entrée rédactée par les mêmes règles
  * Une règle `permissions.allow`, `permissions.deny`, ou `permissions.ask` est exportée comme son nom d'outil avec le contenu rédacté, par exemple `Read([REDACTED])`, quand l'outil est intégré à cette version de Claude Code ou est une référence `mcp__` telle que `mcp__jira__create_issue`. Toute autre règle est exportée comme `"[REDACTED]"`
  * Les hooks suivent les mêmes règles, donc les champs à option fixe et numériques tels que `type` et `timeout` s'affichent, tandis que chaque commande, URL, `matcher`, et condition `if` est exportée comme `"[REDACTED]"`

  Par exemple, les paramètres gérés avec `apiKeyHelper`, deux variables `env`, et une règle de refus sont exportés comme `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`.

  Claude Code coupe la valeur à 8 Ko d'UTF-8, et la valeur coupée n'est pas du JSON valide
* `managed_settings.settings_truncated` (quand `managed_settings.settings` est présent) : `true` quand Claude Code a coupé `managed_settings.settings` à 8 Ko, `false` sinon. Émis comme un booléen, pas une chaîne

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

Claude Code compte chaque réponse en streaming vers les métriques de coûts et de jetons exactement une fois, y compris lorsqu'une passerelle ou un proxy derrière `ANTHROPIC_BASE_URL` diffuse l'utilisation progressivement sur plusieurs images. Avant la v2.1.214, les flux qui contenaient l'utilisation dans plus d'une image gonflaient `claude_code.cost.usage` et `claude_code.token.usage` d'environ une demande complète supplémentaire par image supplémentaire.

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

L'attribut `attempt` sur l'événement enregistre le nombre total de tentatives effectuées. `CLAUDE_CODE_MAX_RETRIES` est par défaut `10` et plafonné à `15`. À partir de la v2.1.199, vous pouvez définir `CLAUDE_CODE_RETRY_WATCHDOG` pour augmenter la valeur par défaut et supprimer le plafond.

Lorsque la demande épuise toutes les tentatives sur une erreur transitoire, `attempt` est égal à un de plus que cette limite effective : 11 par défaut, et jamais plus de 16 sauf si le watchdog est défini. Une valeur inférieure indique une erreur non réessayable telle qu'une réponse `400`, ou une cause avec son propre budget de tentatives plus petit. Par exemple, Claude Code réessaie un échec de chargement des identifiants AWS ou Google Cloud au maximum deux fois.

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

Les [attributs standard](#standard-attributes) sur chaque événement incluent l'identité de l'utilisateur authentifié : `user.email`, `user.account_uuid`, `user.account_id`, et `organization.id` lorsqu'il est connecté avec un compte Claude ou, dans une [session cloud](/docs/fr/claude-code-on-the-web), lorsque les propres identifiants de la session les portent, plus `user.id` et le per-session `session.id`. `user.id` est un identifiant limité à l'installation, sauf sur les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), où il s'agit du sujet IdP du jeton émis par la passerelle.

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

* `tool_result` : conserve `mcp_server_scope` et un `tool_name` redacté au littéral `"mcp_tool"` pour les serveurs configurés par l'utilisateur, omet le contenu des arguments. Pour les serveurs intégrés de Claude Desktop, dans les sessions que Claude Desktop possède, il conserve également la paire `mcp_server_name`/`mcp_tool_name` à l'intérieur de `tool_parameters`, la même exception créée par l'hôte que `tool_decision`, nécessitant Claude Code v2.1.214 ou ultérieur
* `tool_decision` : conserve `tool_source` et un `tool_name` redacté au littéral `"mcp_tool"` pour les serveurs configurés par l'utilisateur, omet le contenu des arguments. Pour les serveurs intégrés de Claude Desktop, dans les sessions que Claude Desktop possède, il conserve également la paire `mcp_server_name`/`mcp_tool_name` à l'intérieur de `tool_parameters` ; `tool_source` et la paire de noms nécessitent tous deux Claude Code v2.1.214 ou ultérieur
* `mcp_server_connection` : omet `server_name` et le message d'erreur, mais conserve `is_plugin`, `plugin_id_hash`, et `plugin.name`, avec les noms de plugins non-Anthropic redactés au littéral `"third-party"`, de sorte que les serveurs fournis par les plugins restent distinguables sans journalisation détaillée

<h3 id="map-security-questions-to-events">
  Mapper les questions de sécurité aux événements
</h3>

Lors de la création de règles de détection, recherchez le signal que vous souhaitez surveiller et interrogez votre backend pour l'événement correspondant et les attributs :

| Signal                                                                                                                                       | Événement                                                                          | Attributs clés                                                                                                                                                                                                                                   |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Appel d'outil autorisé ou refusé, et par quoi                                                                                                | `tool_decision`                                                                    | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                             |
| Escalade du mode de permission                                                                                                               | `permission_mode_changed`                                                          | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                                |
| Hook de politique a bloqué une action                                                                                                        | `hook_execution_complete`                                                          | `hook_event`, `num_blocking`                                                                                                                                                                                                                     |
| Connexion, déconnexion et défaillance d'authentification                                                                                     | `auth`                                                                             | `action`, `success`, `error_category`                                                                                                                                                                                                            |
| Connexion du serveur MCP ou défaillance                                                                                                      | `mcp_server_connection`                                                            | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                               |
| Plugin installé et sa source                                                                                                                 | `plugin_installed`                                                                 | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                     |
| Commandes exécutées et fichiers touchés                                                                                                      | `tool_result` (exécuté) ou `tool_decision` (rejeté) avec `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters` ; `tool_input` (`tool_result` uniquement)                                                                                                                                                                                      |
| Quelles sources de paramètres gérés une machine exécute, si son assistant de politique est sain et pourquoi une machine a refusé de démarrer | `managed_settings_resolved`                                                        | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type` ; `managed_settings.settings` et `managed_settings.resolved_sha256` avec `OTEL_LOG_MANAGED_SETTINGS=1` |

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

Pour confirmer que les événements arrivent, soumettez une invite dans une session exécutée sous cette configuration et vérifiez votre SIEM pour l'événement `claude_code.user_prompt`. Si rien n'arrive, exécutez `claude --debug` et vérifiez le journal de débogage pour les erreurs d'exportation `[3P telemetry]`.

<h2 id="backend-considerations">
  Considérations relatives aux backends
</h2>

Votre choix de backends de métriques, de journaux et de traces détermine les types d'analyses que vous pouvez effectuer :

<h3 id="for-metrics">
  Pour les métriques
</h3>

* **Bases de données de séries chronologiques** : Calculs de taux, métriques agrégées
* **Magasins colonnaires** : Requêtes complexes, analyse d'utilisateurs uniques
* **Plates-formes d'observabilité complètes** : Requêtes avancées, visualisation, alertes

<h3 id="for-events/logs">
  Pour les événements/journaux
</h3>

* **Systèmes d'agrégation de journaux** : Recherche en texte intégral, analyse de journaux
* **Magasins colonnaires** : Analyse d'événements structurés
* **Plates-formes d'observabilité complètes** : Corrélation entre les métriques et les événements

<h3 id="for-traces">
  Pour les traces
</h3>

Choisissez un backend qui prend en charge le stockage de traces distribuées et la corrélation d'intervalles :

* **Systèmes de traçage distribué** : Visualisation d'intervalles, cascades de demandes, analyse de latence
* **Plates-formes d'observabilité complètes** : Recherche de traces et corrélation avec les métriques et les journaux

Pour les organisations nécessitant des métriques d'utilisateurs actifs quotidiens/hebdomadaires/mensuels (DAU/WAU/MAU), envisagez des backends qui prennent en charge les requêtes de valeurs uniques efficaces.

<h2 id="service-information">
  Informations sur le service
</h2>

Toutes les métriques et tous les événements sont exportés avec les attributs de ressource suivants :

* `service.name` : `claude-code` pour les sessions de terminal, `claude-code-desktop` pour les sessions démarrées à partir de l'onglet Code dans l'[application Claude Desktop](/docs/fr/desktop)
* `service.version` : Version actuelle de Claude Code, ou la version de l'application Desktop pour les sessions de l'onglet Code
* `os.type` : Type de système d'exploitation (par exemple, `linux`, `darwin`, `windows`)
* `os.version` : Chaîne de version du système d'exploitation
* `host.arch` : Architecture de l'hôte (par exemple, `amd64`, `arm64`)
* `wsl.version` : Numéro de version WSL (présent uniquement lors de l'exécution sur Windows Subsystem for Linux)
* Nom du compteur : `com.anthropic.claude_code`

Si vos pipelines de collecteur ou vos tableaux de bord filtrent sur `service.name = claude-code`, ajoutez `claude-code-desktop` au filtre pour capturer également la télémétrie des sessions de l'onglet Code.

<h2 id="roi-measurement-resources">
  Ressources de mesure du ROI
</h2>

Pour un guide complet sur la mesure du retour sur investissement pour Claude Code, y compris la configuration de la télémétrie, l'analyse des coûts, les métriques de productivité et les rapports automatisés, consultez le [Guide de mesure du ROI de Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Ce référentiel fournit des configurations Docker Compose prêtes à l'emploi, des configurations Prometheus et OpenTelemetry, et des modèles pour générer des rapports de productivité intégrés à des outils comme Linear.

<h2 id="security-and-privacy">
  Sécurité et confidentialité
</h2>

* L'export OpenTelemetry vers votre backend est opt-in et nécessite une configuration explicite. Pour la télémétrie opérationnelle distincte d'Anthropic et comment la désactiver, consultez [Utilisation des données](/docs/fr/data-usage#telemetry-services)
* Les contenus de fichiers bruts et les extraits de code ne sont pas inclus dans les métriques ou les événements. Les intervalles de trace constituent un chemin de données distinct : voir la puce `OTEL_LOG_TOOL_CONTENT` ci-dessous
* Lorsqu'authentifié via OAuth, `user.email` est inclus dans les attributs de télémétrie, envoyé uniquement au point de terminaison OTel que vous configurez, jamais à Anthropic. Si cela pose un problème pour votre organisation, travaillez avec votre backend de télémétrie pour filtrer ou masquer ce champ
* Le contenu des invites utilisateur n'est pas collecté par défaut. Seule la longueur de l'invite est enregistrée. Pour inclure le contenu de l'invite, définissez `OTEL_LOG_USER_PROMPTS=1`. Sous le traçage bêta détaillé, cette variable s'étend au-delà du texte d'invite : elle contrôle également l'[attribut d'intervalle `new_context`](#new-context-gates), qui porte les résultats d'outil sur l'intervalle `claude_code.llm_request`
* Le texte de réponse de l'assistant n'est pas collecté par défaut. Seule la longueur de la réponse est enregistrée. Pour inclure le texte de réponse, définissez `OTEL_LOG_ASSISTANT_RESPONSES=1`. Comme toutes les données OpenTelemetry de Claude Code, le texte de réponse est envoyé uniquement au point de terminaison OTel que vous configurez, jamais à Anthropic. Lorsque cette variable n'est pas définie, `OTEL_LOG_USER_PROMPTS` est utilisé comme solution de secours, donc définissez `OTEL_LOG_ASSISTANT_RESPONSES=0` si vous souhaitez le contenu de l'invite sans contenu de réponse
* Les arguments d'entrée d'outil et les paramètres ne sont pas enregistrés par défaut. Pour les inclure, définissez `OTEL_LOG_TOOL_DETAILS=1`. Pour les serveurs intégrés de Claude Desktop, dans les sessions que Claude Desktop possède, `tool_decision` et `tool_result` portent la paire `mcp_server_name`/`mcp_tool_name`, des noms créés par l'hôte plutôt que du contenu d'argument, même avec le drapeau désactivé. L'exception nécessite Claude Code v2.1.214 ou version ultérieure. Ces données sont envoyées uniquement au point de terminaison OTEL que vous configurez, jamais à Anthropic. Les arguments peuvent toujours contenir des valeurs sensibles, donc configurez votre backend de télémétrie pour filtrer ou masquer ces attributs selon les besoins. Lorsqu'activé :
  * Les événements `tool_result` et `tool_decision` incluent un attribut `tool_parameters` avec les commandes Bash, les noms de serveur MCP et d'outil, et les noms de compétences. Les champs tels que `full_command` sont émis sans troncature
  * Les événements `tool_result` incluent également un attribut `tool_input` avec les chemins de fichiers, les URL, les modèles de recherche et d'autres arguments. Les valeurs individuelles dépassant 512 caractères sont tronquées et le total est limité à environ 4 K caractères
  * Les événements `user_prompt` incluent le `command_name` verbatim pour les commandes personnalisées, de plugin et MCP
  * Les intervalles de trace incluent le même attribut `tool_input` et les attributs dérivés de l'entrée tels que `file_path`, avec la même troncature que `tool_input`
* Le contenu d'outil n'est pas enregistré dans les intervalles de trace par défaut. Pour l'inclure, définissez `OTEL_LOG_TOOL_CONTENT=1`. L'intervalle `claude_code.tool` porte alors un [événement d'intervalle `tool.output`](#tool-output-span-event) avec les contenus de fichiers bruts et la sortie de commande Bash, tronqués à la limite de contenu (60 Ko par défaut) par attribut. Le contenu d'outil atteint également les intervalles via [`new_context`, dont la porte diffère par intervalle](#new-context-gates). Configurez votre backend de télémétrie pour filtrer ou masquer ces attributs selon les besoins
* Les corps bruts de la demande et de la réponse de l'API Messages d'Anthropic ne sont pas enregistrés par défaut. Pour les inclure, définissez `OTEL_LOG_RAW_API_BODIES` dans votre shell, vos paramètres utilisateur ou vos paramètres gérés. Il est ignoré dans [les paramètres de projet et locaux](/docs/fr/settings-reference#variables-claude-code-ignores-in-env). Les corps contiennent l'historique complet de la conversation, y compris l'invite système, chaque tour d'utilisateur et d'assistant antérieur, et les résultats d'outils, donc l'activation de cette option implique le consentement à tout ce que les autres drapeaux de contenu `OTEL_LOG_*` révèleraient. Claude Code masque toujours le contenu de réflexion étendue de Claude de ces corps, indépendamment des autres paramètres. La valeur que vous définissez détermine comment Claude Code livre les corps :
  * Avec `=1`, Claude Code émet des événements de journaux `api_request_body` et `api_response_body` pour chaque appel d'API. L'attribut `body` des événements porte la charge utile sérialisée en JSON, tronquée à la limite de contenu (60 Ko par défaut)
  * Avec `=file:<dir>`, Claude Code écrit les corps non tronqués dans les fichiers `.request.json` et `.response.json` sous ce répertoire, et les événements portent un chemin `body_ref` à la place du corps en ligne. Livrez le répertoire avec un collecteur de journaux ou un sidecar plutôt que via le flux de télémétrie.

    Pour chaque réponse réussie, Claude Code ajoute également une ligne à `index.jsonl` dans ce répertoire, reliant le fichier de réponse au fichier de demande qui l'a produit et au message de transcription qu'il est devenu. Chaque ligne ne contient aucun contenu de message, et la section [événement du corps de réponse API](#api-response-body-event) énumère ses champs. Le fichier d'index nécessite Claude Code v2.1.274 ou version ultérieure

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Surveiller Claude Code sur Amazon Bedrock
</h2>

Pour des conseils détaillés sur la surveillance de l'utilisation de Claude Code pour Amazon Bedrock, consultez [Implémentation de la surveillance de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
