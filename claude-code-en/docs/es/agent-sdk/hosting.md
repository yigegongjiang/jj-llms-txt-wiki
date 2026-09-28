> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Alojamiento del Agent SDK

> Implemente el Agent SDK en producción: arquitectura de subprocesos, persistencia de sesiones, escalado, observabilidad e aislamiento multiinquilino para Docker, Kubernetes y proveedores de sandbox.

El Agent SDK genera y supervisa un subproceso `claude` CLI que posee un shell, un directorio de trabajo y archivos de sesión en disco. Alojarlo no es como alojar un contenedor de API sin estado. Cada agente en ejecución es un proceso de larga duración vinculado al estado local, lo que determina cómo asigna recursos, persiste sesiones y escala entre inquilinos.

Esta página cubre el autohospedaje en su propia infraestructura. Para Dockerfiles e manifiestos de Kubernetes implementables, consulte el [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Si no necesita ejecutar el bucle del agente en su propia infraestructura, considere [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview) en su lugar. Anthropic aloja el bucle del agente, y su aplicación envía eventos y recibe resultados transmitidos a través de los SDK de cliente o la API REST. La ejecución de herramientas se ejecuta en un sandbox en la nube administrado por Anthropic o en un [sandbox autohospedado](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) en su propia infraestructura.

<h2 id="the-subprocess-model">
  El modelo de subproceso
</h2>

Cada decisión de alojamiento en esta página se deriva de cómo el SDK ejecuta el agente. Cuando su código llama a `query()`, el SDK genera un proceso CLI `claude` separado y se comunica con él a través de stdio. Ese subproceso posee el shell, el directorio de trabajo y las transcripciones de sesión JSONL en el disco local.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="Flujo de solicitud: cliente a su aplicación, que genera un subproceso CLI de claude sobre stdio dentro del contenedor; el subproceso escribe en el disco local y llama a api.anthropic.com sobre HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="Flujo de solicitud: cliente a su aplicación, que genera un subproceso CLI de claude sobre stdio dentro del contenedor; el subproceso escribe en el disco local y llama a api.anthropic.com sobre HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

Una sesión de agente se asigna a un subproceso. Ejecutar N sesiones concurrentes significa N subprocesos, cada uno con su propio árbol de procesos y archivo de transcripción. De forma predeterminada, todos heredan el directorio de trabajo de su aplicación. Cuando las sesiones necesitan sistemas de archivos separados, pase un `cwd` distinto en las opciones de la llamada `query()` de cada sesión:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Los ejemplos de TypeScript en esta página utilizan `await` de nivel superior, así que guárdelos como archivos `.mts` o establezca `"type": "module"` en `package.json`.

<h3 id="state-that-lives-on-local-disk">
  Estado que vive en el disco local
</h3>

Tres tipos de estado de agente viven en el sistema de archivos del contenedor de forma predeterminada. Ninguno de ellos sobrevive a un reinicio del contenedor, una reducción de escala o un movimiento a un nodo diferente.

| Estado                               | Ubicación predeterminada                                                                                         |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| Transcripciones de sesión            | `~/.claude/projects/`, o el directorio `projects/` bajo `CLAUDE_CONFIG_DIR` si está establecido                  |
| Archivos de memoria `CLAUDE.md`      | `~/.claude/CLAUDE.md` para el nivel de usuario y el directorio de trabajo de la sesión para el nivel de proyecto |
| Artefactos del directorio de trabajo | El directorio de trabajo de la sesión                                                                            |

Para persistir transcripciones entre hosts, configure un adaptador [`SessionStore`](/docs/es/agent-sdk/session-storage). Los archivos de memoria y otros artefactos del directorio de trabajo necesitan su propia estrategia de almacenamiento, como un volumen montado o una sincronización de almacén de objetos.

Para saber cómo funcionan las sesiones, la reanudación y la bifurcación a nivel de API, consulte [Sesiones](/docs/es/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Elegir un patrón de sesión
</h2>

Estos cuatro patrones cubren el ciclo de vida de la sesión: cuánto tiempo vive un contenedor en relación con las sesiones que sirve. Para saber dónde se ejecuta el contenedor, el [manual de alojamiento](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) tiene [código desplegable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker local, Modal y Kubernetes. Elija un patrón de sesión aquí y un destino de implementación del manual.

<h3 id="ephemeral-sessions">
  Sesiones efímeras
</h3>

Cree un contenedor para cada tarea del usuario y destrúyalo cuando se complete la tarea. Lo mejor para tareas puntuales. El usuario aún puede interactuar con la IA mientras se completa la tarea, pero una vez completada, el contenedor se destruye.

Los ejemplos de cargas de trabajo incluyen investigación y corrección de errores, extracción de facturas y recibos, traducción de documentos y transformación de medios.

El contenedor ejecuta un punto de entrada de una sola ejecución que lee la tarea de la variable de entorno `TASK_PROMPT`, llama al SDK y sale.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

El script imprime cada mensaje a medida que llega, incluido un mensaje de resultado cuyo `subtype` es `success` cuando la tarea se completa dentro del límite de turnos. Si la tarea alcanza el límite de 20 turnos en su lugar, el `subtype` del mensaje de resultado es `error_max_turns` y la llamada `query()` genera un error después de cederlo, así que envuelva el bucle en un bloque try si el contenedor necesita salir limpiamente. Consulte [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result) para los subtipos de error.

<h3 id="long-running-sessions">
  Sesiones de larga duración
</h3>

Ejecute instancias de contenedor persistentes, a menudo alojando múltiples procesos SDK por contenedor, para servir trabajo continuo. Lo mejor para agentes que toman acciones autónomas, sirven contenido o manejan flujos de mensajes de alto volumen.

Los ejemplos de cargas de trabajo incluyen un agente de correo electrónico que clasifica y responde al correo entrante, un constructor de sitios que aloja un sitio editable por usuario a través de puertos de contenedor, y un chatbot que maneja tráfico continuo desde una plataforma como Slack.

El contenedor expone un punto final HTTP o WebSocket y asigna cada sesión activa a una consulta de larga duración y el subproceso detrás de ella. En TypeScript, use [`streamInput()`](/docs/es/agent-sdk/typescript#query-object) para agregar turnos a una sesión activa y [`startup()`](/docs/es/agent-sdk/typescript#startup) para precalentar subprocesos antes del tráfico entrante. En Python, use [`ClaudeSDKClient`](/docs/es/agent-sdk/python#claudesdkclient) para mantener una sesión abierta entre turnos. Dimensione el contenedor para que pueda contener el número máximo de sesiones concurrentes en memoria.

<h3 id="hybrid-sessions">
  Sesiones híbridas
</h3>

Contenedores efímeros que se hidratan desde un [`SessionStore`](/docs/es/agent-sdk/session-storage) al inicio y persisten actualizaciones de vuelta. Lo mejor para sesiones que abarcan muchas interacciones pero permanecen inactivas entre ellas. El contenedor se apaga durante períodos de inactividad y se reinicia cuando el usuario regresa.

Los ejemplos de cargas de trabajo incluyen un gestor de proyectos personal con check-ins intermitentes, investigación profunda que se pausa y reanuda durante horas, y un agente de soporte al cliente que carga el historial de tickets entre interacciones.

Ajuste el tiempo de espera de inactividad de su proveedor a la frecuencia con la que espera que los usuarios regresen. Apagar un contenedor sin un `SessionStore` configurado pierde la transcripción con él, por lo que el almacén es obligatorio para este patrón, no opcional.

El patrón se basa en reanudar una sesión por ID con un almacén compartido adjunto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  Contenedor multiagente
</h3>

Ejecute múltiples subprocesos SDK dentro de un contenedor. Lo mejor para agentes que deben colaborar estrechamente, por ejemplo simulaciones multiagente donde los agentes interactúan entre sí en un entorno compartido.

Dé a cada agente su propio directorio de trabajo para que no sobrescriban los archivos de los demás, y aisle la carga de configuración para que los archivos `CLAUDE.md` por agente no se filtren entre agentes. Consulte [Aislamiento multiinquilino](#multi-tenant-isolation) para las opciones específicas.

<h2 id="provision-the-container">
  Aprovisionar el contenedor
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing basado en contenedores
</h3>

Ejecute el SDK dentro de un contenedor sandboxed para aislamiento de procesos, límites de recursos, control de red y un sistema de archivos efímero.

Preguntas a responder al elegir un proveedor:

* **Quién ejecuta el sandbox**: un proveedor de sandbox-as-a-service opera la infraestructura para usted, mientras que las opciones autohospedadas le proporcionan software para ejecutar en su propio servidor.
* **Latencia de arranque en frío**: cuánto tiempo transcurre desde "crear un sandbox" hasta "listo para aceptar la primera solicitud". Los patrones efímeros necesitan inicios subsegundos. Los patrones de larga duración toleran más.
* **Almacenamiento persistente**: si el proveedor ofrece volúmenes duraderos o solo disco efímero. El patrón híbrido necesita almacenamiento duradero en algún lugar, ya sea en el sandbox o junto a él.
* **Modelo de precios**: facturación por segundo, por solicitud u horaria plana. Los precios por segundo se adaptan bien a cargas de trabajo efímeras intermitentes. Los precios por hora se adaptan a sesiones de larga duración.
* **Redes**: soporte para reglas de salida personalizadas, proxies de salida y emparejamiento privado de VPC para entornos regulados.

Para opciones autohospedadas como Docker, gVisor y Firecracker, y configuración de aislamiento detallada, consulte [Isolation Technologies](/docs/es/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dependencias de tiempo de ejecución
</h3>

El contenedor necesita el tiempo de ejecución del lenguaje de su SDK:

* Python 3.10+ para el SDK de Python, o Node.js 18+ para el SDK de TypeScript
* Tanto los SDK de TypeScript como de Python incluyen un binario nativo de Claude Code para la mayoría de las instalaciones, y la CLI generada no necesita una instalación separada de Node.js. Consulte la [nota de instalación del quickstart](/docs/es/agent-sdk/quickstart) para las instalaciones que necesitan una instalación separada de Claude Code nativo.

El binario incluido está fijado a la versión del paquete SDK, por lo que actualizar el SDK es cómo actualiza la CLI. El SDK sigue semver: tome versiones de parche continuamente y revise el changelog de [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) o [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) antes de tomar una versión menor.

<h3 id="resources">
  Recursos
</h3>

1 GiB de RAM, 5 GiB de disco y 1 CPU por agente es un punto de partida razonable para una instancia recién iniciada. El uso de memoria crece con la duración de la sesión y la actividad de herramientas, por lo que dimensione para las duraciones de sesión y concurrencia que realmente necesita en lugar de la línea de base inactiva. Consulte [Scaling and concurrency](#scaling-and-concurrency) para saber cómo calcular agentes por host.

<h3 id="network">
  Red
</h3>

El SDK necesita HTTPS de salida a `api.anthropic.com`, o al punto de conexión regional de su proveedor cuando se ejecuta en Amazon Bedrock o en la plataforma de agentes de Google Cloud. Si sus agentes utilizan [MCP servers](/docs/es/agent-sdk/mcp) o herramientas externas, también necesitan acceso de salida a esos puntos de conexión. Para producción, enrute el tráfico de salida a través de un proxy de salida que aplique listas de permitidos de dominio, inyecte credenciales y registre solicitudes. Consulte [Secure Deployment](/docs/es/agent-sdk/secure-deployment) para el patrón completo.

Para el tráfico de entrada, exponga un puerto HTTP o WebSocket en el contenedor. Su aplicación maneja las solicitudes del cliente en ese puerto y llama al SDK internamente; el subproceso en sí no escucha en la red.

<h2 id="handle-production-concerns">
  Gestionar preocupaciones de producción
</h2>

Trabaje a través de estas decisiones antes de implementar un agente autohospedado.

<h3 id="session-and-state-persistence">
  Persistencia de sesión y estado
</h3>

El disco local predeterminado se pierde al reiniciar, reducir escala o mover a un nodo diferente. Para cualquier sesión que un usuario espere reanudar, refleje la transcripción en almacenamiento duradero con un adaptador [`SessionStore`](/docs/es/agent-sdk/session-storage). Consulte [Implementaciones de referencia](/docs/es/agent-sdk/session-storage#reference-implementations) para adaptadores de ejemplo para un almacén de objetos, un almacén de pares clave-valor y una base de datos, y un conjunto de conformidad para el suyo.

Tres cosas que debe saber sobre cómo se comporta `SessionStore`:

* **Solo transcripciones**: `SessionStore` refleja transcripciones, no archivos de memoria `CLAUDE.md` u otros artefactos del directorio de trabajo. Monte un volumen compartido o sincronice esos por separado.
* **Reflejo, no reemplazo**: el subproceso escribe en el disco local primero, y el SDK reenvía una copia de cada lote al almacén. La transcripción local de una sesión nueva sobrevive a la ejecución; una ejecución reanudada desde el almacén elimina su copia local al final, por lo que el almacén contiene la única copia durable. Consulte [Arquitectura de escritura dual](/docs/es/agent-sdk/session-storage#dual-write-architecture).
* **Mensajes `mirror_error`**: cuando el SDK no puede entregar un lote al almacén, descarta el lote, emite un mensaje `{ type: "system", subtype: "mirror_error" }` y continúa la consulta. Alerte sobre estos si la durabilidad del almacén es importante. Consulte [Las escrituras de reflejo son de mejor esfuerzo](/docs/es/agent-sdk/session-storage#mirror-writes-are-best-effort) para el comportamiento de reintentos y tiempos de espera.

<h3 id="observability">
  Observabilidad
</h3>

Los agentes del Agent SDK son procesos de larga duración que generan llamadas de herramientas en muchos viajes de ida y vuelta de API. Sin telemetría, no puede ver qué herramientas se ejecutaron, cuánto tiempo tardaron o dónde se estancó una sesión.

El SDK hereda la configuración de OpenTelemetry del entorno. Establezca las variables de entorno OTEL a nivel de contenedor u orquestador para que cada llamada `query()` exporte tramos, métricas y eventos de registro a su recopilador. El ejemplo a continuación habilita la exportación OTLP para las tres señales. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` es necesario solo para trazas; omítalo si exporta solo métricas y registros.

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

El texto del mensaje y las entradas de herramientas no se incluyen en las exportaciones de forma predeterminada. Consulte [Controlar datos sensibles en exportaciones](/docs/es/agent-sdk/observability#control-sensitive-data-in-exports) para las banderas de inclusión, y [Observabilidad](/docs/es/agent-sdk/observability) para el catálogo de señales completo.

<h3 id="auth-and-secrets">
  Autenticación y secretos
</h3>

Tres preocupaciones de autenticación importan en el momento del alojamiento:

* **API de Anthropic**: el subproceso lee `ANTHROPIC_API_KEY` de su entorno. Suministrelo desde su gestor de secretos, o establezca `ANTHROPIC_BASE_URL` para enrutar llamadas de modelo a través de un proxy que inyecte la clave fuera del contenedor. Consulte [Gestión de credenciales](/docs/es/agent-sdk/secure-deployment#credential-management) para el patrón de proxy y [Configuración en el inicio rápido del SDK](/docs/es/agent-sdk/quickstart#setup) para los métodos de autenticación admitidos.
* **Entrada**: coloque la autenticación en una puerta de enlace frente al contenedor del agente. El agente debe recibir solicitudes preauthenticadas y no debe ser el componente que valide los tokens de usuario.
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

<h3 id="cost">
  Costo
</h3>

El costo de tokens de Anthropic típicamente domina el costo de infraestructura del contenedor por un orden de magnitud o más. Un contenedor mínimamente aprovisionado cuesta aproximadamente \$0.05 por hora, mientras que una única sesión de agente largo puede gastar dólares en tokens. Consulte [Seguimiento de costos](/docs/es/agent-sdk/cost-tracking) para contabilidad de tokens por sesión.

<h3 id="multi-tenant-isolation">
  Aislamiento multiinquilino
</h3>

El comportamiento predeterminado del SDK lee configuración y archivos de memoria `CLAUDE.md` del sistema de archivos. En un contenedor compartido que sirve a múltiples inquilinos, esos archivos pueden filtrar el contexto de un inquilino a la sesión de otro inquilino.

Para aislar inquilinos dentro de un contenedor compartido:

* Pase `settingSources: []` en TypeScript o `setting_sources=[]` en Python para omitir la configuración de usuario, proyecto y local.
* Establezca `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` en `env`. [Auto memory](/docs/es/memory#auto-memory) en `~/.claude/projects/<project>/memory/` se carga en el mensaje del sistema independientemente de `settingSources`. Consulte [Lo que settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para las otras entradas que se cargan incondicionalmente.
* Apunte `CLAUDE_CONFIG_DIR` a un directorio por inquilino para que los inquilinos no compartan la configuración global `~/.claude.json`. Cuando cada directorio de configuración sirve un directorio de trabajo y no comparte un [`SessionStore`](/docs/es/agent-sdk/session-storage) entre inquilinos, también puede establecer [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/es/sessions#name-the-project-directory-yourself) en `env` para mantener las rutas de transcripción bajo él cortas. Requiere Agent SDK de TypeScript v0.3.234 o posterior, o Agent SDK de Python v0.2.140 o posterior.
* Use un directorio de trabajo por inquilino. Pase `cwd` explícitamente en cada llamada `query()`.
* Aplique reglas de salida por inquilino en su proxy, como IPs salientes distintas, credenciales o listas de permitidos de dominio, para que un inquilino comprometido no pueda exfiltrar datos a través de la política saliente de otro inquilino.

El ejemplo a continuación aplica las opciones de configuración, memoria automática, directorio de configuración y directorio de trabajo juntas. Construya `tenantDir` y `configDir` para que cada inquilino obtenga una ruta que ningún otro inquilino pueda leer. En TypeScript, `env` reemplaza el entorno del subproceso, por lo que extienda `...process.env` para mantener variables heredadas como `PATH` y `ANTHROPIC_API_KEY`. En Python, `env` se fusiona en la parte superior del entorno heredado.

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
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
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


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  Limitaciones conocidas
</h2>

Planifique considerando estas limitaciones en su diseño de implementación.

| Limitación                                                                       | Qué hacer                                                                                                                                                                                                                                                                           |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sin tiempo de espera de sesión de nivel superior                                 | Una sesión no agota el tiempo de espera por sí sola. Establezca `maxTurns` en TypeScript o `max_turns` en Python para limitar cuántos viajes de ronda de uso de herramientas realiza el agente antes de detenerse.                                                                  |
| Crecimiento de memoria en sesiones largas                                        | Limite la duración de la sesión o recicle subprocesos periódicamente. Consulte [Escalado y concurrencia](#scaling-and-concurrency).                                                                                                                                                 |
| Los fanouts de subagentes paralelos grandes pueden alcanzar límites de velocidad | Divida el trabajo en lotes más pequeños en lugar de emitir un envío amplio.                                                                                                                                                                                                         |
| Sin plazo de reloj de pared por subagente                                        | Limite cada [subagente](/docs/es/agent-sdk/subagents) con `maxTurns` en su `AgentDefinition`. `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` establece un perro guardián de estancamiento que se activa cuando un subagente deja de producir salida; no es un plazo de tiempo de ejecución total. |

<h2 id="troubleshoot-deployment-failures">
  Solucionar errores de implementación
</h2>

Utilice esta sección cuando un agente que funciona en su máquina falla en un servicio implementado. Cada elemento a continuación nombra un error y vincula la entrada que lo cubre:

* **CLI no encontrado al iniciar el servicio**: en Python, un contenedor o administrador de servicios ejecuta su aplicación con una `PATH` diferente a la de su shell, por lo que una instalación que funciona localmente no es visible para el proceso. En TypeScript, la compilación de la imagen omitió las dependencias opcionales del SDK, o `pathToClaudeCodeExecutable` apunta a un archivo que no existe en la imagen. Consulte [Claude Code no encontrado](/docs/es/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found).
* **CLI presente en la imagen pero no se inicia**: Claude Code no puede iniciarse desde un binario que no coincida con la arquitectura o libc del contenedor, o desde un archivo que perdió su permiso de ejecución en la compilación de la imagen. Consulte [Error al iniciar Claude Code](/docs/es/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code).
* **El proceso de Claude Code se cierra durante la ejecución**: el error que recibe su aplicación depende del lenguaje del SDK y de si la CLI reportó primero un resultado de error. Las entradas bajo [Salida del proceso CLI](/docs/es/agent-sdk/troubleshooting#cli-process-exit) cubren cada mensaje.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Guía de alojamiento](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): recorrido por el notebook con [código implementable](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker, Modal y Kubernetes.
* [Almacenamiento de sesiones](/docs/es/agent-sdk/session-storage): persistir transcripciones entre hosts con un adaptador `SessionStore`.
* [Observabilidad](/docs/es/agent-sdk/observability): exportar trazas OTEL, métricas y registros a su recopilador.
* [Implementación segura](/docs/es/agent-sdk/secure-deployment): controles de red, gestión de credenciales y endurecimiento de aislamiento.
* [Seguimiento de costos](/docs/es/agent-sdk/cost-tracking): contabilidad de tokens y costos por sesión.
