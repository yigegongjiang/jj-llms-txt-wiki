> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración del modelo

> Aprenda sobre la configuración del modelo Claude Code, incluidos los alias de modelo como `opusplan`

<h2 id="available-models">
  Modelos disponibles
</h2>

Para la configuración de `model` en Claude Code, puede configurar:

* Un **alias de modelo**
* Un **nombre de modelo**
  * API de Anthropic: Un **[nombre de modelo](https://platform.claude.com/docs/es/about-claude/models/overview)** completo
  * Amazon Bedrock: un ARN de perfil de inferencia
  * Microsoft Foundry: un nombre de implementación
  * Plataforma de agentes de Google Cloud: un nombre de versión

Para obtener orientación sobre qué modelo y nivel de esfuerzo se ajustan a diferentes tipos de trabajo, consulte [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) en el blog.

<Note>
  `ANTHROPIC_BASE_URL` cambia dónde se envían las solicitudes, no qué modelo las responde. Para enrutar Claude a través de una puerta de enlace LLM, consulte [puertas de enlace LLM](/docs/es/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias de modelo
</h3>

Los alias de modelo proporcionan una forma conveniente de seleccionar configuraciones de modelo sin necesidad de recordar números de versión exactos:

| Alias de modelo  | Comportamiento                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Valor especial que borra cualquier anulación de modelo y revierte al modelo recomendado para su tipo de cuenta, o al [modelo predeterminado de la organización](#organization-default-model) cuando un administrador ha establecido uno. No es en sí mismo un alias de modelo                                                                                                    |
| **`best`**       | Utiliza Fable 5 donde su organización tiene acceso a él, de lo contrario el último modelo Opus                                                                                                                                                                                                                                                                                   |
| **`fable`**      | Utiliza Claude Fable 5 para sus tareas más difíciles y de mayor duración                                                                                                                                                                                                                                                                                                         |
| **`sonnet`**     | Utiliza el último modelo Sonnet para tareas de codificación diaria                                                                                                                                                                                                                                                                                                               |
| **`opus`**       | Utiliza el último modelo Opus para tareas de razonamiento complejo                                                                                                                                                                                                                                                                                                               |
| **`haiku`**      | Utiliza el modelo Haiku rápido y eficiente para tareas simples                                                                                                                                                                                                                                                                                                                   |
| **`sonnet[1m]`** | Utiliza Sonnet con una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/es/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas. Sin efecto cuando `sonnet` ya se resuelve a Sonnet 5 con su ventana nativa de 1M; detrás de una [puerta de enlace LLM](/docs/es/llm-gateway), selecciona la ventana de 1M para Sonnet 5 |
| **`opus[1m]`**   | Utiliza Opus con una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/es/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas                                                                                                                                                                                       |
| **`opusplan`**   | Modo especial que utiliza `opus` durante Plan Mode, luego cambia a `sonnet` para la ejecución                                                                                                                                                                                                                                                                                    |

La versión a la que se resuelven los alias `opus` y `sonnet` depende del proveedor:

| Proveedor                                             | `opus`   | `sonnet`   |
| :---------------------------------------------------- | :------- | :--------- |
| API de Anthropic                                      | Opus 4.8 | Sonnet 5   |
| [Claude Platform on AWS](/docs/es/claude-platform-on-aws)  | Opus 4.8 | Sonnet 4.6 |
| Amazon Bedrock, Plataforma de agentes de Google Cloud | Opus 4.8 | Sonnet 4.5 |
| Microsoft Foundry                                     | Opus 4.6 | Sonnet 4.5 |

Donde un alias se resuelve a un modelo más antiguo, hay modelos más nuevos disponibles seleccionando explícitamente el nombre de modelo completo o estableciendo `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Antes de v2.1.207, `opus` se resolvía a Opus 4.7 en Claude Platform on AWS y a Opus 4.6 en Amazon Bedrock y Plataforma de agentes de Google Cloud.

Los alias apuntan a la versión recomendada para su proveedor y se actualizan con el tiempo. Para fijar una versión específica, utilice el nombre de modelo completo, por ejemplo `claude-opus-4-8`, o establezca la variable de entorno correspondiente como `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Sonnet 5 requiere Claude Code v2.1.197 o posterior. Opus 4.8 requiere v2.1.154 o posterior. Ejecute `claude update` para actualizar.
</Note>

<h3 id="work-with-fable-5">
  Trabajar con Fable 5
</h3>

[Claude Fable 5](https://platform.claude.com/docs/es/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) es el modelo más capaz en Claude Code, adecuado para tareas más grandes que una sola sesión. Sustenta sesiones largas autónomas, investiga antes de actuar y verifica su trabajo más a menudo que los modelos más pequeños.

Fable 5 no es el modelo predeterminado. Selecciónelo con `/model fable`. Las solicitudes que sus clasificadores de seguridad marcan, más a menudo en dominios de ciberseguridad y biología, activan [alternancia automática de modelo](#automatic-model-fallback).

Para aprovechar al máximo Fable 5:

* **Describa el resultado, no los pasos**: entrégale el resultado que desea y déjelo planificar el camino. Para mantenerlo funcionando hasta que ese resultado se cumpla, [establezca un objetivo](/docs/es/goal).
* **Entrégale problemas ambiguos**: las investigaciones de causa raíz, la depuración de interrupciones y las decisiones de arquitectura son donde la investigación y verificación adicionales se rentabilizan.
* **Omita los recordatorios de verificación**: verifica su propio trabajo con menos indicaciones, por lo que los recordatorios para probar o verificar generalmente son innecesarios.
* **Dimensione tareas más grandes**: entrégale trabajo que normalmente dividiría en partes. Mantiene sesiones largas sin perder el hilo.

<Note>
  Fable 5 requiere Claude Code v2.1.170 o posterior. Las versiones anteriores no muestran Fable 5 en el selector de modelo y no pueden seleccionarlo. Ejecute `claude update` para actualizar. Fable 5 no está disponible bajo [retención de datos cero](/docs/es/zero-data-retention), donde el selector `/model` lo omite o lo muestra deshabilitado.
</Note>

<h3 id="setting-your-model">
  Configurar su modelo
</h3>

Puede configurar su modelo de varias formas, enumeradas en orden de prioridad:

1. **Durante la sesión**: utilice `/model <alias|name>` para cambiar inmediatamente, o ejecute `/model` sin argumentos para abrir el selector. El selector solicita confirmación cuando la conversación tiene salida anterior, ya que la siguiente respuesta relee el historial completo sin contexto en caché
2. **Al inicio**: inicie con `claude --model <alias|name>`
3. **Variable de entorno**: establezca `ANTHROPIC_MODEL=<alias|name>`
4. **Configuración**: configure permanentemente en su archivo de configuración utilizando el campo `model`

A partir de v2.1.153, `/model` guarda su selección como predeterminada para nuevas sesiones escribiendo el campo `model` en su configuración de usuario. En el selector:

* `Enter`: cambiar modelo y guardar como predeterminado
* `s`: cambiar modelo solo para esta sesión

Escribir `/model <name>` directamente se comporta como `Enter`. Un modelo establecido con `/model` en [modo no interactivo](/docs/es/headless), con la bandera `-p`, se aplica solo a la sesión actual y no se guarda como su predeterminado. La configuración del proyecto y administrada aún tiene prioridad y se reaplicará en el siguiente lanzamiento. Un [modelo predeterminado de la organización](#organization-default-model) que su administrador ha configurado para anular la selección del usuario también se reaplicará en el siguiente lanzamiento.

En v2.1.144 a v2.1.152, `/model` se aplicaba solo a la sesión actual y `d` en el selector guardaba un predeterminado.

La bandera `--model` y la variable de entorno `ANTHROPIC_MODEL` se aplican solo a la sesión que inicia con ellas. Para ejecutar diferentes modelos en diferentes terminales al mismo tiempo, inicie cada uno con su propia bandera `--model` en lugar de cambiar con `/model`.

Los precios en el selector `/model` aparecen cuando Claude Code se comunica con la API de Anthropic, directamente o a través de una [puerta de enlace LLM](/docs/es/llm-gateway) que la proxifica, y el precio en una fila es el precio del modelo que esa fila selecciona. En [proveedores de terceros](/docs/es/third-party-integrations) como Amazon Bedrock y en la [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), su proveedor o puerta de enlace determina lo que paga, por lo que las filas del selector no muestran precio. El precio es solo una etiqueta de visualización; no afecta qué modelo selecciona una fila o qué factura su proveedor. Antes de v2.1.206, [Claude Platform on AWS](/docs/es/claude-platform-on-aws) y las sesiones de puerta de enlace mostraban precios de lista de Anthropic, y una fila podría mostrar el precio de un modelo diferente al que seleccionaba.

Las sesiones reanudadas iniciadas con `claude --resume`, `--continue`, o el selector `/resume` mantienen el modelo que estaban usando cuando se guardó la transcripción, independientemente de la configuración actual de `model`. Si ese modelo ha sido retirado o es excluido por [`availableModels`](#restrict-model-selection), la sesión cae en el orden de precedencia normal. Esto evita que la selección de `/model` de otra sesión cambie el modelo al reanudar.

Un modelo que elige para el nuevo lanzamiento con `--model` o `ANTHROPIC_MODEL` aún tiene prioridad sobre el modelo restaurado. A partir de v2.1.195, también lo hace una variable de familia [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables).

Cuando el modelo activo al inicio proviene de la configuración del proyecto o administrada en lugar de su propia selección, el encabezado de inicio muestra qué archivo de configuración lo estableció. Ejecute `/model` para anular; la configuración del proyecto o administrada se reaplicará en el siguiente lanzamiento.

Cuando se solicita un cambio de modelo a través del método `setModel()` del [Agent SDK](/docs/es/agent-sdk/overview) o por una aplicación como la [aplicación de escritorio](/docs/es/desktop) que ejecuta Claude Code CLI para usted, Claude Code verifica que la cadena sea una que reconozca antes de guardarla. Esta verificación requiere Claude Code v2.1.200 o posterior. En la API de Anthropic, Claude Code reconoce:

* un alias de modelo
* una entrada del selector `/model`
* cualquier nombre que comience con `claude-`
* un valor que configuró usted mismo como una [opción de modelo personalizado](#add-a-custom-model-option) o en [`modelOverrides`](#override-model-ids-per-version)

Claude Code rechaza una cadena no reconocida con `Model "<name>" is not a recognized model id.` y la sesión mantiene su modelo actual, en lugar de guardar la cadena y fallar en la siguiente solicitud. Consulte [la referencia de errores](/docs/es/errors#model-is-not-a-recognized-model-id) para pasos de recuperación.

La verificación se ejecuta solo en la API de Anthropic. En Amazon Bedrock, Plataforma de agentes de Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/es/claude-platform-on-aws), y detrás de una [puerta de enlace LLM](/docs/es/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizado, su proveedor o puerta de enlace define los nombres de modelo, por lo que Claude Code pasa cualquier cadena sin verificarla. La verificación tampoco cubre la bandera `--model`, la variable de entorno `ANTHROPIC_MODEL`, o la configuración `model`; un valor mal escrito allí produce [There's an issue with the selected model](/docs/es/errors#theres-an-issue-with-the-selected-model) en la primera solicitud en su lugar.

Cuando el modelo solicitado tiene una fecha de retiro programada o se remapea automáticamente a una versión más nueva, Claude Code muestra una advertencia que nombra el modelo solicitado. Las sesiones interactivas la muestran como un aviso de inicio. A partir de v2.1.182, la misma advertencia se escribe en stderr en [modo no interactivo](/docs/es/headless) cuando se utiliza el formato de salida de texto predeterminado. La verificación también cubre un `model` establecido en [frontmatter de subagentos](/docs/es/sub-agents). La advertencia de stderr se suprime para `--output-format json` y `stream-json`; lea el modelo real desde el campo `modelUsage` del [mensaje de resultado](/docs/es/headless#get-structured-output) en su lugar.

Ejemplo de uso:

```bash theme={null}
# Iniciar con Opus
claude --model opus

# Cambiar a Sonnet durante la sesión
/model sonnet
```

Archivo de configuración de ejemplo:

```json theme={null}
{
    "permissions": {
        ...
    },
    "model": "opus"
}
```

<h2 id="restrict-model-selection">
  Restringir la selección de modelo
</h2>

Los administradores empresariales pueden utilizar `availableModels` en [configuración administrada o de política](/docs/es/settings#settings-files) para restringir qué modelos pueden seleccionar los usuarios. Las entradas coinciden con una familia de modelos como `sonnet`, un prefijo de versión como `claude-sonnet-4-5`, o un ID de modelo completo como `claude-sonnet-4-5-20250929`.

Cuando se establece `availableModels`, la lista de permitidos se aplica en todas partes donde un usuario puede especificar un modelo:

* **Modelo de sesión principal**: `/model`, la bandera `--model`, la variable de entorno `ANTHROPIC_MODEL`, la configuración `model`, y el modelo restaurado cuando [se reanuda una sesión](#setting-your-model)
* **Resolución de alias**: las variables de entorno `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, y `ANTHROPIC_DEFAULT_FABLE_MODEL` no pueden redirigir un alias permitido a un modelo fuera de la lista
* **Modo rápido**: `/fast` se niega a cambiar cuando cambiaría implícitamente a un modelo Opus fuera de la lista, con el mensaje "no está en los modelos permitidos de su organización"
* **Modelos de subagentes**: el campo `model` en [frontmatter de subagente](/docs/es/sub-agents#choose-a-model), el parámetro `model` de la herramienta Agent, `CLAUDE_CODE_SUBAGENT_MODEL`, y, en v2.1.197 y anterior, el selector de modelo en el asistente `/agents`&#x20;
* **Modelo de habilidad y comando**: el frontmatter `model` en [habilidades y comandos](/docs/es/skills)
* **Modelo de asesor**: la configuración [`advisorModel`](/docs/es/advisor) configurada y la bandera `--advisor`
* **Modelo de agente de fondo**: el modelo seleccionado en el [selector de envío](/docs/es/agent-view)

En la API de Anthropic y [Claude Platform en AWS](/docs/es/claude-platform-on-aws), un alias de familia de modelos, `opus`, `sonnet`, `haiku`, o `fable`, se resuelve a la versión más reciente de su familia que la lista de permitidos permite. Cuando la lista de permitidos fija versiones específicas, por ejemplo `["sonnet", "claude-opus-4-6"]`, tanto `/model opus` como `--model opus` seleccionan Claude Opus 4.6, el Opus más reciente permitido, y muestran un aviso que nombra tanto los modelos solicitados como los sustituidos. Antes de v2.1.205, un alias cuya versión más reciente lanzada estaba fuera de la lista se rechazaba o reemplazaba como cualquier otra selección bloqueada, incluso cuando la lista permitía una versión anterior.

Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, y [Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) utilizan IDs de implementación específicos del proveedor en lugar de IDs de modelo de Anthropic, por lo que un alias bloqueado allí sigue el comportamiento de rechazo y reemplazo a continuación.

Claude Code maneja cualquier otra selección bloqueada según dónde se estableció el modelo:

* **`/model`**: el cambio se rechaza con un error
* **Bandera `--model`, `ANTHROPIC_MODEL`, o la configuración `model`**: el valor se reemplaza al inicio con una advertencia que nombra tanto los modelos solicitados como los sustituidos, y la sesión comienza en el modelo predeterminado
* **Anulación de subagente, habilidad o comando**: la anulación vuelve al modelo heredado o predeterminado en lugar de fallar la solicitud
* **Configuración `advisorModel`**: el asesor se desactiva para la sesión
* **Bandera `--advisor`**: Claude Code sale con un error al inicio

Los modelos excluidos se ocultan del selector `/model`. Un ID de modelo completo en la lista que no tiene una fila de selector integrada, como una versión anterior que la lista fija, aparece en el selector `/model` como su propia fila etiquetada. Antes de v2.1.199, ese ID era seleccionable solo escribiendo `/model <id>`.

Los cambios de modelo que Claude Code realiza en su nombre se verifican de la misma manera:

* **[Cadenas de modelo de reserva](#fallback-model-chains)**: los elementos fuera de la lista de permitidos se descartan
* **Actualizaciones de modo de plan**: en la API de Anthropic y Claude Platform en AWS, una actualización como [`opusplan`](#opusplan-model-setting) a un modelo excluido utiliza la versión más reciente permitida de la familia de actualización. En proveedores con IDs de modelo específicos del proveedor, y cuando no se permite ninguna versión, la actualización se omite y la planificación continúa en el modelo de la sesión
* **[Fallback automático de modelo](#automatic-model-fallback)**: un fallback cuyo destino está excluido no se ejecuta, por lo que la solicitud marcada termina con un rechazo
* **[Modo rápido](/docs/es/fast-mode)**: habilitar el modo rápido se rechaza cuando el modelo en el que se ejecutaría la sesión después está fuera de la lista de permitidos

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Cobertura de superficie
</h3>

Cada superficie aplica la lista de permitidos que recibe. El mecanismo de entrega que llega a cada superficie difiere:

| Mecanismo de entrega                                                                                      | CLI e IDE | Sesiones locales de escritorio | Sesiones web, móviles y en la nube | Agent SDK y no interactivo | Cowork                       |
| :-------------------------------------------------------------------------------------------------------- | :-------- | :----------------------------- | :--------------------------------- | :------------------------- | :--------------------------- |
| [Configuración administrada por servidor](/docs/es/server-managed-settings) desde la consola de administración | Aplicada  | Aplicada                       | Aplicada                           | Aplicada                   | No entregada                 |
| [MDM o archivos de configuración administrada](/docs/es/settings#settings-files)                               | Aplicada  | Aplicada                       | No entregada                       | Aplicada                   | Aplicada donde se implementa |

* Las sesiones en la nube, en [Claude Code en la web](/docs/es/claude-code-on-the-web) o en la aplicación de escritorio, se ejecutan en máquinas virtuales administradas por Anthropic: la configuración implementada en su dispositivo no las alcanza, por lo que entregue la lista de permitidos a través de configuración administrada por servidor. Un cambio de modelo a mitad de sesión en una sesión en la nube se rechaza cuando el modelo solicitado está excluido por la lista de permitidos. El rechazo del lado del servidor en la creación de sesión se aplica a [restricciones de modelo de organización](#organization-model-restrictions), no a la clave de configuración `availableModels`.
* Cowork, la pestaña de trabajo agéntico en la aplicación Claude Desktop, no es una superficie de Claude Code y no recibe configuración administrada por servidor por diseño. Un archivo de configuración administrada se aplica a sesiones de Cowork cuando está presente donde se ejecuta la sesión; las sesiones remotas de Cowork se ejecutan en máquinas virtuales administradas por Anthropic, donde un archivo implementado en el dispositivo no está presente.
* Las sesiones en [proveedores de terceros](/docs/es/server-managed-settings#platform-availability) como Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, y [Claude Platform en AWS](/docs/es/claude-platform-on-aws) no reciben configuración administrada por servidor, por lo que entregue la lista de permitidos a través de MDM o archivos de configuración administrada allí.
* La entrega administrada por servidor también requiere que la sesión se autentique con un inicio de sesión de organización o una clave API configurada directamente. Las flotas que generan claves solo a través de un script [`apiKeyHelper`](/docs/es/settings#available-settings) deben entregar la lista de permitidos a través de MDM o archivos de configuración administrada.
* La pestaña Código de escritorio también aloja [sesiones SSH](/docs/es/desktop#ssh-sessions), que leen el archivo de configuración administrada del host remoto en el que se ejecutan. Consulte [Configuración administrada de escritorio](/docs/es/desktop#managed-settings).
* Los selectores de modelo en claude.ai y en la aplicación de escritorio ocultan o atenúan los modelos excluidos por la lista de permitidos de su organización. El estado del selector es una conveniencia para los usuarios; la aplicación ocurre en la sesión.

<h3 id="default-model-behavior">
  Comportamiento del modelo predeterminado
</h3>

La opción Predeterminado en el selector de modelo no se ve afectada por `availableModels` a menos que [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) también esté establecido. Por sí solo, `availableModels` deja Predeterminado disponible, resolviéndose al [valor predeterminado de tiempo de ejecución del sistema](#default-model-setting) para la cuenta. Si ese valor predeterminado es un modelo que tiene la intención de restringir, establezca `enforceAvailableModels` también.

Un array `availableModels` vacío nunca activa la aplicación del modelo Predeterminado: con `availableModels: []`, las selecciones de modelo nombrado se bloquean pero el modelo Predeterminado para el tipo de cuenta permanece utilizable independientemente de `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Aplicar la lista de permitidos para el modelo Predeterminado
</h3>

Establezca `enforceAvailableModels: true` junto con una `availableModels` no vacía en configuración administrada para extender la lista de permitidos a la opción Predeterminado. Esto requiere Claude Code v2.1.175 o posterior.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

La opción Predeterminado se resuelve a la cuenta-tipo predeterminado, o al [modelo predeterminado de la organización](#organization-default-model) cuando un administrador ha establecido uno. Cuando ese modelo no está en la lista de permitidos, la opción Predeterminado se resuelve a la primera entrada `availableModels` que nombra un modelo permitido y disponible, y la fila Predeterminado del selector `/model` muestra ese modelo. Esto se aplica en todas partes donde se alcanza el valor predeterminado: inicio de sesión, seleccionar Predeterminado en `/model`, la palabra clave `"default"` en [cadenas de modelo de reserva](#fallback-model-chains), y el fallback utilizado cuando se descarta una selección excluida.

`enforceAvailableModels` no tiene efecto cuando `availableModels` no está establecido o está vacío: con `availableModels: []`, el modelo Predeterminado para el tipo de cuenta permanece utilizable, por lo que la configuración no puede bloquear a los usuarios de cada modelo. Cuando `availableModels` no está vacío pero ninguna entrada se resuelve a un modelo permitido y disponible, la aplicación se degrada y Predeterminado cae al valor predeterminado del tipo de cuenta, con una advertencia visible solo bajo `--debug`. Mantenga al menos una entrada garantizada disponible en la lista para evitar esto.

Implemente ambas claves en la [fuente administrada de mayor precedencia](/docs/es/settings#settings-precedence): las fuentes administradas implementadas por el administrador no se fusionan, por lo que un par colocado en un archivo de configuración administrada se ignora cuando la consola de administración entrega cualquier configuración.

<h3 id="control-the-model-users-run-on">
  Controlar el modelo en el que se ejecutan los usuarios
</h3>

La configuración `model` es una selección inicial, no una aplicación. Establece qué modelo está activo cuando comienza una sesión, pero los usuarios aún pueden abrir `/model` y elegir Predeterminado, que se resuelve al [valor predeterminado de tiempo de ejecución del sistema](#default-model-setting) independientemente de lo que esté configurado en `model`, a menos que [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) lo redirija.

Para controlar completamente la experiencia del modelo, combine estas configuraciones:

* **`availableModels`**: restringe a qué modelos nombrados pueden cambiar los usuarios
* **`enforceAvailableModels`**: extiende la lista de permitidos `availableModels` a la opción Predeterminado, de modo que Predeterminado no pueda resolverse a un modelo fuera de la lista
* **`model`**: establece la selección de modelo inicial cuando comienza una sesión
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: controlan a qué se resuelven la opción Predeterminado y los alias `sonnet`, `opus`, `haiku` y `fable`

Este ejemplo inicia a los usuarios en Sonnet 4.5, limita el selector a Sonnet y Haiku, y asegura que Predeterminado se resuelva a un modelo en la lista de permitidos en lugar del valor predeterminado del nivel:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Sin `enforceAvailableModels` o el bloque `env`, un usuario que seleccione Predeterminado en el selector obtendría la versión más reciente para su nivel, omitiendo la fijación de versión en `model` y `availableModels`. Las dos configuraciones cubren diferentes alcances: `enforceAvailableModels` hace que Predeterminado obedezca la lista de permitidos, mientras que el bloque `env` fija qué versión resuelve un alias permitido como `sonnet`. Use `enforceAvailableModels` solo cuando restringir familias de modelos sea suficiente; agregue el bloque `env` cuando también necesite fijar una versión específica.

<h3 id="merge-behavior">
  Comportamiento de fusión
</h3>

Cuando la [fuente de configuración administrada de mayor precedencia](/docs/es/server-managed-settings#settings-precedence) define `availableModels`, esa lista sola se aplica: las entradas en configuración de usuario, proyecto o local no pueden extenderla, y las fuentes administradas implementadas por el administrador no se fusionan entre sí, por lo que una lista implementada en un archivo de configuración administrada se ignora cuando la configuración administrada por servidor entrega cualquier clave. De lo contrario, las listas de configuración de usuario, proyecto y local se [concatenan y desduplican](/docs/es/settings#settings-precedence) como otras configuraciones de array. A partir de Claude Code v2.1.175, la lista administrada reemplaza entradas de menor precedencia; las versiones anteriores las fusionan.

Dentro de la lista efectiva, una entrada que nombra un modelo específico en una familia, ya sea un prefijo de versión o un ID de modelo completo, desactiva la entrada comodín de esa familia: `["sonnet", "claude-sonnet-4-5"]` permite solo versiones de Sonnet 4.5, no cada modelo de Sonnet.

<h3 id="mantle-model-ids">
  IDs de modelo Mantle
</h3>

Cuando el [punto final Bedrock Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) está habilitado, las entradas en `availableModels` que comienzan con `anthropic.` se agregan al selector `/model` como opciones personalizadas y se enrutan al punto final Mantle. Esta es una excepción a la coincidencia de alias descrita en [Fijar modelos para implementaciones de terceros](#pin-models-for-third-party-deployments). La configuración aún restringe el selector a las entradas enumeradas, y un ID de Mantle incrusta un nombre de familia, por lo que cuenta como una entrada específica y desactiva el comodín de esa familia: junto con cualquier ID de Mantle, enumere los prefijos de versión o IDs completos que desea mantener seleccionables. Consulte [Comportamiento de fusión](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restricciones de modelo de organización
</h3>

Los administradores de la organización en planes Claude Enterprise restringen qué modelos pueden ejecutar los miembros deshabilitando modelos individuales en la consola de administración de claude.ai. Esta restricción se entrega con los derechos de la cuenta cuando Claude Code se autentica, separada de cualquier lista `availableModels` en configuración, y el servidor aplica la misma restricción de forma independiente cuando se crea una sesión. Requiere Claude Code v2.1.187 o posterior.

La restricción se aplica cuando un miembro inicia sesión o utiliza su propia clave API. Las credenciales con alcance de organización, como las claves de servicio de la organización, no están vinculadas a un usuario, por lo que la restricción no se aplica a ellas.

La Consola de Claude no tiene control de restricción de modelo. Las organizaciones sin un plan Claude Enterprise, incluidas aquellas cuyos miembros se autentican a través de la API de Anthropic, restringen modelos con [`availableModels`](#restrict-model-selection) en [configuración administrada](/docs/es/settings#settings-files) en su lugar, agregando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) para cubrir la opción Predeterminado. Estas configuraciones se aplican por Claude Code mismo, no por el servidor.

Un modelo restringido se oculta del selector `/model`. Seleccionarlo por nombre con `--model`, la variable de entorno `ANTHROPIC_MODEL`, o la configuración `model` muestra el aviso `Model "<name>" is restricted by your organization's settings. Using <model> instead.` y la sesión comienza en un modelo permitido. Escribir `/model <name>` para un modelo restringido se rechaza con `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` y la sesión mantiene su modelo actual.

Un [alias de familia de modelos](#restrict-model-selection) como `opus` se resuelve a la versión más reciente de su familia que la organización permite, con el mismo aviso de sustitución. `/model <alias>` se rechaza solo cuando cada versión de su familia está restringida; un alias establecido con `--model`, `ANTHROPIC_MODEL`, o la configuración `model` aún se reemplaza al inicio en ese caso. Antes de v2.1.205, un alias de familia se sustituía o rechazaba basándose únicamente en su versión más reciente lanzada, incluso cuando se permitía una versión anterior.

Las restricciones se aplican org-wide o por rol:

* Desactivar un modelo a nivel de organización lo elimina para cada miembro.
* El acceso a nivel de rol otorga diferentes modelos a diferentes roles personalizados, y un miembro que tiene varios roles puede usar cualquier modelo que uno de sus roles otorgue.
* Los modelos Haiku siempre están disponibles y no se pueden desactivar, por lo que cada miembro mantiene al menos un modelo utilizable.
* Un cambio de acceso toma efecto en nuevas solicitudes dentro de aproximadamente un minuto; el selector `/model` lo refleja la próxima vez que comienza una sesión.

Ambas restricciones se aplican juntas: un modelo es seleccionable solo cuando está permitido por `availableModels` y no está restringido por la organización. Las restricciones de la organización se entregan a sesiones en la API de Anthropic e implementaciones de [puerta de enlace LLM](/docs/es/llm-gateway). Las sesiones en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, y Claude Platform en AWS no las reciben, por lo que use `availableModels` en esos proveedores en su lugar.

<h2 id="organization-default-model">
  Modelo predeterminado de la organización
</h2>

Los administradores de la organización en planes Claude Enterprise pueden establecer un modelo predeterminado para los miembros de Claude Code desde la consola de administración de claude.ai, para toda la organización o por rol personalizado. Cuando se establece uno, la opción Predeterminado se resuelve a ese modelo en lugar del [predeterminado del tipo de cuenta](#default-model-setting). Requiere Claude Code v2.1.196 o posterior.

La fila Predeterminado en el selector `/model` muestra el nombre del modelo predeterminado de la organización con la etiqueta Predeterminado de org. La etiqueta lee Predeterminado de org ya sea que el administrador haya establecido el predeterminado para toda la organización o para su rol. Un predeterminado de rol cubre miembros de ese rol personalizado y tiene prioridad sobre el predeterminado de toda la organización; cuando varios de sus roles establecen diferentes predeterminados, se aplica el modelo más capaz.

El modelo predeterminado de la organización es un punto de partida, no una restricción, y cualquier otra selección de modelo tiene prioridad sobre él:

* la bandera `--model` y la variable de entorno `ANTHROPIC_MODEL`
* un valor `model` en [configuración administrada](/docs/es/settings#settings-files) o suministrado a través de `--settings`
* un valor `model` en su configuración de usuario, proyecto o local, incluido un modelo que guarda con `/model`

Los administradores también pueden configurar el modelo predeterminado de la organización para anular la selección del usuario. Con la anulación activada, tiene prioridad sobre el valor `model` en la configuración de usuario, proyecto y local, por lo que un modelo que guarda con `/model` se aplica para la sesión actual y el modelo predeterminado de la organización regresa en el siguiente lanzamiento. Cuando su selección difiere, `/model` muestra `Your organization's default (<model>) applies on restart`. La bandera `--model`, `ANTHROPIC_MODEL`, la configuración administrada y `--settings` aún tienen prioridad incluso con la anulación activada. La anulación está disponible para un conjunto limitado de organizaciones; pregunte a su equipo de cuenta de Anthropic sobre disponibilidad.

Para limitar qué modelos pueden seleccionar los miembros, use [restricciones de modelo de organización](#organization-model-restrictions) o [`availableModels`](#restrict-model-selection) en su lugar.

Claude Code lee el modelo predeterminado de la organización una vez al inicio, por lo que un predeterminado que el administrador cambia a mitad de sesión toma efecto en el siguiente lanzamiento.

Cuando el modelo predeterminado de la organización no anula la selección del usuario, el primer lanzamiento interactivo después de que el administrador lo cambia borra la clave `model` de su configuración de usuario una vez, por lo que se aplica el nuevo predeterminado. No cambia nada más en el archivo, y un modelo que guarda con `/model` después de ese lanzamiento se mantiene.

El modelo predeterminado de la organización pasa a través de las mismas verificaciones de restricción que cualquier otro modelo Predeterminado antes de ser adoptado:

* [`availableModels`](#restrict-model-selection) por sí solo nunca limita la opción Predeterminado, por lo que un modelo predeterminado de la organización fuera de la lista de permitidos aún se aplica. Cuando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) también está establecido, un modelo predeterminado de la organización fuera de la lista de permitidos se remapea a la primera entrada de la lista de permitidos, como cualquier otro Predeterminado
* un modelo predeterminado de la organización que [restricciones de modelo de organización](#organization-model-restrictions) niegan para su cuenta se reemplaza por el modelo más nuevo permitido en su familia, o una familia de menor costo cuando cada versión de ella está restringida
* un modelo predeterminado de la organización que no está disponible para su cuenta en absoluto, como Fable 5 bajo [retención de datos cero](/docs/es/zero-data-retention), se omite, y la opción Predeterminado se resuelve al predeterminado del tipo de cuenta

A partir de v2.1.199, cuando el modelo predeterminado de la organización es una familia de modelo diferente del predeterminado habitual del tipo de cuenta, el selector `/model` mantiene una fila separada para esa familia habitual, por lo que aún puede cambiar a ella para una sesión. En v2.1.196 a v2.1.198 esa fila falta del selector.

El modelo predeterminado de la organización se entrega a sesiones autenticadas con la API de Anthropic. Las sesiones en implementaciones de [puerta de enlace LLM](/docs/es/llm-gateway), Amazon Bedrock, Plataforma de Agentes de Google Cloud, Microsoft Foundry, y Claude Platform en AWS no lo reciben. Para establecer un predeterminado en esas implementaciones, use la clave `model` en [configuración administrada](/docs/es/settings#settings-files) en su lugar.

<h2 id="organization-effort-limits">
  Límites de esfuerzo de la organización
</h2>

Los administradores de la organización en planes Claude Enterprise pueden establecer un [nivel de esfuerzo](#adjust-effort-level) máximo por modelo para cada rol personalizado, junto con [restricciones de modelo de organización](#organization-model-restrictions) a nivel de rol. Los niveles por encima del límite no se ofrecen en el selector `/effort`, y nombrar un nivel más alto con `--effort` o `/effort` se ejecuta en el límite en su lugar. En sesiones interactivas y ejecuciones de `--print` de texto sin formato, una advertencia nombra los niveles solicitados y aplicados; con salida `json` o `stream-json` o en agentes de fondo, el límite se aplica silenciosamente. Los límites son por modelo, por lo que cambiar de modelos puede cambiar qué niveles están disponibles. Cuando varios de sus roles otorgan el mismo modelo, se aplica el límite menos restrictivo. Requiere Claude Code v2.1.195 o posterior.

Los límites de esfuerzo se entregan junto con [restricciones de modelo de organización](#organization-model-restrictions) y siguen la misma disponibilidad de proveedor: las sesiones en Amazon Bedrock, Plataforma de Agentes de Google Cloud, Microsoft Foundry, y Claude Platform en AWS no los reciben.

<h2 id="special-model-behavior">
  Comportamiento especial del modelo
</h2>

<h3 id="default-model-setting">
  Configuración del modelo `default`
</h3>

El comportamiento de `default` depende del tipo de cuenta:

* **Max, Team Premium, Enterprise de pago por uso y API de Anthropic**: por defecto Opus 4.8
* **Claude Platform en AWS, Amazon Bedrock y Google Cloud's Agent Platform**: por defecto Opus 4.8
* **Pro, Team Standard y asientos de suscripción Enterprise**: por defecto Sonnet 5
* **Microsoft Foundry**: por defecto Sonnet 4.5

Enterprise de pago por uso significa una organización Enterprise facturada por uso en lugar de por asiento de suscripción.

Antes de v2.1.207, `default` se resolvía a Opus 4.7 en Claude Platform en AWS y a Sonnet 4.5 en Amazon Bedrock y Google Cloud's Agent Platform.

Cuando un administrador ha establecido un [modelo predeterminado de la organización](#organization-default-model), `default` se resuelve a ese modelo en lugar del predeterminado del tipo de cuenta anterior. Requiere Claude Code v2.1.196 o posterior.

Cuando la configuración administrada [aplica la lista de permitidos para el modelo predeterminado](#enforce-the-allowlist-for-the-default-model) y el predeterminado del tipo de cuenta no está en `availableModels`, `default` se resuelve al predeterminado aplicado en lugar del predeterminado del tipo de cuenta anterior. Cuando ambos se aplican, el modelo predeterminado de la organización reemplaza el predeterminado del tipo de cuenta primero y la aplicación se aplica a él: un modelo predeterminado de la organización en la lista de permitidos se mantiene, mientras que uno fuera de la lista se resuelve al Predeterminado aplicado.

Fable 5 no es el modelo predeterminado en ningún tipo de cuenta. Las sesiones utilizan Fable 5 solo después de que lo elija, con `/model fable`, una configuración de `model`, o el alias `best` donde Fable 5 está disponible. Elegirlo con `/model` lo guarda como el modelo seleccionado en su configuración de usuario, por lo que las sesiones posteriores comienzan en Fable 5 hasta que cambie de modelos.

<h3 id="opusplan-model-setting">
  Configuración del modelo `opusplan`
</h3>

El alias de modelo `opusplan` proporciona un enfoque híbrido automatizado:

* **En Plan Mode**: utiliza `opus` para razonamiento complejo y decisiones de arquitectura
* **En Execution Mode**: cambia automáticamente a `sonnet` para generación de código e implementación

Esto empareja el razonamiento de Opus para la planificación con la eficiencia de Sonnet para la ejecución.

La fase Opus en Plan Mode utiliza la misma ventana de contexto que la configuración del modelo `opus`. En los niveles de suscripción donde Opus se [actualiza automáticamente a contexto de 1M](#extended-context), `opusplan` recibe la actualización en Plan Mode también. Para forzar contexto de 1M para ambas fases cuando no está en un nivel de actualización automática, establezca el modelo en `opusplan[1m]`.

Cuando [`availableModels`](#restrict-model-selection) excluye el Opus más nuevo pero permite una versión anterior, por ejemplo `["sonnet", "claude-opus-4-6"]`, `opusplan` utiliza el Opus más nuevo permitido para la planificación y se mantiene solo en Sonnet cuando se excluye cada Opus. Una sesión de Haiku que normalmente se actualizaría a Sonnet en Plan Mode de manera similar utiliza el Sonnet más nuevo permitido, y se mantiene en Haiku solo cuando se excluye cada Sonnet. Antes de v2.1.205, Plan Mode se mantenía en el modelo de la sesión siempre que se excluyera la versión más nueva de la familia de actualización, incluso cuando la lista de permitidos permitía una anterior.

La sustitución de una versión anterior permitida se aplica en la API de Anthropic y [Claude Platform en AWS](/docs/es/claude-platform-on-aws). En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y Mantle, cuyos despliegues utilizan ID de modelo específicos del proveedor, Plan Mode se mantiene en el modelo de la sesión siempre que se excluya el modelo de actualización.

Para un enfoque híbrido donde Claude decide a mitad de la tarea cuándo consultar un segundo modelo en lugar de cambiar en el límite del plan, consulte la [herramienta advisor](/docs/es/advisor).

<h3 id="fallback-model-chains">
  Cadenas de modelos de respaldo
</h3>

Cuando el modelo principal está sobrecargado, no disponible o devuelve otro error de servidor no reintentable, Claude Code puede cambiar a un modelo de respaldo en lugar de fallar la solicitud. Los errores de autenticación, facturación, límite de velocidad, tamaño de solicitud y transporte nunca desencadenan un cambio; esos siguen su reintento normal y manejo de errores.

Configure uno o más modelos de respaldo y Claude Code los intenta en orden, mostrando un aviso cuando cambia. El cambio dura solo para el turno actual, por lo que su próximo mensaje intenta el modelo principal primero nuevamente. Las cadenas están limitadas a tres modelos después de la eliminación de duplicados, y las entradas adicionales se ignoran.

Establezca una cadena para una sesión con la bandera `--fallback-model`, que acepta una lista separada por comas:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Para persistir una cadena entre sesiones, establezca `fallbackModel` en [configuración](/docs/es/settings) como una matriz:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

La bandera `--fallback-model` tiene precedencia sobre la configuración `fallbackModel`. Cada elemento acepta un nombre de modelo o alias, y `"default"` se expande al modelo predeterminado.

Dos casos causan que un elemento se omita:

* **Modelo no disponible**: un modelo que no se puede alcanzar, como un modelo retirado fijado en la configuración, se omite y Claude Code continúa con el siguiente elemento.
* **Fuera de la lista de permitidos**: un elemento no permitido por [`availableModels`](#restrict-model-selection) se descarta cuando se lee la cadena y nunca se intenta.

<h3 id="automatic-model-fallback">
  Respaldo automático del modelo
</h3>

Esta sección cubre el respaldo basado en contenido de Fable 5. Para respaldo basado en disponibilidad cuando un modelo está sobrecargado o no disponible, consulte [Cadenas de modelos de respaldo](#fallback-model-chains).

Fable 5 se ejecuta con clasificadores de seguridad para contenido de ciberseguridad y biología. Cuando un clasificador marca una solicitud, Claude Code vuelve a ejecutar esa solicitud en el modelo Opus predeterminado de su proveedor y muestra un aviso en la transcripción. En la API de Anthropic, despliegues de [puerta de enlace LLM](/docs/es/llm-gateway) y [Claude Platform en AWS](/docs/es/claude-platform-on-aws), ese modelo es Opus 4.8. En la [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), es Opus 4.7 a menos que apunte el alias [`opus`](#environment-variables) a otro modelo.

La sesión continúa en ese modelo Opus. Para volver a Fable 5, ejecute `/model fable`.

El destino de respaldo se verifica contra [`availableModels`](#restrict-model-selection). Cuando está bloqueado, no ocurre respaldo. El rechazo aparece como un error normal y el modelo de la sesión no cambia.

<h4 id="check-what-triggered-fallback">
  Verificar qué desencadenó el respaldo
</h4>

El respaldo puede desencadenarse en la primera solicitud de una sesión, antes de que envíe algo inusual, porque la primera solicitud lleva contexto del espacio de trabajo como su contenido de CLAUDE.md y estado de git. Un repositorio que contiene material de seguridad o biología puede activar el clasificador solo en ese contexto.

Para verificar si las personalizaciones son el desencadenante, inicie una sesión con `claude --safe-mode`, que desactiva personalizaciones como CLAUDE.md, skills, servidores MCP y hooks. El estado de git y los nombres de directorios no son personalizaciones y aún se incluyen.

<h4 id="ask-before-switching">
  Preguntar antes de cambiar
</h4>

Para decidir qué sucede cada vez que se marca una solicitud, en lugar de cambiar automáticamente, ejecute `/config` y desactive "cambiar modelos cuando se marca un mensaje". Una solicitud marcada pausa la sesión con dos opciones: cambiar al modelo Opus o editar el indicador e intentar nuevamente en Fable 5.

Algunos casos se comportan de manera diferente:

* Si ambos modelos marcan la misma solicitud, puede editar el indicador e intentar nuevamente, o iniciar una nueva sesión.
* En sesiones móviles de [Claude Code en la web](/docs/es/claude-code-on-the-web), no se admite edición y reintento. Cambie de modelos o continúe la sesión desde un navegador de escritorio o la aplicación de escritorio.
* En [modo no interactivo](/docs/es/cli-reference#cli-flags) e integraciones de SDK que no pueden mostrar el indicador, una solicitud marcada termina el turno con un rechazo en su lugar.
* Cuando el destino de respaldo está bloqueado por [`availableModels`](#restrict-model-selection), el indicador no se muestra. La solicitud marcada termina con el rechazo, igual que el respaldo automático cuando el destino está bloqueado.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Habilitar respaldo en Bedrock, Agent Platform y Foundry
</h4>

En [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), los ID de modelo son específicos del proveedor, por lo que el respaldo automático solo funciona cuando Claude Code puede identificar ambos modelos involucrados:

* Claude Code debe reconocer el modelo actual como Fable 5: el ID del modelo contiene `claude-fable-5`, coincide con el valor de `ANTHROPIC_DEFAULT_FABLE_MODEL`, o se asigna con [`modelOverrides`](#override-model-ids-per-version).
* El destino de respaldo debe resolverse en un modelo Opus: el valor de `ANTHROPIC_DEFAULT_OPUS_MODEL` si se establece, de lo contrario una entrada Opus 4.8 en la lista de modelos del proveedor.

Si no se puede identificar ninguno de los modelos, Claude Code no cambia automáticamente. La solicitud marcada termina con un mensaje de rechazo, y puede cambiar de modelos con [`/model`](#setting-your-model) e intentar nuevamente. Para habilitar el respaldo automático en estos proveedores, establezca `ANTHROPIC_DEFAULT_FABLE_MODEL` en su ID de modelo Fable 5 y `ANTHROPIC_DEFAULT_OPUS_MODEL` en su ID de modelo Opus 4.8.

<h4 id="security-research-and-biology-workloads">
  Cargas de trabajo de investigación de seguridad y biología
</h4>

Las cargas de trabajo en seguridad ofensiva o biología, incluidas pruebas de penetración, ejercicios Capture the Flag (CTF) y bases de código adyacentes a la biología, desencadenan respaldo frecuentemente, a menudo en la primera solicitud. Para trabajo de biología sustancial, espere que casi todas las solicitudes se redirijan.

Este es el enrutamiento esperado para estos dominios, no una bandera de cuenta. Si su organización necesita capacidad de clase Fable para este trabajo, pregunte a su equipo de cuenta de Anthropic sobre programas de acceso de confianza.

<h3 id="adjust-effort-level">
  Ajustar el nivel de esfuerzo
</h3>

[Los niveles de esfuerzo](https://platform.claude.com/docs/es/build-with-claude/effort) controlan el razonamiento adaptativo, que permite que el modelo decida si y cuánto pensar en cada paso basado en la complejidad de la tarea. El esfuerzo menor es más rápido y económico para tareas directas, mientras que el esfuerzo mayor proporciona un razonamiento más profundo para problemas complejos.

Los niveles de esfuerzo disponibles dependen del modelo. Los modelos no listados aquí no admiten esfuerzo:

| Modelo                        | Niveles                                 |
| :---------------------------- | :-------------------------------------- |
| Fable 5                       | `low`, `medium`, `high`, `xhigh`, `max` |
| Sonnet 5, Opus 4.8 y Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 y Sonnet 4.6         | `low`, `medium`, `high`, `max`          |

Si establece un nivel que el modelo activo no admite, Claude Code retrocede al nivel más alto admitido en o por debajo del que estableció. Por ejemplo, `xhigh` se ejecuta como `high` en Opus 4.6. Su organización también puede limitar qué niveles están disponibles para un modelo; consulte [Límites de esfuerzo de la organización](#organization-effort-limits).

El esfuerzo predeterminado es `high` en Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 y Sonnet 4.6, y `xhigh` en Opus 4.7.

Cuando ejecuta Fable 5, Opus 4.8 u Opus 4.7 por primera vez, Claude Code aplica el esfuerzo predeterminado de ese modelo incluso si estableció anteriormente un nivel diferente para otro modelo: `high` en Fable 5 y Opus 4.8, y `xhigh` en Opus 4.7. Ejecute `/effort` nuevamente para elegir un nivel diferente después de cambiar. Ese predeterminado se mantiene entre sesiones hasta que realice una opción de esfuerzo explícita, como ejecutar `/effort` en una sesión interactiva o iniciar con `--effort`.

`low`, `medium`, `high` y `xhigh` persisten entre sesiones cuando los establece en una sesión interactiva. Un nivel establecido con `/effort` en [modo no interactivo](/docs/es/headless), con la bandera `-p`, se aplica solo a la sesión actual y no se guarda como su predeterminado. Un `/effort` no interactivo tampoco puede liberar la retención de predeterminado del modelo anterior: en Fable 5, Opus 4.8 y Opus 4.7 informa `Not applied` y la sesión permanece en el esfuerzo predeterminado del modelo, por lo que pase `--effort` en el lanzamiento en su lugar. `max` proporciona el razonamiento más profundo sin restricción en el gasto de tokens y se aplica solo a la sesión actual, excepto cuando se establece a través de la variable de entorno `CLAUDE_CODE_EFFORT_LEVEL`.

El menú `/effort` también ofrece `ultracode`. Ultracode es una configuración de Claude Code en lugar de un nivel de esfuerzo del modelo: envía `xhigh` al modelo y además tiene Claude orquestar [flujos de trabajo dinámicos](/docs/es/workflows) para tareas sustanciales. Se aplica solo a la sesión actual.

Puede activar ultracode a través de cualquiera de los siguientes:

* **`/effort`**: ejecute `/effort ultracode`, o selecciónelo del menú
* **Bandera `--effort`**: inicie con `claude --effort ultracode`, que inicia la sesión con esfuerzo `xhigh` y ultracode activado
* **`--settings` o una solicitud de control del SDK del Agente**: pase `"ultracode": true`. Una solicitud [`applyFlagSettings()`](/docs/es/agent-sdk/typescript#applyflagsettings) también acepta `effortLevel: "ultracode"`

Pasar `ultracode` a la bandera `--effort` o al valor `effortLevel` del SDK del Agente requiere Claude Code v2.1.203 o posterior. Antes de v2.1.203, `--effort ultracode` imprimía `Unknown --effort value 'ultracode'` y la sesión comenzaba con el esfuerzo predeterminado.

La configuración `effortLevel` persistida y la variable de entorno `CLAUDE_CODE_EFFORT_LEVEL` no aceptan `ultracode`.

Cuando ultracode no está disponible, por ejemplo cuando [los flujos de trabajo están desactivados](/docs/es/workflows#turn-workflows-off), `--effort ultracode` establece solo esfuerzo `xhigh`.

<h4 id="choose-an-effort-level">
  Elegir un nivel de esfuerzo
</h4>

Cada nivel intercambia gasto de tokens contra capacidad. El predeterminado es adecuado para la mayoría de tareas de codificación; ajuste cuando desee un equilibrio diferente.

| Nivel       | Cuándo usarlo                                                                                                                                                                |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Reserve para tareas cortas, limitadas y sensibles a la latencia que no son sensibles a la inteligencia                                                                       |
| `medium`    | Reduce el uso de tokens para trabajo sensible a costos que puede intercambiar algo de inteligencia                                                                           |
| `high`      | Equilibra el uso de tokens e inteligencia. Predeterminado en Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 y Sonnet 4.6                                                              |
| `xhigh`     | Razonamiento más profundo con gasto de tokens más alto. Predeterminado en Opus 4.7                                                                                           |
| `max`       | Puede mejorar el rendimiento en tareas exigentes pero puede mostrar rendimientos decrecientes y es propenso a pensar demasiado. Pruebe antes de adoptar ampliamente          |
| `ultracode` | Una configuración de Claude Code que planifica un [flujo de trabajo dinámico](/docs/es/workflows) para cada tarea sustancial con razonamiento `xhigh` por mensaje. Solo de sesión |

La escala de esfuerzo se calibra por modelo, por lo que el mismo nombre de nivel no representa el mismo valor subyacente en todos los modelos.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Utilizar ultrathink para razonamiento profundo único
</h4>

Incluya `ultrathink` en cualquier lugar de su indicación para solicitar un razonamiento más profundo en ese turno sin cambiar su configuración de esfuerzo de sesión. Claude Code reconoce la palabra clave y añade una instrucción en contexto. El nivel de esfuerzo enviado a la API no cambia. Otras frases como "think", "think hard" y "think more" se pasan como texto de indicación ordinario y no se reconocen como palabras clave.

<h4 id="set-the-effort-level">
  Establecer el nivel de esfuerzo
</h4>

Puede cambiar el esfuerzo a través de cualquiera de los siguientes:

* **`/effort`**: ejecute `/effort` sin argumentos para abrir un control deslizante interactivo, `/effort` seguido de un nombre de nivel para establecerlo directamente, o `/effort auto` para restablecer el predeterminado del modelo
* **En `/model`**: utilice las teclas de flecha izquierda/derecha para ajustar el control deslizante de esfuerzo al seleccionar un modelo
* **Bandera `--effort`**: pase un nombre de nivel para establecerlo para una única sesión al iniciar Claude Code
* **Variable de entorno**: establezca `CLAUDE_CODE_EFFORT_LEVEL` en un nombre de nivel o `auto`
* **Configuración**: establezca `effortLevel` en `low`, `medium`, `high` o `xhigh` en su archivo de configuración. `max` y `ultracode` son [solo de sesión](#adjust-effort-level) y no se aceptan aquí
* **Frontmatter de skill y subagent**: establezca `effort` en un archivo markdown de [skill](/docs/es/skills#frontmatter-reference) o [subagent](/docs/es/sub-agents#supported-frontmatter-fields) para anular el nivel de esfuerzo cuando ese skill o subagent se ejecuta

La variable de entorno tiene precedencia sobre todos los demás métodos, luego su nivel configurado, luego el predeterminado del modelo. El esfuerzo de frontmatter se aplica cuando ese skill o subagent está activo, anulando el nivel de sesión pero no la variable de entorno.

El control deslizante de esfuerzo aparece en `/model` cuando se selecciona un modelo compatible. El nivel de esfuerzo actual también se muestra junto al logotipo y al indicador, por ejemplo "with low effort", para que pueda confirmar qué configuración está activa sin abrir `/model`.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Razonamiento adaptativo y presupuestos de pensamiento fijo
</h4>

El razonamiento adaptativo hace que el pensamiento sea opcional en cada paso, por lo que Claude puede responder más rápido a indicaciones rutinarias y reservar un pensamiento más profundo para pasos que se benefician de él. Si desea que Claude piense más o menos a menudo de lo que produce el nivel actual, puede decirlo directamente en su indicación o en `CLAUDE.md`; el modelo responde a esa orientación dentro de su configuración de esfuerzo.

Fable 5, Sonnet 5 y Opus 4.7 y posterior siempre utilizan razonamiento adaptativo. El modo de presupuesto de pensamiento fijo y `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` no se aplican a ellos.

En Opus 4.6 y Sonnet 4.6, puede establecer `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` para revertir al presupuesto de pensamiento fijo anterior controlado por `MAX_THINKING_TOKENS`. Consulte [variables de entorno](/docs/es/env-vars).

<h3 id="extended-thinking">
  Pensamiento extendido
</h3>

El pensamiento extendido es el razonamiento que Claude emite antes de responder. En modelos que admiten [razonamiento adaptativo](#adjust-effort-level), el nivel de esfuerzo es el control principal de cuánto pensamiento ocurre; la configuración a continuación activa o desactiva el pensamiento y controla cómo se muestra.

| Control                                    | Cómo configurarlo                                                                                                                                                                                                                                                                                                                                                                                                         |
| :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Alternar para la sesión actual             | Presione `Option+T` en macOS o `Alt+T` en Windows y Linux                                                                                                                                                                                                                                                                                                                                                                 |
| Establecer el predeterminado global        | Ejecute `/config` y alterne el modo de pensamiento. Se guarda como `alwaysThinkingEnabled` en `~/.claude/settings.json`                                                                                                                                                                                                                                                                                                   |
| Desactivar independientemente del esfuerzo | Establezca [`MAX_THINKING_TOKENS=0`](/docs/es/env-vars), que desactiva el pensamiento en la API de Anthropic excepto en Fable 5. En [proveedores de terceros](/docs/es/third-party-integrations) esto omite el parámetro `thinking` en su lugar, y los modelos de razonamiento adaptativo aún pueden pensar. Otros valores se aplican solo con un [presupuesto de pensamiento fijo](#adaptive-reasoning-and-fixed-thinking-budgets) |

El pensamiento no se puede desactivar en Fable 5. El alternar de sesión, `alwaysThinkingEnabled` y `MAX_THINKING_TOKENS=0` no tienen efecto allí, y Fable 5 decide por paso cuánto pensar basado en el nivel de esfuerzo.

La salida de pensamiento se colapsa de forma predeterminada. Presione `Ctrl+O` para alternar el modo detallado y ver el razonamiento como texto gris en cursiva. Las sesiones interactivas en la API de Anthropic reciben bloques de pensamiento redactados de forma predeterminada, por lo que establezca `showThinkingSummaries: true` en [configuración](/docs/es/settings) si desea que los resúmenes completos estén disponibles cuando se expandan. Se le cobra por todos los tokens de pensamiento generados, incluso cuando se colapsan o se redactan.

<h3 id="extended-context">
  Contexto extendido
</h3>

Fable 5, Sonnet 5, Opus 4.6 y posterior, y Sonnet 4.6 admiten una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/es/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas con bases de código grandes.

La disponibilidad varía según el modelo y el plan. En la API de Anthropic, Fable 5, Sonnet 5, Opus 4.8 y Opus 4.7 siempre se ejecutan con la ventana de 1M. En los planes Max, Team y Enterprise, Opus se actualiza automáticamente a contexto de 1M sin configuración adicional. Esto se aplica tanto a los asientos de Team Standard como de Team Premium. Sonnet 4.6 con contexto de 1M no es parte de la actualización automática y requiere [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) en todos los planes de suscripción, incluido Max.

| Plan                   | Opus con contexto de 1M                                                                                       | Sonnet 4.6 con contexto de 1M                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Max, Team y Enterprise | Incluido en la suscripción                                                                                    | Requiere [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                    | Requiere [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) | Requiere [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| API y pago por uso     | Acceso completo                                                                                               | Acceso completo                                                                                               |

Para desactivar completamente el contexto de 1M, establezca `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Esto elimina variantes de modelo de 1M del selector de modelo. Consulte [variables de entorno](/docs/es/env-vars).

La ventana de contexto de 1M utiliza precios de modelo estándar sin prima para tokens más allá de 200K. Para planes donde el contexto extendido está incluido en su suscripción, el uso permanece cubierto por su suscripción. Para planes que acceden al contexto extendido a través de créditos de uso, los tokens se facturan a los créditos de uso.

Si su cuenta admite contexto de 1M, la opción aparece en el selector de modelo (`/model`) en las últimas versiones de Claude Code. Si no la ve, intente reiniciar su sesión.

También puede utilizar el sufijo `[1m]` con alias de modelo o nombres de modelo completos:

```bash theme={null}
# Utilizar el alias opus[1m] o sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# O añadir [1m] a un nombre de modelo completo
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Ventana de contexto de Sonnet 5
</h4>

En la API de Anthropic, Sonnet 5 siempre se ejecuta con la ventana de contexto de 1M. No hay variante de 200K, no hay sufijo `[1m]` que seleccionar y no se requieren créditos de uso en ningún plan. Las sesiones se compactan automáticamente antes de que la ventana se llene, aproximadamente a 967K tokens de forma predeterminada; establezca [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars) para elegir un umbral diferente.

Dos configuraciones presupuestan la ventana en 200K en su lugar y se compactan automáticamente en ese límite:

* **Puerta de enlace LLM**: cuando `ANTHROPIC_BASE_URL` apunta a una [puerta de enlace](/docs/es/llm-gateway), Claude Code no puede verificar la compatibilidad con 1M. Para utilizar la ventana completa, seleccione Sonnet 5 (1M context) en el selector de modelo, que se asigna a `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: trata las sesiones de Sonnet 5 como si tuvieran una ventana de 200K, para implementaciones que necesitan limitar el contexto.

<h2 id="checking-your-current-model">
  Verificar su modelo actual
</h2>

Puede ver qué modelo está utilizando actualmente en dos lugares:

* En la [línea de estado](/docs/es/statusline), si tiene una configurada
* En `/status`, que también muestra la información de su cuenta

<h2 id="add-a-custom-model-option">
  Agregar una opción de modelo personalizado
</h2>

Utilice `ANTHROPIC_CUSTOM_MODEL_OPTION` para agregar una única entrada personalizada al selector `/model` sin reemplazar los alias integrados. Esto es útil para probar IDs de modelo que Claude Code no enumera de forma predeterminada. Para implementaciones de puerta de enlace LLM, Claude Code puede completar automáticamente el selector desde el punto final `/v1/models` de la puerta de enlace cuando se establece `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`, por lo que esta variable solo es necesaria cuando el descubrimiento está deshabilitado o no devuelve el modelo que desea. Consulte [descubrimiento de modelo de puerta de enlace](/docs/es/llm-gateway-protocol#model-discovery).

Este ejemplo establece las tres variables para hacer que una implementación de Opus enrutada por puerta de enlace sea seleccionable:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-4-8"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

La entrada personalizada aparece en la parte inferior del selector `/model`. `ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` y `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` son opcionales. Si se omiten, el ID de modelo se utiliza como nombre y la descripción tiene como valor predeterminado `Custom model (<model-id>)`.

Claude Code omite la validación para el ID de modelo establecido en `ANTHROPIC_CUSTOM_MODEL_OPTION`, por lo que puede utilizar cualquier cadena que su punto final de API acepte. Cuando [`availableModels`](#restrict-model-selection) está establecido, incluya también el ID de modelo personalizado en la lista de permitidos: la entrada personalizada se filtra del selector y una selección de `--model` de la misma se rechaza como cualquier otro modelo excluido. Un ID personalizado que incrusta un nombre de familia, como `my-gateway/claude-opus-4-8`, cuenta como una entrada específica para esa familia y deshabilita su comodín, por lo que también debe enumerar las versiones que desea mantener seleccionables. Consulte [Comportamiento de fusión](#merge-behavior).

<h2 id="environment-variables">
  Variables de entorno
</h2>

Puede utilizar las siguientes variables de entorno para controlar los nombres de modelo a los que se asignan los alias. Cada valor debe ser un nombre de modelo completo, o el identificador equivalente para su proveedor de API.

| Variable de entorno              | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                        |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | El modelo a utilizar para `fable`, y el ID de modelo que Claude Code reconoce como Fable 5 para [alternancia automática de modelo](#automatic-model-fallback) en proveedores de terceros                                                                                                                                                                                                                                           |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | El modelo a utilizar para `opus`, o para `opusplan` cuando Plan Mode está activo.                                                                                                                                                                                                                                                                                                                                                  |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | El modelo a utilizar para `sonnet`, o para `opusplan` cuando Plan Mode no está activo.                                                                                                                                                                                                                                                                                                                                             |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | El modelo a utilizar para `haiku`, o [funcionalidad de fondo](/docs/es/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | El modelo a utilizar para todos los [subagents](/docs/es/sub-agents#choose-a-model), [agent teams](/docs/es/agent-teams), y los agentes que ejecuta un [workflow](/docs/es/workflows). Acepta un alias como `haiku` o un nombre de modelo completo, y anula tanto el parámetro `model` por invocación como el frontmatter `model` de la definición del subagent. Establezca en `inherit` para utilizar la resolución de modelo normal en su lugar |

Nota: `ANTHROPIC_SMALL_FAST_MODEL` está deprecado en favor de `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Fijar modelos para implementaciones de terceros
</h3>

Cuando implemente Claude Code a través de [Amazon Bedrock](/docs/es/amazon-bedrock), [Plataforma de Agentes de Google Cloud](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry), o [Claude Platform on AWS](/docs/es/claude-platform-on-aws), fije versiones de modelo antes de implementar para usuarios.

Sin fijar, Claude Code utiliza alias de modelo como `fable`, `opus`, `sonnet` y `haiku` que se resuelven a un ID de modelo predeterminado integrado para cada proveedor. Ese predeterminado puede rezagarse con respecto a la versión más reciente de Anthropic, y el modelo al que apunta puede que aún no esté habilitado en la cuenta de un usuario. Cuando el predeterminado no está disponible, los usuarios de Amazon Bedrock y Plataforma de Agentes de Google Cloud ven un aviso y la sesión retrocede a una versión anterior del modelo predeterminado, o al modelo Sonnet predeterminado cuando el predeterminado es un modelo Opus y no hay versión de Opus disponible. Los usuarios de Microsoft Foundry ven errores en su lugar, porque Microsoft Foundry no tiene ninguna verificación de inicio equivalente.

<Warning>
  Establezca las variables de entorno de modelo en IDs de versión específicos como parte de su configuración inicial. Fijar le permite controlar cuándo sus usuarios se mueven a un nuevo modelo.
</Warning>

Utilice las siguientes variables de entorno con IDs de modelo específicos de versión para su proveedor:

| Proveedor                             | Ejemplo                                                              |
| :------------------------------------ | :------------------------------------------------------------------- |
| Amazon Bedrock                        | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Plataforma de Agentes de Google Cloud | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry                     | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Aplique el mismo patrón para `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` y `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Para IDs de modelo actuales y heredados en todos los proveedores, consulte [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para actualizar usuarios a una nueva versión de modelo, actualice estas variables de entorno e implemente nuevamente.

Para habilitar [contexto extendido](#extended-context) para un modelo fijo, añada `[1m]` al ID de modelo en `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

El sufijo `[1m]` aplica la ventana de contexto de 1M a todo el uso de los alias `opus` y `sonnet`, incluida la fase Opus de modo de plan de [`opusplan`](#opusplan-model-setting).

* Claude Code elimina el sufijo antes de enviar el ID de modelo a su proveedor.
* Solo añada `[1m]` cuando el modelo subyacente [admita contexto de 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* El sufijo se lee por variable, no por modelo. En Amazon Bedrock, Plataforma de Agentes de Google Cloud y Microsoft Foundry, un ID de modelo sin `[1m]` en una variable utiliza contexto de 200K incluso si otra variable establece el mismo modelo con el sufijo. Sonnet 5 siempre se ejecuta con la ventana de 1M en estos proveedores y nunca necesita el sufijo.

<Note>
  Una lista de permitidos `availableModels` entregada a través de [MDM o un archivo de configuración administrado](/docs/es/settings#settings-files) aún se aplica cuando se utilizan proveedores de terceros; [la configuración administrada por servidor no se entrega allí](/docs/es/server-managed-settings#platform-availability). El filtrado coincide con un alias de modelo como `opus`, un prefijo de versión como `claude-opus-4-8`, o el ID de modelo completo en forma de proveedor. Los prefijos específicos del proveedor como `us.anthropic.` no se eliminan, por lo que para permitir un modelo específico, enumere el mismo ID en forma de proveedor que muestra el selector, o asígnelo a través de [`modelOverrides`](#override-model-ids-per-version). Cualquier sufijo `[1m]` se elimina tanto de la entrada de la lista de permitidos como del modelo solicitado antes de coincidir.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personalizar la visualización y capacidades del modelo fijo
</h3>

Cuando fija un modelo en un proveedor de terceros, el ID específico del proveedor aparece tal cual en el selector `/model` y Claude Code puede no reconocer qué características admite el modelo. Puede anular el nombre de visualización y declarar capacidades con variables de entorno complementarias para cada modelo fijo.

Estas variables tienen efecto en proveedores de terceros como Amazon Bedrock, Plataforma de Agentes de Google Cloud y Microsoft Foundry. Las variables `_NAME` y `_DESCRIPTION` también tienen efecto cuando `ANTHROPIC_BASE_URL` apunta a una [puerta de enlace LLM](/docs/es/llm-gateway). No tienen efecto cuando se conecta directamente a `api.anthropic.com`.

| Variable de entorno                                   | Descripción                                                                                                                                 |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nombre de visualización para el modelo Opus fijo en el selector `/model`. Por defecto al ID de modelo cuando no está configurado            |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Descripción de visualización para el modelo Opus fijo en el selector `/model`. Por defecto a `Custom Opus model` cuando no está configurado |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Lista separada por comas de capacidades que admite el modelo Opus fijo                                                                      |

Los mismos sufijos `_NAME`, `_DESCRIPTION` y `_SUPPORTED_CAPABILITIES` están disponibles para `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` y `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code habilita características como [niveles de esfuerzo](#adjust-effort-level) y [pensamiento extendido](#extended-thinking) haciendo coincidir el ID de modelo con patrones conocidos. Los IDs específicos del proveedor como ARNs de Amazon Bedrock o nombres de implementación personalizados a menudo no coinciden con estos patrones, dejando las características compatibles deshabilitadas. Establezca `_SUPPORTED_CAPABILITIES` para indicar a Claude Code qué características admite realmente el modelo:

| Valor de capacidad     | Habilita                                                                                             |
| ---------------------- | ---------------------------------------------------------------------------------------------------- |
| `effort`               | [Niveles de esfuerzo](#adjust-effort-level) y el comando `/effort`                                   |
| `xhigh_effort`         | El nivel de esfuerzo `xhigh`                                                                         |
| `max_effort`           | El nivel de esfuerzo `max`                                                                           |
| `thinking`             | [Pensamiento extendido](#extended-thinking)                                                          |
| `adaptive_thinking`    | Razonamiento adaptativo que asigna dinámicamente el pensamiento basado en la complejidad de la tarea |
| `interleaved_thinking` | Pensamiento entre llamadas de herramientas                                                           |

Cuando se establece `_SUPPORTED_CAPABILITIES`, las capacidades enumeradas se habilitan y las capacidades no enumeradas se deshabilitan para el modelo fijo coincidente. Cuando la variable no está configurada, Claude Code vuelve a la detección integrada basada en el ID de modelo.

Este ejemplo fija Opus a un ARN de modelo personalizado de Amazon Bedrock, establece un nombre amigable y declara sus capacidades:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Anular IDs de modelo por versión
</h3>

Las variables de entorno a nivel de familia anteriores configuran un ID de modelo por alias de familia. Si necesita asignar varias versiones dentro de la misma familia a IDs de proveedor distintos, utilice la configuración `modelOverrides` en su lugar.

`modelOverrides` asigna IDs de modelo individuales de Anthropic a las cadenas específicas del proveedor que Claude Code envía a la API de su proveedor. Cuando un usuario selecciona un modelo asignado en el selector `/model`, Claude Code utiliza su valor configurado en lugar del predeterminado integrado.

Esto permite a los administradores empresariales enrutar cada versión de modelo a un ARN de perfil de inferencia de Amazon Bedrock específico, nombre de versión de Plataforma de Agentes de Google Cloud o nombre de implementación de Microsoft Foundry para gobernanza, asignación de costos o enrutamiento regional.

Establezca `modelOverrides` en su [archivo de configuración](/docs/es/settings#settings-files):

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Las claves deben ser IDs de modelo de Anthropic como se enumeran en la [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para IDs de modelo con fecha, incluya el sufijo de fecha exactamente como aparece allí. Las claves desconocidas se ignoran.

Las anulaciones reemplazan los IDs de modelo integrados que respaldan cada entrada en el selector `/model`. En Amazon Bedrock, las entradas de `modelOverrides` tienen precedencia sobre cualquier perfil de inferencia que Claude Code descubra automáticamente al inicio. Claude Code pasa valores que ya son específicos del proveedor, como ARNs de perfil de inferencia de Amazon Bedrock o nombres de implementación de Microsoft Foundry, al proveedor tal como están.

Las anulaciones también se aplican cuando pasa un ID de modelo de Anthropic directamente a través de `--model`, la variable de entorno `ANTHROPIC_MODEL`, o una variable de entorno `ANTHROPIC_DEFAULT_*_MODEL`. En Amazon Bedrock, Plataforma de Agentes de Google Cloud y [Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint), un ID de modelo de Anthropic sin entrada de `modelOverrides` se resuelve al mismo ID específico del proveedor que la fila del selector `/model` para esa versión, cuando el proveedor admite esa versión. Mantle admite un subconjunto de versiones. Para un ID de modelo de Anthropic fuera de ese subconjunto, Claude Code envía el ID sin procesar a Mantle sin asignarlo, a menos que una entrada de `modelOverrides` lo cubra. Antes de v2.1.200, `--model` y los valores de variable de entorno llegaban al proveedor tal como estaban sin pasar por el mapa de anulación.

`modelOverrides` funciona junto con `availableModels`. La lista de permitidos se evalúa contra el ID de modelo de Anthropic, no el valor de anulación, por lo que una entrada como `"opus"` en `availableModels` continúa coincidiendo incluso cuando las versiones de Opus se asignan a ARNs. Cuando `enforceAvailableModels` se establece en configuración administrada, el Predeterminado aplicado se resuelve a través de `modelOverrides` desde la [fuente administrada de mayor precedencia](/docs/es/server-managed-settings#settings-precedence) únicamente. La asignación de un administrador, como una versión fijada a un ARN de perfil de inferencia, se respeta en el Predeterminado aplicado. Las anulaciones de configuración de usuario o proyecto no la afectan.

Cuando `availableModels` se establece en [configuración administrada](/docs/es/settings#settings-files), solo `modelOverrides` de esa fuente administrada se aplican a un ID de modelo de Anthropic pasado directamente a través de `--model` o las variables de entorno anteriores. Claude Code ignora las anulaciones en configuración de usuario o proyecto para esos IDs, y nunca resuelve un ID que la lista administrada excluye a través de `modelOverrides` de ninguna fuente de configuración. Esta restricción de fuente administrada requiere Claude Code v2.1.200 o posterior. Consulte [Restringir la selección de modelo](#restrict-model-selection) para saber cómo se manejan los IDs bloqueados.

<h3 id="prompt-caching-configuration">
  Configuración de almacenamiento en caché de indicaciones
</h3>

Claude Code utiliza automáticamente [almacenamiento en caché de indicaciones](/docs/es/prompt-caching) para optimizar el rendimiento y reducir costos. Puede desactivar el almacenamiento en caché de indicaciones globalmente o para niveles de modelo específicos:

| Variable de entorno             | Descripción                                                                                                                                              |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Establezca en `1` para desactivar el almacenamiento en caché de indicaciones para todos los modelos. Tiene precedencia sobre la configuración por modelo |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Establezca en `1` para desactivar el almacenamiento en caché de indicaciones solo para modelos Haiku                                                     |
| `DISABLE_PROMPT_CACHING_SONNET` | Establezca en `1` para desactivar el almacenamiento en caché de indicaciones solo para modelos Sonnet                                                    |
| `DISABLE_PROMPT_CACHING_OPUS`   | Establezca en `1` para desactivar el almacenamiento en caché de indicaciones solo para modelos Opus                                                      |
| `DISABLE_PROMPT_CACHING_FABLE`  | Establezca en `1` para desactivar el almacenamiento en caché de indicaciones solo para modelos Fable                                                     |

Para cambiar el TTL de caché u obtener más información sobre qué desencadena un error de caché, consulte [Cómo Claude Code utiliza el almacenamiento en caché de indicaciones](/docs/es/prompt-caching).
