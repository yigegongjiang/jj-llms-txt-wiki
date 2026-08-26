> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Elegir un modo de permisos

> Controle si Claude solicita aprobación antes de editar archivos o ejecutar comandos. Cambie de modo con Mayús+Tab en la CLI o use el selector de modo en VS Code, Desktop y claude.ai.

Cuando Claude desea editar un archivo, ejecutar un comando de shell o realizar una solicitud de red, se detiene y le solicita que apruebe la acción. Los modos de permisos controlan con qué frecuencia ocurre esa pausa. El modo que elija determina el flujo de una sesión: el modo Manual le permite revisar cada acción a medida que llega, mientras que los modos más flexibles permiten que Claude trabaje en tramos más largos sin interrupciones e informe cuando haya terminado. Elija más supervisión para trabajos sensibles, o menos interrupciones cuando confíe en la dirección.

<h2 id="available-modes">
  Modos disponibles
</h2>

Cada modo realiza un equilibrio diferente entre conveniencia y supervisión. La tabla a continuación muestra qué puede hacer Claude sin un aviso de permiso en cada modo.

| Modo                                                                | Qué se ejecuta sin preguntar                                                                                    | Mejor para                                      |
| :------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- | :---------------------------------------------- |
| `default`                                                           | Solo lectura                                                                                                    | Comenzar, trabajo sensible                      |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lecturas, ediciones de archivos y comandos comunes del sistema de archivos (`mkdir`, `touch`, `mv`, `cp`, etc.) | Iterar sobre código que está revisando          |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Solo lectura                                                                                                    | Explorar una base de código antes de cambiarla  |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Todo, con comprobaciones de seguridad en segundo plano                                                          | Tareas largas, reducir fatiga de avisos         |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Solo herramientas preaprobadas                                                                                  | CI restringido y scripts                        |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Todo                                                                                                            | Solo contenedores aislados y máquinas virtuales |

El modo que revisa cada acción se llama **Manual** en la CLI, en `claude --help`, en las extensiones de VS Code y JetBrains, y en la aplicación de escritorio. Su valor de configuración es `default`, que es lo que utilizan los hooks e integraciones de SDK. La CLI acepta `manual` como un alias en cualquier lugar donde escriba el valor, por ejemplo `claude --permission-mode manual` o `"defaultMode": "manual"`. La etiqueta Manual y el alias `manual` requieren Claude Code v2.1.200 o posterior. La etiqueta de la aplicación de escritorio no depende de su versión de CLI.

