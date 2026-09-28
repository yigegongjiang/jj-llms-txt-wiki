> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Encuentra errores con ultrareview

> Ejecuta una revisión de código profunda y multiagente en la nube con /code-review ultra para encontrar y verificar errores antes de fusionar.

<Note>
  Ultrareview es una característica de vista previa de investigación. La característica, los precios y la disponibilidad pueden cambiar según los comentarios. El comando es `/code-review ultra`. Cuando ultrareview está disponible para su cuenta, `/ultrareview` es un alias.
</Note>

Ultrareview es una revisión de código profunda que se ejecuta como una [sesión en la nube](/docs/es/claude-code-on-the-web) en la infraestructura de Anthropic. Cuando ejecuta `/code-review ultra`, Claude Code lanza una flota de agentes revisores en un sandbox en la nube para encontrar errores en su rama o solicitud de extracción.

En comparación con una `/code-review` local, ultrareview ofrece:

* **Mayor señal**: cada hallazgo reportado se reproduce y verifica de forma independiente, por lo que los resultados se centran en errores reales en lugar de sugerencias de estilo
* **Cobertura más amplia**: una flota más grande de agentes revisores explora el cambio en paralelo, lo que expone problemas que una revisión local podría perder
* **Sin uso de recursos locales**: la revisión se ejecuta completamente en un sandbox en la nube, por lo que su terminal permanece libre para otro trabajo mientras se ejecuta

Ultrareview requiere autenticación con una cuenta de claude.ai porque se ejecuta como una sesión en la nube en la infraestructura de Anthropic. Si ha iniciado sesión solo con una clave API, ejecute `/login` y autentica con claude.ai primero. Ultrareview no está disponible cuando se usa Claude Code con Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, y no está disponible para organizaciones que han habilitado Zero Data Retention. Cuando ultrareview no está disponible, `/code-review ultra` ejecuta una revisión local en su sesión en su lugar.

<h2 id="run-ultrareview-from-the-cli">
  Ejecuta ultrareview desde la CLI
</h2>

Inicia una revisión desde cualquier repositorio git:

```text theme={null}
/code-review ultra
```

