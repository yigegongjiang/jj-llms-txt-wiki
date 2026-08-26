> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Despliegue seguro de agentes de IA

> Una guía para asegurar despliegues de Claude Code y Agent SDK con aislamiento, gestión de credenciales y controles de red

Claude Code y el Agent SDK son herramientas poderosas que pueden ejecutar código, acceder a archivos e interactuar con servicios externos en su nombre. Como cualquier herramienta con estas capacidades, desplegarlas cuidadosamente garantiza que obtenga los beneficios mientras mantiene controles apropiados.

A diferencia del software tradicional que sigue rutas de código predeterminadas, estas herramientas generan sus acciones dinámicamente basándose en el contexto y los objetivos. Esta flexibilidad es lo que las hace útiles, pero también significa que su comportamiento puede ser influenciado por el contenido que procesan: archivos, páginas web o entrada del usuario. Esto a veces se llama inyección de indicaciones. Por ejemplo, si el README de un repositorio contiene instrucciones inusuales, Claude Code podría incorporar esas instrucciones en sus acciones de formas que el operador no anticipó. Esta guía cubre formas prácticas de reducir este riesgo.

La buena noticia es que asegurar un despliegue de agentes no requiere infraestructura exótica. Los mismos principios que se aplican a ejecutar cualquier código semi-confiable se aplican aquí: aislamiento, privilegio mínimo y defensa en profundidad. Claude Code incluye varias características de seguridad que ayudan con preocupaciones comunes, y esta guía recorre estas junto con opciones de endurecimiento adicionales para quienes las necesiten.

No todos los despliegues necesitan seguridad máxima. Un desarrollador ejecutando Claude Code en su portátil tiene requisitos diferentes que una empresa procesando datos de clientes en un entorno multiinquilino. Esta guía presenta opciones que van desde las características de seguridad integradas de Claude Code hasta arquitecturas de producción endurecidas, para que pueda elegir lo que se ajuste a su situación.

<h2 id="threat-model">
  Modelo de amenaza
</h2>

