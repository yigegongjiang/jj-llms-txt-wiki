> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Seguridad

> Aprenda sobre las medidas de seguridad de Claude Code y las mejores prácticas para un uso seguro.

<h2 id="how-we-approach-security">
  Cómo abordamos la seguridad
</h2>

<h3 id="security-foundation">
  Fundamento de seguridad
</h3>

La seguridad de su código es primordial. Claude Code está construido con la seguridad en su núcleo, desarrollado de acuerdo con el programa de seguridad integral de Anthropic. Obtenga más información y acceda a recursos (informe SOC 2 Type 2, certificado ISO 27001, etc.) en [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Arquitectura basada en permisos
</h3>

En modo Manual, Claude Code comienza con permisos de solo lectura. Cuando Claude Code necesita editar archivos, ejecutar pruebas o ejecutar comandos, le pregunta primero, y usted elige si aprobar la acción una sola vez o permitirla a partir de entonces.

En modo Manual, Claude Code también pregunta antes de ejecutar comandos Bash que pueden modificar su sistema. Ejecuta un conjunto integrado de [comandos de solo lectura](/docs/es/permissions#read-only-commands) como `ls`, `cat`, y `git status` sin preguntar. Usted y su organización configuran estos permisos directamente.

En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), un modelo clasificador separado revisa las acciones en lugar de usted y bloquea las que juzga inseguras. [Cómo el clasificador evalúa las acciones](/docs/es/permission-modes#how-the-classifier-evaluates-actions) enumera qué acciones Claude Code aprueba directamente, cuáles envía al clasificador y cuáles Claude Code aún le pregunta. Sus reglas explícitas de solicitud y denegación aún se aplican, y su organización puede [desactivar el modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode).

El modo de permiso en el que comienza una sesión depende de su plan, la superficie desde la que la inicia y su configuración y la de su organización; consulte [Permission modes](/docs/es/permission-modes#which-mode-a-session-starts-in).

Para la configuración detallada de permisos, consulte [Permissions](/docs/es/permissions).

<h3 id="built-in-protections">
  Protecciones integradas
</h3>

Para mitigar riesgos en sistemas agénticos:

* **Herramienta bash en sandbox**: [Sandbox](/docs/es/sandboxing) comandos bash con aislamiento del sistema de archivos y red, reduciendo solicitudes de permiso mientras se mantiene la seguridad. Configure con `/sandbox` para definir límites donde Claude Code puede trabajar de forma autónoma
* **Límite de directorio de trabajo**: En modo Manual, Claude Code solo puede escribir en la carpeta donde se inició y sus subcarpetas, y no puede modificar archivos en directorios principales sin permiso explícito. En modo Manual, Claude Code también le pregunta antes de leer rutas fuera de este límite con las herramientas Read, Grep y Glob. Extienda el límite con [directorios adicionales](/docs/es/permissions#working-directories) para omitir la solicitud, o restrinja el acceso de lectura más amplio disponible para comandos Bash de solo lectura con [reglas de sandbox `denyRead`](/docs/es/sandboxing#filesystem-isolation), que se aplican solo cuando el sandbox está habilitado
* **Mitigación de fatiga de solicitudes**: Soporte para listas de permitidos de comandos seguros frecuentemente utilizados por usuario, por base de código u por organización
* **Modo Aceptar Ediciones**: Aprueba automáticamente ediciones de archivos y un conjunto fijo de comandos Bash del sistema de archivos como `mkdir`, `touch`, `rm`, `mv`, `cp`, y `sed` para rutas en el directorio de trabajo. Otros comandos Bash y rutas fuera del alcance aún solicitan aprobación

<h3 id="user-responsibility">
  Responsabilidad del usuario
</h3>

Claude Code solo tiene los permisos que usted le otorga. Usted es responsable de revisar el código y los comandos propuestos para verificar su seguridad antes de aprobarlos.

<h2 id="protect-against-prompt-injection">
  Protéjase contra la inyección de solicitudes
</h2>

La inyección de solicitudes es una técnica donde un atacante intenta anular o manipular las instrucciones de un asistente de IA insertando texto malicioso. Claude Code incluye varias medidas de protección contra estos ataques:

<h3 id="core-protections">
  Protecciones principales
</h3>

* **Sistema de permisos**: En modo Manual, las operaciones sensibles requieren aprobación explícita
* **Análisis consciente del contexto**: Detecta instrucciones potencialmente dañinas analizando la solicitud completa
* **Sanitización de entrada**: Previene la inyección de comandos procesando entradas del usuario
* **Aprobación de comandos de red**: Los comandos que obtienen contenido de la web como `curl` y `wget` no se aprueban automáticamente de forma predeterminada. En modo Manual solicitan aprobación como cualquier otro comando Bash que no sea de solo lectura, por lo que aún puede aprobarlos una vez o agregar una regla de permiso explícita como `Bash(curl *)`. Para detenerlos, agréguelos a [`permissions.deny`](/docs/es/permissions#tool-specific-permission-rules). Una regla de denegación coincide con el comando [tal como está escrito](/docs/es/permissions#bash-rule-limits); para la aplicación de red que no depende del texto del comando, consulte [aislamiento de red de sandbox](/docs/es/sandboxing#network-isolation)

<h3 id="privacy-safeguards">
  Medidas de protección de privacidad
</h3>

Hemos implementado varias medidas de protección para proteger sus datos, incluyendo:

* Períodos de retención limitados para información sensible (consulte el [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) para obtener más información)
* Acceso restringido a datos de sesión del usuario
* Control del usuario sobre preferencias de entrenamiento de datos. Los usuarios de consumidor pueden cambiar su [configuración de privacidad](https://claude.ai/settings/privacy) en cualquier momento.

Para obtener detalles completos, consulte nuestros [Términos de Servicio Comerciales](https://www.anthropic.com/legal/commercial-terms) (para usuarios de Team, Enterprise y API) o [Términos de Consumidor](https://www.anthropic.com/legal/consumer-terms) (para usuarios de Free, Pro y Max) y [Política de Privacidad](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Medidas de protección adicionales
</h3>

* **Aprobación de solicitudes de red**: En modo Manual, la mayoría de las herramientas que realizan solicitudes de red requieren aprobación del usuario de forma predeterminada
* **Ventanas de contexto aisladas**: La obtención web utiliza una ventana de contexto separada para evitar inyectar solicitudes potencialmente maliciosas
* **Verificación de confianza**: Las primeras ejecuciones de base de código y los nuevos servidores MCP requieren verificación de confianza
  * Nota: La verificación de confianza está deshabilitada cuando se ejecuta de forma no interactiva con la bandera `-p`
  * Nota: Cuando inicia Claude Code directamente en su directorio de inicio, la aceptación de confianza se mantiene solo para la sesión actual y no se escribe en el disco, por lo que el mensaje reaparece en cada lanzamiento. No hay ninguna configuración para persistirlo. Inicie Claude Code desde un subdirectorio de proyecto en su lugar, donde la aceptación de confianza se guarda por directorio
* **Detección de inyección de comandos**: En modo Manual, los comandos bash sospechosos requieren aprobación manual incluso si fueron permitidos previamente
* **Coincidencia de cierre seguro**: En modo Manual, los comandos no coincidentes requieren aprobación de forma predeterminada
* **Descripciones en lenguaje natural**: Los comandos bash complejos incluyen explicaciones para la comprensión del usuario
* **Almacenamiento seguro de credenciales**: Las claves API y tokens se almacenan en el Keychain de macOS cuando está disponible, y están protegidos por permisos de archivo en Windows y Linux. Consulte [Credential Management](/docs/es/authentication#credential-management)

<Warning>
  **Riesgo de seguridad de WebDAV en Windows**: Cuando ejecute Claude Code en Windows, le recomendamos que no habilite WebDAV ni permita que Claude Code acceda a rutas como `\\*` que pueden contener subdirectorios de WebDAV. [WebDAV ha sido deprecado por Microsoft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) debido a riesgos de seguridad. Habilitar WebDAV puede permitir que Claude Code desencadene solicitudes de red a hosts remotos, eludiendo el sistema de permisos.
</Warning>

**Mejores prácticas para trabajar con contenido no confiable**:

1. Revise los comandos sugeridos antes de aprobarlos
2. Evite canalizar contenido no confiable directamente a Claude
3. Verifique los cambios propuestos en archivos críticos
4. Utilice máquinas virtuales (VMs) para ejecutar scripts y realizar llamadas de herramientas, especialmente cuando interactúe con servicios web externos
5. Reporte comportamiento sospechoso con `/feedback`

<Warning>
  Aunque estas protecciones reducen significativamente el riesgo, ningún sistema es completamente
  inmune a todos los ataques. Siempre mantenga buenas prácticas de seguridad cuando trabaje
  con cualquier herramienta de IA.
</Warning>

<h2 id="mcp-security">
  Seguridad de MCP
</h2>

Claude Code permite a los usuarios configurar servidores del Protocolo de Contexto del Modelo (MCP). La lista de servidores MCP permitidos se configura en su código fuente, como parte de la configuración de Claude Code que los ingenieros verifican en el control de versiones.

Le recomendamos que escriba sus propios servidores MCP o utilice servidores MCP de proveedores en los que confíe. Puede configurar permisos de Claude Code para servidores MCP. Anthropic revisa los conectores según sus [criterios de listado](https://claude.com/docs/connectors/building/review-criteria) antes de agregarlos al [Directorio de Anthropic](https://claude.ai/directory), pero no realiza auditorías de seguridad ni gestiona ningún servidor MCP.

<h2 id="ide-security">
  Seguridad del IDE
</h2>

Consulte [Seguridad y privacidad de VS Code](/docs/es/vs-code#security-and-privacy) para obtener más información sobre cómo ejecutar Claude Code en un IDE.

<h2 id="cloud-execution-security">
  Seguridad de ejecución en la nube
</h2>

Cuando utiliza [Claude Code en la web](/docs/es/claude-code-on-the-web), hay controles de seguridad adicionales en su lugar. Las sesiones que su organización enruta a un [entorno autohospedado](/docs/es/self-hosted-environments) se ejecutan en su propia infraestructura, donde el aislamiento, el egreso de red y las credenciales de git son responsabilidad de su implementación. En entornos alojados por Anthropic:

* **Máquinas virtuales aisladas**: Cada sesión en la nube se ejecuta en una VM aislada gestionada por Anthropic
* **Controles de acceso a la red**: El acceso a la red está limitado de forma predeterminada y se puede configurar para deshabilitarse o permitir solo dominios específicos
* **Protección de credenciales**: La autenticación se maneja a través de un proxy seguro que utiliza una credencial con alcance dentro del sandbox, que luego se traduce a su token de autenticación de GitHub real
* **Restricciones de rama**: Las operaciones de inserción de Git están restringidas a la rama de trabajo actual
* **Registro de auditoría**: Todas las operaciones en sesiones en la nube se registran para fines de cumplimiento y auditoría
* **Limpieza automática**: Los VMs de sesión se recuperan después de un período de inactividad

Para obtener más detalles sobre la ejecución en la nube, consulte [Claude Code en la web](/docs/es/claude-code-on-the-web); para configurar el acceso a la red para sesiones en la nube, consulte [Configurar entornos en la nube](/docs/es/cloud-environments#network-access).

Las sesiones de [Remote Control](/docs/es/remote-control) funcionan de manera diferente: la interfaz web se conecta a un proceso de Claude Code que se ejecuta en su máquina local. Toda la ejecución de código y el acceso a archivos permanecen locales, y el tráfico de sesión viaja a través de la API de Anthropic sobre TLS; mientras está conectado, la transcripción de la sesión se almacena en los servidores de Anthropic para sincronizar la conversación entre dispositivos, como se describe en [Conexión y seguridad](/docs/es/remote-control#connection-and-security). No hay VMs en la nube ni sandboxing involucrados. La conexión utiliza múltiples credenciales de corta duración y alcance estrecho, cada una limitada a un propósito específico y expirando independientemente, para limitar el radio de explosión de cualquier credencial comprometida.

<h2 id="security-best-practices">
  Mejores prácticas de seguridad
</h2>

<h3 id="working-with-sensitive-code">
  Trabajar con código sensible
</h3>

* Revise todos los cambios sugeridos antes de aprobarlos
* Utilice configuración de permisos específica del proyecto para repositorios sensibles
* Considere utilizar [dev containers](/docs/es/devcontainer) para aislamiento adicional
* Audite regularmente su configuración de permisos con `/permissions`

<h3 id="team-security">
  Seguridad del equipo
</h3>

* Utilice [configuración gestionada](/docs/es/settings#where-settings-live) para aplicar estándares organizacionales
* Comparta configuraciones de permisos aprobadas a través del control de versiones
* Capacite a los miembros del equipo sobre mejores prácticas de seguridad
* Monitoree el uso de Claude Code a través de [métricas de OpenTelemetry](/docs/es/monitoring-usage)
* Audite o bloquee cambios de configuración durante sesiones con [hooks `ConfigChange`](/docs/es/hooks#configchange)

<h3 id="reporting-security-issues">
  Reportar problemas de seguridad
</h3>

Si descubre una vulnerabilidad de seguridad en Claude Code:

1. No la divulgue públicamente
2. Repórtela a través de nuestro [programa HackerOne](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)
3. Incluya pasos de reproducción detallados
4. Permita tiempo para que abordemos el problema antes de la divulgación pública

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Plugin de orientación de seguridad](/docs/es/security-guidance): haga que Claude revise y corrija vulnerabilidades en sus propios cambios de código durante la sesión
* [`/security-review`](/docs/es/commands#all-commands): ejecute un análisis de seguridad bajo demanda sobre los cambios en su rama actual
* [Entornos sandbox](/docs/es/sandbox-environments): compare enfoques de aislamiento y elija uno para su modelo de amenaza
* [Sandboxing](/docs/es/sandboxing): aislamiento del sistema de archivos y red para comandos Bash
* [Permissions](/docs/es/permissions): configure permisos y controles de acceso
* [Monitoring usage](/docs/es/monitoring-usage): rastree y audite la actividad de Claude Code
* [Development containers](/docs/es/devcontainer): entornos seguros y aislados
* [Anthropic Trust Center](https://trust.anthropic.com): certificaciones de seguridad y cumplimiento
* [CISO's guide to agentic AI](https://claude.com/blog/ciso-guide-to-agentic-ai): marco de un líder de seguridad para evaluar implementaciones de IA agéntica