Sin argumentos, ultrareview revisa la diferencia entre su rama actual y la rama predeterminada, incluidos los cambios sin confirmar y preparados. Para cambios sin confirmar en archivos nombrados como credenciales o claves, como archivos `.env` y `*.tfvars`, Claude Code sigue las reglas para [cargar un repositorio local en una sesión en la nube](/docs/es/claude-code-on-the-web#send-local-repositories-without-github).

Para una revisión de rama, Claude Code agrupa el estado del repositorio y lo carga en un sandbox remoto; cuando [revisa una solicitud de extracción](#review-a-pull-request), Claude Code no carga nada de su máquina.

Antes de lanzar, Claude Code muestra un diálogo de confirmación con el alcance de la revisión, sus ejecuciones gratuitas restantes y el costo estimado; para una revisión de rama, el alcance incluye el recuento de archivos y líneas. Después de confirmar, la revisión continúa en segundo plano mientras sigue usando su sesión.

El comando se ejecuta solo cuando lo invoca con `/code-review ultra`; Claude no inicia un ultrareview por su cuenta.

<h3 id="review-against-a-different-base">
  Revisa contra una base diferente
</h3>

Para comparar contra una base que no sea la rama predeterminada, pase el nombre de la rama. Este ejemplo revisa su rama actual contra `develop` en su lugar:

```text theme={null}
/code-review ultra develop
```

La rama base no necesita existir en su clon local; Claude Code la obtiene de `origin`. Si el nombre tiene un error tipográfico, Claude Code sugiere el nombre de rama más cercano en el error.

Un ID de confirmación o etiqueta también funciona como la base, y la revisión cubre los cambios en su rama desde esa confirmación.

<h3 id="review-a-pull-request">
  Revisa una solicitud de extracción
</h3>

Para revisar una solicitud de extracción de GitHub en su lugar de una rama local, pase el número de PR:

```text theme={null}
/code-review ultra 1234
```

El comando también acepta `#1234`, `PR 1234` y URLs de PR pegadas; una URL pegada debe apuntar al repositorio en su directorio actual.

En modo PR, el sandbox remoto clona la solicitud de extracción directamente desde el host en lugar de agrupar su árbol de trabajo local. El modo PR funciona con repositorios en `github.com` y en instancias de [GitHub Enterprise Server](/docs/es/github-enterprise-server) que un administrador ha conectado a Claude Code.

Para repositorios en `github.com`, el sandbox clona con la cuenta de GitHub conectada a su cuenta de Claude, por lo que la cuenta debe poder leer el repositorio del PR. Claude Code verifica esto antes de crear la sesión en la nube, a menos que haya establecido [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars#variables), y rechaza el lanzamiento cuando [ninguna cuenta está conectada](/docs/es/errors#no-github-account-is-connected-to-your-claude-account) o [la cuenta no puede ver el repositorio](/docs/es/errors#your-connected-github-account-cant-see-the-repository); el rechazo nombra la solución. Antes de v2.1.248, Claude Code no verificaba esto antes del lanzamiento.

Ejecute [`/web-setup`](/docs/es/web-quickstart#connect-from-your-terminal) para conectar su inicio de sesión de GitHub CLI a su cuenta de Claude.

<h3 id="post-findings-to-the-pull-request">
  Publica hallazgos en la solicitud de extracción
</h3>

En Claude Code v2.1.227 o posterior, cuando revisa una solicitud de extracción en `github.com`, puede hacer que Claude publique los hallazgos terminados en el PR como un único comentario simple desde su propia cuenta de GitHub. El comentario no es una revisión ni una aprobación, y termina con una nota "Generado por Claude Code". Cuando revisa una rama o una solicitud de extracción de GitHub Enterprise Server, Claude Code muestra los hallazgos solo en su sesión.

Claude Code nunca publica a menos que elija hacerlo en esa ejecución, y `--no-post` es el predeterminado. Publicar es una opción que realiza para cada ejecución:

* **Interactivo**: en el diálogo de lanzamiento, seleccione **Ejecutar y publicar los hallazgos en el PR como yo**. Si agrega `--post` al comando, como en `/code-review ultra 1234 --post`, Claude Code preselecciona esa opción y aún pregunta antes de lanzar.
* **No interactivo**: ejecute el [subcomando `claude ultrareview`](#run-ultrareview-non-interactively) con `--post`. Usted consiente la publicación al ejecutar el subcomando con la bandera, por lo que Claude Code publica sin preguntar. En una ejecución de `claude -p '/code-review ultra'`, Claude Code sale antes de que lleguen los hallazgos, por lo que no publica nada; use el subcomando en su lugar.

Claude Code no publica desde su máquina. Envía el ID de sesión de la revisión a la API de Anthropic, que publica los hallazgos almacenados de la revisión como el comentario a través de la cuenta de GitHub que ha conectado a Claude. Publicar requiere el mismo inicio de sesión en claude.ai que la revisión en sí, y no está disponible en proveedores de terceros o cuando establece [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars).

En una sesión interactiva, Claude Code inicia la publicación cuando llegan los hallazgos, por lo que mantenga la sesión abierta hasta que se complete la revisión. Claude Code mantiene la opción de publicación solo en esa sesión. Si la sesión termina antes de que se complete la revisión, Claude Code no publica nada, incluso si reanuda la conversación más tarde.

Cuando la publicación se completa, Claude le dice el resultado:

* **Publicado**: Claude le proporciona un enlace al comentario.
* **Ya publicado**: una publicación anterior de la misma revisión ya puso el comentario en el PR, por lo que Claude le vincula a la solicitud de extracción en su lugar de publicar de nuevo.
* **Falló**: Claude le dice por qué, y los hallazgos permanecen en su terminal para que pueda publicarlos manualmente.

<h3 id="pass-a-request-in-plain-words">
  Pasa una solicitud en palabras simples
</h3>

En Claude Code v2.1.218 o posterior, también puede describir en qué está trabajando en palabras simples:

```text theme={null}
/code-review ultra check my auth changes
```

La revisión aún cubre su rama actual, el mismo alcance que ejecutar sin argumento. Claude mantiene su texto como una nota, mostrada en el diálogo de lanzamiento, y relaciona los hallazgos con él cuando llegan.

Claude Code trata su texto como una nota solo cuando tiene más de una palabra y no es un nombre de rama o referencia de PR. Lee una sola palabra como un nombre de rama o referencia de PR, por lo que un nombre de rama mal escrito obtiene el error de rama más cercana de [Revisa contra una base diferente](#review-against-a-different-base) en su lugar de lanzar con una nota. Si su texto combina una referencia de PR con otras palabras, como `check PR 123 again`, Claude Code tampoco se lanza; le pide que vuelva a ejecutar con solo el número de PR para revisar ese PR, o sin la referencia para revisar su rama actual.

<Tip>
  Si su repositorio es demasiado grande para agrupar, Claude Code le solicita que use el modo PR en su lugar. Envíe su rama y abra un PR borrador, luego ejecute `/code-review ultra <PR-number>`.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Límites de diferencia y alternativas
</h3>

Ultrareview verifica la diferencia antes de que se ejecute cualquier trabajo de revisión y le dice cuándo no puede revisarla tal como está:

* **Diferencia demasiado grande**: una revisión de rama puede incluir hasta 500 archivos cambiados y 8,000 líneas cambiadas de forma predeterminada. Los valores exactos pueden cambiar, y el [rechazo](/docs/es/errors#diff-is-too-large-for-ultrareview) nombra los que están en vigor, el tamaño de su diferencia y los archivos con la mayoría de líneas cambiadas. Claude Code rechaza una solicitud de extracción demasiado grande de la misma manera, nombrando sus recuentos de archivos y líneas pero no el desglose por archivo
* **Nada que revisar**: cuando la diferencia contra la base está vacía, ultrareview rechaza y nombra la rama o confirmación con la que comparó y el caso en el que se encuentra, como estar en la rama base en sí sin nada sin confirmar, o una rama cuyos compromisos ya son parte de la base. También sugiere la forma de salir para ese caso, como cambiar a la rama con su trabajo, preparar o confirmar ediciones locales, o pasar una base diferente
* **Primera confirmación**: la primera confirmación de un repositorio no tiene nada anterior con lo que comparar, por lo que ultrareview revisa cada archivo en ella después de confirmar en el diálogo de lanzamiento. Si tiene archivos sin rastrear, rechaza en su lugar y le dice que `git add` los que desea revisar. Se aplican los mismos límites de tamaño.

  Una primera confirmación se revisa completamente solo después de esa confirmación, por lo que el subcomando `claude ultrareview` y `claude -p` la rechazan y lo señalan a una sesión interactiva en su lugar. Requiere Claude Code v2.1.277 o posterior
* **Sin base de fusión**: cuando su rama no comparte historial con la rama base, o el repositorio no tiene rama base con la que comparar, ultrareview revisa cada archivo rastreado en el repositorio en su lugar. La alternativa requiere un clon completo y aplica los mismos límites de tamaño. Se lanza solo cuando confirma en el diálogo de lanzamiento o ejecuta el subcomando `claude ultrareview` usted mismo. En `claude -p` y en cualquier otro lugar donde ninguno suceda, ultrareview rechaza, dice que la revisión cubriría cada archivo, y lo señala a una sesión interactiva.

  En un checkout sin ramas u otras referencias, como un HEAD desconectado creado al verificar `FETCH_HEAD` después de obtener una URL, Claude Code [rechaza la revisión](/docs/es/errors#your-checkout-has-no-branches) y sugiere crear una rama primero

<h2 id="pricing-and-free-runs">
  Precios y ejecuciones gratuitas
</h2>

Ultrareview es una característica premium que se factura contra créditos de uso en lugar del uso incluido en su plan.

| Plan              | Ejecuciones gratuitas incluidas | Después de ejecuciones gratuitas                                                                                    |
| ----------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Pro               | 3 ejecuciones gratuitas         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 ejecuciones gratuitas         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team y Enterprise | ninguno                         | facturado como [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Ejecuciones gratuitas**: las tres ejecuciones de Pro y Max son una asignación única por cuenta y no se renuevan.
* **Costo por revisión**: después de usar las ejecuciones gratuitas, típicamente entre $5 y $25 en créditos de uso dependiendo del tamaño del cambio, coincidiendo con la estimación que muestra el diálogo de lanzamiento antes de cada ejecución.
* **Cuándo se cuenta una ejecución**: una vez que la sesión en la nube comienza. Una revisión que detenga temprano o que no se complete correctamente sigue utilizando una ejecución gratuita; una revisión pagada se factura solo por la porción que se ejecutó.

Debido a que ultrareview siempre se factura como créditos de uso fuera de las ejecuciones gratuitas, su cuenta u organización debe tener los créditos de uso habilitados antes de poder lanzar una revisión pagada. Si los créditos de uso no están habilitados, Claude Code bloquea el lanzamiento, y cómo activarlos depende de su acceso de facturación:

* Si puede administrar la facturación de su cuenta, Claude Code le vincula a la configuración de facturación donde puede activar los créditos de uso.
* En planes Team y Enterprise, los miembros sin acceso de facturación envían una solicitud desde la CLI pidiendo a su administrador que active los créditos de uso.

También puede ejecutar `/usage-credits` para verificar o cambiar su configuración de créditos de uso.

Claude Code le pide que confirme la facturación de créditos de uso una vez por conversación: cuando inicia una nueva conversación, por ejemplo con `/clear`, Claude Code muestra la confirmación nuevamente para la próxima revisión pagada.

<h2 id="track-a-running-review">
  Rastrear una revisión en ejecución
</h2>

Una revisión típicamente toma de 5 a 10 minutos. La revisión se ejecuta como una tarea de fondo, por lo que puede seguir trabajando en su sesión, iniciar otros comandos o cerrar la terminal completamente. Si eligió [publicar los hallazgos en la solicitud de extracción](#post-findings-to-the-pull-request), mantenga la sesión abierta hasta que finalice la revisión; si la sesión termina primero, Claude Code no publica nada.

Utilice `/tasks` para ver revisiones en ejecución y completadas, abra la vista de detalle para una revisión o detenga una revisión que está en progreso. Si detiene una revisión, Claude Code archiva la sesión en la nube y no devuelve hallazgos parciales.

Claude también puede indicarle que una revisión fue detenida o que su sesión no fue encontrada:

* Si la sesión en la nube de la revisión se detiene o se [archiva](/docs/es/claude-code-on-the-web#archive-sessions) en claude.ai antes de que finalice la revisión, Claude le indica que fue detenida.
* Si la sesión en la nube de la revisión fue eliminada, o ha iniciado sesión en una cuenta de Claude o una organización diferente desde que la inició, Claude le indica que la sesión no fue encontrada.
* Si cambió de cuenta, la revisión aún puede finalizar bajo la cuenta que la inició. Si la revisión aún se está ejecutando, inicie sesión nuevamente como esa cuenta y reanude la conversación con `claude --resume` para volver a adjuntarla.

Cuando la revisión finaliza, Claude Code muestra los hallazgos verificados como una notificación en su sesión. Cada hallazgo incluye la ubicación del archivo y una explicación del problema para que pueda pedirle a Claude que lo corrija directamente.

<h2 id="run-ultrareview-non-interactively">
  Ejecutar ultrareview de forma no interactiva
</h2>

Use el subcomando `claude ultrareview` para iniciar una ultrareview desde CI o un script sin una sesión interactiva. El subcomando inicia la misma revisión que `/code-review ultra`, se bloquea hasta que la revisión remota finalice e imprime los hallazgos en stdout.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sin argumentos, el subcomando revisa el diff entre su rama actual y la rama predeterminada, con el mismo [respaldo de repositorio completo](#diff-limits-and-fallbacks) que `/code-review ultra` cuando no existe una base de fusión. Pase un número de PR para revisar una solicitud de extracción, o una rama base para revisar contra ella; el [manejo de rama base](#review-against-a-different-base) coincide con el comando interactivo.

Usted da su consentimiento al respaldo de repositorio completo y al aviso de facturación y términos cuando ejecuta el subcomando, por lo que la ejecución comienza sin esperar entrada. Ejecutarlo usted mismo es lo que cuenta como consentimiento. Cuando Claude ejecuta el subcomando por usted, por ejemplo a través de la herramienta Bash, Claude Code rechaza la revisión de repositorio completo.

En Claude Code v2.1.218 o posterior, también puede iniciar la revisión en la nube ejecutando `/code-review ultra` en una sesión no interactiva, por ejemplo `claude -p '/code-review ultra'`. Claude Code inicia la revisión e imprime un enlace de seguimiento sin esperar los hallazgos, a diferencia de `claude ultrareview`, que se bloquea hasta que lleguen. Cuando la revisión facturaría créditos de uso, Claude Code se detiene antes de iniciar y lo señala a `claude ultrareview`, porque la confirmación de facturación necesita una sesión interactiva. Antes de v2.1.218, `/code-review ultra` en una sesión no interactiva ejecutaba una revisión local.

Los mensajes de progreso y la URL de sesión en vivo van a stderr para que stdout permanezca analizable. Use estas banderas para controlar la salida, el tiempo de espera y si publicar los hallazgos:

| Bandera               | Descripción                                                                                                                                                                                                                                                                                                                           |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--json`              | Imprime la carga útil `bugs.json` sin procesar en lugar de los hallazgos formateados                                                                                                                                                                                                                                                  |
| `--timeout <minutes>` | Minutos máximos para esperar a que finalice la revisión. Por defecto es 45                                                                                                                                                                                                                                                            |
| `--post`              | [Publica los hallazgos finalizados](#post-findings-to-the-pull-request) en la solicitud de extracción como un comentario simple desde su cuenta de GitHub. Funciona en objetivos de solicitud de extracción de `github.com`; en otros objetivos, Claude Code ignora la bandera y lo indica. Requiere Claude Code v2.1.227 o posterior |
| `--no-post`           | No publica los hallazgos. Este es el valor predeterminado, y si pasa ambas banderas, Claude Code no publica. Requiere Claude Code v2.1.227 o posterior                                                                                                                                                                                |

Ejecutar `claude ultrareview` requiere la misma autenticación y configuración de créditos de uso que `/code-review ultra`.

El subcomando sale con uno de tres códigos:

* **0**: la revisión se completó, con o sin hallazgos
* **1**: la revisión no se pudo iniciar o se detuvo antes de finalizar, la sesión en la nube generó un error, o se agotó el tiempo de espera
* **130**: interrumpió el subcomando con Ctrl-C

Si interrumpe el subcomando, la revisión remota continúa ejecutándose; siga el enlace de sesión impreso en stderr para verla en el navegador.

Con `--post`, el subcomando inicia la publicación justo después de imprimir los hallazgos e imprime el enlace en stderr.

* Si la ejecución falla, se detiene, se agota el tiempo de espera, o si la interrumpe, el subcomando no publica nada.
* Si la revisión se completa pero el comentario no se publica, Claude Code imprime el motivo en stderr, y los hallazgos permanecen en stdout para que pueda publicarlos manualmente.

Para revisiones automáticas en solicitudes de extracción de GitHub, [Code Review](/docs/es/code-review) se integra directamente con su repositorio y publica hallazgos como comentarios PR en línea sin un paso de CLI.

<h2 id="how-ultrareview-compares-to-/code-review">
  Cómo ultrareview se compara con /code-review
</h2>

Ambas revisiones examinan código, pero las utiliza en diferentes etapas de su flujo de trabajo.

|             | `/code-review`                                                       | `/code-review ultra`                                                                      |
| ----------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Objetivo    | su diff de trabajo, una solicitud de extracción, una rama o una ruta | su diff de trabajo o una solicitud de extracción                                          |
| Se ejecuta  | localmente en su sesión                                              | en un sandbox en la nube                                                                  |
| Profundidad | se escala con el argumento de esfuerzo                               | flota multiagente con verificación independiente                                          |
| Duración    | segundos a pocos minutos                                             | aproximadamente 5 a 10 minutos                                                            |
| Costo       | cuenta hacia el uso normal                                           | ejecuciones gratuitas, luego aproximadamente \$5 a \$25 por revisión como créditos de uso |
| Mejor para  | retroalimentación rápida mientras itera                              | confianza previa a la fusión en cambios sustanciales                                      |

Utilice `/code-review` para retroalimentación rápida mientras trabaja, o pase un número de PR para revisar la solicitud de extracción de un compañero antes de aprobarla. Utilice `/code-review ultra` antes de fusionar un cambio sustancial cuando desee una pasada más profunda que detecte problemas que una revisión local podría perder.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code en la web](/docs/es/claude-code-on-the-web): aprende cómo funcionan las sesiones en la nube y los sandboxes en la nube
* [Gestiona costos de manera efectiva](/docs/es/costs): rastrear el uso y establecer límites de gasto
