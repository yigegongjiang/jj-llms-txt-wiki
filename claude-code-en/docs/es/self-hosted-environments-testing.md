> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Probar entornos autohospedados de extremo a extremo

> Verifique una imagen de ejecutor autohospedado desde CI: envíe una sesión con la CLI, lea las respuestas de Claude a través de un hook Stop y ejecute el bucle completo.

<Note>
  Los entornos autohospedados están en versión beta pública en planes Team y Enterprise; [Disponibilidad y limitaciones](/docs/es/self-hosted-environments#availability-and-limitations) cubre la ruta de habilitación. Esta página es la receta de prueba de CI; consulte el [inicio rápido](/docs/es/self-hosted-environments-quickstart) para la configuración y [Implementar en producción](/docs/es/self-hosted-environments-deploy) para las recetas de flota.
</Note>

En un [entorno autohospedado](/docs/es/self-hosted-environments), las [sesiones en la nube](/docs/es/claude-code-on-the-web) de Claude Code se ejecutan en una imagen de ejecutor que usted crea y mantiene. Antes de implementar una nueva imagen en su entorno de producción, ejecute una sesión completa contra un entorno de prueba desde un script: cree una sesión, lea la respuesta de Claude, envíe un seguimiento y lea esa respuesta también. Esta es la forma de una prueba de humo de CI que verifica su imagen de ejecutor, acceso a git y cualquier herramienta personalizada antes de que promueva un cambio.

Esta receta asume que ya ha [configurado un entorno y un ejecutor](/docs/es/self-hosted-environments-quickstart#set-up-an-environment-and-runner), y que su trabajo de CI inicia el proceso del ejecutor en el mismo host que el script de prueba, la configuración natural para probar una nueva imagen de ejecutor. Un hook Stop que instala en el ejecutor escribe la respuesta final de cada turno en un archivo local, y el script la lee desde allí, por lo que las únicas llamadas a la API de Anthropic son los dos envíos en sí. Si sus ejecutores de prueba están en infraestructura separada, consulte [Ejecutores de prueba remotos](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Instale el hook de captura en su ejecutor de prueba
</h2>

La lectura funciona a través de un [hook Stop](/docs/es/hooks#stop) de Claude Code: cuando Claude termina un turno, el hook recibe el mensaje final del asistente como `last_assistant_message` en su JSON de stdin y lo añade a `$E2E_REPLY_DIR/<session_id>.txt`. Instálelo de la misma manera que el [hook Stop commit-nudge](/docs/es/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), en `~/.claude/` del host del ejecutor, que el ejecutor siembra en cada sesión.

<h3 id="save-the-hook-files">
  Guarde los archivos del hook
</h3>

Guarde los dos archivos a continuación en el host del ejecutor:

* El bloque de configuración: fusione en `~/.claude/settings.json` en el host del ejecutor
* El script: guarde como `~/.claude/hooks/e2e-stop-hook-capture.sh` en el host del ejecutor y hágalo ejecutable

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Antes de iniciar el ejecutor
</h3>

El hook tiene estos requisitos:

* Instálelo antes de iniciar el ejecutor. El ejecutor toma una instantánea de `~/.claude/` una vez al inicio, por lo que un hook añadido a un ejecutor en ejecución solo tiene efecto después de un reinicio.
* Exporte `E2E_REPLY_DIR` al proceso del ejecutor. El hook es una operación sin efecto cuando la variable no está configurada o el directorio no existe, así que configúrelo donde inicie el ejecutor, como la unidad systemd, especificación de pod o paso de CI. El script de prueba a continuación también lo requiere.

Instale este hook solo en ejecutores que sirvan su entorno de prueba. Escribe la respuesta final de cada sesión en disco siempre que `E2E_REPLY_DIR` exista, lo cual es inofensivo en un ejecutor de CI desechable pero no algo que deba llevar a una imagen de ejecutor de entorno de producción donde la variable podría configurarse accidentalmente.

<h2 id="run-the-test-loop">
  Ejecute el bucle de prueba
</h2>

Los indicadores de envío `--environment` y `--ref` requieren Claude Code v2.1.224 o posterior en la máquina que ejecuta el script, el mismo piso que el ejecutor en sí. Con el hook en su lugar y un ejecutor iniciado en este host, el script de prueba:

1. Crea una sesión en el entorno de prueba con `claude -p "<prompt>" --environment <environment-id> --output-format json`, ejecutado desde un checkout de git para que la CLI pueda detectar automáticamente el repositorio desde el remoto `origin`. El `--ref <branch>` opcional basa el checkout de la sesión en una ref nombrada en lugar de HEAD local. El comando crea la sesión, imprime una línea de JSON que contiene `session_id` y sale sin esperar la respuesta de Claude.
2. Espera a que la respuesta aparezca en `$E2E_REPLY_DIR/<session_id>.txt`, escrita por el hook Stop en el ejecutor una vez que el turno se completa.
3. Envía un seguimiento con `claude -p "<message>" --cloud <session_id> --output-format json` (consulte [Enviar un mensaje de seguimiento a una sesión en ejecución](/docs/es/claude-code-on-the-web#send-follow-ups-from-the-cli)), que publica un evento de usuario en la sesión existente y sale.
4. Espera la respuesta del seguimiento de la misma manera que el paso 2.

<h3 id="environment-dispatch-behavior">
  Comportamiento de envío de `--environment`
</h3>

Claude Code crea la sesión, imprime el ID de sesión y un enlace a ella, y sale.

El indicador tiene precedencia sobre la configuración [`remote.defaultEnvironmentId`](/docs/es/settings-reference#remote-defaultenvironmentid). No admite `--output-format stream-json` y no se puede combinar con indicadores que reanuden, se adjunten o preconfiguran una sesión, como `--resume`, `--continue`, `--teleport`, `--session-id` o `--init-only`. `--cloud` se rechaza con un ID de sesión o URL, y en ejecuciones no interactivas cuando lleva una descripción. Un `--cloud` desnudo se trata como ausente. Desde una terminal, puede pasar la tarea como la descripción de `--cloud` en lugar de un prompt posicional.

<h2 id="example-script">
  Script de ejemplo
</h2>

El script a continuación ejecuta el bucle completo contra `$CLAUDE_TEST_ENVIRONMENT_ID`, el ID `ccpool_...` de su entorno de prueba, que se muestra en el diálogo de detalles del entorno en la página de administración o se devuelve por la [llamada create-environment](#create-a-dedicated-test-environment), y afirma una frase centinela en cada respuesta. Ejecute desde un checkout de git del repositorio en el que desea que la sesión funcione, después de iniciar un ejecutor en este host con el hook de captura instalado y `E2E_REPLY_DIR` exportado.

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Reemplace los prompts `TURN1`/`TURN2` y los centinelas `EXPECT1`/`EXPECT2` con lo que sea que ejercite su configuración, como pedirle a Claude que ejecute una de sus herramientas MCP personalizadas y afirmar su salida.

<h2 id="remote-test-runners">
  Ejecutores de prueba remotos
</h2>

Si sus ejecutores de prueba están en infraestructura separada, como una flota de Kubernetes persistente con la que su trabajo de CI no puede compartir un sistema de archivos, cambie la escritura de archivo en el hook Stop por un POST a un endpoint que su controlador escuche:

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

En el lado del controlador, ejecute cualquier cosa que acepte el POST y mantenga la respuesta hasta que la prueba la solicite, como un pequeño oyente HTTP dentro del trabajo de CI o un receptor de webhook que ya ejecuta. El hook se ejecuta en su infraestructura, por lo que el endpoint solo necesita ser accesible desde sus ejecutores.

<h2 id="authenticate-from-ci">
  Autentíquese desde CI
</h2>

Tanto `claude -p ... --environment` como `claude -p ... --cloud` se autentican con un token OAuth de claude.ai; las claves API, como `sk-ant-xxxxx`, no se aceptan para ninguna de las dos llamadas. Dos enfoques hacen que un token esté disponible en CI.

<h3 id="long-lived-ci-host">
  Host de CI de larga duración
</h3>

Ejecute `claude auth login` una vez de forma interactiva en la máquina que ejecuta el script, usando una cuenta de usuario dedicada para automatización. Claude Code almacena el token en el llavero del SO en macOS, o en `~/.claude/.credentials.json` en Linux y Windows. En un host macOS cuyo Keychain no se puede escribir, como es típico en una sesión SSH donde el Keychain de inicio de sesión permanece bloqueado, Claude Code almacena el token en `~/.claude/.credentials.json` también. Consulte [Gestión de credenciales](/docs/es/authentication#credential-management).

La CLI actualiza automáticamente el token de acceso de corta duración en cada invocación, pero la concesión de token de actualización subyacente está limitada a 30 días desde el inicio de sesión inicial, así que vuelva a ejecutar `claude auth login` de forma interactiva en ese host cada 30 días.

<h3 id="ephemeral-ci-runners">
  Ejecutores de CI efímeros
</h3>

No hay un token de CI de larga duración para esto hoy. El alcance que otorga control de sesión remota, `user:sessions:claude_code`, está limitado en el servidor a 30 días, por lo que `claude setup-token`, que acuña un token de solo inferencia de un año, no lo cubre. El [secreto del entorno](/docs/es/self-hosted-environments-quickstart#set-up-an-environment-and-runner) tampoco se acepta, ya que solo autoriza a un ejecutor a registrarse con el entorno, no a crear sesiones.

Para aprovisionar un inicio de sesión almacenado en un ejecutor efímero, configure [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` y `CLAUDE_CODE_OAUTH_SCOPES`](/docs/es/env-vars#variables) para que `claude auth login` intercambie el token sin un navegador; el mismo límite de 30 días se aplica a la concesión de actualización. Póngase en contacto con su equipo de cuenta de Anthropic si necesita una ruta de identidad de máquina que no esté vinculada a una cuenta humana.

<h2 id="create-a-dedicated-test-environment">
  Cree un entorno de prueba dedicado
</h2>

Cree y elimine entornos mediante programación para que cada ejecución de CI obtenga uno limpio; el ejecutor que su trabajo de CI inicia se registra en el entorno nuevo. Las llamadas de creación y eliminación a continuación son los mismos endpoints que la página de administración **Cloud environments** en claude.ai usa, y requieren el encabezado `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Acuñe el token de administrador
</h3>

`$ADMIN_TOKEN` es un token de acceso OAuth de claude.ai para una cuenta que tiene un rol de Propietario, acuñado de la misma manera que [Autentíquese desde CI](#authenticate-from-ci):

* **Acuñelo**: ejecute `claude auth login` con una cuenta que tenga un rol de Propietario, luego lea el token de acceso actual desde donde [Host de CI de larga duración](#long-lived-ci-host) dice que Claude Code lo almacenó.
* **Léalo fresco en cada ejecución**: la CLI rota el token de acceso, y el mismo límite de concesión de actualización de 30 días se aplica, así que no almacene una copia.
* **Páselo a través de stdin**: como hace el ejemplo, para que el token nunca llegue a la lista de argumentos de curl o su registro de compilación.

<h3 id="create-the-environment">
  Cree el entorno
</h3>

Capture la respuesta sin ecoarla: `pool_secret` es una credencial de larga duración que puede registrar ejecutores en el entorno, así que guárdela como un secreto de CI enmascarado e imprima solo el ID del entorno. La forma `-H @-` que mantiene el token fuera de la lista de procesos requiere curl 7.55 o posterior; curl más antiguo trata `@-` como un encabezado literal y envía la solicitud sin autorización.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Hasta que un [Propietario active **Allow self-hosted environments**](/docs/es/self-hosted-environments#availability-and-limitations) para la organización, la llamada falla con un `403` `permission_error` que lee `self-hosted runners are disabled by your organization's policy`.

Inicie un ejecutor en este host con `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, más el hook de captura y `E2E_REPLY_DIR` por [Instale el hook de captura](#install-the-capture-hook-on-your-test-runner), luego ejecute el script de prueba.

<h3 id="delete-the-environment">
  Elimine el entorno
</h3>

Elimine el entorno cuando la ejecución finalice, para que cada ejecución de CI comience limpia:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
