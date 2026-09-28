> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escalar decisiones difíciles con la herramienta advisor

> Empareje su modelo principal con un modelo advisor más fuerte que Claude consulta en momentos clave durante una tarea.

<Note>
  La herramienta advisor es experimental y requiere la API de Anthropic. No está disponible en Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform o Microsoft Foundry. El comportamiento, los precios y la disponibilidad pueden cambiar.
</Note>

La herramienta advisor permite que Claude consulte un segundo modelo, típicamente más fuerte, en momentos clave durante una tarea, como antes de comprometerse con un enfoque, cuando se atasca en un error recurrente, o antes de declarar una tarea completada. El advisor recibe la conversación completa, incluidas todas las llamadas a herramientas y resultados, y devuelve orientación que Claude aplica antes de continuar.

El advisor se ejecuta del lado del servidor en la infraestructura de Anthropic como una [herramienta de servidor](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), disponible tanto para cuentas de suscripción como para cuentas facturadas por API. Usted elige qué modelo actúa como advisor, y Claude decide cuándo llamarlo.

Esta página cubre cómo habilitar el advisor, qué emparejamientos de modelos se aceptan, qué muestra Claude durante una consulta, y cómo se factura el uso del advisor.

<h2 id="when-to-use-the-advisor">
  Cuándo usar el advisor
</h2>

El advisor se ajusta a tareas largas y multietapa donde la mayoría de los turnos son rutinarios pero la calidad del plan determina el resultado. Los ejemplos incluyen refactorizaciones grandes, sesiones de depuración donde un error sigue repitiéndose, y tareas que desea que se verifiquen de forma independiente antes de que Claude las declare completadas.

