> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Observabilité avec OpenTelemetry

> Exportez les traces, les métriques et les événements du SDK Agent vers votre backend d'observabilité en utilisant OpenTelemetry.

Lorsque vous exécutez des agents en production, vous avez besoin de visibilité sur ce qu'ils ont fait :

* quels outils ils ont appelés
* combien de temps chaque requête de modèle a pris
* combien de tokens ont été dépensés
* où les défaillances se sont produites

Le SDK Agent peut exporter ces données sous forme de traces OpenTelemetry, de métriques et d'événements de journal vers n'importe quel backend qui accepte le protocole OpenTelemetry (OTLP), tel que Honeycomb, Datadog, Grafana, Langfuse ou un collecteur auto-hébergé.

Ce guide explique comment le SDK émet la télémétrie, comment configurer l'export et comment étiqueter et filtrer les données une fois qu'elles atteignent votre backend. Pour lire l'utilisation des tokens et les coûts directement à partir du flux de réponse du SDK au lieu d'exporter vers un backend, consultez [Suivre les coûts et l'utilisation](/docs/fr/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Comment la télémétrie circule depuis le SDK
</h2>

Le SDK Agent exécute l'interface de ligne de commande Claude Code en tant que processus enfant et communique avec elle via un tuyau local. L'interface de ligne de commande dispose d'une instrumentation OpenTelemetry intégrée : elle enregistre les spans autour de chaque requête de modèle et exécution d'outil, émet des métriques pour les compteurs de tokens et de coûts, et émet des événements de journal structurés pour les invites et les résultats d'outils. Le SDK ne produit pas de télémétrie propre. Au lieu de cela, il transmet la configuration au processus CLI, et l'interface de ligne de commande exporte directement vers votre collecteur.

La configuration est transmise sous forme de variables d'environnement. Par défaut, le processus enfant hérite de l'environnement de votre application, vous pouvez donc configurer la télémétrie dans l'un de ces deux endroits :

* **Environnement du processus :** définissez les variables dans votre shell, conteneur ou orchestrateur avant le démarrage de votre application. Chaque appel `query()` les récupère automatiquement sans modification du code. C'est l'approche recommandée pour les déploiements en production.
* **Options par appel :** définissez les variables dans `ClaudeAgentOptions.env` (Python) ou `options.env` (TypeScript). Utilisez ceci lorsque différents agents dans le même processus ont besoin de paramètres de télémétrie différents. En Python, `env` est fusionné au-dessus de l'environnement hérité. En TypeScript, `env` remplace complètement l'environnement hérité, donc incluez `...process.env` dans l'objet que vous transmettez.

L'interface de ligne de commande exporte trois signaux OpenTelemetry indépendants. Chacun a son propre commutateur d'activation et son propre exportateur, vous pouvez donc activer uniquement ceux dont vous avez besoin.

| Signal                | Ce qu'il contient                                                                                | Activer avec                                                        |
| --------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------- |
| Métriques             | Compteurs pour les tokens, les coûts, les sessions, les lignes de code et les décisions d'outils | `OTEL_METRICS_EXPORTER`                                             |
| Événements de journal | Enregistrements structurés pour chaque invite, requête API, erreur API et résultat d'outil       | `OTEL_LOGS_EXPORTER`                                                |
| Traces                | Spans pour chaque interaction, requête de modèle, appel d'outil et hook (bêta)                   | `OTEL_TRACES_EXPORTER` plus `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Pour la liste complète des noms de métriques, des noms d'événements et des attributs, consultez la référence Claude Code [Monitoring](/docs/fr/monitoring-usage). Le SDK Agent émet les mêmes données car il exécute la même interface de ligne de commande. Les noms de spans sont listés dans [Lire les traces de l'agent](#read-agent-traces) ci-dessous.

<h2 id="enable-telemetry-export">
  Activer l'export de télémétrie
</h2>

La télémétrie est désactivée jusqu'à ce que vous définissiez `CLAUDE_CODE_ENABLE_TELEMETRY=1` et choisissiez au moins un exportateur. La configuration la plus courante envoie les trois signaux via OTLP HTTP à un collecteur.

L'exemple suivant définit les variables dans un dictionnaire et les transmet via `options.env`. L'agent exécute une seule tâche, et l'interface de ligne de commande exporte les spans, les métriques et les événements vers le collecteur à `collector.example.com` tandis que la boucle consomme le flux de réponse :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # Required for traces, which are in beta. Metrics and log events do not need this.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # Choose an exporter per signal. Use otlp for the SDK; see the Note below.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # Standard OTLP transport configuration.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // Required for traces, which are in beta. Metrics and log events do not need this.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // Choose an exporter per signal. Use otlp for the SDK; see the Note below.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // Standard OTLP transport configuration.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env replaces the inherited environment in TypeScript, so spread
    // process.env first to keep PATH, ANTHROPIC_API_KEY, and other variables.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Comme le processus enfant hérite de l'environnement de votre application par défaut, vous pouvez obtenir le même résultat en exportant ces variables dans un Dockerfile, un manifeste Kubernetes ou un profil shell et en omettant complètement `options.env`.

<Note>
  L'exportateur `console` écrit la télémétrie sur la sortie standard, que le SDK utilise
  comme canal de message. Ne définissez pas `console` comme valeur d'exportateur lors de l'exécution
  via le SDK. Pour inspecter la télémétrie localement, pointez
  `OTEL_EXPORTER_OTLP_ENDPOINT` vers un collecteur local ou un conteneur
  Jaeger tout-en-un à la place.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Vider la télémétrie des appels de courte durée
</h3>

L'interface de ligne de commande regroupe la télémétrie et exporte à intervalles réguliers. À la fermeture propre du processus, elle tente de vider les données en attente, mais le vidage est limité par un délai d'attente court, les spans peuvent donc toujours être supprimés si le collecteur répond lentement. Si votre processus est tué avant l'arrêt de l'interface de ligne de commande, tout ce qui se trouve encore dans le tampon de lot est perdu. La réduction des intervalles d'export réduit les deux fenêtres.

Par défaut, les métriques s'exportent toutes les 60 secondes et les traces et les journaux s'exportent toutes les 5 secondes. L'exemple suivant raccourcit les trois intervalles afin que les données atteignent le collecteur pendant qu'une tâche courte s'exécute toujours :

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... exporter configuration from the previous example ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... exporter configuration from the previous example ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  Lire les traces de l'agent
</h2>

Les traces vous donnent la vue la plus détaillée d'une exécution d'agent. Avec `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` défini, chaque étape de la boucle de l'agent devient un span que vous pouvez inspecter dans votre backend de traçage :

* **`claude_code.interaction`:** enveloppe un seul tour de la boucle de l'agent, de la réception d'une invite à la production d'une réponse.
* **`claude_code.llm_request`:** enveloppe chaque appel à l'API Claude, avec le nom du modèle, la latence et les comptages de tokens comme attributs.
* **`claude_code.tool`:** enveloppe chaque invocation d'outil, avec des spans enfants pour l'attente de permission (`claude_code.tool.blocked_on_user`) et l'exécution elle-même (`claude_code.tool.execution`).
* **`claude_code.hook`:** enveloppe chaque exécution de [hook](/docs/fr/agent-sdk/hooks). Nécessite un traçage bêta détaillé (`ENABLE_BETA_TRACING_DETAILED=1` et `BETA_TRACING_ENDPOINT`) en plus des variables ci-dessus.

Les spans `llm_request`, `tool` et `hook` sont des enfants du span `claude_code.interaction` englobant. Lorsque l'agent génère un sous-agent via l'outil Task, les spans `llm_request` et `tool` du sous-agent se nichent sous le span `claude_code.tool` de l'agent parent, la chaîne de délégation complète apparaît donc comme une seule trace.

Les spans portent un attribut `session.id` par défaut. Lorsque vous effectuez plusieurs appels `query()` contre la même [session](/docs/fr/agent-sdk/sessions), filtrez sur `session.id` dans votre backend pour les voir comme une seule chronologie. L'attribut est omis si `OTEL_METRICS_INCLUDE_SESSION_ID` est défini sur une valeur fausse.

<Note>
  Le traçage est en bêta. Les noms et attributs des spans peuvent changer entre les versions. Consultez
  [Traces (bêta)](/docs/fr/monitoring-usage#traces-beta) dans la référence Monitoring
  pour les variables de configuration de l'exportateur de traces.
</Note>

<h2 id="link-traces-to-your-application">
  Lier les traces à votre application
</h2>

Le SDK propage automatiquement le contexte de trace W3C dans le sous-processus CLI. Lorsque vous appelez `query()` tandis qu'un span OpenTelemetry est actif dans votre application, le SDK injecte `TRACEPARENT` et `TRACESTATE` dans l'environnement du processus enfant, et l'interface de ligne de commande les lit afin que son span `claude_code.interaction` devienne un enfant de votre span. L'exécution de l'agent apparaît alors à l'intérieur de la trace de votre application au lieu d'être une racine déconnectée.

Lorsque la propagation du contexte de trace est activée, l'interface de ligne de commande transfère également `TRACEPARENT` à chaque commande Bash et PowerShell qu'elle exécute. Si une commande lancée via l'outil Bash émet ses propres spans OpenTelemetry, ces spans se nichent sous le span `claude_code.tool.execution` qui enveloppe la commande.

L'injection automatique est ignorée lorsque vous définissez `TRACEPARENT` explicitement dans `options.env`, vous pouvez donc épingler un contexte parent spécifique si nécessaire. Les sessions CLI interactives ignorent complètement le `TRACEPARENT` entrant ; seules les exécutions du SDK Agent et `claude -p` l'honorent. Consultez [Traces (bêta)](/docs/fr/monitoring-usage#traces-beta) dans la référence Monitoring pour la référence complète des spans et des attributs.

<h2 id="tag-telemetry-from-your-agent">
  Étiqueter la télémétrie de votre agent
</h2>

Par défaut, l'interface de ligne de commande rapporte `service.name` comme `claude-code`. Si vous exécutez plusieurs agents ou exécutez le SDK aux côtés d'autres services qui exportent vers le même collecteur, remplacez le nom du service et ajoutez des attributs de ressource afin de pouvoir filtrer par agent dans votre backend.

L'exemple suivant renomme le service et joint les métadonnées de déploiement. Ces valeurs sont appliquées comme attributs de ressource OpenTelemetry sur chaque span, métrique et événement que l'agent émet :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  Attribuer les actions à vos utilisateurs finaux
</h2>

L'interface de ligne de commande joint les [attributs d'identité](/docs/fr/monitoring-usage#standard-attributes) à chaque événement en fonction de la credential qu'elle utilise pour appeler Anthropic. Lorsque vous créez une application qui sert de nombreux utilisateurs finaux à partir d'un seul déploiement, ces attributs identifient la credential de votre service, pas l'utilisateur final au nom duquel l'agent a agi.

Pour rendre les appels d'outils et l'activité MCP attribuables aux utilisateurs finaux de votre application, injectez l'identité de l'utilisateur final en tant qu'attributs de ressource sur chaque appel `query()`. Encodez en pourcentage les valeurs avant de les interpoler, car `OTEL_RESOURCE_ATTRIBUTES` [réserve les virgules, les espaces et les signes égal](/docs/fr/monitoring-usage#multi-team-organization-support). L'exemple suivant joint l'utilisateur demandeur et le locataire à chaque span et événement d'une requête. Il suppose un objet `request` de votre framework web portant les identifiants d'utilisateur et de locataire :

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

Avec l'identité de l'utilisateur final attachée, les événements `tool_decision`, `tool_result`, `mcp_server_connection` et `permission_mode_changed`, qui s'exportent en tant qu'enregistrements de journal nommés avec un préfixe `claude_code.`, deviennent une piste d'audit par utilisateur que vous pouvez transférer à une plateforme SIEM (Security Information and Event Management). Consultez [Audit des événements de sécurité](/docs/fr/monitoring-usage#audit-security-events) dans la référence Monitoring pour la liste complète des événements pertinents pour la sécurité et les attributs que chacun porte.

<h2 id="control-sensitive-data-in-exports">
  Contrôler les données sensibles dans les exports
</h2>

La télémétrie est structurelle par défaut. Les durées, les noms de modèles et les noms d'outils sont enregistrés sur chaque span ; les comptages de tokens sont enregistrés lorsque la requête API sous-jacente retourne les données d'utilisation, les spans pour les requêtes échouées ou abandonnées peuvent donc les omettre. Le contenu que votre agent lit et écrit n'est pas enregistré par défaut. Ces variables opt-in ajoutent du contenu aux données exportées :

| Variable                  | Ajoute                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Texte d'invite sur les événements `claude_code.user_prompt` et sur le span `claude_code.interaction`                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `OTEL_LOG_TOOL_DETAILS=1` | Arguments d'entrée d'outil (chemins de fichiers, commandes shell, modèles de recherche) sur les événements `claude_code.tool_result`                                                                                                                                                                                                                                                                                                                                                                                                               |
| `OTEL_LOG_TOOL_CONTENT=1` | Corps d'entrée et de sortie d'outil complets en tant qu'événements de span sur `claude_code.tool`, tronqués à 60 Ko. Nécessite que le [traçage](#read-agent-traces) soit activé                                                                                                                                                                                                                                                                                                                                                                    |
| `OTEL_LOG_RAW_API_BODIES` | Requête et réponse JSON complètes de l'API Anthropic Messages en tant qu'événements de journal `claude_code.api_request_body` et `claude_code.api_response_body`. Définissez sur `1` pour les corps en ligne tronqués à 60 Ko, ou `file:<dir>` pour les corps non tronqués sur disque avec un chemin `body_ref` dans l'événement. Les corps incluent l'historique de conversation complet et le contenu de la réflexion étendue est masqué. L'activation de ceci implique le consentement à tout ce que les trois variables ci-dessus révèleraient |

Laissez ces variables non définies à moins que votre pipeline d'observabilité ne soit approuvé pour stocker les données que votre agent traite. Consultez [Sécurité et confidentialité](/docs/fr/monitoring-usage#security-and-privacy) dans la référence Monitoring pour la liste complète des attributs et du comportement de masquage.

<h2 id="related-documentation">
  Documentation connexe
</h2>

Ces guides couvrent des sujets adjacents pour la surveillance et le déploiement d'agents :

* [Suivre les coûts et l'utilisation](/docs/fr/agent-sdk/cost-tracking) : lire les données de tokens et de coûts à partir du flux de messages sans backend externe.
* [Héberger le SDK Agent](/docs/fr/agent-sdk/hosting) : déployer des agents dans des conteneurs où vous pouvez définir les variables OpenTelemetry au niveau de l'environnement.
* [Monitoring](/docs/fr/monitoring-usage) : la référence complète pour chaque variable d'environnement, métrique et événement que l'interface de ligne de commande émet.
