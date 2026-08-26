> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración de red empresarial

> Configure Claude Code para entornos empresariales con servidores proxy, Autoridades de Certificación (CA) personalizadas y autenticación mutua de Seguridad de la Capa de Transporte (mTLS).

Claude Code admite varias configuraciones de red y seguridad empresarial a través de variables de entorno. Esto incluye enrutar el tráfico a través de servidores proxy corporativos, confiar en Autoridades de Certificación (CA) personalizadas y autenticarse con certificados de Seguridad de la Capa de Transporte mutua (mTLS) para mayor seguridad.

<Note>
  Todas las variables de entorno que se muestran en esta página también se pueden configurar en [`settings.json`](/docs/es/settings).
</Note>

<h2 id="proxy-configuration">
  Configuración de proxy
</h2>

<h3 id="environment-variables">
  Variables de entorno
</h3>

Claude Code respeta las variables de entorno de proxy estándar:

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

De forma predeterminada, Claude Code confía tanto en sus certificados CA de Mozilla incluidos como en el almacén de certificados de su sistema operativo. La lectura del almacén del sistema operativo requiere un tiempo de ejecución con `tls.getCACertificates`: el instalador nativo siempre lo tiene, y las instalaciones de npm necesitan Node 22.15 o posterior. En versiones anteriores de Node, solo se aplican el conjunto incluido y `NODE_EXTRA_CA_CERTS`. Los proxies de inspección TLS empresariales como CrowdStrike Falcon y Zscaler funcionan sin configuración adicional cuando su certificado raíz se instala en el almacén de confianza del sistema operativo y el tiempo de ejecución puede leerlo.

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

Claude Code lee los archivos de certificado y clave al iniciar y los vuelve a leer cada vez que aplica la configuración, incluyendo cuando la configuración cambia durante una sesión. Para rotar el certificado y la clave, reemplace los archivos en las mismas rutas.

<h2 id="network-access-requirements">
  Requisitos de acceso a la red
</h2>

Claude Code requiere acceso a las siguientes URL. Agregue estas a la lista blanca en su configuración de proxy y reglas de firewall, especialmente en entornos de red en contenedores o restringidos.

| URL                            | Requerido para                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Solicitudes de API de Claude                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude.ai`                    | Autenticación de cuenta de claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `platform.claude.com`          | Autenticación de cuenta de Anthropic Console                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `mcp-proxy.anthropic.com`      | [Conectores MCP desde claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai), incluidos los conectores que configura un administrador de la organización. El tráfico de conectores se enruta a través de este proxy; los conectores están habilitados de forma predeterminada para usuarios autenticados en claude.ai. Para deshabilitarlos, establezca [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/es/env-vars) o la configuración [`disableClaudeAiConnectors`](/docs/es/settings#available-settings) |
| `downloads.claude.ai`          | Descargas de ejecutables de plugins; instalador nativo y actualizador automático nativo                                                                                                                                                                                                                                                                                                                                                                                                   |
| `storage.googleapis.com`       | Recuentos de instalación y metadatos de plugins mostrados en `/plugin`. Las cargas de [artefactos](/docs/es/artifacts) firmadas intentan este host primero; la publicación recurre a `api.anthropic.com` cuando está bloqueado                                                                                                                                                                                                                                                                 |
| `storage.googleapis.com`       | Instalador nativo y actualizador automático nativo en versiones anteriores a 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                      |
| `bridge.claudeusercontent.com` | Puente WebSocket de la extensión [Claude en Chrome](/docs/es/chrome)                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `*.claudeusercontent.com`      | Visualización de [artefactos](/docs/es/artifacts) en claude.ai. El visor carga el contenido de cada artefacto desde un subdominio aislado de este origen. Requerido en el navegador del visor, no por la CLI en sí                                                                                                                                                                                                                                                                             |
| `raw.githubusercontent.com`    | Fuente de registro de cambios para [`/release-notes`](/docs/es/commands) y las notas de la versión mostradas después de actualizar                                                                                                                                                                                                                                                                                                                                                             |

Si instala Claude Code a través de npm o administra su propia distribución binaria, los usuarios finales no necesitan el instalador nativo y los usos del actualizador automático de `downloads.claude.ai`. Los otros usos en la tabla se aplican independientemente del método de instalación.

Claude Code también envía telemetría operativa opcional de forma predeterminada, que puede desactivar con variables de entorno. Consulte [Servicios de telemetría](/docs/es/data-usage#telemetry-services) para saber cómo desactivarla antes de finalizar su lista blanca.

Cuando utiliza [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry) o una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, el tráfico del modelo y la autenticación van a su proveedor o puerta de enlace en lugar de `api.anthropic.com`, `claude.ai` o `platform.claude.com`. La herramienta WebFetch aún llama a `api.anthropic.com` para su [verificación de seguridad de dominio](/docs/es/data-usage#webfetch-domain-safety-check) a menos que establezca `skipWebFetchPreflight: true` en [configuración](/docs/es/settings).

[Claude Code en la web](/docs/es/claude-code-on-the-web) y [Code Review](/docs/es/code-review) se conectan a sus repositorios desde infraestructura administrada por Anthropic. Si su organización de GitHub Enterprise Cloud restringe el acceso por dirección IP, habilite [herencia de lista de permitidos de IP para aplicaciones de GitHub instaladas](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). La aplicación de GitHub de Claude registra sus rangos de IP, por lo que habilitar esta configuración permite el acceso sin configuración manual. Para [agregar los rangos a su lista de permitidos manualmente](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) en su lugar, o para configurar otros firewalls, consulte [direcciones IP de la API de Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Para instancias de [GitHub Enterprise Server](/docs/es/github-enterprise-server) autohospedadas detrás de un firewall, agregue a la lista blanca las mismas [direcciones IP de la API de Anthropic](https://platform.claude.com/docs/en/api/ip-addresses) para que la infraestructura de Anthropic pueda acceder a su host GHES para clonar repositorios y publicar comentarios de revisión.

<h3 id="desktop-and-claude-ai">
  Desktop y claude.ai
</h3>

La tabla anterior cubre principalmente la CLI independiente. La aplicación Claude Desktop y claude.ai en un navegador cargan su código de aplicación desde hosts CDN adicionales de Anthropic, incluido `assets-proxy.anthropic.com`. Permitir `claude.ai` mientras se bloquean esos hosts produce una página en blanco en lugar de un error. Consulte [requisitos de acceso a la red](/docs/es/desktop#network-access-requirements) en la página Desktop.

<h2 id="additional-resources">
  Recursos adicionales
</h2>

* [Configuración de Claude Code](/docs/es/settings)
* [Referencia de variables de entorno](/docs/es/env-vars)
* [Guía de solución de problemas](/docs/es/troubleshooting)
