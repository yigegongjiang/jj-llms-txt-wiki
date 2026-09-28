> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Detectar problemas de seguridad mientras Claude escribe código

> Instale el plugin security-guidance para que Claude revise sus propios cambios de código en busca de vulnerabilidades y las corrija en la misma sesión.

El plugin security guidance hace que Claude revise sus propios cambios de código en busca de vulnerabilidades comunes mientras trabaja y corrija lo que encuentra en la misma sesión. El plugin detecta problemas como inyección, deserialización insegura y APIs DOM inseguras antes de que el código llegue a una solicitud de extracción, reduciendo cuánta revisión de seguridad recae en los revisores humanos posteriores.

Una vez instalado, el plugin se ejecuta automáticamente. No hay nada que invocar y ningún comando separado que recordar.

El plugin es el compañero en sesión de [Code Review](/docs/es/code-review), que se ejecuta en solicitudes de extracción. Este plugin reduce lo que llega al PR. Code Review detecta lo que sí llega al PR. Para ver cómo el plugin se superpone con revisión bajo demanda y escaneo de CI, o para escanear código que ya tiene en lugar de cambios que Claude está escribiendo, consulte [Cómo se ajusta esto con otras herramientas de seguridad](#how-this-fits-with-other-security-tools).

<h2 id="prerequisites">
  Requisitos previos
</h2>

* Python 3.7 o posterior en su `PATH`. La revisión de confirmación agéntica requiere Python 3.10 o posterior, al igual que todas las revisiones respaldadas por modelos cuando Claude Code utiliza un proveedor de terceros como Amazon Bedrock o la Plataforma de Agentes de Google Cloud. El plugin prefiere los intérpretes versionados `python3.13` a través de `python3.10`, luego recurre a `python3`, `python` y `py -3`
* Un repositorio git para el directorio en el que trabaja. Las revisiones de fin de turno y confirmación se comparan con el estado de git y se omiten silenciosamente fuera de un repositorio. La verificación de patrón por edición funciona en cualquier lugar

En la primera ejecución, el plugin crea un entorno virtual bajo `~/.claude/security/` e instala el Claude Agent SDK en él, lo que requiere `pip` y acceso a la red. Si esa instalación falla, o el Python disponible es más antiguo que 3.10, la revisión de confirmación en autenticación de primera parte recurre a una revisión de un solo disparo en lugar de la agéntica; en un proveedor de terceros como Amazon Bedrock o la Plataforma de Agentes de Google Cloud, las revisiones respaldadas por modelos necesitan el SDK en sí, por lo que se omiten. El plugin muestra un aviso único cuando una versión anterior de Python es la causa.

<h2 id="install-the-plugin">
  Instalar el plugin
</h2>

En una sesión de Claude Code en terminal, instale desde el [marketplace oficial de Anthropic](/docs/es/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

`/plugin` abre un panel interactivo en la CLI de terminal. Si Claude responde que `/plugin` no está disponible en este entorno, instale de otra manera:

* **Aplicación de escritorio Claude, sesión local o SSH**: abra el [navegador de plugins](/docs/es/desktop#install-plugins) haciendo clic en el botón **+** junto al prompt, luego **Plugins**, luego **Agregar plugin**
* **Extensión de VS Code**: instale desde el [diálogo **Administrar plugins**](/docs/es/vs-code#manage-plugins)
* **Sesiones en la nube**: una sesión en la nube no carga plugins de su configuración de usuario o del archivo `.claude/settings.json` del repositorio, como explica [Lo que se transfiere de su configuración](/docs/es/cloud-environments#what-carries-over-from-your-setup). Para plugins que su organización distribuye a través de configuración administrada, consulte [Administrar plugins para su organización](/docs/es/plugins/org)

La instalación en terminal solicita un alcance. Elija alcance de usuario para escribir el plugin en su configuración de usuario, de modo que se cargue en cada nueva sesión local que inicie en esta máquina.

Si la instalación falla, haga coincidir el mensaje que Claude Code reporta:

* `Marketplace "claude-plugins-official" not found`: agregue el marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, luego reintente la instalación.
* El plugin [no se encuentra en el marketplace](/docs/es/plugins/install#install-a-plugin): verifique el nombre del plugin.

Verifique el resumen de instalación. Si reporta `Run /reload-plugins to activate.`, consulte [Aplicar cambios de plugin sin reiniciar](/docs/es/plugins/cli-reference#reload-plugins) para activar el plugin en su sesión actual.

<h3 id="enable-for-your-team-in-local-sessions">
  Habilitar en sesiones locales para su equipo
</h3>

Para activar el plugin en las sesiones locales que sus compañeros de equipo inician en el repositorio, declárelo en la configuración registrada del proyecto:

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

Los administradores pueden habilitar el plugin en toda la organización configurando [`enabledPlugins`](/docs/es/settings-reference#enabledplugins) en [configuración administrada](/docs/es/admin-setup).

<h2 id="what-the-plugin-checks">
  Lo que el plugin verifica
</h2>

El plugin revisa el trabajo de Claude en tres puntos, cada uno con una profundidad diferente:

* [En cada edición de archivo](#on-each-file-edit): una coincidencia de patrón rápida para llamadas riesgosas, sin llamada de modelo
* [Al final de cada turno](#at-the-end-of-each-turn): una revisión de modelo en segundo plano de todo lo que ese turno cambió
* [En cada commit o push que Claude realiza](#on-each-commit-or-push-claude-makes): una revisión agéntica más profunda que lee el código circundante

Puede extender cada capa [agregando sus propias reglas](#add-your-own-rules). Las verificaciones integradas no se pueden eliminar individualmente, pero puede [desactivar cada capa](#disable-or-uninstall) de forma independiente.

<h3 id="on-each-file-edit">
  En cada edición de archivo
</h3>

Cuando Claude escribe en un archivo, el plugin escanea el nuevo contenido en busca de patrones conocidos riesgosos. Esta es una coincidencia de patrón sin llamada de modelo, por lo que no agrega ningún costo de uso.

Categorías de patrón de ejemplo:

* Ejecución de código dinámico: `eval(`, `new Function`, `os.system`, `child_process.exec`
* Deserialización insegura: `pickle`
* Inyección DOM: `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* Archivos de flujo de trabajo: ediciones bajo `.github/workflows/`, que pueden otorgar permisos a nivel de repositorio

La verificación se ejecuta después de que la edición se realiza y agrega la advertencia al contexto de Claude para el siguiente paso. Cada advertencia se activa una vez por patrón por archivo por sesión, por lo que las coincidencias repetidas en el mismo archivo no inundan la conversación.

Puede [agregar sus propios patrones](#add-custom-per-edit-patterns) a esta capa con un archivo `security-patterns.yaml`.

<h3 id="at-the-end-of-each-turn">
  Al final de cada turno
</h3>

Un turno es una ronda de Claude respondiendo: usted envía un mensaje, Claude trabaja y responde, y el turno termina. Después de cada turno, el plugin calcula un git diff de todo lo que cambió en el árbol de trabajo durante el turno, incluidos los cambios de las herramientas de edición de Claude, comandos Bash y subagentes, y lo envía a una revisión de Claude separada enfocada en seguridad. La revisión se ejecuta en segundo plano, por lo que la respuesta de Claude no se retrasa. Si la revisión encuentra problemas, Claude se vuelve a solicitar con los hallazgos y los aborda como seguimiento.

Esto detecta lo que sí llega a problemas que una coincidencia de cadena no puede, tales como:

* Omisión de autorización
* Referencias directas a objetos inseguras
* Inyección
* Falsificación de solicitud del lado del servidor
* Criptografía débil

Usted ve tanto el hallazgo como la resolución de Claude directamente en su sesión. La revisión cubre hasta 30 archivos cambiados por turno y se activa como máximo tres veces seguidas antes de ceder el control a usted.

<h3 id="on-each-commit-or-push-claude-makes">
  En cada commit o push que Claude realiza
</h3>

Cuando Claude ejecuta `git commit` o `git push` a través de su herramienta Bash, el plugin ejecuta una revisión agéntica más profunda del cambio en segundo plano. Esta revisión lee el código circundante, incluidos los llamadores, sanitizadores y archivos relacionados, para decidir si un hallazgo es real antes de reportarlo. El contexto adicional mantiene los falsos positivos bajos en patrones que se ven peligrosos en aislamiento pero son seguros en su base de código.

Esta capa se activa solo en commits y pushes que Claude realiza a través de su herramienta Bash. Los commits que ejecuta desde su propio shell, incluido el escape de shell `!` dentro de una sesión, no se revisan. Las revisiones de commit y push se limitan a 20 por hora móvil. Si los hallazgos de la revisión de commit duplican lo que la revisión de fin de turno ya reportó, Claude no se vuelve a solicitar, por lo que un commit limpio no produce salida visible de esta capa.

<h3 id="review-independence-and-limits">
  Independencia de revisión y límites
</h3>

El plugin no le pide a la misma instancia de Claude que escribió el código que se califique a sí misma. La verificación por edición es una coincidencia de cadena determinista sin modelo involucrado. Las revisiones de fin de turno y commit se ejecutan como una llamada de Claude separada con un contexto fresco y un prompt enfocado en seguridad: el revisor comienza desde el diff, no tiene inversión en el enfoque original, e instrucciones solo para encontrar problemas.

Ninguna de las capas bloquea escrituras o commits. Los hallazgos llegan a Claude que escribe como instrucciones, Claude los aborda en la conversación, y el modelo de revisión puede perder problemas. Trate el plugin como una capa de defensa en profundidad, no como una solución de seguridad completa. Vea [Cómo esto se ajusta con otras herramientas de seguridad](#how-this-fits-with-other-security-tools).

<h2 id="add-your-own-rules">
  Agregue sus propias reglas
</h2>

El plugin tiene dos puntos de extensión: un archivo de orientación Markdown para las revisiones respaldadas por modelo, y un archivo de patrones YAML o JSON para la coincidencia de cadena por edición. Ambos son aditivos. Puede agregar verificaciones pero no puede deshabilitar las integradas desde estos archivos.

<h3 id="add-guidance-for-the-model-backed-reviews">
  Agregue orientación para las revisiones respaldadas por modelo
</h3>

Cree `.claude/claude-security-guidance.md` en su proyecto y describa su modelo de amenaza y lista de verificación de revisión en lenguaje simple. Las revisiones respaldadas por modelo lo cargan como contexto adicional junto con la lista de verificación de vulnerabilidades integrada.

El siguiente ejemplo es para un servicio web con rutas de administrador con puerta de rol y una política de registro de datos de cliente:

```markdown .claude/claude-security-guidance.md theme={null}
# Security guidance for this repo

- Do not log `customer_id` or `account_number` at INFO level or above.
- All routes under `/admin` must call `require_role("admin")` before any database read.
- Use `crypto.timingSafeEqual` for token comparison instead of `===`.
```

Estas reglas son orientación para el revisor, no guardrails deterministas. El plugin expone las violaciones como hallazgos para que Claude las corrija, pero no bloquea escrituras ni garantiza que se detecte cada violación. La orientación es solo aditiva: una regla que dice ignorar una clase de vulnerabilidad no suprime esos hallazgos. Para aplicación estricta, empareje el plugin con un [hook que bloquee la edición](/docs/es/hooks-guide#block-edits-to-protected-files) o una verificación de CI.

<h3 id="add-custom-per-edit-patterns">
  Agregue patrones personalizados por edición
</h3>

Cree `.claude/security-patterns.yaml` para agregar reglas de regex o subcadena a la [verificación de patrón por edición](#on-each-file-edit). Estos se ejecutan como coincidencias de cadena deterministas junto con los patrones integrados:

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "Hardcoded API key prefix. Load credentials from the secret manager."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "Multi-tenant code must filter by org_id."
```

| Campo           | Tipo   | Descripción                                                                                                                                                                                     |
| :-------------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `rule_name`     | string | Identificador mostrado en la advertencia                                                                                                                                                        |
| `reminder`      | string | Texto de advertencia agregado al contexto de Claude, limitado a 1 KB                                                                                                                            |
| `regex`         | string | Regex de Python coincidido contra el contenido editado                                                                                                                                          |
| `substrings`    | list   | Subcadenas literales; proporcione esto o `regex`                                                                                                                                                |
| `paths`         | list   | Patrones glob opcionales; la regla se aplica solo a archivos coincidentes. Los globs coinciden contra la ruta completa del archivo, por lo que prefije patrones relativos al proyecto con `**/` |
| `exclude_paths` | list   | Patrones glob opcionales para omitir; la coincidencia es la misma que `paths`                                                                                                                   |

El plugin también lee `.claude/security-patterns.yml` y `.claude/security-patterns.json` con el mismo esquema. JSON funciona en cualquier instalación de Python. Los formularios YAML requieren que PyYAML sea importable, que el plugin no instala para usted. El plugin carga hasta 50 reglas personalizadas y omite regexes que parecen propensas a retroceso catastrófico.

<h3 id="rule-file-lookup-locations">
  Ubicaciones de búsqueda de archivos de reglas
</h3>

El plugin busca `claude-security-guidance.md` y `security-patterns.yaml` en las mismas ubicaciones, independientemente de cómo se habilitó el plugin:

| Alcance        | Ruta                                        | Notas                                                    |
| :------------- | :------------------------------------------ | :------------------------------------------------------- |
| Usuario        | `~/.claude/claude-security-guidance.md`     | Se aplica a cada proyecto en su máquina                  |
| Proyecto       | `.claude/claude-security-guidance.md`       | Registrado con el repositorio                            |
| Proyecto local | `.claude/claude-security-guidance.local.md` | Para anulaciones personales; agréguelo a su `.gitignore` |

El plugin carga todas las ubicaciones que existen y las concatena, con un límite combinado de 8 KB para el archivo de orientación. Los administradores pueden distribuir reglas en toda la organización empujando el archivo de alcance de usuario a `~/.claude/` a través de la administración de dispositivos. Las mismas rutas se aplican a `security-patterns.yaml`.

<h2 id="usage-cost">
  Costo de uso
</h2>

La [verificación de patrón por edición](#on-each-file-edit) no realiza llamada de modelo y no agrega costo. Las revisiones de [fin de turno](#at-the-end-of-each-turn) y [confirmación](#on-each-commit-or-push-claude-makes) cada una gasta uso de modelo adicional que cuenta hacia su [uso](/docs/es/costs) como cualquier otra solicitud de Claude. La revisión de confirmación es agéntica y puede tomar varios turnos de modelo por confirmación. Espere aproximadamente una llamada de revisión por turno que cambie archivos y una revisión más profunda por confirmación, ambas sujetas a los límites anteriores.

Ambas revisiones respaldadas por modelo usan Claude Opus 4.7 de forma predeterminada. Configure `SECURITY_REVIEW_MODEL` para elegir un modelo diferente para la revisión de fin de turno y `SG_AGENTIC_MODEL` para la revisión de confirmación.

El plugin está disponible en todos los planes.

<h2 id="disable-or-uninstall">
  Deshabilitar o desinstalar
</h2>

Para desactivar capas individuales mientras mantiene el resto, configure la variable de entorno coincidente:

| Variable                        | Efecto                                                                                       |
| :------------------------------ | :------------------------------------------------------------------------------------------- |
| `ENABLE_PATTERN_RULES=0`        | Deshabilitar la [verificación de patrón por edición](#on-each-file-edit)                     |
| `ENABLE_STOP_REVIEW=0`          | Deshabilitar la [revisión de diff de fin de turno](#at-the-end-of-each-turn)                 |
| `ENABLE_COMMIT_REVIEW=0`        | Deshabilitar la [revisión de confirmación e inserción](#on-each-commit-or-push-claude-makes) |
| `ENABLE_CODE_SECURITY_REVIEW=0` | Deshabilitar todas las revisiones respaldadas por modelo a la vez                            |
| `SECURITY_GUIDANCE_DISABLE=1`   | Deshabilitar el plugin completamente sin desinstalar                                         |

Para pausar el plugin en su alcance de usuario:

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

Para eliminarlo de su alcance de usuario:

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

Si el plugin se habilitó a través del `.claude/settings.json` de un proyecto, desinstalar desde `/plugin` escribe una anulación a su `.claude/settings.local.json` en lugar de editar el archivo registrado, por lo que el plugin permanece desactivado para usted mientras que los compañeros de equipo no se ven afectados. El mismo diálogo también ofrece desinstalar el plugin para todos eliminándolo del `.claude/settings.json` compartido. Si se habilitó a través de [configuración administrada](/docs/es/admin-setup), solo un administrador puede deshabilitarlo.

<h2 id="how-the-plugin-integrates-with-claude-code">
  Cómo se integra el plugin con Claude Code
</h2>

El plugin se construye completamente en [hooks](/docs/es/hooks), el mecanismo para ejecutar su propio código en puntos específicos del bucle de Claude. Se registra:

| Evento de hook                                                | Propósito                                                                                                   |
| :------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------- |
| `SessionStart`                                                | Inicializar el entorno de Python del plugin                                                                 |
| `UserPromptSubmit`                                            | Capturar la línea de base del árbol de trabajo contra la que el diff de revisión de fin de turno se compara |
| `PostToolUse` en `Edit`, `Write` y `NotebookEdit`             | Coincidencia de patrón por edición                                                                          |
| `Stop`                                                        | Revisión de diff de fin de turno, ejecutada en segundo plano                                                |
| `PostToolUse` en `Bash`, filtrado a `git commit` y `git push` | Revisión de confirmación e inserción, ejecutada en segundo plano                                            |

Si construye sus propios hooks, el [código fuente del plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance) es un ejemplo funcional de ejecutar una llamada de modelo separada desde un hook y alimentar el resultado de vuelta a la sesión.

<h2 id="how-this-fits-with-other-security-tools">
  Cómo se ajusta esto con otras herramientas de seguridad
</h2>

El plugin es una capa en un enfoque de defensa en profundidad. Detecta problemas lo antes posible, mientras el código aún está en el editor, pero no es una garantía y no reemplaza verificaciones posteriores. Una pila típica:

| Etapa                          | Herramienta                                                       | Qué cubre                                                                                                                  |
| :----------------------------- | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| En sesión                      | Plugin de orientación de seguridad                                | Vulnerabilidades comunes en código que Claude escribe, corregidas en la misma sesión                                       |
| Bajo demanda, paso único       | [`/security-review`](/docs/es/commands#all-commands)                   | Paso de seguridad único en la rama actual, ejecutado cuando lo solicita                                                    |
| Bajo demanda, escaneo profundo | [Plugin de seguridad de Claude](/docs/es/claude-security)              | Escaneo de vulnerabilidades multiagente de un repositorio o diff, con hallazgos y parches revisados de forma independiente |
| En solicitud de extracción     | [Code Review](/docs/es/code-review), planes de equipo y empresa        | Revisión de corrección y seguridad multiagente con contexto completo de base de código                                     |
| En CI                          | Sus analizadores estáticos existentes y escáneres de dependencias | Reglas específicas del idioma, verificaciones de cadena de suministro y aplicación de políticas que el plugin no intenta   |

Para encontrar problemas de seguridad en código que ya tiene, en lugar de en cambios que Claude está escribiendo, pida a Claude en una sesión que revise un archivo o directorio específico en busca de vulnerabilidades, o use el [Plugin de seguridad de Claude](/docs/es/claude-security) para un escaneo multiagente más profundo de todo el repositorio; [`/security-review`](/docs/es/commands#all-commands) cubre solo los cambios en su rama actual. De cualquier forma, la revisión lee el código fuente en su checkout, no un sitio en ejecución o un servicio implementado.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

El plugin escribe diagnósticos de tiempo de ejecución en `~/.claude/security/log.txt`. Verifique allí primero si las revisiones no aparecen.

Razones comunes por las que una capa de revisión se omite sin un mensaje en la conversación:

* El directorio no es un repositorio git: las revisiones de fin de turno y confirmación requieren estado de git y se omiten fuera de un repositorio
* La sesión no tiene autenticación de Anthropic y ningún proveedor de terceros configurado: las revisiones respaldadas por modelo se omiten y solo se ejecuta la verificación de patrón por edición
* Un archivo `security-patterns.yaml` está presente pero PyYAML no es importable: el archivo se ignora. Use `security-patterns.json` en su lugar

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para profundizar en los temas que toca esta página:

* [Code Review](/docs/es/code-review): configurar la revisión multiagente en tiempo de PR
* [Automatizar flujos de trabajo con hooks](/docs/es/hooks-guide): construir sus propias verificaciones en los mismos puntos del ciclo de vida
* [Buscar plugins en el marketplace oficial](/docs/es/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): dónde explorar los otros plugins oficiales
