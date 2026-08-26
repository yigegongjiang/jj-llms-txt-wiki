> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Encuentra errores con ultrareview

> Ejecuta una revisión de código profunda y multiagente en la nube con /code-review ultra para encontrar y verificar errores antes de fusionar.

<Note>
  Ultrareview es una característica de vista previa de investigación. La característica, los precios y la disponibilidad pueden cambiar según los comentarios. El comando ahora se invoca como `/code-review ultra`, y `/ultrareview` permanece como un alias.
</Note>

Ultrareview es una revisión de código profunda que se ejecuta en Claude Code en la infraestructura web. Cuando ejecuta `/code-review ultra`, Claude Code lanza una flota de agentes revisores en un sandbox remoto para encontrar errores en su rama o solicitud de extracción.

En comparación con una `/code-review` local o `/review`, ultrareview ofrece:

* **Mayor señal**: cada hallazgo reportado se reproduce y verifica de forma independiente, por lo que los resultados se centran en errores reales en lugar de sugerencias de estilo
* **Cobertura más amplia**: una flota más grande de agentes revisores explora el cambio en paralelo, lo que expone problemas que una revisión local podría perder
* **Sin uso de recursos locales**: la revisión se ejecuta completamente en un sandbox remoto, por lo que su terminal permanece libre para otro trabajo mientras se ejecuta

Ultrareview requiere autenticación con una cuenta de Claude.ai porque se ejecuta en Claude Code en la infraestructura web. Si ha iniciado sesión solo con una clave API, ejecute `/login` y autentica con Claude.ai primero. Ultrareview no está disponible cuando se usa Claude Code con Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, y no está disponible para organizaciones que han habilitado Zero Data Retention.

<h2 id="run-ultrareview-from-the-cli">
  Ejecuta ultrareview desde la CLI
</h2>

Inicia una revisión desde cualquier repositorio git en la CLI de Claude Code.

```text theme={null}
/code-review ultra
```

Sin argumentos, ultrareview revisa la diferencia entre su rama actual y la rama predeterminada, incluidos los cambios sin confirmar y preparados en su árbol de trabajo. Claude Code agrupa el estado del repositorio y lo carga en un sandbox remoto para la revisión.

Para revisar una solicitud de extracción de GitHub en su lugar, pase el número de PR.

```text theme={null}
/code-review ultra 1234
```

En modo PR, el sandbox remoto clona la solicitud de extracción directamente desde el host en lugar de agrupar su árbol de trabajo local. El modo PR funciona con repositorios en `github.com` y en instancias de [GitHub Enterprise Server](/docs/es/github-enterprise-server) que un administrador ha conectado a Claude Code.

<Tip>
  Si su repositorio es demasiado grande para agrupar, Claude Code le solicita que use el modo PR en su lugar. Envíe su rama y abra un PR borrador, luego ejecute `/code-review ultra <PR-number>`.

  Si la diferencia de la solicitud de extracción es demasiado grande, Claude Code rechaza la revisión con una sugerencia de alcance antes de que se ejecute cualquier trabajo de revisión.
</Tip>

Antes de lanzar, Claude Code muestra un diálogo de confirmación con el alcance de la revisión (incluido el recuento de archivos y líneas cuando se revisa una rama), sus ejecuciones gratuitas restantes y el costo estimado. Después de confirmar, la revisión continúa en segundo plano y puede seguir usando su sesión. El comando se ejecuta solo cuando lo invoca con `/code-review ultra`; Claude no inicia un ultrareview por su cuenta.

<h2 id="pricing-and-free-runs">
  Precios y ejecuciones gratuitas
</h2>

Ultrareview es una característica premium que se factura contra créditos de uso en lugar del uso incluido en su plan.

| Plan              | Ejecuciones gratuitas incluidas | Después de ejecuciones gratuitas                                                                                    |
| ----------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Pro               | 3 ejecuciones gratuitas         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 ejecuciones gratuitas         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team y Enterprise | ninguno                         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |

Los suscriptores de Pro y Max reciben tres ejecuciones gratuitas de ultrareview para probar la característica. Estas tres ejecuciones son una asignación única por cuenta y no se renuevan. Después de usar las tres, o después de que finalice el período de ejecuciones gratuitas, cada revisión se factura a los créditos de uso y típicamente cuesta entre $5 y $20 dependiendo del tamaño del cambio. Una ejecución se cuenta una vez que la sesión remota comienza, por lo que una revisión que detenga temprano o que no se complete correctamente sigue utilizando una ejecución gratuita. Para una revisión pagada, los créditos de uso se facturan solo por la porción que se ejecutó.

Debido a que ultrareview siempre se factura como créditos de uso fuera de las ejecuciones gratuitas, su cuenta u organización debe tener los créditos de uso habilitados antes de poder lanzar una revisión pagada. Si los créditos de uso no están habilitados, Claude Code bloquea el lanzamiento y le vincula a la configuración de facturación donde puede activarlos. También puede ejecutar `/usage-credits` para verificar o cambiar su configuración actual.

