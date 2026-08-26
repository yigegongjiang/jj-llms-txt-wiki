> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar Claude Code a una puerta de enlace LLM

> Apunte Claude Code a la puerta de enlace LLM de su organización. Compruebe si su administrador ya la configuró, o establezca la URL base y las credenciales usted mismo, luego verifique la conexión y corrija los errores de la puerta de enlace.

Una [puerta de enlace LLM](/docs/es/llm-gateway) es un proxy que su organización ejecuta entre Claude Code y el proveedor del modelo. Cuando su organización usa una, Claude Code se autentica en la puerta de enlace con una credencial que su organización emite en lugar de su inicio de sesión personal en claude.ai.

Esta página es para desarrolladores que ejecutan Claude Code a través de una puerta de enlace que opera su organización. Cubre dos caminos: [comprobar si su administrador ya la configuró para usted](#check-for-an-existing-configuration), y [configurarla usted mismo](#configure-claude-code-yourself) cuando no lo haya hecho.

<Note>
  * Para implementar una puerta de enlace para su organización, consulte [Implementar una puerta de enlace LLM](/docs/es/llm-gateway-rollout)
  * Para ver qué envía Claude Code a una puerta de enlace, consulte la [referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Comprobar una configuración existente
</h2>

Los administradores pueden distribuir la dirección de la puerta de enlace y la credencial a través de [configuración administrada](/docs/es/settings#settings-files), administración de dispositivos, o un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), para que Claude Code las recoja al iniciar sin que tenga que configurar nada. Para comprobar si su organización ya lo hizo:

<Steps>
  <Step title="Iniciar Claude Code">
    Ejecute `claude`. Si se abre en la pantalla de inicio de sesión en lugar de una sesión, no se distribuyó ninguna credencial de puerta de enlace; [configúrela usted mismo](#configure-claude-code-yourself) a continuación.
  </Step>

  <Step title="Comprobar la pestaña Estado">
    Si Claude Code inició una sesión sin mostrar la pantalla de inicio de sesión, ejecute `/status`, abra la pestaña **Estado**, y compruebe dos líneas:

    * `Anthropic base URL`: esta línea solo aparece cuando se establece una dirección de puerta de enlace. Si no está ahí, Claude Code no está apuntando a la puerta de enlace; [configúrela usted mismo](#configure-claude-code-yourself) a continuación.
    * `Auth token` o `API key`: una línea que nombre `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, o un `apiKeyHelper` confirma que una credencial de puerta de enlace está activa. Una línea `Login method` que nombre una cuenta de claude.ai en su lugar significa que la credencial no se distribuyó; [establézcala usted mismo](#set-the-credential-variable).
  </Step>

  <Step title="Enviar un mensaje de prueba">
    Cierre el menú `/status` y envíe cualquier solicitud en Claude Code. Una respuesta normal de Claude, sin error, confirma que la conexión de la puerta de enlace funciona.
  </Step>
</Steps>

Si ambas líneas en el menú `/status` se ven bien pero el mensaje a Claude falla, consulte la [tabla de solución de problemas](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Configurar Claude Code usted mismo
</h2>

Para configurar Claude Code para la puerta de enlace usted mismo, necesita de su equipo de puerta de enlace:

* La URL base de la puerta de enlace
* Una credencial: una cadena de clave o token, o un comando que obtenga una
  * Si su equipo de puerta de enlace no dijo qué tipo de credencial es, la sección [variable de credencial](#set-the-credential-variable) a continuación cubre qué intentar

Las secciones a continuación cubren la configuración en orden:

* [Establecer la variable de credencial](#set-the-credential-variable) y [establecer la URL base](#set-the-base-url-and-credential): las dos variables que toda conexión de puerta de enlace necesita
* [Verificar la conexión](#verify-the-connection): confirmar que funciona antes de persistir nada
* [Configurar cada superficie](#configure-each-surface): si está usando una superficie además de la CLI de Claude Code, como VS Code, vea cómo configurarla con sus credenciales de puerta de enlace
* [Configuración adicional](#additional-configuration): variables que algunas puertas de enlace necesitan más allá de la URL base y la credencial, como un encabezado personalizado, un asistente de credencial, descubrimiento de modelos, una URL base en formato de proveedor, o desactivar el tráfico fuera de la ruta de la puerta de enlace. Establezca estos solo si su administrador los nombró o su red restringe la salida

<h3 id="set-the-credential-variable">
  Establecer la variable de credencial
</h3>

Para autenticar Claude Code en la puerta de enlace, establezca su credencial en una variable de entorno. Qué variable depende de lo que su equipo de puerta de enlace le dijo:

| Establecer la credencial en                             | Usar cuando                                                                |
| :------------------------------------------------------ | :------------------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Su equipo de puerta de enlace dijo "bearer token" o "Authorization header" |
| `ANTHROPIC_API_KEY`                                     | Su equipo de puerta de enlace dijo "API key" o "x-api-key"                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | La credencial rota o viene de un almacén                                   |

Si no le dijeron cuál, use `ANTHROPIC_AUTH_TOKEN`; la [solicitud de verificación](#verify-the-connection) a continuación muestra cómo saber si necesita cambiar.

<h3 id="set-the-base-url-and-credential">
  Establecer la URL base y la credencial
</h3>

Establezca la URL base de la puerta de enlace y la variable de credencial que eligió arriba como variables de entorno. Los ejemplos usan `ANTHROPIC_AUTH_TOKEN`; cámbielo por `ANTHROPIC_API_KEY` si esa es [la variable que eligió](#set-the-credential-variable). Puede establecerlos [en su shell](#set-as-shell-environment-variables), que dura una sesión de terminal, o [en un archivo de configuración de Claude Code](#set-in-a-settings-file), que persiste en todas partes donde se ejecuta Claude Code.

Para su primera conexión, comience con exportaciones de shell y ejecute la [solicitud de verificación](#verify-the-connection) antes de mover los valores a un archivo de configuración.

<h4 id="set-as-shell-environment-variables">
  Establecer como variables de entorno de shell
</h4>

Reemplace los valores con los que su equipo de puerta de enlace le dio:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Las exportaciones de shell se aplican solo a esa sesión de terminal y a los programas iniciados desde ella; un editor lanzado desde el dock o el menú Inicio no las verá. Para que persistan en nuevas terminales, agregue las mismas líneas a su perfil de shell, como `~/.zshrc`, `~/.bashrc`, o su `$PROFILE` de PowerShell, o use un archivo de configuración en su lugar.

<h4 id="set-in-a-settings-file">
  Establecer en un archivo de configuración
</h4>

Para que la configuración se aplique en todas partes donde se ejecuta Claude Code sin depender de su shell, establezca las variables en el bloque `env` de un [archivo de configuración](/docs/es/settings). Los archivos de configuración tienen diferentes alcances:

* `~/.claude/settings.json` se aplica a todos sus proyectos. En Windows la ruta es `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` se aplica a un proyecto. Claude Code lo agrega a su gitignore cuando crea el archivo; si lo crea usted mismo, agréguelo a su gitignore manualmente primero para que no cometa accidentalmente su credencial

<Warning>
  No ponga la credencial en el `.claude/settings.json` de un proyecto. Ese archivo se confirma y se comparte con todos los que clonan el repositorio.
</Warning>

El bloque `env` se ve igual en cualquiera de los archivos:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Cuando tanto una exportación de shell como un bloque `env` de archivo de configuración establecen la misma variable, se aplica el valor del archivo de configuración. Ejecute `/status` para ver qué URL base y fuente de credencial está usando Claude Code.

<h3 id="verify-the-connection">
  Verificar la conexión
</h3>

Con las variables exportadas en su shell, envíe una solicitud de un token a la puerta de enlace directamente. Esto confirma que la URL y la credencial funcionan antes de abrir Claude Code, por lo que una falla apunta a la puerta de enlace en lugar de su configuración. Los comandos a continuación leen las variables de shell, por lo que necesitan las [exportaciones de shell](#set-as-shell-environment-variables) incluso si también pone los valores en un archivo de configuración.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Si su puerta de enlace espera claves en el encabezado `x-api-key`, reemplace el encabezado `Authorization` con `x-api-key: $ANTHROPIC_API_KEY` en el comando Bash, o la entrada de tabla hash `"Authorization"` con `"x-api-key" = "$env:ANTHROPIC_API_KEY"` en el comando PowerShell.

Una respuesta JSON que comience con `{"id":"msg_` e incluya un campo `"content":[...]` significa que la puerta de enlace es alcanzable y la credencial funciona. Un error que nombre un modelo desconocido aún prueba que la URL y la credencial funcionan, ya que la puerta de enlace autenticó la solicitud antes de rechazar el nombre del modelo; no necesita encontrar un modelo que su puerta de enlace sirva para esta prueba. Un `401` significa que la credencial fue rechazada: si adivinó la variable, cambie a la otra y re-exporte.

<h4 id="confirm-in-claude-code">
  Confirmar en Claude Code
</h4>

Inicie `claude` desde el mismo shell para que herede las exportaciones, envíe un mensaje, y ejecute `/status`.

En la pestaña **Estado**, la línea `Anthropic base URL` debe mostrar su dirección de puerta de enlace, lo que confirma que las solicitudes se enrutan allí; si la línea no está ahí, la variable no llegó a la sesión. Una línea `Auth token` o `API key` que nombre la variable que estableció confirma que la credencial de puerta de enlace está activa en lugar de un inicio de sesión de claude.ai guardado.

Si el mensaje falla, o `/status` no muestra la URL de la puerta de enlace, consulte la [tabla de solución de problemas](#troubleshoot-gateway-errors) a continuación.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Cómo la variable de credencial se asigna a un encabezado
</h3>

Cada variable envía la credencial en un encabezado HTTP diferente: `ANTHROPIC_AUTH_TOKEN` en `Authorization: Bearer`, `ANTHROPIC_API_KEY` en `x-api-key`, y `apiKeyHelper` en ambos. Una credencial en la variable incorrecta llega a la puerta de enlace en un encabezado que no lee, y la solicitud falla con `401`. Si la solicitud de verificación devolvió `401`, cambie a la otra variable e intente de nuevo.

<h3 id="conflicts-with-an-existing-login">
  Conflictos con un inicio de sesión existente
</h3>

Una variable de credencial de puerta de enlace tiene precedencia sobre un inicio de sesión de claude.ai guardado o una clave de Consola. Su inicio de sesión de claude.ai permanece guardado y sin usar mientras la variable está establecida; desestablezca la variable y Claude Code vuelve a ella. Con `ANTHROPIC_AUTH_TOKEN`, la variable tiene precedencia inmediatamente. Con `ANTHROPIC_API_KEY`, se le solicita una vez en modo interactivo para aprobar la clave antes de que tome el control.

Ejecute `/status` para confirmar qué fuente de credencial está activa. Si el inicio muestra una advertencia de conflicto de autenticación que nombra dos fuentes, consulte la primera fila de la [tabla de solución de problemas](#troubleshoot-gateway-errors) para saber cuál descartar. Para borrar un inicio de sesión guardado para que solo permanezca la credencial de puerta de enlace, ejecute `/logout`.

<h2 id="configure-each-surface">
  Configurar cada superficie
</h2>

La CLI lee las variables de entorno y archivos de configuración anteriores. Las otras superficies son la extensión de VS Code, la aplicación de escritorio, GitHub Actions, el Agent SDK, y las superficies en la nube como Slack y la web; las secciones a continuación cubren si esa configuración llega a cada una.

<h3 id="vs-code-extension">
  Extensión de VS Code
</h3>

Establezca las variables de puerta de enlace para la [extensión de VS Code](/docs/es/vs-code) en `claudeCode.environmentVariables`, en la configuración de usuario propia de VS Code abierta con el comando **Preferences: Open User Settings (JSON)**. La extensión comprueba las credenciales de esta configuración antes de lanzarse, por lo que es el lugar confiable para la credencial de puerta de enlace; los valores en `~/.claude/settings.json` llegan al proceso generado pero no a la comprobación de inicio de sesión propia de la extensión.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Aplicación de escritorio
</h3>

La aplicación de escritorio lee el enrutamiento de puerta de enlace de su [configuración de inferencia de terceros](https://claude.com/docs/third-party/claude-desktop/gateway), no de `ANTHROPIC_BASE_URL` o `settings.json`. Esa configuración puede provenir de su organización o de un formulario en la aplicación misma:

* **Distribuida por un administrador**: si su organización ha [implementado la configuración](/docs/es/llm-gateway-rollout#distribute-through-managed-settings), la aplicación de escritorio se enruta a través de la puerta de enlace sin configuración de su parte
* **Configurada localmente**: para dispositivos sin una configuración distribuida por administrador, abra Help → Troubleshooting → Enable Developer Mode, que reinicia la aplicación con un menú Developer. Luego abra Developer → Configure Third-Party Inference e ingrese su URL base de puerta de enlace. Una configuración distribuida por administrador tiene prioridad y hace que este formulario sea de solo lectura

Con la configuración de puerta de enlace activa, la aplicación de escritorio ejecuta sesiones solo en su máquina local: el selector de entorno no ofrece sesiones SSH ni entornos en la nube alojados por Anthropic, y [Control Remoto](/docs/es/remote-control) no está disponible. Para usar Claude Code en un host remoto a través de la puerta de enlace, ejecute la CLI en ese host con [`ANTHROPIC_BASE_URL` y la credencial de puerta de enlace](#set-the-base-url-and-credential) establecidas allí.

Si la aplicación de escritorio muestra `Gateway was unreachable`, la aplicación no pudo alcanzar la URL base configurada al iniciar; compruebe la URL y la ruta de red con la [prueba de curl anterior](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/es/github-actions) lee `ANTHROPIC_BASE_URL` y `ANTHROPIC_CUSTOM_HEADERS` del bloque `env` del flujo de trabajo. Pase la credencial como la entrada `anthropic_api_key` de la acción; la acción la establece como `ANTHROPIC_API_KEY`, por lo que llega a la puerta de enlace en el encabezado `x-api-key`.

Para una puerta de enlace `x-api-key`, establezca la URL base en `env` y pase la clave de puerta de enlace como entrada:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Para una puerta de enlace de token portador, pase el mismo secreto dos veces: como la entrada `anthropic_api_key` y como `ANTHROPIC_AUTH_TOKEN` en el bloque `env` del flujo de trabajo. La acción requiere `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN`, o federación de identidad de carga de trabajo antes de lanzar Claude Code, y no lee `ANTHROPIC_AUTH_TOKEN`, por lo que la entrada está ahí solo para satisfacer esa comprobación de lanzamiento. La variable de entorno es lo que pone la clave en el encabezado `Authorization` que la puerta de enlace lee; la copia en `x-api-key` se ignora:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Para las otras opciones de autenticación de la acción, incluidas `CLAUDE_CODE_OAUTH_TOKEN` y federación de identidad de carga de trabajo, consulte [Claude Code GitHub Actions](/docs/es/github-actions) y el [README](https://github.com/anthropics/claude-code-action#readme) de la acción.

<h3 id="agent-sdk">
  Agent SDK
</h3>

El [Agent SDK](/docs/es/agent-sdk/overview) no tiene opciones específicas de puerta de enlace; pasa variables de entorno al proceso de Claude Code que genera. Cada SDK acepta una opción `env` que establece el entorno del proceso generado, y los SDK de TypeScript y Python lo tratan de manera diferente:

* TypeScript: el proceso generado hereda el entorno principal de forma predeterminada, pero establecer `options.env` reemplaza el entorno completamente. Extienda `process.env` en él para mantener sus variables de puerta de enlace.
* Python: `ClaudeAgentOptions(env=...)` se fusiona en el entorno heredado, por lo que las variables de puerta de enlace establecidas en el proceso principal se transmiten sin extender.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-web-and-remote-control">
  Slack, web y Control Remoto
</h3>

[Claude Code en Slack](/docs/es/slack) y [Claude Code en la web](/docs/es/claude-code-on-the-web) son productos alojados por Anthropic que siempre usan la API de Anthropic; no son parte de una implementación de puerta de enlace. Las variables de puerta de enlace establecidas en la configuración de entorno de una sesión en la nube no se aplican. Si su tráfico debe permanecer en la puerta de enlace, no habilite estas superficies para esos usuarios.

[Control Remoto](/docs/es/remote-control) y [dictado de voz](/docs/es/voice-dictation) ambos se basan en una identidad de claude.ai: Control Remoto para emparejar una sesión en vivo con su cuenta, y dictado de voz para alcanzar el punto final de transcripción de claude.ai. No están disponibles mientras `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o un `apiKeyHelper` está activo. A partir de v2.1.196, Control Remoto también está deshabilitado mientras `ANTHROPIC_BASE_URL` apunta a un host que no es de Anthropic, por lo que iniciar sesión con claude.ai no es suficiente por sí solo.

Para restaurar cualquiera de estas características, inicie sesión con claude.ai y desestablezca las variables de puerta de enlace que comprueba. La sección Control Remoto de `claude doctor` nombra la variable de credencial a desestablecer.

* Dictado de voz: desestablezca la credencial de puerta de enlace
* Control Remoto: desestablezca la credencial de puerta de enlace y `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Configuración adicional
</h2>

Estas configuraciones cubren casos más allá de la URL base y la credencial. Establézcalas solo si las instrucciones de su administrador, las reglas de salida de su red, o la [tabla de solución de problemas](#troubleshoot-gateway-errors) llaman a una.

<h3 id="send-additional-headers">
  Enviar encabezados adicionales
</h3>

Algunas puertas de enlace enrutan o etiquetan solicitudes usando un encabezado personalizado además de la credencial, por ejemplo un identificador de inquilino o una clave de enrutamiento. Para enviar uno, establezca [`ANTHROPIC_CUSTOM_HEADERS`](/docs/es/env-vars) con un par `Name: Value` por línea. El ejemplo a continuación agrega un encabezado de enrutamiento llamado `X-Org-Route`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

También puede establecer `ANTHROPIC_CUSTOM_HEADERS` en el bloque `env` de un archivo de configuración. Use `\n` entre pares allí, ya que las cadenas JSON no pueden abarcar múltiples líneas:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

<h3 id="add-gateway-models-to-the-model-picker">
  Agregar modelos de puerta de enlace al selector de modelos
</h3>

El descubrimiento de modelos consulta la puerta de enlace para su lista de modelos al iniciar y agrega esos nombres al selector `/model` junto con las entradas integradas.

Habilítelo si su puerta de enlace sirve nombres de modelos que no están en la lista integrada de Claude Code y desea seleccionarlos del selector. Si los modelos integrados son los que usa, no necesita descubrimiento; su administrador también puede haberlo habilitado ya a través de configuración administrada.

Para habilitarlo, establezca `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` en su shell o en el bloque `env` de `~/.claude/settings.json`. El descubrimiento requiere Claude Code v2.1.129 o posterior.&#x20;

Los modelos descubiertos aparecen como entradas `/model` adicionales etiquetadas como `From gateway`. Para confirmar que el descubrimiento se ejecutó, inicie `claude --debug` y busque las líneas `[gatewayDiscovery]`: un éxito registra cuántos modelos se almacenaron en caché, y un `404`, tiempo de espera, o redirección se registra allí también. Para cuándo se ejecuta el descubrimiento, qué filtra, y el formato de respuesta que las puertas de enlace sirven, consulte la [referencia de descubrimiento de modelos](/docs/es/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Rotar credenciales con apiKeyHelper
</h3>

Un `apiKeyHelper` es un comando que Claude Code ejecuta para obtener su credencial de puerta de enlace, en lugar de leerla de una variable de entorno estática.

Use un asistente cuando la credencial expira en un cronograma, viene de un comando de almacén o SSO, o su administrador le dijo que configure uno. Si su credencial es una cadena fija que establece una vez, la [variable de credencial](#set-the-credential-variable) es todo lo que necesita y puede omitir esta sección.

El asistente es cualquier comando de shell que imprime la credencial actual en stdout. Claude Code lo ejecuta a través de su shell del sistema, por lo que en Windows puede ser un ejecutable o una invocación de PowerShell. Escriba el script, hágalo ejecutable, y haga referencia a él desde `apiKeyHelper` en su [archivo de configuración](/docs/es/settings):

<Tabs>
  <Tab title="Bash o Zsh">
    Por ejemplo, un script que lee desde un almacén:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Haga referencia a su ruta en `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Por ejemplo, un script que lee desde un almacén:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Haga referencia a la invocación de PowerShell en `%USERPROFILE%\.claude\settings.json`, escapando las barras invertidas en la cadena JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code almacena en caché la salida del asistente durante cinco minutos de forma predeterminada y lo re-ejecuta cuando una solicitud devuelve HTTP 401. Para cambiar la duración del caché, establezca `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` en milisegundos, por ejemplo `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` para 15 minutos.

El valor del asistente se envía en los encabezados `Authorization` y `x-api-key`, por lo que funciona cualquiera que sea el encabezado que su puerta de enlace lea.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Desactivar tráfico fuera de la ruta de la puerta de enlace
</h3>

La puerta de enlace lleva solicitudes de modelo, pero Claude Code también envía tráfico de fondo no esencial fuera de la ruta de la puerta de enlace, a Anthropic y a servicios de terceros como GitHub: comprobaciones de versión, telemetría, informes de errores, notas de lanzamiento, y solicitudes similares. En una red que solo permite salida a la puerta de enlace, estas solicitudes fallan y pueden aparecer como conexiones bloqueadas en su monitoreo de salida.

Para desactivar ese tráfico, establezca `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` junto con las variables de puerta de enlace, en las mismas exportaciones de shell o bloque `env` del archivo de configuración:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Establecer la variable tiene estos efectos y limitaciones:

* Desactiva las actualizaciones automáticas, por lo que planifique otra ruta de actualización, como su gestor de paquetes o distribución administrada.
* Suprime la comprobación de disponibilidad del [modo rápido](/docs/es/fast-mode). A menos que una comprobación anterior ya haya habilitado el modo rápido en la máquina, `/fast` reporta que el modo rápido no está disponible.
* Desactiva el [descubrimiento de modelos de puerta de enlace](#add-gateway-models-to-the-model-picker), aunque el descubrimiento consulta la puerta de enlace en sí. Los modelos descubiertos previamente permanecen disponibles desde el caché local, pero la lista no se actualiza.
* La comprobación de seguridad de dominio de la herramienta WebFetch no se ve afectada y aún llama a `api.anthropic.com`. Desactívela por separado con `skipWebFetchPreflight: true` en [configuración](/docs/es/settings) si su red bloquea ese host.
* Para cada flujo de telemetría y la variable que lo controla, consulte [servicios de telemetría](/docs/es/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Enrutar a un proveedor en la nube a través de una puerta de enlace
</h3>

Estas configuraciones apuntan Claude Code a una puerta de enlace a través de una variable de URL base específica del proveedor en lugar de `ANTHROPIC_BASE_URL`. Las puertas de enlace de Amazon Bedrock y Google Cloud's Agent Platform aceptan los formatos de solicitud nativos de esos proveedores; las puertas de enlace de Microsoft Foundry y Claude Platform en AWS aceptan el formato de Mensajes de Anthropic y difieren solo en qué variable de URL base las alcanza.

Use una solo si su equipo de puerta de enlace nombró específicamente Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform en AWS. Si la [solicitud de verificación](#verify-the-connection) anterior devolvió JSON, puede omitir esta sección.

Establezca el bloque para el proveedor que su equipo de puerta de enlace nombró. Las variables de omitir autenticación le dicen a Claude Code que no firme solicitudes con credenciales de proveedor, ya que la puerta de enlace tiene esas. Si la puerta de enlace necesita su propio token, agregue `ANTHROPIC_AUTH_TOKEN` después del bloque, excepto para Microsoft Foundry, que usa `ANTHROPIC_FOUNDRY_API_KEY` como se muestra. Una puerta de enlace de Microsoft Foundry que espera un token de portador puede usar [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/es/env-vars) en su lugar; tiene precedencia sobre `ANTHROPIC_FOUNDRY_API_KEY` cuando ambos están establecidos. `ANTHROPIC_FOUNDRY_AUTH_TOKEN` requiere Claude Code v2.1.203 o posterior.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Ponga la credencial de la puerta de enlace en `ANTHROPIC_FOUNDRY_API_KEY`; se envía a la puerta de enlace como el encabezado `x-api-key`. Una puerta de enlace que espera un token de portador puede tomar [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/es/env-vars) en su lugar. Claude Code envía ese valor como el encabezado `Authorization: Bearer`, y tiene precedencia sobre `ANTHROPIC_FOUNDRY_API_KEY` cuando ambos están establecidos. Requiere Claude Code v2.1.203 o posterior.

Para una puerta de enlace que inyecta su propio encabezado `Authorization`, establezca `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` y deje ambas variables de credencial sin establecer. Claude Code entonces envía solicitudes sin una credencial de Azure y preserva el encabezado `Authorization` que suministra, por ejemplo a través de `ANTHROPIC_CUSTOM_HEADERS`. Antes de v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` sin una clave de API dejaba el cliente de Microsoft Foundry incapaz de enviar solicitudes.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform en AWS
</h4>

Consulte [Claude Platform en AWS](/docs/es/claude-platform-on-aws) para el ID del espacio de trabajo.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h2 id="troubleshoot-gateway-errors">
  Solucionar problemas de errores de puerta de enlace
</h2>

Estos son los errores más comunes al ejecutar Claude Code a través de una puerta de enlace, con la causa del lado de la puerta de enlace y la solución:

| Error                                                                                                                                                                                                                                | Causa                                                                                                                                                                                                                                                                                                                                                                               | Solución                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una advertencia de inicio que nombra dos fuentes de credencial y termina en `auth may not work as expected`. Las versiones más antiguas muestran `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` en su lugar. | Una credencial de puerta de enlace y un inicio de sesión guardado están ambos activos; la variable se usa para solicitudes, pero el inicio de sesión obsoleto puede causar comportamiento de autenticación inesperado                                                                                                                                                               | Desestablezca la variable para usar el inicio de sesión guardado, o ejecute `/logout` para usar la credencial de puerta de enlace                                                                                                                                                                                                                                                                                                                                                              |
| Errores `401` que nombran un token inválido o no reconocido                                                                                                                                                                          | La credencial no es una que la puerta de enlace emitió, o está en un encabezado que la puerta de enlace no lee                                                                                                                                                                                                                                                                      | Confirme que la variable coincida con su tipo de credencial en la [tabla de credencial](#set-the-credential-variable), y regenere la clave en la puerta de enlace si fue revocada                                                                                                                                                                                                                                                                                                              |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                | El comando en la configuración [`apiKeyHelper`](/docs/es/settings#available-settings) salió con un error, agotó el tiempo de espera, o no imprimió nada, por lo que las solicitudes llevan una clave de marcador de posición                                                                                                                                                             | Ejecute el comando directamente para ver por qué falla, y vuelva a autenticarse con su proveedor de credenciales si reporta una sesión expirada; consulte [la referencia de errores](/docs/es/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                                                                           |
| `Unable to connect to API (ConnectionRefused)`, o `(ECONNREFUSED)` de instalaciones de npm, a menudo después de una pausa silenciosa mientras Claude Code [reintenta con retroceso](/docs/es/errors#automatic-retries)                    | Nada respondió en la URL base: la dirección es incorrecta, o una VPN o firewall bloquea la ruta a la puerta de enlace                                                                                                                                                                                                                                                               | Ejecute la [prueba de curl anterior](#verify-the-connection), que falla inmediatamente con la misma causa, y confirme la URL y la ruta de red con su equipo de puerta de enlace                                                                                                                                                                                                                                                                                                                |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                             | La puerta de enlace o un proxy intermedio devolvió una respuesta que no es de API, a menudo una página de error HTML o de inicio de sesión                                                                                                                                                                                                                                          | Pruebe con la [solicitud de curl anterior](#verify-the-connection); corrija la ruta de puerta de enlace que devuelve JSON no válido                                                                                                                                                                                                                                                                                                                                                            |
| Errores `400` que nombran `context_management`, `Extra inputs are not permitted`, u otros campos no reconocidos                                                                                                                      | La puerta de enlace reenvía solicitudes a un upstream que rechaza campos que Claude Code envía a puntos finales en formato de Anthropic                                                                                                                                                                                                                                             | Establezca `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, que suprime la mayoría de campos de pre-lanzamiento; consulte [paso de características](/docs/es/llm-gateway-protocol#feature-pass-through). Algunos betas no están controlados por esta bandera; para esos, establezca la variable de proveedor `CLAUDE_CODE_USE_*` coincidente para que Claude Code envíe solo lo que ese proveedor acepta                                                                                                 |
| Errores `400` que nombran `thinking` o `adaptive`, como `Input tag 'adaptive' found`                                                                                                                                                 | La compilación del modelo upstream no acepta razonamiento adaptativo, que Claude Code solicita para modelos Claude 4.6 y posteriores                                                                                                                                                                                                                                                | Actualice el upstream de la puerta de enlace. En Opus 4.6 y Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` funciona en su lugar. Las variables de capacidad de [configuración de modelo](/docs/es/model-config) se aplican solo a las configuraciones de proveedor, como `CLAUDE_CODE_USE_BEDROCK` y `CLAUDE_CODE_USE_VERTEX`, no detrás de una puerta de enlace `ANTHROPIC_BASE_URL`                                                                                                        |
| Errores `400` que indican un contexto o límite de token en las propias palabras de la puerta de enlace, como `ContextWindowExceededError` o `prompt token count of N exceeds the limit of M`                                         | La puerta de enlace aplica un contexto más pequeño que la ventana nativa del modelo y reescribe el error upstream, por lo que el compacto automático y reintento, que coincide con la redacción `prompt is too long` de Anthropic, no se dispara                                                                                                                                    | Ejecute `/compact` para recuperar la sesión. Para prevenirlo, establezca `CLAUDE_CODE_AUTO_COMPACT_WINDOW` al límite de la puerta de enlace; el valor se fija a al menos 100,000 tokens y como máximo la ventana de contexto del modelo, por lo que un límite de puerta de enlace por debajo de 100,000 no puede coincidir y `/compact` permanece como la recuperación allí. También establezca `CLAUDE_CODE_MAX_OUTPUT_TOKENS` por debajo del límite de salida del modelo de puerta de enlace |
| Modelos faltantes del selector `/model`                                                                                                                                                                                              | Los nombres de modelos de puerta de enlace no están en la lista integrada de Claude Code                                                                                                                                                                                                                                                                                            | Habilite [descubrimiento de modelos de puerta de enlace](#add-gateway-models-to-the-model-picker) o agregue nombres con las variables de [configuración de modelo](/docs/es/model-config)                                                                                                                                                                                                                                                                                                           |
| Claude Code le pide que inicie sesión aunque la [prueba de curl](#verify-the-connection) tenga éxito                                                                                                                                 | La CLI no tiene credencial propia: una URL base alcanzable no es una, y un bloque `env` en el `.claude/settings.json` o `.claude/settings.local.json` de un proyecto se aplica solo después del asistente de primera ejecución y la solicitud de confianza                                                                                                                          | Establezca `ANTHROPIC_AUTH_TOKEN` en algún lugar que Claude Code lea antes de la configuración de primera ejecución: una exportación de shell, el bloque `env` en `~/.claude/settings.json`, o configuración administrada                                                                                                                                                                                                                                                                      |
| `ANTHROPIC_API_KEY` está establecido pero ignorado, sin solicitud                                                                                                                                                                    | La clave necesita una aprobación única en sesiones interactivas, y una clave previamente rechazada se ignora sin preguntar de nuevo                                                                                                                                                                                                                                                 | Habilítela bajo `/config` con la opción `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `This machine's managed settings require a first-party login`                                                                                                                                                                        | La configuración administrada incluye `forceLoginMethod` o `forceLoginOrgUUID`, que en Claude Code v2.1.146 y posterior no pueden coexistir con `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper`                                                                                                                                                                       | Su administrador debe eliminar `forceLoginMethod` y `forceLoginOrgUUID` de la configuración administrada para usar credenciales de puerta de enlace, o eliminar la credencial de puerta de enlace para usar inicio de sesión de primera parte. Los dos no pueden combinarse                                                                                                                                                                                                                    |
| `403` con un cuerpo HTML como `403 Forbidden`, cuando los registros propios de la puerta de enlace no muestran ninguna solicitud recibida                                                                                            | Un firewall de aplicación web o proxy inverso frente a la puerta de enlace bloqueó el cuerpo de la solicitud antes de que llegara a la puerta de enlace. Los avisos de Claude Code incluyen etiquetas de estilo XML y código fuente que coinciden con reglas de cuerpo de secuencias de comandos entre sitios, por lo que una prueba de curl corta pasa mientras una sesión real no | Exima la ruta `/v1/messages` de la puerta de enlace de la inspección del cuerpo de la solicitud. En AWS WAF esta es la regla administrada `CrossSiteScripting_Body`; en nginx con ModSecurity es la regla de cuerpo OWASP CRS equivalente                                                                                                                                                                                                                                                      |
| Errores de certificado o TLS como `SSL certificate verification failed` o `Self-signed certificate detected`, cuando la [prueba de curl](#verify-the-connection) tiene éxito                                                         | El tiempo de ejecución de Claude Code no está confiando en la misma autoridad de certificación que `curl` usa. Común detrás de proxies de inspección TLS corporativos                                                                                                                                                                                                               | Establezca `NODE_EXTRA_CA_CERTS` a la ruta del paquete de CA; consulte [almacén de certificados de CA](/docs/es/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                                                                |

Si Claude Code le solicita que inicie sesión repetidamente después de eliminar la configuración de puerta de enlace, la causa es generalmente almacenamiento de credenciales en lugar de la puerta de enlace; consulte [errores de autenticación](/docs/es/errors#authentication-errors).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Descripción general de puertas de enlace LLM](/docs/es/llm-gateway): qué es una puerta de enlace y cómo interactúa con las suscripciones de claude.ai
* [Implementar una puerta de enlace LLM para su organización](/docs/es/llm-gateway-rollout): la lista de verificación orientada al administrador para implementar y distribuir la configuración de puerta de enlace
* [Referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol): qué envía Claude Code a una puerta de enlace, incluidos los encabezados y campos que la puerta de enlace debe reenviar
* [Configuración](/docs/es/settings): dónde viven los archivos de configuración y cómo se lee el bloque `env`
* [Autenticación](/docs/es/authentication): cómo interactúan las variables de credencial, `apiKeyHelper`, e inicio de sesión OAuth