En todos los modos excepto `bypassPermissions`, las escrituras en [rutas protegidas](#protected-paths) nunca se aprueban automáticamente, protegiendo el estado del repositorio y la configuración de Claude contra corrupción accidental.

Los modos establecen la línea base. Superponga [reglas de permisos](/docs/es/permissions#manage-permissions) en la parte superior para preaprobación o bloqueo de herramientas específicas. Las reglas de denegación, las reglas de solicitud explícita, la [configuración de `ask` de la organización en herramientas de conector](/docs/es/mcp#organization-controls-on-connector-tools) y el marcador [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se aplican en todos los modos, incluido `bypassPermissions`. Las reglas de permitir no tienen efecto en ese modo porque todo lo demás ya está aprobado.

<h2 id="switch-permission-modes">
  Cambiar modos de permisos
</h2>

Puede cambiar modos durante una sesión, al iniciar o como configuración predeterminada persistente. El modo se establece a través de estos controles, no pidiendo a Claude en el chat. Seleccione su interfaz a continuación para ver cómo cambiarlo.

<Tabs>
  <Tab title="CLI">
    **Durante una sesión**: presione `Shift+Tab` para ciclar `default` → `acceptEdits` → `plan`. El modo actual aparece en la barra de estado. El modo manual, `default` en ese ciclo, muestra una insignia gris `⏸ manual mode on`. Antes de v2.1.203, la barra de estado no mostraba ninguna insignia en modo Manual.

    No todos los modos están en el ciclo predeterminado:

    * `auto`: aparece cuando su cuenta cumple con los [requisitos del modo automático](#eliminate-prompts-with-auto-mode); cambiar a él cambia modos sin una solicitud de confirmación
    * `bypassPermissions`: aparece después de que inicia con `--permission-mode bypassPermissions`, `--dangerously-skip-permissions` o `--allow-dangerously-skip-permissions`; la variante `--allow-` añade el modo al ciclo sin activarlo
    * `dontAsk`: nunca aparece en el ciclo; establézcalo con `--permission-mode dontAsk`

    Los modos opcionales habilitados se insertan después de `plan`, con `bypassPermissions` primero y `auto` último. Si tiene ambos habilitados, ciclará a través de `bypassPermissions` en el camino a `auto`.

    **Al iniciar**: pase el modo como una bandera.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Como predeterminado**: establezca `defaultMode` en [configuración](/docs/es/settings#settings-files).

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    La misma bandera `--permission-mode` funciona con `-p` para [ejecuciones no interactivas](/docs/es/headless).
  </Tab>

  <Tab title="VS Code">
    **Durante una sesión**: haga clic en el indicador de modo en la parte inferior del cuadro de solicitud.

    **Como predeterminado**: establezca `claudeCode.initialPermissionMode` en la configuración de VS Code, o use el panel de configuración de la extensión Claude Code.

    El indicador de modo muestra estas etiquetas, asignadas al modo que cada una aplica:

    | Etiqueta de interfaz   | Modo                |
    | :--------------------- | :------------------ |
    | Manual                 | `default`           |
    | Editar automáticamente | `acceptEdits`       |
    | Plan                   | `plan`              |
    | Auto                   | `auto`              |
    | Omitir permisos        | `bypassPermissions` |

    Antes de v2.1.205, la extensión etiquetaba `plan` como Plan mode y `auto` como Auto mode.

    El modo Auto aparece en el indicador de modo cuando su cuenta cumple con todos los requisitos enumerados en la [sección de modo automático](#eliminate-prompts-with-auto-mode). La configuración `claudeCode.initialPermissionMode` no acepta `auto`. Para iniciar en modo automático de forma predeterminada, establezca `defaultMode` en su [configuración de usuario](/docs/es/settings#settings-files) en su lugar. Claude Code ignora `defaultMode: "auto"` en la configuración de proyecto y local.

    Omitir permisos requiere el toggle **Allow dangerously skip permissions** en la configuración de la extensión antes de que aparezca en el indicador de modo.

    Consulte la [guía de VS Code](/docs/es/vs-code) para obtener detalles específicos de la extensión.
  </Tab>

  <Tab title="JetBrains">
    El plugin de JetBrains ejecuta Claude Code en la terminal del IDE, por lo que cambiar modos funciona igual que en la CLI: presione `Shift+Tab` para ciclar, o pase `--permission-mode` al iniciar.
  </Tab>

  <Tab title="Desktop">
    **Durante una sesión**: use el selector de modo junto al botón de envío. No todos los modos aparecen en el selector:

    * **Auto**: aparece cuando su cuenta cumple con los [requisitos del modo automático](#eliminate-prompts-with-auto-mode)
    * **Omitir permisos**: requiere el toggle **Allow bypass permissions mode** en la configuración de Desktop en planes Pro y Max; en planes Team y Enterprise, la política de la organización lo controla en su lugar

    Para obtener detalles específicos de desktop, consulte [Elegir un modo de permisos](/docs/es/desktop#choose-a-permission-mode) en la guía de Desktop.

    **Como predeterminado**: establezca `defaultMode` en [configuración](/docs/es/settings#settings-files). La aplicación de desktop lee los mismos archivos de configuración que la CLI y aplica el modo a nuevas sesiones locales.

    Un modo que elige en el selector de modo se recuerda por carpeta y tiene prioridad sobre `defaultMode` para esa carpeta. Plan es la excepción: seleccionarlo se aplica solo a la sesión actual.

    Este ejemplo establece el modo Plan como predeterminado para nuevas sesiones locales:

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    Use el menú desplegable de modo junto al cuadro de solicitud en [claude.ai/code](https://claude.ai/code) o en la aplicación móvil. Los avisos de permisos aparecen en claude.ai para aprobación. Los modos que aparecen dependen de dónde se ejecute la sesión:

    * **Sesiones en la nube** en [Claude Code en la web](/docs/es/claude-code-on-the-web): Aceptar ediciones, Plan y Auto. Aceptar ediciones corresponde al modo `default`: el entorno en la nube aprueba previamente ediciones de archivos independientemente del modo, por lo que el menú desplegable muestra Aceptar ediciones en lugar de Manual. Las sesiones en la nube aún respetan `defaultMode: "acceptEdits"` de la configuración. El modo Auto aparece solo cuando su organización lo permite y el modelo seleccionado lo admite. Omitir permisos no está disponible.
    * **Sesiones de [Control Remoto](/docs/es/remote-control)** en su máquina local: Manual, Aceptar ediciones y Plan. No puede seleccionar Auto u Omitir permisos desde la aplicación. El menú desplegable muestra el modo en el que se encuentra la sesión local, incluido un modo establecido desde la terminal, y se actualiza cuando el modo cambia en la aplicación o en la terminal. La única excepción es Omitir permisos: la sesión nunca reporta ese modo a claude.ai, por lo que cambiar a él desde la terminal no cambia lo que muestra el menú desplegable. Antes de v2.1.202, las sesiones conectadas con `/remote-control` o `claude --remote-control` no reportaban su modo en absoluto, por lo que claude.ai y la aplicación móvil podrían mostrar un modo en el que la sesión no estaba. La discrepancia afectó solo la etiqueta: Claude Code generó avisos de permisos desde el modo real de la sesión, y aún aparecieron en la aplicación para aprobación.

    Para Control Remoto, también puede establecer el modo de inicio al iniciar el host:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Aprobar automáticamente ediciones de archivos con el modo acceptEdits
</h2>

El modo `acceptEdits` permite que Claude cree y edite archivos en su directorio de trabajo sin solicitar confirmación. La barra de estado muestra `⏵⏵ accept edits on` mientras este modo está activo.

Además de las ediciones de archivos, el modo `acceptEdits` aprueba automáticamente comandos Bash comunes del sistema de archivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` y `sed`. Estos comandos también se aprueban automáticamente cuando van precedidos de variables de entorno seguras como `LANG=C` o `NO_COLOR=1`, o envoltorios de procesos como `timeout`, `nice` o `nohup`. Al igual que las ediciones de archivos, la aprobación automática se aplica solo a las rutas dentro de su directorio de trabajo o `additionalDirectories`. Las rutas fuera de ese alcance, las escrituras en [rutas protegidas](#protected-paths) y todos los demás comandos Bash excepto el [conjunto integrado de solo lectura](/docs/es/permissions#read-only-commands) aún solicitan confirmación.

Cuando la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) está habilitada, el modo `acceptEdits` también aprueba automáticamente `Set-Content`, `Add-Content`, `Clear-Content` y `Remove-Item` en rutas dentro del alcance, junto con sus alias comunes. Se aplican las mismas reglas de alcance y rutas protegidas.

Utilice `acceptEdits` cuando desee revisar los cambios en su editor o mediante `git diff` después del hecho en lugar de aprobar cada edición en línea.

Presione `Shift+Tab` una vez desde el modo Manual para entrar en él, o comience directamente con él:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analice antes de editar con el modo plan
</h2>

El modo plan le indica a Claude que investigue y proponga cambios sin realizarlos. Claude lee archivos, ejecuta comandos de shell para explorar y escribe un plan, pero no edita su fuente. Los avisos de permiso se aplican como lo hacen en el modo Manual a menos que [el modo automático](/docs/es/auto-mode-config) esté disponible y `useAutoModeDuringPlan` esté activado, que es el valor predeterminado. Con el modo automático activo, el clasificador aprueba comandos de solo lectura como búsquedas y lecturas de archivos sin solicitar confirmación. Las ediciones permanecen bloqueadas de cualquier forma hasta que usted apruebe el plan.

Ingrese al modo plan presionando `Shift+Tab` o prefijando un único aviso con `/plan`. También puede comenzar en modo plan desde la CLI:

```bash theme={null}
claude --permission-mode plan
```

Presione `Shift+Tab` nuevamente para salir del modo plan sin aprobar un plan.

<h3 id="review-and-approve-a-plan">
  Revise y apruebe un plan
</h3>

Cuando el plan esté listo, Claude lo presenta y pregunta cómo proceder. Desde ese aviso puede:

* Aprobar e iniciar en modo automático
* Aprobar y aceptar ediciones
* Aprobar y revisar cada edición manualmente
* Continuar planificando con comentarios
* Refinar con [Ultraplan](/docs/es/ultraplan) para revisión basada en navegador

Aprobar un plan sale del modo plan y cambia la sesión al modo de permiso que describe cada opción de aprobación, por lo que Claude comienza a editar. Para planificar nuevamente, vuelva al modo plan con `Shift+Tab`, o prefije su próximo aviso con `/plan`.

Presione `Ctrl+G` para abrir el plan propuesto en su editor de texto predeterminado y edítelo directamente antes de que Claude continúe. Cuando [`showClearContextOnPlanAccept`](/docs/es/settings#available-settings) está habilitado, cada opción de aprobación también ofrece borrar el contexto de planificación primero.

Aceptar un plan también nombra la sesión automáticamente a partir del contenido del plan, a menos que ya haya establecido un nombre con `--name` o `/rename`.

<h3 id="set-plan-mode-as-the-default">
  Establezca el modo plan como predeterminado
</h3>

Para hacer que el modo plan sea el predeterminado para un proyecto, establezca `defaultMode` en `.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  Eliminar solicitudes de permiso con modo automático
</h2>

El modo automático permite que Claude se ejecute sin solicitudes de permiso rutinarias. Un modelo clasificador separado revisa las acciones antes de que se ejecuten, bloqueando cualquier cosa que escale más allá de su solicitud, se dirija a infraestructura no reconocida o parezca impulsada por contenido hostil que Claude haya leído. Las [reglas de solicitud](/docs/es/permissions#manage-permissions) explícitas aún fuerzan una solicitud.

Las eliminaciones dirigidas al directorio raíz del sistema de archivos o al directorio de inicio, como `rm -rf /` y `rm -rf ~`, solicitan aprobación en lugar de ir al clasificador. Esta solicitud también se activa cuando el comando contiene sustitución de comandos con `$(...)` o comillas invertidas, o sustitución de procesos con `<(...)`, ya sea que la eliminación esté dentro de la sustitución, como en `echo "$(rm -rf ~)"`, o en otro lugar del mismo comando. Antes de v2.1.208, los comandos que contenían esas formas iban al clasificador en lugar de solicitar.

El modo automático también anima a Claude a seguir trabajando sin detenerse para hacer preguntas aclaratorias, aunque Claude aún pregunta cuando su solicitud o una skill depende explícitamente de ello. Para un comportamiento más autónomo mientras se mantienen las solicitudes de permiso, establezca el [estilo de salida proactivo](/docs/es/output-styles) en su lugar.

<Warning>
  El modo automático reduce las solicitudes de permiso pero no garantiza la seguridad. Úselo para tareas donde confía en la dirección general, no como reemplazo de revisión en operaciones sensibles.
</Warning>

El modo automático está disponible solo cuando su cuenta cumple con todos estos requisitos:

* **Plan**: Todos los planes.
* **Propietario**: en Team y Enterprise, un Propietario debe habilitarlo en [configuración de administrador de Claude Code](https://claude.ai/admin-settings/claude-code) antes de que los usuarios puedan activarlo. Los administradores también pueden desactivar el modo automático estableciendo `permissions.disableAutoMode` en `"disable"` en [configuración administrada](/docs/es/permissions#managed-settings). Para la pestaña Code de la aplicación de escritorio, `disableAutoMode` es el control a nivel de organización, y el toggle de configuración de administrador no se aplica.
* **Modelo**: en la API de Anthropic, Claude Opus 4.6 o posterior, o Sonnet 4.6 o posterior. En Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry y sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, solo Claude Sonnet 5, Opus 4.7 y Opus 4.8. Los modelos más antiguos, incluidos Sonnet 4.5, Opus 4.5, Haiku y modelos claude-3, no son compatibles en ningún proveedor.
* **Proveedor**: disponible por defecto en la API de Anthropic, Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry y sesiones de puerta de enlace de aplicaciones Claude con sesión iniciada. En v2.1.158 a v2.1.206, el modo automático estaba desactivado en todos estos proveedores excepto la API de Anthropic hasta que estableciera `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 eliminó el requisito.

Si Claude Code informa que el modo automático no está disponible, uno de estos requisitos no se cumple; esto no es una interrupción transitoria. Un mensaje separado que nombre un modelo y diga que el modo automático "no puede determinar la seguridad" de una acción es una interrupción transitoria del clasificador; consulte la [referencia de errores](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action).

Si establece `defaultMode: "auto"` en [configuración](/docs/es/settings#available-settings) y la sesión comienza en modo `default` sin error, la configuración probablemente esté en `.claude/settings.json` o `.claude/settings.local.json`. Claude Code v2.1.142 y posterior ignoran `auto` de esos archivos para que un repositorio no pueda otorgarse a sí mismo modo automático. Muévalo a `~/.claude/settings.json`.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Modo automático en Bedrock, Agent Platform o Foundry
</h3>

En [Amazon Bedrock](/docs/es/amazon-bedrock), [Agent Platform de Google Cloud](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry) y sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, el modo automático aparece en el ciclo `Shift+Tab` por defecto. Aparecer en el ciclo no cambia el modo en que comienza una sesión: las sesiones aún comienzan en su [`defaultMode`](/docs/es/settings#available-settings), que es Manual a menos que lo cambie. Solo Claude Sonnet 5, Opus 4.7 y Opus 4.8 son compatibles en estos proveedores.

Para hacer que el modo automático sea el modo de inicio predeterminado, establezca `"permissions": {"defaultMode": "auto"}` en la configuración de usuario o administrada.

Para evitar que los desarrolladores usen el modo automático, establezca `disableAutoMode` en `"disable"` en [configuración administrada](/docs/es/permissions#managed-settings). Esto elimina `auto` del ciclo `Shift+Tab` y rechaza `--permission-mode auto` al inicio.

En v2.1.158 a v2.1.206, el modo automático estaba desactivado en estos proveedores hasta que estableciera `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, y Claude Code ignoraba `defaultMode: "auto"` en estos proveedores a menos que la variable también estuviera establecida. La variable aún se acepta por compatibilidad y no tiene efecto desde v2.1.207 en adelante.

<h3 id="what-the-classifier-blocks-by-default">
  Qué bloquea el clasificador por defecto
</h3>

El clasificador confía en su directorio de trabajo y en los remotos que se configuraron para él cuando comenzó la sesión. Un remoto agregado o reapuntado durante la sesión con `git remote add` o `git remote set-url` no es de confianza, y todo lo demás se trata como externo hasta que [configure infraestructura de confianza](/docs/es/auto-mode-config). Antes de v2.1.200, los remotos agregados a mitad de sesión también eran de confianza.

**Bloqueado por defecto**:

* Descargar y ejecutar código, como `curl | bash`
* Enviar datos sensibles a puntos finales externos
* Implementaciones y migraciones de producción
* Eliminación masiva en almacenamiento en la nube
* Otorgar permisos de IAM o repositorio
* Modificar infraestructura compartida
* Destruir irreversiblemente archivos que existían antes de la sesión
* Force push
* Hacer push a la rama predeterminada del repositorio cuando el push contiene contenido sensible como secretos o datos personales o confiados, contiene cambios ocultos o mal descritos en relación con lo que pidió, contiene contenido portado o leído por primera vez desde fuera del repositorio, o evita una solicitud de extracción, revisión o verificación que pidió. Un push simple a la rama predeterminada no se bloquea por sí solo, y levantar el bloqueo de un push marcado requiere nombrar el contenido marcado o la revisión omitida, no solo el push. El clasificador es una capa: las [reglas `permissions.deny`](/docs/es/permissions#manage-permissions) se aplican en todos los modos y pueden bloquear pushes a la rama predeterminada completamente, y la protección de rama del remoto aún se aplica. Antes de v2.1.203, cualquier push directo a la rama predeterminada se bloqueaba
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` o `git stash clear`, que el clasificador presume descartaría cambios sin confirmar
* `git commit --amend` cuando el commit en HEAD no fue creado en esta sesión
* Desde v2.1.198, `git commit --amend` cuando el commit en HEAD ya ha sido enviado. Un reword solo de mensaje no se bloquea: `--amend -m` sin nada recién preparado, en un commit que Claude creó durante esta sesión
* `terraform destroy`, `pulumi destroy`, `cdk destroy` o `terragrunt destroy`, y aplicar un plan que destruya recursos

Claude Code v2.1.195 y posterior bloquean más categorías por defecto. Varias dependen de entradas de [entorno](/docs/es/auto-mode-config#define-trusted-infrastructure), como objetivos remotos sensibles y alcances de IaC protegidos, que puede reducir a nombres concretos.

* Escribir en un gestor de secretos, o cambiar registros DNS o certificados TLS
* Fusionar una solicitud de extracción que ningún humano ha aprobado, aprobar la propia solicitud de extracción de Claude o deshabilitar verificaciones de CI
* Publicar un comentario que es en sí mismo un comando para automatización, como `atlantis apply` o `/deploy` o `/merge` de un bot
* Alternar, aumentar gradualmente o eliminar una bandera de característica de producción
* Aplicar cambios de infraestructura a un alcance de IaC protegido, o drenar y eliminar nodos de clúster
* Escrituras en un clúster de cómputo compartido que van más allá del recurso que nombró, como un selector de etiqueta o `--all` que captura trabajos de otros usuarios
* Crear recursos de Kubernetes que se ejecutan en cada nodo o interceptan tráfico de clúster, como DaemonSets y webhooks de admisión
* Shells interactivos o port-forwards en un objetivo remoto sensible
* Abrir un túnel o shell inverso que hace que un servicio local sea accesible desde la internet pública
* Imprimir una credencial o token en vivo en la transcripción o un archivo
* Acceder a una ubicación listada como ubicación de datos sensibles en su [entorno](/docs/es/auto-mode-config#define-trusted-infrastructure), o copiar datos de una. A partir de v2.1.198, esto también bloquea enviar datos de uno a una audiencia que la entrada excluye
* Enrutar una instalación de paquete alrededor de su registro de paquetes interno a un registro público. A partir de v2.1.198, esto también se aplica cuando le ha dicho a Claude que existe un registro interno o espejo en la conversación, no solo cuando uno está listado en su entorno
* Ejecutar un comando con una bandera que desactiva una protección de seguridad, como `--insecure`
* Lanzar un bucle de agente autónomo que se ejecuta sin aprobación humana o sandbox, como uno iniciado con `--dangerously-skip-permissions` o `--no-sandbox`. A partir de v2.1.198, esto también cubre ejecutar un agente de terceros o harness de evaluación con aislamiento y aprobación por acción deshabilitados, como un runner iniciado con `--yes-always`
* Acciones del navegador de [Claude en Chrome](/docs/es/chrome) que podrían enviar contenido de página, cookies o credenciales fuera del origen

Claude Code v2.1.198 y posterior también bloquean estos por defecto:

* Eliminar archivos en `/tmp`, `$TMPDIR` u otro directorio compartido de scratch o caché por comodín, glob o filtro de edad en lugar de por una ruta nombrada específica
* Incluir detalles sensibles en contenido enviado, cargado, publicado o escrito a otras personas o sistemas compartidos, cuando su propio mensaje no autorizó esos detalles para ese destinatario. Los cuerpos de PR e issue, mensajes de commit y comentarios cuentan como este tipo de contenido saliente cuando el repositorio está fuera del límite de confianza o es público, incluidos los repositorios públicos de su propia organización; las rutas de archivo internas, nombres en código, datos de respuesta de API en vivo como correos electrónicos o identificadores de cuenta e identificadores de infraestructura cuentan como detalles sensibles. El alcance de PR, issue y mensaje de commit requiere Claude Code v2.1.200 o posterior. Los datos personales en vivo de una respuesta de API en un cuerpo de PR o issue, como una dirección de correo electrónico, un identificador de cuenta u organización o una métrica de uso, requieren que nombre esos detalles y el destinatario independientemente de la visibilidad del repositorio o límite de confianza. Esa verificación requiere Claude Code v2.1.203 o posterior
* Enviar pulsaciones de teclas a su propio panel tmux de Claude Code para conducir su propia interfaz, que el clasificador trata como Claude cambiando sus propios permisos u supervisión

Claude Code v2.1.200 y posterior también bloquean estos por defecto:

* Comentar, eliminar o pasar forzadamente una prueba o aserción que proteja el comportamiento de seguridad, como autenticación, control de acceso, validación de entrada o sandboxing
* Eliminar o desmantelar un recurso con estado que Claude no creó en la sesión, cuando no se aplica ninguna regla de eliminación más específica y no nombró ese recurso
* Reapuntar una URL base de API, punto final de proxy, receptor de webhook o espejo de registro en un host de terceros que no se ajusta a la tarea, incluidos en archivos de ejemplo como `.env.example`
* Cambiar dónde van los pushes con `git remote set-url` o `git remote add`, a menos que nombre el nuevo remoto
* Hacer push de secretos o datos personales o confiados a un repositorio conocido como público, o hacer push de material confidencial allí que no sea parte del trabajo propio de ese repositorio. El tema propio de un repositorio de dotfiles es la única excepción para datos personales o confiados, y el contenido de un repositorio privado que llega a cualquier superficie pública se bloquea de la misma manera; ambos refinamientos requieren Claude Code v2.1.203 o posterior. Antes de v2.1.203, los datos personales se agrupaban con material confidencial y se bloqueaban solo cuando no eran parte del trabajo propio de ese repositorio. Cuando la visibilidad de un repositorio no está establecida, el clasificador no bloquea solo por eso; juzga el contenido contra las otras reglas en su lugar
* Abrir una solicitud de extracción contra un repositorio u organización diferente, hacer fork con `gh repo fork` o hacer push a un repositorio de terceros, a menos que nombre ese objetivo externo

Claude Code v2.1.203 y posterior también bloquean estos por defecto:

* Contenido de un almacén local sensible, o de un archivo cuyo nombre, ruta o tipo lo marca como sensible, entrando en un commit, un push, texto de PR o issue, un gist o paste, o una publicación de paquete, a menos que nombre tanto la fuente como el destino. Las transcripciones de sesión y registros de conversación, carpetas de punto de credencial y configuración como claves SSH, credenciales en la nube, perfiles de navegador e historial de shell, y exportaciones de datos de usuario cuentan, y que el repositorio sea privado no lo exime

Claude Code v2.1.205 y posterior también bloquean estos por defecto:

* Escribir en transcripciones de sesión de Claude Code, los archivos de historial `.jsonl` bajo `~/.claude/projects/` o su directorio de configuración configurado, ya sea directamente o a través de un comando de shell. La regla también cubre las líneas de metadatos que Claude Code agrega a cada entrada de transcripción para sus propias verificaciones. Una transcripción es estado de sesión que Claude Code escribe, no un archivo de trabajo, y una entrada manipulada llega a cada verificación posterior una vez que reanuda la sesión, por lo que el modo automático bloquea estas escrituras como defensa en profundidad. Leer una transcripción no se bloquea
* Una eliminación forzada recursiva como `rm -rf "$VAR"` o `Remove-Item -Recurse -Force $dir` cuyo objetivo es una variable de shell, o un glob enraizado en una, que no está asignado en ningún lugar de la conversación que el clasificador ve. El valor provino solo de la salida de comando anterior, que el clasificador nunca recibe, por lo que el clasificador no puede verificar el objetivo de eliminación contra las otras reglas de eliminación. El clasificador lee la conversación en lugar de la salida de comando por diseño, por lo que bloquea la llamada en lugar de adivinar el objetivo. El bloqueo se levanta cuando nombra la ruta exacta que se está eliminando, o cuando Claude vuelve a ejecutar la eliminación con la ruta literal resuelta escrita en el comando. Las eliminaciones cuyo objetivo el clasificador puede resolver no se ven afectadas

**Permitido por defecto**:

* Operaciones de archivo local en su directorio de trabajo
* Instalar dependencias declaradas en sus archivos de bloqueo o manifiestos
* Leer `.env` y enviar credenciales a su API coincidente
* Solicitudes HTTP de solo lectura
* Hacer push a la rama en la que comenzó o una que Claude creó
* Pushes rutinarios a la rama predeterminada del repositorio. Antes de v2.1.203, cualquier push directo a la rama predeterminada se bloqueaba

Claude Code v2.1.195 y posterior también permiten estos por defecto:

* Eliminar los trabajos exactos que Claude creó anteriormente en la misma sesión
* Leer, revisar o escribir código, configuraciones y modelos de amenaza relacionados con seguridad como parte de su tarea
* Mensajes entre agentes que trabajan juntos en la misma sesión multiagente
* Enviar datos a los dominios, buckets y servicios de confianza que lista en [`environment`](/docs/es/auto-mode-config#define-trusted-infrastructure). Esto cubre solo el flujo de datos, no operaciones destructivas o de credencial en la misma infraestructura
* Navegación de [Claude en Chrome](/docs/es/chrome) a un dominio interno de confianza, localhost o una URL que nombró

Las solicitudes de acceso a la red de sandbox se enrutan a través del clasificador en lugar de permitirse por defecto. A partir de v2.1.198, el clasificador reutiliza su veredicto para un host y puerto de red en lugar de volver a ejecutarse en cada conexión:

* Un permiso se reutiliza hasta que nuevo contenido entra en la conversación, momento en el cual ese host se verifica nuevamente
* En la CLI interactiva, un rechazo se descarta cuando termina el turno
* En [modo no interactivo](/docs/es/headless) y sesiones del SDK de Agent, no hay límite de turno, por lo que un rechazo se reutiliza para el resto de la ejecución
* Cambiar su modo de permiso o reglas descarta todos los veredictos en caché

Ejecute `claude auto-mode defaults` para ver las listas de reglas completas. Si las acciones rutinarias se bloquean, un administrador puede agregar repositorios, buckets y servicios de confianza a través de la configuración `autoMode.environment`: consulte [Configurar modo automático](/docs/es/auto-mode-config).

Hacer push a su rama de trabajo, hacer un push rutinario a la rama predeterminada del repositorio y crear una solicitud de extracción que coincida con su solicitud se ejecutan sin una solicitud. El clasificador bloquea un push solo cuando lleva riesgo, como un force push o contenido que evita una revisión que configuró. Para requerir un punto de control humano antes de estas acciones mientras permanece en modo automático, agregue reglas `permissions.ask`: consulte [Límites comunes](/docs/es/auto-mode-config#common-boundaries).

<h3 id="boundaries-you-state-in-conversation">
  Límites que establece en la conversación
</h3>

El clasificador trata los límites que establece en la conversación como una señal de bloqueo. Si le dice a Claude "no hagas push" o "espera hasta que revise antes de implementar", el clasificador bloquea acciones coincidentes incluso cuando las reglas predeterminadas las permitirían. Un límite permanece en vigor hasta que lo levante en un mensaje posterior. El propio juicio de Claude de que se cumplió una condición no lo levanta.

Los límites no se almacenan como reglas. El clasificador los relee de la transcripción en cada verificación, por lo que un límite puede perderse si la [compactación de contexto](/docs/es/costs#reduce-token-usage) elimina el mensaje que lo estableció. Para una garantía dura, agregue una [regla de rechazo](/docs/es/permissions#permission-rule-syntax) en su lugar.

<h3 id="when-auto-mode-falls-back">
  Cuándo el modo automático retrocede
</h3>

Cada acción denegada muestra una notificación y aparece en `/permissions` bajo la pestaña Recientemente denegado, donde puede presionar `r` para reintentar con una aprobación manual.

Si el clasificador bloquea una acción 3 veces seguidas o 20 veces en total, el modo automático se pausa y Claude Code reanuda las solicitudes. Aprobar la acción solicitada reanuda el modo automático. Estos umbrales no son configurables. Cualquier acción permitida reinicia el contador consecutivo, mientras que el contador total persiste para la sesión y se reinicia solo cuando su propio límite desencadena un retroceso.

En [modo no interactivo](/docs/es/headless) con la bandera `-p`, los bloqueos repetidos abortan la sesión ya que no hay usuario para solicitar.

Los bloqueos repetidos generalmente significan que el clasificador carece de contexto sobre su infraestructura. Use `/feedback` para reportar falsos positivos, o haga que un administrador [configure infraestructura de confianza](/docs/es/auto-mode-config).

<AccordionGroup>
  <Accordion title="Cómo el clasificador evalúa acciones">
    Cada acción pasa por un orden de decisión fijo. El primer paso coincidente gana:

    1. Las acciones que coinciden con sus [reglas de permiso, solicitud o rechazo](/docs/es/permissions#manage-permissions) se resuelven inmediatamente. Las escrituras en [rutas protegidas](#protected-paths) se enrutan al clasificador incluso cuando una regla de permiso coincide. Las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y las herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) le solicitan directamente incluso cuando una regla de permiso coincide. Las reglas de solicitud con alcance de contenido retroceden a una solicitud de permiso
    2. Las acciones de solo lectura y ediciones de archivo en su directorio de trabajo se aprueban automáticamente, excepto las escrituras en [rutas protegidas](#protected-paths)
    3. Todo lo demás va al clasificador. Una herramienta de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) omite el clasificador y le solicita directamente, por lo que una aprobación requerida por la organización nunca se aprueba automáticamente. A partir de v2.1.199, una herramienta MCP marcada con [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) también omite el clasificador y le solicita directamente, por lo que un paso de consentimiento nunca se aprueba automáticamente en nombre del autor de la herramienta
    4. Si el clasificador bloquea, Claude recibe la razón e intenta una alternativa

    Al entrar en modo automático, se descartan las reglas de permiso amplias que otorgan ejecución de código arbitraria:

    * Blanket `Bash(*)` o `PowerShell(*)`
    * Intérpretes con comodín como `Bash(python*)`
    * Comandos de ejecución del gestor de paquetes
    * Reglas de permiso `Agent`

    Las reglas estrechas como `Bash(npm test)` se mantienen. Las reglas descartadas se restauran cuando sale del modo automático.

    El clasificador ve mensajes de usuario, llamadas de herramienta y contenido de CLAUDE.md. Los resultados de herramientas se eliminan, por lo que el contenido hostil en un archivo o página web no puede manipularlo directamente. Una sonda separada del lado del servidor escanea los resultados de herramientas entrantes y marca contenido sospechoso antes de que Claude lo lea. Para más información sobre cómo funcionan estas capas juntas, consulte el [anuncio de modo automático](https://claude.com/blog/auto-mode) y la [inmersión profunda de ingeniería](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Cómo el modo automático maneja subagentes">
    El clasificador verifica el trabajo de [subagente](/docs/es/sub-agents) en tres puntos:

    1. Antes de que comience un subagente, se evalúa la descripción de tarea delegada, por lo que una tarea que parece peligrosa se bloquea en el momento de su creación.
    2. Mientras se ejecuta el subagente, cada una de sus acciones pasa por el clasificador con las mismas reglas que la sesión principal, y cualquier `permissionMode` en el frontmatter del subagente se ignora.
    3. Cuando el subagente termina, el clasificador revisa su historial de acciones completo; si esa verificación de retorno marca una preocupación, se antepone una advertencia de seguridad a los resultados del subagente.

    El paso 1 requiere Claude Code v2.1.178 o posterior. Las versiones anteriores aplicaban el clasificador en los pasos 2 y 3, pero no evaluaban la descripción de tarea antes de que comenzara el subagente.
  </Accordion>

  <Accordion title="Costo y latencia">
    El clasificador se ejecuta en un modelo configurado por servidor que es independiente de su selección de `/model`, por lo que cambiar modelos no cambia la disponibilidad del clasificador. Las llamadas del clasificador cuentan hacia su uso de tokens. Cada verificación envía una porción de la transcripción más la acción pendiente, agregando un viaje de ida y vuelta antes de la ejecución. Las lecturas y ediciones de directorio de trabajo fuera de rutas protegidas omiten el clasificador, por lo que la sobrecarga proviene principalmente de comandos de shell y operaciones de red. A partir de v2.1.198, un veredicto de red de sandbox para un host y puerto se reutiliza en lugar de reclasificarse en cada conexión, por lo que las conexiones repetidas al mismo host no agregan cada una una verificación. [Qué bloquea el clasificador por defecto](#what-the-classifier-blocks-by-default) describe cuánto tiempo dura un permiso y un rechazo.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Permitir solo herramientas preaprobadas con el modo dontAsk
</h2>

Si establece el modo `dontAsk`, Claude Code deniega automáticamente cada llamada de herramienta que de otro modo le solicitaría confirmación. Claude ejecuta solo acciones que coincidan con sus reglas `permissions.allow`, [comandos Bash de solo lectura](/docs/es/permissions#read-only-commands) y llamadas aprobadas por un [hook PreToolUse](/docs/es/permissions#extend-permissions-with-hooks). Utilice este modo para canalizaciones de CI o entornos restringidos donde predefine exactamente qué puede hacer Claude; la sesión nunca espera entrada. La barra de estado muestra `⏵⏵ don't ask on` mientras este modo está activo.

Claude Code deniega llamadas que coincidan con sus reglas explícitas de [`ask`](/docs/es/permissions#manage-permissions) en lugar de solicitar confirmación. También deniega la herramienta integrada `AskUserQuestion` y las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), incluso si sus reglas de permiso coinciden. Deniega las herramientas MCP marcadas como [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) de la misma manera, porque su tarjeta de aprobación necesita una respuesta que este modo nunca recopila; esto requiere Claude Code v2.1.199 o posterior.

Las sesiones en la nube en [Claude Code en la web](/docs/es/claude-code-on-the-web) ignoran `defaultMode: "dontAsk"`; consulte [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) para obtener más detalles.

Configúrelo al inicio con la bandera:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Omitir todas las comprobaciones con el modo bypassPermissions
</h2>

El modo `bypassPermissions` desactiva los avisos de permisos y las comprobaciones de seguridad para que las llamadas a herramientas se ejecuten inmediatamente, incluidas las escrituras en [rutas protegidas](#protected-paths). Antes de v2.1.126, las escrituras en rutas protegidas aún solicitaban confirmación en este modo.

Las [reglas ask](/docs/es/permissions#manage-permissions) explícitas y las herramientas de conector [que su organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) aún fuerzan un aviso en este modo. Las herramientas MCP marcadas con [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) también aún solicitan confirmación; esto requiere Claude Code v2.1.199 o posterior.

Las eliminaciones dirigidas al directorio raíz del sistema de archivos o al directorio de inicio, como `rm -rf /` y `rm -rf ~`, aún solicitan confirmación como un cortacircuitos contra errores del modelo. El cortacircuitos también se activa cuando el comando contiene sustitución de comandos con `$(...)` o comillas invertidas, o sustitución de procesos con `<(...)`, ya sea que la eliminación esté dentro de la sustitución, como en `echo "$(rm -rf ~)"`, o en otro lugar del mismo comando. La forma simple, escrita como su propio comando, ha solicitado confirmación en este modo desde que se introdujo el cortacircuitos; antes de v2.1.208, los comandos que contenían esas formas no solicitaban confirmación.

<Warning>
  Utilice este modo solo en entornos aislados como contenedores, máquinas virtuales o dev containers sin acceso a Internet, donde Claude Code no pueda dañar su sistema anfitrión.
</Warning>

No puede entrar en `bypassPermissions` desde una sesión que se inició sin una de las banderas de habilitación; reinicie con una para habilitarlo:

```bash theme={null}
claude --permission-mode bypassPermissions
```

La bandera `--dangerously-skip-permissions` es equivalente.

En Linux y macOS, Claude Code se niega a iniciarse en este modo cuando se ejecuta como root o bajo `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

La comprobación se omite automáticamente dentro de un sandbox reconocido. Para ejecutarse de forma autónoma en un contenedor, utilice la configuración de [dev container](/docs/es/devcontainer), que ejecuta Claude Code como un usuario no root.

[Claude Code en la web](/docs/es/claude-code-on-the-web) no respeta `defaultMode: "bypassPermissions"` o `"dontAsk"` de sus archivos de configuración, por lo que la configuración registrada en un repositorio no puede iniciar una sesión en la nube en modo bypass-permissions. La configuración se ignora silenciosamente y la sesión se inicia en el modo que se muestra en el menú desplegable de modo. Consulte [Cambiar modos de permisos](#switch-permission-modes) para ver qué modos ofrecen las sesiones en la nube.

<Warning>
  `bypassPermissions` no ofrece protección contra inyección de prompts o acciones no intencionadas. Para comprobaciones de seguridad de fondo con muchos menos avisos de permisos, utilice el [modo automático](#eliminate-prompts-with-auto-mode) en su lugar. Los administradores pueden bloquear este modo estableciendo `permissions.disableBypassPermissionsMode` en `"disable"` en [configuración administrada](/docs/es/permissions#managed-settings).
</Warning>

<h2 id="protected-paths">
  Rutas protegidas
</h2>

Las escrituras en un pequeño conjunto de rutas nunca se aprueban automáticamente, en todos los modos excepto `bypassPermissions`. Esto previene la corrupción accidental del estado del repositorio y la configuración propia de Claude.

| Modo                             | Escrituras en rutas protegidas |
| :------------------------------- | :----------------------------- |
| `default`, `acceptEdits`, `plan` | Solicitadas                    |
| `auto`                           | Enrutadas al clasificador      |
| `dontAsk`                        | Denegadas                      |
| `bypassPermissions`              | Permitidas                     |

Las reglas [`permissions.allow`](/docs/es/permissions#manage-permissions) en archivos de configuración no pre-aprueban escrituras en rutas protegidas. La verificación de seguridad se ejecuta antes de que Claude Code evalúe las reglas de permitir desde la configuración, por lo que una entrada como `Edit(.claude/**)` en `~/.claude/settings.json` o `.claude/settings.json` no cambia el resultado por modo en la tabla anterior. En modos que solicitan, la solicitud para una escritura en `.claude/` ofrece **Sí, y permitir que Claude edite su propia configuración para esta sesión**, lo que aprueba escrituras posteriores en `.claude/` en esa sesión sin solicitar de nuevo.

Directorios protegidos:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, excepto por `.claude/worktrees` donde Claude almacena sus propios git worktrees

Archivos protegidos:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="see-also">
  Véase también
</h2>

* [Permisos](/docs/es/permissions): reglas de permitir, preguntar y denegar; políticas gestionadas
* [Configurar modo automático](/docs/es/auto-mode-config): indique al clasificador qué infraestructura confía su organización
* [Hooks](/docs/es/hooks): lógica de permisos personalizada mediante hooks `PreToolUse` y `PermissionRequest`
* [Ultraplan](/docs/es/ultraplan): ejecutar modo plan en una sesión de Claude Code en la web con revisión basada en navegador
* [Seguridad](/docs/es/security): salvaguardas y mejores prácticas
* [Sandboxing](/docs/es/sandboxing): aislamiento del sistema de archivos y red para comandos Bash
* [Modo no interactivo](/docs/es/headless): ejecutar Claude Code con la bandera `-p`
