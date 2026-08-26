> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar el modo automático

> Indique al clasificador del modo automático qué repositorios, buckets y dominios confía su organización. Establezca el contexto del entorno, anule las reglas de bloqueo y permiso predeterminadas e inspeccione su configuración efectiva con los subcomandos de la CLI del modo automático.

[El modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) permite que Claude Code se ejecute sin solicitudes de permiso rutinarias al enrutar las llamadas de herramientas a través de un clasificador que bloquea cualquier cosa irreversible, destructiva o dirigida fuera de su entorno. Las reglas de denegación y solicitud explícita se evalúan antes del clasificador y aún bloquean o solicitan. Utilice el bloque de configuración `autoMode` para indicar a ese clasificador qué repositorios, buckets y dominios confía su organización, de modo que deje de bloquear operaciones internas rutinarias.

<Note>
  El modo automático está disponible para todos los usuarios en cada proveedor, incluida la API de Anthropic, Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry y sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada. Si Claude Code informa que el modo automático no está disponible para su cuenta, consulte los [requisitos completos](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), que también cubren los modelos admitidos y la habilitación del propietario en planes de equipo y empresa. En v2.1.158 a v2.1.206, el modo automático en Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry y sesiones de puerta de enlace de aplicaciones Claude requería establecer `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 eliminó el requisito.
</Note>

De forma predeterminada, el clasificador confía solo en el directorio de trabajo y en los remotos configurados del repositorio actual. Las acciones como insertar en la organización de control de código fuente de su empresa o escribir en un bucket de nube de equipo se bloquean hasta que las agregue a `autoMode.environment`.

Para saber cómo habilitar el modo automático y qué bloquea de forma predeterminada, consulte [Modos de permiso](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). Esta página es la referencia de configuración.

Esta página cubre cómo:

* [Agregar un checkpoint manual](#common-boundaries) para inserciones y solicitudes de extracción con `permissions.ask`
* [Elegir dónde establecer reglas](#where-the-classifier-reads-configuration) en CLAUDE.md, configuración de usuario y configuración administrada
* [Definir infraestructura de confianza](#define-trusted-infrastructure) con `autoMode.environment`
* [Anular las reglas de bloqueo y permiso](#override-the-block-and-allow-rules) cuando los valores predeterminados no se ajustan a su canalización
* [Enrutar todos los comandos de shell a través del clasificador](#route-all-shell-commands-through-the-classifier) con `autoMode.classifyAllShell`
* [Inspeccionar su configuración efectiva](#inspect-the-defaults-and-your-effective-config) con los subcomandos `claude auto-mode`
* [Revisar denegaciones](#review-denials) para saber qué agregar a continuación

<h2 id="common-boundaries">
  Límites comunes
</h2>

El modo automático permite inserciones en su rama de trabajo, inserciones rutinarias en la rama predeterminada del repositorio y creación de solicitudes de extracción de forma predeterminada. El clasificador bloquea una inserción solo cuando conlleva riesgo, como una inserción forzada o contenido que elude una revisión que configuró. Si desea un punto de control humano antes de cada inserción o solicitud de extracción, agregue reglas de permisos: las recetas a continuación mantienen el modo automático activado para todo lo demás.

El mecanismo más directo es [`permissions.ask`](/docs/es/permissions#permission-rule-syntax). Las reglas de solicitud con alcance de contenido como las que se muestran a continuación se evalúan antes del clasificador y siempre fuerzan un aviso de permiso, incluso en modo automático, porque una regla de solicitud explícita es su intención declarada de ser solicitado para esa acción. Agregue las reglas en su [configuración](/docs/es/settings#settings-files):

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

Elija el mecanismo que se ajuste a lo firme que deba ser el límite:

| Límite                        | Mecanismo                                                        | Comportamiento en modo automático                                                                                                                                                                                                                            |
| :---------------------------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Solicitar antes de la acción  | `permissions.ask`                                                | Siempre solicita para reglas con alcance de contenido como la receta anterior. El clasificador no puede aprobar automáticamente una acción coincidente.                                                                                                      |
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

Para la mayoría de las organizaciones, `autoMode.environment` es el único campo que necesita establecer. Indica al clasificador qué repositorios, buckets y dominios son de confianza: el clasificador lo utiliza para decidir qué significa "externo", por lo que cualquier destino no listado es un objetivo potencial de exfiltración.

A partir de Claude Code v2.1.198, `claude auto-mode defaults` imprime tres tipos de entrada de entorno. Las versiones anteriores a v2.1.195 imprimen solo los primeros cinco espacios de confianza.

* **Espacios de contexto**: describen su organización, stack y postura de seguridad para que el clasificador lea las otras reglas en su contexto. A diferencia de los otros dos tipos, los espacios de contexto no tienen reglas propias que los dirijan. Cada uno se establece de forma predeterminada en `None configured` o en la suposición conservadora nombrada junto a él:
  * **Organización**
  * **Uso principal de Claude Code**: se establece de forma predeterminada en desarrollo de software
  * **Proveedor(es) de nube**
  * **Visibilidad del repositorio**: se asume que un repositorio es privado a menos que su host remoto y nombre indiquen lo contrario, o una verificación de visibilidad anterior en la conversación que el clasificador lee muestre que es público. El clasificador lee sus mensajes y los comandos que ejecuta Claude, no su salida, por lo que la evidencia tiene que ser algo que pueda leer, como su propio mensaje nombrando el repositorio como público; la salida de un `gh repo view` por sí sola no llega a él. La verificación de evidencia de transcripción requiere Claude Code v2.1.200 o posterior
  * **Compartición interna / alojamiento de fragmentos**: los servicios públicos de paste y gist se tratan como fuera del límite de confianza hasta que nombre uno
  * **CLI específicas de la organización**
  * **Gestión de secretos**
  * **Ramas predeterminadas / protegidas**: `main` y `master` se tratan como protegidas hasta que nombre otras
  * **Objetivos de implementación de CI/CD**
  * **Postura de red**
  * **Espacios de nombres / entornos de implementación protegidos**: recurre a la heurística de objetivos remotos sensibles hasta que nombre algunos
  * **Retención de datos / desclasificación**
* **Espacios de confianza**: nombran lo que el clasificador trata como dentro de su límite. Los espacios son Repositorio de confianza, Control de código fuente, Dominios internos de confianza, Buckets de nube de confianza, Servicios internos clave y Registro de paquetes interno. Las entradas de repositorio y control de código fuente se establecen de forma predeterminada en el repositorio de trabajo y sus remotos configurados. Todos los demás espacios de confianza se establecen de forma predeterminada en `None configured`, por lo que nada más es de confianza hasta que lo agregue. La visibilidad de un repositorio solo abarca material confidencial: un repositorio privado es un destino aceptable para material confidencial, pero hacer que un repositorio sea privado nunca borra secretos o datos personales o confiados en él, y el clasificador trata el contenido portado, reapuntado o leído por primera vez desde fuera del repositorio de trabajo como no siendo trabajo propio de ese repositorio. Este alcance requiere Claude Code v2.1.203 o posterior.
* **Espacios de sensibilidad**: nombran lo que las reglas de protección tratan como de alto riesgo. Los espacios son Ubicaciones de datos sensibles y audiencias, Objetivos remotos sensibles y Ámbitos de IaC protegidos. Cada uno se establece de forma predeterminada en una heurística amplia, como tratar cualquier host o espacio de nombres cuyo nombre lleve `prod` o `production` como un objetivo remoto sensible, por lo que las reglas de protección están activas antes de que configure nada. Nombrar objetivos concretos en un espacio de sensibilidad hace que esas reglas se apliquen a los objetivos nombrados en lugar de la heurística.

Para agregar sus propias entradas junto con los valores predeterminados, incluya la cadena literal `"$defaults"` en la matriz. Las entradas predeterminadas se insertan en esa posición, por lo que sus entradas personalizadas pueden ir antes o después de ellas.

El siguiente ejemplo mantiene las entradas predeterminadas y agrega repositorios, buckets, dominios y servicios de una organización.

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

Las entradas son prosa, no regex o patrones de herramientas. El clasificador las lee como reglas en lenguaje natural. Escríbalas como lo haría al describir su infraestructura a un nuevo ingeniero. Una sección de entorno exhaustiva cubre:

* **Organización**: el nombre de su empresa y para qué se utiliza principalmente Claude Code, como desarrollo de software, automatización de infraestructura o ingeniería de datos
* **Control de código fuente**: cada organización de GitHub, GitLab o Bitbucket a la que sus desarrolladores envían
* **Proveedores de nube y buckets de confianza**: nombres de buckets o prefijos que Claude debería poder leer y escribir
* **Dominios internos de confianza**: nombres de host para API, paneles y servicios dentro de su red, como `*.internal.example.com`
* **Servicios internos clave**: CI, registros de artefactos, índices de paquetes internos, herramientas de incidentes
* **Registro de paquetes interno**: el registro npm, PyPI u otro privado a través del cual deben enrutarse las instalaciones, de modo que las instalaciones que lo omitan para un registro público se bloqueen
* **Ubicaciones de datos sensibles y audiencias**: los buckets, bases de datos o rutas que contienen datos personales, datos comerciales confidenciales, credenciales, datos regulados o material similar sensible, y las audiencias con las que los datos en cada ubicación pueden compartirse, para que el clasificador proteja esas ubicaciones en lugar de adivinar por el contenido. Claude Code v2.1.195 a v2.1.197 nombran esta entrada ubicaciones de PII / datos regulados y cubren solo ubicaciones que contienen datos personales o regulados, sin la dimensión de audiencia
* **Objetivos remotos sensibles**: los espacios de nombres, hosts o contenedores que cuentan como producción, de modo que los shells remotos y los reenvíos de puertos hacia ellos necesiten su aprobación explícita
* **Ámbitos de IaC protegidos**: los recursos de infraestructura cuya aplicación o destrucción siempre debe requerir que nombre el cambio
* **Contexto adicional**: restricciones de industria regulada, infraestructura multiinquilino o requisitos de cumplimiento que afecten lo que el clasificador debe tratar como riesgoso

Las entradas de Registro de paquetes interno, Ubicaciones de datos sensibles y audiencias, Objetivos remotos sensibles y Ámbitos de IaC protegidos requieren Claude Code v2.1.195 o posterior. Las versiones anteriores aún las leen como contexto simple pero no tienen las reglas integradas que las dirigen.

Una plantilla de inicio útil: complete los campos entre corchetes y elimine las líneas que no se apliquen.

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

Cuanto más contexto específico proporcione, mejor podrá el clasificador distinguir operaciones internas rutinarias de intentos de exfiltración.

No necesita completar todo de una vez. Un despliegue razonable: comience con los valores predeterminados y agregue su organización de control de código fuente y servicios internos clave, lo que resuelve los falsos positivos más comunes como enviar a sus propios repositorios. Agregue dominios de confianza y buckets de nube a continuación. Complete el resto a medida que surjan bloqueos.

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

<h2 id="route-all-shell-commands-through-the-classifier">
  Enrutar todos los comandos de shell a través del clasificador
</h2>

De forma predeterminada, las reglas de permiso estrechas de Bash y PowerShell como `Bash(npm test)` se trasladan al modo automático y se resuelven antes de que se ejecute el clasificador. El modo automático suspende solo las reglas amplias que otorgan ejecución de código arbitrario, como `Bash(*)` o intérpretes con caracteres comodín. Esto significa que una regla estrecha aún puede dejar pasar un argumento destructivo sin que el clasificador lo vea, por ejemplo una ruta de script o bandera que el prefijo de la regla no anticipó.

Establezca `autoMode.classifyAllShell` en `true` para suspender todas las reglas de permiso de Bash y PowerShell mientras el modo automático está activo, de modo que el clasificador evalúe cada comando de shell independientemente de su lista de permisos.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Esto intercambia latencia por cobertura: un comando que una regla de permiso habría aprobado instantáneamente ahora espera una decisión del clasificador, y cada comando de shell cuenta como una llamada del clasificador.

La configuración se aplica solo mientras el modo automático está activo, y sus reglas de permiso se comportan normalmente en otros modos de permiso.

<Note>
  `autoMode.classifyAllShell` requiere Claude Code v2.1.193 o posterior. Las versiones anteriores ignoran la clave y continúan trasladando reglas de permiso de shell estrechas al modo automático.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Inspeccione los valores predeterminados y su configuración efectiva
</h2>

Tres subcomandos de CLI lo ayudan a inspeccionar y validar su configuración.

Imprima las reglas `environment`, `allow`, `soft_deny` y `hard_deny` integradas como JSON:

```bash theme={null}
claude auto-mode defaults
```

Para leer la redacción completa de una regla sin canalizar a través de `jq`, pase `--label` con el inicio de la etiqueta de la regla, como `claude auto-mode defaults --label 'Git Destructive'`. La coincidencia es un prefijo que no distingue mayúsculas de minúsculas en la etiqueta de cada regla, y las secciones sin coincidencia se imprimen como listas vacías. Requiere Claude Code v2.1.208 o posterior.

Imprima lo que el clasificador realmente utiliza como JSON, con su configuración aplicada donde se establece y valores predeterminados en caso contrario:

```bash theme={null}
claude auto-mode config
```

Obtenga retroalimentación de IA sobre sus reglas `allow`, `soft_deny` y `hard_deny` personalizadas:

```bash theme={null}
claude auto-mode critique
```

Ejecute `claude auto-mode config` después de guardar su configuración para confirmar que las reglas efectivas son las que espera, con `"$defaults"` expandido en su lugar. Si ha escrito reglas personalizadas, `claude auto-mode critique` las revisa y marca entradas que son ambiguas, redundantes o probables que causen falsos positivos.

Si necesita eliminar o reescribir una regla integrada en lugar de agregar una junto a ella, guarde la salida de `claude auto-mode defaults` en un archivo, edite las listas y pegue el resultado en su archivo de configuración en lugar de `"$defaults"`.

<h2 id="review-denials">
  Revisar denegaciones
</h2>

Cuando el modo automático deniega una llamada de herramienta, la denegación se registra en `/permissions` bajo la pestaña Denegados recientemente. Presione `r` en una acción denegada para marcarla para reintentar: cuando salga del diálogo, Claude Code envía un mensaje indicando al modelo que puede reintentar esa llamada de herramienta y reanuda la conversación.

En Claude Code v2.1.193 y posterior, la razón del clasificador para cada denegación aparece junto a la llamada de herramienta bloqueada en la transcripción, en la notificación de denegación y bajo cada entrada en la pestaña Denegados recientemente. Utilice la razón para decidir si la solución es una entrada `environment`, una excepción `allow` o reintentar con intención explícita en su próximo mensaje.

Las denegaciones repetidas para el mismo destino generalmente significan que el clasificador carece de contexto. Agregue ese destino a `autoMode.environment`, luego ejecute `claude auto-mode config` para confirmar que surtió efecto.

Para reaccionar a las denegaciones mediante programación, utilice el [hook `PermissionDenied`](/docs/es/hooks#permissiondenied).

<h2 id="see-also">
  Ver también
</h2>

* [Modos de permiso](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): qué es el modo automático, qué bloquea de forma predeterminada y cómo habilitarlo
* [Configuración administrada](/docs/es/server-managed-settings): implemente la configuración `autoMode` en toda su organización
* [Permisos](/docs/es/permissions): reglas de permiso, pregunta y denegación que se aplican antes de que se ejecute el clasificador
* [Configuración](/docs/es/settings): la referencia de configuración completa, incluida la clave `autoMode`
