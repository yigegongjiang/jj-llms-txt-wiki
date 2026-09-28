> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar el modo automático

> Indique al clasificador del modo automático qué repositorios, buckets y dominios confía su organización. Establezca el contexto del entorno, anule las reglas de bloqueo y permiso predeterminadas e inspeccione su configuración efectiva con los subcomandos de la CLI del modo automático.

[El modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) permite que Claude Code se ejecute sin solicitudes de permiso rutinarias al enrutar las llamadas de herramientas a través de un clasificador que bloquea cualquier cosa irreversible, destructiva o dirigida fuera de su entorno. Las reglas de denegación y solicitud explícita se evalúan antes del clasificador y aún bloquean o solicitan. Utilice el bloque de configuración `autoMode` para indicar a ese clasificador qué repositorios, buckets y dominios confía su organización, de modo que deje de bloquear operaciones internas rutinarias.

<Note>
  El modo automático está disponible para todos los usuarios en cada proveedor, incluida la API de Anthropic, [Claude Platform en AWS](/docs/es/claude-platform-on-aws), Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry y sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada. Si Claude Code informa que el modo automático no está disponible para su cuenta, consulte los [requisitos completos](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), que también cubren los modelos admitidos y el control a nivel de organización en planes de equipo y empresa. En v2.1.158 a v2.1.206, el modo automático en Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry y sesiones de puerta de enlace de aplicaciones Claude requería establecer `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 eliminó el requisito.
</Note>

De forma predeterminada, el clasificador confía solo en el directorio de trabajo y en los remotos configurados del repositorio actual. Las acciones como insertar en la organización de control de código fuente de su empresa o escribir en un bucket de nube de equipo se bloquean hasta que las agregue a `autoMode.environment`.

Para saber cómo habilitar el modo automático y qué bloquea de forma predeterminada, consulte [modo automático en la página Modos de permiso](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). Esta página es la referencia de configuración.

Esta página cubre cómo:

* [Agregar un checkpoint manual](#add-a-human-checkpoint) para inserciones y solicitudes de extracción con `permissions.ask`
* [Elegir dónde establecer reglas](#where-the-classifier-reads-configuration) en CLAUDE.md, configuración de usuario y configuración administrada
* [Definir infraestructura de confianza](#define-trusted-infrastructure) con `autoMode.environment`
* [Generar entradas de entorno](#generate-environment-entries) con `/auto-mode-setup`
* [Anular las reglas de bloqueo y permiso](#override-the-block-and-allow-rules) cuando los valores predeterminados no se ajustan a su canalización
* [Editar reglas desde `/permissions`](#edit-rules-from-permissions) sin abrir un archivo de configuración
* [Enrutar todos los comandos de shell a través del clasificador](#route-all-shell-commands-through-the-classifier) con `autoMode.classifyAllShell`
* [Inspeccionar su configuración efectiva](#inspect-the-defaults-and-your-effective-config) con los subcomandos `claude auto-mode`
* [Revisar denegaciones](#review-denials) para saber qué agregar a continuación

<h2 id="common-boundaries">
  Límites comunes
</h2>

El modo automático permite inserciones en cualquier rama del repositorio en el que está trabajando, incluida la rama predeterminada, y creación de solicitudes de extracción de forma predeterminada. Una rama no predeterminada cuyo nombre la marca como destino de implementación o publicación, como `production`, `release` o `gh-pages`, no está cubierta por ese valor predeterminado: el clasificador juzga una inserción allí en sus propios términos, incluso como una implementación de producción. El contenido de la inserción también se sigue verificando, por lo que una inserción forzada, un secreto que entra en la confirmación o un cambio que enviaría secretos fuera del repositorio cuando CI o una canalización de implementación lo ejecuta permanece bloqueado.

<Info>Antes de v2.1.211, el clasificador permitía inserciones solo en su rama de trabajo, ramas que Claude creó e inserciones rutinarias en la rama predeterminada.</Info>

Si desea un punto de control humano antes de cada inserción o solicitud de extracción de Claude, agregue reglas de permisos: las [recetas a continuación](#add-a-human-checkpoint) mantienen el modo automático activado para todo lo demás.

<h3 id="add-a-human-checkpoint">
  Agregar un punto de control humano
</h3>

El mecanismo más directo es [`permissions.ask`](/docs/es/permissions#permission-rule-syntax). Las reglas de solicitud con alcance de contenido como las que se muestran a continuación se evalúan antes del clasificador y siempre fuerzan un aviso de permiso, incluso en modo automático, porque una regla de solicitud explícita es su intención declarada de ser solicitado para esa acción. Agregue las reglas en su [configuración](/docs/es/settings#where-settings-live):

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Estas reglas coinciden con comandos que comienzan con `git push` o `gh pr create`. Una inserción que Claude escribe de otra manera, como `git -C <dir> push` o `git -c <key>=<value> push`, [no coincide con la regla](/docs/es/permissions#bash-rule-limits), por lo que no se somete a punto de control. Para un punto de control que inspeccione el texto completo del comando, agregue un [hook PreToolUse](/docs/es/hooks#pretooluse).

Elija el mecanismo que se ajuste a lo firme que deba ser el límite:

| Límite                        | Mecanismo                                                        | Comportamiento en modo automático                                                                                                                                                                                                                            |
| :---------------------------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Solicitar antes de la acción  | `permissions.ask`                                                | Siempre solicita para un comando que coincida con una regla con alcance de contenido como la receta anterior. El clasificador no puede aprobar automáticamente una acción coincidente.                                                                       |
| Nunca ejecutar la acción      | `permissions.deny`                                               | Bloquea antes de que se consulte el clasificador. Ni el clasificador ni la intención del usuario pueden anularlo.                                                                                                                                            |
| Límite único para esta sesión | Indíquelo en la conversación, como "no inserte hasta que revise" | El clasificador bloquea las acciones coincidentes, pero el límite se puede perder si la [compactación de contexto](/docs/es/costs#reduce-token-usage) elimina el mensaje que lo indicaba. Utilice una regla de solicitud o denegación para una garantía duradera. |

<h2 id="where-the-classifier-reads-configuration">
  Dónde el clasificador lee la configuración
</h2>

El clasificador lee el mismo contenido de [CLAUDE.md](/docs/es/memory) que Claude carga, por lo que una instrucción como "nunca hagas push forzado" en el CLAUDE.md de su proyecto dirige tanto a Claude como al clasificador al mismo tiempo. Comience allí para las convenciones del proyecto y las reglas de comportamiento.

Para las reglas que se aplican en todos los proyectos, como la infraestructura de confianza o las reglas de denegación en toda la organización, utilice el bloque de configuración `autoMode`. El clasificador lee `autoMode` de los siguientes ámbitos:

| Ámbito                           | Archivo                                                   | Usar para                                                            |
| :------------------------------- | :-------------------------------------------------------- | :------------------------------------------------------------------- |
| Un desarrollador                 | `~/.claude/settings.json`                                 | Infraestructura de confianza personal                                |
| En toda la organización          | [Configuración administrada](/docs/es/server-managed-settings) | Infraestructura de confianza distribuida a todos los desarrolladores |
| Bandera `--settings` o Agent SDK | JSON en línea                                             | Anulaciones por invocación para automatización                       |

El clasificador no lee `autoMode` de la configuración del proyecto en `.claude/settings.json` o `.claude/settings.local.json`. Ambos archivos residen en el directorio del repositorio, por lo que un repositorio registrado o un paso de compilación podría inyectar sus propias reglas de permiso. Antes de v2.1.207, el clasificador también leía `.claude/settings.local.json`; mueva cualquier bloque `autoMode` en ese archivo a `~/.claude/settings.json`. Excluir `.claude/settings.local.json` también cierra el caso en el que un repositorio confirma el archivo o una herramienta local o paso de compilación lo escribe.

Las entradas de cada ámbito se combinan. Un desarrollador puede extender `environment`, `allow`, `soft_deny` y `hard_deny` con entradas personales pero no puede eliminar las entradas que proporciona la configuración administrada. Debido a que las reglas de permiso actúan como excepciones a las reglas de bloqueo suave dentro del clasificador, una entrada `allow` agregada por un desarrollador puede anular una entrada `soft_deny` de la organización: la combinación es aditiva, no un límite de política dura.

<Note>
  El clasificador es una segunda puerta que se ejecuta después del [sistema de permisos](/docs/es/permissions). Para acciones que nunca deben ejecutarse independientemente de la intención del usuario o la configuración del clasificador, utilice `permissions.deny` en la configuración administrada, que bloquea la acción antes de que se consulte el clasificador y no puede ser anulada.
</Note>

<h2 id="define-trusted-infrastructure">
  Definir infraestructura de confianza
</h2>

Para la mayoría de las organizaciones, `autoMode.environment` es el único campo que necesita configurar. Le indica al clasificador qué repositorios, buckets y dominios son de confianza: el clasificador lo utiliza para decidir qué significa "externo", por lo que cualquier destino no listado es un objetivo potencial de exfiltración.

A partir de Claude Code v2.1.198, `claude auto-mode defaults` imprime tres tipos de entrada de entorno. Las versiones anteriores a v2.1.195 imprimen solo los primeros cinco espacios de confianza.

* **Espacios de contexto**: describen su organización, stack y postura de seguridad para que el clasificador lea las otras reglas en su contexto. Cada uno tiene como valor predeterminado `None configured` o la suposición conservadora nombrada junto a él:
  * **Organización**
  * **Uso principal de Claude Code**: tiene como valor predeterminado desarrollo de software
  * **Proveedor(es) de nube**
  * **Visibilidad del repositorio**: se asume que un repositorio es privado a menos que su host remoto y nombre indiquen lo contrario, o el clasificador lea una verificación de visibilidad anterior en la conversación que muestre que es público.

    En las solicitudes del clasificador enviadas por Claude Code mismo, el clasificador lee sus mensajes y los comandos que ejecuta Claude, no su salida. La evidencia tiene que ser algo que el clasificador pueda leer, como su propio mensaje nombrando el repositorio como público; la salida de un `gh repo view` por sí sola no llega a él. La verificación de evidencia de transcripción requiere Claude Code v2.1.200 o posterior
  * **Intercambio interno / alojamiento de fragmentos**: los servicios públicos de paste y gist se tratan como fuera del límite de confianza hasta que nombre uno
  * **CLI específicas de la organización**
  * **Gestión de secretos**
  * **Objetivos de implementación de CI/CD**
  * **Postura de red**
  * **Contención de host**: tiene como valor predeterminado una máquina de desarrollador ordinaria o ejecutor de CI con internet abierto. Si Claude Code se ejecuta en un contenedor, VM o pod con una lista de permitidos de salida o vecinos que no debe tocar, nombre los hosts permitidos, si el punto final de metadatos de nube debe ser accesible, y qué proyecto de nube, clúster o registro utiliza la tarea y bajo qué identidad. Hasta que esta entrada nombre esa identidad, el clasificador [bloquea](/docs/es/permission-modes#what-the-classifier-blocks-by-default) solicitudes de las propias credenciales del host. Requiere Claude Code v2.1.257 o posterior
  * **Espacios de nombres / entornos de implementación protegidos**: recurre a la heurística de objetivos remotos sensibles hasta que los nombre
  * **Retención de datos / desclasificación**
* **Espacios de confianza**: nombre lo que el clasificador trata como dentro de su límite. Los espacios son Repositorio de confianza, Control de fuente, Dominios internos de confianza, Buckets de nube de confianza, Servicios internos clave y Registro de paquetes interno. Las entradas de repositorio y control de fuente tienen como valor predeterminado el repositorio de trabajo y sus remotos configurados. Todos los demás espacios de confianza tienen como valor predeterminado `None configured`, por lo que nada más es de confianza hasta que lo agregue. La visibilidad de un repositorio solo abarca material confidencial: un repositorio privado es un destino aceptable para material confidencial, pero hacer un repositorio privado nunca borra secretos o datos personales o confiados en él, y el clasificador trata el contenido portado, reapuntado o leído por primera vez desde fuera del repositorio de trabajo como no siendo trabajo propio de ese repositorio. Este alcance requiere Claude Code v2.1.203 o posterior.
* **Espacios de sensibilidad**: nombre lo que las reglas de protección tratan como de alto riesgo. Los espacios son Ubicaciones de datos sensibles y audiencias, Objetivos remotos sensibles y Alcances de IaC protegidos. Cada uno tiene como valor predeterminado una heurística amplia, como tratar cualquier host o espacio de nombres cuyo nombre lleve `prod` o `production` como un objetivo remoto sensible, por lo que las reglas de protección están activas antes de que configure nada. Nombrar objetivos concretos en un espacio de sensibilidad hace que esas reglas se apliquen a los objetivos nombrados en lugar de la heurística.

<Info>Antes de v2.1.211, los espacios de contexto también incluían una entrada de ramas predeterminadas / protegidas que trataba `main` y `master` como protegidas hasta que nombrara otras. v2.1.211 la eliminó: [los pushes a cualquier rama del repositorio en el que está trabajando](#common-boundaries) se permiten de forma predeterminada, por lo que no hay un valor predeterminado de rama protegida para configurar.</Info>

Para agregar sus propias entradas junto a los valores predeterminados, incluya la cadena literal `"$defaults"` en la matriz. Las entradas predeterminadas se insertan en esa posición, por lo que sus entradas personalizadas pueden ir antes o después de ellas.

El siguiente ejemplo mantiene las entradas predeterminadas y agrega los repositorios, buckets, dominios y servicios de una organización.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Después de guardar su configuración, ejecute `claude auto-mode config` para [confirmar que las reglas efectivas](#inspect-the-defaults-and-your-effective-config) incluyen sus entradas.

Las entradas son prosa, no regex o patrones de herramientas. El clasificador las lee como reglas en lenguaje natural. Escríbalas de la manera en que describiría su infraestructura a un nuevo ingeniero. Una sección de entorno exhaustiva cubre:

* **Organización**: el nombre de su empresa y para qué se utiliza principalmente Claude Code, como desarrollo de software, automatización de infraestructura o ingeniería de datos
* **Control de fuente**: cada organización de GitHub, GitLab o Bitbucket a la que sus desarrolladores hacen push
* **Proveedores de nube y buckets de confianza**: nombres de buckets o prefijos a los que Claude debería poder leer y escribir
* **Dominios internos de confianza**: nombres de host para API, paneles y servicios dentro de su red, como `*.internal.example.com`
* **Servicios internos clave**: CI, registros de artefactos, índices de paquetes internos, herramientas de incidentes
* **Registro de paquetes interno**: el registro privado de npm, PyPI u otro que las instalaciones deben enrutar, por lo que las instalaciones que lo omiten para un registro público se bloquean
* **Ubicaciones de datos sensibles y audiencias**: los buckets, bases de datos o rutas que contienen datos personales, datos comerciales confidenciales, credenciales, datos regulados o material similar sensible, y las audiencias con las que se pueden compartir los datos en cada ubicación, para que el clasificador proteja esas ubicaciones en lugar de adivinar por contenido. Claude Code v2.1.195 a v2.1.197 nombran esta entrada ubicaciones de PII / datos regulados y cubren solo ubicaciones que contienen datos personales o regulados, sin la dimensión de audiencia
* **Objetivos remotos sensibles**: los espacios de nombres, hosts o contenedores que cuentan como producción, por lo que los shells remotos y port-forwards en ellos necesitan su aprobación explícita
* **Alcances de IaC protegidos**: los recursos de infraestructura cuya aplicación o destrucción siempre debe requerir que nombre el cambio
* **Contexto adicional**: restricciones de industria regulada, infraestructura multiinquilino o requisitos de cumplimiento que afecten lo que el clasificador debe tratar como riesgoso

Las entradas de Registro de paquetes interno, Ubicaciones de datos sensibles y audiencias, Objetivos remotos sensibles y Alcances de IaC protegidos requieren Claude Code v2.1.195 o posterior. Las versiones anteriores aún las leen como contexto simple pero no tienen las reglas integradas que las orientan.

Una plantilla de inicio útil: complete los campos entre corchetes y elimine cualquier línea que no se aplique.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Cuanto más contexto específico proporcione, mejor podrá el clasificador distinguir las operaciones internas rutinarias de los intentos de exfiltración.

No necesita completar todo de una vez. Un despliegue razonable: comience con los valores predeterminados y agregue su organización de control de fuente y servicios internos clave, lo que resuelve los falsos positivos más comunes como hacer push a sus propios repositorios. Agregue dominios de confianza y buckets de nube a continuación. Complete el resto a medida que surjan bloqueos.

<h2 id="generate-environment-entries">
  Generar entradas de entorno con `/auto-mode-setup`
</h2>

Ejecute `/auto-mode-setup` para que Claude Code redacte entradas de `autoMode.environment` y, a veces, también [entradas de reglas](#override-the-block-and-allow-rules), a partir de su proyecto y sus sesiones recientes en él. Si acepta el borrador, Claude Code lo escribe en `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` requiere un plan Pro, Max o Team y Claude Code v2.1.228 o posterior. En Windows nativo requiere v2.1.233 o posterior. No puede ejecutarlo en una [sesión en la nube](/docs/es/claude-code-on-the-web). También necesita [obtención de banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), por lo que no puede ejecutarlo en una sesión donde haya desactivado la obtención de banderas.
</Note>

<h3 id="what-auto-mode-setup-reads">
  Qué lee `/auto-mode-setup`
</h3>

Si `~/.claude/settings.json` ya contiene entradas de `autoMode`, Claude Code comienza preguntando si desea agregar a su lista de entorno o reemplazarla, y mantiene las reglas que escribió de cualquier forma. Claude Code luego pregunta cómo utiliza este proyecto y ofrece dos escaneos opcionales antes de escanear cualquier cosa. En el escaneo, Claude Code siempre lee estas fuentes:

* El `CLAUDE.md`, `README.md`, archivos de configuración y remotos de git de este proyecto
* Su configuración de `autoMode` y `permissions.allow`
* Los hosts, buckets y nombres de comandos de los comandos que Claude ejecutó en sus sesiones recientes en este proyecto, nunca sus mensajes

Los dos escaneos opcionales agregan una fuente cada uno:

* La primera palabra de cada comando en su historial de shell
* Los hosts remotos y nombres de los repositorios bajo su directorio de inicio

<h3 id="review-and-save-the-draft">
  Revisar y guardar el borrador
</h3>

Claude Code escanea en segundo plano y luego le muestra el borrador. Acepta o descarta como un todo, así que edite `~/.claude/settings.json` después para ajustar entradas individuales. Cuando acepta, Claude Code escribe el borrador y lo reconcilia con la configuración que ya tiene:

* Claude Code escribe la lista de `environment` sin `"$defaults"`, porque el borrador especifica las entradas integradas que dejó sin cambios
* Claude Code incluye `"$defaults"` en cada una de las listas de `allow`, `soft_deny` y `hard_deny` a las que el borrador agrega entradas, a menos que ya haya escrito una lista de `allow` sin ella, por lo que las [reglas integradas](#override-the-block-and-allow-rules) que no ha reemplazado permanecen en vigor
* Después de guardar, Claude Code ofrece eliminar reglas de `permissions.allow` en `~/.claude/settings.json` que el modo automático ignora, como `Bash(*)`, o que aprueban automáticamente comandos destructivos

Luego ejecute `claude auto-mode config` para [ver el resultado efectivo](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  Desactivar `/auto-mode-setup`
</h3>

Una vez que el modo automático ha bloqueado varias acciones y aún no tiene entradas de `autoMode.environment`, Claude Code muestra un diálogo titulado "¿Enseñar al modo automático sobre su entorno?" al final de un turno y ofrece ejecutar `/auto-mode-setup` para usted. Para detener la oferta pero mantener el comando, seleccione **No mostrar de nuevo** en ese diálogo.

Para desactivar tanto el comando como la oferta, agregue esta entrada de [`skillOverrides`](/docs/es/skills#override-skill-visibility-from-settings) a `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` es un comando integrado en lugar de una [skill agrupada](/docs/es/skills#bundled-skills), por lo que esta entrada de `skillOverrides` aún se aplica a él, pero [`disableBundledSkills`](/docs/es/settings-reference#disablebundledskills) no lo desactiva.

<h2 id="override-the-block-and-allow-rules">
  Anular las reglas de bloqueo y permiso
</h2>

Tres campos adicionales le permiten reemplazar las listas de reglas integradas del clasificador:

* `autoMode.hard_deny`: límites de seguridad incondicionales
* `autoMode.soft_deny`: acciones destructivas que la intención del usuario puede anular
* `autoMode.allow`: excepciones a las reglas de bloqueo suave

Cada uno es una matriz de descripciones en prosa, leídas como reglas en lenguaje natural. Para bloqueos basados en patrones de herramientas que se ejecutan antes del clasificador, utilice [`permissions.deny`](/docs/es/permissions).

Dentro del clasificador, la precedencia funciona en cuatro niveles:

* Las reglas `hard_deny` bloquean incondicionalmente. La intención del usuario y las excepciones `allow` no se aplican.
* Las reglas `soft_deny` bloquean a continuación. La intención del usuario y las excepciones `allow` pueden anular estas.
* Las reglas `allow` luego anulan las reglas `soft_deny` coincidentes como excepciones.
* La intención explícita del usuario anula los bloqueos suaves restantes: si el mensaje del usuario describe directa y específicamente la acción exacta que Claude está a punto de tomar, el clasificador la permite incluso cuando una regla `soft_deny` coincide.

Las solicitudes generales no cuentan como intención explícita. Pedirle a Claude que "limpie el repositorio" no autoriza force-push, pero pedirle que "force-push esta rama" sí.

Para flexibilizar, agregue a `allow` cuando el clasificador marca repetidamente un patrón rutinario que las excepciones predeterminadas no cubren. Para endurecer, agregue a `soft_deny` para riesgos destructivos específicos de su entorno que los valores predeterminados pierden, o a `hard_deny` para límites de seguridad que nunca deben cruzarse.

Para mantener las reglas integradas mientras agrega las suyas propias, incluya la cadena literal `"$defaults"` en la matriz. Las reglas predeterminadas se insertan en esa posición, por lo que sus reglas personalizadas pueden ir antes o después de ellas, y continúa heredando actualizaciones a medida que la lista integrada cambia en las versiones.

El siguiente ejemplo mantiene los valores predeterminados en las cuatro listas y agrega reglas específicas de la organización a cada una.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  Establecer cualquiera de `environment`, `allow`, `soft_deny` o `hard_deny` sin `"$defaults"` reemplaza la lista predeterminada completa para esa sección. Si establece una matriz sin `"$defaults"`, descarta las reglas integradas para esa sección:

  * `soft_deny`: todas las reglas de bloqueo suave integradas, incluido force push, `curl | bash`, despliegues de producción y omisión de modo automático
  * `hard_deny`: la regla integrada de exfiltración de datos
</Danger>

Cada sección se evalúa de forma independiente, por lo que establecer `environment` solo deja intactas las listas predeterminadas `allow`, `soft_deny` y `hard_deny`. Solo omita `"$defaults"` cuando tenga la intención de asumir la propiedad completa de la lista. Para hacerlo de forma segura, ejecute `claude auto-mode defaults` para imprimir las reglas integradas, cópielas en su archivo de configuración, luego revise cada regla contra su propia canalización y tolerancia al riesgo.

<h2 id="edit-rules-from-permissions">
  Editar reglas desde `/permissions`
</h2>

Para ver y editar reglas de clasificador sin abrir un archivo de configuración, ejecute [`/permissions`](/docs/es/permissions#manage-permissions) y seleccione la pestaña **Auto mode**. La pestaña requiere Claude Code v2.1.246 o posterior, y aparece solo cuando [el modo automático está disponible](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para su sesión.

La pestaña enumera las entradas `allow`, `soft_deny`, `hard_deny` y `environment` de cada uno de los [ámbitos que lee el clasificador](#where-the-classifier-reads-configuration), y muestra si las reglas integradas están en vigor para cada sección. Claude Code muestra entradas de [configuración administrada](/docs/es/server-managed-settings) o la bandera `--settings` como solo lectura, y guarda cada cambio que realiza en la pestaña en `~/.claude/settings.json`. Desde la pestaña puede:

* Agregar, editar o eliminar reglas en las secciones `allow`, `soft_deny` y `hard_deny`. Cuando agrega la primera regla a una sección, Claude Code también inserta `"$defaults"` para que las [reglas integradas](#override-the-block-and-allow-rules) sigan en vigor.
* Desactivar o reactivar las reglas integradas para `allow`, `soft_deny` o `hard_deny`. Claude Code registra la opción agregando o eliminando `"$defaults"` en su lista para esa sección, por lo que una sección necesita al menos una regla propia antes de poder desactivar sus reglas integradas.
* Editar las entradas `environment` como un documento en su editor. Si aún no ha configurado ninguna entrada `environment`, Claude Code primero le pregunta si desea reemplazar el entorno integrado, luego abre el editor en el texto integrado completo. Cuando guarda, Claude Code reemplaza su matriz `autoMode.environment` con el documento. Incluya la línea `"$defaults"` para [mantener las entradas integradas](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Enrutar todos los comandos de shell a través del clasificador
</h2>

De forma predeterminada, las reglas estrechas de Bash y PowerShell como `Bash(npm test)` permanecen en vigor en modo automático. Claude Code las resuelve antes de que se ejecute el clasificador, a menos que el comando lleve [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code suspende solo las reglas amplias que otorgan ejecución de código arbitrario, como `Bash(*)` o intérpretes con comodines, junto con cada regla que nombra [`Monitor`](/docs/es/tools-reference#monitor-tool), porque los comandos Monitor se ejecutan a través del shell. Esto significa que una regla estrecha aún puede dejar pasar un argumento destructivo sin que el clasificador lo vea, por ejemplo una ruta de script o bandera que el prefijo de la regla no anticipó.

Establezca `autoMode.classifyAllShell` en `true` para suspender cada regla de permiso de Bash y PowerShell mientras el modo automático está activo, de modo que el clasificador evalúe cada comando de shell independientemente de su lista de permisos.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Esto intercambia latencia por cobertura: un comando que una regla de permiso habría aprobado instantáneamente ahora espera una decisión del clasificador, y cada comando de shell cuenta como una llamada al clasificador.

La configuración se aplica solo mientras el modo automático está activo, y sus reglas de permiso se comportan normalmente en otros modos de permisos.

<Note>
  `autoMode.classifyAllShell` requiere Claude Code v2.1.193 o posterior. Las versiones anteriores ignoran la clave y continúan llevando reglas de permiso de shell estrechas al modo automático.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Inspeccione los valores predeterminados y su configuración efectiva
</h2>

Los subcomandos `claude auto-mode` le ayudan a inspeccionar, validar y restablecer su configuración.

Imprima las reglas integradas `environment`, `allow`, `soft_deny` y `hard_deny` como JSON:

```bash theme={null}
claude auto-mode defaults
```

Para leer la redacción completa de una regla sin canalizar a través de `jq`, pase `--label` con el inicio de la etiqueta de la regla, como `claude auto-mode defaults --label 'Git Destructive'`. La coincidencia es un prefijo que no distingue mayúsculas de minúsculas en la etiqueta de cada regla, y las secciones sin coincidencia se imprimen como listas vacías. Requiere Claude Code v2.1.208 o posterior.

Imprima lo que el clasificador realmente utiliza como JSON, con su configuración aplicada donde esté establecida y valores predeterminados en caso contrario:

```bash theme={null}
claude auto-mode config
```

Tanto `defaults` como `config` imprimen las cuatro listas de reglas como un único objeto JSON, con cada regla como una cadena de prosa. Este es un ejemplo truncado:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

Obtenga comentarios de IA sobre sus reglas personalizadas `allow`, `soft_deny` y `hard_deny`:

```bash theme={null}
claude auto-mode critique
```

Ejecute `claude auto-mode config` después de guardar su configuración para confirmar que las reglas efectivas son las que espera, con `"$defaults"` expandido en su lugar. Si ha escrito reglas personalizadas, `claude auto-mode critique` las revisa y marca las entradas que son ambiguas, redundantes o que probablemente causen falsos positivos.

Para descartar sus personalizaciones y volver a los valores predeterminados integrados, ejecute el subcomando reset. Requiere Claude Code v2.1.212 o posterior y elimina la sección `autoMode` de su archivo de configuración de usuario:

```bash theme={null}
claude auto-mode reset
```

El comando resume lo que eliminará y pregunta `Reset auto mode configuration to defaults?` antes de escribir; pase `--yes` para omitir la confirmación. Reset cambia solo `~/.claude/settings.json`: las reglas `autoMode` de [configuración administrada por el servidor](/docs/es/server-managed-settings) o la bandera `--settings` aún se aplican.

<h2 id="review-denials">
  Revisar denegaciones
</h2>

Para revisar y reintentar acciones que el clasificador del modo automático denegó, abra `/permissions` y seleccione la pestaña **Recently denied** (Denegadas recientemente), donde Claude Code registra cada denegación. Presione `r` en una acción denegada para marcarla para reintentar: cuando salga del diálogo, Claude Code envía un mensaje indicando al modelo que puede reintentar esa llamada de herramienta y reanuda la conversación.

Cuando el clasificador produce [ningún veredicto sobre la acción](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action), porque una verificación de seguridad separada del modo automático rechazó la solicitud del clasificador o su respuesta no se analizó, Claude Code deniega la acción sin registrarla en **Recently denied** (Denegadas recientemente). La entrada de error vinculada cubre lo que se le dice a Claude y cómo ejecutar la acción si la necesita.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Corregir una denegación con una regla de permiso, una entrada de entorno o un reintento
</h3>

Para ver qué bloqueó el clasificador, encuentre la llamada de herramienta en la conversación. Si la llamada aparece acortada o plegada en una línea de resumen como `Ran 3 shell commands` (Ejecutó 3 comandos de shell), presione `Ctrl+O` para abrir el [visor de transcripción](/docs/es/interactive-mode#transcript-viewer), que la expande.

Otros dos lugares en la pantalla que reportan denegaciones omiten el comando o la URL: el aviso cerca de la caja de entrada, como `bash denied by auto mode · [Data Exfiltration] · /permissions`, proporciona la herramienta y la razón, y la pestaña **Recently denied** (Denegadas recientemente) enumera un comando de shell por la descripción que Claude escribió para él. Para capturar la entrada exacta de estas denegaciones mediante programación, agregue un [hook `PermissionDenied`](/docs/es/hooks#permissiondenied), que la recibe como `tool_input`.

El texto debajo de la llamada le indica si hay algo que corregir. El texto que reporta un problema con el clasificador en sí, como un modelo que `is temporarily unavailable` (no está disponible temporalmente) o un error del clasificador, significa que Claude Code bloqueó la llamada sin un veredicto final del clasificador; consulte [Auto mode cannot determine the safety of an action](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action) (El modo automático no puede determinar la seguridad de una acción) para saber qué hacer. De lo contrario, una línea que dice `Denied by auto mode classifier` (Denegado por clasificador de modo automático) con una razón como `[Production Deploy]` (Implementación en producción) o `Blocked by classifier` (Bloqueado por clasificador) significa que el clasificador juzgó la llamada como insegura, así que elija la corrección de lo que la llamada estaba intentando alcanzar o hacer:

* Un destino que Claude necesita durante toda la tarea, como un registro de paquetes, un dominio interno o un host de repositorio: agréguelo a `autoMode.environment`.
* Un comando que desea ejecutar sin revisión de ahora en adelante: agregue una regla `allow`.
* Una acción única que tenía la intención de hacer: indique esa intención en su próximo mensaje y deje que Claude reintente.

Puede agregar la entrada de entorno o la regla `allow` desde la pestaña [**Auto mode**](#edit-rules-from-permissions) del diálogo `/permissions`.

En la mayoría de las sesiones, la razón nombra la regla que el clasificador coincidió, entre corchetes, como `[Data Exfiltration]` (Exfiltración de datos) o `[Production Deploy]` (Implementación en producción), y algunas sesiones ejecutan un modelo clasificador que agrega una breve explicación. Claude Code selecciona el modelo clasificador, por lo que la forma que ve no es algo que configure.

<h3 id="fix-repeated-denials">
  Corregir denegaciones repetidas
</h3>

Las denegaciones repetidas para el mismo destino generalmente significan que al clasificador le falta contexto. Agregue ese destino a `autoMode.environment`, o [ejecute `/auto-mode-setup`](#generate-environment-entries) para que Claude Code redacte las entradas, luego ejecute `claude auto-mode config` para confirmar que el cambio surtió efecto.

Para reaccionar a las denegaciones mediante programación, use el [hook `PermissionDenied`](/docs/es/hooks#permissiondenied).

<h2 id="see-also">
  Véase también
</h2>

* [Modos de permiso](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): qué es el modo automático, qué bloquea de forma predeterminada y qué sesiones se inician en él
* [Configuración administrada](/docs/es/server-managed-settings): implemente la configuración de `autoMode` en toda su organización
* [Permisos](/docs/es/permissions): reglas de permitir, preguntar y denegar que se aplican antes de que se ejecute el clasificador
* [Todos los ajustes](/docs/es/settings-reference#automode): todas las claves de configuración, incluida `autoMode`
