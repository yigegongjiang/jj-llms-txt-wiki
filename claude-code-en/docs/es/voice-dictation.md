> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dictado de voz

> Hable sus indicaciones en la CLI de Claude Code con dictado de voz de mantener para grabar o tocar para grabar.

Hable sus indicaciones en lugar de escribirlas en la CLI de Claude Code. Su voz se transcribe en vivo en la entrada de indicaciones, por lo que puede mezclar voz y escritura en el mismo mensaje. Habilite el dictado con `/voice`, luego mantenga presionada una tecla mientras habla o toque una vez para comenzar y nuevamente para enviar.

<Note>
  El modo de toque requiere Claude Code v2.1.116 o posterior. Verifique su versión con `claude --version`.
</Note>

El dictado también funciona en [vista de agente](/docs/es/agent-view#peek-and-reply). Mantenga presionada o toque su tecla de micrófono mientras la entrada de envío o una respuesta del panel de vista previa esté enfocada para dictar a una sesión en segundo plano.

<h2 id="requirements">
  Requisitos
</h2>

El dictado de voz transmite su audio grabado a los servidores de Anthropic para su transcripción. El audio no se procesa localmente. Necesita todos los siguientes requisitos:

* **Una cuenta de Claude.ai**: el servicio de voz a texto solo está disponible cuando se autentica con una, y no está disponible cuando Claude Code está configurado para usar una clave de API de Anthropic directamente, Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.
* **Una organización sin cumplimiento de HIPAA habilitado**: `/voice` muestra `Voice mode is disabled by your organization's policy` cuando se aplica esta restricción.
* **Un micrófono local**: el dictado de voz no funciona en entornos remotos como [Claude Code en la web](/docs/es/claude-code-on-the-web) o sesiones SSH.
* **WSLg, si ejecuta Claude Code en WSL**: WSLg se incluye con WSL2 cuando se instala desde Microsoft Store en Windows 10 u 11. Si WSLg no está disponible, por ejemplo en WSL1, ejecute Claude Code en Windows nativo en su lugar.

La transcripción no consume mensajes de Claude ni tokens y no cuenta hacia los límites mostrados en `/usage`. Consulte [uso de datos](/docs/es/data-usage) para ver cómo Anthropic maneja sus datos.

La grabación de audio utiliza un módulo nativo integrado en macOS, Linux y Windows. En Linux, si el módulo nativo no puede cargarse, Claude Code vuelve a `arecord` de ALSA utils o `rec` de SoX. Si ninguno está disponible, `/voice` imprime un comando de instalación para su gestor de paquetes.

La [extensión de VS Code](/docs/es/vs-code) de Claude Code también admite dictado de voz con el mismo requisito de cuenta de Claude.ai. No está disponible en sesiones remotas de VS Code, incluidas SSH, Dev Containers y Codespaces, porque el micrófono está en su máquina local y la extensión se ejecuta en el host remoto.

<h2 id="enable-voice-dictation">
  Habilitar dictado de voz
</h2>

Ejecute `/voice` para habilitar el dictado. La primera vez que lo habilite, Claude Code ejecuta una verificación de micrófono. En macOS, esto activa la solicitud de permiso de micrófono del sistema para su terminal si nunca se le ha otorgado.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` acepta un argumento de modo opcional:

| Comando       | Efecto                                                   |
| :------------ | :------------------------------------------------------- |
| `/voice`      | Alternar activado o desactivado, mantener el modo actual |
| `/voice hold` | Habilitar en [modo de mantener](#hold-to-record)         |
| `/voice tap`  | Habilitar en [modo de toque](#tap-to-record-and-send)    |
| `/voice off`  | Desactivar                                               |

El dictado de voz persiste entre sesiones. Configúrelo directamente en su [archivo de configuración de usuario](/docs/es/settings) en lugar de ejecutar `/voice`:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Mientras el dictado de voz está habilitado, el pie de página de entrada muestra una sugerencia `hold space to speak` cuando la indicación está vacía. La sugerencia refleja su enlace actual `voice:pushToTalk` y se actualiza si [revincula la tecla de dictado](#rebind-the-dictation-key). El texto de la sugerencia es el mismo en ambos modos, y no aparece si tiene una [línea de estado personalizada](/docs/es/statusline) configurada.

La transcripción se ajusta para vocabulario de codificación en ambos modos. Los términos de desarrollo comunes como `regex`, `OAuth`, `JSON` y `localhost` se reconocen correctamente, y el nombre del proyecto actual y el nombre de la rama de git se agregan automáticamente como sugerencias de reconocimiento.

<h2 id="hold-to-record">
  Mantener para grabar
</h2>

El modo de mantener es pulsar para hablar: la grabación se ejecuta mientras mantiene la tecla presionada y se detiene cuando la suelta. Este es el modo predeterminado.

Mantenga presionada la `Barra espaciadora` para comenzar a grabar. Claude Code detecta una tecla mantenida observando eventos rápidos de repetición de teclas desde su terminal, por lo que hay un breve calentamiento antes de que comience la grabación. El pie de página muestra `keep holding…` durante el calentamiento, luego cambia a una forma de onda en vivo una vez que la grabación está activa.

Los primeros caracteres de repetición de tecla escriben en la entrada durante el calentamiento y se eliminan automáticamente cuando se activa la grabación. Un único toque de `Barra espaciadora` aún escribe un espacio, ya que la detección de mantener solo se activa en repetición rápida.

<Tip>
  Para omitir el calentamiento, cambie al [modo de toque](#tap-to-record-and-send) con `/voice tap`, o [reenlace a una combinación de modificador](#rebind-the-dictation-key) como `meta+k`. Las combinaciones de modificadores comienzan a grabar en la primera pulsación de tecla.
</Tip>

Su voz aparece en la indicación mientras habla, atenuada hasta que se finaliza la transcripción. Suelte la `Barra espaciadora` para detener la grabación y finalizar el texto. La transcripción se inserta en la posición del cursor y el cursor permanece al final del texto insertado, por lo que puede mezclar escritura y dictado en cualquier orden. Mantenga presionada la `Barra espaciadora` nuevamente para agregar otra grabación, o mueva el cursor primero para insertar voz en otro lugar de la indicación:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

De forma predeterminada, soltar la tecla inserta la transcripción y espera a que presione `Enter`. Establezca `"autoSubmit": true` en el objeto de configuración `voice` para enviar la indicación automáticamente cuando suelte la tecla, siempre que la transcripción tenga al menos tres palabras.

<h2 id="tap-to-record-and-send">
  Tocar para grabar y enviar
</h2>

El modo de toque alterna la grabación con una sola pulsación de tecla: toque una vez para comenzar, hable, luego toque nuevamente para enviar la indicación. No hay calentamiento y no necesita mantener la tecla presionada.

Habilite el modo de toque con `/voice tap`. Con la entrada de indicación vacía, toque la `Barra espaciadora` para comenzar a grabar. El pie de página muestra una forma de onda en vivo mientras se graba. Toque la `Barra espaciadora` nuevamente para detener.

Claude Code inserta la transcripción y envía la indicación automáticamente cuando la transcripción tiene al menos tres palabras. Las transcripciones más cortas se insertan pero no se envían, por lo que un toque accidental no envía una palabra extraviada.

El umbral de tres palabras cuenta palabras para idiomas escritos sin espacios. A partir de v2.1.195, las transcripciones en japonés, chino y tailandés cuentan palabras individuales, por lo que se envían automáticamente en modo de toque y en modo de retención con `autoSubmit`. Las versiones anteriores contaban una transcripción sin espacios como una palabra y nunca la enviaban automáticamente.

El primer toque solo comienza a grabar cuando la entrada de indicación está vacía, por lo que aún puede escribir espacios normalmente mientras compone un mensaje. El segundo toque detiene la grabación independientemente del contenido de entrada. La grabación también se detiene automáticamente después de 15 segundos de silencio o dos minutos en total.

<h2 id="change-the-dictation-language">
  Cambiar el idioma del dictado
</h2>

El dictado de voz utiliza la misma [configuración de `language`](/docs/es/settings) que controla el idioma de respuesta de Claude. Si esa configuración está vacía, el dictado predeterminado es inglés. En la extensión de VS Code, si `language` está vacío, el dictado utiliza la configuración `accessibility.voice.speechLanguage` de VS Code antes de predeterminar al inglés.

<Accordion title="Idiomas de dictado admitidos">
  | Idioma    | Código |
  | :-------- | :----- |
  | Checo     | `cs`   |
  | Danés     | `da`   |
  | Holandés  | `nl`   |
  | Inglés    | `en`   |
  | Francés   | `fr`   |
  | Alemán    | `de`   |
  | Griego    | `el`   |
  | Hindi     | `hi`   |
  | Indonesio | `id`   |
  | Italiano  | `it`   |
  | Japonés   | `ja`   |
  | Coreano   | `ko`   |
  | Noruego   | `no`   |
  | Polaco    | `pl`   |
  | Portugués | `pt`   |
  | Ruso      | `ru`   |
  | Español   | `es`   |
  | Sueco     | `sv`   |
  | Turco     | `tr`   |
  | Ucraniano | `uk`   |
</Accordion>

Establezca el idioma en `/config` o directamente en la configuración. Puede usar el [código de idioma BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) o el nombre del idioma:

```json theme={null}
{
  "language": "japanese"
}
```

Si su configuración de `language` no está en la lista admitida, `/voice` le advierte al habilitar y vuelve al inglés para el dictado. Las respuestas de texto de Claude no se ven afectadas por esta alternativa.

<h2 id="rebind-the-dictation-key">
  Reenlazar la tecla de dictado
</h2>

La tecla de dictado está vinculada a `voice:pushToTalk` en el contexto `Chat` y predeterminada a `Space`. El mismo enlace controla los modos de mantener y tocar. Reenlácelo en [`~/.claude/keybindings.json`](/docs/es/keybindings):

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

La acción `voice:pushToTalk` utiliza una tecla a la vez. Cuando enlaza una tecla personalizada, reemplaza el enlace predeterminado `Space` en lugar de agregar un segundo disparador, por lo que la línea `"space": null` en este ejemplo es para mayor claridad y puede omitirse sin cambiar el comportamiento.

En modo de mantener, evite enlazar una tecla de letra simple como `v` ya que la detección de mantener se basa en la repetición de teclas y la letra se escribe en la indicación durante el calentamiento. Use `Space`, o use una combinación de modificador como `meta+k` para comenzar a grabar en la primera pulsación de tecla sin calentamiento. El modo de toque no tiene calentamiento, por lo que la mayoría de las teclas funcionan.

Algunas teclas no se entregan a las aplicaciones de terminal y no se pueden enlazar en absoluto. Por ejemplo, `Caps Lock` muestra un error si intenta enlazarlo. Consulte [personalizar atajos de teclado](/docs/es/keybindings) para la sintaxis completa de enlace de teclas y la lista de atajos reservados.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Problemas comunes cuando el dictado de voz no se activa o no graba:

* **`Voice mode requires a Claude.ai account`**: está autenticado con una clave de API o un proveedor de terceros. Ejecute `/login` para iniciar sesión con una cuenta de Claude.ai.
* **`Voice mode is disabled by your organization's policy`**: la configuración de cumplimiento de su organización desactiva el dictado de voz, como se describe en [Requisitos](#requirements). Póngase en contacto con el administrador de su organización para confirmar si el dictado de voz está disponible para su organización.
* **`Microphone access is denied`**: otorgue permiso de micrófono a su terminal en la configuración del sistema. En macOS, vaya a Configuración del Sistema → Privacidad y Seguridad → Micrófono y habilite su aplicación de terminal, luego ejecute `/voice` nuevamente. En Windows, vaya a Configuración → Privacidad y seguridad → Micrófono y active el acceso al micrófono para aplicaciones de escritorio, luego ejecute `/voice` nuevamente. Si su terminal no aparece en la configuración de macOS, consulte [Terminal no aparece en la configuración de micrófono de macOS](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` en Linux**: el módulo de audio nativo no pudo cargarse y no hay alternativa instalada. Instale SoX con el comando que se muestra en el mensaje de error, por ejemplo `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX está instalado, pero el host no tiene un dispositivo de captura de audio, por ejemplo un servidor sin interfaz gráfica o un contenedor. Ejecute Claude Code en una máquina con micrófono. A partir de v2.1.195, Claude Code en Linux reporta este mensaje en esa situación; las versiones anteriores le pedían que instalara SoX incluso cuando ya estaba instalado.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg enruta el audio a través de PulseAudio en lugar de un dispositivo ALSA, por lo que SoX necesita que su backend de PulseAudio esté instalado explícitamente. Ejecute `sudo apt install sox libsox-fmt-pulse`. Instalar `sox` solo extrae el backend de ALSA, que no puede grabar en WSL porque no hay un dispositivo `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`**: el dictado de voz encontró varios fallos de captura seguidos y dejó de intentar nuevas sesiones hasta que una tenga éxito. Un fallo cuenta si el micrófono no se inicia o el grabador se inicia y luego se detiene sin producir ningún audio. Esto generalmente significa que el micrófono o la pila de audio en este host no puede capturar audio, por ejemplo un servidor sin interfaz gráfica, un shell remoto sin paso de audio, o un permiso de micrófono denegado. Confirme un dispositivo de entrada que funcione, corrija la causa subyacente de las entradas anteriores, luego active la voz nuevamente. Antes de v2.1.202, solo los fallos de inicio contaban hacia la pausa.
* **Nada sucede al mantener presionada la `Barra espaciadora` en modo de mantener**: observe la entrada de indicación mientras mantiene presionada. Si los espacios siguen acumulándose, el dictado de voz probablemente esté desactivado; ejecute `/voice hold` para habilitarlo. Si solo aparecen uno o dos espacios y luego nada, el dictado de voz está activado pero la detección de mantener no se activa. La detección de mantener requiere que su terminal envíe eventos de repetición de teclas, por lo que no puede detectar una tecla mantenida si la repetición de teclas está deshabilitada a nivel del sistema operativo. Cambie al modo de toque con `/voice tap` para evitar el requisito de repetición de teclas.
* **Tocar la `Barra espaciadora` escribe un espacio en lugar de grabar en modo de toque**: el primer toque solo comienza a grabar cuando la entrada de indicación está vacía. Borre la entrada primero, o verifique que esté en modo de toque ejecutando `/voice tap`.
* **`No audio detected from microphone`**: la grabación comenzó pero capturó silencio. Confirme que el dispositivo de entrada correcto está configurado como predeterminado del sistema y que su nivel de entrada no está silenciado ni cerca de cero. En Windows, abra Configuración → Sistema → Sonido → Entrada y seleccione su micrófono. En macOS, abra Configuración del Sistema → Sonido → Entrada.
* **`Voice connection failed`**: su grabación nunca llegó al servicio de transcripción porque la conexión falló. Verifique su red e intente nuevamente. Una grabación que no captura audio reporta `No audio detected from microphone` en lugar de este mensaje. Antes de v2.1.200, un micrófono silencioso podría reportar un fallo de conexión, lo que sugería un problema de red cuando el problema real era el dispositivo de entrada.
* **`No speech detected`**: el audio llegó al servicio de transcripción pero no se reconocieron palabras. Hable más cerca del micrófono, reduzca el ruido de fondo y confirme que su [idioma de dictado](#change-the-dictation-language) coincida con el idioma que está hablando.
* **La transcripción es confusa o en el idioma incorrecto**: el dictado predeterminado es inglés. Si está dictando en otro idioma, configúrelo en `/config` primero. Consulte [Cambiar el idioma del dictado](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal no aparece en la configuración de micrófono de macOS
</h3>

Si su aplicación de terminal no aparece en Configuración del Sistema → Privacidad y Seguridad → Micrófono, no hay alternancia que pueda habilitar. Restablezca el estado de permiso para su terminal para que la siguiente ejecución de `/voice` active una solicitud de permiso de macOS nueva.

<Steps>
  <Step title="Restablecer el permiso de micrófono para su terminal">
    Ejecute `tccutil reset Microphone <bundle-id>`, reemplazando `<bundle-id>` con el identificador de su terminal: `com.apple.Terminal` para la Terminal integrada, o `com.googlecode.iterm2` para iTerm2. Para otras terminales, busque el identificador con `osascript -e 'id of app "AppName"'`.

    <Warning>
      Puede ejecutar `tccutil reset Microphone` sin un ID de paquete, pero revoca el acceso al micrófono de todas las aplicaciones en su Mac, incluidas aplicaciones como Zoom o Slack. Cada aplicación deberá volver a solicitar acceso en el próximo uso, así que no lo ejecute durante una llamada activa.
    </Warning>
  </Step>

  <Step title="Salir y relanzar su terminal">
    macOS no volverá a solicitar un proceso que ya se está ejecutando. Salga de la aplicación de terminal con Cmd+Q, no solo cierre sus ventanas, luego ábrala nuevamente.
  </Step>

  <Step title="Activar una solicitud nueva">
    Inicie Claude Code y ejecute `/voice`. macOS solicita acceso al micrófono; permítalo.
  </Step>
</Steps>

<h2 id="see-also">
  Ver también
</h2>

* [Personalizar atajos de teclado](/docs/es/keybindings): reenlace `voice:pushToTalk` y otras acciones de teclado de CLI
* [Configurar configuración](/docs/es/settings): referencia completa para `voice`, `language` y otras claves de configuración
* [Modo interactivo](/docs/es/interactive-mode): atajos de teclado, modos de entrada y controles de sesión
* [Comandos](/docs/es/commands): referencia para `/voice`, `/config` y todos los demás comandos
