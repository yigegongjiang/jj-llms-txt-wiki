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

Si alguno de estos establece un modelo advisor, el advisor se habilita para sesiones cuyo modelo principal [lo admite](#choose-an-advisor-model). Para dejar de usarlo, vea [Desactivar el advisor](#turn-the-advisor-off).

<Note>
  Para usar Fable 5 como advisor, necesita Claude Code v2.1.170 o posterior y [acceso a Fable 5](/docs/es/model-config#work-with-fable-5) para su organización.
</Note>

<h3 id="use-the-/advisor-command">
  Usar el comando `/advisor`
</h3>

Ejecute `/advisor` sin argumentos para abrir un selector que enumere los modelos advisor disponibles, o pase el modelo directamente:

```
/advisor opus
```

Su selección se guarda en `advisorModel` en su configuración de usuario y persiste entre sesiones. Si la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización excluye el modelo advisor guardado, el advisor no se invoca hasta que seleccione un modelo permitido con `/advisor`. Si su modelo principal actual no admite el advisor, la selección aún se guarda y se activa cuando cambia a un [modelo principal compatible](#choose-an-advisor-model) con [`/model`](/docs/es/model-config#setting-your-model).

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

La bandera tiene prioridad sobre la configuración `advisorModel` para esa sesión. Sale con un error si el modelo principal de la sesión no admite el advisor, o si el modelo advisor solicitado está excluido por la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización.

<h2 id="choose-an-advisor-model">
  Elegir un modelo advisor
</h2>

El advisor debe ser al menos tan capaz como el modelo principal. Los advisors aceptados para cada modelo principal son:

| Modelo principal     | Advisors aceptados        | Notas                                                                                                                                                                             |
| -------------------- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Haiku 4.5            | Fable, Opus, Sonnet       | Haiku puede llamar al advisor pero no puede actuar como uno                                                                                                                       |
| Sonnet 4.6           | Fable, Opus, Sonnet       |                                                                                                                                                                                   |
| Sonnet 5             | Fable, Opus, Sonnet 5     | Un advisor Sonnet 4.6 se rechaza                                                                                                                                                  |
| Opus 4.6             | Fable, Opus, Sonnet 5     | Sonnet 5 y Opus 4.6 se clasifican como igualmente capaces, por lo que un Opus 4.6 principal acepta un advisor Sonnet 5                                                            |
| Opus 4.7 o posterior | Fable, Opus 4.7, Opus 4.8 | Opus 4.7 y Opus 4.8 se clasifican como igualmente capaces, por lo que cualquiera acepta al otro como advisor. Un Opus 4.7 principal con un advisor Opus 4.6 o Sonnet 5 se rechaza |
| Fable 5 (v2.1.170+)  | Fable                     | Un advisor Opus o Sonnet se rechaza                                                                                                                                               |

Fable 5 requiere Claude Code v2.1.170 o posterior y acceso a Fable 5, ya sea que actúe como modelo principal o como advisor.

Establezca el advisor como `opus`, `sonnet`, o `fable`. Estos alias se resuelven a la versión más reciente de cada modelo. También puede pasar un ID de modelo completo como `claude-opus-4-8`.

Los subagentes heredan el advisor configurado y aplican la misma verificación de emparejamiento contra su propio modelo.

Claude Code valida el emparejamiento antes de enviar una solicitud:

* Si el advisor es menos capaz que el modelo principal, el advisor no se adjunta a las solicitudes del modelo principal. La salida del comando `/advisor` y una notificación muestran esto. Los subagentes cuyo propio modelo satisface el emparejamiento aún pueden usar el advisor.
* Si el modelo principal o el advisor es un modelo que Claude Code no reconoce, el advisor no se adjunta.

<h3 id="common-model-pairings">
  Emparejamientos de modelos comunes
</h3>

Cualquier emparejamiento aceptado funciona. Estas combinaciones equilibran el costo contra la capacidad de diferentes formas:

| Emparejamiento                    | Cuándo usar                                                                                                                                                                                      |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sonnet principal + advisor Opus   | Sonnet maneja el trabajo rutinario y escala la planificación, fallos ambiguos y verificaciones de finalización a Opus                                                                            |
| Sonnet principal + advisor Fable  | Orientación de Fable 5 en puntos de decisión sin ejecutar Fable 5 en todo. Requiere v2.1.170 o posterior y acceso a Fable 5                                                                      |
| Haiku principal + advisor Opus    | Modelo principal de menor costo con planificación fuerte. Espere un costo más alto que Haiku solo pero menor que cambiar el modelo principal a Sonnet u Opus                                     |
| Opus principal + advisor Opus     | Un segundo Opus revisa el primero. Útil para tareas de alto riesgo donde una verificación independiente importa más que el costo                                                                 |
| Fable principal + advisor Fable   | Emparejamiento de mayor capacidad cuando Fable 5 está disponible (v2.1.170+). Fable es un nivel superior a Opus y Sonnet, por lo que es el único advisor aceptado para un modelo principal Fable |
| Sonnet principal + advisor Sonnet | Una segunda opinión de menor costo para detectar descuidos rutinarios                                                                                                                            |

<h2 id="when-claude-consults-the-advisor">
  Cuándo Claude consulta el advisor
</h2>

Claude decide cuándo llamar al advisor. Tiende a consultar antes de comprometerse con un enfoque, cuando un error sigue repitiéndose, y antes de declarar una tarea completada, pero el tiempo es impulsado por el modelo en lugar de basarse en reglas.

Puede solicitar una consulta en su indicación de la misma manera que solicitaría cualquier herramienta, por ejemplo `consulta al advisor antes de continuar`. No hay configuración para limitar o forzar llamadas al advisor; si desea que Claude consulte más o menos a menudo durante una tarea, dígalo en sus instrucciones.

<h2 id="what-you-see-during-a-session">
  Qué ve durante una sesión
</h2>

Cuando Claude llama al advisor, la transcripción muestra una línea `Advising` con el nombre del modelo advisor mientras la llamada está en progreso. Cuando el resultado regresa, la línea confirma que el advisor ha revisado la conversación. Presione `Ctrl+O` para expandirla y leer la orientación completa del advisor.

Claude generalmente sigue la orientación del advisor, pero se adapta cuando su propia evidencia contradice una afirmación específica: si un paso recomendado falla cuando se intenta, o el contenido del archivo contradice el consejo, Claude expone el conflicto en lugar de seguir la orientación incondicionalmente.

El advisor siempre recibe la conversación completa, y Claude controla el tiempo. Para más control o una configuración diferente, vea [cómo se compara el advisor con subagents y opusplan](#compare-with-related-features).

<h2 id="cost">
  Costo
</h2>

Cada llamada al advisor envía la conversación al modelo advisor, por lo que consume tokens a las tasas del modelo advisor además del uso de su modelo principal. Con facturación por API, los tokens del advisor se cobran a las tasas de entrada y salida del modelo advisor. En planes de suscripción, el uso del advisor cuenta hacia los límites de uso de su plan.

Claude llama al advisor en puntos de decisión en lugar de en cada turno, por lo que emparejar un modelo principal más rápido con un advisor más fuerte típicamente cuesta menos que ejecutar el modelo más fuerte en todo. El uso del advisor cuenta hacia los totales de sesión mostrados por [`/usage`](/docs/es/costs#track-your-costs).

Para cómo se reportan los tokens del advisor en respuestas de API, vea [Uso y facturación](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) en la documentación de la API de Claude.

<h2 id="impact-on-prompt-caching">
  Impacto en el almacenamiento en caché de indicaciones
</h2>

Habilitar o deshabilitar el advisor a mitad de sesión no invalida el [caché de indicaciones](/docs/es/prompt-caching) de su modelo principal. A diferencia de [cambiar modelo o nivel de esfuerzo](/docs/es/prompt-caching#actions-that-invalidate-the-cache), alternar `/advisor` mantiene el prefijo en caché intacto, y la orientación devuelta por el advisor se almacena en caché como parte de la transcripción en turnos posteriores.

La propia lectura del advisor de la conversación no se almacena en caché. Cada llamada al advisor procesa la transcripción completa de nuevo, sin reutilización entre llamadas.

<h2 id="requirements">
  Requisitos
</h2>

La herramienta advisor requiere todo lo siguiente:

* **Solo API de Anthropic**: el advisor es una herramienta ejecutada por servidor. No está disponible en Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform o Microsoft Foundry. A través de una [puerta de enlace LLM](/docs/es/llm-gateway) configurada con `ANTHROPIC_BASE_URL`, la disponibilidad depende de si la puerta de enlace reenvía la solicitud intacta a la API de Anthropic.
* **Modelo principal admitido**: Opus 4.6 o posterior, Sonnet 4.6 o posterior, o Haiku 4.5. Fable 5 también califica en Claude Code v2.1.170 o posterior.

<h2 id="turn-the-advisor-off">
  Desactivar el advisor
</h2>

Para dejar de usar el advisor y borrar su `advisorModel` guardado, ejecute `/advisor off` o elija **No advisor** en el selector `/advisor`:

```
/advisor off
```

Para deshabilitar la herramienta advisor completamente, establezca `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. El comando `/advisor` se vuelve no disponible y cualquier `advisorModel` configurado se ignora. La bandera `--advisor` se acepta pero no tiene efecto; los scripts existentes que la pasan continúan funcionando sin errores. Vea [Variables de entorno](/docs/es/env-vars).

<h2 id="compare-with-related-features">
  Comparar con características relacionadas
</h2>

El advisor es una de varias formas de combinar fortalezas de modelos. Elija según cuándo desee que un segundo modelo esté involucrado.

| Enfoque                                                            | Cuándo se ejecuta el modelo más fuerte                                                                                                         | Cómo comienza                               |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| Herramienta advisor                                                | En puntos de decisión a mitad de tarea                                                                                                         | Claude la llama cuando necesita orientación |
| [`opusplan`](/docs/es/model-config#opusplan-model-setting)              | Durante el modo plan cuando [permitido por `availableModels`](/docs/es/model-config#restrict-model-selection), luego cambia a Sonnet para ejecución | Usted entra en modo plan                    |
| [Subagents](/docs/es/sub-agents#choose-a-model) con `model` establecido | Para toda la subtarea delegada                                                                                                                 | Claude delega, o usted invoca el subagent   |
| [`/model`](/docs/es/model-config#setting-your-model)                    | Para todos los turnos posteriores                                                                                                              | Usted cambia modelos                        |

<h2 id="see-also">
  Ver también
</h2>

* [Configuración de modelos](/docs/es/model-config): cambie modelos, establezca niveles de esfuerzo, y use `opusplan`
* [Gestionar costos de forma efectiva](/docs/es/costs): rastrear el uso de tokens entre modelos
* [Herramienta advisor en la API de Claude](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): comprenda la herramienta de servidor subyacente, o úsela directamente desde la API de Mensajes
* [La estrategia advisor](https://claude.com/blog/the-advisor-strategy): por qué emparejar un modelo principal rápido con un advisor más fuerte funciona
