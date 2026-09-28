> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración de red empresarial

> Configure Claude Code para entornos empresariales con servidores proxy, Autoridades de Certificación (CA) personalizadas y autenticación mutua de Seguridad de la Capa de Transporte (mTLS).

Claude Code admite varias configuraciones de red y seguridad empresarial a través de variables de entorno. Esto incluye enrutar el tráfico a través de servidores proxy corporativos, confiar en Autoridades de Certificación (CA) personalizadas y autenticarse con certificados de Seguridad de la Capa de Transporte mutua (mTLS) para mayor seguridad.

Establezca estas variables de entorno antes de iniciar Claude Code. Las variables exportadas en su shell se leen una sola vez al inicio, por lo que una sesión en ejecución no recoge cambios posteriores en su entorno de shell.

<Note>
  Todas las variables de entorno que se muestran en esta página también se pueden configurar en [`settings.json`](/docs/es/settings).
</Note>

<h2 id="proxy-configuration">
  Configuración de proxy
</h2>

<h3 id="environment-variables">
  Variables de entorno
</h3>

Claude Code respeta las variables de entorno de proxy estándar. En sesiones de Claude Desktop donde la aplicación gestiona la conexión del proveedor, Claude Code las lee solo desde la configuración gestionada y `~/.claude/settings.json`; consulte [autenticación mTLS](#mtls-authentication) para las reglas de alcance.

```bash theme={null}
# Proxy HTTPS (recomendado)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (si HTTPS no está disponible)
export HTTP_PROXY=http://proxy.example.com:8080

# Omitir proxy para solicitudes específicas - formato separado por espacios
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Omitir proxy para solicitudes específicas - formato separado por comas
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Omitir proxy para todas las solicitudes
export NO_PROXY="*"
```

Las variantes en minúsculas también funcionan, y Claude Code utiliza la primera que esté configurada en el orden `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY`.

Claude Code nunca envía sus conexiones WebSocket a `localhost`, `::1`, o `127.0.0.0/8` a través del proxy, por lo que no necesita una entrada de loopback en `NO_PROXY` para ellas.

<Note>
  Claude Code no admite proxies SOCKS.
</Note>

<h3 id="basic-authentication">
  Autenticación básica
</h3>

Si su proxy requiere autenticación básica, incluya las credenciales en la URL del proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Evite codificar contraseñas en scripts. Utilice variables de entorno o almacenamiento seguro de credenciales en su lugar.
</Warning>

<Tip>
  Para proxies que requieren autenticación avanzada (NTLM, Kerberos, etc.), considere utilizar un servicio LLM Gateway que admita su método de autenticación.
</Tip>

<h2 id="ca-certificate-store">
  Almacén de certificados CA
</h2>

De forma predeterminada, Claude Code confía tanto en sus certificados CA de Mozilla incluidos como en el almacén de certificados de su sistema operativo. La lectura del almacén del sistema operativo requiere un tiempo de ejecución con `tls.getCACertificates`: el instalador nativo siempre lo tiene, y las instalaciones de npm necesitan Node 22.15 o posterior. En versiones anteriores de Node, solo se aplican el conjunto incluido y `NODE_EXTRA_CA_CERTS`. Los proxies de inspección TLS empresariales funcionan sin configuración adicional cuando su certificado raíz se instala en el almacén de confianza del sistema operativo y el tiempo de ejecución puede leerlo.

`CLAUDE_CODE_CERT_STORE` acepta una lista separada por comas de fuentes. Los valores reconocidos son `bundled` para el conjunto de CA de Mozilla incluido con Claude Code y `system` para el almacén de certificados del sistema operativo. El valor predeterminado es `bundled,system`.

Para confiar solo en el conjunto de CA de Mozilla incluido:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Para confiar solo en el almacén de certificados del sistema operativo:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` no tiene una clave de esquema dedicada en `settings.json`. Establézcalo a través del bloque `env` en `~/.claude/settings.json` o directamente en el entorno del proceso.
</Note>

<h2 id="custom-ca-certificates">
  Certificados CA personalizados
</h2>

Si su entorno empresarial utiliza una CA personalizada, configure Claude Code para confiar en ella directamente:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autenticación mTLS
</h2>

Para entornos empresariales que requieren autenticación de certificado de cliente:

```bash theme={null}
# Certificado de cliente para autenticación
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Clave privada del cliente
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Opcional: Frase de contraseña para clave privada cifrada
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code lee los archivos de certificado y clave al iniciar y los vuelve a leer cada vez que aplica configuración, como cuando su organización cambia el bloque `env` en [configuración administrada](/docs/es/server-managed-settings) a mitad de sesión.

Para rotar el certificado y la clave, reemplace los archivos en las mismas rutas. Claude Code recoge el reemplazo en una sesión en ejecución sin necesidad de reiniciar. Cuando una solicitud de API falla con un error a nivel de conexión, como un restablecimiento de conexión o un error de protocolo de enlace TLS, vuelve a leer ambos archivos e intenta nuevamente la solicitud con el nuevo par. Antes de v2.1.232, Claude Code no volvía a leer en errores de conexión, por lo que mantenía el par que ya había cargado hasta que aplicaba configuración nuevamente o usted reiniciaba.

Claude Code vuelve a leer los archivos en respuesta a solicitudes fallidas, no observando cambios en ellos:

* **Tiempo**: Claude Code no hace nada en el momento en que reemplaza los archivos. Presenta el nuevo par en el reintento después de una falla calificada, o en la siguiente solicitud después de que aplica configuración, lo que ocurra primero.
* **Rechazos de puerta de enlace**: Claude Code vuelve a leer cuando su puerta de enlace restablece la conexión o rechaza el protocolo de enlace TLS después de que deja de aceptar el par anterior. No vuelve a leer cuando la puerta de enlace completa el protocolo de enlace y responde con un error HTTP. En ese caso, Claude Code carga el nuevo par cuando aplica configuración nuevamente o cuando lo reinicia.
* **Rotaciones parcialmente escritas**: cuando Claude Code vuelve a leer mientras su rotación está a mitad de escritura, como leer un certificado y clave que no coinciden entre sí, mantiene el par anterior y vuelve a leer en la siguiente falla.
* **Exportadores de telemetría OTLP**: Claude Code mantiene el certificado que los [exportadores](/docs/es/monitoring-usage#mtls-authentication) cargaron en el primer uso, por lo que reinicie Claude Code para que un certificado rotado llegue a su recopilador de telemetría.
* **Desactivar la recarga**: establezca [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/es/env-vars#variables) para desactivar la recarga de error de conexión. Claude Code recoge archivos rotados solo cuando aplica configuración nuevamente o en el siguiente inicio.

Para confirmar que Claude Code recogió una rotación, [inicie la sesión con registro de depuración](#verify-your-configuration) y busque `Stale connection — reloaded rotated mTLS client material` en el registro. Claude Code no registra esta línea cuando recoge la rotación mientras aplica configuración en su lugar, por lo que una línea faltante por sí sola no significa que la rotación haya fallado.

Reemplace los archivos antes de que expire el par actual para que Claude Code no cargue un par ya expirado en el siguiente inicio.

En [sesiones en la nube](/docs/es/claude-code-on-the-web), el entorno de alojamiento administra la conexión a la API, por lo que Claude Code ignora las siguientes variables cuando provienen de un bloque `env` de archivo de configuración:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code anota cada clave ignorada en el registro de depuración de la sesión.

En sesiones de [Claude Desktop](/docs/es/desktop) donde la aplicación administra la conexión del proveedor, como la pestaña Code en un [proveedor de terceros](/docs/es/third-party-integrations) y sesiones de Cowork, Claude Code lee estas variables y las variables proxy `HTTP_PROXY`, `HTTPS_PROXY` y `NO_PROXY` solo desde [configuración administrada](/docs/es/managed-settings) y `~/.claude/settings.json`: las ignora en los archivos de configuración propios de un repositorio, por lo que un repositorio extraído no puede redirigir la ruta TLS o proxy de una sesión cuyas credenciales provienen de la aplicación. En una sesión de pestaña Code local, SSH o WSL con sesión iniciada a través de claude.ai, la aplicación no administra la conexión, y Claude Code lee estas variables desde cada ámbito de configuración, como cualquier sesión de terminal; [las sesiones en la nube](/docs/es/claude-code-on-the-web) siguen las reglas de sesión en la nube anteriores dondequiera que las inicie. Antes de v2.1.217, Claude Code ignoraba estas variables en cada archivo de configuración cuando la aplicación administraba la conexión.

<h2 id="verify-your-configuration">
  Verificar su configuración
</h2>

Generalmente se entera de una dirección de proxy incorrecta o una ruta de certificado incorrecta a partir de un [error de conexión o certificado](/docs/es/errors#network-and-connection-errors) en una solicitud posterior, ya que Claude Code no valida la mayoría de estas configuraciones cuando las lee. La única configuración que verifica al iniciar es la URL del proxy: cuando no puede analizar el valor, como uno que carece del esquema `http://`, Claude Code detiene el lanzamiento con un error que nombra la variable a corregir.

Para confirmar que su configuración se cargó antes de enviar una solicitud, inicie Claude Code con registro de depuración:

```bash theme={null}
claude --debug
```

La salida de depuración va a `~/.claude/debug/<session-id>.txt` en lugar de la terminal, o a una ruta que establezca con `--debug-file <path>`. En el registro, busque las líneas que confirmen que cada archivo se cargó:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Si Claude Code no puede leer uno de estos archivos, el registro muestra una línea `Failed to read` o `Failed to load` con la razón en su lugar.

También puede ejecutar `/status` en una sesión interactiva y verificar estas filas:

* **Proxy**: muestra la URL del proxy activo y marca un valor que no puede analizar como inválido e ignorado.
* **mTLS client cert** y **mTLS client key**: aparecen solo cuando los archivos se cargaron, por lo que una fila faltante significa que la carga falló y el registro de depuración tiene la razón.
* **Additional CA cert(s)**: muestra la ruta `NODE_EXTRA_CA_CERTS` sin verificar que el archivo se cargó, así que confirme este en el registro de depuración.

<h2 id="apply-network-settings-to-background-agents">
  Aplicar configuración de red a agentes en segundo plano
</h2>

[Los agentes en segundo plano](/docs/es/agent-view) no se ejecutan dentro de la terminal que los envió. Un proceso supervisor por usuario se inicia bajo demanda, sobrevive a su shell y aloja cada sesión de `claude agents`, `--bg` y `/background`. Consulte [Cómo se alojan las sesiones en segundo plano](/docs/es/agent-view#how-background-sessions-are-hosted). Esto cambia cómo la configuración en esta página llega a esas sesiones.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Establecer variables de red en la configuración, no en el shell
</h3>

El supervisor es un proceso compartido por cada terminal. Hereda el entorno del shell que lo inicia primero, y un supervisor instalado por el sistema operativo no recibe ningún entorno de shell. Si exporta un proxy, ruta de CA o variable de mTLS solo en su shell, llega a los agentes en segundo plano cuando ese shell sucedió a iniciar en frío el supervisor, y silenciosamente no llega cuando un shell diferente lo hizo.

Coloque las mismas variables en el bloque `env` de `~/.claude/settings.json` o [configuración administrada](/docs/es/settings) en su lugar. Cada variable en esta página se puede establecer allí, y la configuración es la única que llega a cada sesión en segundo plano en cada máquina.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Configurar un iniciador corporativo como una configuración
</h3>

Algunas organizaciones requieren que cada proceso de Claude Code se inicie a través de un iniciador corporativo que aplique sandboxing, controles de red o inyección de credenciales. El supervisor y sus trabajadores inician Claude Code desde una ruta fija en lugar de buscar `claude` en `PATH`, por lo que cada agente en segundo plano omite un contenedor que coloca anteriormente en `PATH`.

Establezca la configuración [`processWrapper`](/docs/es/settings-reference#processwrapper) para prefijar el supervisor, sus trabajadores y los otros procesos en segundo plano enumerados en [Qué cubre el iniciador](/docs/es/corporate-launcher#what-the-launcher-covers) con su iniciador. La variable de entorno equivalente [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/es/env-vars) tiene prioridad cuando ambas se establecen, y está sujeta a la misma regla: entréguela a través de la configuración administrada o `~/.claude/settings.json`, no una exportación de shell. [Ejecutar Claude Code detrás de un iniciador corporativo](/docs/es/corporate-launcher) cubre el contrato que el iniciador debe satisfacer, qué alcanza y qué no alcanza, y cómo implementarlo.

<Note>
  Un supervisor ya en ejecución mantiene la configuración de lanzamiento con la que se inició. Después de implementar la configuración del iniciador, ejecute [`claude daemon stop --any`](/docs/es/agent-view#the-supervisor-process) para que el siguiente `claude agents` o `--bg` inicie un supervisor que la respete. Un servicio instalado toma `claude daemon stop` sin `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Perros guardianes de inactividad en streaming
</h2>

Claude Code ejecuta cuatro temporizadores independientes que abortan una respuesta de modelo en streaming cuando se queda en silencio, de modo que una conexión muerta falla y se reintenta en lugar de quedarse colgada. El plazo de primer byte cubre la espera de encabezados de respuesta, antes de que haya llegado ninguna parte de la respuesta. Cada uno de los otros tres supervisa una respuesta activa para una señal diferente.

| Temporizador                               | Aborta cuando                                                                                                                                                                                                                                                     | Se ejecuta en                                                                                                                                                                                                                                                                                                                                                                         | Tiempo de espera predeterminado                                                                                                     |
| :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| Plazo de primer byte                       | No llegan encabezados de respuesta después de que Claude Code envía la solicitud                                                                                                                                                                                  | API de Anthropic directa y [Claude Platform on AWS](/docs/es/claude-platform-on-aws), incluyendo a través de un proxy HTTPS, pero no cuando `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL` los enrutan a través de una [gateway](/docs/es/gateways). Opt-in en Amazon Bedrock con `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; no se ejecuta en Agent Platform de Google Cloud o Microsoft Foundry | 180 segundos en la API de Anthropic directa, 300 segundos en otros lugares, más un segundo por cada 32KB del cuerpo de la solicitud |
| Perro guardián a nivel de evento           | No se analizan eventos de respuesta. En conexiones donde se ejecuta el perro guardián a nivel de byte, los bytes que llegan, incluyendo pings de keep-alive, también reinician este perro guardián, durante aproximadamente cinco minutos sin un evento analizado | Cada proveedor                                                                                                                                                                                                                                                                                                                                                                        | 300 segundos                                                                                                                        |
| Perro guardián a nivel de byte             | No llegan bytes en el cable, incluyendo pings de keep-alive de SSE                                                                                                                                                                                                | API de Anthropic directa, [Claude Platform on AWS](/docs/es/claude-platform-on-aws), y conexiones de [gateway](/docs/es/gateways), incluyendo un `ANTHROPIC_BASE_URL` personalizado. Opt-in en respuestas `vnd.amazon.eventstream` de Amazon Bedrock con `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; no se ejecuta en Agent Platform de Google Cloud o Microsoft Foundry                           | 180 segundos en la API de Anthropic directa, 300 segundos en otros lugares                                                          |
| Tiempo de espera de inactividad del cuerpo | No llegan bytes durante 5 minutos                                                                                                                                                                                                                                 | Proveedores distintos de la API de Anthropic directa y Claude Platform on AWS, a menos que [`API_FORCE_IDLE_TIMEOUT`](/docs/es/env-vars) cambie eso                                                                                                                                                                                                                                        | 5 minutos                                                                                                                           |

Configure los temporizadores con estas variables, cada una detallada en la [referencia de variables de entorno](/docs/es/env-vars):

* `CLAUDE_ENABLE_STREAM_WATCHDOG` y `CLAUDE_ENABLE_BYTE_WATCHDOG` fuerzan el perro guardián correspondiente activado con `1` o desactivado con `0`, dentro de los tipos de conexión que la tabla enumera; ninguna variable extiende un perro guardián a un tipo de conexión que no cubre. `CLAUDE_ENABLE_BYTE_WATCHDOG` establecido en `0` también desactiva el plazo de primer byte.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` establece el tiempo de espera de ambos perros guardianes. Claude Code eleva los valores por debajo de 5 minutos a 5 minutos, y limita el valor a 30 minutos para el perro guardián a nivel de byte.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` establece el tiempo de espera del perro guardián a nivel de byte sin cambiar el del perro guardián a nivel de evento, limitado entre 10 segundos y 30 minutos, y tiene prioridad sobre `CLAUDE_STREAM_IDLE_TIMEOUT_MS` para ese perro guardián.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` establece el plazo de primer byte directamente. Déjelo sin establecer y Claude Code usa el tiempo de espera del perro guardián a nivel de byte, de modo que `CLAUDE_STREAM_IDLE_TIMEOUT_MS` y `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` también cambian el plazo. Para los límites, la asignación de carga, el límite de `API_TIMEOUT_MS` y cuánto tiempo espera el reintento después de un aborto sin respuesta, consulte [Sin respuesta de la API](/docs/es/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` establecido en `0` desactiva el tiempo de espera de inactividad del cuerpo, y establecido en `1` lo activa para cada proveedor. Los perros guardianes se ejecutan independientemente de él, por lo que para permitir que un stream se pause más tiempo que sus umbrales, también aumente o desactive los perros guardianes.

Cuando un perro guardián aborta un stream estancado, Claude Code trata el aborto como una falla a mitad de stream, y lo que ve depende de cuán lejos haya llegado la respuesta. Claude Code reintenta la solicitud o termina el turno con un error, mantiene la salida completada y muestra un [aviso de respuesta incompleta](/docs/es/errors#the-response-above-may-be-incomplete), o termina el turno normalmente. [Reintentos automáticos](/docs/es/errors#automatic-retries) dice dónde se aplica cada resultado.

En una [sesión no interactiva](/docs/es/headless), y para la respuesta de un subagente en cualquier sesión, Claude Code puede primero solicitar a Claude que continúe la respuesta cortada; [la entrada de ese aviso](/docs/es/errors#the-response-above-may-be-incomplete) dice cuándo lo hace y cuándo aún ve el aviso.

Cuando se activa el plazo de primer byte, no ha comenzado ninguna respuesta, por lo que no hay salida parcial para mantener. Para saber cómo Claude Code reenvía la solicitud y cuándo termina el turno en su lugar, consulte [Sin respuesta de la API](/docs/es/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Requisitos de acceso a la red
</h2>

Claude Code requiere acceso a las siguientes URLs. Agregue estas a la lista de permitidos en su configuración de proxy y reglas de firewall, especialmente en entornos de red en contenedores o restringidos. La verificación de conectividad de configuración de primera ejecución apunta aquí cuando no puede alcanzar `api.anthropic.com` o `platform.claude.com`; consulte [No se puede conectar a los servicios de Anthropic](/docs/es/errors#unable-to-connect-to-anthropic-services) para los mensajes de la verificación y los pasos de recuperación.

| URL                                  | Requerido para                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Solicitudes de API de Claude, incluida la verificación de seguridad de dominio de WebFetch [domain safety check](/docs/es/data-usage#webfetch-domain-safety-check), obtención de banderas de características y registro de eventos de telemetría                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `claude.ai`                          | Autenticación de cuenta de claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `claude.com`                         | El inicio de sesión de la cuenta de claude.ai abre una página `claude.com` en el navegador, que redirige a `claude.ai`; las búsquedas de documentación de WebFetch preaprobadas también llegan a este host desde la CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `platform.claude.com`                | Autenticación de cuenta de Anthropic Console. El intercambio, actualización y revocación de tokens OAuth también van a este host para cuentas de claude.ai, por lo que tanto los inicios de sesión de Console como de claude.ai lo requieren                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `mcp-proxy.anthropic.com`            | [Conectores MCP desde claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai), incluidos los conectores que configura un administrador de la organización. El tráfico de conectores se enruta a través de este proxy; los conectores están habilitados de forma predeterminada para usuarios autenticados en claude.ai. Para evitar que Claude Code los obtenga, establezca [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/es/env-vars) o la configuración [`disableClaudeAiConnectors`](/docs/es/settings-reference#disableclaudeaiconnectors)                                                                                                                                               |
| `downloads.claude.ai`                | Descargas de ejecutables de plugins; instalador nativo, actualizador automático nativo y verificaciones de versión de actualización                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `storage.googleapis.com`             | Recuentos de instalación de plugins y metadatos mostrados en `/plugin`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `storage.googleapis.com`             | Instalador nativo y actualizador automático nativo en versiones anteriores a 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `registry.npmjs.org`                 | Instalaciones de plugins (obtención de paquetes de plugins de origen npm e instalación de dependencias de paquetes Node.js de plugins), servidores MCP lanzados con `npx` y el registro de paquetes para instalaciones de npm y bun de Claude Code en sí                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `bridge.claudeusercontent.com`       | Puente WebSocket de extensión [Claude en Chrome](/docs/es/chrome)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `*.frame.claudeusercontent.com`      | Lecturas de contenido de [Artifact](/docs/es/artifacts). La CLI obtiene los archivos de un artefacto de este host cuando Claude abre uno, y solo cuando la herramienta Artifact está [disponible](/docs/es/artifacts#availability) para su cuenta. Para desactivar la herramienta y eliminar este requisito, establezca [`"enableArtifact": false`](/docs/es/settings-reference#enableartifact) o [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/es/env-vars); Claude Code también respeta la configuración [`disableArtifact`](/docs/es/settings-reference#disableartifact) obsoleta. Consulte [Deshabilitar artefactos](/docs/es/artifacts#disable-artifacts) para ver cómo interactúan estas configuraciones |
| `github.com`                         | Clonación de [marketplaces de plugins](/docs/es/plugins/overview) y plugins alojados en GitHub, incluido el marketplace oficial de Anthropic, sobre HTTPS o SSH. Para clonar fuentes de GitHub `owner/repo` solo sobre HTTPS, establezca [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/es/env-vars)                                                                                                                                                                                                                                                                                                                                                                                     |
| `raw.githubusercontent.com`          | Fuente de registro de cambios para [`/release-notes`](/docs/es/commands). En sesiones interactivas, Claude Code también la obtiene en segundo plano al inicio cuando su registro de cambios en caché aún no cubre la versión en ejecución, como el primer inicio después de una actualización; las sesiones no interactivas y en la nube nunca la obtienen                                                                                                                                                                                                                                                                                                                       |
| `*-review.googlesource.com`          | Búsqueda de cambios de Gerrit en checkouts de `googlesource.com`. Cuando una sesión de pestaña de Claude Desktop Code se inicia o se reanuda en un checkout [confiable](/docs/es/permissions#project-allow-rules-and-workspace-trust) cuyo `origin` es un host `googlesource.com`, Claude Code pregunta anónimamente al servidor `-review` de ese host por el cambio abierto que coincida con el `Change-Id` de HEAD, una vez por inicio o reanudación. Otros tipos de sesión omiten la búsqueda, y no se contacta a ningún otro host de Gerrit. Opcional: deshabilitar con [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars)                                           |
| `http-intake.logs.us5.datadoghq.com` | Eventos de telemetría operativa, enviados solo cuando la CLI usa la API de Anthropic directamente, nunca para Amazon Bedrock, Agent Platform de Google Cloud o Microsoft Foundry. Opcional: deshabilitar con [`DISABLE_TELEMETRY`](/docs/es/data-usage#telemetry-services) o `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                                                                                      |
| `browser-intake-us5-datadoghq.com`   | Informes de errores operativos, enviados cuando la CLI usa la API de Anthropic directamente y una puerta de lanzamiento del lado del servidor los habilita. Opcional: deshabilitar con `DISABLE_ERROR_REPORTING` o `DISABLE_TELEMETRY`; consulte [Servicios de telemetría](/docs/es/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                                                               |
| `formulae.brew.sh`                   | Verificaciones de versión de actualización en instalaciones de Homebrew. Otros métodos de instalación no contactan este host                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `code.claude.com`                    | Búsquedas de documentación de Claude Code por el agente claude-code-guide integrado y solicitudes de WebFetch preaprobadas. Bloquear este host solo afecta las búsquedas de documentación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

Si instala Claude Code a través de npm o administra su propia distribución binaria, los usuarios finales no necesitan los usos del instalador nativo y actualizador automático de `downloads.claude.ai`, pero las instalaciones de npm y bun necesitan su registro de paquetes, `registry.npmjs.org`, a menos que su organización lo refleje. Los otros usos en la tabla se aplican independientemente del método de instalación.

Los dos hosts de ingesta de Datadog llevan solo telemetría operativa opcional, y establecer [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars) deshabilita ambos. Las sesiones en proveedores de terceros nunca envían a estos hosts, incluso cuando una plataforma establece [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars) y las métricas de telemetría están habilitadas de forma predeterminada. Consulte [Servicios de telemetría](/docs/es/data-usage#telemetry-services) para todo lo que Claude Code envía y cómo deshabilitarlo antes de finalizar su lista de permitidos.

Cuando se usa [Amazon Bedrock](/docs/es/amazon-bedrock), [Agent Platform de Google Cloud](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry) o una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, el tráfico del modelo y la autenticación van a su proveedor o puerta de enlace en lugar de `api.anthropic.com`, `claude.ai` o `platform.claude.com`. La herramienta WebFetch aún llama a `api.anthropic.com` para su [verificación de seguridad de dominio](/docs/es/data-usage#webfetch-domain-safety-check) a menos que establezca `skipWebFetchPreflight: true` en [configuración](/docs/es/settings).

Cuando se enruta a través de una [puerta de enlace LLM](/docs/es/llm-gateway) con [`ANTHROPIC_BASE_URL`](/docs/es/llm-gateway-connect#set-the-base-url-and-credential), la verificación de disponibilidad de [modo rápido](/docs/es/fast-mode) aún llama a `api.anthropic.com` en lugar de la URL base de la puerta de enlace. La verificación respeta un proxy HTTP configurado, por lo que donde un bloqueo de red es la causa, una entrada de lista de permitidos para `api.anthropic.com` en el proxy es la solución. Un bloqueo de red falla la verificación solo donde el host es inaccesible incluso a través del proxy, y el modo rápido luego reporta un error de conectividad. El mismo error de conectividad aparece cuando la verificación presenta una credencial emitida por la puerta de enlace que Anthropic rechaza; la lista de permitidos no ayuda allí, ya que nada está bloqueado. Consulte [usar modo rápido detrás de proxies y puertas de enlace LLM](/docs/es/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) para las variables que lo restauran.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Listas de permitidos de IP de la organización y salida de proxy
</h3>

Si su organización tiene [lista de permitidos de IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) habilitada para Claude, enrute `bridge.claudeusercontent.com` a través del mismo proxy de salida que `claude.ai` y `api.anthropic.com`, por ejemplo colocándolo en el mismo segmento de aplicación de Zscaler o política de dirección de Netskope. Si no puede enrutarlo de esa manera, agregue la dirección de salida que su proxy usa para ese host a la lista de permitidos de IP de su organización, pero solo cuando esa dirección está dedicada a su organización: un rango de salida de proxy compartido también admite a otros clientes del proveedor de proxy.

Anthropic verifica las conexiones a `bridge.claudeusercontent.com` contra la lista de permitidos de IP de su organización usando la dirección desde la que llegan. Si su proxy envía tráfico para ese host a través de una dirección que no está en esa lista de permitidos, Claude Code no puede conectarse a la extensión [Claude en Chrome](/docs/es/chrome) aunque el resto de Claude Code funcione.

<h3 id="github-allow-lists-and-firewalls">
  Listas de permitidos de GitHub y firewalls
</h3>

[Claude Code en la web](/docs/es/claude-code-on-the-web) en entornos alojados por Anthropic y [Code Review](/docs/es/code-review) se conectan a sus repositorios desde infraestructura administrada por Anthropic; las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments) se conectan desde dentro de su red, a menos que el ejecutor opte por el [proxy git de Anthropic](/docs/es/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que obtiene desde el lado de Anthropic.

Si su organización de GitHub Enterprise Cloud restringe el acceso por dirección IP, habilite [herencia de lista de permitidos de IP para aplicaciones GitHub instaladas](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) y también [agregue una entrada de lista de permitidos](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) para las [direcciones IP salientes](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) de Anthropic. La herencia cubre solo las solicitudes que realiza la aplicación GitHub de Claude como instalación, no las solicitudes que realiza en nombre de sus usuarios. Para otros firewalls, consulte las [direcciones IP de la API de Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Para instancias de [GitHub Enterprise Server](/docs/es/github-enterprise-server) autohospedadas detrás de un firewall, agregue a la lista de permitidos las [direcciones IP salientes](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) de Anthropic para que la infraestructura de Anthropic pueda alcanzar su host GHES para clonar repositorios y publicar comentarios de revisión. Las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments-deploy#configure-git) alcanzan su host GHES desde dentro de su red en su lugar, por lo que esa exposición se aplica solo a sesiones alojadas por Anthropic, a flujos previos a la sesión alojados como el selector de repositorio, y a ejecutores autohospedados que opten por el [proxy git de Anthropic](/docs/es/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que obtiene desde el lado de Anthropic. Para un host GHES que solo es enrutable dentro de su red, el [conector SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) lleva los flujos previos a la sesión alojados sobre una conexión saliente en su lugar, por lo que la lista de permitidos no es necesaria para ellos.

<h3 id="desktop-and-claude-ai">
  Escritorio y claude.ai
</h3>

La tabla anterior cubre la CLI independiente. La aplicación Claude Desktop y claude.ai en un navegador cargan su código de aplicación y contenido del usuario desde hosts CDN adicionales de Anthropic, incluidos `assets-proxy.anthropic.com` y los otros orígenes `*.claudeusercontent.com` que sirven [artefactos](/docs/es/artifacts) en esas aplicaciones. Permitir `claude.ai` mientras se bloquean esos hosts produce una página en blanco en lugar de un error. Consulte [requisitos de acceso a la red](/docs/es/desktop#network-access-requirements) en la página de Desktop.

Un [artefacto](/docs/es/artifacts) que carga una fuente tipográfica desde [Google Fonts](/docs/es/artifacts#improve-the-visual-design) también solicita `fonts.googleapis.com` y `fonts.gstatic.com`. Ambos hosts son opcionales. Si los bloquea, los artefactos se renderizan en fuentes tipográficas alternativas. Bloquee con un rechazo rápido en lugar de una caída silenciosa para que la solicitud de fuente falle inmediatamente en lugar de retrasar el primer renderizado de la página.

Los artefactos también pueden cargar bibliotecas de JavaScript, como React o un paquete de gráficos, desde `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` y `unpkg.com`, y desde ningún otro host externo. Si bloquea esos hosts, las partes de un artefacto que dependen de una biblioteca no funcionan, y a diferencia de una fuente bloqueada, una biblioteca bloqueada no tiene alternativa. Bloquee con un rechazo rápido aquí también, para que una solicitud de biblioteca bloqueada falle de inmediato en lugar de colgarse hasta que se agote el tiempo de espera.

<h2 id="additional-resources">
  Recursos adicionales
</h2>

* [Archivos de configuración y precedencia](/docs/es/settings)
* [Referencia de variables de entorno](/docs/es/env-vars)
* [Guía de solución de problemas](/docs/es/troubleshooting)