Añade menos valor en tareas cortas donde hay poco que planificar, o en trabajo donde cada turno necesita el modelo más fuerte. Para esos casos, [cambie el modelo principal](/docs/es/model-config#setting-your-model) en su lugar, o vea [cómo se compara el advisor con opusplan y subagents](#compare-with-related-features) para otras formas de obtener una segunda opinión.

<h2 id="enable-the-advisor">
  Habilitar el advisor
</h2>

Puede configurar el modelo advisor de tres formas:

* **Comando `/advisor`**: establezca o cambie el advisor a mitad de sesión y guárdelo como su predeterminado
* **Configuración `advisorModel`**: configure un predeterminado persistente en su [archivo de configuración](/docs/es/settings)
* **Bandera `--advisor`**: establezca el advisor para una única sesión al iniciar

Cada una de estas opciones habilita el advisor para sesiones cuyo modelo principal [lo admite](#choose-an-advisor-model). Después de que la sesión comience, Claude Code muestra una notificación `Advisor Tool (experimental) is on and may use more tokens · /advisor`. Para dejar de usar el advisor, vea [Desactivar el advisor](#turn-the-advisor-off).

En algunos planes, Fable como advisor también necesita su [consentimiento único para facturar el uso de Fable a créditos de uso](/docs/es/model-config#fable-and-usage-credits). Para saber qué sucede antes de que haya dado ese consentimiento, vea [Advisor de Fable y créditos de uso](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Usar el comando `/advisor`
</h3>

Ejecute `/advisor` sin argumentos para abrir un selector que enumere los modelos advisor disponibles, o pase el modelo directamente:

```
/advisor opus
```

El comando confirma con `Advisor set to` seguido del nombre del modelo advisor. Su selección se guarda en `advisorModel` en su configuración de usuario y persiste entre sesiones, excepto en los casos que la [entrada `advisorModel`](/docs/es/settings-reference#advisormodel) enumera como aplicables solo a la sesión actual.

El comando también funciona donde no hay un selector de terminal: en [modo no interactivo](/docs/es/headless) con `-p`, en el Agent SDK, en la aplicación de escritorio y sobre [Control Remoto](/docs/es/remote-control). Esto requiere Claude Code v2.1.260 o posterior. En esas superficies:

* Ejecute `/advisor` sin argumento para imprimir el modelo advisor actual y los alias que acepta.
* Ejecute `/advisor` con un modelo, como `/advisor opus`, para establecerlo.
* Ejecute `/advisor off` para desactivarlo.

Claude Code no invoca un advisor guardado que la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización excluya. Para usar el advisor, seleccione un modelo permitido con `/advisor`. Claude Code aún guarda un advisor que su modelo principal actual no admite. Ese advisor se activa después de que cambie a un [modelo principal compatible](#choose-an-advisor-model) con [`/model`](/docs/es/model-config#setting-your-model). Si la API ya rechazó el advisor guardado en la conversación actual, permanece desactivado hasta `/clear` o `/compact`, incluso después de cambiar modelos.

En algunos planes, Fable como advisor también necesita su [consentimiento único para facturar el uso de Fable a créditos de uso](/docs/es/model-config#fable-and-usage-credits). Para saber qué hace `/advisor fable` antes de que haya dado ese consentimiento, vea [Advisor de Fable y créditos de uso](#fable-advisor-and-usage-credits).

<h3 id="set-advisormodel-in-settings">
  Establecer `advisorModel` en la configuración
</h3>

Para configurar el advisor como predeterminado sin abrir una sesión, establézcalo en su archivo de configuración:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Usar la bandera `--advisor`
</h3>

Para establecer el advisor para una única sesión sin cambiar su configuración guardada, inicie con la bandera:

```bash theme={null}
claude --advisor opus
```

Claude Code usa la bandera en lugar de la configuración `advisorModel` para esa sesión. No enumera `--advisor` en `claude --help`. Claude Code sale con un error al iniciar si:

* El modelo principal de la sesión no admite el advisor
* El modelo solicitado, como Haiku, no puede actuar como advisor
* La lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización excluye el modelo solicitado
* Solicitó Fable y su cuenta aún requiere el [consentimiento de créditos de uso](#fable-advisor-and-usage-credits)

Si inicia una [sesión en segundo plano](/docs/es/agent-view) con `--advisor` y se aplica una de estas opciones, Claude Code inicia la sesión sin el advisor en lugar de salir.

<h2 id="choose-an-advisor-model">
  Elegir un modelo advisor
</h2>

El advisor debe ser al menos tan capaz como el modelo principal. Los advisors aceptados para cada modelo principal son:

| Modelo principal    | Advisors aceptados                    | Notas                                                                                    |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------------------------------- |
| Haiku 4.5           | Fable, Opus, Sonnet                   | Haiku puede llamar al advisor pero no puede actuar como uno                              |
| Sonnet 4.6          | Fable, Opus, Sonnet                   |                                                                                          |
| Sonnet 5            | Fable, Opus 4.7 o posterior, Sonnet 5 | Un advisor Sonnet 4.6 se rechaza, y la API rechaza un advisor Opus 4.6                   |
| Opus 4.6            | Fable, Opus, Sonnet 5                 | Un advisor Sonnet 4.6 se rechaza                                                         |
| Opus 4.7 u Opus 4.8 | Fable, y Opus 4.7 o posterior         | Un advisor Opus 4.6 o Sonnet se rechaza                                                  |
| Opus 5.5 u Opus 5   | Fable, y Opus 5 o posterior           | Un advisor Opus 4.6 o Sonnet se rechaza, y la API rechaza un advisor Opus 4.7 u Opus 4.8 |
| Fable 5             | Fable 5.1 o Fable 5                   | Un advisor Opus o Sonnet se rechaza                                                      |
| Fable 5.1           | Fable 5.1                             | Un advisor Opus o Sonnet se rechaza, y la API rechaza un advisor Fable 5                 |

Fable 5.1 requiere Claude Code v2.1.257 o posterior. Ambos modelos Fable requieren [acceso a Fable](/docs/es/model-config#work-with-fable).

Establezca el advisor como `fable`, `opus`, o `sonnet`. Estos alias se resuelven a la versión predeterminada integrada de Claude Code para cada familia de modelos, que avanza con nuevas versiones de Claude Code. También puede pasar un ID de modelo completo como `claude-opus-5-5`.

Los subagentes heredan el advisor configurado y aplican la misma verificación de emparejamiento contra su propio modelo.

Claude Code valida el emparejamiento antes de enviar una solicitud, y la API lo valida de nuevo:

* Para un advisor que la tabla lista como rechazado, Claude Code no lo adjunta a las solicitudes del modelo principal. La salida del comando `/advisor` y una notificación muestran esto. Los subagentes cuyo propio modelo satisface el emparejamiento aún pueden usar el advisor.
* Para un advisor que la tabla lista como rechazado por la API, Claude Code lo adjunta y la API lo rechaza. Claude Code luego reenvía esa solicitud sin el advisor, y el resto de la conversación se ejecuta sin uno, por lo que no ve ningún error y no obtiene llamadas de advisor. Elija un advisor aceptado con `/advisor`; el cambio entra en vigor después de `/clear` o `/compact` y en nuevas sesiones.
* Si el modelo principal o el advisor es un modelo que Claude Code no reconoce, el advisor no se adjunta.

<h3 id="fable-advisor-and-usage-credits">
  Advisor Fable y créditos de uso
</h3>

En algunos planes, el uso de Fable se factura a créditos de uso, y Fable como advisor se factura de la misma manera. Si su cuenta requiere el [consentimiento único para facturar el uso de Fable a créditos de uso](/docs/es/model-config#fable-and-usage-credits), Claude Code lo solicita cuando selecciona un modelo Fable con `/model` y no aplica Fable como advisor hasta que haya aceptado ese consentimiento.

Antes de haberlo aceptado, Claude Code no guarda Fable como advisor cuando escribe `/advisor fable` o elige Fable en el selector `/advisor`. En su lugar, le señala `/model fable`. Con `claude --advisor fable`, Claude Code sale al iniciar con un mensaje que señala `/model fable`. En una [sesión en segundo plano](#use-the-advisor-flag), inicia la sesión sin el advisor en lugar de salir. Con Fable ya guardado como su `advisorModel`, Claude Code envía solicitudes sin el advisor. En una sesión interactiva cuyo modelo principal admite el advisor, también muestra una notificación que señala `/model fable`.

Para aceptar el consentimiento, ejecute `/model fable` y elija continuar en Fable. Claude Code registra el consentimiento y [guarda Fable como su modelo seleccionado](/docs/es/model-config#default-model-setting). Luego seleccione Fable como advisor.

<h3 id="common-model-pairings">
  Emparejamientos de modelos comunes
</h3>

Cualquier emparejamiento aceptado funciona. Estas combinaciones equilibran el costo contra la capacidad de diferentes formas:

| Emparejamiento                    | Cuándo usar                                                                                                                                                  |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sonnet principal + advisor Opus   | Sonnet maneja el trabajo rutinario y escala la planificación, fallos ambiguos y verificaciones de finalización a Opus                                        |
| Sonnet principal + advisor Fable  | Orientación de Fable en puntos de decisión sin ejecutar Fable en todo. Requiere acceso a Fable                                                               |
| Haiku principal + advisor Opus    | Modelo principal de menor costo con planificación fuerte. Espere un costo más alto que Haiku solo pero menor que cambiar el modelo principal a Sonnet u Opus |
| Opus principal + advisor Opus     | Un segundo Opus revisa el primero. Útil para tareas de alto riesgo donde una verificación independiente importa más que el costo                             |
| Fable principal + advisor Fable   | Emparejamiento de mayor capacidad cuando Fable está disponible. Claude Code no aplica un advisor Opus o Sonnet a un modelo principal Fable                   |
| Sonnet principal + advisor Sonnet | Una segunda opinión de menor costo para detectar descuidos rutinarios                                                                                        |

<h2 id="when-claude-consults-the-advisor">
  Cuándo Claude consulta el advisor
</h2>

Claude decide cuándo llamar al advisor. Tiende a consultar antes de comprometerse con un enfoque, cuando un error sigue repitiéndose, y antes de declarar una tarea completada, pero el tiempo es impulsado por el modelo en lugar de basarse en reglas.

Puede solicitar una consulta en su indicación de la misma manera que solicitaría cualquier herramienta, por ejemplo `consulta al advisor antes de continuar`. No hay configuración para limitar o forzar llamadas al advisor; si desea que Claude consulte más o menos a menudo durante una tarea, dígalo en sus instrucciones.

<h2 id="what-you-see-during-a-session">
  Qué ve durante una sesión
</h2>

Cuando Claude llama al advisor, la transcripción muestra una línea `Advising` con el nombre del modelo advisor mientras la llamada está en progreso. Cuando el resultado regresa, la línea informa si el advisor proporcionó orientación:

* **Reviewed**: la línea confirma que el advisor ha revisado la conversación. Cuando el advisor devolvió orientación legible, presione `Ctrl+O` para leerla.
* **Declined**: la línea lee `Advisor declined to advise on this request`. Si el advisor proporcionó una razón, presione `Ctrl+O` para leerla.

Claude generalmente sigue la orientación del advisor, pero se adapta cuando su propia evidencia contradice una afirmación específica: si un paso recomendado falla cuando se intenta, o el contenido del archivo contradice el consejo, Claude expone el conflicto en lugar de seguir la orientación incondicionalmente.

El advisor siempre recibe la conversación completa, y Claude controla el tiempo. Para más control o una configuración diferente, vea [cómo se compara el advisor con subagents y opusplan](#compare-with-related-features).

<h2 id="cost">
  Costo
</h2>

Cuando Claude llama al advisor, el modelo advisor lee la conversación, por lo que cada llamada consume tokens a las tasas del modelo advisor además del uso de su modelo principal. La forma en que esos tokens del advisor se facturan depende de cómo pague:

* **Facturación por API**: paga las tasas de entrada y salida del modelo advisor para los tokens del advisor
* **Planes de suscripción**: el uso del advisor cuenta hacia los límites de uso de su plan, excepto que un advisor Fable se factura a [créditos de uso](/docs/es/model-config#fable-and-usage-credits) en planes donde el uso de Fable lo hace

Si su cuenta requiere el consentimiento de créditos de uso, un advisor Fable no se factura nada antes de que lo otorgue, porque Claude Code [no aplica la selección](#fable-advisor-and-usage-credits) hasta entonces.

Claude llama al advisor en puntos de decisión en lugar de en cada turno, por lo que emparejar un modelo principal más rápido con un advisor más fuerte típicamente cuesta menos que ejecutar el modelo más fuerte en todo. El uso del advisor cuenta hacia los totales de sesión mostrados por [`/usage`](/docs/es/costs#track-your-costs).

Para cómo se reportan los tokens del advisor en respuestas de API, vea [Uso y facturación](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) en la documentación de la API de Claude.

<h2 id="impact-on-prompt-caching">
  Impacto en el almacenamiento en caché de indicaciones
</h2>

Habilitar o deshabilitar el advisor a mitad de sesión no invalida el [caché de indicaciones](/docs/es/prompt-caching) de su modelo principal. A diferencia de [cambiar modelos](/docs/es/prompt-caching#switching-models), alternar `/advisor` mantiene el prefijo en caché intacto, y la orientación devuelta por el advisor se almacena en caché como parte de la transcripción en turnos posteriores.

La propia lectura del advisor de la conversación no se almacena en caché. Cada llamada al advisor procesa la transcripción completa de nuevo, sin reutilización entre llamadas.

<h2 id="requirements">
  Requisitos
</h2>

La herramienta advisor requiere todo lo siguiente:

* **Solo API de Anthropic**: el advisor es una herramienta ejecutada por servidor. No está disponible en Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform o Microsoft Foundry. A través de una [puerta de enlace LLM](/docs/es/llm-gateway) configurada con `ANTHROPIC_BASE_URL`, la disponibilidad depende de si la puerta de enlace reenvía la solicitud intacta a la API de Anthropic. Si la puerta de enlace o su upstream no reconoce la herramienta advisor, consulte [Reintento automático y reenvío de errores](/docs/es/llm-gateway-protocol#automatic-retry-and-error-forwarding) para ver cómo responde Claude Code.
* **Modelo principal admitido**: Fable, Opus 4.6 o posterior, Sonnet 4.6 o posterior, o Haiku 4.5. Consulte [Elegir un modelo advisor](#choose-an-advisor-model) para saber qué advisors acepta cada uno.
* **Obtención de indicadores de características**: Claude Code activa el advisor a través de un indicador de características que obtiene de Anthropic. En una sesión donde se establece una variable que desactiva la obtención de indicadores, como `DISABLE_TELEMETRY`, el advisor permanece desactivado. Consulte [Características que necesitan obtención de indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Desactivar el advisor
</h2>

Para dejar de usar el advisor, ejecute `/advisor off` o elija **No advisor** en el selector `/advisor`:

```
/advisor off
```

Para deshabilitar la herramienta advisor completamente, establezca `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. El comando `/advisor` se vuelve no disponible y cualquier `advisorModel` configurado se ignora. La bandera `--advisor` se acepta pero no tiene efecto. Vea [Variables de entorno](/docs/es/env-vars).

<h2 id="compare-with-related-features">
  Comparar con características relacionadas
</h2>

El advisor es una de varias formas de combinar fortalezas de modelos. Elija según cuándo desee que un segundo modelo esté involucrado.

| Enfoque                                                            | Cuándo se ejecuta el modelo más fuerte                                                                                                         | Cómo comienza                               |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| Herramienta advisor                                                | En puntos de decisión a mitad de tarea                                                                                                         | Claude la llama cuando necesita orientación |
| [`opusplan`](/docs/es/model-config#opusplan-model-setting)              | Durante el modo plan cuando [permitido por `availableModels`](/docs/es/model-config#restrict-model-selection), luego cambia a Sonnet para ejecución | Usted entra en modo plan                    |
| [Subagents](/docs/es/sub-agents#choose-a-model) con `model` establecido | Para toda la subtarea delegada                                                                                                                 | Claude delega, o usted invoca el subagent   |
| [`/model`](/docs/es/model-config#setting-your-model)                    | Desde la siguiente solicitud en adelante                                                                                                       | Usted cambia modelos                        |

<h2 id="see-also">
  Ver también
</h2>

* [Configuración de modelos](/docs/es/model-config): cambie modelos, establezca niveles de esfuerzo, y use `opusplan`
* [Gestionar costos de forma efectiva](/docs/es/costs): rastrear el uso de tokens entre modelos
* [Herramienta advisor en la API de Claude](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): comprenda la herramienta de servidor subyacente, o úsela directamente desde la API de Mensajes
* [La estrategia advisor](https://claude.com/blog/the-advisor-strategy): por qué emparejar un modelo principal rápido con un advisor más fuerte funciona
