> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Alojamiento del Agent SDK

> Implementar el Agent SDK en producción: arquitectura de subprocesos, persistencia de sesiones, escalado, observabilidad y aislamiento multiinquilino para Docker, Kubernetes y proveedores de sandbox.

El Agent SDK genera y supervisa un subproceso `claude` CLI que posee un shell, un directorio de trabajo y archivos de sesión en disco. Alojarlo no es como alojar un contenedor de API sin estado. Cada agente en ejecución es un proceso de larga duración vinculado al estado local, lo que determina cómo asigna recursos, persiste sesiones y escala entre inquilinos.

Esta página cubre el autohospedaje en su propia infraestructura: comprenda [el modelo de subprocesos](#the-subprocess-model), [elija un patrón de sesión](#choose-a-session-pattern), [aprovisione el contenedor](#provision-the-container) y [maneje las preocupaciones de producción](#handle-production-concerns) como persistencia, observabilidad, autenticación y aislamiento multiinquilino. Para Dockerfiles e manifiestos de Kubernetes implementables, consulte el [manual de alojamiento](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Si no necesita control de infraestructura, aislamiento personalizado o su propio plano de datos, considere [Managed Agents](https://platform.claude.com/docs/es/managed-agents/overview) en su lugar: una API REST alojada donde Anthropic ejecuta el agente y el sandbox, por lo que su aplicación envía eventos y transmite resultados sin necesidad de operar infraestructura de alojamiento.

<Info>
  Para endurecimiento de seguridad más allá del sandboxing básico, incluidos controles de red, gestión de credenciales y opciones de aislamiento, consulte [Implementación Segura](/docs/es/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  El modelo de subproceso
</h2>

Cada decisión de alojamiento en esta página se deriva de cómo el SDK ejecuta el agente. Cuando su código llama a `query()`, el SDK genera un proceso CLI `claude` separado y se comunica con él a través de stdio. Ese subproceso posee el shell, el directorio de trabajo y las transcripciones de sesión JSONL en el disco local.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Flujo de solicitud: cliente a su aplicación, que genera un subproceso CLI de claude sobre stdio dentro del contenedor; el subproceso escribe en el disco local y llama a api.anthropic.com sobre HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Una sesión de agente se asigna a un subproceso. Ejecutar N sesiones concurrentes significa N subprocesos, cada uno con su propio árbol de procesos y archivo de transcripción. De forma predeterminada, todos heredan el directorio de trabajo de su aplicación, así que pase `cwd` en cada llamada a `query()` cuando las sesiones necesiten sistemas de archivos separados:

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  Estado que vive en el disco local
</h3>

Tres tipos de estado del agente viven en el sistema de archivos del contenedor de forma predeterminada. Ninguno de ellos sobrevive a un reinicio del contenedor, una reducción de escala o un movimiento a un nodo diferente.

| Estado                               | Ubicación predeterminada                                                                                         |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| Transcripciones de sesión            | `~/.claude/projects/`, o el directorio `projects/` bajo `CLAUDE_CONFIG_DIR` si está configurado                  |
| Archivos de memoria `CLAUDE.md`      | `~/.claude/CLAUDE.md` para el nivel de usuario y el directorio de trabajo de la sesión para el nivel de proyecto |
| Artefactos del directorio de trabajo | El directorio de trabajo de la sesión                                                                            |

Para persistir transcripciones entre hosts, configure un adaptador [`SessionStore`](/docs/es/agent-sdk/session-storage). Los archivos de memoria y otros artefactos del directorio de trabajo necesitan su propia estrategia de almacenamiento, como un volumen montado o una sincronización de almacén de objetos.

Para saber cómo funcionan las sesiones, la reanudación y la bifurcación a nivel de API, consulte [Sessions](/docs/es/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Elegir un patrón de sesión
</h2>

Estos cuatro patrones cubren el ciclo de vida de la sesión: cuánto tiempo vive un contenedor en relación con las sesiones que sirve. Para saber dónde se ejecuta el contenedor, el [manual de alojamiento](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) tiene [código desplegable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker local, Modal y Kubernetes. Elija un patrón de sesión aquí y un destino de implementación del manual.

<h3 id="ephemeral-sessions">
  Sesiones efímeras
</h3>

Cree un contenedor para cada tarea del usuario y destrúyalo cuando la tarea se complete. Lo mejor para tareas puntuales. El usuario aún puede interactuar con la IA mientras se completa la tarea, pero una vez completada, el contenedor se destruye.

Los ejemplos de cargas de trabajo incluyen investigación y corrección de errores, extracción de facturas y recibos, traducción de documentos y transformación de medios.

El contenedor ejecuta un punto de entrada de una sola ejecución que llama al SDK y sale. El ejemplo a continuación muestra una versión mínima de TypeScript. Guárdelo como `entrypoint.mts` o establezca `"type": "module"` en `package.json` para que `await` de nivel superior esté disponible.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Sesiones de larga duración
</h3>

Ejecute instancias de contenedor persistentes, a menudo alojando múltiples procesos del SDK por contenedor, para servir trabajo continuo. Lo mejor para agentes que toman acciones autónomas, sirven contenido o manejan flujos de mensajes de alto volumen.

Los ejemplos de cargas de trabajo incluyen un agente de correo electrónico que clasifica y responde al correo entrante, un constructor de sitios que aloja un sitio editable por usuario a través de puertos de contenedor, y un chatbot que maneja tráfico continuo desde una plataforma como Slack.

El contenedor expone un punto final HTTP o WebSocket y asigna cada sesión activa a una consulta de larga duración y el subproceso detrás de ella. En TypeScript, use [`streamInput()`](/docs/es/agent-sdk/typescript#query-object) para agregar turnos a una sesión activa y [`startup()`](/docs/es/agent-sdk/typescript#startup) para precalentar subprocesos antes del tráfico entrante. En Python, use [`ClaudeSDKClient`](/docs/es/agent-sdk/python#claudesdkclient) para mantener una sesión abierta entre turnos. Dimensione el contenedor para que pueda contener el número máximo de sesiones concurrentes en memoria.

<h3 id="hybrid-sessions">
  Sesiones híbridas
</h3>

Contenedores efímeros que se hidratan desde un [`SessionStore`](/docs/es/agent-sdk/session-storage) al inicio y persisten actualizaciones de vuelta. Lo mejor para sesiones que abarcan muchas interacciones pero permanecen inactivas entre ellas. El contenedor se apaga durante períodos de inactividad y se reinicia cuando el usuario regresa.

Los ejemplos de cargas de trabajo incluyen un gestor de proyectos personal con verificaciones intermitentes, investigación profunda que se pausa y reanuda durante horas, y un agente de soporte al cliente que carga el historial de tickets entre interacciones.

Ajuste el tiempo de espera de inactividad de su proveedor a la frecuencia con la que espera que los usuarios regresen. Apagar un contenedor sin un `SessionStore` configurado pierde la transcripción con él, por lo que el almacén es obligatorio para este patrón, no opcional.

El patrón se basa en reanudar una sesión por ID con un almacén compartido adjunto:

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

Consulte [Almacenamiento de sesiones](/docs/es/agent-sdk/session-storage) para la interfaz completa de `SessionStore` y adaptadores de referencia.

<h3 id="multi-agent-container">
  Contenedor multiagente
</h3>

Ejecute múltiples subprocesos del SDK dentro de un contenedor. Lo mejor para agentes que deben colaborar estrechamente, por ejemplo simulaciones multiagente donde los agentes interactúan entre sí en un entorno compartido.

Dé a cada agente su propio directorio de trabajo para que no sobrescriban los archivos de los demás, e aisle la carga de configuración para que los archivos `CLAUDE.md` por agente no se filtren entre agentes. Consulte [Aislamiento multiinquilino](#multi-tenant-isolation) para las opciones específicas.

<h2 id="provision-the-container">
  Aprovisionar el contenedor
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing basado en contenedor
</h3>

Ejecute el SDK dentro de un contenedor aislado para aislamiento de procesos, límites de recursos, control de red y un sistema de archivos efímero. Varios proveedores se especializan en entornos de contenedor aislado que se ajustan al modelo del Agent SDK.

Preguntas a responder al elegir un proveedor:

* **Quién ejecuta el sandbox**: un proveedor de sandbox-as-a-service opera la infraestructura para usted, mientras que las opciones autohospedadas le proporcionan software para ejecutar en su propio servidor.
* **Latencia de inicio en frío**: cuánto tiempo desde "crear un sandbox" hasta "listo para aceptar la primera solicitud". Los patrones efímeros necesitan inicios subsegundos. Los patrones de larga duración toleran más.
* **Almacenamiento persistente**: si el proveedor ofrece volúmenes duraderos o solo disco efímero. El patrón híbrido necesita almacenamiento duradero en algún lugar, ya sea en el sandbox o junto a él.
* **Modelo de precios**: facturación por segundo, por solicitud u horaria plana. La facturación por segundo se adapta bien a cargas de trabajo efímeras intermitentes. La facturación horaria se adapta a sesiones de larga duración.
* **Redes**: soporte para reglas de salida personalizadas, proxies salientes y emparejamiento privado de VPC para entornos regulados.

Proveedores a evaluar:

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), con una [implementación de demostración](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Para opciones autohospedadas como Docker, gVisor y Firecracker, y configuración de aislamiento detallada, consulte [Tecnologías de Aislamiento](/docs/es/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dependencias de tiempo de ejecución
</h3>

El contenedor necesita solo el tiempo de ejecución del idioma de su SDK:

* Python 3.10+ para el SDK de Python, o Node.js 18+ para el SDK de TypeScript
* Ambos paquetes SDK incluyen un binario nativo de Claude Code para la plataforma del host, por lo que no se necesita una instalación separada de Claude Code o Node.js para la CLI generada

El binario incluido está fijado a la versión del paquete SDK, por lo que actualizar el SDK es cómo actualiza la CLI. El SDK sigue semver: tome versiones de parche continuamente y revise el changelog de [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) o [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) antes de tomar una versión menor.

<h3 id="resources">
  Recursos
</h3>

1 GiB de RAM, 5 GiB de disco y 1 CPU por agente es un punto de partida razonable para una instancia recién iniciada. El uso de memoria crece con la duración de la sesión y la actividad de herramientas, por lo que dimensione para las duraciones de sesión y concurrencia que realmente necesita en lugar de la línea de base inactiva. Consulte [Escalado y concurrencia](#scaling-and-concurrency) para saber cómo calcular agentes por host.

<h3 id="network">
  Red
</h3>

El SDK necesita HTTPS saliente a `api.anthropic.com`, o al punto de conexión regional de su proveedor cuando se ejecuta en Amazon Bedrock o Google Cloud's Agent Platform. Si sus agentes utilizan [servidores MCP](/docs/es/agent-sdk/mcp) o herramientas externas, también necesitan acceso saliente a esos puntos de conexión. Para producción, enrute el tráfico saliente a través de un proxy de salida que aplique listas de permitidos de dominio, inyecte credenciales y registre solicitudes. Consulte [Implementación Segura](/docs/es/agent-sdk/secure-deployment) para el patrón completo.

Para tráfico entrante, exponga un puerto HTTP o WebSocket en el contenedor. Su aplicación maneja solicitudes de cliente en ese puerto y llama al SDK internamente; el subproceso en sí no escucha en la red.

<h2 id="handle-production-concerns">
  Gestionar preocupaciones de producción
</h2>

Trabaje a través de estas decisiones antes de desplegar un agente autohospedado.

<h3 id="session-and-state-persistence">
  Persistencia de sesión y estado
</h3>

El disco local predeterminado se pierde al reiniciar, reducir escala o mover a un nodo diferente. Para cualquier sesión que un usuario espere reanudar, refleje la transcripción en almacenamiento duradero con un adaptador [`SessionStore`](/docs/es/agent-sdk/session-storage). Consulte [Implementaciones de referencia](/docs/es/agent-sdk/session-storage#reference-implementations) para adaptadores de S3, Redis y Postgres y un conjunto de conformidad para el suyo.

Tres cosas que debe saber sobre cómo se comporta `SessionStore`:

* **Solo transcripciones**: `SessionStore` refleja transcripciones, no archivos de memoria `CLAUDE.md` u otros artefactos del directorio de trabajo. Monte un volumen compartido o sincronice esos por separado.
* **Reflejo, no reemplazo**: el subproceso escribe en el disco local primero, y el almacén recibe una copia de cada lote. Las escrituras locales siguen siendo autoritativas.
* **Mensajes `mirror_error`**: un lote que el almacén rechaza se envía hasta tres veces en total, con un retroceso corto antes de cada reintento; una llamada que agota el tiempo de espera no se reintenta. Si el lote aún falla, el SDK lo descarta, emite un mensaje `{ type: "system", subtype: "mirror_error" }` y continúa la consulta. Alerte sobre estos si la durabilidad del almacén es importante.

<h3 id="observability">
  Observabilidad
</h3>

Los agentes del SDK del agente son procesos de larga duración que generan llamadas de herramientas en muchos viajes de ida y vuelta de API. Sin telemetría, no puede ver qué herramientas se ejecutaron, cuánto tiempo tardaron o dónde se estancó una sesión.

El SDK hereda la configuración de OpenTelemetry del entorno. Establezca las variables de entorno OTEL a nivel de contenedor u orquestador para que cada llamada `query()` exporte tramos, métricas y eventos de registro a su recopilador. El ejemplo a continuación habilita la exportación OTLP para las tres señales. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` es necesario solo para trazas; omítalo si exporta solo métricas y registros.

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

El texto del mensaje y las entradas de herramientas no se incluyen en las exportaciones de forma predeterminada. Consulte [Controlar datos sensibles en exportaciones](/docs/es/agent-sdk/observability#control-sensitive-data-in-exports) para las banderas de inclusión voluntaria, y [Observabilidad](/docs/es/agent-sdk/observability) para el catálogo de señales completo.

<h3 id="auth-and-secrets">
  Autenticación y secretos
</h3>

Tres preocupaciones de autenticación importan en el momento del alojamiento:

* **API de Anthropic**: el subproceso lee `ANTHROPIC_API_KEY` de su entorno. Suministrelo desde su gestor de secretos, o establezca `ANTHROPIC_BASE_URL` para enrutar llamadas de modelo a través de un proxy que inyecte la clave fuera del contenedor. Consulte [Gestión de credenciales](/docs/es/agent-sdk/secure-deployment#credential-management) para el patrón de proxy y la [descripción general del SDK](/docs/es/agent-sdk/overview#get-started) para los métodos de autenticación admitidos.
* **Entrada**: coloque la autenticación en una puerta de enlace frente al contenedor del agente. El agente debe recibir solicitudes preauthenticadas y no debe ser el componente que valide los tokens del usuario.
* **Herramientas salientes**: mantenga las credenciales de herramientas fuera del entorno del agente. Enrute las llamadas salientes a través de un proxy que inyecte claves API después de que la solicitud salga del contenedor. El agente realiza la llamada; el proxy añade la credencial.

<h3 id="scaling-and-concurrency">
  Escalado y concurrencia
</h3>

Cada sesión se ejecuta en su propio subproceso, por lo que la concurrencia en un host está limitada por cuántos subprocesos puede contener su RAM.

Dimensione cada host con esta fórmula:

```text theme={null}
agentes por host = (RAM del host - sobrecarga) / (límite máximo de RAM por sesión)
```

Mida el límite máximo por sesión ejecutando una sesión representativa hasta su longitud objetivo bajo su carga de herramientas esperada y registrando el RSS máximo. El punto de partida de 1 GiB en [Recursos](#resources) es un piso, no el límite máximo.

El enrutamiento de escalado horizontal depende de su patrón. Para sesiones de larga duración, donde los contenedores contienen muchas sesiones, ejecute un grupo de contenedores detrás de un equilibrador de carga y fije cada sesión a un contenedor usando hash consistente en `sessionId`. Una sesión fijada sigue golpeando el mismo contenedor y, por lo tanto, el mismo subproceso en ejecución, hasta que se desaloja o el contenedor se reinicia.

Los grandes abanicos de [subagentes](/docs/es/agent-sdk/subagents) concurrentes desde una única sesión pueden alcanzar límites de velocidad de API. Divida el trabajo en lotes más pequeños en lugar de emitir un envío amplio.

<h3 id="cost">
  Costo
</h3>

El costo de tokens de Anthropic típicamente domina el costo de infraestructura del contenedor por un orden de magnitud o más. Un contenedor mínimamente aprovisionado se ejecuta aproximadamente a \$0.05 por hora, mientras que una única sesión de agente largo puede gastar dólares en tokens. Consulte [Seguimiento de costos](/docs/es/agent-sdk/cost-tracking) para contabilidad de tokens por sesión.

<h3 id="multi-tenant-isolation">
  Aislamiento multiinquilino
</h3>

El comportamiento predeterminado del SDK lee configuraciones y archivos de memoria `CLAUDE.md` del sistema de archivos. En un contenedor compartido que sirve a múltiples inquilinos, esos archivos pueden filtrar el contexto de un inquilino a la sesión de otro inquilino.

Para aislar inquilinos dentro de un contenedor compartido:

* Pase `settingSources: []` en TypeScript o `setting_sources=[]` en Python para que no se cargue ninguna configuración del sistema de archivos.
* Establezca `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` en `env`. [Auto memory](/docs/es/memory#auto-memory) en `~/.claude/projects/<project>/memory/` se carga en el mensaje del sistema independientemente de `settingSources`. Consulte [Lo que settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para las otras entradas que se cargan incondicionalmente.
* Apunte `CLAUDE_CONFIG_DIR` a un directorio por inquilino para que los inquilinos no compartan la configuración global `~/.claude.json`.
* Use un directorio de trabajo por inquilino. Pase `cwd` explícitamente en cada llamada `query()`.
* Aplique reglas de salida por inquilino en su proxy, como IPs salientes distintas, credenciales o listas de permitidos de dominio, para que un inquilino comprometido no pueda exfiltrar datos a través de la política de salida de otro inquilino.

El ejemplo a continuación aplica las cuatro opciones a nivel de SDK juntas. Construya `tenantDir` y `configDir` para que cada inquilino obtenga una ruta que ningún otro inquilino pueda leer. En TypeScript, `env` reemplaza el entorno del subproceso, por lo que extienda `...process.env` para mantener variables heredadas como `PATH` y `ANTHROPIC_API_KEY`. En Python, `env` se fusiona en la parte superior del entorno heredado.

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

Para controles de red por inquilino, consulte [Despliegue Seguro](/docs/es/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Limitaciones conocidas
</h2>

Planifique alrededor de estas en su diseño de implementación.

| Limitación                                                                           | Qué hacer                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sin tiempo de espera de sesión de nivel superior                                     | Una sesión no agota el tiempo de espera por sí sola. Establezca `maxTurns` en `Options` para limitar cuántos viajes de uso de herramientas realiza el agente antes de detenerse.                                                                                                                                                                 |
| Crecimiento de memoria en sesiones largas                                            | Limite la duración de la sesión o recicle subprocesos periódicamente. Consulte [Escalado y concurrencia](#scaling-and-concurrency).                                                                                                                                                                                                              |
| Los despliegues paralelos grandes de subagentos pueden alcanzar límites de velocidad | Divida el trabajo en lotes más pequeños en lugar de emitir un envío amplio.                                                                                                                                                                                                                                                                      |
| Sin plazo de reloj de pared por subagentos                                           | Limite cada [subagentos](/docs/es/agent-sdk/subagents) con `maxTurns` en su `AgentDefinition`. Solo para subagentos en segundo plano, `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` establece un perro guardián de estancamiento que se activa cuando un subagentos `run_in_background` deja de producir salida; no es un plazo de tiempo de ejecución total. |

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Guía de alojamiento](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): recorrido por el notebook con [código implementable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker, Modal y Kubernetes.
* [Almacenamiento de sesiones](/docs/es/agent-sdk/session-storage): persistir transcripciones entre hosts con un adaptador `SessionStore`.
* [Observabilidad](/docs/es/agent-sdk/observability): exportar trazas OTEL, métricas y registros a su recopilador.
* [Implementación segura](/docs/es/agent-sdk/secure-deployment): controles de red, gestión de credenciales y endurecimiento de aislamiento.
* [Seguimiento de costos](/docs/es/agent-sdk/cost-tracking): contabilidad de tokens y costos por sesión.
