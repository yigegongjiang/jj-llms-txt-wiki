> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Ejecute Claude Code en flujos de trabajo de GitHub Actions para responder a menciones @claude, automatizar tareas y convertir problemas en solicitudes de extracción

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) es una GitHub Action que ejecuta Claude Code dentro de los flujos de trabajo de su repositorio. Mencione `@claude` en un comentario de solicitud de extracción o problema para que Claude analice código, implemente cambios e inserte confirmaciones. También puede darle a la GitHub Action de Claude Code un mensaje para que se ejecute automáticamente en cualquier evento de GitHub. Úselo para convertir problemas en solicitudes de extracción, corregir errores desde un comentario o automatizar tareas recurrentes.

Varios productos comparten el nombre Claude Code. Esta página cubre la integración de flujo de trabajo `claude-code-action`, que configura con archivos de flujo de trabajo en su repositorio. Para los productos relacionados, consulte:

* [Code Review](/docs/es/code-review): revisión automática en cada solicitud de extracción, sin escribir un flujo de trabajo
* [Claude Code en la nube](/docs/es/claude-code-on-the-web): sesiones de Claude Code que se ejecutan en infraestructura en la nube en lugar de su máquina
* [Claude Agent SDK](/docs/es/agent-sdk/overview): automatización personalizada fuera de GitHub Actions. La GitHub Action de Claude Code se construye sobre el SDK
* [GitHub Enterprise Server](/docs/es/github-enterprise-server): Claude Code con GitHub autohospedado

<h2 id="setup">
  Configuración
</h2>

Puede configurar la GitHub Action de Claude Code de una de dos formas:

* **Configuración rápida**: ejecute `/install-github-app` desde Claude Code. Claude Code instala la GitHub App, agrega su secreto de autenticación y prepara la solicitud de extracción del flujo de trabajo para usted
* **Configuración manual**: instale la aplicación, agregue el secreto y copie el archivo de flujo de trabajo en su repositorio usted mismo. Use esta ruta cuando no ejecute Claude Code localmente, cuando el comando falle o cuando desee control total de los archivos de flujo de trabajo

Para cualquiera de las rutas, necesita acceso de administrador al repositorio.

<h3 id="quick-setup">
  Configuración rápida
</h3>

`/install-github-app` funciona solo con repositorios de github.com. Si el repositorio remoto de git está en gitlab.com o bitbucket.org, el comando imprime un aviso y sale en lugar de iniciar la configuración. Para ejecutar Claude Code desde canalizaciones de GitLab, consulte [Claude Code GitLab CI/CD](/docs/es/gitlab-ci-cd).

