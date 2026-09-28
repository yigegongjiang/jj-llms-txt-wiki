> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implementar una puerta de enlace LLM para su organización

> Implemente un producto de puerta de enlace para Claude Code: configúrelo para reenviar lo que Claude Code envía, emita credenciales de desarrollador, distribuya la configuración a través de ajustes administrados y verifique la implementación.

Esta página guía a un administrador a través de la implementación de una puerta de enlace LLM para Claude Code. Asume que tiene una puerta de enlace implementada que cumple con los [requisitos de la puerta de enlace](#gateway-requirements). La implementación u operación de ningún producto específico se cubre aquí; implemente el suyo siguiendo la documentación del proveedor.

<Note>
  * Para conectar Claude Code en su propia máquina a una puerta de enlace existente, consulte [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect)
  * Para saber qué Claude Code envía a una puerta de enlace y qué reenviar, consulte la [guía de compatibilidad de puerta de enlace](/docs/es/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Requisitos previos
</h2>

Para completar la implementación, necesitará:

* Una puerta de enlace implementada en su infraestructura, sirviendo HTTPS en la dirección exacta que distribuirá a los desarrolladores, no una dirección que redirija a ella, y configurada para enrutar nombres de modelos Claude a su proveedor
* Una credencial de proveedor para que la puerta de enlace reenvíe con:
  * Para la API de Anthropic: una clave API de la [Consola Claude](https://platform.claude.com/settings/keys)
  * Para un proveedor en la nube: credenciales en la nube con acceso a modelos. Consulte los requisitos previos en la página [Amazon Bedrock](/docs/es/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai#prerequisites) o [Microsoft Foundry](/docs/es/microsoft-foundry#prerequisites)
* Una forma de entregar archivos de configuración a máquinas de desarrolladores, como MDM o gestión de configuración
  * Si aún no tiene una, [cómo llegan las configuraciones a los dispositivos](/docs/es/admin-setup#decide-how-settings-reach-devices) compara las opciones

<h3 id="gateway-requirements">
  Requisitos de la puerta de enlace
</h3>

Cualquiera que sea el producto que proporcione la puerta de enlace, debe:

* **Aceptar un formato de API compatible**: uno de los formatos en la [tabla de formatos de API](/docs/es/llm-gateway-protocol#api-formats). Los pasos de implementación a continuación asumen la API de Mensajes de Anthropic en `POST /v1/messages`, que la mayoría de puertas de enlace sirven
* **Transmitir respuestas**: pasar eventos enviados por el servidor a medida que llegan, incluidos los pings de mantenimiento de conexión, en lugar de almacenar en búfer la respuesta completa; [transmisión](/docs/es/llm-gateway-protocol#streaming) cubre qué almacenamiento en búfer o pings eliminados rompen
* **Enrutar nombres de modelos Claude**: asignar cada nombre que los desarrolladores usan a un modelo ascendente. Claude Code envía un nombre de modelo como `claude-sonnet-4-6` en cada solicitud; en la mayoría de productos de puerta de enlace la asignación es una lista de modelos o tabla de enrutamiento en la configuración propia de la puerta de enlace
* **Reenviar encabezados y cuerpo sin cambios**: pasar `anthropic-beta`, `anthropic-version` y el cuerpo de la solicitud en ambas direcciones; la [tabla de paso de características](/docs/es/llm-gateway-protocol#feature-pass-through) asigna cada uno a la característica que se rompe sin él
* **Devolver errores ascendentes sin modificar**: la recuperación automática de Claude Code coincide con la redacción del error, por lo que envolver errores en el sobre propio de la puerta de enlace lo rompe, a menos que el mensaje del sobre lleve uno de los tokens `capability_rejected:` que [una puerta de enlace de aplicaciones Claude sustituye por la redacción de errores de proveedores en la nube](/docs/es/claude-apps-gateway-config#upstream-error-messages)
* **Eximir la ruta de la inspección WAF del cuerpo de la solicitud**: los indicadores de Claude Code llevan código fuente y etiquetas de estilo XML que coinciden con reglas de cuerpo de secuencias de comandos entre sitios; un WAF frente a la puerta de enlace devuelve `403` en sesiones reales mientras que las solicitudes de prueba cortas pasan

Opcionalmente, sirva `GET /v1/models` para que Claude Code pueda rellenar el selector de modelos desde su puerta de enlace con [descubrimiento de modelos](/docs/es/llm-gateway-protocol#model-discovery).

<h2 id="rollout-steps">
  Pasos de implementación
</h2>

La implementación toma cinco pasos, cada uno con un punto de control:

1. [Confirmar que la puerta de enlace enruta sus modelos](#confirm-the-gateway-routes-your-models)
2. [Emitir a cada desarrollador una credencial](#issue-developer-credentials)
3. [Probar Claude Code contra la puerta de enlace](#test-claude-code-against-the-gateway)
4. [Distribuir la URL base y las credenciales](#distribute-the-configuration)
5. [Verificar desde una máquina de desarrollador](#verify-the-rollout)

Los pasos implican tres credenciales diferentes, y los puntos de control las nombran por marcador de posición para que pueda saber cuál es la culpable cuando algo falla:

| Credencial                                    | Quién la tiene                                                                                                              | Marcador de posición en puntos de control                                |
| :-------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------- |
| Credencial de proveedor                       | La puerta de enlace, que la reenvía al proveedor ascendente                                                                 | Configurada en la puerta de enlace; nunca aparece en comandos de cliente |
| Credencial administrativa de puerta de enlace | Usted, si su producto de puerta de enlace emite una para su interfaz de administrador o prueba                              | `<gateway-key>`                                                          |
| Clave de desarrollador                        | Cada desarrollador, emitida por la puerta de enlace en [Emitir credenciales de desarrollador](#issue-developer-credentials) | `<developer-key>`                                                        |

<h3 id="confirm-the-gateway-routes-your-models">
  Confirmar que la puerta de enlace enruta sus modelos
</h3>

Su puerta de enlace ya debe estar configurada con su credencial de proveedor, escuchando en su URL base y reenviando solicitudes a la API de su proveedor. Pruebe que la ruta funciona de extremo a extremo con una solicitud mínima, sustituyendo dos valores de su implementación:

* `<gateway-key>` es cualquier credencial que le permita llamar a la puerta de enlace en este momento: una clave administrativa, una clave de prueba, o su propia clave de desarrollador si ya ha emitido una. No todos los productos de puerta de enlace tienen una credencial de administrador separada; si el suyo no la tiene, emítase una clave de desarrollador en [Emitir credenciales de desarrollador](#issue-developer-credentials) primero
* `model` es un nombre de modelo Claude que su puerta de enlace está configurada para enrutar. El ejemplo usa `claude-sonnet-4-6`; sustituya un nombre que haya configurado

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Punto de control**: un `200` con un campo `content` significa que la puerta de enlace alcanzó al proveedor con ese nombre de modelo. Un `404` significa que ese nombre no está enrutado en la puerta de enlace; un `401` del proveedor significa que la credencial de proveedor de la puerta de enlace es incorrecta.

Repita la solicitud una vez por cada nombre de modelo Claude en la configuración de enrutamiento de su puerta de enlace. Un nombre que la puerta de enlace no enruta devuelve `404` a cualquier desarrollador que lo seleccione, así que pruebe cada nombre antes de la implementación.

<Note>
  Evite servir la puerta de enlace detrás de una redirección. Una redirección puede descartar el cuerpo de la solicitud o eliminar el encabezado de credencial en solicitudes de inferencia, y [descubrimiento de modelos](/docs/es/llm-gateway-protocol#model-discovery) trata cualquier redirección como un fallo para que la credencial no pueda filtrarse a un destino de redirección.
</Note>

<h3 id="issue-developer-credentials">
  Emitir credenciales de desarrollador
</h3>

Cada desarrollador necesita su propia clave de puerta de enlace para autenticarse. Cree una credencial por desarrollador en la puerta de enlace, siguiendo la documentación de gestión de credenciales de su producto.

Confirme que una clave recién emitida funciona contra la puerta de enlace con la misma solicitud que [Confirmar que la puerta de enlace enruta sus modelos](#confirm-the-gateway-routes-your-models), reemplazando `<gateway-key>` con la nueva `<developer-key>`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Punto de control**: un `200` con un campo `content` significa que la clave de desarrollador alcanza la puerta de enlace y la puerta de enlace la reenvía. Un `401` aquí, cuando [el paso anterior](#confirm-the-gateway-routes-your-models) tuvo éxito, significa que la clave de desarrollador es incorrecta o aún no ha surtido efecto en la puerta de enlace.

Emitir una clave por desarrollador en lugar de una clave compartida es lo que hace que la atribución de uso por desarrollador y la desvinculación individual funcionen. La variable de entorno que contiene la clave depende de qué encabezado lee la puerta de enlace. Para una puerta de enlace que verifica credenciales en el encabezado `Authorization: Bearer`, los desarrolladores establecen su clave en `ANTHROPIC_AUTH_TOKEN`. Para una puerta de enlace que lee claves del encabezado `x-api-key`, los desarrolladores establecen `ANTHROPIC_API_KEY` en su lugar; la [tabla de credenciales](/docs/es/llm-gateway-connect#set-the-credential-variable) cubre la asignación.

<h3 id="test-claude-code-against-the-gateway">
  Probar Claude Code contra la puerta de enlace
</h3>

Ejecute Claude Code a través de la puerta de enlace usted mismo antes de distribuir nada, usando la misma configuración que la implementación entregará en toda la flota. Escriba estos directamente en una terminal, no en un archivo `.env` o de configuración; duran solo para esta sesión de terminal, así que cerrarla devuelve su máquina a su configuración normal. Use `ANTHROPIC_API_KEY` en lugar de `ANTHROPIC_AUTH_TOKEN` si su puerta de enlace lee el encabezado `x-api-key`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Luego envíe un indicador de una sola vez a través de la puerta de enlace:

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Punto de control**: el indicador devuelve una respuesta, y la solicitud aparece en el registro de la puerta de enlace como un `POST` a la ruta `/v1/messages` con estado `200`. Claude Code añade una cadena de consulta como `?beta=true`, así que coincida en la ruta, no en la URL completa. Dos mensajes de fallo apuntan en direcciones diferentes:

* `Not logged in`: verifique el registro de la puerta de enlace para distinguir las dos causas. Si está vacío, ninguna credencial alcanzó la sesión y ninguna solicitud salió de la máquina; vuelva a ejecutar las exportaciones en el shell que está probando. Si muestra una solicitud rechazada con `x-api-key` en el cuerpo `401`, la puerta de enlace espera claves en ese encabezado en su lugar; cambie a `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` significa que se envió una credencial y fue rechazada, y el registro de la puerta de enlace dice dónde: un `401` que nombra `api.anthropic.com` o el punto final de su proveedor significa que la puerta de enlace alcanzó el ascendente pero su credencial de proveedor fue rechazada, así que la clave de desarrollador funcionó y la credencial de proveedor que la puerta de enlace tiene es incorrecta o un marcador de posición

Una URL base incorrecta o inaccesible produce un síntoma diferente: Claude Code [reintenta la conexión con retroceso](/docs/es/errors#automatic-retries) y puede quedarse sin salida durante varios minutos antes de reportar un error. Si el comando parece colgarse, verifique el registro de la puerta de enlace en lugar de esperar; ninguna solicitud que llegue significa que `ANTHROPIC_BASE_URL` no apunta a la puerta de enlace.

<h3 id="distribute-the-configuration">
  Distribuir la configuración
</h3>

Cada máquina de desarrollador necesita la dirección de la puerta de enlace y una credencial. Puede distribuirlas centralmente a través de [configuración administrada](/docs/es/managed-settings#delivery-mechanisms), para que los desarrolladores no configuren nada, o entregue a los desarrolladores los valores para establecer ellos mismos.

<h4 id="what-to-distribute">
  Qué distribuir
</h4>

El mismo conjunto de variables se aplica cualquiera que sea el camino que elija. La mayoría de implementaciones solo necesitan `ANTHROPIC_BASE_URL` y una credencial; incluya las filas condicionales cuando su configuración de puerta de enlace lo requiera.

| Variable o configuración                                                                                                                                                                                                        | Qué hace                                                                                                                                                                                                                                                 | Incluir cuando                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                            | Envía las solicitudes de API de Claude Code a la puerta de enlace en lugar de `api.anthropic.com`                                                                                                                                                        | Siempre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `apiKeyHelper`, o una credencial en `ANTHROPIC_AUTH_TOKEN` o `ANTHROPIC_API_KEY`                                                                                                                                                | Autentica cada solicitud a la puerta de enlace. El ayudante ejecuta un comando para obtener la clave; las variables contienen una clave estática, enviada como `Authorization: Bearer` y `x-api-key` respectivamente                                     | Siempre; uno de los tres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                      | Añade encabezados HTTP adicionales a cada solicitud de API                                                                                                                                                                                               | Su puerta de enlace requiere un encabezado de inquilino o enrutamiento en cada solicitud                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `CLAUDE_CODE_GATEWAY_HINT_HEADERS`                                                                                                                                                                                              | Envía los [encabezados de sugerencia de puerta de enlace](/docs/es/llm-gateway-protocol#gateway-hint-headers), que clasifican cada solicitud para decisiones de enrutamiento y programación en la puerta de enlace. Requiere Claude Code v2.1.273 o posterior | Su puerta de enlace lee los encabezados de sugerencia                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                    | Consulta `/v1/models` de la puerta de enlace al inicio y añade los nombres devueltos al selector `/model`                                                                                                                                                | Su puerta de enlace sirve `/v1/models` y desea que los selectores de los desarrolladores se rellenen desde ella                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                        | Detiene el envío de Claude Code de encabezados de capacidad de pre-lanzamiento y campos de cuerpo. [Deshabilitar capacidades de pre-lanzamiento](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities) cubre el alcance exacto                      | Su puerta de enlace reenvía a un Amazon Bedrock o Google Cloud's Agent Platform ascendente que rechaza campos beta. Consulte [Requisitos de la puerta de enlace](#gateway-requirements)                                                                                                                                                                                                                                                                                                                                         |
| `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` o `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK`                                                                                                                                            | Restaura [modo rápido](/docs/es/fast-mode) cuando su verificación de disponibilidad, que llama a `api.anthropic.com` directamente en lugar de seguir `ANTHROPIC_BASE_URL`, falla, es interceptada, o se omite por falta de una credencial de Anthropic        | Su organización usa modo rápido, y los desarrolladores se autentican con `ANTHROPIC_AUTH_TOKEN` solo, con una clave emitida por puerta de enlace en `ANTHROPIC_API_KEY` o desde un `apiKeyHelper`, o su red bloquea o intercepta solicitudes directas a `api.anthropic.com`; [usar modo rápido detrás de proxies y puertas de enlace LLM](/docs/es/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) cubre cuál de las dos variables coincide con su configuración                                                            |
| `ANTHROPIC_MODEL` o [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/es/model-config)                                                                                                                                                         | Establece qué nombre de modelo Claude Code solicita para la sesión principal y para el tráfico de fondo                                                                                                                                                  | Su puerta de enlace enruta nombres de modelos que no coinciden con los valores predeterminados de Claude Code, o enruta [funcionalidad de fondo](/docs/es/costs#background-token-usage) a un modelo diferente. Enrute tanto los nombres de anulación como los nombres predeterminados de Claude Code en la puerta de enlace, ya que algunas sub-llamadas pueden solicitar el nombre predeterminado independientemente de la anulación; la [configuración de modelos](/docs/es/model-config) cubre qué modelo usa cada parte de una sesión |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, o `ANTHROPIC_AWS_BASE_URL` con las [variables para ese proveedor](/docs/es/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Apunte Claude Code a la puerta de enlace a través de una URL base específica del proveedor. Amazon Bedrock y Google Cloud's Agent Platform también cambian al formato de solicitud nativo de esos proveedores                                            | Su puerta de enlace está frente a Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o la Plataforma Claude en AWS; consulte [Formatos de API](/docs/es/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                                                                             |

<h4 id="distribute-through-managed-settings">
  Distribuir a través de configuración administrada
</h4>

Entregue las variables a través del bloque `env` de un [archivo de configuración administrada](/docs/es/managed-settings#delivery-mechanisms), impulsado por MDM, política de registro o gestión de configuración:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Añada las variables condicionales de la tabla al mismo bloque `env`. Un `ANTHROPIC_BASE_URL` administrado se aplica y no puede ser anulado por una exportación de shell de un desarrollador, ya que Claude Code lo aplica sobre el entorno del proceso y configuraciones de menor precedencia.

No incluya `forceLoginMethod` o `forceLoginOrgUUID` en la configuración administrada junto con una credencial de puerta de enlace. Cualquiera de las dos claves bloquea `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` y `apiKeyHelper` al inicio, y los desarrolladores no pueden proceder. Ven `This machine's managed settings require a first-party login`, o [`Administrator policy requires a Cloud gateway sign-in`](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in) bajo un valor `"gateway"`.

La entrega de [configuración administrada por servidor](/docs/es/server-managed-settings#platform-availability) requiere una conexión directa a `api.anthropic.com`, por lo que no alcanza sesiones enrutadas por puerta de enlace. Las implementaciones de puerta de enlace usan esta ruta de configuración administrada basada en archivos, que aplica las mismas claves.

Para la credencial, distribuya un comando [`apiKeyHelper`](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper) en el archivo de configuración administrada como se muestra arriba; el comando se autentica en su almacén de secretos como el desarrollador local, por lo que cada máquina recibe su propia clave. Alternativamente, entregue a cada desarrollador su clave a través de su proceso de secretos existente y haga que establezcan `ANTHROPIC_AUTH_TOKEN` ellos mismos.

Algunos entornos necesitan entrega separada:

* La aplicación de escritorio lee el enrutamiento de puerta de enlace desde su configuración de inferencia de terceros, no desde configuración administrada; implemente ese archivo a través de MDM junto con configuración administrada para que las sesiones de escritorio también se enruten a través de la puerta de enlace. Consulte la [documentación de configuración de terceros de escritorio](https://claude.com/docs/third-party/claude-desktop/configuration) y la [documentación de puerta de enlace de escritorio](https://claude.com/docs/third-party/claude-desktop/gateway)
* Los ejecutores de CI necesitan `ANTHROPIC_BASE_URL` y la credencial establecidas en el [entorno del ejecutor](/docs/es/llm-gateway-connect#configure-each-surface)
* WSL en máquinas Windows administradas lee la configuración administrada de Windows solo cuando [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings) es `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Entregue a los desarrolladores los valores para establecer ellos mismos
</h4>

Si no tiene distribución de configuración administrada en su lugar, envíe a cada desarrollador lo que necesita para seguir la [página de conexión](/docs/es/llm-gateway-connect#configure-claude-code-yourself):

* La URL de la puerta de enlace
* Su credencial personal
* **Qué variable poner la credencial en**: `ANTHROPIC_AUTH_TOKEN` para una puerta de enlace de token portador, o `ANTHROPIC_API_KEY` para una puerta de enlace `x-api-key`. Decirle a los desarrolladores cuál es ahorra el ensayo y error descrito en la [página de conexión](/docs/es/llm-gateway-connect#set-the-credential-variable)
* Cualquier variable condicional de la [tabla Qué distribuir](#what-to-distribute), con sus valores

La [página de conexión](/docs/es/llm-gateway-connect#configure-claude-code-yourself) guía a los desarrolladores a través de establecer cada una.

**Punto de control**: en una máquina de desarrollador, `claude` inicia una sesión sin mostrar la pantalla de inicio de sesión, ya que la credencial distribuida satisface la autenticación. Luego ejecute `/status` y abra la pestaña **Status**: la línea `Anthropic base URL` muestra la dirección de la puerta de enlace, y para distribución administrada la línea `Setting sources` incluye configuración administrada. Una pantalla de inicio de sesión, o una línea `Anthropic base URL` faltante, significa que la configuración no alcanzó la máquina.

<h3 id="verify-the-rollout">
  Verificar la implementación
</h3>

Confirme que todo funciona desde una máquina de desarrollador, no el host de la puerta de enlace, para que la prueba cubra la ruta de red que los desarrolladores usan. Envíe una solicitud de transmisión, que verifica el punto final, paso de transmisión y enrutamiento de modelos de una vez:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Debería ver líneas `data:` llegar incrementalmente. La respuesta completa llegando de una vez después de una pausa significa que la puerta de enlace está almacenando en búfer, lo que detiene Claude Code; un `404` significa que el nombre del modelo no está enrutado. Repita por nombre de modelo.

Luego inicie `claude` y envíe un mensaje. Cada síntoma en este paso tiene una causa:

* Un indicador de inicio de sesión significa una brecha de credencial. Ejecute `/status` y abra la pestaña **Status**: cuando la línea `Setting sources` no incluye configuración administrada, la distribución no alcanzó la máquina; cuando lo hace, la credencial de desarrollador no fue entregada, así que establezca `ANTHROPIC_AUTH_TOKEN` o el `apiKeyHelper`
* Los errores `Failed to authenticate` significan que la puerta de enlace está rechazando solicitudes; su registro dice qué credencial falló. Un rechazo que la puerta de enlace registra a sí misma nombra la clave de desarrollador, mientras que un `401` de `api.anthropic.com` o el punto final de su proveedor significa que la credencial de proveedor que la puerta de enlace tiene fue rechazada
* Un indicador de aprobación de una sola vez para la clave es esperado en el primer uso cuando la puerta de enlace espera claves en el encabezado `x-api-key`, establecidas como `ANTHROPIC_API_KEY`. Con `ANTHROPIC_AUTH_TOKEN`, no aparece ningún indicador y la variable toma el control silenciosamente; un inicio de sesión de claude.ai previamente guardado está inactivo para esa sesión

Si su organización usa [modo rápido](/docs/es/fast-mode), ejecute `/fast` aquí también: la verificación de disponibilidad llama a `api.anthropic.com` directamente en lugar de seguir la URL base de la puerta de enlace, así que una sesión enrutada por puerta de enlace puede reportar modo rápido como no disponible o deshabilitado aunque la inferencia funcione. [Usar modo rápido detrás de proxies y puertas de enlace LLM](/docs/es/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) asigna cada mensaje a la variable que lo restaura, distribuida con [el resto de la configuración](#distribute-the-configuration).

Finalmente, verifique los registros de la puerta de enlace para el mensaje que envió: la credencial identifica al desarrollador, y el [encabezado `x-claude-code-session-id`](/docs/es/llm-gateway-protocol#request-headers) agrupa solicitudes por sesión. Si las características fallan con los [síntomas de solución de problemas](/docs/es/llm-gateway-connect#troubleshoot-gateway-errors), la puerta de enlace está eliminando encabezados o reescribiendo errores; consulte los [requisitos de la puerta de enlace](#gateway-requirements) arriba.

<h2 id="maintain-the-gateway">
  Mantener la puerta de enlace
</h2>

Después de la implementación, tres tipos de cambios llegan a la puerta de enlace con el tiempo. Cada uno tiene un síntoma a vigilar y una acción a tomar.

| Cambio                                                                                                 | Síntoma cuando la puerta de enlace no se ha puesto al día                                                                                                                                  | Acción                                                                                                                                                                                                                                                                                                                                  |
| :----------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Los nuevos lanzamientos de Claude Code añaden valores `anthropic-beta` y campos de cuerpo de solicitud | Los desarrolladores reportan errores `400` que nombran un nuevo campo después de actualizar Claude Code; consulte [paso de características](/docs/es/llm-gateway-protocol#feature-pass-through) | Reenvíe encabezados `anthropic-*` y cuerpos de solicitud textualmente en lugar de permitir listas; pruebe nuevos lanzamientos de Claude Code contra la puerta de enlace antes de que lleguen a los desarrolladores, comprobando las áreas en [Planificar actualizaciones de versión de Claude Code](#plan-claude-code-version-upgrades) |
| Nuevos modelos Claude se vuelven disponibles                                                           | Los desarrolladores que seleccionan un nuevo nombre de modelo obtienen `404`; el selector `/model` no lo lista                                                                             | Añada el nombre del modelo a la configuración de enrutamiento de la puerta de enlace, luego vuelva a ejecutar la [verificación de enrutamiento](#confirm-the-gateway-routes-your-models). Si distribuye `ANTHROPIC_MODEL` o las variables de modelo predeterminado, actualice la configuración administrada                             |
| Las credenciales expiran o necesitan rotación                                                          | Todas las solicitudes de desarrollador comienzan a fallar con `401` del ascendente                                                                                                         | Rote la credencial de proveedor de la puerta de enlace en su propio cronograma; las claves de desarrollador rotan en la puerta de enlace, y un [`apiKeyHelper`](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper) maneja la rotación por desarrollador sin redistribuir configuración                                       |

Al dimensionar límites de velocidad por clave, tenga en cuenta que el cliente [reintenta fallos transitorios](/docs/es/errors#automatic-retries), incluidas respuestas `429`, hasta 10 veces con retroceso, honrando `Retry-After`. Mantenga la [guía de compatibilidad](/docs/es/llm-gateway-protocol) como referencia para lo que cada lanzamiento de Claude Code envía.

<h3 id="plan-claude-code-version-upgrades">
  Planificar actualizaciones de versión de Claude Code
</h3>

Algunos comportamientos de Claude Code están integrados en la versión instalada en lugar de configurarse en su puerta de enlace, por lo que mover desarrolladores a un nuevo lanzamiento puede cambiar el comportamiento en toda su implementación incluso cuando la configuración de la puerta de enlace no ha cambiado. Para controlar cuándo sucede eso, fije los desarrolladores a una versión probada con [`requiredMaximumVersion`](/docs/es/settings-reference#requiredmaximumversion), o con [`DISABLE_UPDATES`](/docs/es/setup#disable-auto-updates) si distribuye Claude Code a través de su propio canal. Antes de aumentar la fijación, lea la entrada del [registro de cambios](/docs/en/changelog) del nuevo lanzamiento y [pruébelo contra la puerta de enlace](#test-claude-code-against-the-gateway).

Cuando prueba un lanzamiento, los nuevos encabezados o campos de solicitud que la puerta de enlace rechaza aparecen como los errores `400` descritos en [Mantener la puerta de enlace](#maintain-the-gateway). La tabla siguiente cubre cambios dependientes de la versión que no producen un error, con la configuración que mantiene cada uno constante en las actualizaciones.

| Área                                                      | Qué puede cambiar cuando los desarrolladores actualizan                                                                                                                                                                                                                                                                                                                                                                                        | Configuración que lo mantiene constante                                                                                                                                                                                                                                                                                                                                        |
| :-------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Valores predeterminados de indicadores de características | Las sesiones que [no obtienen indicadores de características de Anthropic](/docs/es/env-vars#features-that-need-feature-flag-fetching), como sesiones en un proveedor de nube o con telemetría desactivada, utilizan los valores predeterminados de indicadores integrados en la versión instalada. Cuando un lanzamiento cambia uno de esos valores predeterminados, el comportamiento cambia para esos desarrolladores tan pronto como actualicen | La fijación de versión en sí, `requiredMaximumVersion` o `DISABLE_UPDATES`                                                                                                                                                                                                                                                                                                     |
| Suposiciones de capacidad del modelo                      | Un ID de modelo que la versión instalada no reconoce, como el alias de puerta de enlace `prod-opus`, se ejecuta en suposiciones predeterminadas para [razonamiento adaptativo](/docs/es/model-config#adaptive-reasoning-and-fixed-thinking-budgets), el parámetro de esfuerzo y la [ventana de contexto](/docs/es/model-config#correct-the-window-for-a-gateway-or-custom-model-id) hasta que una versión posterior reconoce el ID o lo asigna           | Enrute IDs de modelo de Anthropic en la puerta de enlace, o añada una entrada [`modelOverrides`](/docs/es/model-config#override-model-ids-per-version) que asigne el ID de modelo de Anthropic a su alias. En una conexión de proveedor de nube, puede en su lugar [declarar las capacidades de un modelo fijado](/docs/es/model-config#customize-pinned-model-display-and-capabilities) |
| Modelo predeterminado y alias                             | El modelo en el que las nuevas sesiones comienzan de forma predeterminada, y los modelos que alias como `opus` y `sonnet` resuelven, están [integrados en cada versión](/docs/es/model-config#pin-models-for-third-party-deployments) y pueden cambiar cuando los desarrolladores actualizan                                                                                                                                                        | [`ANTHROPIC_DEFAULT_MODEL`](/docs/es/model-config#set-a-default-model-for-new-sessions) para el modelo en el que las nuevas sesiones comienzan, y las [variables `ANTHROPIC_DEFAULT_*_MODEL`](/docs/es/model-config#environment-variables), como `ANTHROPIC_DEFAULT_OPUS_MODEL`, para lo que cada alias resuelve. `ANTHROPIC_DEFAULT_MODEL` requiere Claude Code v2.1.236 o posterior    |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect): los pasos de configuración orientados al desarrollador, con configuración por superficie y una tabla de solución de problemas que puede entregar a los desarrolladores
* [Guía de compatibilidad de puerta de enlace](/docs/es/llm-gateway-protocol): la referencia para operadores de puerta de enlace, cubriendo puntos finales, encabezados a reenviar y la tabla de paso de características
* [Qué valor utiliza Claude Code](/docs/es/settings#which-value-claude-code-uses): cómo se combinan las configuraciones administradas, de proyecto y de usuario
* [Mecanismos de entrega](/docs/es/managed-settings#delivery-mechanisms): dónde va el archivo administrado en cada plataforma
* [Configurar Claude Code para su organización](/docs/es/admin-setup): la implementación más amplia de la que esta puerta de enlace es una parte, incluida la aplicación de políticas, visibilidad de uso y manejo de datos