<h2 id="track-a-running-review">
  Rastrear una revisión en ejecución
</h2>

Una revisión típicamente toma de 5 a 10 minutos. La revisión se ejecuta como una tarea de fondo, por lo que puede seguir trabajando en su sesión, iniciar otros comandos o cerrar la terminal completamente.

Utilice `/tasks` para ver revisiones en ejecución y completadas, abra la vista de detalle para una revisión o detenga una revisión que está en progreso. Detener una revisión archiva la sesión en la nube, y los hallazgos parciales no se devuelven. Cuando la revisión finaliza, los hallazgos verificados aparecen como una notificación en su sesión. Cada hallazgo incluye la ubicación del archivo y una explicación del problema para que pueda pedirle a Claude que lo corrija directamente.

<h2 id="run-ultrareview-non-interactively">
  Ejecuta ultrareview de forma no interactiva
</h2>

Usa el subcomando `claude ultrareview` para iniciar un ultrareview desde CI o un script sin una sesión interactiva. El subcomando lanza la misma revisión que `/code-review ultra`, se bloquea hasta que finalice la revisión remota, imprime los hallazgos en stdout y sale con código 0 en caso de éxito o 1 en caso de fallo.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sin argumentos, el subcomando revisa la diferencia entre su rama actual y la rama predeterminada. Pase un número de PR para revisar una solicitud de extracción, o pase una rama base para revisar la diferencia contra esa rama en su lugar. Invocar el subcomando cuenta como consentimiento para el aviso de facturación y términos que muestra el comando interactivo.

Los mensajes de progreso y la URL de sesión en vivo van a stderr para que stdout permanezca analizable. Use estas banderas para controlar la salida y el tiempo de espera:

| Bandera               | Descripción                                                                          |
| --------------------- | ------------------------------------------------------------------------------------ |
| `--json`              | Imprime la carga útil `bugs.json` sin procesar en lugar de los hallazgos formateados |
| `--timeout <minutes>` | Minutos máximos para esperar a que finalice la revisión. Por defecto es 30           |

Ejecutar `claude ultrareview` requiere la misma autenticación y configuración de uso adicional que `/code-review ultra`. El subcomando sale con código 0 cuando la revisión se completa con o sin hallazgos, código 1 cuando la revisión falla al lanzarse, la sesión remota genera un error o el tiempo de espera se agota, y código 130 cuando se interrumpe con Ctrl-C. La revisión remota continúa ejecutándose si interrumpe el subcomando; siga la URL de sesión impresa en stderr para verla en el navegador.

Para revisiones automáticas en solicitudes de extracción de GitHub, [Code Review](/docs/es/code-review) se integra directamente con su repositorio e publica hallazgos como comentarios de PR en línea sin un paso de CLI.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Cómo ultrareview se compara con /code-review y /review
</h2>

Los tres comandos revisan código, pero se dirigen a diferentes etapas de su flujo de trabajo.

|             | `/code-review`                          | `/review <pr>`                                                        | `/code-review ultra`                                                                      |
| ----------- | --------------------------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Objetivo    | su diff de trabajo                      | una solicitud de extracción de GitHub                                 | su diff de trabajo o una solicitud de extracción                                          |
| Se ejecuta  | localmente en su sesión                 | localmente en su sesión                                               | remotamente en un sandbox en la nube                                                      |
| Profundidad | se escala con el argumento de esfuerzo  | una revisión de una sola pasada en el esfuerzo de la sesión           | flota multiagente con verificación independiente                                          |
| Duración    | segundos a pocos minutos                | segundos a pocos minutos                                              | aproximadamente 5 a 10 minutos                                                            |
| Costo       | cuenta hacia el uso normal              | cuenta hacia el uso normal                                            | ejecuciones gratuitas, luego aproximadamente \$5 a \$20 por revisión como créditos de uso |
| Mejor para  | retroalimentación rápida mientras itera | revisar la solicitud de extracción de un compañero antes de aprobarla | confianza previa a la fusión en cambios sustanciales                                      |

Utilice `/code-review` para retroalimentación rápida mientras trabaja. Utilice `/review <pr>` para revisar una solicitud de extracción de la misma manera que lo haría antes de aprobarla. Utilice `/code-review ultra` antes de fusionar un cambio sustancial cuando desee una pasada más profunda que detecte problemas que una revisión local podría perder.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code en la web](/docs/es/claude-code-on-the-web): aprende cómo funcionan las sesiones remotas y los sandboxes en la nube
* [Planifica cambios complejos con ultraplan](/docs/es/ultraplan): la contraparte de planificación de ultrareview para trabajo de diseño inicial
* [Gestiona costos de manera efectiva](/docs/es/costs): rastrear el uso y establecer límites de gasto
