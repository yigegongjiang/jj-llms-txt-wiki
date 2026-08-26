> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Enviar eventos a una sesión en ejecución con channels

> Utilice channels para enviar mensajes, alertas y webhooks a su sesión de Claude Code desde un servidor MCP. Reenvíe resultados de CI, mensajes de chat y eventos de monitoreo para que Claude pueda reaccionar mientras está fuera.

<Note>
  Los channels están en [vista previa de investigación](#research-preview). Requieren autenticación de Anthropic a través de claude.ai o una clave API de Console, y no están disponibles en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Las organizaciones de Team y Enterprise deben [habilitarlos explícitamente](#enterprise-controls).
</Note>

Un channel es un servidor MCP que envía eventos a su sesión de Claude Code en ejecución, para que Claude pueda reaccionar a cosas que suceden mientras no está en la terminal. Los channels pueden ser bidireccionales: Claude lee el evento y responde a través del mismo channel, como un puente de chat. Los eventos solo llegan mientras la sesión está abierta, por lo que para una configuración siempre activa, ejecuta Claude en un proceso de fondo o terminal persistente.

A diferencia de las integraciones que generan una sesión en la nube nueva o esperan a ser consultadas, el evento llega a la sesión que ya tiene abierta: vea [cómo se comparan los channels](#how-channels-compare).

Instala un channel como un plugin y lo configura con tus propias credenciales. Telegram, Discord e iMessage se incluyen en la vista previa de investigación.

Cuando Claude responde a través de un channel, ve el mensaje entrante en su terminal pero no el texto de respuesta. La terminal muestra la llamada de herramienta y una confirmación (como "enviado"), y la respuesta real aparece en la otra plataforma.

Si administra una organización de Team, Enterprise o Console, consulte [Habilitar channels para su organización](#enterprise-controls). Para crear su propio channel, consulte la [referencia de Channels](/docs/es/channels-reference).

<h2 id="supported-channels">
  Canales compatibles
</h2>

Cada canal compatible es un plugin que requiere [Bun](https://bun.sh). Para una demostración práctica del flujo de plugins antes de conectar una plataforma real, pruebe el [inicio rápido de fakechat](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Vea el [código fuente completo de Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram).

    <Steps>
      <Step title="Crear un bot de Telegram">
        Abra [BotFather](https://t.me/BotFather) en Telegram y envíe `/newbot`. Asígnele un nombre para mostrar y un nombre de usuario único que termine en `bot`. Copie el token que devuelve BotFather.
      </Step>

      <Step title="Instalar el plugin">
        En Claude Code, ejecute:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Si Claude Code informa que el plugin no se encuentra en ningún marketplace, su marketplace falta o está desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha agregado antes. Luego reintente la instalación.

        Después de instalar, ejecute `/reload-plugins` para activar el comando de configuración del plugin.
      </Step>

      <Step title="Configurar su token">
        Ejecute el comando de configuración con el token de BotFather:

        ```
        /telegram:configure <token>
        ```

        Esto lo guarda en `~/.claude/channels/telegram/.env`. También puede establecer `TELEGRAM_BOT_TOKEN` en su entorno de shell antes de lanzar Claude Code.
      </Step>

      <Step title="Reiniciar con canales habilitados">
        Salga de Claude Code y reinicie con la bandera de canal. Esto inicia el plugin de Telegram, que comienza a sondear mensajes de su bot:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Emparejar su cuenta">
        Abra Telegram y envíe cualquier mensaje a su bot. El bot responde con un código de emparejamiento.

        <Note>Si su bot no responde, asegúrese de que Claude Code se esté ejecutando con `--channels` del paso anterior. El bot solo puede responder mientras el canal está activo.</Note>

        De vuelta en Claude Code, ejecute:

        ```
        /telegram:access pair <code>
        ```

        Luego bloquee el acceso para que solo su cuenta pueda enviar mensajes:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Vea el [código fuente completo de Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord).

    <Steps>
      <Step title="Crear un bot de Discord">
        Vaya al [Portal de Desarrolladores de Discord](https://discord.com/developers/applications), haga clic en **Nueva Aplicación** y asígnele un nombre. En la sección **Bot**, cree un nombre de usuario y luego haga clic en **Restablecer Token** y copie el token.
      </Step>

      <Step title="Habilitar Intención de Contenido de Mensaje">
        En la configuración de su bot, desplácese hasta **Intenciones de Puerta de Enlace Privilegiadas** y habilite **Intención de Contenido de Mensaje**.
      </Step>

      <Step title="Invitar el bot a su servidor">
        Vaya a **OAuth2 > Generador de URL**. Seleccione el alcance `bot` y habilite estos permisos:

        * Ver Canales
        * Enviar Mensajes
        * Enviar Mensajes en Hilos
        * Leer Historial de Mensajes
        * Adjuntar Archivos
        * Agregar Reacciones

        Abra la URL generada para agregar el bot a su servidor.
      </Step>

      <Step title="Instalar el plugin">
        En Claude Code, ejecute:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Si Claude Code informa que el plugin no se encuentra en ningún marketplace, su marketplace falta o está desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha agregado antes. Luego reintente la instalación.

        Después de instalar, ejecute `/reload-plugins` para activar el comando de configuración del plugin.
      </Step>

      <Step title="Configurar su token">
        Ejecute el comando de configuración con el token del bot que copió:

        ```
        /discord:configure <token>
        ```

        Esto lo guarda en `~/.claude/channels/discord/.env`. También puede establecer `DISCORD_BOT_TOKEN` en su entorno de shell antes de lanzar Claude Code.
      </Step>

      <Step title="Reiniciar con canales habilitados">
        Salga de Claude Code y reinicie con la bandera de canal. Esto conecta el plugin de Discord para que su bot pueda recibir y responder a mensajes:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Emparejar su cuenta">
        Envíe un mensaje directo a su bot en Discord. El bot responde con un código de emparejamiento.

        <Note>Si su bot no responde, asegúrese de que Claude Code se esté ejecutando con `--channels` del paso anterior. El bot solo puede responder mientras el canal está activo.</Note>

        De vuelta en Claude Code, ejecute:

        ```
        /discord:access pair <code>
        ```

        Luego bloquee el acceso para que solo su cuenta pueda enviar mensajes:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Vea el [código fuente completo de iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

    El canal de iMessage lee su base de datos de Mensajes directamente y envía respuestas a través de AppleScript. Requiere macOS y no necesita token de bot ni servicio externo.

    <Steps>
      <Step title="Otorgar Acceso Completo al Disco">
        La base de datos de Mensajes en `~/Library/Messages/chat.db` está protegida por macOS. La primera vez que el servidor la lee, macOS solicita acceso: haga clic en **Permitir**. El mensaje nombra cualquier aplicación que haya lanzado Bun, como Terminal, iTerm o su IDE.

        Si el mensaje no aparece o hizo clic en No Permitir, otorgue acceso manualmente en **Configuración del Sistema > Privacidad y Seguridad > Acceso Completo al Disco** y agregue su terminal. Sin esto, el servidor se cierra inmediatamente con `authorization denied`.
      </Step>

      <Step title="Instalar el plugin">
        En Claude Code, ejecute:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Si Claude Code informa que el plugin no se encuentra en ningún marketplace, su marketplace falta o está desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha agregado antes. Luego reintente la instalación.
      </Step>

      <Step title="Reiniciar con canales habilitados">
        Salga de Claude Code y reinicie con la bandera de canal:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Envíese un mensaje a sí mismo">
        Abra Mensajes en cualquier dispositivo conectado a su ID de Apple y envíese un mensaje a sí mismo. Llega a Claude inmediatamente: el auto-chat evita el control de acceso sin configuración.

        <Note>La primera respuesta que Claude envía activa un mensaje de Automatización de macOS preguntando si su terminal puede controlar Mensajes. Haga clic en **Aceptar**.</Note>
      </Step>

      <Step title="Permitir otros remitentes">
        De forma predeterminada, solo sus propios mensajes pasan. Para permitir que otro contacto llegue a Claude, agregue su identificador:

        ```
        /imessage:access allow +15551234567
        ```

        Los identificadores son números de teléfono en formato `+country` o correos electrónicos de ID de Apple como `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

También puede [crear su propio canal](/docs/es/channels-reference) para sistemas que aún no tienen un plugin.

<h2 id="quickstart">
  Inicio rápido
</h2>

Fakechat es un channel de demostración oficialmente compatible que ejecuta una interfaz de chat en localhost, sin nada que autenticar y sin servicio externo que configurar.

Una vez que instale y habilite fakechat, puede escribir en el navegador y el mensaje llega a su sesión de Claude Code. Claude responde y la respuesta aparece de nuevo en el navegador. Después de haber probado la interfaz de fakechat, pruebe [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) o [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Para probar la demostración de fakechat, necesitará:

* Claude Code [instalado y autenticado](/docs/es/quickstart#step-1-install-claude-code) con una cuenta de claude.ai o una clave API de Claude Console
* [Bun](https://bun.sh) instalado. Los plugins de channel precompilados son scripts de Bun. Verifique con `bun --version`; si eso falla, [instale Bun](https://bun.sh/docs/installation).
* **Organización de Team, Enterprise o Console administrada**: su administrador debe [habilitar channels](#enterprise-controls) en la configuración administrada

<Steps>
  <Step title="Instalar el plugin de channel fakechat">
    Inicie una sesión de Claude Code y ejecute el comando de instalación:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Si Claude Code informa que el plugin no se encuentra en ningún marketplace, su marketplace falta o está desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha agregado antes. Luego reintente la instalación.
  </Step>

  <Step title="Reiniciar con el channel habilitado">
    Salga de Claude Code y reinicie con `--channels` y pase el plugin fakechat que instaló:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    El servidor fakechat se inicia automáticamente.

    <Tip>
      Puede pasar varios plugins a `--channels`, separados por espacios.
    </Tip>
  </Step>

  <Step title="Enviar un mensaje">
    Abra la interfaz de fakechat en [http://localhost:8787](http://localhost:8787) y escriba un mensaje:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    El mensaje llega a su sesión de Claude Code como un evento `<channel source="fakechat">`. Claude lo lee, hace el trabajo y llama a la herramienta `reply` de fakechat. La respuesta aparece en la interfaz de chat.
  </Step>
</Steps>

Si Claude encuentra un mensaje de permiso mientras está fuera de la terminal, la sesión se pausa hasta que responda. Los servidores de channel que declaran la [capacidad de retransmisión de permisos](/docs/es/channels-reference#relay-permission-prompts) pueden reenviarle estos mensajes para que pueda aprobar o denegar de forma remota. Para uso desatendido, [`--dangerously-skip-permissions`](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) evita la mayoría de los mensajes, pero solo úselo en entornos en los que confíe. Las reglas de solicitud explícita, las herramientas de conector [que su organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y las herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) aún generan mensajes.

Cuando ejecuta channels en modo no interactivo con `-p`, las herramientas que necesitan entrada de terminal, como preguntas de opción múltiple y aprobación de plan mode, se deshabilitan para que la sesión nunca se quede esperando entrada.

<h2 id="security">
  Seguridad
</h2>

Cada plugin de channel aprobado mantiene una lista de permitidos del remitente: solo los ID que ha agregado pueden enviar mensajes, y todos los demás se descartan silenciosamente.

Telegram y Discord inician la lista mediante emparejamiento:

1. Encuentre su bot en Telegram o Discord y envíele cualquier mensaje
2. El bot responde con un código de emparejamiento
3. En su sesión de Claude Code, apruebe el código cuando se le solicite
4. Su ID de remitente se agrega a la lista de permitidos

iMessage funciona de manera diferente: enviarse un mensaje a sí mismo evita la puerta automáticamente, y agrega otros contactos por identificador con `/imessage:access allow`.

Además de eso, controla qué servidores están habilitados en cada sesión con `--channels`, y su organización controla la disponibilidad con [`channelsEnabled`](#enterprise-controls) en planes de Team y Enterprise de claude.ai y en organizaciones de Console que implementan configuración administrada.

Estar en `.mcp.json` no es suficiente para enviar mensajes: un servidor también tiene que estar nombrado en `--channels`.

La lista de permitidos también controla la [retransmisión de permisos](/docs/es/channels-reference#relay-permission-prompts) si el channel la declara. Cualquiera que pueda responder a través del channel puede aprobar o denegar el uso de herramientas en su sesión, por lo que solo agregue remitentes de lista de permitidos en los que confíe con esa autoridad.

<h2 id="enterprise-controls">
  Controles empresariales
</h2>

Los administradores controlan la disponibilidad a través de dos [configuraciones administradas](/docs/es/settings) que los usuarios no pueden anular. El valor predeterminado depende de cómo se autentique:

* **claude.ai Team y Enterprise**: los channels se bloquean hasta que un administrador los habilita.
* **Anthropic Console con autenticación de clave API**: los channels se permiten de forma predeterminada. Solo necesita esta configuración si su organización implementa configuración administrada.

En todos los casos, ningún channel se ejecuta hasta que un usuario lo opte por la sesión con `--channels`.

| Configuración           | Propósito                                                                                                                                                                                                                                                                                                                                                            | Cuando no está configurado                                                                                                                                                                                                 |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Interruptor maestro. Debe ser `true` para que cualquier channel entregue mensajes. Establézcalo a través del botón de alternancia de la [consola de administrador de claude.ai](https://claude.ai/admin-settings/claude-code) o directamente en la configuración administrada. Bloquea todos los channels incluida la bandera de desarrollo cuando está desactivado. | claude.ai Team y Enterprise: channels bloqueados. Console: channels permitidos a menos que su organización implemente configuración administrada, en cuyo caso los channels se bloquean hasta que se establezca esta clave |
| `allowedChannelPlugins` | Qué plugins pueden registrarse una vez que los channels están habilitados. Reemplaza la lista mantenida por Anthropic cuando se establece. Solo se aplica cuando `channelsEnabled` es `true`.                                                                                                                                                                        | Se aplica la lista predeterminada de Anthropic                                                                                                                                                                             |

Los usuarios de Pro y Max sin una organización omiten estas comprobaciones por completo: los channels están disponibles y los usuarios optan por participar por sesión con `--channels`.

<h3 id="enable-channels-for-your-organization">
  Habilitar channels para su organización
</h3>

Habilite channels para su organización desde [**claude.ai → Configuración de administrador → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), que requiere el rol de administrador, o estableciendo `channelsEnabled` en `true` en la configuración administrada.

Una vez habilitado, los usuarios de su organización pueden usar `--channels` para optar por servidores de channel en sesiones individuales. Si la configuración está deshabilitada o no está establecida, el servidor MCP aún se conecta y sus herramientas funcionan, pero los mensajes de channel no llegarán. Un mensaje de advertencia de inicio le dice al usuario que un administrador habilite la configuración.

<h3 id="restrict-which-channel-plugins-can-run">
  Restringir qué plugins de channel pueden ejecutarse
</h3>

De forma predeterminada, cualquier plugin en la lista de permitidos mantenida por Anthropic puede registrarse como un channel. Los administradores en planes de Team y Enterprise pueden reemplazar esa lista de permitidos con la suya propia estableciendo `allowedChannelPlugins` en la configuración administrada. Úselo para restringir qué plugins oficiales están permitidos, aprobar channels de su propio marketplace interno, o ambos. Cada entrada nombra un plugin y el marketplace del que proviene:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Cuando `allowedChannelPlugins` está establecido, reemplaza completamente la lista de permitidos de Anthropic: solo los plugins listados pueden registrarse. Déjelo sin establecer para volver a la lista de permitidos predeterminada de Anthropic. Si establece una matriz vacía, bloquea todos los plugins de channel de la lista de permitidos, pero `--dangerously-load-development-channels` aún puede omitirlo para pruebas locales. Para bloquear channels completamente incluida la bandera de desarrollo, déjelo sin establecer en su lugar.

Esta configuración requiere `channelsEnabled: true`. Si un usuario pasa un plugin a `--channels` que no está en su lista, Claude Code se inicia normalmente pero el channel no se registra, y el aviso de inicio explica que el plugin no está en la lista aprobada de la organización.

<h2 id="research-preview">
  Vista previa de investigación
</h2>

Los channels son una característica de vista previa de investigación. La disponibilidad se está implementando gradualmente, y la sintaxis de la bandera `--channels` y el contrato de protocolo pueden cambiar según los comentarios.

Durante la vista previa, `--channels` solo acepta plugins de una lista de permitidos mantenida por Anthropic, o de la lista de permitidos de su organización si un administrador ha establecido [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run). Los plugins de channel en [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) son el conjunto aprobado predeterminado. Si pasa algo que no está en la lista de permitidos efectiva, Claude Code se inicia normalmente pero el channel no se registra, y el aviso de inicio le dice por qué.

Para probar un channel que está creando, use `--dangerously-load-development-channels`. Vea [Probar durante la vista previa de investigación](/docs/es/channels-reference#test-during-the-research-preview) para obtener información sobre cómo probar channels personalizados que cree.

Informe de problemas o comentarios en el [repositorio de GitHub de Claude Code](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Cómo se comparan los channels
</h2>

Varias características de Claude Code se conectan a sistemas fuera de la terminal, cada una adecuada para un tipo diferente de trabajo:

| Característica                                      | Qué hace                                                                | Bueno para                                                               |
| --------------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| [Claude Code en la web](/docs/es/claude-code-on-the-web) | Ejecuta tareas en una nueva sandbox en la nube, clonada desde GitHub    | Delegar trabajo asincrónico independiente que verifica más tarde         |
| [Claude en Slack](/docs/es/slack)                        | Genera una sesión web desde una mención `@Claude` en un canal o hilo    | Iniciar tareas directamente desde el contexto de conversación del equipo |
| [Servidor MCP](/docs/es/mcp) estándar                    | Claude lo consulta durante una tarea; nada se envía a la sesión         | Dar a Claude acceso bajo demanda para leer o consultar un sistema        |
| [Control Remoto](/docs/es/remote-control)                | Conduce su sesión local desde claude.ai o la aplicación móvil de Claude | Dirigir una sesión en progreso mientras está fuera de su escritorio      |

Los channels cierran la brecha en esa lista al enviar eventos de fuentes que no son de Claude a su sesión local ya en ejecución.

* **Puente de chat**: pregúntele a Claude algo desde su teléfono a través de Telegram, Discord o iMessage, y la respuesta regresa en el mismo chat mientras el trabajo se ejecuta en su máquina contra sus archivos reales.
* **[Receptor de webhook](/docs/es/channels-reference#example-build-a-webhook-receiver)**: un webhook de CI, su rastreador de errores, una canalización de implementación u otro servicio externo llega donde Claude ya tiene sus archivos abiertos y recuerda lo que estaba depurando.

<h2 id="next-steps">
  Próximos pasos
</h2>

Una vez que tenga un channel en ejecución, explore estas características relacionadas:

* [Crear su propio channel](/docs/es/channels-reference) para sistemas que aún no tienen plugins
* [Control Remoto](/docs/es/remote-control) para conducir una sesión local desde su teléfono en lugar de reenviar eventos a ella
* [Tareas programadas](/docs/es/scheduled-tasks) para sondear en un temporizador en lugar de reaccionar a eventos enviados