Los agentes pueden tomar acciones no intencionadas debido a inyección de indicaciones (instrucciones incrustadas en el contenido que procesan) o error del modelo. Los modelos Claude están diseñados para resistir esto; consulte la [descripción general del modelo](https://platform.claude.com/docs/es/about-claude/models/overview) y la tarjeta del sistema para el modelo que despliega para obtener detalles de evaluación.

La defensa en profundidad sigue siendo una buena práctica. Por ejemplo, si un agente procesa un archivo malicioso que le instruye enviar datos de clientes a un servidor externo, los controles de red pueden bloquear esa solicitud completamente.

<h2 id="built-in-security-features">
  Características de seguridad integradas
</h2>

Claude Code incluye varias características de seguridad que abordan preocupaciones comunes. Consulte la [documentación de seguridad](/docs/es/security) para obtener detalles completos.

* **Sistema de permisos**: Cada herramienta y comando bash se puede configurar para permitir, bloquear o solicitar aprobación del usuario. Use patrones glob para crear reglas como "permitir todos los comandos npm" o "bloquear cualquier comando con sudo". Las organizaciones pueden establecer políticas que se apliquen a todos los usuarios. Consulte [permisos](/docs/es/permissions).
* **Análisis de comandos para permisos**: Antes de ejecutar comandos bash, Claude Code los analiza en un AST y compara el resultado con sus reglas de permisos. Los comandos que no se pueden analizar limpiamente, o que no coinciden con una regla de permiso, requieren aprobación explícita. Un pequeño conjunto de construcciones como `eval` siempre requieren aprobación independientemente de las reglas de permiso. Esta es una puerta de permiso, no un sandbox; no deduce si un comando es peligroso a partir de su ruta de destino o efectos.
* **Resumen de búsqueda web**: Los resultados de búsqueda se resumen en lugar de pasar contenido sin procesar directamente al contexto, reduciendo el riesgo de inyección de indicaciones de contenido web malicioso.
* **Modo sandbox**: Los comandos bash pueden ejecutarse en un entorno aislado que restringe el acceso al sistema de archivos y la red. Consulte la [documentación de sandboxing](/docs/es/sandboxing) para obtener detalles.

<h2 id="security-principles">
  Principios de seguridad
</h2>

Para despliegues que requieren endurecimiento adicional más allá de los valores predeterminados de Claude Code, estos principios guían las opciones disponibles.

<h3 id="security-boundaries">
  Límites de seguridad
</h3>

Un límite de seguridad separa componentes con diferentes niveles de confianza. Para despliegues de alta seguridad, puede colocar recursos sensibles (como credenciales) fuera del límite que contiene el agente. Si algo sale mal en el entorno del agente, los recursos fuera de ese límite permanecen protegidos.

Por ejemplo, en lugar de dar a un agente acceso directo a una clave API, podría ejecutar un proxy fuera del entorno del agente que inyecte la clave en las solicitudes. El agente puede hacer llamadas API, pero nunca ve la credencial en sí. Este patrón es útil para despliegues multiinquilino o cuando se procesa contenido no confiable.

<h3 id="least-privilege">
  Privilegio mínimo
</h3>

Cuando sea necesario, puede restringir el agente solo a las capacidades requeridas para su tarea específica:

| Recurso                 | Opciones de restricción                                     |
| ----------------------- | ----------------------------------------------------------- |
| Sistema de archivos     | Montar solo directorios necesarios, preferir solo lectura   |
| Red                     | Restringir a puntos finales específicos a través de proxy   |
| Credenciales            | Inyectar a través de proxy en lugar de exponer directamente |
| Capacidades del sistema | Descartar capacidades de Linux en contenedores              |

<h3 id="defense-in-depth">
  Defensa en profundidad
</h3>

Para entornos de alta seguridad, superponer múltiples controles proporciona protección adicional. Las opciones incluyen:

* Aislamiento de contenedores
* Restricciones de red
* Controles del sistema de archivos
* Validación de solicitudes en un proxy

La combinación correcta depende de su modelo de amenaza y requisitos operacionales.

<h2 id="isolation-technologies">
  Tecnologías de aislamiento
</h2>

Las diferentes tecnologías de aislamiento ofrecen diferentes compensaciones entre fortaleza de seguridad, rendimiento y complejidad operacional.

<Info>
  En todas estas configuraciones, Claude Code (o su aplicación Agent SDK) se ejecuta dentro del límite de aislamiento (el sandbox, contenedor o VM). Los controles de seguridad descritos a continuación restringen lo que el agente puede acceder desde dentro de ese límite.
</Info>

| Tecnología              | Fortaleza de aislamiento                | Sobrecarga de rendimiento | Complejidad |
| ----------------------- | --------------------------------------- | ------------------------- | ----------- |
| Sandbox runtime         | Buena (valores predeterminados seguros) | Muy baja                  | Baja        |
| Contenedores (Docker)   | Depende de la configuración             | Baja                      | Media       |
| gVisor                  | Excelente (con configuración correcta)  | Media/Alta                | Media       |
| VMs (Firecracker, QEMU) | Excelente (con configuración correcta)  | Alta                      | Media/Alta  |

<h3 id="sandbox-runtime">
  Sandbox runtime
</h3>

Para aislamiento ligero sin contenedores, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) aplica restricciones de sistema de archivos y red a nivel del SO.

La principal ventaja es la simplicidad: no se requiere configuración de Docker, imágenes de contenedor o configuración de red. El proxy y las restricciones del sistema de archivos están integrados. Proporciona un archivo de configuración especificando dominios y rutas permitidas.

**Cómo funciona:**

* **Sistema de archivos**: Utiliza primitivos del SO (`bubblewrap` en Linux, `sandbox-exec` en macOS) para restringir el acceso de lectura/escritura a rutas configuradas
* **Red**: Elimina el espacio de nombres de red (Linux) o utiliza perfiles Seatbelt (macOS) para enrutar el tráfico de red a través de un proxy integrado
* **Configuración**: Listas de permitidos basadas en JSON para dominios y rutas del sistema de archivos

**Configuración:**

```bash theme={null}
npm install @anthropic-ai/sandbox-runtime
```

Luego cree un archivo de configuración especificando rutas y dominios permitidos.

**Consideraciones de seguridad:**

