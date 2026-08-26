> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Permitir que Claude use su computadora desde la CLI

> Habilite computer use en la CLI de Claude Code para que Claude pueda abrir aplicaciones, hacer clic, escribir y ver su pantalla en macOS. Pruebe aplicaciones nativas, depure problemas visuales y automatice herramientas solo GUI sin salir de su terminal.

<Note>
  Computer use es una vista previa de investigación en macOS que requiere un plan Pro o Max. No está disponible en planes Team o Enterprise. Requiere una sesión interactiva, por lo que no está disponible en modo no interactivo con la bandera `-p`.
</Note>

Computer use permite que Claude abra aplicaciones, controle su pantalla y trabaje en su máquina de la manera que lo haría usted. Desde la CLI, Claude puede compilar una aplicación Swift, lanzarla, hacer clic en cada botón y capturar una pantalla del resultado, todo en la misma conversación donde escribió el código.

Esta página cubre cómo funciona computer use en la CLI. Para la aplicación de escritorio en macOS o Windows, consulte [computer use en Desktop](/docs/es/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  Qué puede hacer con computer use
</h2>

Computer use maneja tareas que requieren una GUI: cualquier cosa que normalmente tendría que dejar la terminal y hacer manualmente.

* **Compilar y validar aplicaciones nativas**: pida a Claude que compile una aplicación de barra de menú de macOS. Claude escribe el Swift, lo compila, lo lanza y hace clic en cada control para verificar que funciona antes de que usted lo abra.
* **Pruebas de UI de extremo a extremo**: señale a Claude una aplicación Electron local y diga "prueba el flujo de incorporación". Claude abre la aplicación, hace clic en el registro y captura cada paso. Sin configuración de Playwright, sin arnés de prueba.
* **Depurar problemas visuales y de diseño**: dígale a Claude "el modal se está cortando en ventanas pequeñas". Claude redimensiona la ventana, reproduce el error, captura una pantalla, parcha el CSS y verifica la corrección. Claude ve lo que usted ve.
* **Impulsar herramientas solo GUI**: interactúe con herramientas de diseño, paneles de control de hardware, el simulador de iOS o aplicaciones propietarias que no tienen CLI ni API.

<h2 id="when-computer-use-applies">
  Cuándo se aplica computer use
</h2>

Claude tiene varias formas de interactuar con una aplicación o servicio. Computer use es la más amplia y lenta, por lo que Claude intenta la herramienta más precisa primero:

* Si tiene un [servidor MCP](/docs/es/mcp) para el servicio, Claude lo usa.
* Si la tarea es un comando shell, Claude usa Bash.
* Si la tarea es trabajo en navegador y tiene [Claude en Chrome](/docs/es/chrome) configurado, Claude lo usa.
* Si ninguno de esos se aplica, Claude usa computer use.

El control de pantalla se reserva para cosas que nada más puede alcanzar: aplicaciones nativas, simuladores y herramientas sin API.

<h2 id="enable-computer-use">
  Habilitar computer use
</h2>

Computer use está disponible como un servidor MCP integrado llamado `computer-use`. Está desactivado de forma predeterminada hasta que lo habilite.

<Steps>
  <Step title="Abra el menú MCP">
    En una sesión interactiva de Claude Code, ejecute:

    ```text theme={null}
    /mcp
    ```

    Encuentre `computer-use` en la lista de servidores. Se muestra como deshabilitado.
  </Step>

  <Step title="Habilite el servidor">
    Seleccione `computer-use` y elija **Enable**. La configuración persiste por proyecto, por lo que solo hace esto una vez para cada proyecto donde desee computer use.
  </Step>

  <Step title="Otorgue permisos de macOS">
    La primera vez que Claude intente usar su computadora, verá un mensaje para otorgar dos permisos de macOS:

    * **Accessibility**: permite que Claude haga clic, escriba y desplace
    * **Screen Recording**: permite que Claude vea lo que hay en su pantalla

    El mensaje incluye enlaces para abrir el panel de Configuración del Sistema relevante. Otorgue ambos, luego seleccione **Try again** en el mensaje. macOS puede requerir que reinicie Claude Code después de otorgar Screen Recording.
  </Step>
</Steps>

Después de la configuración, pida a Claude que haga algo que necesite la GUI:

```text theme={null}
Build the app target, launch it, and click through each tab to make
sure nothing crashes. Screenshot any error states you find.
```

<h2 id="approve-apps-per-session">
  Apruebe aplicaciones por sesión
</h2>

Habilitar el servidor `computer-use` no otorga a Claude acceso a todas las aplicaciones en su máquina. La primera vez que Claude necesita una aplicación específica en una sesión, aparece un mensaje en su terminal mostrando:

* Qué aplicaciones Claude desea controlar
* Cualquier permiso adicional solicitado, como acceso al portapapeles
* Cuántas otras aplicaciones se ocultarán mientras Claude trabaja

Elija **Allow for this session** o **Deny**. Las aprobaciones duran para la sesión actual. Puede aprobar múltiples aplicaciones a la vez cuando Claude las solicita juntas.

Las aplicaciones con amplio alcance muestran una advertencia adicional en el mensaje para que sepa qué otorga aprobarlas:

| Advertencia                                | Se aplica a                                              |
| :----------------------------------------- | :------------------------------------------------------- |
| Equivalente a acceso shell                 | Terminal, iTerm, VS Code, Warp y otras terminales e IDEs |
| Puede leer o escribir cualquier archivo    | Finder                                                   |
| Puede cambiar la configuración del sistema | System Settings                                          |

Estas aplicaciones no están bloqueadas. La advertencia le permite decidir si la tarea justifica ese nivel de acceso.

El nivel de control de Claude también varía según la categoría de aplicación: los navegadores y plataformas de trading son solo lectura, las terminales e IDEs son solo clic, y todo lo demás obtiene control total. Consulte [permisos de aplicación en Desktop](/docs/es/desktop#app-permissions) para el desglose completo de niveles.

<h2 id="how-claude-works-on-your-screen">
  Cómo Claude trabaja en su pantalla
</h2>

Comprender el flujo le ayuda a anticipar qué hará Claude y cómo intervenir.

<h3 id="one-session-at-a-time">
  Una sesión a la vez
</h3>

Computer use mantiene un bloqueo en toda la máquina desde la primera acción de computer use hasta que la sesión que lo adquirió sale. A partir de v2.1.195, terminar la tarea no libera el bloqueo; solo salir de la sesión lo hace. Si otra sesión de Claude Code ya está usando su computadora, los nuevos intentos fallan con un mensaje que le dice qué sesión mantiene el bloqueo. Salga de esa sesión primero.

<h3 id="apps-are-hidden-while-claude-works">
  Las aplicaciones se ocultan mientras Claude trabaja
</h3>

Cuando Claude comienza a controlar su pantalla, otras aplicaciones visibles se ocultan para que Claude interactúe solo con las aplicaciones aprobadas. Su ventana de terminal permanece visible y se excluye de las capturas de pantalla, por lo que puede ver la sesión y Claude nunca ve su propio resultado.

Cuando Claude termina el turno, las aplicaciones ocultas se restauran automáticamente.

<h3 id="screenshots-are-downscaled-automatically">
  Las capturas de pantalla se reducen automáticamente
</h3>

Claude Code reduce el tamaño de cada captura de pantalla antes de enviarla al modelo. No necesita reducir su resolución de pantalla ni redimensionar ventanas en pantallas Retina u otras pantallas de alta resolución. Un MacBook Pro de 16 pulgadas con resolución Retina nativa captura a 3456×2234 y se reduce a aproximadamente 1372×887, preservando la relación de aspecto.

No hay configuración para cambiar el tamaño objetivo. Si el texto o los controles en pantalla son demasiado pequeños para que Claude los lea después de reducir el tamaño, aumente su tamaño en la aplicación en lugar de cambiar su resolución de pantalla.

<h3 id="stop-at-any-time">
  Detener en cualquier momento
</h3>

Cuando Claude adquiere el bloqueo, aparece una notificación de macOS: "Claude is using your computer · press Esc to stop". Presione `Esc` en cualquier lugar para abortar la acción actual inmediatamente, o presione `Ctrl+C` en la terminal. De cualquier manera, Claude se detiene, muestra sus aplicaciones y le devuelve el control. La sesión mantiene el [bloqueo de computer use](#one-session-at-a-time) hasta que sale.

Una segunda notificación aparece cuando Claude termina.

<h2 id="safety-and-the-trust-boundary">
  Seguridad y el límite de confianza
</h2>

<Warning>
  A diferencia de la [herramienta Bash en sandbox](/docs/es/sandboxing), computer use se ejecuta en su escritorio real con acceso a las aplicaciones que aprueba. Claude verifica cada acción e identifica posibles inyecciones de solicitud desde el contenido en pantalla, pero el límite de confianza es diferente. Consulte la [guía de seguridad de computer use](https://support.claude.com/en/articles/14128542) para mejores prácticas.
</Warning>

Los guardarraíles integrados reducen el riesgo sin requerir configuración:

* **Aprobación por aplicación**: Claude solo puede controlar aplicaciones que ha aprobado en la sesión actual.
* **Advertencias centinela**: las aplicaciones que otorgan acceso shell, sistema de archivos o configuración del sistema se marcan antes de que las apruebe.
* **Terminal excluida de capturas de pantalla**: Claude nunca ve su ventana de terminal, por lo que los mensajes en pantalla en su sesión no pueden retroalimentarse al modelo.
* **Escape global**: la tecla `Esc` aborta computer use desde cualquier lugar, y la pulsación de tecla se consume para que la inyección de solicitud no pueda usarla para descartar diálogos.
* **Archivo de bloqueo**: solo una sesión puede controlar su máquina a la vez.

<h2 id="example-workflows">
  Flujos de trabajo de ejemplo
</h2>

Estos ejemplos muestran formas comunes de combinar computer use con tareas de codificación.

<h3 id="validate-a-native-build">
  Validar una compilación nativa
</h3>

Después de hacer cambios en una aplicación de macOS o iOS, haga que Claude compile y verifique en un solo paso:

```text theme={null}
Build the MenuBarStats target, launch it, open the preferences window,
and verify the interval slider updates the label. Screenshot the
preferences window when you're done.
```

Claude ejecuta `xcodebuild`, lanza la aplicación, interactúa con la UI y reporta lo que encuentra.

<h3 id="reproduce-a-layout-bug">
  Reproducir un error de diseño
</h3>

Cuando un error visual solo aparece en ciertos tamaños de ventana, deje que Claude lo encuentre:

```text theme={null}
The settings modal clips its footer on narrow windows. Resize the app
window down until you can reproduce it, screenshot the clipped state,
then check the CSS for the modal container.
```

Claude redimensiona la ventana, captura el estado roto y lee las hojas de estilo relevantes.

<h3 id="test-a-simulator-flow">
  Probar un flujo de simulador
</h3>

Impulse el simulador de iOS sin escribir XCTest:

```text theme={null}
Open the iOS Simulator, launch the app, tap through the onboarding
screens, and tell me if any screen takes more than a second to load.
```

Claude controla el simulador de la misma manera que lo haría con un ratón.

<h2 id="differences-from-the-desktop-app">
  Diferencias de la aplicación de escritorio
</h2>

Las superficies CLI y Desktop comparten el mismo motor de computer use, con algunas diferencias:

| Característica                  | Desktop                                                      | CLI                                |
| :------------------------------ | :----------------------------------------------------------- | :--------------------------------- |
| Plataformas                     | macOS y Windows                                              | Solo macOS                         |
| Habilitar                       | Alternar en **Settings > General** (bajo **Desktop app**)    | Habilitar `computer-use` en `/mcp` |
| Lista de aplicaciones denegadas | Configurable en Settings                                     | Aún no disponible                  |
| Alternar auto-unhide            | Opcional                                                     | Siempre activado                   |
| Integración de Dispatch         | Las sesiones generadas por Dispatch pueden usar computer use | No aplicable                       |

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  "Computer use is in use by another Claude session"
</h3>

Otra sesión de Claude Code mantiene el bloqueo, que conserva hasta que sale. Salga de esa sesión. Si la otra sesión se bloqueó, el bloqueo se libera automáticamente cuando Claude detecta que el proceso ya no se está ejecutando.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  El mensaje de permisos de macOS sigue reapareciendo
</h3>

macOS a veces requiere un reinicio del proceso solicitante después de otorgar Screen Recording. Salga completamente de Claude Code e inicie una nueva sesión. Si el mensaje persiste, abra **System Settings > Privacy & Security > Screen Recording** y confirme que su aplicación de terminal está listada y habilitada.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` no aparece en `/mcp`
</h3>

El servidor solo aparece en configuraciones elegibles. Verifique que:

* Está en macOS. Computer use en la CLI no está disponible en Linux o Windows. En Windows, use [computer use en Desktop](/docs/es/desktop#let-claude-use-your-computer) en su lugar.
* Está en un plan Pro o Max. Ejecute `/status` para confirmar su suscripción.
* Está autenticado a través de claude.ai. Computer use no está disponible con proveedores de terceros como Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Si accede a Claude exclusivamente a través de un proveedor de terceros, necesita una cuenta separada de claude.ai para usar esta característica.
* Está en una sesión interactiva. Computer use no está disponible en modo no interactivo con la bandera `-p`.

<h2 id="see-also">
  Ver también
</h2>

* [Computer use en Desktop](/docs/es/desktop#let-claude-use-your-computer): la misma capacidad con una página de configuración gráfica
* [Claude en Chrome](/docs/es/chrome): automatización de navegador para tareas basadas en web
* [MCP](/docs/es/mcp): conecte Claude a herramientas y APIs estructuradas
* [Sandboxing](/docs/es/sandboxing): cómo la herramienta Bash de Claude aísla el acceso al sistema de archivos y red
* [Guía de seguridad de computer use](https://support.claude.com/en/articles/14128542): mejores prácticas para computer use seguro
