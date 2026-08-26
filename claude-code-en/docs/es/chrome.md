> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code con Chrome

> Conecta Claude Code a tu navegador Chrome para probar aplicaciones web, depurar con registros de consola, automatizar el relleno de formularios y extraer datos de páginas web.

Claude Code se integra con la [extensión del navegador Claude en Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) para brindarte capacidades de automatización del navegador desde la CLI o la [extensión de VS Code](/docs/es/vs-code#automate-browser-tasks-with-chrome). Construye tu código, luego prueba y depura en el navegador sin cambiar de contexto.

Claude abre nuevas pestañas para tareas del navegador y comparte el estado de inicio de sesión de tu navegador, por lo que puede acceder a cualquier sitio en el que ya hayas iniciado sesión. Las acciones del navegador se ejecutan en una ventana de Chrome visible en tiempo real. Cuando Claude encuentra una página de inicio de sesión o CAPTCHA, se detiene y te pide que lo manejes manualmente.

<Note>
  La integración de Chrome funciona con Google Chrome y Microsoft Edge. Aún no es compatible con Brave, Arc u otros navegadores basados en Chromium. Tampoco es compatible en Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Capacidades
</h2>

Con Chrome conectado, puedes encadenar acciones del navegador con tareas de codificación en un único flujo de trabajo:

* **Depuración en vivo**: lee errores de consola y estado del DOM directamente, luego corrige el código que los causó
* **Verificación de diseño**: construye una interfaz de usuario a partir de un mock de Figma, luego ábrelo en el navegador para verificar que coincida
* **Prueba de aplicaciones web**: prueba la validación de formularios, verifica regresiones visuales o verifica flujos de usuario
* **Aplicaciones web autenticadas**: interactúa con Google Docs, Gmail, Notion o cualquier aplicación en la que hayas iniciado sesión sin conectores de API
* **Extracción de datos**: extrae información estructurada de páginas web y guárdala localmente
* **Automatización de tareas**: automatiza tareas repetitivas del navegador como entrada de datos, relleno de formularios o flujos de trabajo multisitio
* **Grabación de sesión**: graba interacciones del navegador como GIF para documentar o compartir lo que sucedió

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de usar Claude Code con Chrome, necesitas:

* Navegador [Google Chrome](https://www.google.com/chrome/) o [Microsoft Edge](https://www.microsoft.com/edge)
* Extensión [Claude en Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versión 1.0.36 o superior, disponible en la Chrome Web Store para ambos navegadores
* [Claude Code](/docs/es/quickstart#step-1-install-claude-code)
* Un plan directo de Anthropic (Pro, Max, Team o Enterprise)

<Note>
  La integración de Chrome no está disponible a través de proveedores de terceros como Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Si accedes a Claude exclusivamente a través de un proveedor de terceros, necesitas una cuenta separada de claude.ai para usar esta función.
</Note>

<h2 id="get-started-in-the-cli">
  Comenzar en la CLI
</h2>

<Steps>
  <Step title="Lanzar Claude Code con Chrome">
    Inicia Claude Code con la bandera `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    También puedes habilitar Chrome dentro de una sesión existente ejecutando `/chrome`.
  </Step>

  <Step title="Pídele a Claude que use el navegador">
    Este ejemplo navega a una página, interactúa con ella e informa lo que encuentra, todo desde su terminal o editor:

    ```text theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    La primera acción del navegador solicita permiso para usar la skill `claude-in-chrome`. Apruébalo y Claude abre una nueva pestaña e inicia la tarea.
  </Step>
</Steps>

Ejecuta `/chrome` en cualquier momento para verificar el estado de la conexión, administrar permisos, reconectar la extensión o elegir qué navegador conectado usar. Si más de un navegador está conectado cuando comienza una acción del navegador, Claude le solicita que elija uno.

Para VS Code, consulta [automatización del navegador en VS Code](/docs/es/vs-code#automate-browser-tasks-with-chrome).

<h3 id="enable-chrome-by-default">
  Habilitar Chrome de forma predeterminada
</h3>

Para evitar pasar `--chrome` en cada sesión, ejecuta `/chrome` y selecciona "Habilitado de forma predeterminada".

En la [extensión de VS Code](/docs/es/vs-code#automate-browser-tasks-with-chrome), Chrome está disponible siempre que la extensión de Chrome esté instalada. No se necesita ninguna bandera adicional.

<Note>
  Habilitar Chrome de forma predeterminada en la CLI aumenta el uso del contexto ya que las herramientas del navegador siempre se cargan. Si nota un aumento en el consumo de contexto, deshabilite esta configuración y use `--chrome` solo cuando sea necesario.
</Note>

<h3 id="manage-site-permissions">
  Administrar permisos del sitio
</h3>

Los permisos a nivel de sitio se heredan de la extensión de Chrome. Administre los permisos en la configuración de la extensión de Chrome para controlar qué sitios puede examinar Claude, hacer clic e introducir texto.

<h3 id="browser-tools-in-plan-mode">
  Herramientas del navegador en modo plan
</h3>

En [modo plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), las llamadas de herramientas del navegador que solo leen la página o el estado del navegador se ejecutan sin un aviso de permiso, y las llamadas que cambian el estado solicitan aprobación.

* **Llamadas de solo lectura**: `read_page`, `get_page_text`, `find`, lectura de mensajes de consola o solicitudes de red, y captura de pantalla
* **Llamadas que cambian el estado**: clics, escritura, navegación, gestión de pestañas y ventanas, y grabación de un GIF

A partir de v2.1.199, una llamada que de otro modo sería de solo lectura que establece una bandera de entrada que cambia el estado, como `createIfEmpty` en `tabs_context_mcp`, `clear` en los lectores de consola y red, o `save_to_disk` en una captura de pantalla, también solicita aprobación. Una llamada `browser_batch` se ejecuta sin un aviso solo cuando cada acción dentro de ella es de solo lectura.

<h2 id="example-workflows">
  Flujos de trabajo de ejemplo
</h2>

Estos ejemplos muestran formas comunes de combinar acciones del navegador con tareas de codificación. Ejecuta `/mcp`, selecciona `claude-in-chrome` y luego selecciona **Ver herramientas** para ver la lista completa de herramientas del navegador disponibles.

<h3 id="test-a-local-web-application">
  Probar una aplicación web local
</h3>

Al desarrollar una aplicación web, pídele a Claude que verifique que tus cambios funcionen correctamente:

```text theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude navega a tu servidor local, interactúa con el formulario e informa lo que observa.

<h3 id="debug-with-console-logs">
  Depurar con registros de consola
</h3>

Claude puede leer la salida de la consola para ayudar a diagnosticar problemas. Dile a Claude qué patrones buscar en lugar de pedirle toda la salida de la consola, ya que los registros pueden ser detallados:

```text theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude lee los mensajes de la consola y puede filtrar patrones específicos o tipos de error.

<h3 id="automate-form-filling">
  Automatizar el relleno de formularios
</h3>

Acelera tareas repetitivas de entrada de datos:

```text theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude lee tu archivo local, navega por la interfaz web e introduce los datos para cada registro.

<h3 id="draft-content-in-google-docs">
  Redactar contenido en Google Docs
</h3>

Usa Claude para escribir directamente en tus documentos sin configuración de API:

```text theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude abre el documento, hace clic en el editor e introduce el contenido. Esto funciona con cualquier aplicación web en la que hayas iniciado sesión: Gmail, Notion, Sheets y más.

<h3 id="extract-data-from-web-pages">
  Extraer datos de páginas web
</h3>

Extrae información estructurada de sitios web:

```text theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude navega a la página, lee el contenido y compila los datos en un formato estructurado.

<h3 id="run-multi-site-workflows">
  Ejecutar flujos de trabajo multisitio
</h3>

Coordina tareas en múltiples sitios web:

```text theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude trabaja en pestañas para recopilar información y completar el flujo de trabajo.

<h3 id="record-a-demo-gif">
  Grabar un GIF de demostración
</h3>

Crea grabaciones compartibles de interacciones del navegador:

```text theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude graba la secuencia de interacción y la guarda como un archivo GIF.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="extension-not-detected">
  Extensión no detectada
</h3>

Si Claude Code no puede detectar la extensión de Chrome:

1. Verifica que la extensión de Chrome esté instalada y habilitada en `chrome://extensions`
2. Verifica que Claude Code esté actualizado ejecutando `claude --version`
3. Comprueba que Chrome se está ejecutando
4. Ejecuta `/chrome` y selecciona "Reconectar extensión" para restablecer la conexión
5. Si el problema persiste, reinicia tanto Claude Code como Chrome

La primera vez que habilitas la integración de Chrome, Claude Code instala un archivo de configuración del host de mensajería nativa. Chrome lee este archivo al iniciarse, por lo que si la extensión no se detecta en tu primer intento, reinicia Chrome para recoger la nueva configuración.

A partir de v2.1.199, Claude Code abre una pestaña del navegador pidiéndote que conectes la extensión solo en esa primera instalación. Las sesiones posteriores que reescriben el archivo de configuración, por ejemplo después de cambiar compilaciones de Claude Code o directorios de configuración, no la vuelven a abrir.

Si la conexión aún falla, verifica que el archivo de configuración del host exista en:

Para Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: comprueba `HKCU\Software\Google\Chrome\NativeMessagingHosts\` en el Registro de Windows

Para Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: comprueba `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` en el Registro de Windows

<h3 id="browser-not-responding">
  El navegador no responde
</h3>

Si los comandos del navegador de Claude dejan de funcionar:

1. Comprueba si un cuadro de diálogo modal (alerta, confirmación, solicitud) está bloqueando la página. Los cuadros de diálogo de JavaScript bloquean eventos del navegador e impiden que Claude reciba comandos. Descarta el cuadro de diálogo manualmente, luego dile a Claude que continúe.
2. Pídele a Claude que cree una nueva pestaña e intente de nuevo
3. Reinicia la extensión de Chrome deshabilitándola y volviéndola a habilitar en `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  La conexión se cae durante sesiones largas
</h3>

El trabajador de servicio de la extensión de Chrome puede quedarse inactivo durante sesiones extendidas, lo que rompe la conexión. Si las herramientas del navegador dejan de funcionar después de un período de inactividad, ejecuta `/chrome` y selecciona "Reconectar extensión".

<h3 id="windows-specific-issues">
  Problemas específicos de Windows
</h3>

En Windows, puedes encontrar:

* **Conflictos de tuberías nombradas (EADDRINUSE)**: si otro proceso está usando la misma tubería nombrada, reinicia Claude Code. Cierra cualquier otra sesión de Claude Code que pueda estar usando Chrome.
* **Errores del host de mensajería nativa**: si el host de mensajería nativa falla al iniciarse, intenta reinstalar Claude Code para regenerar la configuración del host.

<h3 id="common-error-messages">
  Mensajes de error comunes
</h3>

Estos son los errores más frecuentes y cómo resolverlos:

| Error                                          | Causa                                                          | Solución                                                               |
| ---------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------------- |
| "La extensión del navegador no está conectada" | El host de mensajería nativa no puede alcanzar la extensión    | Reinicia Chrome y Claude Code, luego ejecuta `/chrome` para reconectar |
| "Extensión no detectada"                       | La extensión de Chrome no está instalada o está deshabilitada  | Instala o habilita la extensión en `chrome://extensions`               |
| "No hay pestaña disponible"                    | Claude intentó actuar antes de que una pestaña estuviera lista | Pídele a Claude que cree una nueva pestaña e intente de nuevo          |
| "El extremo receptor no existe"                | El trabajador de servicio de la extensión se quedó inactivo    | Ejecuta `/chrome` y selecciona "Reconectar extensión"                  |

<h2 id="see-also">
  Ver también
</h2>

* [Usar la computadora](/docs/es/computer-use): controlar aplicaciones nativas de macOS cuando una tarea no se puede realizar en un navegador
* [Usar Claude Code en VS Code](/docs/es/vs-code#automate-browser-tasks-with-chrome): automatización del navegador en la extensión de VS Code
* [Referencia de CLI](/docs/es/cli-reference): banderas de línea de comandos incluyendo `--chrome`
* [Flujos de trabajo comunes](/docs/es/common-workflows): más formas de usar Claude Code
* [Datos y privacidad](/docs/es/data-usage): cómo Claude Code maneja sus datos
* [Comenzar con Claude en Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): documentación completa para la extensión de Chrome, incluyendo atajos de teclado, programación y permisos