Antes de comenzar, instale la [GitHub CLI](https://cli.github.com) y auténtiquese con `gh auth login`. Claude Code la verifica y le advierte si falta.

Abra `claude` en el repositorio que desea conectar, ejecute `/install-github-app` y siga las indicaciones. Claude Code instala la GitHub App de Claude, luego configura un secreto de autenticación para los flujos de trabajo:

* Si Claude Code ya tiene una clave API, reutiliza esa clave y ofrece mantener el secreto `ANTHROPIC_API_KEY` existente del repositorio si ya está configurado
* De lo contrario, elija entre crear un token de larga duración con su suscripción de Claude y pegar una clave API

Claude Code guarda la credencial como un secreto del repositorio, denominado `ANTHROPIC_API_KEY` para una clave API o `CLAUDE_CODE_OAUTH_TOKEN` para un token de suscripción.

Claude Code luego inserta una rama con los archivos de flujo de trabajo que selecciona, ya configurados para usar ese secreto, y abre GitHub en su navegador con una solicitud de extracción lista para crear. Cree y fusione esa solicitud de extracción, y `@claude` funciona en el repositorio.

Si selecciona el flujo de trabajo de revisión, Claude publica cada revisión en la solicitud de extracción misma, como un comentario en línea en cada problema que encuentra o como un comentario de resumen cuando no encuentra ninguno. Claude omite algunas solicitudes de extracción, como borradores. El [ejemplo de flujo de trabajo de revisión](#run-a-skill) usa la misma skill y las enumera. Antes de v2.1.229, Claude escribía su revisión solo en el registro de ejecución del flujo de trabajo.

Para actualizar un flujo de trabajo de revisión que una versión anterior generó, haga una de las siguientes:

* Ejecute `/install-github-app` nuevamente. Cuando el repositorio ya tiene un `claude.yml`, seleccione **Actualizar archivo de flujo de trabajo con la última versión**. Claude Code inserta copias nuevas de los archivos de flujo de trabajo en una rama nueva y abre la solicitud de extracción, igual que una primera instalación.
* Agregue el argumento `--comment` y la línea `claude_args` del [ejemplo de flujo de trabajo de revisión](#run-a-skill) al archivo registrado usted mismo, lo que mantiene cualquier otra edición que haya realizado.

Después de instalar la GitHub App, Claude Code pregunta si desea continuar con la configuración de GitHub Actions. Elija **Omitir por ahora** para detener solo con la GitHub App instalada. Ejecute `/install-github-app` nuevamente más tarde para terminar los pasos de flujo de trabajo y secreto.

<Note>
  * Cuando instala la GitHub App, le otorga varios permisos. Consulte [Permisos de GitHub App](#github-app-permissions) para el conjunto completo
  * La configuración rápida funciona con la API de Claude y las suscripciones de Claude. Si usa Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, consulte [Usar Claude Code GitHub Actions con proveedores en la nube](/docs/es/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Configuración manual
</h3>

Para configurar la GitHub Action de Claude Code sin ejecutar `/install-github-app`, instale la aplicación, agregue un secreto y copie un archivo de flujo de trabajo usted mismo:

<Steps>
  <Step title="Instale la GitHub App de Claude">
    Instale la [GitHub App de Claude](https://github.com/apps/claude) en su repositorio. La GitHub Action de Claude Code depende de tres de los permisos de la aplicación:

    * **Contents**: lectura y escritura, para que Claude pueda modificar archivos del repositorio
    * **Issues**: lectura y escritura, para que Claude pueda responder a problemas
    * **Pull requests**: lectura y escritura, para que Claude pueda crear PR e insertar cambios

    Durante la instalación, también otorga permisos que otras características de Claude usan. Consulte [Permisos de GitHub App](#github-app-permissions) para el conjunto completo.
  </Step>

  <Step title="Agregue un secreto de autenticación">
    Agregue uno de los siguientes secretos a su repositorio, dependiendo de cómo se autentique. Consulte la guía de GitHub sobre [uso de secretos en GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: una clave API de Claude de la [Consola de Claude](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: un token OAuth que se autentica con su suscripción de Claude, disponible en planes Pro, Max, Team y Enterprise. Genere uno ejecutando `claude setup-token` localmente. Consulte [Generar un token de larga duración](/docs/es/authentication#generate-a-long-lived-token)

    En archivos de flujo de trabajo, pase el secreto a la entrada correspondiente: `anthropic_api_key` para una clave API, o `claude_code_oauth_token` para un token OAuth.
  </Step>

  <Step title="Copie el archivo de flujo de trabajo">
    Copie [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) en el directorio `.github/workflows/` de su repositorio. El archivo es un flujo de trabajo funcional, no solo un ejemplo. Tal como está confirmado, Claude responde cada vez que alguien menciona `@claude` en un problema o solicitud de extracción, autenticándose con el secreto `ANTHROPIC_API_KEY`. Si agregó `CLAUDE_CODE_OAUTH_TOKEN` en su lugar, cambie la línea `anthropic_api_key` del flujo de trabajo a `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Después de la configuración, pruebe la GitHub Action de Claude Code etiquetando `@claude` en un comentario de problema o PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Configurar para una organización
</h3>

Con la configuración rápida o manual, configura un repositorio a la vez. Para implementar la GitHub Action de Claude Code en toda una organización:

* Instale la [GitHub App de Claude](https://github.com/apps/claude) una vez a nivel de organización, eligiendo todos los repositorios o una lista seleccionada
* Almacene el secreto de autenticación como un secreto de Actions a nivel de organización para que cada repositorio no necesite su propia copia
* Agregue el archivo de flujo de trabajo a cada repositorio que deba ejecutar la GitHub Action de Claude Code, o defina el trabajo una vez como un [flujo de trabajo reutilizable](https://docs.github.com/en/actions/using-workflows/reusing-workflows) que cada repositorio llama

Para un secreto compartido entre repositorios, auténtiquese con una clave API de la [Consola de Claude](https://platform.claude.com) en lugar de un token OAuth, ya que un token OAuth está vinculado a la suscripción de la persona que ejecutó `claude setup-token`.

Para evitar almacenar un secreto de larga duración en absoluto, auténtiquese a través de federación de identidad de carga de trabajo, donde la GitHub Action de Claude Code intercambia el token OpenID Connect (OIDC) de GitHub del flujo de trabajo por acceso a la API de Claude a través de una cuenta de servicio de la Consola de Claude. Establezca estas entradas:

* `anthropic_federation_rule_id`: el ID de la regla de federación, `fdrl_...`
* `anthropic_organization_id`: su ID de organización de Anthropic
* `anthropic_service_account_id`: el ID de la cuenta de servicio, `svac_...`. Opcional, ya que la regla de federación que crea en la Consola ya apunta a una cuenta de servicio
* `anthropic_workspace_id`: el ID del espacio de trabajo, `wrkspc_...`. Opcional cuando la regla de federación apunta a un único espacio de trabajo

Otorgue al flujo de trabajo el permiso `id-token: write`, que la GitHub Action de Claude Code necesita para el intercambio de federación incluso cuando pasa su propio `github_token`. Consulte la [guía de configuración de la GitHub Action de Claude Code](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) para la configuración del lado de la Consola.

Para preguntas sobre manejo de datos y retención en una revisión de seguridad, consulte [uso de datos](/docs/es/data-usage) y [seguridad](/docs/es/security).

<h3 id="uninstall">
  Desinstalar
</h3>

Para eliminar la GitHub Action de Claude Code, deshaga cada parte de la configuración que se aplique a su instalación:

* **Archivos de flujo de trabajo**: elimine los flujos de trabajo que usan `anthropics/claude-code-action` de `.github/workflows/`. Si usó la configuración rápida, busque `claude.yml` y, si seleccionó el flujo de trabajo de revisión, `claude-code-review.yml`. Con los flujos de trabajo eliminados, la GitHub Action de Claude Code ya no se ejecuta
* **Secretos**: elimine el secreto `ANTHROPIC_API_KEY` o `CLAUDE_CODE_OAUTH_TOKEN` del repositorio, y de los secretos de Actions a nivel de organización si lo [compartió entre repositorios](#set-up-for-an-organization). Si elimina un secreto, la credencial que contenía sigue siendo válida. Para retirar completamente una clave API, también elimine la clave en la [Consola de Claude](https://platform.claude.com)
* **GitHub App**: desinstale la GitHub App de Claude en la configuración de su repositorio u organización en GitHub Apps, pero solo si no la usa para otra característica de Claude, como Code Review o auto-fix web

Si configuró un [proveedor en la nube](/docs/es/github-actions-cloud-providers), también elimine los secretos del proveedor, como `AWS_ROLE_TO_ASSUME`, los secretos `GCP_*` o los secretos `AZURE_*`, y desinstale la GitHub App personalizada junto con sus secretos `APP_ID` y `APP_PRIVATE_KEY`.

<h3 id="github-app-permissions">
  Permisos de GitHub App
</h3>

La [GitHub App de Claude](https://github.com/apps/claude) es compartida por cada característica de Claude que se integra con GitHub, incluida la GitHub Action de Claude Code, [Code Review](/docs/es/code-review) y [auto-fix para solicitudes de extracción](/docs/es/claude-code-on-the-web#auto-fix-pull-requests) en Claude Code en la web. Una GitHub App tiene un conjunto de permisos único que cubre todas sus características, por lo que el conjunto incluye algunos permisos que la GitHub Action de Claude Code no usa.

Cuando instala la aplicación, otorga los siguientes permisos:

| Permiso          | Acceso              |
| ---------------- | ------------------- |
| Actions          | Lectura y escritura |
| Checks           | Lectura y escritura |
| Contents         | Lectura y escritura |
| Discussions      | Lectura y escritura |
| Issues           | Lectura y escritura |
| Members          | Lectura             |
| Metadata         | Lectura             |
| Pull requests    | Lectura y escritura |
| Repository hooks | Lectura y escritura |
| Statuses         | Lectura             |
| Workflows        | Lectura y escritura |

El conjunto de permisos también puede cambiar antes de las características que lo usan. Cuando la aplicación solicita un permiso que no tenía antes, GitHub solicita al propietario de la cuenta que lo apruebe, un propietario de la organización para una instalación de organización, y la instalación mantiene sus permisos antiguos hasta que lo hagan. Por ejemplo, cuando el acceso de Actions cambia de lectura a escritura, la aplicación puede volver a ejecutar flujos de trabajo en lugar de solo ver ejecuciones y registros, por lo que GitHub le pide al propietario que apruebe el cambio.

Cuando instala la aplicación, acepta su conjunto de permisos completo. GitHub no le permite aceptar un subconjunto. Si su organización requiere solo los permisos que usa la GitHub Action de Claude Code, cree una GitHub App personalizada con Contents, Issues y Pull requests en su lugar, siguiendo la [guía de configuración de la GitHub Action de Claude Code](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Una aplicación personalizada cubre solo la GitHub Action de Claude Code. Code Review y auto-fix web aún requieren la aplicación oficial.

Para detalles sobre cómo la GitHub Action de Claude Code limita lo que Claude puede hacer con estos permisos, consulte la [documentación de seguridad](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Modos interactivo y de automatización
</h2>

La GitHub Action de Claude Code detecta cómo ejecutarse desde su configuración de flujo de trabajo:

* **Modo interactivo**: cuando el flujo de trabajo no proporciona entrada `prompt`, Claude espera la frase de disparo, `@claude` por defecto, en un comentario de problema o solicitud de extracción, en una revisión de solicitud de extracción o en el cuerpo o título de un problema recién abierto, luego responde a esa solicitud. El progreso y los resultados aparecen como un comentario en el problema o PR que desencadena.
* **Modo de automatización**: cuando el flujo de trabajo proporciona una entrada `prompt`, Claude se ejecuta sin esperar una mención, sujeto solo a las [comprobaciones sobre quién puede desencadenar ejecuciones](#who-can-trigger-runs). Por defecto, los resultados aparecen en el registro de ejecución del flujo de trabajo en lugar de un comentario. Claude puede publicar en el problema o solicitud de extracción cuando el mensaje lo dirige y tiene una herramienta que puede publicar, como en el [ejemplo de revisión de código](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Quién puede desencadenar ejecuciones
</h3>

En ambos modos, la GitHub Action de Claude Code ejecuta dos comprobaciones en el actor que desencadena antes de que Claude comience, y la ejecución falla cuando cualquiera de las comprobaciones la rechaza:

* **Acceso de escritura**: en eventos de problema y solicitud de extracción, el usuario que desencadena debe tener acceso de escritura al repositorio. Para permitir usuarios específicos sin acceso de escritura, establezca `allowed_non_write_users` y pase su propia entrada `github_token`. Los eventos que ningún usuario crea, como un disparador `schedule`, omiten esta comprobación.
* **Actor humano**: en cada evento, la GitHub Action de Claude Code rechaza un actor bot a menos que lo enumere en `allowed_bots`, lo que evita que los bots desencadenen Claude en un bucle. Esta comprobación también se aplica a ejecuciones programadas, que GitHub atribuye a un usuario del repositorio, generalmente el que cambió por última vez el cronograma `cron` del flujo de trabajo. Si ese usuario es un bot, enumérelo en `allowed_bots`.

<h2 id="example-use-cases">
  Casos de uso de ejemplo
</h2>

El [directorio de ejemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) contiene flujos de trabajo listos para usar para diferentes escenarios.

Los ejemplos en esta página muestran autenticación de clave API. Si se autentica con una suscripción de Claude, reemplace la línea `anthropic_api_key` en cualquier ejemplo con `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Responder a menciones @claude
</h3>

Este flujo de trabajo ejecuta la GitHub Action de Claude Code en modo interactivo, por lo que Claude responde cada vez que alguien menciona `@claude` en un comentario de problema o PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Las partes de este flujo de trabajo que no son código estándar:

* `id-token: write`: requerido para la autenticación predeterminada de GitHub App de la GitHub Action de Claude Code
* `actions: read`: permite que Claude lea resultados de CI en PR
* `actions/checkout`: le da a Claude una copia local del repositorio para trabajar
* `if`: evita que los ejecutores se inicien en comentarios que no mencionen `@claude`. La GitHub Action de Claude Code también verifica la frase de disparo antes de responder

Una vez que el flujo de trabajo está en su lugar, mencione `@claude` en cualquier comentario de problema o PR con una solicitud:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude responde en un comentario en el mismo problema o PR y lo actualiza mientras trabaja.

<h3 id="run-a-skill">
  Ejecutar una skill
</h3>

La entrada `prompt` acepta una invocación de [skill](/docs/es/skills) así como texto sin formato:

* Para una skill en el directorio `.claude/skills/` de su repositorio, ejecute `actions/checkout` antes del paso `anthropics/claude-code-action` para que los archivos de skill estén disponibles en el ejecutor, luego pase `/skill-name` como `prompt`.
* Para una skill empaquetada en un [plugin](/docs/es/plugins/overview), instale el plugin con las entradas `plugin_marketplaces` y `plugins`, luego pase el `/plugin-name:skill-name` con espacio de nombres como `prompt`. La entrada `plugins` toma `plugin-name@marketplace-name`, donde el nombre del marketplace viene del manifiesto propio del marketplace en lugar de su URL de repositorio.

El siguiente flujo de trabajo instala el plugin `code-review` y ejecuta su skill cuando se abre, actualiza, reabre o marca una solicitud de extracción como lista para revisión. Ejecuta el mismo plugin que el flujo de trabajo de revisión de la configuración rápida. Use un flujo de trabajo como este cuando desee controlar el mensaje, el modelo y los disparadores usted mismo. Para revisiones automáticas sin mantener un archivo de flujo de trabajo, consulte [Code Review](/docs/es/code-review). En repositorios públicos, GitHub retiene secretos de ejecuciones desencadenadas por solicitudes de extracción de bifurcación, por lo que la revisión se ejecuta solo en solicitudes de extracción de ramas en el mismo repositorio.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Dos líneas en este flujo de trabajo controlan dónde va la revisión:

* **`--comment`**: Claude publica su revisión en la solicitud de extracción, como un comentario en línea en cada problema que encuentra o como un comentario de resumen cuando no encuentra ninguno. Sin él, Claude no publica nada, y usted lee los hallazgos en el registro de ejecución del flujo de trabajo.
* **`claude_args`**: mantenga esta línea aunque la propia `allowed-tools` frontmatter de la skill nombre la misma herramienta, porque la GitHub Action de Claude Code inicia el servidor MCP que publica comentarios en línea solo cuando `--allowedTools` en `claude_args` lo nombra.

Claude omite solicitudes de extracción en borrador y cerradas, solicitudes de extracción que juzga que no necesitan revisión, como las automatizadas o triviales, y solicitudes de extracción que ya tienen un comentario de Claude.

<h3 id="run-on-a-schedule">
  Ejecutar en un cronograma
</h3>

Con una entrada `prompt`, la GitHub Action de Claude Code se ejecuta en modo de automatización en cualquier evento de GitHub, incluido un cronograma cron. Para un mensaje de texto sin formato, Claude no tiene acceso a shell o API de GitHub hasta que otorgue las herramientas que el mensaje necesita, con `--allowedTools` en `claude_args` o una [regla `permissions.allow`](/docs/es/permissions#permission-rule-syntax) en la entrada `settings`. Si invoca una skill en su lugar, Claude puede usar las herramientas que su [frontmatter `allowed-tools`](/docs/es/skills#pre-approve-tools-for-a-skill) otorga. GitHub ejecuta flujos de trabajo programados solo desde la rama predeterminada y, en repositorios públicos, desactiva el cronograma después de 60 días sin actividad del repositorio.

Este flujo de trabajo genera un informe en el registro de ejecución del flujo de trabajo a las 09:00 UTC cada día. Su línea `claude_args` [pasa argumentos de CLI](#pass-cli-arguments) que seleccionan el modelo y permiten dos herramientas MCP de GitHub. Claude lee confirmaciones y problemas a través de la API de GitHub con esas herramientas, por lo que puede omitir el paso de checkout:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Mejores prácticas
</h2>

<h3 id="define-project-standards-in-claude-md">
  Defina estándares de proyecto en CLAUDE.md
</h3>

Cree un archivo `CLAUDE.md` en la raíz de su repositorio para definir directrices de estilo de código, criterios de revisión, reglas específicas del proyecto y patrones preferidos. Claude sigue estas directrices al crear PR y responder a solicitudes. Consulte la [documentación de memoria](/docs/es/memory) para detalles.

<h3 id="protect-your-credentials">
  Proteja sus credenciales
</h3>

<Warning>
  Nunca confirme claves API u tokens OAuth directamente en su repositorio. Siempre almacénelos como GitHub Secrets y haga referencia a ellos en flujos de trabajo, por ejemplo `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Otorgue al flujo de trabajo solo los permisos que necesita y revise los cambios de Claude antes de fusionar.

Para una guía de seguridad completa que incluya permisos y autenticación, consulte la [documentación de seguridad de Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Gestionar costos
</h3>

Cada ejecución consume dos tipos de recursos:

* **Minutos de GitHub Actions**: la GitHub Action de Claude Code se ejecuta en ejecutores alojados en GitHub, que consumen sus minutos de GitHub Actions. Consulte la [documentación de facturación de GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) para precios y límites de minutos.
* **Tokens de API**: cada interacción consume tokens según la longitud de mensajes y respuestas, complejidad de tareas y tamaño de la base de código. Consulte la [página de precios de Claude](https://claude.com/platform/api) para tasas de tokens actuales. Si se autentica con un token OAuth, las ejecuciones usan su suscripción de Claude en lugar de facturación de API.

Puede reducir ambos tipos de costo dándole a Claude un contexto más claro y limitando cuánto trabajo puede hacer cada ejecución:

* Escriba solicitudes específicas de `@claude` para que Claude necesite menos turnos para terminar
* Use plantillas de problemas para proporcionar contexto por adelantado
* Mantenga su `CLAUDE.md` conciso, ya que Claude lo lee en cada ejecución
* Establezca `--max-turns` en `claude_args` para limitar iteraciones
* Establezca tiempos de espera a nivel de flujo de trabajo para evitar trabajos descontrolados
* Use los controles de concurrencia de GitHub para limitar ejecuciones paralelas

Para seguimiento de uso en toda su organización, consulte el [panel de análisis](/docs/es/analytics) y [monitoreo](/docs/es/monitoring-usage). Para cómo se mide y factura el uso, consulte [costos](/docs/es/costs).

<h2 id="use-a-cloud-provider">
  Usar un proveedor en la nube
</h2>

Por defecto, la GitHub Action de Claude Code llama a la API de Claude directamente con su clave API u token OAuth. Para enrutar la inferencia a través de su propia cuenta en la nube en su lugar, establezca la entrada para su proveedor y siga [Usar Claude Code GitHub Actions con proveedores en la nube](/docs/es/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Con los tres proveedores, se autentica a través de federación de identidad OIDC en lugar de una clave API de Claude, por lo que no almacena credenciales de nube estáticas en su repositorio.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude no responde a comandos @claude
</h3>

* Verifique que la GitHub App esté instalada en el repositorio
* Compruebe que los flujos de trabajo estén habilitados para el repositorio
* Asegúrese de que su clave API u token OAuth esté configurado en secretos del repositorio
* Confirme que el comentario contiene `@claude` como una palabra completa, no `/claude` o `@claude-bot`
* Confirme que el usuario que comenta tiene acceso de escritura al repositorio. Consulte [Quién puede desencadenar ejecuciones](#who-can-trigger-runs) para las excepciones

<h3 id="ci-not-running-on-claude’s-commits">
  CI no se ejecuta en las confirmaciones de Claude
</h3>

* GitHub no desencadena flujos de trabajo en confirmaciones realizadas con el `GITHUB_TOKEN` predeterminado. Si pasa `github_token: ${{ secrets.GITHUB_TOKEN }}` a la GitHub Action de Claude Code, elimínelo para que se autentique como la GitHub App de Claude, o pase un token de aplicación personalizado en su lugar
* Compruebe que los disparadores del flujo de trabajo de CI incluyan los eventos que producen los insertos de Claude, como `push` o `pull_request`

<h3 id="authentication-errors">
  Errores de autenticación
</h3>

* Confirme que la clave API u token OAuth sea válido probándolo localmente con `claude` antes de depurar el flujo de trabajo
* Para Bedrock, Agent Platform y Foundry, consulte la [sección de solución de problemas](/docs/es/github-actions-cloud-providers#troubleshooting) de la página del proveedor en la nube

Para más soluciones, consulte las [Preguntas frecuentes](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) de la GitHub Action de Claude Code.

<h2 id="advanced-configuration">
  Configuración avanzada
</h2>

<h3 id="action-parameters">
  Parámetros de acción
</h3>

Estas son las entradas más comúnmente usadas. Cada una se asigna a una clave `with:` en el paso `anthropics/claude-code-action`.

| Parámetro                 | Descripción                                                                                                                                                                                        | Requerido                                                                                                                                                                                       |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Instrucciones para Claude, como texto sin formato o una invocación de [skill](/docs/es/skills). Cuando se omite, Claude responde a la [frase de disparo](#interactive-and-automation-modes) en su lugar | No                                                                                                                                                                                              |
| `claude_args`             | Argumentos de CLI pasados a Claude Code                                                                                                                                                            | No                                                                                                                                                                                              |
| `anthropic_api_key`       | Clave API de Claude                                                                                                                                                                                | Para la API de Claude, a menos que use `claude_code_oauth_token` o [federación de identidad de carga de trabajo](#set-up-for-an-organization). No se usa para Bedrock, Agent Platform o Foundry |
| `claude_code_oauth_token` | Token OAuth para autenticarse con una suscripción de Claude, generado con `claude setup-token`                                                                                                     | No                                                                                                                                                                                              |
| `github_token`            | Token para operaciones de GitHub. Cuando se omite, la GitHub Action de Claude Code se autentica como la GitHub App de Claude                                                                       | No                                                                                                                                                                                              |
| `plugin_marketplaces`     | Lista separada por saltos de línea de URLs de Git de marketplace de plugins                                                                                                                        | No                                                                                                                                                                                              |
| `plugins`                 | Lista separada por saltos de línea de nombres de plugins a instalar antes de la ejecución                                                                                                          | No                                                                                                                                                                                              |
| `settings`                | Configuración de Claude Code, como una cadena JSON o una ruta a un archivo JSON de configuración                                                                                                   | No                                                                                                                                                                                              |
| `trigger_phrase`          | Frase de disparo a la que Claude responde. Predeterminado: `@claude`                                                                                                                               | No                                                                                                                                                                                              |
| `use_bedrock`             | Usar Amazon Bedrock en lugar de la API de Claude                                                                                                                                                   | No                                                                                                                                                                                              |
| `use_vertex`              | Usar Google Cloud's Agent Platform en lugar de la API de Claude                                                                                                                                    | No                                                                                                                                                                                              |
| `use_foundry`             | Usar Microsoft Foundry en lugar de la API de Claude                                                                                                                                                | No                                                                                                                                                                                              |

Para la lista de entrada completa, consulte la [referencia de configuración](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) de la GitHub Action de Claude Code.

<h3 id="pass-cli-arguments">
  Pasar argumentos de CLI
</h3>

El parámetro `claude_args` acepta cualquier [argumento de CLI de Claude Code](/docs/es/cli-reference):

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argumentos comunes:

* `--max-turns`: limitar el número de turnos de conversación
* `--model`: modelo a usar, por ejemplo `claude-sonnet-5`. Sin este argumento, la GitHub Action de Claude Code usa el [modelo predeterminado](/docs/es/model-config) de Claude Code
* `--mcp-config`: ruta a [configuración de MCP](/docs/es/mcp)
* `--allowedTools`: lista separada por comas de herramientas permitidas. El alias `--allowed-tools` también funciona
* `--debug`: habilitar salida de depuración

<h2 id="upgrade-from-beta">
  Actualizar desde beta
</h2>

Si sus flujos de trabajo aún hacen referencia a `anthropics/claude-code-action@beta`, actualícelos a v1:

1. Cambie `@beta` a `@v1` en la línea `uses`
2. Elimine la entrada `mode`, ya que la GitHub Action de Claude Code ahora [detecta el modo automáticamente](#interactive-and-automation-modes)
3. Reemplace `direct_prompt` con `prompt`
4. Mueva opciones de CLI como `max_turns` y `model` a `claude_args`. `custom_instructions` no tiene una bandera con el mismo nombre y se convierte en `--append-system-prompt`

Para la asignación de entrada completa y ejemplos antes y después, consulte la [guía de migración](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Usar Claude Code GitHub Actions con proveedores en la nube](/docs/es/github-actions-cloud-providers): enrutar la inferencia a través de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry
* [Referencia de configuración](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): la lista completa de entradas de acción
* [Directorio de ejemplos](https://github.com/anthropics/claude-code-action/tree/main/examples): flujos de trabajo listos para usar para más escenarios
* [Code Review](/docs/es/code-review): revisión automática de solicitudes de extracción sin mantener un archivo de flujo de trabajo