1. **Kernel del mismo host**: A diferencia de las VMs, los procesos aislados comparten el kernel del host. Una vulnerabilidad del kernel podría teóricamente permitir escape. Para algunos modelos de amenaza esto es aceptable, pero si necesita aislamiento a nivel de kernel, use gVisor o una VM separada.

2. **Sin inspección TLS**: El proxy lista permitidos dominios basándose en el nombre de host suministrado por el cliente y no termina ni inspecciona tráfico encriptado. El código que se ejecuta dentro del sandbox puede potencialmente usar [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) u técnicas similares para alcanzar hosts fuera de la lista de permitidos. Si su modelo de amenaza requiere garantías más fuertes, configure un [proxy que termina TLS](#traffic-forwarding). Consulte las [limitaciones de seguridad del sandboxing](/docs/es/sandboxing#security-limitations) para obtener más detalles. Por separado, si el agente tiene credenciales permisivas para un dominio permitido, asegúrese de que no pueda usar ese dominio para desencadenar otras solicitudes de red o para exfiltrar datos.

Para muchos casos de uso de desarrollador único y CI/CD, sandbox-runtime eleva significativamente la barra con configuración mínima. Las secciones a continuación cubren contenedores y VMs para despliegues que requieren aislamiento más fuerte.

<h3 id="containers">
  Contenedores
</h3>

Los contenedores proporcionan aislamiento a través de espacios de nombres de Linux. Cada contenedor tiene su propia vista del sistema de archivos, árbol de procesos y pila de red, mientras comparte el kernel del host.

Una configuración de contenedor endurecida por seguridad podría verse así:

```bash theme={null}
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Aquí está lo que hace cada opción:

| Opción                             | Propósito                                                                                                                                                                |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--cap-drop ALL`                   | Elimina capacidades de Linux como `NET_ADMIN` y `SYS_ADMIN` que podrían permitir escalada de privilegios                                                                 |
| `--security-opt no-new-privileges` | Previene que los procesos ganen privilegios a través de binarios setuid                                                                                                  |
| `--security-opt seccomp=...`       | Restringe las llamadas del sistema disponibles; el valor predeterminado de Docker bloquea \~44, los perfiles personalizados pueden bloquear más                          |
| `--read-only`                      | Hace el sistema de archivos raíz del contenedor inmutable, previniendo que el agente persista cambios                                                                    |
| `--tmpfs /tmp:...`                 | Proporciona un directorio temporal escribible que se borra cuando el contenedor se detiene                                                                               |
| `--network none`                   | Elimina todas las interfaces de red; el agente se comunica a través del socket Unix montado a continuación                                                               |
| `--memory 2g`                      | Limita el uso de memoria para prevenir agotamiento de recursos                                                                                                           |
| `--pids-limit 100`                 | Limita el recuento de procesos para prevenir fork bombs                                                                                                                  |
| `--user 1000:1000`                 | Se ejecuta como un usuario no root                                                                                                                                       |
| `-v ...:/workspace:ro`             | Monta código solo lectura para que el agente pueda analizarlo pero no modificarlo. **Evite montar directorios de host sensibles como `~/.ssh`, `~/.aws`, o `~/.config`** |
| `-v .../proxy.sock:...`            | Monta un socket Unix conectado a un proxy ejecutándose fuera del contenedor (ver a continuación)                                                                         |

**Arquitectura de socket Unix:**

Con `--network none`, el contenedor no tiene interfaces de red en absoluto. La única forma para que el agente alcance el mundo exterior es a través del socket Unix montado, que se conecta a un proxy ejecutándose en el host. Este proxy puede aplicar listas de permitidos de dominios, inyectar credenciales y registrar todo el tráfico.

Esta es la misma arquitectura utilizada por [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Incluso si el agente se ve comprometido a través de inyección de indicaciones, no puede exfiltrar datos a servidores arbitrarios. Solo puede comunicarse a través del proxy, que controla qué dominios son alcanzables. Para más detalles, consulte la [publicación del blog de sandboxing de Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opciones de endurecimiento adicionales:**

| Opción           | Propósito                                                                                                                                  |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `--userns-remap` | Asigna root del contenedor a usuario host sin privilegios; requiere configuración del daemon pero limita el daño del escape del contenedor |
| `--ipc private`  | Aísla la comunicación entre procesos para prevenir ataques entre contenedores                                                              |

<h3 id="gvisor">
  gVisor
</h3>

Los contenedores estándar comparten el kernel del host: cuando el código dentro de un contenedor hace una llamada del sistema, va directamente al mismo kernel que ejecuta el host. Esto significa que una vulnerabilidad del kernel podría permitir escape del contenedor. gVisor aborda esto interceptando llamadas del sistema en el espacio de usuario antes de que lleguen al kernel del host, implementando su propia capa de compatibilidad que maneja la mayoría de syscalls sin involucrar el kernel real.

Si un agente ejecuta código malicioso (quizás debido a inyección de indicaciones), ese código se ejecuta en el contenedor y podría intentar exploits del kernel. Con gVisor, la superficie de ataque es mucho más pequeña: el código malicioso necesitaría explotar primero la implementación del espacio de usuario de gVisor y tendría acceso limitado al kernel real.

Para usar gVisor con Docker, instale el runtime `runsc` y configure el daemon:

```json theme={null}
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Luego ejecute contenedores con:

```bash theme={null}
docker run --runtime=runsc agent-image
```

**Consideraciones de rendimiento:**

| Carga de trabajo             | Sobrecarga                                                       |
| ---------------------------- | ---------------------------------------------------------------- |
| Computación vinculada a CPU  | \~0% (sin intercepción de syscall)                               |
| Syscalls simples             | \~2× más lento                                                   |
| Intensivo en E/S de archivos | Hasta 10-200× más lento para patrones pesados de apertura/cierre |

Para entornos multiinquilino o cuando se procesa contenido no confiable, el aislamiento adicional a menudo vale la sobrecarga.

<h3 id="virtual-machines">
  Máquinas virtuales
</h3>

Las VMs proporcionan aislamiento a nivel de hardware a través de extensiones de virtualización de CPU. Cada VM ejecuta su propio kernel, creando un límite fuerte. Una vulnerabilidad en el kernel invitado no compromete directamente el host. Sin embargo, las VMs no son automáticamente "más seguras" que alternativas como gVisor. La seguridad de las VMs depende en gran medida del hipervisor y del código de emulación de dispositivos.

Firecracker está diseñado para aislamiento ligero de microVM. Puede arrancar VMs en menos de 125ms con menos de 5 MiB de sobrecarga de memoria, eliminando emulación de dispositivos innecesaria para reducir la superficie de ataque.

Con este enfoque, la VM del agente no tiene interfaz de red externa. En su lugar, se comunica a través de `vsock` (sockets virtuales). Todo el tráfico se enruta a través de vsock a un proxy en el host, que aplica listas de permitidos e inyecta credenciales antes de reenviar solicitudes.

<h3 id="cloud-deployments">
  Despliegues en la nube
</h3>

Para despliegues en la nube, puede combinar cualquiera de las tecnologías de aislamiento anteriores con controles de red nativos de la nube:

1. Ejecute contenedores de agentes en una subred privada sin puerta de enlace de internet
2. Configure reglas de firewall en la nube (AWS Security Groups, firewall de VPC de GCP) para bloquear todo el tráfico de salida excepto a su proxy
3. Ejecute un proxy (como [Envoy](https://www.envoyproxy.io/) con su filtro `credential_injector`) que valide solicitudes, aplique listas de permitidos de dominios, inyecte credenciales y reenvíe a APIs externas
4. Asigne permisos IAM mínimos a la cuenta de servicio del agente, enrutando acceso sensible a través del proxy cuando sea posible
5. Registre todo el tráfico en el proxy para propósitos de auditoría

<h2 id="credential-management">
  Gestión de credenciales
</h2>

Los agentes a menudo necesitan credenciales para llamar a APIs, acceder a repositorios o interactuar con servicios en la nube. El desafío es proporcionar este acceso sin exponer las credenciales en sí.

<h3 id="the-proxy-pattern">
  El patrón de proxy
</h3>

El enfoque recomendado es ejecutar un proxy fuera del límite de seguridad del agente que inyecte credenciales en solicitudes salientes. El agente envía solicitudes sin credenciales, el proxy las agrega y reenvía la solicitud a su destino.

Este patrón tiene varios beneficios:

1. El agente nunca ve las credenciales reales
2. El proxy puede aplicar una lista de permitidos de puntos finales permitidos
3. El proxy puede registrar todas las solicitudes para auditoría
4. Las credenciales se almacenan en una ubicación segura en lugar de distribuirse a cada agente

<h3 id="configuring-claude-code-to-use-a-proxy">
  Configurar Claude Code para usar un proxy
</h3>

Claude Code admite dos métodos para enrutar solicitudes de muestreo a través de un proxy:

**Opción 1: ANTHROPIC\_BASE\_URL (simple pero solo para solicitudes de API de muestreo)**

```bash theme={null}
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Esto le dice a Claude Code y al Agent SDK que envíen solicitudes de muestreo a su proxy en lugar de directamente a la API de Claude. Su proxy recibe solicitudes HTTP en texto plano, puede inspeccionarlas y modificarlas (incluyendo inyectar credenciales), luego reenvía a la API real.

**Opción 2: HTTP\_PROXY / HTTPS\_PROXY (en todo el sistema)**

```bash theme={null}
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code y el Agent SDK respetan estas variables de entorno estándar, enrutando todo el tráfico HTTP a través del proxy. Para HTTPS, el proxy crea un túnel CONNECT encriptado: no puede ver o modificar contenidos de solicitudes sin intercepción TLS.

<h3 id="implementing-a-proxy">
  Implementar un proxy
</h3>

Puede construir su propio proxy o usar uno existente:

* [Envoy Proxy](https://www.envoyproxy.io/): proxy de grado de producción con filtro `credential_injector` para agregar encabezados de autenticación
* [mitmproxy](https://mitmproxy.org/): proxy que termina TLS para inspeccionar y modificar tráfico HTTPS
* [Squid](http://www.squid-cache.org/): proxy de caché con listas de control de acceso
* [LiteLLM](https://github.com/BerriAI/litellm): puerta de enlace LLM con inyección de credenciales y limitación de velocidad

<h3 id="credentials-for-other-services">
  Credenciales para otros servicios
</h3>

Más allá del muestreo de la API de Claude, los agentes a menudo necesitan acceso autenticado a otros servicios, como repositorios git, bases de datos y APIs internas. Hay dos enfoques principales:

<h4 id="custom-tools">
  Herramientas personalizadas
</h4>

Proporcione acceso a través de un servidor MCP o herramienta personalizada que enrute solicitudes a un servicio ejecutándose fuera del límite de seguridad del agente. El agente llama a la herramienta, pero la solicitud autenticada real ocurre fuera. Las llamadas de herramienta van a un proxy que inyecta las credenciales.

Por ejemplo, un servidor MCP de git podría aceptar comandos del agente pero reenviarlos a un proxy de git ejecutándose en el host, que agrega autenticación antes de contactar al repositorio remoto. El agente nunca ve las credenciales.

Ventajas:

* **Sin intercepción TLS**: El servicio externo hace solicitudes autenticadas directamente
* **Las credenciales permanecen fuera**: El agente solo ve la interfaz de herramienta, no las credenciales subyacentes

<h4 id="traffic-forwarding">
  Reenvío de tráfico
</h4>

Para llamadas de API de Claude, `ANTHROPIC_BASE_URL` le permite enrutar solicitudes a un proxy que puede inspeccionarlas y modificarlas en texto plano. Pero para otros servicios HTTPS (GitHub, registros npm, APIs internas), el tráfico a menudo está encriptado de extremo a extremo. Incluso si lo enruta a través de un proxy a través de `HTTP_PROXY`, el proxy solo ve un túnel TLS opaco y no puede inyectar credenciales.

Para modificar tráfico HTTPS a servicios arbitrarios, sin usar una herramienta personalizada, necesita un proxy que termina TLS que desencripte tráfico, lo inspeccione o modifique, luego lo reencripte antes de reenviarlo. Esto requiere:

1. Ejecutar el proxy fuera del contenedor del agente
2. Instalar el certificado CA del proxy en el almacén de confianza del agente (para que el agente confíe en los certificados del proxy)
3. Configurar `HTTP_PROXY`/`HTTPS_PROXY` para enrutar tráfico a través del proxy

Este enfoque maneja cualquier servicio basado en HTTP sin escribir herramientas personalizadas, pero agrega complejidad alrededor de la gestión de certificados.

Tenga en cuenta que no todos los programas respetan `HTTP_PROXY`/`HTTPS_PROXY`. La mayoría de herramientas (curl, pip, npm, git) lo hacen, pero algunas pueden omitir estas variables y conectarse directamente. Por ejemplo, `fetch()` de Node.js ignora estas variables por defecto; en Node 24+ puede establecer `NODE_USE_ENV_PROXY=1` para habilitar soporte. Para cobertura completa, puede usar [proxychains](https://github.com/haad/proxychains) para interceptar llamadas de red, o configurar iptables para redirigir tráfico saliente a un proxy transparente.

<Info>
  Un **proxy transparente** intercepta tráfico a nivel de red, por lo que el cliente no necesita ser configurado para usarlo. Los proxies regulares requieren que los clientes se conecten explícitamente y hablen HTTP CONNECT o SOCKS. Los proxies transparentes (como Squid o mitmproxy en modo transparente) pueden manejar conexiones TCP sin procesar redirigidas.
</Info>

Ambos enfoques aún requieren el proxy que termina TLS y certificado CA confiable. Solo aseguran que el tráfico realmente alcance el proxy.

<h2 id="filesystem-configuration">
  Configuración del sistema de archivos
</h2>

Los controles del sistema de archivos determinan qué archivos el agente puede leer y escribir.

<h3 id="read-only-code-mounting">
  Montaje de código solo lectura
</h3>

Cuando el agente necesita analizar código pero no modificarlo, monte el directorio solo lectura:

```bash theme={null}
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
  Incluso el acceso solo lectura a un directorio de código puede exponer credenciales. Archivos comunes a excluir o sanitizar antes de montar:

  | Archivo                                                 | Riesgo                                             |
  | ------------------------------------------------------- | -------------------------------------------------- |
  | `.env`, `.env.local`                                    | Claves API, contraseñas de base de datos, secretos |
  | `~/.git-credentials`                                    | Contraseñas/tokens de Git en texto plano           |
  | `~/.aws/credentials`                                    | Claves de acceso de AWS                            |
  | `~/.config/gcloud/application_default_credentials.json` | Tokens ADC de Google Cloud                         |
  | `~/.azure/`                                             | Credenciales de CLI de Azure                       |
  | `~/.docker/config.json`                                 | Tokens de autenticación de registro de Docker      |
  | `~/.kube/config`                                        | Credenciales de clúster de Kubernetes              |
  | `.npmrc`, `.pypirc`                                     | Tokens de registro de paquetes                     |
  | `*-service-account.json`                                | Claves de cuenta de servicio de GCP                |
  | `*.pem`, `*.key`                                        | Claves privadas                                    |

  Considere copiar solo los archivos de origen necesarios, o usar filtrado de estilo `.dockerignore`.
</Warning>

<h3 id="writable-locations">
  Ubicaciones escribibles
</h3>

Si el agente necesita escribir archivos, tiene algunas opciones dependiendo de si desea que los cambios persistan:

Para espacios de trabajo efímeros en contenedores, use montajes `tmpfs` que existen solo en memoria y se borran cuando el contenedor se detiene:

```bash theme={null}
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Si desea revisar cambios antes de persistirlos, un sistema de archivos de superposición permite que el agente escriba sin modificar archivos subyacentes. Los cambios se almacenan en una capa separada que puede inspeccionar, aplicar o descartar. Para salida completamente persistente, monte un volumen dedicado pero manténgalo separado de directorios sensibles.

<h2 id="further-reading">
  Lectura adicional
</h2>

* [Documentación de seguridad de Claude Code](/docs/es/security)
* [Alojamiento del Agent SDK](/docs/es/agent-sdk/hosting)
* [Manejo de permisos](/docs/es/agent-sdk/permissions)
* [Sandbox runtime](https://github.com/anthropic-experimental/sandbox-runtime)
* [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
* [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
* [Docker Security Best Practices](https://docs.docker.com/engine/security/)
* [gVisor Documentation](https://gvisor.dev/docs/)
* [Firecracker Documentation](https://firecracker-microvm.github.io/)
