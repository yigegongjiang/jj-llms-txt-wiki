> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Prueba aplicaciones iOS en el simulador

> Claude Code Desktop abre tu aplicación en el panel del Simulador de iOS cuando Claude la compila, ejecuta o verifica, con un simulador separado para cada sesión.

<Note>
  El panel del Simulador de iOS está en beta pública en Claude Code Desktop en macOS. Está disponible en los planes Pro, Max, Team y Enterprise, excepto en organizaciones Enterprise que tengan habilitada una configuración HIPAA.
</Note>

El panel del Simulador de iOS muestra tu aplicación ejecutándose en el Simulador de iOS de Apple junto a tu conversación en Claude Code Desktop. Cuando Claude compila, instala, lanza o verifica tu aplicación en un simulador, el panel se abre automáticamente y transmite la pantalla del dispositivo en vivo. Úsalo para ver a Claude ejecutar y probar tu aplicación, o para navegar por la aplicación tú mismo mientras Claude sigue trabajando.

El panel del simulador controla el simulador directamente, por lo que no necesita [uso de computadora](/docs/es/desktop#let-claude-use-your-computer) y nunca toma el control de tu pantalla ni oculta tus otras ventanas. Desde la CLI, Claude accede al Simulador de iOS a través del [uso de computadora](/docs/es/computer-use#test-a-simulator-flow) en su lugar, que controla el simulador en tu pantalla de la misma manera que lo harías con un ratón.

<h2 id="requirements">
  Requisitos
</h2>

El panel del simulador utiliza las herramientas del simulador de Apple, que la aplicación de escritorio no incluye. Antes de iniciar una sesión, asegúrate de tener:

* Claude Desktop v1.24012.0 o posterior
* Una Mac, ya que el Simulador de iOS de Apple se ejecuta solo en macOS
* [Xcode](https://developer.apple.com/xcode/) con la plataforma iOS instalada, que proporciona los dispositivos del simulador. Si Xcode aún no enumera simuladores, consulta [El panel del simulador dice que no se encontraron simuladores](#the-simulator-pane-says-no-simulators-were-found)
  * Usa Xcode 26.x. El panel aún no funciona con Xcode 27, que reemplaza la aplicación Simulator con Device Hub. Si `xcode-select` apunta a Xcode 27 en tu Mac, consulta [El panel del simulador falla con Xcode 27](#the-simulator-pane-fails-with-xcode-27)

<Note>
  En esta página, "dispositivo" se refiere a un iPhone o iPad simulado, uno de los mismos dispositivos del simulador que administras en Xcode en **Window → Devices and Simulators**, no hardware físico.
</Note>

El panel del simulador está disponible solo en sesiones locales. En sesiones en [nube](/docs/es/desktop#run-long-running-tasks-in-the-cloud) y [SSH](/docs/es/desktop#ssh-sessions), Claude se ejecuta en una máquina que no puede acceder a los simuladores en su Mac.

<h2 id="run-your-app-in-the-simulator">
  Ejecuta tu aplicación en el simulador
</h2>

No necesitas un comando o configuración para abrir el panel del simulador. Claude lo abre cuando ejecuta tu aplicación en un simulador.

<Steps>
  <Step title="Abre tu proyecto de iOS">
    En Claude Code Desktop, abre la pestaña **Code** e inicia una sesión con la carpeta del proyecto de tu aplicación como la [carpeta del proyecto](/docs/es/desktop#start-a-session). Cualquier proyecto que compile una aplicación para el Simulador de iOS funciona.
  </Step>

  <Step title="Pídele a Claude que ejecute o pruebe la aplicación">
    Expresa la tarea alrededor de ejecutar o verificar la aplicación. Por ejemplo:

    ```text theme={null}
    Compila la aplicación y ejecútala en el simulador para verificar el flujo de incorporación.
    ```
  </Step>

  <Step title="Observa la aplicación en el panel del simulador">
    Cuando la aplicación se lanza en un simulador, el panel del Simulador de iOS se abre junto a la conversación. La primera vez que Claude usa un dispositivo, la aplicación de escritorio te pide que lo permitas; consulta [Otorga a Claude acceso a un dispositivo](#grant-claude-access-to-a-device). Claude instala la aplicación, navega por ella y lee la pantalla para verificar sus propios cambios mientras observas.
  </Step>
</Steps>

El panel del simulador se abre cada vez que Claude lanza la aplicación en un simulador, en cualquier punto de la sesión. Cuando tu solicitud es sobre ver la aplicación, por ejemplo "¿se ve bien la nueva pantalla?", Claude inicia un simulador antes de comenzar el trabajo. Después de que Claude corrija un error o cambie una pantalla, pídele que verifique el cambio: relanzar la aplicación reabre el panel si no está abierto.

El panel del simulador muestra el dispositivo en el que la aplicación realmente se lanzó. Para probar en un dispositivo específico, nómbralo en tu solicitud, por ejemplo "ejecútalo en el simulador de iPhone SE", y Claude apunta a ese dispositivo cuando compila y lanza.

Un dispositivo que Claude inicia también aparece en la aplicación Simulator de Apple, y Claude puede instalar la aplicación en un dispositivo que ya tengas iniciado.

También puedes abrir el panel del simulador tú mismo. Una vez que la sesión tiene un simulador adjunto o ha editado archivos Swift, el menú **Views** en la barra de herramientas de la sesión muestra una entrada **iOS Simulator**. Si el panel aún no muestra un dispositivo, haz clic en **Attach simulator**, o elige un dispositivo específico del menú de dispositivos junto a él; elegir un dispositivo apagado lo inicia. Si faltan Xcode o sus simuladores, el panel muestra los pasos de configuración en su lugar y los marca a medida que los completas.

<h2 id="control-the-simulator-yourself">
  Controla el simulador tú mismo
</h2>

El panel del simulador es interactivo, no solo un visor. Mientras Claude trabaja, o entre tareas, puedes:

* Tocar y deslizar haciendo clic y arrastrando en la pantalla del dispositivo
* Presionar botones de hardware con los mismos atajos de teclado que la aplicación Simulator de Apple: **Cmd+Shift+H** para Inicio, **Cmd+L** para bloquear, **Cmd+Flecha Arriba** y **Cmd+Flecha Abajo** para volumen
* Rotar el dispositivo un cuarto de vuelta en el sentido de las agujas del reloj con el botón de rotación o **Cmd+Flecha Derecha**
* Cambiar qué dispositivo muestra el panel desde el menú de dispositivos, que enumera la versión del SO de cada simulador y si está iniciado
* Guardar una captura de pantalla con **Cmd+S** o una grabación de pantalla con **Cmd+R**, usando los botones de captura del panel o los atajos; los archivos se guardan en tu Escritorio
* Detener la transmisión de un dispositivo sin apagarlo haciendo clic en **Detach simulator**, que devuelve el panel a su estado **Attach simulator**

La fila bajo el nombre del dispositivo ajusta la transmisión de video desde el simulador. Reduce la **Frame rate** o **Resolution** si el panel sobrecarga tu Mac, cambia **Encoding** entre H.264 y JPEG, o marca **FPS** para mostrar la velocidad de fotogramas que el panel está recibiendo. Estos ajustes cambian cómo el panel muestra el dispositivo, no cómo se ejecuta la aplicación.

Tú y Claude controlan el mismo dispositivo, por lo que tus toques cambian el estado de la aplicación que Claude ve. Para que Claude verifique una pantalla específica, navega a ella tocando, luego pregunta. Mientras Claude controla el dispositivo, el panel muestra una insignia **Claude is using this device** encima de la pantalla; espera a tocar hasta que la insignia desaparezca, para que el resultado refleje la aplicación en lugar de tu entrada.

<h2 id="how-sessions-manage-devices">
  Cómo las sesiones administran dispositivos
</h2>

Cada dispositivo pertenece a la sesión que lo lanzó, por lo que las [sesiones paralelas](/docs/es/desktop#work-in-parallel-with-sessions) no comparten un dispositivo: lo que ves en el panel de una sesión refleja el trabajo de esa sesión, no el de otra. Cambiar sesiones en la barra lateral cambia la vista del simulador junto con la conversación, y cambiar de nuevo reanuda el mismo dispositivo donde se quedó. Si Claude trabaja con más de un dispositivo, cada uno abre su propio panel, hasta 4 por sesión.

Claude Code Desktop apaga los simuladores que inició una vez que ya no están en uso: cuando cierras la aplicación, cuando archivas la sesión, o 10 minutos después de que desconectes un dispositivo de su panel. Los dispositivos que inicias tú mismo, ya sea desde el panel o en la aplicación Simulator de Apple, nunca se apagan automáticamente. Para apagar el dispositivo adjunto de inmediato, usa el botón de apagado en el panel.

<h2 id="grant-claude-access-to-a-device">
  Otorga a Claude acceso a un dispositivo
</h2>

Claude solicita tu consentimiento antes de controlar un dispositivo, mientras que compilar la aplicación o abrir una URL en ella sigue el modo de permiso de tu sesión. Tú u tu organización también pueden desactivar completamente el acceso de Claude.

<h3 id="allow-a-device-the-first-time">
  Permite un dispositivo la primera vez
</h3>

La primera vez que Claude usa un simulador, la aplicación de escritorio te pide que lo permitas. El consentimiento cubre controlar ese dispositivo y tomar capturas de pantalla del mismo, y lo das una vez por dispositivo en lugar de una vez por sesión. Las capturas de pantalla de Claude del dispositivo se envían a Anthropic y se mantienen bajo tu configuración normal de retención de conversaciones, por lo que no inicies sesión en cuentas reales en un dispositivo que Claude use.

Después de permitir un dispositivo, las acciones de Claude en él, como tocar, escribir, lanzar la aplicación y tomar capturas de pantalla, se ejecutan sin más indicaciones. Tienen la misma confianza que tú haciendo clic en el panel, y solo tocan el dispositivo simulado, por lo que el panel no necesita los permisos de Accesibilidad y Grabación de Pantalla de macOS que requiere el uso de computadora.

Si rechazas, el dispositivo aún se inicia y el panel aún funciona para tus propios toques; solo el acceso de Claude se mantiene desactivado. Para cambiar de opinión más tarde, haz clic en **Let Claude use it** en el panel.

<h3 id="actions-that-follow-your-permission-mode">
  Acciones que siguen tu modo de permiso
</h3>

Dos acciones siguen el [modo de permiso](/docs/es/permissions#permission-modes) de tu sesión en lugar del consentimiento único:

* Abrir una URL en el dispositivo, por ejemplo para probar un enlace profundo o cargar una página en Safari del dispositivo, porque una URL puede llevar datos fuera del dispositivo.
* Compilar la aplicación, porque `xcodebuild` ejecuta los scripts de compilación del proyecto en tu Mac. Verificar una compilación ya en progreso no genera indicaciones.

<h3 id="turn-off-simulator-access">
  Desactiva el acceso del simulador
</h3>

Puedes desactivar el acceso del simulador de Claude en la configuración de la aplicación de escritorio. Las organizaciones tienen dos formas de desactivarlo para todos:

* La [configuración administrada](/docs/es/desktop#managed-settings) `disableMobileSimulatorTools` bloquea las herramientas del simulador de Claude. El panel del simulador sigue siendo utilizable para tus propios toques, y la configuración no se puede anular desde dentro de la aplicación.
* La clave de política `requireCoworkFullVmSandbox`, que ejecuta las herramientas de Claude dentro de una máquina virtual aislada en lugar de en tu Mac, desactiva el panel del simulador y las herramientas del simulador de Claude por completo, por lo que el panel no puede adjuntar un dispositivo mientras esté configurado.

Claude te informa cuando se aplica cualquiera de estos.

<h2 id="limitations">
  Limitaciones
</h2>

Claude controla solo dispositivos simulados y no puede controlar un iPhone o iPad físico. Para probar en uno, ejecuta la aplicación en él desde Xcode tú mismo, luego describe lo que ves o adjunta una captura de pantalla a la conversación para que Claude trabaje con ella.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  El panel del simulador no se abre cuando Claude ejecuta la aplicación
</h3>

Claude puede no haber reconocido que querías ejecutar o probar la aplicación, o puede que falten las herramientas del simulador. Verifica lo siguiente:

* Expresa el objetivo explícitamente, por ejemplo "ejecuta la aplicación en el Simulador de iOS y navega por el flujo de registro".
* Confirma que Xcode y los simuladores de iOS estén instalados y que tu versión de Xcode cumpla con los [requisitos](#requirements).
* Si tu organización administra Claude Code, las [herramientas del simulador pueden estar deshabilitadas por política](#turn-off-simulator-access).
* Si estás en una organización Enterprise que tiene habilitada una configuración HIPAA, el panel del simulador no está disponible para ti.
* El panel del simulador requiere Claude Desktop v1.24012.0 o posterior. Abre **Claude → Check for Updates**, luego reinicia la aplicación.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  El panel del simulador dice que no se encontraron simuladores
</h3>

Si `xcode-select` apunta a Xcode 27, el panel puede informar que no hay simuladores aunque existan dispositivos; consulta [El panel del simulador falla con Xcode 27](#the-simulator-pane-fails-with-xcode-27). De lo contrario, Xcode está instalado pero no tiene simuladores de iOS para enumerar. El panel del simulador muestra los pasos de configuración a seguir y los marca a medida que cada uno se completa. Para instalar la pieza faltante manualmente, descarga el tiempo de ejecución del simulador de iOS de la configuración de Xcode, o ejecuta `xcodebuild -downloadPlatform iOS`.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  El panel del simulador falla con Xcode 27
</h3>

El panel aún no funciona con Xcode 27, que reemplaza la aplicación Simulator con Device Hub. Con Xcode 27 seleccionado, adjuntar un dispositivo falla, o el panel informa que no se encontraron simuladores aunque existan dispositivos.

El panel usa cualquier Xcode al que apunte `xcode-select`. Si Xcode 27 es tu única instalación, instala Xcode 26.x junto a él primero. Luego selecciona la instalación 26.x por su ruta. Por ejemplo, si está instalado como `/Applications/Xcode-26.4.app`:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Ejecuta `xcode-select -p` para verificar qué instalación está seleccionada.

<h2 id="see-also">
  Ver también
</h2>

* [Uso de computadora en Desktop](/docs/es/desktop#let-claude-use-your-computer): control de pantalla para aplicaciones sin un panel dedicado
* [Uso de computadora desde la CLI](/docs/es/computer-use): cómo la CLI accede al Simulador de iOS
* [Trabaja en paralelo con sesiones](/docs/es/desktop#work-in-parallel-with-sessions): cómo las sesiones aíslan los cambios
* [Comienza con Claude Code Desktop](/docs/es/desktop-quickstart)
