> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizar sesiones en entornos autohospedados

> Personaliza sesiones de entornos autohospedados con scripts contenedores para credenciales por sesión, hooks de ciclo de vida y generación de ejecutores bajo demanda.

<Note>
  Los entornos autohospedados están en versión beta pública en planes Team y Enterprise; un [Propietario](/docs/es/cloud-environments#organization-shared-environments) los habilita activando **Allow self-hosted environments** en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Esta página asume un ejecutor funcional; consulte el [inicio rápido](/docs/es/self-hosted-environments-quickstart) para la configuración e [Implementar en producción](/docs/es/self-hosted-environments-deploy) para las recetas de flota.
</Note>

Un [entorno autohospedado](/docs/es/self-hosted-environments) ejecuta [sesiones en la nube](/docs/es/claude-code-on-the-web) de Claude Code en su propia infraestructura, ejecutadas por un proceso ejecutor que usted implementa. Sin configuración, ese ejecutor clona el repositorio de la sesión, genera Claude Code y limpia. Esta página es para el ingeniero de plataforma que opera los ejecutores: cubre los puntos de extensión para cuando esos valores predeterminados no se ajustan, desde el aprovisionamiento de credenciales por sesión hasta reemplazar completamente el checkout. Los contenedores y hooks se ejecutan como archivos ejecutables en el host del ejecutor, que es Linux o macOS, y los ejemplos en esta página asumen un shell POSIX.

Algunas variables de entorno de hook en esta página aún usan `pool`, como `CLAUDE_RUNNER_POOL_ID`; los nombres de banderas CLI y variables de entorno usan `environment`, como `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Scripts contenedores
</h2>

Use un script contenedor cuando cada sesión necesite configuración que el ejecutor no puede hacer por sí solo: aprovisionamiento de credenciales de corta duración limitadas al creador de la sesión, exportación de secretos específicos del entorno, preparación de cadenas de herramientas de lenguaje o aplicación de límites de recursos alrededor del proceso secundario. El ejecutor inicia su contenedor en lugar del binario de Claude Code, una vez por sesión. Termine el contenedor con `exec` en `$CLAUDE_RUNNER_CLAUDE_BIN`, el binario propio del ejecutor, para que las señales y códigos de salida se propaguen correctamente.

Apunte `--exec-path`, o `SELF_HOSTED_RUNNER_EXEC_PATH`, al contenedor cuando inicie el ejecutor:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

El ejecutor establece lo siguiente en el entorno del contenedor:

| Variable                            | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | El JWT de la sesión, con prefijo `sk-ant-cc-`. Su reclamación `act` identifica al creador de la sesión, con el correo electrónico del creador y el asunto del proveedor de identidad ascendente cuando la superficie de creación los registró. El valor es el token en el momento del generación; los refrescos llegan a través de stdin del hijo, por lo que un contenedor solo ve el valor inicial. Consulte [Verificar identidad de sesión](/docs/es/self-hosted-environments-identity).                                                                                                                                                                                                                                                   |
| `CCR_SESSION_ACCOUNT_EMAIL`         | El correo electrónico del creador de la sesión, preextraído por el ejecutor de la reclamación `act.email` del token sin verificación de firma. Adecuado para etiquetado, como remolques de commit. Cuando el correo electrónico controla la emisión de credenciales, verifique el token y lea la reclamación de él en su lugar; consulte [Aprovisionar credenciales limitadas al creador de la sesión](#provision-credentials-scoped-to-the-session-creator). No establecido cuando el token no lleva correo electrónico del creador. Trate como información de identificación personal.                                                                                                                                                 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | La superficie del cliente que creó la sesión, como `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli` o `scheduled_trigger`. Anthropic registra el valor una vez en la creación de la sesión, por lo que el contenedor y cada hook de ciclo de vida ven el mismo valor. Úselo solo para análisis de adopción y etiquetado, no como señal de autorización. No establecido cuando la sesión no tiene una superficie registrada o reconocida, por lo que haga referencia a él como `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` bajo `set -u`. Requiere Claude Code v2.1.229 o posterior.                                                                                                                                                     |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Ruta absoluta al binario de Claude Code propio del ejecutor. Termine su contenedor con `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` para entregar al binario fijado sin codificar una ruta de instalación.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | ID de sesión en forma etiquetada `cse_...`. Esta es la misma sesión que los [hooks de ciclo de vida](#lifecycle-hooks) ven como `CLAUDE_RUNNER_SESSION_ID` en forma `session_...`; las variables UUID coinciden en ambas, y sustituir el prefijo `cse_` con `session_` produce el ID que se muestra en la URL de la sesión.                                                                                                                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | El mismo ID de sesión en forma UUID canónica, para sistemas que usan UUID como clave.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Ruta absoluta a un archivo por sesión que contiene el JWT de sesión actual, mantenido fresco en los refrescos de token. Los subprocesos de shell lo leen para su encabezado `Authorization` al descargar archivos adjuntos que el usuario agregó a la sesión. `exec` preserva la variable automáticamente; un contenedor que reconstruye el entorno del hijo debe llevar la variable, o las descargas de archivos adjuntos dejan de funcionar silenciosamente.                                                                                                                                                                                                                                                                           |
| `CLAUDE_CONFIG_DIR`                 | Directorio de configuración de Claude por sesión, escrito al inicio de la sesión desde la instantánea de la configuración del host del ejecutor que el ejecutor captura al inicio; consulte [Permisos y aprobación de herramientas](#permissions-and-tool-approval). Las escrituras aquí se aíslan a esta sesión. El directorio permanece bajo `<base-dir>/_sessions/` después de que la sesión finaliza a menos que inicie el ejecutor con [`--remove-session-state`](/docs/es/self-hosted-environments-reference#runner-cli-flags); consulte [Reutilizar un checkout precalentado](/docs/es/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                        |
| `ANTHROPIC_BASE_URL`                | La URL base de API que el hijo usará, entregada por el plano de control por sesión y normalmente `https://api.anthropic.com`. No la anule: la credencial de inferencia de la sesión es un token OAuth emitido por Anthropic que otros proveedores no aceptan, por lo que la inferencia en entornos autohospedados no es enrutable a otros lugares.                                                                                                                                                                                                                                                                                                                                                                                       |
| `CLAUDE_CODE_OAUTH_TOKEN`           | El token de acceso OAuth de corta duración que el hijo usa para inferencia de modelo, limitado a inferencia de modelo y carga de archivos solamente, con una vida útil de aproximadamente 30 minutos. El ejecutor lo vuelve a acuñar antes del vencimiento y entrega la rotación a través de stdin del hijo, por lo que un contenedor que no [mantiene stdin adjunto](#keep-stdin-and-file-descriptor-3-attached) solo ve el valor inicial. No confíe en la lista de permitidos de IP de su organización para limitar el uso de este token: trate como una credencial de portador que permanece utilizable durante aproximadamente 30 minutos si se filtra, y no la registre, escriba en disco o reenvíe fuera del contenedor de sesión. |

El contenedor también hereda el resto del entorno administrado del hijo, incluidas cualquier variable de entorno proporcionada por el servidor. `exec` lo propaga todo automáticamente; si su contenedor genera el hijo de otra manera, reenvíe el entorno completo.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Mantener stdin y descriptor de archivo 3 adjuntos
</h3>

El stdin del hijo es el canal de control del ejecutor. Las rotaciones de token y las señales de fin de sesión llegan en él. El ejecutor también abre una tubería en el descriptor de archivo 3 y lee las señales de actividad del hijo de ella para impulsar los tiempos de espera de inactividad e inicio. Un simple `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` preserva ambos automáticamente.

Si su contenedor coloca el hijo en segundo plano con un simple `&`, corta el stdin del hijo: la sesión se ve saludable hasta que la vida útil del token OAuth inicial de aproximadamente 30 minutos expira, luego cada llamada de API falla con `401 authentication_error`. Si su contenedor debe colocar el hijo en segundo plano, por ejemplo para mantener viva una trampa de desmontaje, guarde stdin en el descriptor de archivo 4 o superior y vuelva a adjuntarlo explícitamente:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

No cierre ni reutilice el descriptor de archivo 3 en el contenedor. Redirigir stdout y stderr del hijo está bien.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Aprovisionar credenciales limitadas al creador de la sesión
</h3>

Use el subcomando `decode-token` para leer reclamaciones del JWT de sesión. Lee el token de un argumento, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` o de stdin, en ese orden; consulte [Verificar el token dentro de la sesión](/docs/es/self-hosted-environments-identity#verify-the-token-inside-the-session) para lo que verifica. El ejemplo a continuación decodifica la identidad del creador, la intercambia por credenciales de AWS de corta duración y ejecuta en Claude Code:

```bash theme={null}
#!/bin/bash
# Clave en el ID de usuario de Anthropic estable y requiere un creador humano.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Use `jq -re` en lugar de `jq -r` cuando la reclamación extraída controla una decisión de autenticación, para que una reclamación ausente salga con código distinto de cero en lugar de pasar la cadena literal `null` aguas abajo. Las sesiones creadas por una identidad de servicio de la organización, como sesiones de bot y agente, llevan un asunto `agent:` en lugar de `user:`, por lo que este ejemplo las rechaza; si su entorno sirve esas sesiones, decida explícitamente si el contenedor vuelve a una credencial predeterminada para ellas en lugar de salir. Cuando su intercambio de credenciales necesita el asunto o correo electrónico de SSO en su lugar, lea `.act.attested_by.sub` o `.act.email` y maneje su ausencia: el token los lleva solo cuando la superficie de creación los registró, y una [sesión despachada por CLI](/docs/es/self-hosted-environments-testing#run-the-test-loop) puede carecer de ambos. Para la referencia de reclamación completa y verificación desde servicios fuera del ejecutor, consulte [Verificar identidad de sesión](/docs/es/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Hooks de ciclo de vida
</h2>

Los hooks de ciclo de vida reemplazan etapas del pipeline por sesión del ejecutor con sus propios scripts. Apunte el ejecutor a un directorio de hooks con `--hooks-dir <path>`, o `SELF_HOSTED_RUNNER_HOOKS_DIR`. El ejecutor busca archivos ejecutables con nombres bien conocidos; cualquier hook que no esté presente cae en el comportamiento integrado, por lo que solo escribe los que necesita. Los hooks se ejecutan con los privilegios propios del ejecutor, y los hijos de sesión comparten ese UID, por lo que monte el directorio de hooks como solo lectura, o incorpórelo en la imagen, para que el código de sesión no pueda modificarlo; consulte la [sección de endurecimiento](/docs/es/self-hosted-environments-deploy#harden-your-deployment).

Estos hooks son distintos de los [hooks de Claude Code](/docs/es/hooks), que se ejecutan dentro de la sesión; los hooks de ciclo de vida se ejecutan en el ejecutor, alrededor de la sesión.

<h3 id="checkout">
  checkout
</h3>

Se ejecuta una vez por repositorio, en lugar del clon e incorporación integrados del ejecutor. Use el hook para clonar desde un espejo de lectura, sembrar un árbol de trabajo desde un archivo, o aplicar autenticación git por sesión. El ejecutor establece:

| Variable                           | Descripción                                                                                                                                                                 |
| :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | URL del repositorio a clonar, después de que se hayan aplicado `--git-host-rewrite` y `--git-ssh-rewrite`                                                                   |
| `CLAUDE_RUNNER_REPO_REF`           | Revisión a verificar: rama, etiqueta o SHA de commit como la sesión lo solicitó. Vacío significa la rama predeterminada del repositorio.                                    |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Ruta absoluta donde el árbol de trabajo debe dejarse                                                                                                                        |
| `CLAUDE_RUNNER_SESSION_ID`         | ID de sesión en forma etiquetada `session_...`, para registro y correlación                                                                                                 |
| `CLAUDE_RUNNER_SESSION_UUID`       | El mismo ID de sesión en forma UUID canónica                                                                                                                                |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL base de API de Anthropic para llamadas limitadas a sesión                                                                                                               |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La superficie del cliente que creó la sesión, como `web_claude_ai`, `desktop_app` o `ios`. No establecido cuando la sesión no tiene una superficie registrada o reconocida. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | El token de acceso de sesión, para llamadas de API limitadas a sesión                                                                                                       |

El script debe dejar un árbol de trabajo en `CLAUDE_RUNNER_CHECKOUT_PATH` verificado en la revisión solicitada. HEAD desacoplado está bien; el ejecutor crea la rama de trabajo de la sesión encima. El ejecutor verifica que la ruta contenga un `.git` después; si su hook materializa una fuente no git como Perforce o un tarball desempaquetado, establezca `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` en el entorno del ejecutor para omitir esa verificación. Los flujos basados en git como la creación de rama de trabajo y el envío de resultados requieren un checkout de git, por lo que exporte resultados de árboles no git con un hook [`post-session`](#post-session).

El ejecutor no pasa una credencial git al hook. En su lugar, acuñe una credencial de clon por sesión de la identidad de la sesión: verifique `CLAUDE_CODE_SESSION_ACCESS_TOKEN` con una biblioteca JWT estándar contra el punto final JWKS bajo `CLAUDE_RUNNER_API_BASE_URL`, como se describe en [Verificar el token desde su servicio](/docs/es/self-hosted-environments-identity#verify-the-token-from-your-service), luego haga que su servicio de credenciales emita una credencial de clon de corta duración para la identidad en la reclamación `act` del token. `CLAUDE_RUNNER_CLAUDE_BIN` no se establece en el entorno del hook de checkout, por lo que el subcomando `decode-token` no está disponible aquí. Volver a la autenticación git que el host ya tiene, como un agente SSH, ayudante de credenciales o `.netrc`, también es una opción.

Cuando el hook sale con código distinto de cero, o sale con 0 sin dejar un checkout utilizable detrás, lo que hace el ejecutor depende del repositorio:

* **Un repositorio al que la sesión envía resultados**: el ejecutor falla la sesión, y en una salida distinta de cero muestra la cola del stderr del script al usuario.
* **Un repositorio que la sesión solo lee**, como un repositorio agregado a una sesión en ejecución: el ejecutor registra una línea `[runner:warn]` con el detalle de falla, publica un paso `Skipped` a la sesión, elimina lo que el hook dejó en la ruta de checkout y continúa con los repositorios restantes. Cuando el ejecutor no puede eliminar la ruta inmediatamente, reintenta la eliminación al final de la sesión. Si omitir deja la sesión sin repositorio en absoluto, el ejecutor falla la sesión de todas formas.

Antes de v2.1.228, el ejecutor fallaba la sesión en un fallo de hook para cualquier repositorio, por lo que un repositorio de solo lectura que el hook no podía servir fallaba la sesión nuevamente en cada ejecutor nuevo en el que la sesión se reanudaba.

El ejecutor elimina la ruta de checkout después de que la sesión termina.

<h3 id="post-session">
  post-session
</h3>

Se ejecuta una vez por sesión, después de que el hijo de Claude Code ha salido y antes de que el ejecutor desmonte el espacio de trabajo. Este hook es su única oportunidad para guardar trabajo no confirmado: en `--capacity` por encima de uno, el ejecutor elimina árboles de trabajo por sesión justo después de que el hook regresa, y en `--capacity 1` el [clon canónico](/docs/es/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) reutilizado se reinicia cuando la siguiente sesión comienza, por lo que los cambios rastreados no confirmados no sobreviven en ninguna ruta. Los usos típicos son enviar una rama de instantánea de cambios no confirmados, archivar registros o emitir un evento de fin de sesión a sus propios sistemas.

El hook se dispara en cada fin de sesión donde se generó un proceso hijo, sea cual sea la causa; los valores `CLAUDE_RUNNER_EXIT_REASON` a continuación enumeran los casos. No puede dispararse cuando el ejecutor termina abruptamente, como una preferencia de VM o una pérdida de energía; si necesita garantías contra terminación abrupta, tome instantáneas periódicamente desde dentro de la sesión con un hook `PostToolUse` de Claude Code en su lugar. El ejecutor establece:

| Variable                           | Descripción                                                                                                                                                                                                            |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | ID de sesión en forma etiquetada `session_...`                                                                                                                                                                         |
| `CLAUDE_RUNNER_SESSION_UUID`       | El mismo ID de sesión en forma UUID canónica                                                                                                                                                                           |
| `CLAUDE_RUNNER_EXIT_REASON`        | Cómo terminó la sesión; consulte los valores a continuación de la tabla                                                                                                                                                |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Rutas absolutas separadas por dos puntos de los árboles de trabajo de la sesión. Vacío para sesiones sin repositorio.                                                                                                  |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Ruta al registro de depuración de la sesión, aún en disco mientras se ejecuta el hook                                                                                                                                  |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL base de API de Anthropic para llamadas limitadas a sesión                                                                                                                                                          |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La superficie del cliente que creó la sesión, como `web_claude_ai`, `desktop_app` o `ios`. No establecido cuando la sesión no tiene una superficie registrada o reconocida. Requiere Claude Code v2.1.229 o posterior. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | El token de acceso de sesión, para llamadas de API limitadas a sesión                                                                                                                                                  |

`CLAUDE_RUNNER_EXIT_REASON` toma uno de cuatro valores:

* `completed`: la sesión terminó limpiamente. El proceso de Claude Code salió normalmente, o la sesión fue archivada o eliminada mientras aún se estaba ejecutando.
* `failed`: el proceso de Claude Code se bloqueó, o la configuración falló después de que comenzó.
* `interrupted`: el ejecutor detuvo la sesión. Liberó la sesión para liberar la ranura, la sesión agotó el tiempo de espera al inicio, el servidor movió la sesión fuera de este ejecutor, el ejecutor estaba drenando, o la sesión superó su límite [`--kill-session-after-min`](/docs/es/self-hosted-environments-reference#runner-cli-flags).
* `abandoned`: reservado para una sesión que otro ejecutor reclamó. El hook actualmente no se dispara en ese caso.

Los [contadores de ciclo de vida de sesión](/docs/es/self-hosted-environments-reference#session-lifecycle-counter-semantics) cuentan una liberación, un tiempo de espera de inicio y un movimiento de servidor como `completed` en lugar de `interrupted`, porque el ejecutor devolvió la ranura limpiamente. Espere esa diferencia si compara recibos de hook con los contadores.

El estado de salida del hook nunca afecta el resultado de la sesión; un fallo se registra e ignora. El ejecutor espera hasta `--post-session-hook-timeout-sec`, 60 segundos por defecto, en cada fin de sesión incluido el apagado del ejecutor. Este ejemplo guarda trabajo no confirmado en una rama de rescate:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Fije la configuración que la sesión podría haber plantado en .git/config del checkout:
# -c anula los ajustes a nivel de repositorio, bloqueando fsmonitor escrito por sesión,
# ruta de hook y configuración de programa gpg de ejecutar código con los privilegios del hook.
# credential.helper a nivel de repositorio, core.sshCommand y pushurl aún se aplican;
# si el hook tiene credenciales que la sesión no tenía, fije la URL de envío y el ayudante también
# (consulte la nota a continuación del script).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

El hook envía con cualquier credencial git disponible en su propio entorno en el host del ejecutor. Bajo la [postura de no credenciales en la imagen](/docs/es/self-hosted-environments-deploy#configure-git), incluido cuando el clon integrado pasa por el proxy git de Anthropic, no hay ninguna, por lo que acuñe una credencial de envío de corta duración dentro del hook antes de enviar: intercambie el token de sesión que el hook recibe en `CLAUDE_CODE_SESSION_ACCESS_TOKEN` con su propio servicio de token, verificándolo como [Verificar identidad de sesión](/docs/es/self-hosted-environments-identity) describe. Cuando el hook tiene una credencial que la sesión no tenía, también fije dónde envía: reemplace `origin` con una URL proporcionada por el operador y pase `-c credential.helper=` más su propio ayudante, para que la configuración a nivel de repositorio que la sesión escribió no pueda redirigir el envío acreditado.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Tiempo del hook cuando el ejecutor libera una sesión
</h4>

Una sesión liberada puede reanudarse en otro ejecutor. En un ejecutor en v2.1.236 o posterior, lo que la sesión estaba haciendo en la liberación decide si puede reanudarse antes de que este hook termine:

* **Inactivo después de un turno, o tiempo de espera agotado al inicio**: el ejecutor detiene el hijo y ejecuta este hook hasta completarse. Solo entonces libera la sesión. Un mensaje de usuario enviado mientras se ejecuta el hook no puede reanudar la sesión en otro ejecutor antes de que el hook termine.
* **Esperando que el usuario responda a un mensaje, como un mensaje de permiso**: el ejecutor libera la sesión primero, luego ejecuta este hook. Un mensaje de usuario enviado mientras se ejecuta el hook puede reanudar la sesión en otro ejecutor antes de que el hook termine.

Esto se aplica siempre que el ejecutor libera una sesión: en el tiempo de espera inactivo, en el tiempo [`--retire-at`](/docs/es/self-hosted-environments-reference#runner-cli-flags), y, en un ejecutor en v2.1.260 o posterior, en el límite [`--kill-session-after-min`](/docs/es/self-hosted-environments-reference#runner-cli-flags) de una sesión. Una sesión cuyo turno ha terminado y que solo contiene tareas de fondo cuenta como inactiva aquí. Antes de v2.1.236, el ejecutor liberaba la sesión primero y luego ejecutaba este hook en ambos casos.

Durante un drenaje `SIGTERM`, el ejecutor mantiene el arrendamiento de sesión hasta que el hook termina; consulte [Tiempo de apagado](/docs/es/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

Se ejecuta una vez por sesión después del checkout, en lugar de la generación de hijo integrada. El hook recibe el mismo entorno que un [script contenedor](#wrapper-scripts) y debe `exec` en `"$CLAUDE_RUNNER_CLAUDE_BIN"` de la misma manera. Use el hook `command` para mantener toda la personalización en un directorio de hooks; use `--exec-path` cuando el contenedor vive en otro lugar. Si `--exec-path` también se establece, la bandera tiene precedencia y el hook `command` se ignora.

Siempre `exec` el binario propio del ejecutor en lugar de un `claude` resuelto por PATH; de lo contrario, anula el [fijación de versión](/docs/es/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  Ejecutores bajo demanda
</h2>

En lugar de ejecutar una flota fija, puede arrancar un ejecutor por sesión. El orquestador es un subcomando separado y sin estado que sondea a Anthropic para solicitudes de generación, una por sesión que está en cola sin ejecutor disponible, y ejecuta su hook `spawn-runner` para cada una. Su hook envía una carga de trabajo a su plataforma: un Job de Kubernetes, una instancia de EC2, un despacho de Nomad.

Los ejecutores bajo demanda mejoran la higiene de credenciales. En una flota fija, el secreto del entorno vive en cada host del ejecutor, que es el mismo host que ejecuta sesiones de usuario. Con el orquestador, el secreto del entorno permanece solo en el host del orquestador, que nunca ejecuta código de usuario; cada ejecutor generado recibe una orden de trabajo de un solo uso que registra exactamente un ejecutor y luego expira.

Para iniciar el orquestador, pase el secreto del entorno y un directorio de hooks que contenga un script `spawn-runner` ejecutable:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

El orquestador no mantiene estado entre sondeos, por lo que puede ejecutar dos o más réplicas contra el mismo entorno para disponibilidad. Cada solicitud de generación es reclamada del lado del servidor por exactamente una réplica. Todas las réplicas deben usar el mismo valor `--expected-spawn-seconds`; consulte el [contrato del hook](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  El hook spawn-runner
</h3>

El orquestador ejecuta `${hooks-dir}/spawn-runner` una vez por solicitud de generación. El hook debe enviar trabajo de forma asincrónica, sin esperar a que el ejecutor arranque, y regresar dentro de `--hook-timeout`, 60 segundos por defecto. El hook recibe:

| Variable                              | Descripción                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Ruta a un archivo temporal que contiene el JWT de orden de trabajo firmado que el nuevo ejecutor registra. Eliminado después de que el hook sale. No registre el contenido del archivo.                                                                                                                                                                                |
| `CLAUDE_RUNNER_ORDER_ID`              | Clave de idempotencia opaca, única por solicitud de generación y segura para nombres de recursos de Kubernetes. Úsela como clave de deduplicación de su aprovisionador.                                                                                                                                                                                                |
| `CLAUDE_RUNNER_SESSION_ID`            | La sesión para la que es esta solicitud. Vacío para solicitudes de precalentamiento, que arrancan un ejecutor en espera antes de cualquier sesión específica cuando [`--min-idle`](/docs/es/self-hosted-environments-reference#orchestrator-cli-flags) se establece, por lo que no asuma que la variable se establece.                                                      |
| `CLAUDE_RUNNER_SESSION_UUID`          | El mismo ID de sesión en forma UUID canónica. Vacío para solicitudes de precalentamiento.                                                                                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_ATTEMPT`               | Cuántas solicitudes de generación ha tenido esta sesión. `0` para solicitudes de precalentamiento.                                                                                                                                                                                                                                                                     |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Hora del servidor del encabezado HTTP `Date` de la respuesta de sondeo. Cuando el hook verifica el `exp` del JWT de orden de trabajo, compare contra este valor en lugar del reloj local para tolerar sesgo. Vacío cuando la puerta de enlace omitió el encabezado.                                                                                                    |
| `CLAUDE_RUNNER_POOL_ID`               | El ID del entorno al que el nuevo ejecutor debe unirse, en forma `ccpool_...`                                                                                                                                                                                                                                                                                          |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | ID etiquetado de la cuenta que encoló la sesión, para enrutamiento por cuenta, cuota o contracargo. Vacío cuando no está disponible, y siempre vacío para sesiones de canal Claude Tag, que ninguna cuenta encola.                                                                                                                                                     |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | Correo electrónico de la cuenta que encoló la sesión. Vacío cuando no está disponible. Trate el correo electrónico como información de identificación personal y no lo registre.                                                                                                                                                                                       |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL de la primera fuente git de la sesión, para enrutamiento a un ejecutor con ese repositorio precalentado. Vacío cuando la sesión no tiene fuentes git.                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Revisión de la primera fuente git de la sesión: rama, SHA o etiqueta. Vacío cuando no se especifica.                                                                                                                                                                                                                                                                   |
| `CLAUDE_RUNNER_REPO_SOURCES`          | Matriz JSON de `{url, revision}` para todas las fuentes git de la sesión, para hooks que enrutan en un repositorio secundario. Vacío cuando no hay fuentes.                                                                                                                                                                                                            |
| `CLAUDE_RUNNER_CORRELATION_ID`        | El ID de correlación proporcionado en la creación de sesión, devuelto para que el hook pueda asignar esta orden de trabajo a la solicitud que creó la sesión. Vacío cuando la sesión no tiene ninguno.                                                                                                                                                                 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | La superficie del cliente que creó la sesión, como `web_claude_ai`, `desktop_app`, `ios` o `scheduled_trigger`, para análisis de adopción. No establecido cuando la sesión no tiene una superficie registrada o reconocida, y para solicitudes de precalentamiento; verifíquelo con `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, que permanece seguro bajo `set -u`. |

El ejecutor generado se registra con la orden de trabajo en lugar del secreto del entorno:

* **Inicie con la orden de trabajo**: apunte [`--environment-secret-file`](/docs/es/self-hosted-environments-reference#runner-cli-flags) a un archivo que contenga el JWT de orden de trabajo, o establezca `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` al valor JWT.
* **Copie el JWT antes de que el hook salga**: el orquestador elimina el archivo de orden de trabajo después de que el hook sale, por lo que copie el JWT en la carga de trabajo que envía, como un Secret de Kubernetes en el Job generado, en lugar de pasar la ruta del archivo.
* **Use `--capacity 1` en ejecutores generados**: una orden de trabajo vinculada a sesión registra exactamente un ejecutor vinculado a esa sesión, por lo que una capacidad más alta agrega espacios que nunca reciben trabajo, y el ejecutor registra una advertencia al inicio.
* **Las órdenes de trabajo de precalentamiento registran sin vincular**: el ejecutor en espera no está vinculado a una sesión y reclama trabajo en cola como un ejecutor de flota fija.

El contrato tiene cuatro reglas agnósticas del aprovisionador:

1. **Sea idempotente en `CLAUDE_RUNNER_ORDER_ID`.** La reentrega de la misma solicitud debe generar como máximo un ejecutor. Derive un nombre de recurso determinista del ID y deje que su plataforma rechace el duplicado.
2. **No reintente la carga de trabajo.** Un ID de orden significa como máximo una carga de trabajo creada. Si el ejecutor nunca se registra, Anthropic reintenta con un ID de orden nuevo después de `--expected-spawn-seconds`.
3. **Use el contrato de código de salida.** Salida 0 significa enviado. Salida 1 significa fallo reintentable; la sesión retrocede y se reintenta. Salida 2 o superior significa no reintentable; la sesión se bloquea de generar nuevamente hasta que un [Propietario](/docs/es/cloud-environments#organization-shared-environments) selecciona **Retry** en ella en la pestaña **Activity** del entorno. En salida distinta de cero, la cola del stderr del hook aparece allí como la razón de falla, por lo que escriba el error procesable a stderr y nunca secretos. Para una solicitud de precalentamiento no hay sesión para fallar: el orquestador registra una salida distinta de cero localmente solamente, y el servidor reintenta la generación después del arrendamiento.
4. **Establezca `--expected-spawn-seconds` a al menos su tiempo de arranque p99.** Este es el arrendamiento del lado del servidor. Todas las réplicas del orquestador deben usar el mismo valor.

Todo lo que el hook escribe a stdout o stderr aparece en el registro del orquestador con credenciales automáticamente redactadas. Si las sesiones permanecen en cola, verifique el cuerpo `/healthz` del orquestador para contar colas, luego abra la pestaña **Activity** de su entorno en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments): expanda una sesión fallida allí para su error de generación, y seleccione **Retry** para reintentarla.

<h2 id="mcp-servers">
  Servidores MCP
</h2>

Para que los [servidores MCP](/docs/es/mcp) estén disponibles en cada sesión, agréguelos en el momento de la compilación de la imagen con el mismo comando `claude mcp add` que se utiliza en una instalación de escritorio. Si su ejecutor es un proceso simple en lugar de un contenedor, ejecute el mismo comando como usuario del ejecutor en el host y luego reinicie el ejecutor: lee la configuración del host una sola vez al inicio. La bandera `--scope user` es obligatoria; el alcance local predeterminado escribe bajo una clave por directorio que el ejecutor no proporciona en las sesiones. Por ejemplo, en su Dockerfile:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

El ejecutor captura la configuración del host una sola vez al inicio. La captura obtiene la clave `mcpServers` del `.claude.json` del host, que se encuentra junto a (no dentro de) `~/.claude/`, y el ejecutor proporciona solo esa clave en la configuración aislada de cada sesión; el estado de la cuenta y el historial del proyecto se descartan. Para confirmar que los servidores llegaron a las sesiones, inicie una sesión en el entorno y pida a Claude que enumere sus herramientas MCP; el ejecutor también registra una advertencia de inicio para cualquier entrada capturada cuyo `type` no reconoce y descarta la entrada, por lo que puede ver por qué falta ese servidor en las sesiones. Cuando se establece `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`, el ejecutor lee `.claude.json` de ese directorio en su lugar, por lo que apuntar la variable a un directorio vacío también desactiva la propagación de MCP.

Claude Code también carga servidores MCP de otras fuentes:

* El archivo [MCP administrado](/docs/es/managed-mcp) de alcance empresarial en su ruta estándar del sistema: `/etc/claude-code/managed-mcp.json` en hosts ejecutores de Linux, `/Library/Application Support/ClaudeCode/managed-mcp.json` en hosts macOS. Úselo para flotas bloqueadas donde solo los servidores enumerados por el administrador pueden cargarse. Consulte [control exclusivo con managed-mcp.json](/docs/es/managed-mcp#exclusive-control-with-managed-mcp-json) para las reglas de precedencia. Cuando este archivo está en el host del ejecutor, Claude Code omite los servidores MCP que el plano de control de Anthropic entrega a una sesión, incluidos los conectores de claude.ai, y los nombra en una advertencia en stderr del hijo de la sesión, que el ejecutor registra en el nivel de registro `debug`. Antes de v2.1.229, esas sesiones salían al inicio con `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* La clave [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers) en [configuración administrada](/docs/es/managed-settings) en el host del ejecutor: proporciona servidores HTTP y SSE sin tomar control exclusivo, por lo que los servidores de las otras fuentes aún se cargan. Requiere Claude Code v2.1.259 o posterior.
* `<repo>/.mcp.json`: alcance del proyecto. Confirme el archivo en el repositorio; sus servidores se aprueban automáticamente en sesiones en la nube.

Cuando la entrega de conectores está habilitada para su organización, el plano de control de Anthropic entrega los conectores que ha configurado en claude.ai a sesiones creadas interactivamente a través de la configuración de MCP proporcionada por el servidor, enrutada a través de `api.anthropic.com`. Las sesiones creadas mediante programación, como [ejecuciones de CLI](/docs/es/self-hosted-environments-testing#run-the-test-loop), no reciben entrega de conectores; proporcióneles servidores MCP a través de cualquiera de las otras fuentes que esta sección enumera en su lugar. El token OAuth del hijo no lleva un alcance para obtener conectores directamente, por lo que el hijo no intenta esa obtención en sí mismo; la entrega es impulsada por el servidor.

`settings.json` no lleva definiciones de servidor MCP, y no hay un campo `mcpServers` de nivel superior en el esquema de configuración. En la configuración administrada, proporcione servidores con la clave [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers) en su lugar.

Las sesiones heredan el entorno del ejecutor, por lo que establezca [`ENABLE_TOOL_SEARCH`](/docs/es/mcp#scale-with-mcp-tool-search) allí para controlar la búsqueda de herramientas MCP para cada sesión que genera un ejecutor; la página de MCP cubre los valores.

<h2 id="prompt-sessions-to-push-their-work">
  Solicitar a sesiones que envíen su trabajo
</h2>

Las sesiones alojadas por Anthropic ejecutan un hook [`Stop`](/docs/es/hooks#stop), el hook de Claude Code que se ejecuta cuando Claude termina de responder, que solicita a Claude que confirme y envíe su trabajo. El ejecutor no instala uno. Sin él, una sesión que termina con cambios no confirmados deja ese trabajo solo en el disco del ejecutor, y el botón **Create PR** en claude.ai/code permanece inactivo hasta que la rama existe en el remoto.

La implementación de referencia a continuación tiene dos partes. Fusione el bloque de configuración en `~/.claude/settings.json` en el host del ejecutor, que el ejecutor siembra en cada sesión, y guarde el script como `~/.claude/hooks/stop-hook-nudge.sh` en el host del ejecutor y hágalo ejecutable:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Implementación de referencia de Stop-hook para ejecutores autohospedados.
#
# Solicita a Claude una vez por turno si el directorio del proyecto tiene cambios
# no confirmados O commits no enviados, para que el trabajo no se pierda cuando una
# sesión inactiva se libera y para que el botón "Create PR" en claude.ai/code se encienda.
#
# Nivel de ejecutor (sin cambios de repositorio): coloque este archivo en ~/.claude/hooks/
# en el host del ejecutor y fusione el bloque de configuración de Stop-hook adjunto
# en ~/.claude/settings.json — el ejecutor siembra ambos en cada sesión.
# Alternativa a nivel de repositorio: confirme en <repo>/.claude/hooks/ y cambie la
# ruta del comando settings.json a $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: carga útil JSON del hook (consulte https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} para solicitar, o nada para permitir stop.

# Guardia de reentrada: el arnés establece stop_hook_active=true cuando reinvoca
# el hook Stop después de un bloque. Salga para que solo solicitemos una vez por turno.
# El arnés emite JSON compacto (sin espacio después de los dos puntos), que este
# patrón depende; use jq si necesita una verificación tolerante a espacios en blanco.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# No es un repositorio git → nada que solicitar.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# Sin remoto → "enviar al remoto" es insatisfacible; salga.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Cambios no confirmados (preparados, no preparados o sin rastrear). Excluya .claude/
# completamente — la configuración sembrada por operador y el estado de tiempo de ejecución
# escrito por CLI (bloqueo del programador, árboles de trabajo, estado de rutina)
# viven allí y ninguno es "trabajo no confirmado" que el modelo necesita enviar.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Commits no enviados. Cuente commits en HEAD no alcanzables desde ninguna
# referencia de seguimiento remoto o FETCH_HEAD. Esto funciona uniformemente para:
#   - checkouts init+fetch (predeterminado del ejecutor: solo existe FETCH_HEAD)
#   - checkouts basados en clon (existen origin/*)
#   - el predeterminado del ejecutor: el hijo comienza en la rama de resultado
#     de la sesión, que el ejecutor crea después del checkout
#   - HEAD desacoplado, cuando una configuración personalizada omite esa creación de rama
# Sin punto de referencia en absoluto (nunca obtenido), permanezca silencioso en lugar
# de falso positivo en un turno de solo lectura.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base es "" o "FETCH_HEAD", división de palabra intencional
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch está influenciado por atacante — git-check-ref-format(1) permite `"`
    # en nombres de referencia. `\` está prohibido (regla 10) pero escapado de todas formas como defensa
    # económica en profundidad.
    # Escape caracteres metacaracteres JSON antes de interpolar en la carga útil construida a mano
    # para que una rama como x","continue":false no pueda inyectar claves en
    # el JSON de salida del hook que el arnés analiza. $unpushed es seguro — la
    # guardia -gt anterior rechaza cualquier cosa que no sea un entero simple.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

El hook solicita a Claude que confirme y envíe antes de que la sesión termine, y permanece silencioso cuando el directorio no es un repositorio git o no tiene remoto.

<h2 id="permissions-and-tool-approval">
  Permisos y aprobación de herramientas
</h2>

Una sesión autohospedada no tiene terminal adjunta, por lo que un mensaje de permiso sin respuesta detiene el turno hasta que el usuario responde en la UI. El plano de control de Anthropic envía la lista de herramientas de cada sesión y las reglas de permiso con la carga útil de trabajo; la configuración predeterminada aprueba previamente llamadas de herramientas rutinarias, incluido `Bash`, y las sesiones en la nube [aprueban previamente ediciones de archivos independientemente del modo](/docs/es/permission-modes#switch-permission-modes). Una llamada que nada aprueba previamente solicita a través de la UI de sesión.

<Note>
  Solo fije el modo automático en un entorno cuyo contenedor de sesión se ejecute con [egreso de red de negación predeterminada](/docs/es/self-hosted-environments-deploy#default-deny-egress) y el resto de la [sección de endurecimiento](/docs/es/self-hosted-environments-deploy#harden-your-deployment) en su lugar. Las llamadas de herramientas rutinarias, incluidas solicitudes de red `Bash`, se ejecutan sin un humano en el bucle tanto en el conjunto de herramientas aprobadas previamente predeterminado como en modo automático, por lo que el límite de red es lo que limita dónde esas llamadas pueden llegar.
</Note>

Para mantener solicitudes al mínimo independientemente de lo que el plano de control envíe, fije el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) desde su script contenedor o hook [`command`](#command). El modo automático permite que las sesiones se ejecuten sin solicitudes de permiso rutinarias: un modelo clasificador separado revisa acciones antes de que se ejecuten y bloquea las que rechaza, y las reglas de solicitud explícita aún fuerzan una solicitud; la página de modos de permiso cubre lo que el clasificador verifica. El ejecutor agrega banderas calculadas por servidor antes de invocar el contenedor, y para banderas de valor único como `--permission-mode` el analizador honra la última ocurrencia, por lo que una bandera que agrega después de `"$@"` anula el valor enviado por servidor:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Para aprobar previamente herramientas específicas en su lugar, agregue `--allowed-tools` con sus reglas, por ejemplo `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. Las banderas de lista como `--allowed-tools` y `--disallowed-tools` se acumulan en ocurrencias en lugar de anular, por lo que sus reglas se aplican además de cualquier regla que el plano de control envíe. Para estrechar, agregue `--disallowed-tools`, que niega herramientas incluso si otra regla las permite.

<h3 id="how-each-session’s-config-is-assembled">
  Cómo se ensambla la configuración de cada sesión
</h3>

El ejecutor da a cada sesión su propio directorio de configuración, sembrado desde una instantánea en memoria de `~/.claude/` del host que el ejecutor captura una vez al inicio: `settings.json`, `CLAUDE.md`, hooks, agentes, comandos y habilidades en su imagen de ejecutor se aplican a cada sesión como la línea de base a nivel de usuario. Si cambia la configuración en un host en ejecución, el cambio tiene efecto solo después de reiniciar el ejecutor.

Establezca `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` para sembrar desde una ruta diferente, o apúntelo a un directorio vacío para deshabilitar la siembra.

El `.claude/settings.json` confirmado en el repositorio se superpone como configuración del proyecto. Las sesiones también leen [`managed-settings.json`](/docs/es/settings#where-settings-live) desde la ruta estándar del sistema en su imagen de ejecutor. Si sus claves se aplican junto con [configuración administrada por servidor](/docs/es/server-managed-settings) sigue [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources): por defecto, cuando su organización entrega cualquier clave administrada por servidor, las sesiones ignoran el archivo de imagen del ejecutor aparte de las [claves que Claude Code lee de cada fuente de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source), como el bloque `env`, los bloqueos de sandbox, las rutas binarias de sandbox y `forceRemoteSettingsRefresh`. Consulte [precedencia de configuración](/docs/es/settings#settings-precedence).

Cuando el plano de control de Anthropic proporciona una sesión con [hooks de Claude Code](/docs/es/hooks), el ejecutor los instala junto a, no sobre, su propia configuración. Requiere Claude Code v2.1.229 o posterior.

* **Dónde aterrizan**: el ejecutor escribe cada script de hook suministrado en un subdirectorio reservado `hooks/.ccr-launcher/` del directorio de configuración de la sesión y registra los scripts en un archivo de configuración separado que pasa a la sesión con `--settings`, dejando el `settings.json` sembrado y sus propios scripts en `hooks/<name>` sin tocar. El ejecutor recrea el subdirectorio reservado para cada sesión y no siembra contenido del host en `~/.claude/hooks/.ccr-launcher/` en sesiones.
* **Quién los autor**: el plano de control puebla los scripts desde constantes fijas en su propia implementación, nunca desde entrada por sesión o de terceros.
* **Qué aún los rige**: los hooks entregados a través de `--settings` entran en la configuración de hook ordinaria fusionada, no en el nivel administrado, por lo que su configuración administrada aún se aplica. `disableAllHooks` los deshabilita, y no están entre las categorías que [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly) mantiene cargadas.

<h3 id="repository-committed-permission-rules">
  Reglas de permiso confirmadas en el repositorio
</h3>

No coloque una entrada `"Edit"`, `"Write"` o `"NotebookEdit"` desnuda en un `permissions.allow` confirmado en el repositorio. Una regla de herramienta de archivo desnuda coincide con la herramienta independientemente de la ruta, otorgando escrituras en cualquier lugar del host en lugar de solo el espacio de trabajo, por lo que la guardia de confinamiento de alcance de escritura del ejecutor marca la sesión; con [`--confine-repo-settings enforce`](/docs/es/self-hosted-environments-reference#runner-cli-flags) se niega a generar la sesión en lugar de registrar y continuar. Consulte la [sección de endurecimiento](/docs/es/self-hosted-environments-deploy#harden-your-deployment).

Un repositorio no necesita ninguna regla de herramienta de archivo en absoluto: las sesiones en la nube [aprueban previamente ediciones de archivos independientemente del modo](/docs/es/permission-modes#switch-permission-modes). Si confirma una regla, limítela al espacio de trabajo, como `"Edit(/**)"`; una barra diagonal inicial única es relativa a la raíz del proyecto, que es el espacio de trabajo de la sesión. Las reglas de herramienta de archivo desnudas están bien en el `settings.json` a nivel de host del operador, ya que ese archivo no está confirmado en el repositorio.

Un `defaultMode` de `auto` solo se honra desde el archivo de configuración a nivel de imagen o a nivel de usuario, por lo que un repositorio verificado no puede otorgarse a sí mismo modo automático. Para qué modos aceptan las sesiones en la nube y la sintaxis de regla completa, consulte [modos de permiso](/docs/es/permission-modes).

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Referencia](/docs/es/self-hosted-environments-reference): cada bandera CLI, variable de entorno y métrica
* [Verificar identidad de sesión](/docs/es/self-hosted-environments-identity): validar el token de sesión desde servicios fuera del ejecutor
