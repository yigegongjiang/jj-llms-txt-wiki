> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en Slack

> Delega tareas de codificación directamente desde tu espacio de trabajo de Slack

<Note>
  Claude Code en Slack está siendo reemplazado por [Claude Tag](https://claude.com/product/tag) para espacios de trabajo de Team y Enterprise. Claude Tag ejecuta @Claude como la identidad compartida de tu organización con acceso configurado por el administrador, bajo la misma aplicación de Slack, por lo que no hay nada que reinstalar y las configuraciones existentes continúan funcionando durante la transición. Para cambiar un espacio de trabajo, consulta [Migrar desde el Claude anterior en Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
</Note>

Claude Code en Slack trae el poder de Claude Code directamente a tu espacio de trabajo de Slack. Cuando mencionas `@Claude` con una tarea de codificación, Claude detecta automáticamente la intención y crea una sesión de Claude Code en la web, permitiéndote delegar trabajo de desarrollo sin salir de tus conversaciones de equipo.

Esta integración se basa en la aplicación Claude para Slack existente pero agrega enrutamiento inteligente a Claude Code en la web para solicitudes relacionadas con codificación. Cada sesión se ejecuta bajo tu propia cuenta de Claude, utilizando tus repositorios conectados y tus límites de plan.

<h2 id="use-cases">
  Casos de uso
</h2>

* **Investigación y corrección de errores**: Pídele a Claude que investigue y corrija errores tan pronto como se reporten en los canales de Slack.
* **Revisiones de código rápidas y modificaciones**: Haz que Claude implemente pequeñas características o refactorice código basado en comentarios del equipo.
* **Depuración colaborativa**: Cuando las discusiones del equipo proporcionan contexto crucial (por ejemplo, reproducciones de errores o reportes de usuarios), Claude puede usar esa información para informar su enfoque de depuración.
* **Ejecución de tareas en paralelo**: Inicia tareas de codificación en Slack mientras continúas con otro trabajo, recibiendo notificaciones cuando se completen.

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de usar Claude Code en Slack, asegúrese de tener lo siguiente:

| Requisito              | Detalles                                                                                              |
| :--------------------- | :---------------------------------------------------------------------------------------------------- |
| Plan de Claude         | Pro, Max, Team o Enterprise con acceso a Claude Code (asientos premium o asientos Chat + Claude Code) |
| Claude Code en la web  | El acceso a [Claude Code en la web](/docs/es/claude-code-on-the-web) debe estar habilitado                 |
| Cuenta de GitHub       | Conectada a Claude Code en la web con al menos un repositorio autenticado                             |
| Autenticación de Slack | Su cuenta de Slack vinculada a su cuenta de Claude a través de la aplicación Claude                   |

<h2 id="setting-up-claude-code-in-slack">
  Configuración de Claude Code en Slack
</h2>

<Steps>
  <Step title="Instala la aplicación Claude en Slack">
    Un administrador del espacio de trabajo debe instalar la aplicación Claude desde el Slack App Marketplace. Visita el [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) y haz clic en "Add to Slack" para comenzar el proceso de instalación.
  </Step>

  <Step title="Conecta tu cuenta de Claude">
    Después de que la aplicación esté instalada, autentica tu cuenta individual de Claude:

    1. Abre la aplicación Claude en Slack haciendo clic en "Claude" en tu sección de Aplicaciones
    2. Navega a la pestaña App Home
    3. Haz clic en "Connect" para vincular tu cuenta de Slack con tu cuenta de Claude
    4. Completa el flujo de autenticación en tu navegador
  </Step>

  <Step title="Configura Claude Code en la web">
    Asegúrate de que tu Claude Code en la web esté correctamente configurado:

    * Visita [claude.ai/code](https://claude.ai/code) e inicia sesión con la misma cuenta que conectaste a Slack
    * Conecta tu cuenta de GitHub si aún no está conectada
    * Autentica al menos un repositorio con el que quieras que Claude trabaje
  </Step>

  <Step title="Elige tu modo de enrutamiento">
    Después de conectar tus cuentas, configura cómo Claude maneja tus mensajes en Slack. Navega a la App Home de Claude en Slack para encontrar la configuración de **Routing Mode**.

    | Modo            | Comportamiento                                                                                                                                                                                                                                                          |
    | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Code only**   | Claude enruta todas las @menciones a sesiones de Claude Code. Mejor para equipos que usan Claude en Slack exclusivamente para tareas de desarrollo.                                                                                                                     |
    | **Code + Chat** | Claude analiza cada mensaje y enruta inteligentemente entre Claude Code (para tareas de codificación) y Claude Chat (para escritura, análisis y preguntas generales). Mejor para equipos que quieren un único punto de entrada @Claude para todos los tipos de trabajo. |

    <Note>
      En modo Code + Chat, si Claude enruta un mensaje a Chat pero querías una sesión de codificación, puedes hacer clic en "Retry as Code" para crear una sesión de Claude Code en su lugar. De manera similar, si se enruta a Code pero querías una sesión de Chat, puedes elegir esa opción en ese hilo.
    </Note>
  </Step>

  <Step title="Agrega Claude a los canales">
    Claude no se agrega automáticamente a ningún canal después de la instalación. Para usar Claude en un canal, invítalo escribiendo `/invite @Claude` en ese canal. Claude solo puede responder a @menciones en canales donde ha sido agregado.
  </Step>
</Steps>

<h2 id="how-it-works">
  Cómo funciona
</h2>

<h3 id="automatic-detection">
  Detección automática
</h3>

Cuando mencionas @Claude en un canal o hilo de Slack, Claude analiza automáticamente tu mensaje para determinar si es una tarea de codificación. Si Claude detecta intención de codificación, enrutará tu solicitud a Claude Code en la web en lugar de responder como un asistente de chat regular.

También puedes decirle explícitamente a Claude que maneje una solicitud como una tarea de codificación, incluso si no la detecta automáticamente.

<Note>
  Claude Code en Slack solo funciona en canales (públicos o privados). No funciona en mensajes directos (DMs).
</Note>

<h3 id="context-gathering">
  Recopilación de contexto
</h3>

**De hilos**: Cuando @mencionas a Claude en un hilo, recopila contexto de todos los mensajes en ese hilo para entender la conversación completa.

**De canales**: Cuando se menciona directamente en un canal, Claude observa los mensajes recientes del canal para obtener contexto relevante.

Este contexto ayuda a Claude a entender el problema, seleccionar el repositorio apropiado e informar su enfoque para la tarea.

<Warning>
  Cuando @Claude se invoca en Slack, Claude tiene acceso al contexto de la conversación para entender mejor tu solicitud. Claude puede seguir direcciones de otros mensajes en el contexto, por lo que los usuarios deben asegurarse de usar Claude solo en conversaciones de Slack de confianza.
</Warning>

<h3 id="session-flow">
  Flujo de sesión
</h3>

1. **Iniciación**: @mencionas a Claude con una solicitud de codificación
2. **Detección**: Claude analiza tu mensaje y detecta intención de codificación
3. **Creación de sesión**: Se crea una nueva sesión de Claude Code en claude.ai/code
4. **Actualizaciones de progreso**: Claude publica actualizaciones de estado en tu hilo de Slack a medida que avanza el trabajo
5. **Finalización**: Cuando termina, Claude te @menciona con un resumen y botones de acción
6. **Revisión**: Haz clic en "View Session" para ver la transcripción completa, o "Create PR" para abrir una solicitud de extracción

<h2 id="user-interface-elements">
  Elementos de la interfaz de usuario
</h2>

<h3 id="app-home">
  App Home
</h3>

La pestaña App Home muestra tu estado de conexión y te permite conectar o desconectar tu cuenta de Claude de Slack.

<h3 id="message-actions">
  Acciones de mensaje
</h3>

* **View Session**: Abre la sesión completa de Claude Code en tu navegador donde puedes ver todo el trabajo realizado, continuar la sesión o hacer solicitudes adicionales.
* **Create PR**: Crea una solicitud de extracción directamente desde los cambios de la sesión.
* **Retry as Code**: Si Claude inicialmente responde como un asistente de chat pero querías una sesión de codificación, haz clic en este botón para reintentar la solicitud como una tarea de Claude Code.
* **Change Repo**: Te permite seleccionar un repositorio diferente si Claude eligió incorrectamente.

<h3 id="repository-selection">
  Selección de repositorio
</h3>

Claude selecciona automáticamente un repositorio basado en el contexto de tu conversación de Slack. Si múltiples repositorios podrían aplicarse, Claude puede mostrar un menú desplegable permitiéndote elegir el correcto.

<h2 id="access-and-permissions">
  Acceso y permisos
</h2>

<h3 id="user-level-access">
  Acceso a nivel de usuario
</h3>

| Tipo de acceso             | Requisito                                                                       |
| :------------------------- | :------------------------------------------------------------------------------ |
| Sesiones de Claude Code    | Cada usuario ejecuta sesiones bajo su propia cuenta de Claude                   |
| Uso y límites de velocidad | Las sesiones cuentan contra los límites del plan del usuario individual         |
| Acceso al repositorio      | Los usuarios solo pueden acceder a repositorios que han conectado personalmente |
| Historial de sesiones      | Las sesiones aparecen en tu historial de Claude Code en claude.ai/code          |

<h3 id="workspace-level-access">
  Acceso a nivel del espacio de trabajo
</h3>

Los administradores del espacio de trabajo de Slack controlan si la aplicación Claude está disponible en su espacio de trabajo:

| Control                         | Descripción                                                                                                                                                  |
| :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Instalación de la aplicación    | Los administradores del espacio de trabajo deciden si instalar la aplicación Claude desde el Slack App Marketplace                                           |
| Distribución de Enterprise Grid | Para organizaciones de Enterprise Grid, los administradores de la organización pueden controlar qué espacios de trabajo tienen acceso a la aplicación Claude |
| Eliminación de la aplicación    | Eliminar la aplicación de un espacio de trabajo revoca inmediatamente el acceso para todos los usuarios en ese espacio de trabajo                            |

<h3 id="channel-based-access-control">
  Control de acceso basado en canales
</h3>

Claude no se agrega automáticamente a ningún canal después de la instalación. Los usuarios deben invitar explícitamente a Claude a los canales donde quieren usarlo:

* **Invitación requerida**: Escribe `/invite @Claude` en cualquier canal para agregar Claude a ese canal
* **La membresía del canal controla el acceso**: Claude solo puede responder a @menciones en canales donde ha sido agregado
* **Control de acceso a través de canales**: Los administradores pueden controlar quién usa Claude Code administrando qué canales Claude es invitado y quién tiene acceso a esos canales
* **Soporte de canal privado**: Claude funciona en canales públicos y privados, dando a los equipos flexibilidad para controlar la visibilidad

Este modelo basado en canales permite a los equipos restringir el uso de Claude Code a canales específicos, proporcionando una capa adicional de control de acceso más allá de los permisos a nivel del espacio de trabajo.

<h2 id="what’s-accessible-where">
  Qué es accesible dónde
</h2>

**En Slack**: Verás actualizaciones de estado, resúmenes de finalización y botones de acción. La transcripción completa se conserva y siempre es accesible.

**En la web**: La sesión completa de Claude Code con historial de conversación completo, todos los cambios de código, operaciones de archivo y la capacidad de continuar la sesión o crear solicitudes de extracción.

Para cuentas Enterprise y Team, las sesiones creadas desde Claude en Slack son
automáticamente visibles para la organización. Consulta [Compartir sesiones de Claude Code en la web](/docs/es/claude-code-on-the-web#share-sessions)
para más detalles.

<h2 id="best-practices">
  Mejores prácticas
</h2>

<h3 id="writing-effective-requests">
  Escribir solicitudes efectivas
</h3>

* **Sé específico**: Incluye nombres de archivos, nombres de funciones o mensajes de error cuando sea relevante.
* **Proporciona contexto**: Menciona el repositorio o proyecto si no está claro en la conversación.
* **Define el éxito**: Explica qué significa "hecho"—¿debería Claude escribir pruebas? ¿Actualizar documentación? ¿Crear un PR?
* **Usa hilos**: Responde en hilos cuando discutas errores o características para que Claude pueda recopilar el contexto completo.

<h3 id="when-to-use-slack-vs-web">
  Cuándo usar Slack vs. web
</h3>

**Usa Slack cuando**: El contexto ya existe en una discusión de Slack, quieres iniciar una tarea de forma asincrónica, o estás colaborando con compañeros de equipo que necesitan visibilidad.

**Usa la web directamente cuando**: Necesitas cargar archivos, quieres interacción en tiempo real durante el desarrollo, o estás trabajando en tareas más largas y complejas.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  "Claude Code no está habilitado para tu cuenta"
</h3>

Este error significa que tu cuenta de Claude aún no tiene un entorno en la nube, no que un administrador necesite habilitar algo. Inicia sesión en [claude.ai/code](https://claude.ai/code) una vez con la misma cuenta que conectaste a Slack. La primera visita crea tu entorno en la nube predeterminado, y el error se resuelve en tu próxima mención. Cada usuario debe hacer esto individualmente.

<h3 id="sessions-not-starting">
  Las sesiones no se inician
</h3>

1. Verifica que tu cuenta de Claude esté conectada en la App Home de Claude
2. Comprueba que tengas acceso a Claude Code en la web habilitado
3. Asegúrate de tener al menos un repositorio de GitHub conectado a Claude Code

<h3 id="repository-not-showing">
  El repositorio no se muestra
</h3>

1. Conecta el repositorio en Claude Code en la web en [claude.ai/code](https://claude.ai/code)
2. Verifica tus permisos de GitHub para ese repositorio
3. Intenta desconectar y reconectar tu cuenta de GitHub

<h3 id="wrong-repository-selected">
  Se seleccionó el repositorio incorrecto
</h3>

1. Haz clic en el botón "Change Repo" para seleccionar un repositorio diferente
2. Incluye el nombre del repositorio en tu solicitud para una selección más precisa

<h3 id="authentication-errors">
  Errores de autenticación
</h3>

1. Desconecta y reconecta tu cuenta de Claude en la App Home
2. Asegúrate de estar conectado a la cuenta de Claude correcta en tu navegador
3. Comprueba que tu plan de Claude incluya acceso a Claude Code

<h3 id="session-expiration">
  Expiración de sesión
</h3>

1. Las sesiones permanecen accesibles en tu historial de Claude Code en la web
2. Puedes continuar o hacer referencia a sesiones pasadas desde [claude.ai/code](https://claude.ai/code)

<h2 id="current-limitations">
  Limitaciones actuales
</h2>

* **Solo GitHub**: Actualmente admite repositorios en GitHub.
* **Un PR a la vez**: Cada sesión puede crear una solicitud de extracción.
* **Se aplican límites de velocidad**: Las sesiones usan los límites de velocidad del plan de Claude individual.
* **Se requiere acceso web**: Los usuarios deben tener acceso a Claude Code en la web; aquellos sin él solo recibirán respuestas de chat estándar de Claude.

<h2 id="related-resources">
  Recursos relacionados
</h2>

<CardGroup>
  <Card title="Claude Code en la web" icon="globe" href="/docs/es/claude-code-on-the-web">
    Obtén más información sobre Claude Code en la web
  </Card>

  <Card title="Claude para Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Documentación general de Claude para Slack
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    @Claude administrado por la organización en Slack con acceso configurado por el administrador
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Instala la aplicación Claude desde el Slack Marketplace
  </Card>

  <Card title="Centro de ayuda de Claude" icon="circle-question" href="https://support.claude.com">
    Obtén soporte adicional
  </Card>
</CardGroup>
