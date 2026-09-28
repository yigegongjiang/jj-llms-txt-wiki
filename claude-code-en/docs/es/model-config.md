> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración del modelo

> Configure qué modelo utiliza Claude Code, niveles de esfuerzo, contexto extendido y la ventana de auto-compact

<h2 id="available-models">
  Modelos disponibles
</h2>

Para la configuración de `model` en Claude Code, puede configurar:

* Un **alias de modelo**
* Un **nombre de modelo**
  * API de Anthropic: un **[nombre de modelo](https://platform.claude.com/docs/en/about-claude/models/overview)** completo
  * Amazon Bedrock: un ARN de perfil de inferencia
  * Microsoft Foundry: un nombre de implementación
  * Agent Platform de Google Cloud: un nombre de versión

Para obtener orientación sobre qué modelo y nivel de esfuerzo se ajustan a diferentes tipos de trabajo, consulte [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) en el blog.

<Note>
  `ANTHROPIC_BASE_URL` cambia dónde se envían las solicitudes, no qué modelo las responde. Para enrutar Claude a través de una puerta de enlace LLM, consulte [LLM gateways](/docs/es/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias de modelos
</h3>

Use un alias de modelo para seleccionar configuraciones de modelo sin recordar números de versión exactos:

| Alias de modelo  | Comportamiento                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Valor especial que borra cualquier anulación de modelo y revierte al [valor predeterminado en tiempo de ejecución para su cuenta](#default-model-setting). No es en sí mismo un alias de modelo                                                                                                                                                                                  |
| **`best`**       | Utiliza el modelo que el alias [`fable`](#fable-alias-resolution) se resuelve a donde Fable está disponible para usted; de lo contrario, el mismo modelo que `opus`                                                                                                                                                                                                              |
| **`fable`**      | Utiliza el [modelo Fable para su proveedor](#fable-alias-resolution) para sus tareas más difíciles y de mayor duración                                                                                                                                                                                                                                                           |
| **`sonnet`**     | Utiliza el último modelo Sonnet para tareas de codificación diaria                                                                                                                                                                                                                                                                                                               |
| **`opus`**       | Utiliza el último modelo Opus para tareas de razonamiento complejo                                                                                                                                                                                                                                                                                                               |
| **`haiku`**      | Utiliza el modelo Haiku rápido y eficiente para tareas simples                                                                                                                                                                                                                                                                                                                   |
| **`sonnet[1m]`** | Utiliza Sonnet con una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas. Sin efecto cuando `sonnet` ya se resuelve a Sonnet 5 con su ventana nativa de 1M; detrás de una [puerta de enlace LLM](/docs/es/llm-gateway), selecciona la ventana de 1M para Sonnet 5 |
| **`opus[1m]`**   | Utiliza Opus con una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas                                                                                                                                                                                       |
| **`opusplan`**   | Modo especial que utiliza `opus` durante Plan Mode, luego cambia a `sonnet` para la ejecución                                                                                                                                                                                                                                                                                    |

La versión a la que se resuelven los alias `opus` y `sonnet` depende del proveedor:

| Proveedor                                            | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| API de Anthropic                                     | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/es/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Agent Platform de Google Cloud       | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

A menos que establezca `ANTHROPIC_DEFAULT_FABLE_MODEL`, el alias `fable` se resuelve a Fable 5.1, excepto en sesiones de [Claude apps gateway](/docs/es/claude-apps-gateway), donde `fable` y `best` se resuelven a Fable 5. Antes de v2.1.257, `fable` se resolvía a Fable 5 en cada proveedor.

Una puerta de enlace que no está configurada para servir `claude-fable-5-1` rechaza solicitudes para ese modelo. Para usar Fable 5.1 a través de una puerta de enlace que lo sirve, selecciónelo con `/model claude-fable-5-1`.

Donde un alias se resuelve a un modelo más antiguo, los modelos más nuevos están disponibles seleccionando explícitamente el nombre de modelo completo o estableciendo `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Antes de v2.1.280, `opus` se resolvía a Opus 5 en la API de Anthropic, Claude Platform on AWS, Amazon Bedrock y Agent Platform de Google Cloud desde v2.1.219. Antes de v2.1.219, `opus` se resolvía a Opus 4.8 en la API de Anthropic desde v2.1.154, y en Claude Platform on AWS, Amazon Bedrock y Agent Platform de Google Cloud desde v2.1.207. Antes de v2.1.207, `opus` se resolvía a Opus 4.7 en Claude Platform on AWS y a Opus 4.6 en Amazon Bedrock y Agent Platform de Google Cloud.

Los alias apuntan a la versión recomendada para su proveedor y se actualizan con el tiempo. Para fijar una versión específica, use el nombre de modelo completo, por ejemplo `claude-opus-5-5`, o establezca la variable de entorno correspondiente como `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 requiere Claude Code v2.1.280 o posterior. Opus 5 requiere v2.1.219 o posterior. Sonnet 5 requiere v2.1.197 o posterior. Ejecute `claude update` para actualizar.
</Note>

<h3 id="work-with-fable">
  Trabajar con Fable
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) y Claude Fable 5 son los modelos más capaces en Claude Code, adecuados para tareas más grandes que una sola sesión. Sostienen sesiones autónomas largas, investigan antes de actuar y verifican su trabajo más a menudo que los modelos más pequeños. Fable 5.1 es la versión más reciente.

Ninguno de los modelos Fable es el valor predeterminado de tipo de cuenta en ningún plan o proveedor. Seleccione uno explícitamente:

* **Fable 5.1**: ejecute `/model fable`, o inicie con `claude --model fable`. En sesiones de [Claude apps gateway](/docs/es/claude-apps-gateway), donde el alias se resuelve a Fable 5, ejecute `/model claude-fable-5-1` en su lugar.
* **Fable 5**: selecciónelo por ID de modelo. En la API de Anthropic, ejecute `/model claude-fable-5` o inicie con `claude --model claude-fable-5`. En otros proveedores, use el ID de modelo Fable 5 de su proveedor o [fíjelo](#pin-models-for-third-party-deployments) con `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Si se conecta a la API de Anthropic directamente y su configuración de usuario contiene `claude-fable-5` o `claude-fable-5[1m]` como modelo, por ejemplo porque seleccionó Fable en el selector `/model` antes de v2.1.257, Claude Code cambia ese valor guardado al alias `fable` o `fable[1m]` la primera vez que ejecuta v2.1.257 o posterior. La línea del modelo de inicio muestra `(auto-updated)` una vez. Un valor `claude-fable-5` en la configuración del proyecto, local o administrada se deja tal como está.

Las solicitudes que los clasificadores de seguridad de un modelo Fable marcan, más a menudo en dominios de ciberseguridad y biología, desencadenan [alternancia automática de modelo](#automatic-model-fallback).

Para aprovechar al máximo Fable:

* **Describa el resultado, no los pasos**: entrégale el resultado que desea y déjelo planificar el camino. Para mantenerlo trabajando hacia ese resultado, [establezca un objetivo](/docs/es/goal).
* **Entrégale problemas ambiguos**: investigaciones de causa raíz, depuración de interrupciones y decisiones de arquitectura son donde la investigación y verificación adicionales se pagan.
* **Omita los recordatorios de verificación**: verifica su propio trabajo con menos indicaciones, por lo que los recordatorios para probar o verificar generalmente son innecesarios.
* **Dimensione tareas más grandes**: entrégale trabajo que normalmente dividiría en piezas. Mantiene sesiones largas sin perder el hilo.

<Note>
  Fable 5.1 requiere Claude Code v2.1.257 o posterior. Si una solicitud de una versión anterior falla, consulte [Claude Code does not support this model](/docs/es/errors#claude-code-does-not-support-this-model). Ejecute `claude update` para actualizar. Para disponibilidad bajo retención de datos cero, consulte [Model availability under ZDR](/docs/es/zero-data-retention#model-availability-under-zdr).
</Note>

En la API de Anthropic, un modelo Fable aparece en el selector `/model` a menos que [`availableModels`](#restrict-model-selection) o [restricciones de modelo de organización](#organization-model-restrictions) lo excluyan. Cuando su organización no puede usar Fable en absoluto, por ejemplo bajo [retención de datos cero](/docs/es/zero-data-retention#model-availability-under-zdr), la fila permanece en el selector atenuada, con una nota sobre por qué.

<h4 id="fable-and-usage-credits">
  Fable y créditos de uso
</h4>

Dependiendo de su plan y nivel de asiento, el uso de Fable puede facturarse a [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) en lugar de extraer de los límites incluidos en su plan. Cuando lo hace, el selector `/model` muestra "Requires usage credits" en la fila Fable. Para administrar créditos de uso, consulte [Add usage credits to your subscription](/docs/es/costs#add-usage-credits-to-your-subscription).

En sesiones interactivas, Claude Code muestra un mensaje de consentimiento antes de que una solicitud de Fable facture créditos de uso. Los miembros de planes Enterprise con facturación de organización no ven el mensaje. Puede continuar en Fable usando créditos de uso o cambiar a su modelo predeterminado. También puede descartar el mensaje:

* En el selector `/model`, mantiene su modelo actual.
* A mitad de sesión, Claude Code continúa el turno en su modelo predeterminado.

Después de elegir continuar en Fable usando créditos de uso, Claude Code no muestra el mensaje nuevamente.

En una sesión con [Remote Control](/docs/es/remote-control) conectado, una [sesión en segundo plano](/docs/es/agent-view), o una sesión de compañero de [equipo de agentes](/docs/es/agent-teams), es posible que nadie esté en la terminal, por lo que Claude Code mantiene el mensaje de consentimiento a mitad de sesión para el plazo [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry), cinco minutos por defecto. Si nadie ha respondido antes del plazo, Claude Code finaliza el turno sin enviar la solicitud y agrega un aviso a la transcripción, que el cliente de Remote Control también muestra. Su selección de modelo no cambia, y Claude Code solicita consentimiento nuevamente en su próximo mensaje.

Lo que puede hacer mientras el mensaje está esperando depende de la sesión:

* Con Remote Control conectado o en la sesión de un compañero, presione cualquier tecla en la terminal para cancelar el plazo, y Claude Code espera su respuesta.
* En una sesión en segundo plano, responda antes del plazo.
* Si envía un nuevo mensaje desde el cliente remoto antes de que alguien escriba en la terminal, Claude Code finaliza el turno de la misma manera, y su nuevo mensaje inicia el siguiente turno. Después de que alguien escriba en la terminal, Claude Code continúa esperando la respuesta y pone en cola su nuevo mensaje detrás de ella.

En [modo no interactivo](/docs/es/headless) con la bandera `-p` y a través del Agent SDK, Claude Code nunca muestra el mensaje de consentimiento. Cuando una solicitud de Fable allí facturase a créditos de uso, Claude Code la factura sin preguntar.

<h3 id="setting-your-model">
  Configurar su modelo
</h3>

Puede configurar su modelo de varias formas, enumeradas en orden de prioridad:

1. **Durante la sesión**: use `/model <alias|name>` para cambiar inmediatamente, o ejecute `/model` sin argumento para abrir el selector. Consulte [cuando Claude Code le pide que confirme el cambio](/docs/es/prompt-caching#switching-models)
2. **Al inicio**: inicie con `claude --model <alias|name>`
3. **Variable de entorno**: establezca `ANTHROPIC_MODEL=<alias|name>`
4. **Configuración**: configure permanentemente en su archivo de configuración usando el campo `model`
5. **[Predeterminado para nuevas sesiones](#set-a-default-model-for-new-sessions)**: establezca `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` guarda su elección como predeterminada para nuevas sesiones escribiendo el campo `model` en su configuración de usuario. En el selector:

* `Enter`: cambiar modelo y guardar como predeterminado
* `s`: cambiar modelo solo para esta sesión y dejar su predeterminado sin cambios. Para usar una tecla diferente, reenlace [`modelPicker:thisSessionOnly`](/docs/es/keybindings#model-picker-actions)

Escribir `/model <name>` directamente se comporta como `Enter`. Para cambiar solo para esta sesión, abra el selector con `/model` y presione `s` en la fila del modelo.

Si cambia modelos con `/model`, el cambio también llega a [subagentes que heredan el modelo de la conversación principal](/docs/es/sub-agents#choose-a-model), porque Claude Code resuelve su modelo desde el que su sesión está usando cuando Claude los inicia. Cambie a Opus antes de que Claude delegue investigación o ejecuciones de prueba a uno de ellos, y ese trabajo se ejecuta en Opus también. Para mantener un subagente personalizado en un modelo más pequeño, establezca `model` en su definición.

Si establece un modelo con `/model` en [modo no interactivo](/docs/es/headless), con la bandera `-p`, su elección se aplica solo a la sesión actual y no se guarda como predeterminado; `/model` en ese modo requiere Claude Code v2.1.205 o posterior. La configuración del proyecto y administrada aún tienen precedencia y se reaplicarán en el próximo inicio. Un [modelo predeterminado de organización](#organization-default-model) que su administrador ha configurado para anular la selección del usuario también se reaplicará en el próximo inicio.

En v2.1.144 a v2.1.152, `/model` se aplicaba solo a la sesión actual y `d` en el selector guardaba un predeterminado.

La bandera `--model` y la variable de entorno `ANTHROPIC_MODEL` se aplican solo a la sesión que inicia con ellas. Para ejecutar diferentes modelos en diferentes terminales al mismo tiempo, inicie cada uno con su propia bandera `--model` en lugar de cambiar con `/model`.

Los precios en el selector `/model` aparecen cuando Claude Code habla con la API de Anthropic, directamente o a través de una [puerta de enlace LLM](/docs/es/llm-gateway) que la proxifica, y el precio en una fila es el precio del modelo que esa fila selecciona. En [proveedores de terceros](/docs/es/third-party-integrations) como Amazon Bedrock y en la [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), su proveedor o puerta de enlace determina lo que paga, por lo que las filas del selector no muestran precio. El precio es solo una etiqueta de visualización; no afecta qué modelo selecciona una fila o qué factura su proveedor. Antes de v2.1.206, [Claude Platform on AWS](/docs/es/claude-platform-on-aws) y las sesiones de puerta de enlace mostraban precios de lista de Anthropic, y una fila podría mostrar el precio de un modelo diferente al que seleccionaba.

Las sesiones reanudadas iniciadas con `claude --resume`, `--continue`, o el selector `/resume` mantienen el modelo que estaban usando cuando se guardó la transcripción, independientemente de la configuración actual de `model`. Si el modelo restaurado se ha retirado o está excluido por [`availableModels`](#restrict-model-selection), la sesión cae a través del orden de precedencia normal. Esto evita que la elección de `/model` de otra sesión cambie el modelo en la reanudación. En proveedores que usan ID de implementación específicos del proveedor en lugar de ID de modelo de Anthropic, como Amazon Bedrock, Agent Platform de Google Cloud y Microsoft Foundry, el modelo de transcripción no se restaura en absoluto y la sesión resuelve su modelo a través del orden de precedencia normal.

Un modelo que elige para el nuevo inicio con `--model` o `ANTHROPIC_MODEL` aún tiene precedencia sobre el modelo restaurado. A partir de v2.1.195, también lo hace una variable de familia [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables). [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) también puede, bajo las condiciones enumeradas en su sección.

Cuando el modelo activo al inicio proviene de la configuración del proyecto o administrada en lugar de su propia selección, el encabezado de inicio muestra qué archivo de configuración lo estableció. Ejecute `/model` para anular; la configuración del proyecto o administrada se reaplicará en el próximo inicio. En plataformas que incrustan Claude Code y establecen [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars), la configuración de modelo del host tiene precedencia sobre la configuración de modelo administrada, mientras que una lista de permitidos `availableModels` administrada permanece en vigor a menos que el host suministre la suya propia; [Exceptions to managed settings precedence](/docs/es/settings#exceptions-to-managed-settings-precedence) dice qué claves y variables anula el host.

Si usted u su organización configuran [PreModelSwitch hooks](/docs/es/hooks#premodelswitch), se ejecutan antes de que se aplique un cambio solicitado y pueden bloquearlo o pedirle que confirme.

Cuando Claude Code no puede determinar qué PreModelSwitch hooks entregan sus [plugins administrados](/docs/es/settings-reference#enabledplugins) de la organización, por ejemplo porque un plugin administrado no se cargó, rechaza el cambio en lugar de aplicarlo sin verificar, y verifica nuevamente en cada nuevo intento. Consulte [Model switch was blocked by a PreModelSwitch hook](/docs/es/errors#model-switch-was-blocked-by-a-premodelswitch-hook) para el mensaje y recuperación.

Cuando cambia modelos a través del método `setModel()` del [Agent SDK](/docs/es/agent-sdk/overview) o desde un dispositivo conectado a través de [Remote Control](/docs/es/remote-control), o una aplicación como la [aplicación de escritorio](/docs/es/desktop) que ejecuta Claude Code CLI cambia para usted, Claude Code verifica que la cadena sea una que reconozca antes de guardarla. Esta verificación requiere Claude Code v2.1.200 o posterior. Verificar una selección de Remote Control requiere Claude Code v2.1.260 o posterior en su máquina. En la API de Anthropic, Claude Code reconoce:

* un alias de modelo
* una entrada del selector `/model`
* cualquier nombre que comience con `claude-`
* un valor que configuró usted mismo como una [opción de modelo personalizado](#add-a-custom-model-option) o en [`modelOverrides`](#override-model-ids-per-version)

Claude Code rechaza una cadena no reconocida con `Model "<name>" is not a recognized model id.` y la sesión mantiene su modelo actual, en lugar de guardar la cadena y fallar en la próxima solicitud. Consulte [la referencia de errores](/docs/es/errors#model-is-not-a-recognized-model-id) para pasos de recuperación.

La verificación se ejecuta solo en la API de Anthropic. En Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/es/claude-platform-on-aws), y detrás de una [puerta de enlace LLM](/docs/es/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizado, su proveedor o puerta de enlace define los nombres de modelo, por lo que Claude Code pasa cualquier cadena sin verificarla. La verificación tampoco cubre la bandera `--model`, la variable de entorno `ANTHROPIC_MODEL`, o la configuración de `model`; un valor mal escrito allí produce [There's an issue with the selected model](/docs/es/errors#theres-an-issue-with-the-selected-model) en la primera solicitud en su lugar. Claude Code aún puede escribir la [línea de diagnóstico de modelo no reconocido](/docs/es/errors#unrecognized-model-id-on-a-request) en el momento de la solicitud, en cada proveedor.

Cuando el modelo solicitado tiene una fecha de retiro programada o se remapea automáticamente a una versión más nueva, Claude Code muestra una advertencia que nombra el modelo solicitado. Las sesiones interactivas la muestran como un aviso de inicio. A partir de v2.1.182, la misma advertencia se escribe en stderr en [modo no interactivo](/docs/es/headless) cuando se usa el formato de salida de texto predeterminado. La verificación también cubre un `model` establecido en [frontmatter de subagente](/docs/es/sub-agents). La advertencia de stderr se suprime para `--output-format json` y `stream-json`; lea el modelo real del campo `modelUsage` del [mensaje de resultado](/docs/es/headless#get-structured-output) en su lugar.

Por ejemplo, inicie una sesión en Opus:

```bash theme={null}
claude --model opus
```

Luego cambie modelos desde dentro de la sesión:

```text theme={null}
/model sonnet
```

Archivo de configuración de ejemplo:

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Establecer un modelo predeterminado para nuevas sesiones
</h4>

Establezca `ANTHROPIC_DEFAULT_MODEL=<alias|name>` para elegir el modelo en el que sus sesiones comienzan de forma predeterminada. Requiere Claude Code v2.1.236 o posterior.

Claude Code inicia una nueva sesión en el modelo de la variable solo cuando ninguno de estos selecciona un modelo:

* La bandera `--model`
* `ANTHROPIC_MODEL`
* Un valor de `model` en cualquier archivo de configuración, incluida la elección que guarda con `/model`
* Un [modelo predeterminado de organización](#organization-default-model)

Una elección que guarda con `/model` tiene precedencia sobre la variable en lanzamientos posteriores también. Con `ANTHROPIC_MODEL` establecido en su lugar, Claude Code vuelve al modelo de esa variable en el próximo lanzamiento, sea lo que sea que haya guardado con `/model`.

Claude Code también resuelve la opción Predeterminado al modelo de la variable, a menos que se aplique un modelo predeterminado de organización. Cuando la opción Predeterminado se resuelve al modelo de la variable, la fila Predeterminado en el selector `/model` muestra la etiqueta Set by ANTHROPIC\_DEFAULT\_MODEL.

Claude Code ignora la variable en estos casos, y la opción Predeterminado se resuelve como si no la hubiera establecido:

* La establece en `default`, `inherit`, `opusplan`, o `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) está activado
* [`availableModels`](#restrict-model-selection) o [restricciones de modelo de organización](#organization-model-restrictions) excluyen el modelo
* El modelo no está disponible para su cuenta

Cuando una nueva sesión comenzaría en el modelo de la variable, una sesión que reanuda con `claude --resume`, `--continue`, o el selector `/resume` también comienza en él. Claude Code no restaura el modelo guardado en la transcripción de esa sesión. De lo contrario, Claude Code no usa la variable cuando [reanuda una sesión](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Una nueva sesión comienza en un modelo diferente al que eligió
</h4>

Cuando elige un modelo con `/model` y su próxima sesión comienza en algo más, estas son las causas habituales:

* **Lo eligió para una sesión.** Presionar `s` en el selector, iniciar con `--model`, y ejecutar `/model` en modo no interactivo se aplican solo a la sesión actual y dejan su predeterminado guardado solo.
* **Algo con mayor prioridad establece el modelo.** Un valor de `model` en la configuración del proyecto o administrada, `ANTHROPIC_MODEL` en su shell, o un [predeterminado de organización](#organization-default-model) que su administrador estableció para anular las elecciones del usuario se aplica nuevamente en cada lanzamiento. Su elección de `/model` aún se guarda; está superada. Cuando la configuración del proyecto o administrada establece el modelo, el encabezado de inicio nombra el archivo.
* **Claude Code no pudo guardar su elección.** `/model` escribe `model` en `~/.claude/settings.json`. Si no puede escribir en ese archivo, por ejemplo porque otra herramienta lo genera o lo vincula a una copia de solo lectura, el modelo que eligió dura para la sesión y el próximo lanzamiento lee el valor anterior. Establezca `model` en la herramienta que genera el archivo, o haga que el archivo sea escribible. Consulte [A change you made in Claude Code is lost in new sessions](/docs/es/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Reanudó una sesión.** Una sesión que reanuda con `claude --resume` o `--continue` generalmente [mantiene el modelo que estaba usando](#setting-your-model) en lugar de su predeterminado actual.

<h2 id="restrict-model-selection">
  Restringir la selección de modelos
</h2>

Los administradores empresariales pueden usar `availableModels` en [configuración administrada o de políticas](/docs/es/managed-settings) para restringir qué modelos pueden seleccionar los usuarios. Las entradas coinciden con una familia de modelos como `sonnet`, un prefijo de versión como `claude-sonnet-4-5`, o un ID de modelo completo como `claude-sonnet-4-5-20250929`. Un prefijo de versión también coincide con IDs de modelo posteriores que lo extienden con otro segmento, por lo que `claude-fable-5` permite tanto Fable 5 como Fable 5.1, mientras que `claude-fable-5-1` permite solo Fable 5.1.

En plataformas que integran Claude Code y establecen [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars), la configuración de modelo del host tiene precedencia sobre la configuración de modelos administrada, mientras que una lista de permitidos `availableModels` administrada permanece en vigor a menos que el host proporcione la suya propia; [Excepciones a la precedencia de configuración administrada](/docs/es/settings#exceptions-to-managed-settings-precedence) indica qué claves y variables anula el host.

Cuando se establece `availableModels`, la lista de permitidos se aplica en todas partes donde un usuario puede especificar un modelo:

* **Modelo de sesión principal**: `/model`, la bandera `--model`, la variable de entorno `ANTHROPIC_MODEL`, la configuración `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), y el modelo restaurado cuando [se reanuda una sesión](#setting-your-model)
* **Resolución de alias**: las variables de entorno `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, y `ANTHROPIC_DEFAULT_FABLE_MODEL` no pueden redirigir un alias permitido a un modelo fuera de la lista
* **Modo rápido**: `/fast` se niega a alternar cuando implicaría cambiar implícitamente a un modelo Opus fuera de la lista, con el mensaje "is not in your organization's allowed models"
* **Modelos de subagente y compañero**: el campo `model` en la [frontmatter de subagente](/docs/es/sub-agents#choose-a-model), el parámetro `model` de la herramienta Agent, [modelos de compañero de equipo de agente](/docs/es/agent-teams#specify-teammates-and-models), `CLAUDE_CODE_SUBAGENT_MODEL`, y, en v2.1.197 y anteriores, el selector de modelos en el asistente `/agents`&#x20;
* **Modelos de skill y comando**: la frontmatter `model` en [skills y comandos](/docs/es/skills)
* **Modelo de asesor**: la configuración [`advisorModel`](/docs/es/advisor) configurada y la bandera `--advisor`
* **Modelo de agente de fondo**: el modelo seleccionado en el [selector de envío](/docs/es/agent-view)

En la API de Anthropic y [Claude Platform en AWS](/docs/es/claude-platform-on-aws), un alias de familia de modelos, `opus`, `sonnet`, `haiku`, o `fable`, se resuelve a su modelo habitual cuando la lista de permitidos permite ese modelo. Cuando la lista de permitidos bloquea ese modelo, Claude Code sustituye la versión más nueva de la familia que la lista de permitidos permite y muestra un aviso que nombra tanto los modelos solicitados como los sustituidos. Con `["sonnet", "claude-opus-4-6"]`, por ejemplo, tanto `/model opus` como `--model opus` seleccionan Claude Opus 4.6, el Opus más nuevo permitido. Antes de v2.1.205, un alias cuya versión más nueva lanzada estaba fuera de la lista se rechazaba o reemplazaba como cualquier otra selección bloqueada, incluso cuando la lista permitía una versión anterior.

La sustitución necesita una versión permitida en la que aterrizar: cuando la lista de permitidos no permite ninguna versión de la familia del alias, el alias sigue el comportamiento de rechazo y reemplazo a continuación como cualquier otro valor bloqueado.

Claude Code maneja cualquier otra selección bloqueada según dónde se estableció el modelo:

* **`/model`**: Claude Code rechaza el cambio con un error
* **Bandera `--model`, `ANTHROPIC_MODEL`, o la configuración `model`**: Claude Code reemplaza el valor al inicio con una advertencia que nombra tanto los modelos solicitados como los sustituidos, y la sesión comienza en el modelo predeterminado
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**: Claude Code ignora la variable
* **Anulación de subagente o compañero**: Claude Code ejecuta el subagente o compañero en un modelo de respaldo en lugar de fallar la solicitud. Consulte [Elegir un modelo](/docs/es/sub-agents#choose-a-model) para el respaldo del subagente y [Especificar compañeros y modelos](/docs/es/agent-teams#specify-teammates-and-models) para el respaldo del compañero.

  En sesiones interactivas, Claude Code le advierte cuando sustituye el modelo de un subagente, por este respaldo o por la sustitución de versión más nueva permitida anterior, nombrando los modelos solicitados y sustituidos; no informa del respaldo de un compañero.

  Donde opera la sustitución de versión más nueva permitida anterior, un alias de familia bloqueado la sigue en su lugar. Antes de v2.1.222, un alias retrocedía como cualquier otro valor bloqueado en cada proveedor
* **Anulación de skill o comando**: Claude Code ignora la anulación, incluido un alias de familia bloqueado, y el skill o comando se ejecuta en el modelo de sesión. Un skill o comando que [se ejecuta en un subagente](/docs/es/skills#run-skills-in-a-subagent) sigue el comportamiento del subagente anterior en su lugar
* **Configuración `advisorModel`**: el asesor se deshabilita para la sesión
* **Bandera `--advisor`**: Claude Code sale con un error al inicio. En una [sesión de fondo](/docs/es/agent-view), inicia la sesión sin el asesor en lugar de salir

Claude Code oculta los modelos excluidos del selector `/model`. Un ID de modelo completo en la lista que no tiene una fila de selector integrada, como una versión anterior que la lista fija, aparece en el selector `/model` como su propia fila etiquetada, a menos que Claude Code reemplace las opciones integradas con una alineación [`modelPicker`](/docs/es/settings-reference#modelpicker). Antes de v2.1.199, tal ID era seleccionable solo escribiendo `/model <id>`.

Los cambios de modelo que Claude Code realiza en su nombre se verifican de la misma manera:

* **[Cadenas de modelo de respaldo](#fallback-model-chains)**: las entradas fuera de la lista de permitidos se descartan
* **Actualizaciones de modo de plan**: en la API de Anthropic y Claude Platform en AWS, una actualización como [`opusplan`](#opusplan-model-setting) a un modelo excluido usa la versión más nueva permitida de la familia de actualización. En proveedores con IDs de modelo específicos del proveedor, y cuando no se permite ninguna versión, la actualización se omite y la planificación continúa en el modelo de la sesión
* **[Respaldo automático de modelo](#automatic-model-fallback)**: un respaldo cuyo destino está excluido no se ejecuta, por lo que la solicitud marcada termina con un rechazo en su lugar
* **[Clasificador de modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode)**: el valor predeterminado de Claude Sonnet 5 del clasificador se aplica solo cuando la lista de permitidos permite Sonnet 5. Cuando está excluido, el clasificador se ejecuta en el modelo de la sesión, que la lista de permitidos ya rige, o en un modelo Opus cuando la sesión se ejecuta en un [modelo Fable](#work-with-fable). En proveedores distintos de la API de Anthropic, ese respaldo de Opus se ejecuta en el modelo Opus predeterminado del proveedor sin consultar la lista de permitidos. Requiere Claude Code v2.1.210 o posterior
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

| Mecanismo de entrega                                                                                      | CLI e IDE | Sesiones locales de escritorio | Sesiones web, móviles y en la nube                                                                                                                                                                                                                                       | Agent SDK y no interactivo | Cowork                       |
| :-------------------------------------------------------------------------------------------------------- | :-------- | :----------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------- | :--------------------------- |
| [Configuración administrada por servidor](/docs/es/server-managed-settings) desde la consola de administración | Aplicada  | Aplicada                       | Aplicada                                                                                                                                                                                                                                                                 | Aplicada                   | No entregada                 |
| [MDM o archivos de configuración administrada](/docs/es/managed-settings#delivery-mechanisms)                  | Aplicada  | Aplicada                       | No entregada en entornos alojados por Anthropic; en [entornos autohospedados](/docs/es/self-hosted-environments), aplicada desde la imagen del ejecutor según [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) | Aplicada                   | Aplicada donde se implementa |

* Las sesiones en la nube, en [Claude Code en la web](/docs/es/claude-code-on-the-web) o en la aplicación de escritorio, se ejecutan en máquinas virtuales administradas por Anthropic de forma predeterminada: la configuración implementada en su dispositivo no las alcanza, por lo que entregue la lista de permitidos a través de configuración administrada por servidor. Las sesiones que su organización enruta a un [entorno autohospedado](/docs/es/self-hosted-environments) se ejecutan en su propio cálculo y también leen el archivo de configuración administrada en la imagen del ejecutor. [Cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) indica cuándo se aplica ese archivo. Un cambio de modelo a mitad de sesión en una sesión en la nube se rechaza cuando el modelo solicitado está excluido por la lista de permitidos. Cuando `availableModels` en su configuración administrada por servidor no está vacío, el servidor rechaza la solicitud de un usuario para iniciar una sesión en la nube en un modelo que la lista excluye.
* Cowork, la pestaña de trabajo agéntico en la aplicación de escritorio de Claude, ejecuta sus sesiones en Claude Code pero, por diseño, no recibe configuración administrada por servidor desde la consola de administración de claude.ai. Un archivo de configuración administrada se aplica a sesiones de Cowork cuando está presente donde se ejecuta la sesión; las sesiones remotas de Cowork se ejecutan en máquinas virtuales administradas por Anthropic, donde un archivo implementado en el dispositivo no está presente.
* Las sesiones en [proveedores de terceros](/docs/es/server-managed-settings#platform-availability) como Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, y [Claude Platform en AWS](/docs/es/claude-platform-on-aws) no reciben configuración administrada por servidor, por lo que entregue la lista de permitidos a través de MDM o archivos de configuración administrada allí.
* La entrega administrada por servidor también requiere que la sesión se autentique con un [inicio de sesión o clave elegible](/docs/es/server-managed-settings#platform-availability). Las flotas que generan claves solo a través de un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) deben entregar la lista de permitidos a través de MDM o archivos de configuración administrada.
* La pestaña de código de escritorio también aloja [sesiones SSH](/docs/es/desktop#ssh-sessions), que leen el archivo de configuración administrada del host remoto en el que se ejecutan. Consulte [Configuración administrada de escritorio](/docs/es/desktop#managed-settings).
* Los selectores de modelos en claude.ai y en la aplicación de escritorio ocultan o atenúan los modelos excluidos por la lista de permitidos de su organización. El estado del selector es una conveniencia para los usuarios; no aplica la lista de permitidos.

<h3 id="default-model-behavior">
  Comportamiento del modelo predeterminado
</h3>

Por sí solo, `availableModels` deja la opción Predeterminado en el [predeterminado de tiempo de ejecución](#default-model-setting) del sistema para la cuenta hasta que también establezca [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model). Si ese predeterminado es un modelo que tiene la intención de restringir, establezca también `enforceAvailableModels`.

Una matriz `availableModels` vacía nunca activa la aplicación del modelo predeterminado: con `availableModels: []`, las selecciones de modelos nombrados se bloquean pero el modelo predeterminado para el tipo de cuenta sigue siendo utilizable independientemente de `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Aplicar la lista de permitidos para el modelo predeterminado
</h3>

Establezca `enforceAvailableModels: true` junto con un `availableModels` no vacío en la configuración administrada para extender la lista de permitidos a la opción Predeterminado. Esto requiere Claude Code v2.1.175 o posterior.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

La opción Predeterminado se resuelve al predeterminado del tipo de cuenta, o al [modelo predeterminado de organización](#organization-default-model) cuando un administrador ha establecido uno. Cuando ese modelo no está en la lista de permitidos, la opción Predeterminado se resuelve en su lugar a la primera entrada `availableModels` que nombra un modelo permitido y disponible, y la fila Predeterminado del selector `/model` muestra ese modelo. Esto se aplica en todas partes donde se alcanza el predeterminado: inicio de sesión, seleccionar Predeterminado en `/model`, la palabra clave `"default"` en [cadenas de modelo de respaldo](#fallback-model-chains), y el respaldo utilizado cuando se descarta una selección excluida.

`enforceAvailableModels` remapea la opción Predeterminado solo cuando `availableModels` no está vacío. Con `availableModels: []`, el modelo predeterminado para el tipo de cuenta sigue siendo utilizable, por lo que la configuración no puede bloquear a los usuarios de cada modelo. Cuando `availableModels` no está vacío pero ninguna entrada se resuelve a un modelo permitido y disponible, la aplicación se omite y Predeterminado se resuelve al predeterminado del tipo de cuenta, con una advertencia visible solo bajo `--debug`. Mantenga al menos una entrada garantizada disponible en la lista para evitar esto.

Implemente ambas claves juntas en la fuente administrada de mayor rango que entregue. De forma predeterminada, Claude Code lee solo esa fuente, por lo que un par colocado en un archivo de configuración administrada se ignora cuando la consola de administración entrega cualquier configuración; bajo la fusión de opción de participación en [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources), Claude Code aún ignora un mapa `modelOverrides` de una fuente clasificada por debajo de la que establece `availableModels`.

<h3 id="control-the-model-users-run-on">
  Controlar el modelo en el que se ejecutan los usuarios
</h3>

La configuración `model` es una selección inicial, no una aplicación. Establece qué modelo está activo cuando comienza una sesión, pero los usuarios aún pueden abrir `/model` y seleccionar Predeterminado, que se resuelve al [predeterminado de tiempo de ejecución](#default-model-setting) del sistema independientemente de lo que se establezca `model`, a menos que [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) lo redirija.

Para controlar completamente la experiencia del modelo, combine estas configuraciones:

* **`availableModels`**: restringe qué modelos nombrados pueden cambiar los usuarios
* **`enforceAvailableModels`**: extiende la lista de permitidos `availableModels` a la opción Predeterminado, por lo que Predeterminado no puede resolverse a un modelo fuera de la lista
* **`model`**: establece la selección de modelo inicial cuando comienza una sesión
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: controlan a qué se resuelven los alias `sonnet`, `opus`, `haiku`, y `fable`, y qué versión usa el [predeterminado del tipo de cuenta](#default-model-setting)

Este ejemplo inicia a los usuarios en Sonnet 4.5, limita el selector a Sonnet y Haiku, y asegura que Predeterminado se resuelva a un modelo en la lista de permitidos en lugar del predeterminado de nivel:

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

Sin `enforceAvailableModels` o el bloque `env`, un usuario que selecciona Predeterminado en el selector obtiene el [predeterminado de tiempo de ejecución](#default-model-setting) en lugar de la versión fijada en `model`. Las dos configuraciones cubren diferentes alcances: `enforceAvailableModels` hace que Predeterminado obedezca la lista de permitidos, mientras que el bloque `env` fija qué versión se resuelve un alias permitido como `sonnet`. Use `enforceAvailableModels` solo cuando restringir familias de modelos sea suficiente; agregue el bloque `env` cuando también necesite fijar una versión específica.

<h3 id="merge-behavior">
  Comportamiento de fusión
</h3>

Cuando la configuración administrada que Claude Code aplica define `availableModels`, esa lista sola se aplica, aparte de una [plataforma host que proporciona la suya propia](/docs/es/settings#exceptions-to-managed-settings-precedence): las entradas en la configuración de usuario, proyecto o local no pueden extenderla, y Claude Code nunca fusiona `availableModels` entre fuentes administradas tampoco; [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) indica qué lista de fuente se aplica. De lo contrario, las listas de la configuración de usuario, proyecto y local se [concatenan y desduplican](/docs/es/settings#settings-precedence) como otras configuraciones de matriz. Antes de Claude Code v2.1.175, las entradas de alcances de menor precedencia se fusionaban en la lista administrada en lugar de ser reemplazadas por ella.

Dentro de la lista efectiva, una entrada que nombra un modelo específico en una familia, ya sea un prefijo de versión o un ID de modelo completo, deshabilita la entrada de comodín de esa familia: `["sonnet", "claude-sonnet-4-5"]` permite solo versiones de Sonnet 4.5, no cada modelo de Sonnet.

<h3 id="mantle-model-ids">
  IDs de modelo de Mantle
</h3>

Cuando el [punto final de Amazon Bedrock Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) está habilitado, las entradas en `availableModels` que comienzan con `anthropic.` se agregan al selector `/model` como opciones personalizadas y se enrutan al punto final de Mantle. Esta es una excepción a la coincidencia de alias descrita en [Fijar modelos para implementaciones de terceros](#pin-models-for-third-party-deployments). La configuración aún restringe el selector a entradas enumeradas, y un ID de Mantle incrusta un nombre de familia, por lo que cuenta como una entrada específica y deshabilita el comodín de esa familia: junto con cualquier ID de Mantle, enumere los prefijos de versión o IDs completos que desea mantener seleccionables. Consulte [Comportamiento de fusión](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restricciones de modelo de organización
</h3>

Los administradores de organización en planes Claude Enterprise restringen qué modelos pueden ejecutar los miembros deshabilitando modelos individuales en la consola de administración de claude.ai. Esta restricción se entrega con los derechos de la cuenta cuando Claude Code se autentica, separada de cualquier lista `availableModels` en la configuración, y el servidor aplica la misma restricción de forma independiente cuando se crea una sesión. Requiere Claude Code v2.1.187 o posterior.

La restricción se aplica cuando un miembro inicia sesión o usa su propia clave API. Las credenciales con alcance de organización, como las claves de servicio de organización, no están vinculadas a un usuario, por lo que la restricción no se aplica a ellas.

La consola de Claude no tiene control de restricción de modelo. Las organizaciones sin un plan Claude Enterprise, incluidas las cuyos miembros se autentican a través de la API de Anthropic, restringen modelos con [`availableModels`](#restrict-model-selection) en [configuración administrada](/docs/es/managed-settings) en su lugar, agregando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) para cubrir la opción Predeterminado. [Cobertura de superficie](#surface-coverage) dice cómo cada superficie recibe y aplica estas configuraciones.

Un modelo restringido se oculta del selector `/model`. Seleccionarlo por nombre con `--model`, la variable de entorno `ANTHROPIC_MODEL`, o la configuración `model` muestra el aviso `Model "<name>" is restricted by your organization's settings. Using <model> instead.` y la sesión comienza en un modelo permitido. Escribir `/model <name>` para un modelo restringido se rechaza con `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` y la sesión mantiene su modelo actual.

Un [alias de familia de modelos](#restrict-model-selection) como `opus` se resuelve a su modelo habitual cuando la organización lo permite. Cuando la organización restringe ese modelo, Claude Code sustituye la versión más nueva de la familia que la organización permite, con el mismo aviso de sustitución. `/model <alias>` se rechaza solo cuando cada versión de su familia está restringida; un alias establecido con `--model`, `ANTHROPIC_MODEL`, o la configuración `model` aún se reemplaza al inicio en ese caso. Antes de v2.1.205, un alias de familia se sustituía o rechazaba basándose solo en su versión más nueva lanzada, incluso cuando se permitía una versión anterior.

Las restricciones se aplican en toda la organización o por rol:

* Deshabilitar un modelo a nivel de organización lo elimina para cada miembro.
* El acceso a nivel de rol otorga diferentes modelos a diferentes roles personalizados, y un miembro que tiene varios roles puede usar cualquier modelo que uno de sus roles otorgue.
* Los modelos de Haiku siempre están disponibles y no se pueden deshabilitar, por lo que cada miembro mantiene al menos un modelo utilizable.
* Un cambio de acceso entra en vigor en nuevas solicitudes dentro de aproximadamente un minuto; el selector `/model` lo refleja la próxima vez que comienza una sesión.

Ambas restricciones se aplican juntas: un modelo es seleccionable solo cuando está permitido por `availableModels` y no está restringido por la organización. Las restricciones de organización llegan a sesiones en la API de Anthropic y implementaciones de [puerta de enlace LLM](/docs/es/llm-gateway) solo; en cualquier otro proveedor, use `availableModels` en su lugar.

<h2 id="organization-default-model">
  Modelo predeterminado de la organización
</h2>

Los administradores de la organización en planes Claude Enterprise pueden establecer un modelo predeterminado para los miembros de Claude Code desde la consola de administración de claude.ai, para toda la organización o por rol personalizado. Cuando se establece uno, la opción Predeterminado se resuelve a ese modelo. Requiere Claude Code v2.1.196 o posterior.

La fila Predeterminado en el selector `/model` muestra el nombre del modelo predeterminado de la organización con la etiqueta Predeterminado de org. La etiqueta dice Predeterminado de org tanto si el administrador estableció el predeterminado para toda la organización como para su rol. Un predeterminado de rol cubre a los miembros de ese rol personalizado y tiene prioridad sobre el predeterminado de toda la organización; cuando varios de sus roles establecen predeterminados diferentes, se aplica el modelo más capaz.

El modelo predeterminado de la organización es un punto de partida, no una restricción. Estas selecciones tienen prioridad sobre él:

* la bandera `--model` y la variable de entorno `ANTHROPIC_MODEL`
* un valor `model` en [configuración administrada](/docs/es/managed-settings) o suministrado a través de `--settings`
* un valor `model` en su configuración de usuario, proyecto o local, incluido un modelo que guarde con `/model`

Los administradores también pueden configurar el modelo predeterminado de la organización para anular la selección del usuario. Con la anulación activada, tiene prioridad sobre el valor `model` en la configuración de usuario, proyecto y local, por lo que un modelo que guarde con `/model` se aplica para la sesión actual y el modelo predeterminado de la organización vuelve en el siguiente lanzamiento. Cuando su selección difiere, `/model` muestra `Se aplica el predeterminado de su organización (<model>) al reiniciar`. La bandera `--model`, `ANTHROPIC_MODEL`, la configuración administrada y `--settings` siguen teniendo prioridad incluso con la anulación activada. La anulación está disponible para un conjunto limitado de organizaciones; consulte con su equipo de cuenta de Anthropic sobre la disponibilidad.

Para limitar qué modelos pueden seleccionar los miembros, use [restricciones de modelo de la organización](#organization-model-restrictions) o [`availableModels`](#restrict-model-selection) en su lugar.

Claude Code lee el modelo predeterminado de la organización una vez al inicio, por lo que un predeterminado que el administrador cambia a mitad de sesión entra en vigor en el siguiente lanzamiento.

Cuando el modelo predeterminado de la organización no anula la selección del usuario, el primer lanzamiento interactivo después de que el administrador lo cambie borra la clave `model` de su configuración de usuario una sola vez, por lo que se aplica el nuevo predeterminado. No cambia nada más en el archivo, y un modelo que guarde con `/model` después de ese lanzamiento se mantiene.

El modelo predeterminado de la organización pasa a través de estas comprobaciones de restricción antes de ser adoptado:

* [`availableModels`](#restrict-model-selection) por sí solo no se aplica al modelo predeterminado de la organización, por lo que un modelo predeterminado de la organización fuera de la lista de permitidos sigue aplicándose. Cuando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) también está establecido, un modelo predeterminado de la organización fuera de la lista de permitidos se remapea a la primera entrada de la lista de permitidos, como cualquier otro Predeterminado
* un modelo predeterminado de la organización que [restricciones de modelo de la organización](#organization-model-restrictions) deniegan para su cuenta se reemplaza por el modelo más reciente permitido en su familia, o una familia de menor costo cuando cada versión de ella está restringida
* un modelo predeterminado de la organización que no está disponible para su cuenta en absoluto se omite, y la opción Predeterminado se resuelve como lo haría [sin un modelo predeterminado de la organización](#default-model-setting)

A partir de v2.1.199, cuando el modelo predeterminado de la organización es una familia de modelos diferente del predeterminado habitual del tipo de cuenta, el selector `/model` mantiene una fila separada para esa familia habitual, por lo que aún puede cambiar a ella para una sesión. En v2.1.196 a v2.1.198 esa fila falta en el selector.

El modelo predeterminado de la organización llega solo a sesiones autenticadas con la API de Anthropic. Para establecer un predeterminado en cualquier otro lugar, incluidas las implementaciones de [puerta de enlace LLM](/docs/es/llm-gateway), use la clave `model` en [configuración administrada](/docs/es/managed-settings) en su lugar.

<h2 id="organization-effort-limits">
  Límites de esfuerzo de la organización
</h2>

Su organización puede limitar el [nivel de esfuerzo](#adjust-effort-level) de dos formas. En un plan Claude Enterprise, los administradores de la organización establecen límites de esfuerzo por rol, descritos a continuación. En cualquier plan y cualquier proveedor, incluidos Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, la configuración administrada [`maxEffortLevel`](/docs/es/settings-reference#maxeffortlevel) limita el esfuerzo en el cliente. Cuando ambos se aplican a un modelo, se aplica el límite más bajo.

Los administradores de la organización en planes Claude Enterprise pueden establecer un [nivel de esfuerzo](#adjust-effort-level) máximo por modelo para cada rol personalizado, junto con [restricciones de modelo de la organización](#organization-model-restrictions) a nivel de rol. Los niveles por encima del límite no se ofrecen en el selector `/effort`, y nombrar un nivel superior con `--effort` o `/effort` se ejecuta en el límite en su lugar. En sesiones interactivas y ejecuciones simples de texto `--print`, una advertencia nombra los niveles solicitados y aplicados; con salida `json` o `stream-json` o en agentes en segundo plano, el límite se aplica silenciosamente. Los límites son por modelo, por lo que cambiar de modelo puede cambiar qué niveles están disponibles. Cuando varios de sus roles otorgan el mismo modelo, se aplica el límite menos restrictivo. Requiere Claude Code v2.1.195 o posterior.

Los límites de esfuerzo se entregan junto con [restricciones de modelo de la organización](#organization-model-restrictions) y llegan a las mismas sesiones.

<h2 id="special-model-behavior">
  Comportamiento especial del modelo
</h2>

<h3 id="default-model-setting">
  Configuración del modelo `default`
</h3>

El comportamiento de `default` depende de tu tipo de cuenta:

* **Pro, Max, Team, Enterprise y Anthropic API**: por defecto Opus 5.5
* **Claude Platform en AWS, Amazon Bedrock y Agent Platform de Google Cloud**: por defecto Opus 5.5
* **Microsoft Foundry**: por defecto Sonnet 4.5

Antes de v2.1.280, `default` se resolvía a Sonnet 5 en Pro y Team Standard, y a Opus 5 en Max, Team Premium, Enterprise, Anthropic API, Claude Platform en AWS, Amazon Bedrock y Agent Platform de Google Cloud desde v2.1.219. Antes de v2.1.219, `default` se resolvía a Opus 4.8 en Anthropic API, Max, Team Premium y Enterprise de pago por uso desde v2.1.154, y en Claude Platform en AWS, Amazon Bedrock y Agent Platform de Google Cloud desde v2.1.207. Antes de v2.1.207, `default` se resolvía a Opus 4.7 en Claude Platform en AWS y a Sonnet 4.5 en Amazon Bedrock y Agent Platform de Google Cloud.

Cuando un administrador ha establecido un [modelo predeterminado de la organización](#organization-default-model), `default` se resuelve a ese modelo en lugar del predeterminado del tipo de cuenta anterior. Requiere Claude Code v2.1.196 o posterior. `default` también puede resolverse al modelo que estableces con [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), bajo las condiciones enumeradas en su sección.

Cuando la configuración administrada [aplica la lista de permitidos para el modelo Default](#enforce-the-allowlist-for-the-default-model) y el predeterminado del tipo de cuenta no está en `availableModels`, `default` se resuelve al Default aplicado en lugar del predeterminado del tipo de cuenta anterior. Cuando ambos se aplican, el modelo predeterminado de la organización reemplaza primero el predeterminado del tipo de cuenta y luego se aplica la aplicación: un modelo predeterminado de la organización en la lista de permitidos se mantiene, mientras que uno fuera de la lista se resuelve al Default aplicado.

Los modelos Fable no son el predeterminado del tipo de cuenta en ningún plan ni proveedor. Elegir uno con `/model` lo guarda como el modelo seleccionado en tu configuración de usuario, por lo que las sesiones posteriores comienzan en él. Para el cambio único que Claude Code realiza en una selección de Fable 5 guardada en v2.1.257, consulta [Trabajar con Fable](#work-with-fable).

<h3 id="opusplan-model-setting">
  Configuración del modelo `opusplan`
</h3>

El alias del modelo `opusplan` proporciona un enfoque híbrido automatizado:

* **En plan mode**: utiliza `opus` para razonamiento complejo y decisiones de arquitectura
* **En execution mode**: cambia automáticamente a `sonnet` para generación de código e implementación

Esto combina el razonamiento de Opus para la planificación con la eficiencia de Sonnet para la ejecución.

La fase Opus del plan mode utiliza la misma ventana de contexto que la configuración del modelo `opus`, y la fase de ejecución utiliza la misma ventana que `sonnet`. Cuando `opus` y `sonnet` se resuelven a modelos que se ejecutan con la [ventana de contexto de 1M](#extended-context) por defecto, como lo hacen los modelos actuales en Anthropic API, ambas fases se ejecutan con ella. Para solicitar contexto de 1M para ambas fases donde no lo hacen, [establece el modelo](#setting-your-model) a `opusplan[1m]`, por ejemplo con `/model opusplan[1m]`. Establecerlo con `/model` requiere Claude Code v2.1.265 o posterior; en versiones anteriores, utiliza la bandera `--model` o la configuración `model` en su lugar.

Cuando [`availableModels`](#restrict-model-selection) excluye el Opus más nuevo pero permite una versión anterior, por ejemplo `["sonnet", "claude-opus-4-6"]`, `opusplan` utiliza el Opus más nuevo permitido para la planificación y se mantiene solo en Sonnet cuando se excluye cada Opus. Una sesión de Haiku que normalmente se actualizaría a Sonnet en plan mode utiliza de manera similar el Sonnet más nuevo permitido, y se mantiene solo en Haiku cuando se excluye cada Sonnet. Antes de v2.1.205, el plan mode se mantenía en el modelo de la sesión siempre que se excluyera la versión más nueva de la familia de actualización, incluso cuando la lista de permitidos permitía una anterior.

La sustitución de una versión anterior permitida se aplica en Anthropic API y [Claude Platform en AWS](/docs/es/claude-platform-on-aws). En Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry y Mantle, cuyas implementaciones utilizan ID de modelo específicos del proveedor, el plan mode se mantiene en el modelo de la sesión siempre que se excluya el modelo de actualización.

Para un enfoque híbrido donde Claude decide a mitad de la tarea cuándo consultar un segundo modelo en lugar de cambiar en el límite del plan, consulta la [herramienta advisor](/docs/es/advisor).

<h3 id="fallback-model-chains">
  Cadenas de modelos de respaldo
</h3>

Cuando el modelo principal está sobrecargado, no disponible o devuelve otro error de servidor no reintentable, Claude Code puede cambiar a un modelo de respaldo en lugar de fallar la solicitud. Los errores de autenticación, facturación, límite de velocidad, tamaño de solicitud y transporte, y una [denegación por la verificación de política de tu organización](/docs/es/errors#automatic-retries), nunca desencadenan un cambio; esos siguen su manejo normal de reintentos y errores.

Configura uno o más modelos de respaldo y Claude Code los intenta en orden, mostrando un aviso cuando cambia. El cambio dura solo el turno actual, por lo que tu siguiente mensaje intenta el modelo principal primero nuevamente. Claude Code limita las cadenas a tres modelos después de la eliminación de duplicados e ignora entradas adicionales.

Establece una cadena para una sesión con la bandera `--fallback-model`, que acepta una lista separada por comas:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Para persistir una cadena entre sesiones, establece `fallbackModel` en [settings](/docs/es/settings) como un array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

La bandera `--fallback-model` tiene precedencia sobre la configuración `fallbackModel`. Cada entrada acepta un nombre de modelo o alias, y `"default"` se expande al modelo predeterminado.

Claude Code no confirma la cadena al inicio y `/status` no la muestra. El aviso mostrado cuando ocurre un cambio es el primer signo visible de que se ha configurado un respaldo.

Cuando una solicitud falla, Claude Code intenta cada entrada en orden hasta que una la acepta. Una entrada que tampoco se puede alcanzar, como un modelo retirado fijado en la configuración, falla a la siguiente de la misma manera. Claude Code elimina dos tipos de entrada antes de ese recorrido:

* **Fuera de la lista de permitidos**: Claude Code descarta cualquier entrada no permitida por [`availableModels`](#restrict-model-selection) cuando lee la cadena.
* **Ventana de contexto más pequeña durante la compactación**: la cadena también cubre [compactación](/docs/es/context-window#what-survives-compaction), pero Claude Code no retrocederá a un modelo con una ventana de contexto más pequeña que la del principal, ya que resumir allí cortaría parte de la conversación primero. Si cada respaldo es más pequeño, la compactación muestra el error original y puedes reintentar.

Claude Code también aplica la cadena a [subagentes](/docs/es/sub-agents). Cuando la solicitud de un subagente falla, Claude Code intenta tus modelos de respaldo configurados en orden, y el subagente continúa en el modelo que acepta la solicitud. El modelo de tu sesión no cambia. Antes de v2.1.247, un fallo que la cadena cubría terminaba el subagente en su lugar.

<h3 id="automatic-model-fallback">
  Respaldo automático de modelo
</h3>

Esta sección cubre el respaldo basado en contenido de modelos Fable, Opus 5.5 y Opus 5. Para respaldo basado en disponibilidad cuando un modelo está sobrecargado o no disponible, consulta [Cadenas de modelos de respaldo](#fallback-model-chains).

Los modelos Fable, Opus 5.5 y Opus 5 se ejecutan con clasificadores de seguridad, que con mayor frecuencia marcan contenido de ciberseguridad y biología. Cuando un clasificador marca una solicitud y la categoría marcada tiene un modelo de respaldo, Claude Code vuelve a ejecutar la solicitud en ese modelo y muestra un aviso en la transcripción. Para esas dos categorías, el modelo de respaldo depende de qué modelo rechazó:

* **Fable 5.1, Fable 5 y Opus 5.5**: las solicitudes marcadas por biología se vuelven a ejecutar en Opus 5, y las solicitudes marcadas por ciberseguridad se vuelven a ejecutar en Opus 4.8.
* **Opus 5**: las solicitudes marcadas por ciberseguridad se vuelven a ejecutan en Opus 4.8. Las solicitudes marcadas por biología terminan con un rechazo en su lugar, porque Opus 5 ejecuta sus propios clasificadores de biología sin modelo de respaldo.

En Amazon Bedrock, Agent Platform de Google Cloud y Microsoft Foundry, Claude Code resuelve estos objetivos a través de tu implementación en su lugar, y si estableces `ANTHROPIC_DEFAULT_OPUS_MODEL`, las categorías que tienen un respaldo se vuelven a ejecutan en el modelo fijado; consulta [Habilitar respaldo en Bedrock, Agent Platform y Foundry](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Después de un respaldo, la sesión continúa en el modelo de respaldo. Para volver a tu modelo original, ejecuta [`/model`](#setting-your-model).

El respaldo basado en categoría requiere Claude Code v2.1.219 o posterior. Antes de v2.1.219, cada solicitud de Fable 5 marcada se volvía a ejecutar en el modelo Opus predeterminado de tu proveedor, y Opus 5 no era una fuente de respaldo.

El modelo de respaldo se verifica contra [`availableModels`](#restrict-model-selection). Cuando está bloqueado, no ocurre respaldo. El rechazo se muestra como un error normal y el modelo de la sesión no cambia.

<h4 id="check-what-triggered-fallback">
  Verificar qué desencadenó el respaldo
</h4>

El respaldo puede desencadenarse en la primera solicitud de una sesión, antes de que envíes algo inusual, porque la primera solicitud lleva contexto del espacio de trabajo como tu contenido de CLAUDE.md y estado de git. Un repositorio que contiene material de seguridad o biología puede activar el clasificador solo en ese contexto.

Para verificar si las personalizaciones son el desencadenante, inicia una sesión con `claude --safe-mode`, que desactiva personalizaciones como CLAUDE.md, skills, servidores MCP y hooks. El estado de git y los nombres de directorios no son personalizaciones y aún se incluyen.

<h4 id="ask-before-switching">
  Preguntar antes de cambiar
</h4>

Para decidir qué sucede cada vez que se marca una solicitud, en lugar de cambiar automáticamente, ejecuta `/config` y desactiva **Cambiar modelos cuando se marca un mensaje**, o establece [`switchModelsOnFlag`](/docs/es/settings-reference#switchmodelsonflag) a `false` en tu archivo de configuración. Una solicitud marcada pausa la sesión con dos opciones: cambiar al modelo de respaldo, o editar el prompt e intentar nuevamente en el modelo actual.

Algunos casos se comportan de manera diferente:

* Cuando la categoría marcada no tiene modelo de respaldo, como un marcado de biología en Opus 5, Claude Code no muestra el prompt y la solicitud termina con el rechazo.
* Si ambos modelos marcan la misma solicitud, puedes editar el prompt e intentar nuevamente, o iniciar una nueva sesión.
* En sesiones móviles de [Claude Code en la web](/docs/es/claude-code-on-the-web), no se admite edición e reintento. Cambia modelos, o continúa la sesión desde un navegador de escritorio o la aplicación de escritorio.
* En [modo no interactivo](/docs/es/cli-reference#cli-flags) e integraciones de SDK que no pueden mostrar el prompt, una solicitud marcada termina el turno con un rechazo en su lugar.
* Cuando el objetivo de respaldo está bloqueado por [`availableModels`](#restrict-model-selection), Claude Code no muestra el prompt. La solicitud marcada termina con el rechazo, igual que el respaldo automático cuando el objetivo está bloqueado.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Habilitar respaldo en Bedrock, Agent Platform y Foundry
</h4>

En [Amazon Bedrock](/docs/es/amazon-bedrock), [Agent Platform de Google Cloud](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), los ID de modelo son específicos del proveedor, por lo que el respaldo automático solo funciona cuando Claude Code puede identificar ambos modelos involucrados:

* Claude Code debe reconocer el modelo actual como una fuente de respaldo. Fable 5.1 y Fable 5 se reconocen cuando el ID del modelo contiene `claude-fable-5`, coincide con el valor de `ANTHROPIC_DEFAULT_FABLE_MODEL`, o se asigna con [`modelOverrides`](#override-model-ids-per-version). Opus 5.5 y Opus 5 se reconocen por su ID de modelo del proveedor o una asignación de [`modelOverrides`](#override-model-ids-per-version).
* El modelo de respaldo debe resolverse en tu implementación. Si estableces `ANTHROPIC_DEFAULT_OPUS_MODEL`, las solicitudes marcadas se vuelven a ejecutan en ese modelo para cada categoría que tiene un respaldo; un marcado de biología en Opus 5 aún termina con un rechazo. Si no lo estableces, las solicitudes marcadas por ciberseguridad se vuelven a ejecutan en una entrada de Opus 4.8 en la lista de modelos del proveedor, y las solicitudes marcadas por biología de un modelo Fable o Opus 5.5 en una entrada de Opus 5.

Si alguno de los modelos no se puede identificar, Claude Code no cambia automáticamente. La solicitud marcada termina con un mensaje de rechazo, y puedes cambiar modelos con [`/model`](#setting-your-model) e intentar nuevamente. Establecer `ANTHROPIC_DEFAULT_FABLE_MODEL` a tu ID de modelo Fable habilita el reconocimiento de Fable. Establecer `ANTHROPIC_DEFAULT_OPUS_MODEL` a un ID de modelo Opus proporciona a las categorías marcadas un objetivo de respaldo, a menos que el pin nombre un modelo fuera de la familia Opus o el modelo que rechazó; entonces Claude Code no cambia y el rechazo se mantiene.

<h4 id="security-research-and-biology-workloads">
  Cargas de trabajo de investigación de seguridad y biología
</h4>

Las cargas de trabajo en seguridad ofensiva o biología, incluidas pruebas de penetración, ejercicios Capture the Flag (CTF) y bases de código adyacentes a biología, desencadenan respaldo frecuentemente, a menudo en la primera solicitud. Para trabajo sustancial de biología en Fable 5.1, Fable 5 u Opus 5.5, Claude Code mueve la sesión a Opus 5 en la primera solicitud marcada, y las solicitudes posteriores marcadas por biología terminan en rechazos allí, porque Opus 5 no tiene respaldo de biología. En Opus 5, obtienes esos rechazos desde la primera solicitud marcada.

Este es el enrutamiento esperado para estos dominios, no un marcado de cuenta. Si tu organización necesita capacidad de clase Fable para este trabajo, pregunta a tu equipo de cuenta de Anthropic sobre programas de acceso de confianza.

<h3 id="adjust-effort-level">
  Ajustar el nivel de esfuerzo
</h3>

Los [niveles de esfuerzo](https://platform.claude.com/docs/en/build-with-claude/effort) controlan el razonamiento adaptativo, que permite al modelo decidir si y cuánto pensar en cada paso según la complejidad de la tarea. El esfuerzo menor es más rápido y económico para tareas directas, mientras que el esfuerzo mayor proporciona razonamiento más profundo para problemas complejos.

Los niveles de esfuerzo disponibles dependen del modelo. Los modelos no enumerados aquí no admiten esfuerzo:

| Modelo                                          | Niveles                                 |
| :---------------------------------------------- | :-------------------------------------- |
| Fable 5.1 y Fable 5                             | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8 y Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 y Sonnet 4.6                           | `low`, `medium`, `high`, `max`          |

Si estableces un nivel que el modelo activo no admite, Claude Code retrocede al nivel más alto admitido en o por debajo del que estableces. Por ejemplo, `xhigh` se ejecuta como `high` en Opus 4.6. Tu organización o tu propia configuración también pueden limitar los niveles que ofrece un modelo; consulta [Límites de esfuerzo de la organización](#organization-effort-limits).

Con la configuración [`ultracode`](/docs/es/settings-reference#ultracode) desactivada, Claude Code resuelve el nivel de esfuerzo de la sesión en este orden, tomando el primero que se aplique:

1. Una opción explícita: la variable de entorno [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/es/env-vars#variables), lanzar con `--effort`, o `/effort` en la sesión ([un `/effort` no interactivo tiene efecto más estrecho](#non-interactive-effort))
2. Tu configuración: el nivel que guardaste para el modelo o una clave [`effortLevel`](/docs/es/settings-reference#effortlevel), con la precedencia entre ellos y entre archivos de configuración indicada en [`modelSettings`](/docs/es/settings-reference#modelsettings)
3. El esfuerzo predeterminado del modelo: `high` en cada modelo que admite esfuerzo, excepto que Opus 5.5 por defecto es `medium`, Opus 4.7 por defecto es `xhigh` y, cuando tu organización establece un nivel de esfuerzo predeterminado para su [modelo predeterminado de la organización](#organization-default-model), ese nivel es el predeterminado cuando ejecutas ese modelo

Opus 5.5 comienza en `medium` a menos que una de las fuentes anteriores establezca un nivel para él, y una `effortLevel` de nivel superior en tu archivo de configuración de usuario no cuenta para Opus 5.5. Esa clave es la forma anterior que `/effort` escribía antes de que Claude Code guardara niveles por modelo: sigue aplicándose donde se aplicaba antes, en Opus 5, Fable 5.1 y modelos anteriores, mientras que Opus 5.5 y modelos lanzados después de él comienzan en su propio predeterminado hasta que elijas un nivel para ellos con `/effort` o el selector `/model`. Una `effortLevel` de nivel superior en configuración de proyecto, local o administrada, o una pasada con `--settings`, se aplica a cada modelo.

Cuando estableces `low`, `medium`, `high` o `xhigh` en una sesión interactiva en tu máquina, eliges cuánto tiempo dura eligiendo cómo lo confirmas:

* `Enter` en el deslizador `/effort` o el selector `/model`, o un nivel escrito después de `/effort`: guarda el nivel como tu predeterminado y aplícalo en sesiones posteriores
* `s` en el deslizador `/effort` o el selector `/model`: aplica el nivel solo a esta sesión. Requiere Claude Code v2.1.257 o posterior

Claude Code guarda el nivel por modelo, bajo la clave [`modelSettings`](/docs/es/settings-reference#modelsettings) en tu configuración de usuario, por lo que cada modelo mantiene su propio nivel guardado.

`max` es el nivel de razonamiento más profundo. A menos que lo establezas a través de la variable de entorno `CLAUDE_CODE_EFFORT_LEVEL`, Claude Code aplica `max` solo a la sesión actual.

<Note>
  Un nivel que eliges del control de esfuerzo en un teléfono o navegador conectado a través de [Control Remoto](/docs/es/remote-control#what-connected-devices-see) se aplica solo a esa sesión.
</Note>

<span id="non-interactive-effort" />

Cuando estableces un nivel con `/effort` en una ejecución de [`-p`](/docs/es/headless), Claude Code lo aplica solo a esa sesión y no lo guarda como tu predeterminado.

El menú `/effort` también ofrece `ultracode`. Ultracode es una configuración de Claude Code en lugar de un nivel de esfuerzo del modelo: envía `xhigh` al modelo y además tiene Claude orquestar [flujos de trabajo dinámicos](/docs/es/workflows) para tareas sustanciales. Para dónde se puede establecer persistentemente, consulta la configuración [`ultracode`](/docs/es/settings-reference#ultracode).

Puedes activar ultracode a través de cualquiera de los siguientes:

* **`/effort`**: ejecuta `/effort ultracode`, o selecciónalo del menú
* **Bandera `--effort`**: lanza con `claude --effort ultracode`, que inicia la sesión con esfuerzo `xhigh` y ultracode activado
* **Configuración `ultracode`**: establece [`"ultracode": true`](/docs/es/settings-reference#ultracode) en un archivo de configuración, con `--settings`, o en una solicitud de control del Agent SDK. Una solicitud [`applyFlagSettings()`](/docs/es/agent-sdk/typescript#applyflagsettings) también acepta `effortLevel: "ultracode"`
* **Selector `/model`**: mueve el deslizador de esfuerzo a `ultracode` con las teclas de flecha mientras eliges un modelo. Claude Code lo activa para la sesión actual, incluso cuando guardas ese modelo como tu predeterminado

Pasar `ultracode` a la bandera `--effort` o al valor `effortLevel` del Agent SDK requiere Claude Code v2.1.203 o posterior. Antes de v2.1.203, `--effort ultracode` imprimía `Unknown --effort value 'ultracode'` y la sesión comenzaba con el esfuerzo predeterminado.

La configuración `effortLevel` persistida y la variable de entorno `CLAUDE_CODE_EFFORT_LEVEL` no aceptan `ultracode`. Cuando `CLAUDE_CODE_EFFORT_LEVEL` se establece a un nivel distinto de `xhigh`, las solicitudes se ejecutan en ese nivel y la orquestación de flujo de trabajo de ultracode permanece inactiva. Seleccionar ultracode entonces muestra una advertencia de que la variable de entorno anula el esfuerzo para la sesión.

<span id="when-ultracode-is-available" />

Ultracode no está disponible cuando:

* [Los flujos de trabajo están desactivados](/docs/es/workflows#turn-workflows-off)
* El modelo no admite esfuerzo `xhigh`
* Se aplica un [límite de esfuerzo](#organization-effort-limits) por debajo de `xhigh` al modelo

En esos casos `--effort ultracode` inicia la sesión con ultracode desactivado, al nivel de esfuerzo más alto que el modelo y cualquier límite permiten, hasta `xhigh`.

<h4 id="choose-an-effort-level">
  Elegir un nivel de esfuerzo
</h4>

Cada nivel intercambia gasto de tokens contra capacidad. El predeterminado se adapta a la mayoría de tareas de codificación; ajusta cuando quieras un equilibrio diferente.

| Nivel       | Cuándo usarlo                                                                                                                                                       |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `low`       | Reserva para tareas cortas, limitadas y sensibles a la latencia que no son sensibles a la inteligencia                                                              |
| `medium`    | Reduce el uso de tokens para trabajo sensible a costos que puede sacrificar algo de inteligencia. El predeterminado en Opus 5.5                                     |
| `high`      | Equilibra el uso de tokens e inteligencia. El predeterminado en cada modelo excepto Opus 5.5 y Opus 4.7                                                             |
| `xhigh`     | Razonamiento más profundo con gasto de tokens más alto. El predeterminado en Opus 4.7                                                                               |
| `max`       | Puede mejorar el rendimiento en tareas exigentes pero puede mostrar rendimientos decrecientes y es propenso a pensar demasiado. Prueba antes de adoptar ampliamente |
| `ultracode` | Una configuración de Claude Code que planifica un [flujo de trabajo dinámico](/docs/es/workflows) para cada tarea sustancial con razonamiento `xhigh` por mensaje        |

La escala de esfuerzo se calibra por modelo, por lo que el mismo nombre de nivel no representa el mismo valor subyacente entre modelos.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Usar ultrathink para razonamiento profundo único
</h4>

Incluye `ultrathink` en cualquier lugar en tu prompt para solicitar razonamiento más profundo en ese turno sin cambiar tu configuración de esfuerzo de sesión. Claude Code reconoce la palabra clave y añade una instrucción en contexto. El nivel de esfuerzo enviado a la API no cambia. Claude Code pasa otras frases como "think", "think hard" y "think more" como texto de prompt ordinario y no las reconoce como palabras clave.

<h4 id="set-the-effort-level">
  Establecer el nivel de esfuerzo
</h4>

Puedes cambiar el esfuerzo a través de cualquiera de los siguientes:

* **`/effort`**: ejecuta `/effort` sin argumentos para abrir un deslizador interactivo, `/effort` seguido de un nombre de nivel para establecerlo directamente, o `/effort auto` para borrar tu nivel guardado para el modelo activo. Puedes ejecutarlo mientras Claude está trabajando, y una vez que confirmes la [advertencia de caché](/docs/es/prompt-caching#changing-effort-level), si Claude Code muestra una, Claude Code aplica el nuevo nivel a la siguiente solicitud en el turno
* **En `/model`**: usa las teclas de flecha izquierda/derecha para ajustar el deslizador de esfuerzo cuando selecciones un modelo
* **Bandera `--effort`**: pasa un nombre de nivel para establecerlo para una única sesión cuando lanzas Claude Code
* **Variable de entorno**: establece `CLAUDE_CODE_EFFORT_LEVEL` a un nombre de nivel o `auto`
* **Configuración**: establece un nivel por modelo en [`modelSettings`](/docs/es/settings-reference#modelsettings), o establece [`effortLevel`](/docs/es/settings-reference#effortlevel) a `low`, `medium`, `high` o `xhigh` como el predeterminado para modelos sin uno. `max` no se acepta como un nivel en ninguna clave, y `ultracode` tiene su propia clave [`ultracode`](/docs/es/settings-reference#ultracode)
* **Desde un dispositivo conectado**: en una sesión de [Control Remoto](/docs/es/remote-control#what-connected-devices-see), elige un nivel del control de esfuerzo en tu teléfono o en tu navegador. El nivel se aplica solo a la sesión actual. Requiere Claude Code v2.1.234 o posterior
* **Frontmatter de skill y subagente**: establece `effort` en un archivo markdown de [skill](/docs/es/skills#frontmatter-reference) o [subagente](/docs/es/sub-agents#supported-frontmatter-fields) para anular el nivel de esfuerzo cuando ese skill o subagente se ejecuta

El esfuerzo de frontmatter se aplica cuando ese skill o subagente está activo, anulando el nivel de sesión pero no la variable de entorno. Un [`maxEffortLevel`](/docs/es/settings-reference#maxeffortlevel) o [límite de esfuerzo de la organización](#organization-effort-limits) aún limita el nivel en el que se ejecuta el skill o subagente.

Si estableces `effortLevel` en [configuración administrada](/docs/es/managed-settings), Claude Code lo aplica en el paso de configuración del [orden de resolución de esfuerzo](#adjust-effort-level), y los usuarios aún pueden cambiar el nivel con `/effort` o `--effort`. Para mantener a los usuarios en o por debajo de un nivel, establece [`maxEffortLevel`](/docs/es/settings-reference#maxeffortlevel).

El deslizador de esfuerzo aparece en `/model` cuando se selecciona un modelo admitido. El nivel de esfuerzo actual también se muestra en el encabezado de la sesión junto al nombre del modelo, por ejemplo "with low effort", para que puedas confirmar qué configuración está activa sin abrir `/model`. El pie de página también muestra brevemente el nivel de esfuerzo al inicio y cuando cambia.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Razonamiento adaptativo y presupuestos de pensamiento fijo
</h4>

El razonamiento adaptativo hace que el pensamiento sea opcional en cada paso, por lo que Claude puede responder más rápido a prompts rutinarios y reservar pensamiento más profundo para pasos que se benefician de él. Si quieres que Claude piense más o menos a menudo de lo que el nivel actual produce, puedes decirlo directamente en tu prompt o en `CLAUDE.md`; el modelo responde a esa orientación dentro de su configuración de esfuerzo.

Los modelos Fable, Sonnet 5 y Opus 4.7 y posteriores siempre utilizan razonamiento adaptativo. El modo de presupuesto de pensamiento fijo y `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` no se aplican a ellos.

En Opus 4.6 y Sonnet 4.6, puedes establecer `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` para revertir al presupuesto de pensamiento fijo anterior controlado por `MAX_THINKING_TOKENS`. Consulta [variables de entorno](/docs/es/env-vars).

<h3 id="extended-thinking">
  Pensamiento extendido
</h3>

El pensamiento extendido es el razonamiento que Claude emite antes de responder. En modelos que admiten [razonamiento adaptativo](#adjust-effort-level), el nivel de esfuerzo es el control principal de cuánto pensamiento ocurre; las configuraciones a continuación activan o desactivan el pensamiento y controlan cómo se muestra. Con el pensamiento desactivado en Anthropic API, Claude Code envía esfuerzo `high` en lugar de un nivel más alto a modelos que sabe que [no aceptan esa combinación](/docs/es/errors#effort-isnt-available-with-thinking-turned-off), como Opus 5.

| Control                                        | Cómo establecerlo                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Alternar para la sesión actual                 | Presiona `Option+T` en macOS o `Alt+T` en Windows y Linux                                                                                                                                                                                                                                                                                                                                                                                   |
| Establecer el predeterminado global            | Ejecuta `/config` y alterna el modo de pensamiento. Guardado como `alwaysThinkingEnabled` en `~/.claude/settings.json`                                                                                                                                                                                                                                                                                                                      |
| Desactivar a través de una variable de entorno | Establece [`MAX_THINKING_TOKENS=0`](/docs/es/env-vars), que desactiva el pensamiento en Anthropic API excepto en Opus 5.5 y modelos Fable. En [proveedores de terceros](/docs/es/third-party-integrations), Claude Code omite el parámetro `thinking` en su lugar, y los modelos de razonamiento adaptativo aún pueden pensar. Otros valores se aplican solo con un [presupuesto de pensamiento fijo](#adaptive-reasoning-and-fixed-thinking-budgets) |

No puedes desactivar el pensamiento en Opus 5.5 o los modelos Fable. El alternador de sesión, `alwaysThinkingEnabled` y `MAX_THINKING_TOKENS=0` no tienen efecto allí, y el modelo decide por paso cuánto pensar según el nivel de esfuerzo.

Claude Code colapsa la salida de pensamiento por defecto. Presiona `Ctrl+O` para alternar el modo detallado y ver el razonamiento como texto gris en cursiva. Las sesiones interactivas en Anthropic API reciben bloques de pensamiento redactados por defecto, así que establece `showThinkingSummaries: true` en [configuración](/docs/es/settings) si quieres que los resúmenes completos estén disponibles cuando expandas. Se te cobra por todos los tokens de pensamiento generados, incluso cuando están colapsados o redactados.

<h3 id="extended-context">
  Contexto extendido
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 y posteriores, y Sonnet 4.6 admiten una [ventana de contexto de 1 millón de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sesiones largas con bases de código grandes.

En Anthropic API, Fable 5.1, Fable 5, Sonnet 5 y Opus 4.7 y posteriores se ejecutan con la ventana de 1M en cada plan, incluido Pro. No seleccionas una variante `[1m]` ni activas créditos de uso para la ventana de 1M en estos modelos. El uso de Fable en sí puede facturarse a créditos de uso en algunos planes; consulta [Fable y créditos de uso](#fable-and-usage-credits).

Opus 4.6 y Sonnet 4.6 alcanzan 1M solo a través de su variante `[1m]`, y el acceso a esa variante depende de tu plan. En planes Max, Team y Enterprise, incluidos tanto asientos de Team Standard como Team Premium, Opus 4.6 con contexto de 1M se incluye con tu suscripción. Sonnet 4.6 con contexto de 1M requiere [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) en cada plan de suscripción, incluido Max.

| Plan                   | Opus 4.6 con contexto de 1M                                                                                   | Sonnet 4.6 con contexto de 1M                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Max, Team y Enterprise | Incluido con suscripción                                                                                      | Requiere [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                    | Requiere [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Requiere [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API y pago por uso     | Acceso completo                                                                                               | Acceso completo                                                                                               |

Claude Code verifica estos requisitos de plan solo cuando se conecta directamente a Anthropic API. Si apuntas `ANTHROPIC_BASE_URL` a una [puerta de enlace LLM](/docs/es/llm-gateway#subscriptions-and-gateways) y tu inicio de sesión guardado de claude.ai permanece como la credencial activa, Claude Code no verifica los créditos de uso de tu plan. Las opciones `[1m]` permanecen disponibles en `/model`, y la puerta de enlace decide si la solicitud tiene éxito. Antes de v2.1.229, Claude Code rechazaba `/model sonnet[1m]` en esa configuración cuando no podía confirmar créditos de uso en la cuenta.

Para desactivar contexto de 1M, establece `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code elimina variantes de modelo de 1M del selector de modelos. En modelos con una ventana nativa de 1M, como Sonnet 5 y los modelos Fable, también trata el modelo como si tuviera una ventana de contexto de 200K:

* Con compactación automática activada, las sesiones se compactan en el límite de 200K a través de [compactación automática](#set-the-auto-compact-window). Establecer la ventana de compactación automática por encima de 200K no levanta la retención, porque Claude Code limita esa ventana a la ventana de contexto del modelo.
* Con compactación automática desactivada, las sesiones se detienen en el límite de 200K con el [error de límite de contexto](/docs/es/errors#prompt-is-too-long) en lugar de compactarse.

Antes de v2.1.223, Claude Code mantenía solo sesiones de Sonnet 5, Opus 4.8 y Opus 5 a 200K. Consulta [variables de entorno](/docs/es/env-vars).

La ventana de contexto de 1M utiliza precios de modelo estándar sin prima para tokens más allá de 200K. Para planes donde el contexto extendido se incluye con tu suscripción, el uso permanece cubierto por tu suscripción. Para planes que acceden a contexto extendido a través de créditos de uso, los tokens se facturan a créditos de uso.

Si tu cuenta admite contexto de 1M, la opción aparece en el selector `/model` en las últimas versiones de Claude Code. Si no la ves, intenta reiniciar tu sesión.

También puedes usar el sufijo `[1m]` con alias de modelo o nombres de modelo completos:

```text theme={null}
# Usa el alias opus[1m] o sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# O añade [1m] a un nombre de modelo completo
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Ventana de contexto de Sonnet 5
</h4>

En Anthropic API, Sonnet 5 siempre se ejecuta con la ventana de contexto de 1M. No hay variante de 200K, no hay sufijo `[1m]` para seleccionar, y no se requieren créditos de uso en ningún plan. Las sesiones se compactan automáticamente antes de que la ventana se llene, a aproximadamente 967K tokens por defecto; establece [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars) para elegir un umbral diferente.

Dos configuraciones presupuestan la ventana en 200K en su lugar:

* **Puerta de enlace LLM**: cuando `ANTHROPIC_BASE_URL` apunta a una [puerta de enlace](/docs/es/llm-gateway), Claude Code no puede verificar soporte de 1M. Para usar la ventana completa, selecciona Sonnet 5 (1M context) en el selector de modelos, que se asigna a `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: mantiene sesiones en cada modelo con una ventana nativa de 1M a una ventana de 200K; consulta [Contexto extendido](#extended-context) para cómo se aplica la retención. Útil para implementaciones que necesitan limitar el contexto.

<h2 id="context-window-and-auto-compaction">
  Ventana de contexto y auto-compactación
</h2>

La ventana de auto-compactación es qué tan llena puede estar la ventana de contexto antes de que Claude Code compacte la conversación. Para ver qué compactación conserva y descarta por mecanismo, consulte [Lo que sobrevive a la compactación](/docs/es/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Establecer la ventana de auto-compactación
</h3>

Puede establecer la ventana de auto-compactación en tres lugares:

* **Para esta sesión y las posteriores**: ejecute `/autocompact` con un valor, como `/autocompact 500k`. Claude Code lo guarda en la configuración de usuario como [`autoCompactWindow`](/docs/es/settings-reference#autocompactwindow) y lo aplica a la sesión actual; si un [ámbito de configuración](/docs/es/settings#settings-precedence) de mayor prioridad, como la configuración administrada, establece la clave, el comando guarda su valor pero la sesión mantiene la ventana de ese ámbito, y el comando lo indica. Ejecute `/autocompact auto` para volver a la ventana ajustada para su modelo.
* **Para un lanzamiento**: pase [`--autocompact`](/docs/es/cli-reference#cli-flags) al iniciar Claude Code. La bandera anula su configuración guardada para ese lanzamiento sin cambiarla, y `claude --autocompact auto` ejecuta la sesión en la ventana ajustada incluso si su configuración guardada tiene un valor. A diferencia de `/autocompact`, la bandera no es anulada por un ámbito de configuración de mayor prioridad, como la configuración administrada.
* **En scripts y entornos en la nube**: establezca [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars). Mientras esté establecida, tiene precedencia sobre el comando, la bandera y la configuración, y `/autocompact` reporta la anulación en lugar de cambiar la ventana.

El comando y la bandera aceptan un tamaño de ventana de 100K a 1M de tokens, en cualquiera de estas formas:

* Un recuento de tokens simple, como `200000`
* Un sufijo `k` o `M`, como `500k` o `1M`
* Un número simple de 100 a 1000, que significa miles, por lo que `200` establece 200.000

La variable de entorno acepta solo el recuento de tokens simple. Claude Code limita la ventana al tamaño de la ventana de contexto del modelo.

<h3 id="default-auto-compact-thresholds">
  Umbrales de auto-compactación predeterminados
</h3>

Si no establece una ventana de auto-compactación, Claude Code compacta cuando la conversación alcanza el límite de contexto del modelo, excepto en estas sesiones:

* Las [sesiones en la nube](/docs/es/claude-code-on-the-web) se compactan cuando la conversación se acerca al límite del modelo
* Sonnet 4.6 y Opus 4.6 sin [contexto extendido](#extended-context) se compactan en el límite de 200K, y también lo hacen Opus 4.8 y posteriores cuando se ejecutan con una ventana de contexto de 200K, como en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry
* Cuando establece [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/es/env-vars), los modelos con una ventana nativa de 1M, como Sonnet 5 y los modelos Fable, se compactan en el límite de 200K
* Los modelos que se ejecutan con una ventana nativa de 1M, como Sonnet 5, los modelos Fable y Opus 4.7 y posteriores en la API de Anthropic, se compactan antes de que la ventana se llene, a aproximadamente 967K tokens de forma predeterminada. En Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, [Fijar modelos para implementaciones de terceros](#pin-models-for-third-party-deployments) indica qué modelos se ejecutan con esa ventana; para las configuraciones que presupuestan Sonnet 5 en 200K en su lugar, consulte [Ventana de contexto de Sonnet 5](#sonnet-5-context-window)
* Las sesiones en un ID de modelo que Claude Code no reconoce, como un alias de [puerta de enlace LLM](/docs/es/llm-gateway), se compactan en la ventana de contexto que Claude Code asume para el ID; consulte [Corregir la ventana para una puerta de enlace o ID de modelo personalizado](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Corregir la ventana para una puerta de enlace o ID de modelo personalizado
</h3>

En una [puerta de enlace LLM](/docs/es/llm-gateway) u otra implementación personalizada, Claude Code puede asumir una ventana de contexto para el ID del modelo que difiere de la ventana real del modelo, independientemente de si resuelve el ID a un modelo Claude. Establezca [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/es/env-vars) a la ventana que Claude Code debe asumir en su lugar.

Cómo se aplica la variable depende del ID. Claude Code trata un ID como un proveedor o una ortografía personalizada cuando no comienza con `claude-`, en cualquier caso, o cuando lleva un sufijo que Claude Code elimina al leer el ID, como la fecha `@YYYYMMDD` utilizada en Google Cloud's Agent Platform. Antes de v2.1.259, Claude Code no contaba un sufijo eliminado, por lo que un ID `claude-` no reconocido con un sufijo de fecha se trataba como un nombre `claude-` simple.

Un proveedor no reconocido o una ortografía personalizada, la misma ortografía con `[1m]` y todos los demás ID son tres casos separados:

* Si Claude Code no puede resolver un proveedor o una ortografía personalizada a un modelo que reconoce y el ID no contiene `[1m]`, la variable se aplica directamente y la compactación proactiva continúa en la ventana declarada.
* Si Claude Code no puede resolver un proveedor o una ortografía personalizada a un modelo que reconoce y el ID contiene `[1m]`, en cualquier caso, Claude Code asume una ventana de 1M para él y la variable no se aplica por sí sola. Para corregir la ventana mientras se mantiene la compactación proactiva, también establezca [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/es/env-vars). Con esa variable establecida, Claude Code dimensiona el ID como la misma ortografía sin `[1m]`, por lo que `CLAUDE_CODE_MAX_CONTEXT_TOKENS` se aplica cuando se aplicaría a esa ortografía sin etiquetar.

  Con una ventana declarada superior a 200K, Claude Code muestra una [advertencia de inicio](/docs/es/errors#the-200k-limit-isnt-enforced) de que el límite de 200K no se aplica. La advertencia es esperada en esta configuración.
* Si el ID se resuelve a un modelo que Claude Code reconoce, o el ID es un nombre `claude-` simple sin sufijo para que Claude Code elimine, en cualquier caso, la variable tiene efecto solo cuando también establece [`DISABLE_COMPACT`](/docs/es/env-vars), que desactiva toda compactación.

  Por ejemplo, un ID que contiene un nombre de modelo Claude que Claude Code conoce, como `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, o el fechado `claude-sonnet-4-5@20250929`, se resuelve a ese modelo. Esto incluye IDs que también contienen `[1m]`: Claude Code resuelve `claude-opus-4-8[1m]` a Opus 4.8 incluso con `CLAUDE_CODE_DISABLE_1M_CONTEXT` establecida.

Para un ID de modelo que Claude Code no reconoce, establezca [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/es/env-vars) para que Claude Code compacte solo después de que la API rechace la conversación con un [error demasiado largo que Claude Code reconoce](/docs/es/errors#prompt-is-too-long). Claude Code no ejecuta esa recuperación cuando una puerta de enlace [reescribe el error](/docs/es/llm-gateway-connect#troubleshoot-gateway-errors) a una redacción que Claude Code no reconoce.

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

Para enumerar varios modelos en su propio orden y bajo etiquetas que elija, establezca [`modelPicker`](/docs/es/settings-reference#modelpicker). Su entrada indica qué filas mantiene el selector cuando esa alineación reemplaza la integrada.

Este ejemplo establece las tres variables para hacer que una implementación de Opus enrutada por puerta de enlace sea seleccionable. Claude Code lee variables de entorno al iniciarse, por lo que ejecute las exportaciones antes de lanzar `claude`, o reinicie una sesión existente para aplicarlas:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` y `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` son opcionales:

* Si omite el nombre, la entrada muestra el nombre del modelo cuando Claude Code [reconoce el ID](#customize-pinned-model-display-and-capabilities), y el ID del modelo en caso contrario.
* Si omite la descripción, Claude Code utiliza `Custom model (<model-id>)`.

Claude Code enumera la entrada personalizada después de las entradas integradas, y cualquier fila de [`modelPicker`](/docs/es/settings-reference#modelpicker) que agregue viene después de ella.

Claude Code omite la validación para el ID de modelo establecido en `ANTHROPIC_CUSTOM_MODEL_OPTION`, por lo que puede utilizar cualquier cadena que su punto final de API acepte.

Cuando [`availableModels`](#restrict-model-selection) está establecido, incluya también el ID de modelo personalizado en la lista de permitidos. De lo contrario, Claude Code filtra la entrada personalizada del selector y rechaza una selección de `--model` de la misma como cualquier otro modelo excluido.

Un ID personalizado que incrusta un nombre de familia, como `my-gateway/claude-opus-5-5`, cuenta como una entrada específica para esa familia y deshabilita su comodín, por lo que también debe enumerar las versiones que desea mantener seleccionables. Consulte [Comportamiento de fusión](#merge-behavior).

<h2 id="environment-variables">
  Variables de entorno
</h2>

Utilice las siguientes variables de entorno para controlar los nombres de modelo a los que se asignan los alias. Cada valor debe ser un nombre de modelo completo, o el identificador equivalente para su proveedor de API. Para elegir el modelo en el que comienzan sus sesiones, establezca [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), que esta tabla omite.

| Variable de entorno              | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | El modelo a utilizar para `fable`, y el ID de modelo que Claude Code reconoce como modelo Fable para [alternancia automática de modelo](#automatic-model-fallback) en proveedores de terceros                                                                                                                                                                                                                                                                                                                                           |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | El modelo a utilizar para `opus`, o para `opusplan` cuando Plan Mode está activo.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | El modelo a utilizar para `sonnet`, o para `opusplan` cuando Plan Mode no está activo.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | El modelo a utilizar para `haiku`, o [funcionalidad de fondo](/docs/es/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | El modelo predeterminado para [subagentes](/docs/es/sub-agents#choose-a-model), compañeros de [equipo de agentes](/docs/es/agent-teams#specify-teammates-and-models) y agentes de [workflow](/docs/es/workflows) que no tienen asignado un modelo de otra manera. Acepta un alias como `haiku` o un nombre de modelo completo. Un modelo por invocación o el campo `model` de una definición, incluido `inherit`, tiene precedencia. Para cambiar eso, establezca [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/es/sub-agents#run-every-subagent-on-one-model) |

Nota: `ANTHROPIC_SMALL_FAST_MODEL` está deprecado en favor de `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Fijar modelos para implementaciones de terceros
</h3>

Cuando implemente Claude Code a través de [Amazon Bedrock](/docs/es/amazon-bedrock), [Plataforma de Agentes de Google Cloud](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry), o [Claude Platform on AWS](/docs/es/claude-platform-on-aws), fije versiones de modelo antes de implementar para usuarios.

Sin fijar, Claude Code utiliza alias de modelo como `fable`, `opus`, `sonnet` y `haiku` que se resuelven a un ID de modelo predeterminado integrado para cada proveedor. Ese predeterminado puede rezagarse con respecto a la versión más reciente de Anthropic, y el modelo al que apunta puede que aún no esté habilitado en la cuenta de un usuario. Cuando el predeterminado no está disponible, los usuarios de Amazon Bedrock y Plataforma de Agentes de Google Cloud ven un aviso y la sesión retrocede a una versión anterior del modelo predeterminado, o al modelo Sonnet predeterminado cuando el predeterminado es un modelo Opus y no hay versión de Opus disponible. Los usuarios de Microsoft Foundry ven errores en su lugar, porque Microsoft Foundry no tiene ninguna verificación de inicio equivalente.

En Amazon Bedrock y Plataforma de Agentes de Google Cloud, un usuario que inicia la sesión en una versión específica de Sonnet u Opus, por ejemplo con `--model`, `ANTHROPIC_MODEL`, o la configuración `model`, fija esa versión como el predeterminado de la sesión para el alias coincidente: la verificación de inicio omite el predeterminado integrado que reemplaza y no muestra ningún aviso de alternancia. Antes de v2.1.211, la verificación se ejecutaba y podía mostrar un aviso incluso cuando un modelo de sesión estaba configurado explícitamente.

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

Para habilitar [contexto extendido](#extended-context) para un modelo fijo, añada `[1m]` al ID de modelo en `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, o `ANTHROPIC_DEFAULT_FABLE_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Con el sufijo `[1m]`, la ventana de contexto de 1M se aplica a todo el uso del alias fijo, incluida la fase Opus de modo de plan de [`opusplan`](#opusplan-model-setting) y [subagentes](/docs/es/sub-agents#choose-a-model) cuyo frontmatter `model` nombra el alias.

* Claude Code elimina el sufijo antes de enviar el ID de modelo a su proveedor.
* Solo añada `[1m]` cuando el modelo subyacente [admita contexto de 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* El sufijo se lee por variable, no por modelo. En Amazon Bedrock, Plataforma de Agentes de Google Cloud y Microsoft Foundry, un ID de modelo sin `[1m]` en una variable utiliza contexto de 200K incluso si otra variable establece el mismo modelo con el sufijo. Sonnet 5 siempre se ejecuta con la ventana de 1M en estos proveedores y nunca necesita el sufijo.

<Note>
  Una lista de permitidos `availableModels` entregada a través de [MDM o un archivo de configuración administrado](/docs/es/managed-settings#delivery-mechanisms) aún se aplica cuando se utilizan proveedores de terceros; [la configuración administrada por servidor no se entrega allí](/docs/es/server-managed-settings#platform-availability).

  El filtrado coincide con un alias de modelo como `opus`, un prefijo de versión como `claude-opus-4-8`, o el ID de modelo completo en forma de proveedor. Los prefijos específicos del proveedor como `us.anthropic.` no se eliminan, por lo que para permitir un modelo específico, enumere su ID completo en forma de proveedor, o asígnelo a través de [`modelOverrides`](#override-model-ids-per-version). Para un modelo fijo, ese ID es el valor que establece en su variable `ANTHROPIC_DEFAULT_*_MODEL`. Cualquier sufijo `[1m]` se elimina tanto de la entrada de la lista de permitidos como del modelo solicitado antes de coincidir.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personalizar la visualización y capacidades del modelo fijo
</h3>

Cuando fija un modelo en un proveedor de terceros, su fila en el selector `/model` muestra el nombre del modelo de forma predeterminada si Claude Code reconoce el ID fijo, y el ID sin procesar en caso contrario:

* **Reconocido**: el ID exacto de un modelo que Claude Code conoce, como su ID de API de Anthropic o la forma de su proveedor o puerta de enlace, con o sin el sufijo `[1m]`. Fije `us.anthropic.claude-sonnet-4-5-20250929-v1:0` y la fila lee `Sonnet 4.5`.
* **No reconocido**: cualquier otro ID, como un ARN de perfil de inferencia de aplicación o una versión de modelo que Claude Code no conoce, a menos que una entrada de [`modelOverrides`](#override-model-ids-per-version) asigne un modelo a esa cadena exacta. En Microsoft Foundry, los nombres de implementación están definidos por el usuario, por lo que Claude Code nunca reconoce un ID fijo allí, asignado o no, y la fila muestra el nombre de implementación de forma predeterminada.

Cuando una fila muestra el nombre del modelo, su descripción predeterminada incluye el ID fijo para que aún pueda ver qué ID está fijo.

Claude Code también puede no reconocer qué características admite un modelo fijo. Puede establecer el nombre de visualización y la descripción usted mismo y declarar capacidades con variables de entorno complementarias para cada modelo fijo.

Estas variables tienen efecto en proveedores de terceros como Amazon Bedrock, Plataforma de Agentes de Google Cloud y Microsoft Foundry. Las variables `_NAME` y `_DESCRIPTION` también tienen efecto cuando `ANTHROPIC_BASE_URL` apunta a una [puerta de enlace LLM](/docs/es/llm-gateway). No tienen efecto cuando se conecta directamente a `api.anthropic.com`.

| Variable de entorno                                   | Descripción                                                                                                                                                                                                   |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nombre de visualización para el modelo Opus fijo en el selector `/model`. Cuando no está configurado, la fila muestra el nombre del modelo si Claude Code reconoce el ID fijo, y el ID fijo en caso contrario |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Descripción de visualización para el modelo Opus fijo en el selector `/model`. Cuando no está configurado, la fila muestra una descripción predeterminada que comienza con `Custom Opus model`                |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Lista separada por comas de capacidades que admite el modelo Opus fijo                                                                                                                                        |

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

En plataformas que integran Claude Code y establecen [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars), la configuración de modelo del host tiene precedencia sobre la configuración de modelo administrada, mientras que una lista de permitidos `availableModels` administrada permanece en vigor a menos que el host proporcione la suya propia; [Excepciones a la precedencia de configuración administrada](/docs/es/settings#exceptions-to-managed-settings-precedence) dice qué claves y variables anula el host.

Las variables de entorno a nivel de familia anteriores configuran un ID de modelo por alias de familia. Si necesita asignar varias versiones dentro de la misma familia a IDs de proveedor distintos, utilice la configuración `modelOverrides` en su lugar.

`modelOverrides` asigna IDs de modelo individuales de Anthropic a las cadenas específicas del proveedor que Claude Code envía a la API de su proveedor. Cuando un usuario selecciona un modelo asignado en el selector `/model`, Claude Code utiliza su valor configurado en lugar del predeterminado integrado.

Esto permite a los administradores empresariales enrutar cada versión de modelo a un ARN de perfil de inferencia de Amazon Bedrock específico, nombre de versión de Plataforma de Agentes de Google Cloud o nombre de implementación de Microsoft Foundry para gobernanza, asignación de costos o enrutamiento regional.

Establezca `modelOverrides` en su [archivo de configuración](/docs/es/settings#where-settings-live):

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

Para detener la [línea de diagnóstico](/docs/es/errors#unrecognized-model-id-on-a-request) `[claude-code:unrecognized_model]` para un ID como un alias de puerta de enlace, añada una entrada con ese ID como su valor.

Las anulaciones reemplazan los IDs de modelo integrados que respaldan cada entrada en el selector `/model`. En Amazon Bedrock, las entradas de `modelOverrides` tienen precedencia sobre cualquier perfil de inferencia que Claude Code descubra automáticamente al inicio. Claude Code pasa valores que ya son específicos del proveedor, como ARNs de perfil de inferencia de Amazon Bedrock o nombres de implementación de Microsoft Foundry, al proveedor tal como están.

Las anulaciones también se aplican cuando pasa un ID de modelo de Anthropic directamente a través de `--model`, la variable de entorno `ANTHROPIC_MODEL`, o una variable de entorno `ANTHROPIC_DEFAULT_*_MODEL`. En Amazon Bedrock, Plataforma de Agentes de Google Cloud y [Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint), un ID de modelo de Anthropic sin entrada de `modelOverrides` se resuelve al mismo ID específico del proveedor que la fila del selector `/model` para esa versión, cuando el proveedor admite esa versión. Mantle admite un subconjunto de versiones. Para un ID de modelo de Anthropic fuera de ese subconjunto, Claude Code envía el ID sin procesar a Mantle sin asignarlo, a menos que una entrada de `modelOverrides` lo cubra. Antes de v2.1.200, `--model` y los valores de variable de entorno llegaban al proveedor tal como estaban sin pasar por el mapa de anulación.

`modelOverrides` funciona junto con `availableModels`. La lista de permitidos se evalúa contra el ID de modelo de Anthropic, no el valor de anulación, por lo que una entrada como `"opus"` en `availableModels` continúa coincidiendo incluso cuando las versiones de Opus se asignan a ARNs. Cuando `enforceAvailableModels` se establece en configuración administrada, el Predeterminado aplicado se resuelve a través de `modelOverrides` desde [configuración administrada](/docs/es/managed-settings#how-claude-code-combines-managed-sources) únicamente. La asignación de un administrador, como una versión fijada a un ARN de perfil de inferencia, se respeta en el Predeterminado aplicado. Las anulaciones de configuración de usuario o proyecto no la afectan.

Cuando `availableModels` se establece en [configuración administrada](/docs/es/managed-settings), solo `modelOverrides` de configuración administrada se aplican a un ID de modelo de Anthropic pasado directamente a través de `--model` o las variables de entorno anteriores. Claude Code ignora las anulaciones en configuración de usuario o proyecto para esos IDs, y nunca resuelve un ID que la lista administrada excluye a través de `modelOverrides` de ninguna fuente de configuración. Esta restricción de fuente administrada requiere Claude Code v2.1.200 o posterior. Consulte [Restringir la selección de modelo](#restrict-model-selection) para saber cómo se manejan los IDs bloqueados.

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

Para elegir el TTL de caché para la conversación principal y para subagentes por separado, consulte [elegir el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself). Para saber qué desencadena un error de caché, consulte [Cómo Claude Code utiliza el almacenamiento en caché de indicaciones](/docs/es/prompt-caching).
