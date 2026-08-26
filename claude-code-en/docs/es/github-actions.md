> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Aprenda a integrar Claude Code en su flujo de trabajo de desarrollo con Claude Code GitHub Actions

Claude Code GitHub Actions trae automatización impulsada por IA a su flujo de trabajo de GitHub. Con una simple mención `@claude` en cualquier PR o problema, Claude puede analizar su código, crear solicitudes de extracción, implementar características y corregir errores, todo mientras sigue los estándares de su proyecto. Para revisiones automáticas publicadas en cada PR sin un disparador, consulte [GitHub Code Review](/docs/es/code-review).

<Note>
  Claude Code GitHub Actions se construye sobre el [Claude Agent SDK](/docs/es/agent-sdk/overview), que permite la integración programática de Claude Code en sus aplicaciones. Puede usar el SDK para crear flujos de trabajo de automatización personalizados más allá de GitHub Actions.
</Note>

<h2 id="why-use-claude-code-github-actions">
  ¿Por qué usar Claude Code GitHub Actions?
</h2>

* **Creación instantánea de PR**: Describa lo que necesita y Claude crea un PR completo con todos los cambios necesarios
* **Implementación de código automatizada**: Convierta problemas en código funcional con un único comando
* **Sigue sus estándares**: Claude respeta sus directrices `CLAUDE.md` y patrones de código existentes
* **Configuración simple**: Comience en minutos con nuestro instalador y clave API
* **Seguro por defecto**: Su código permanece en los ejecutores de Github

<h2 id="what-can-claude-do">
  ¿Qué puede hacer Claude?
</h2>

Claude Code proporciona una poderosa GitHub Action que transforma la forma en que trabaja con código:

<h3 id="claude-code-action">
  Claude Code Action
</h3>

Esta GitHub Action le permite ejecutar Claude Code dentro de sus flujos de trabajo de GitHub Actions. Puede usar esto para crear cualquier flujo de trabajo personalizado sobre Claude Code.

[Ver repositorio →](https://github.com/anthropics/claude-code-action)

<h2 id="setup">
  Configuración
</h2>

<h2 id="quick-setup">
  Configuración rápida
</h2>

Ejecute `/install-github-app` en la terminal de Claude Code para configurar la integración de forma interactiva. El comando instala la aplicación Claude GitHub en su repositorio y luego lo guía a través de la adición de los flujos de trabajo de GitHub Actions y el secreto de la clave API.

Después de que se instale la aplicación de GitHub, el comando pregunta si desea continuar con la configuración de GitHub Actions. En Claude Code v2.1.187 y versiones posteriores, puede elegir **Omitir por ahora** para detener solo con la aplicación instalada y volver a los pasos de flujo de trabajo y secreto ejecutando `/install-github-app` nuevamente. Las versiones anteriores proceden directamente a la selección de flujo de trabajo.

<Note>
  * Debe ser administrador del repositorio para instalar la aplicación de GitHub y agregar secretos
  * La aplicación de GitHub solicitará permisos de lectura y escritura para Contenidos, Problemas y Solicitudes de extracción
  * Este método de inicio rápido solo está disponible para usuarios directos de Claude API. Si está usando Amazon Bedrock o Google Cloud's Agent Platform, consulte la sección [Usar con Amazon Bedrock y Google Cloud](#using-with-amazon-bedrock-and-google-cloud).
</Note>

<h2 id="manual-setup">
  Configuración manual
</h2>

Si el comando `/install-github-app` falla o prefiere la configuración manual, siga estas instrucciones de configuración manual:

1. **Instale la aplicación de GitHub de Claude** en su repositorio: [https://github.com/apps/claude](https://github.com/apps/claude)

   La aplicación de GitHub de Claude requiere los siguientes permisos de repositorio:

   * **Contenidos**: Lectura y escritura (para modificar archivos del repositorio)
   * **Problemas**: Lectura y escritura (para responder a problemas)
   * **Solicitudes de extracción**: Lectura y escritura (para crear PR e insertar cambios)

   Para más detalles sobre seguridad y permisos, consulte la [documentación de seguridad](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).
2. **Agregue ANTHROPIC\_API\_KEY** a sus secretos del repositorio ([Aprenda cómo usar secretos en GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **Copie el archivo de flujo de trabajo** de [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) en el directorio `.github/workflows/` de su repositorio

<Tip>
  Después de completar la configuración rápida o manual, pruebe la acción etiquetando `@claude` en un comentario de problema o PR.
</Tip>

<h2 id="upgrading-from-beta">
  Actualización desde Beta
</h2>

<Warning>
  Claude Code GitHub Actions v1.0 introduce cambios importantes que requieren actualizar sus archivos de flujo de trabajo para actualizar a v1.0 desde la versión beta.
</Warning>

Si actualmente está usando la versión beta de Claude Code GitHub Actions, le recomendamos que actualice sus flujos de trabajo para usar la versión GA. La nueva versión simplifica la configuración mientras agrega características poderosas como la detección automática de modo.

<h3 id="essential-changes">
  Cambios esenciales
</h3>

Todos los usuarios de beta deben hacer estos cambios en sus archivos de flujo de trabajo para actualizar:

1. **Actualice la versión de la acción**: Cambie `@beta` a `@v1`
2. **Elimine la configuración de modo**: Elimine `mode: "tag"` o `mode: "agent"` (ahora se detecta automáticamente)
3. **Actualice las entradas de solicitud**: Reemplace `direct_prompt` con `prompt`
4. **Mueva opciones de CLI**: Convierta `max_turns`, `model`, `custom_instructions`, etc. a `claude_args`

<h3 id="breaking-changes-reference">
  Referencia de cambios importantes
</h3>

| Entrada Beta antigua  | Nueva entrada v1.0                         |
| --------------------- | ------------------------------------------ |
| `mode`                | *(Eliminado - se detecta automáticamente)* |
| `direct_prompt`       | `prompt`                                   |
| `override_prompt`     | `prompt` con variables de GitHub           |
| `custom_instructions` | `claude_args: --append-system-prompt`      |
| `max_turns`           | `claude_args: --max-turns`                 |
| `model`               | `claude_args: --model`                     |
| `allowed_tools`       | `claude_args: --allowedTools`              |
| `disallowed_tools`    | `claude_args: --disallowedTools`           |
| `claude_env`          | `settings` formato JSON                    |

<h3 id="before-and-after-example">
  Ejemplo antes y después
</h3>

**Versión beta:**

```yaml theme={null}
- uses: anthropics/claude-code-action@beta
  with:
    mode: "tag"
    direct_prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    custom_instructions: "Follow our coding standards"
    max_turns: "10"
    model: "claude-sonnet-5"
```

**Versión GA (v1.0):**

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    claude_args: |
      --append-system-prompt "Follow our coding standards"
      --max-turns 10
      --model claude-sonnet-5
```

<Tip>
  La acción ahora detecta automáticamente si ejecutar en modo interactivo (responde a menciones `@claude`) o modo de automatización (se ejecuta inmediatamente con un solicitud) según su configuración.
</Tip>

<h2 id="example-use-cases">
  Casos de uso de ejemplo
</h2>

Claude Code GitHub Actions puede ayudarle con una variedad de tareas. El [directorio de ejemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) contiene flujos de trabajo listos para usar para diferentes escenarios.

<h3 id="basic-workflow">
  Flujo de trabajo básico
</h3>

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          # Responds to @claude mentions in comments
```

<h3 id="using-skills">
  Usar skills
</h3>

La entrada `prompt` acepta una [invocación de skill](/docs/es/skills) así como texto sin formato:

* Para un skill en el directorio `.claude/skills/` de su repositorio, ejecute `actions/checkout` antes del paso de acción y pase `/skill-name`.
* Para un skill empaquetado en un plugin, instale el plugin con las entradas `plugin_marketplaces` y `plugins` y pase el `/plugin-name:skill-name` con espacio de nombres.

El siguiente flujo de trabajo instala el plugin `code-review` y ejecuta su skill en cada solicitud de extracción nueva o actualizada:

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
```

<h3 id="custom-automation-with-prompts">
  Automatización personalizada con solicitudes
</h3>

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: "--model opus"
```

<h3 id="common-use-cases">
  Casos de uso comunes
</h3>

En comentarios de problema o PR:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude analizará automáticamente el contexto y responderá apropiadamente.

<h2 id="best-practices">
  Mejores prácticas
</h2>

<h3 id="claude-md-configuration">
  Configuración de CLAUDE.md
</h3>

Cree un archivo `CLAUDE.md` en la raíz de su repositorio para definir directrices de estilo de código, criterios de revisión, reglas específicas del proyecto y patrones preferidos. Este archivo guía la comprensión de Claude de los estándares de su proyecto.

<h3 id="security-considerations">
  Consideraciones de seguridad
</h3>

<Warning>Nunca confirme claves API directamente en su repositorio.</Warning>

Para una guía de seguridad completa que incluya permisos, autenticación y mejores prácticas, consulte la [documentación de seguridad de Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

Siempre use GitHub Secrets para claves API:

* Agregue su clave API como un secreto del repositorio llamado `ANTHROPIC_API_KEY`
* Haga referencia a él en flujos de trabajo: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* Limite los permisos de acción solo a lo necesario
* Revise las sugerencias de Claude antes de fusionar

Siempre use GitHub Secrets (por ejemplo, `${{ secrets.ANTHROPIC_API_KEY }}`) en lugar de codificar claves API directamente en sus archivos de flujo de trabajo.

<h3 id="optimizing-performance">
  Optimización del rendimiento
</h3>

Use plantillas de problemas para proporcionar contexto, mantenga su `CLAUDE.md` conciso y enfocado, y configure tiempos de espera apropiados para sus flujos de trabajo.

<h3 id="ci-costs">
  Costos de CI
</h3>

Al usar Claude Code GitHub Actions, tenga en cuenta los costos asociados:

**Costos de GitHub Actions:**

* Claude Code se ejecuta en ejecutores alojados en GitHub, que consumen sus minutos de GitHub Actions
* Consulte la [documentación de facturación de GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) para obtener detalles de precios y límites de minutos

**Costos de API:**

* Cada interacción de Claude consume tokens de API según la longitud de solicitudes y respuestas
* El uso de tokens varía según la complejidad de la tarea y el tamaño de la base de código
* Consulte la [página de precios de Claude](https://claude.com/platform/api) para obtener las tasas de tokens actuales

**Consejos de optimización de costos:**

* Use comandos específicos `@claude` para reducir llamadas API innecesarias
* Configure `--max-turns` apropiado en `claude_args` para evitar iteraciones excesivas
* Establezca tiempos de espera a nivel de flujo de trabajo para evitar trabajos descontrolados
* Considere usar controles de concurrencia de GitHub para limitar ejecuciones paralelas

<h2 id="configuration-examples">
  Ejemplos de configuración
</h2>

Claude Code Action v1 simplifica la configuración con parámetros unificados:

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

Características clave:

* **Interfaz de solicitud unificada** - Use `prompt` para todas las instrucciones
* **Skills** - Invoque [skills](/docs/es/skills) instalados directamente desde la solicitud
* **Paso de CLI** - Cualquier argumento de CLI de Claude Code a través de `claude_args`
* **Disparadores flexibles** - Funciona con cualquier evento de GitHub

Visite el [directorio de ejemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) para archivos de flujo de trabajo completos.

<Tip>
  Al responder a comentarios de problema o PR, Claude responde automáticamente a menciones @claude. Para otros eventos, use el parámetro `prompt` para proporcionar instrucciones.
</Tip>

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Usar con Amazon Bedrock y Google Cloud
</h2>

Para entornos empresariales, puede usar Claude Code GitHub Actions con su propia infraestructura en la nube. Este enfoque le da control sobre la residencia de datos y la facturación mientras mantiene la misma funcionalidad.

<h3 id="prerequisites">
  Requisitos previos
</h3>

Antes de configurar Claude Code GitHub Actions con proveedores en la nube, necesita:

<h4 id="for-google-cloud’s-agent-platform">
  Para Google Cloud's Agent Platform:
</h4>

1. Un proyecto de Google Cloud con Google Cloud's Agent Platform habilitado
2. Federación de identidad de carga de trabajo configurada para GitHub Actions
3. Una cuenta de servicio con los permisos requeridos
4. Una aplicación de GitHub (recomendado) o use el GITHUB\_TOKEN predeterminado

<h4 id="for-amazon-bedrock">
  Para Amazon Bedrock:
</h4>

1. Una cuenta de AWS con Amazon Bedrock habilitado
2. Proveedor de identidad OIDC de GitHub configurado en AWS
3. Un rol de IAM con permisos de Amazon Bedrock
4. Una aplicación de GitHub (recomendado) o use el GITHUB\_TOKEN predeterminado

<Steps>
  <Step title="Crear una aplicación de GitHub personalizada (Recomendado para proveedores de terceros)">
    Para el mejor control y seguridad al usar proveedores de terceros como Google Cloud's Agent Platform o Amazon Bedrock, le recomendamos crear su propia aplicación de GitHub:

    1. Vaya a [https://github.com/settings/apps/new](https://github.com/settings/apps/new)
    2. Complete la información básica:
       * **Nombre de la aplicación de GitHub**: Elija un nombre único (por ejemplo, "YourOrg Claude Assistant")
       * **URL de inicio**: El sitio web de su organización o la URL del repositorio
    3. Configure los ajustes de la aplicación:
       * **Webhooks**: Desmarque "Activo" (no es necesario para esta integración)
    4. Establezca los permisos requeridos:
       * **Permisos del repositorio**:
         * Contenidos: Lectura y escritura
         * Problemas: Lectura y escritura
         * Solicitudes de extracción: Lectura y escritura
    5. Haga clic en "Crear aplicación de GitHub"
    6. Después de la creación, haga clic en "Generar una clave privada" y guarde el archivo `.pem` descargado
    7. Anote su ID de aplicación en la página de configuración de la aplicación
    8. Instale la aplicación en su repositorio:
       * Desde la página de configuración de su aplicación, haga clic en "Instalar aplicación" en la barra lateral izquierda
       * Seleccione su cuenta u organización
       * Elija "Solo repositorios seleccionados" y seleccione el repositorio específico
       * Haga clic en "Instalar"
    9. Agregue la clave privada como un secreto a su repositorio:
       * Vaya a Configuración de su repositorio → Secretos y variables → Acciones
       * Cree un nuevo secreto llamado `APP_PRIVATE_KEY` con el contenido del archivo `.pem`
    10. Agregue el ID de la aplicación como un secreto:

    * Cree un nuevo secreto llamado `APP_ID` con el ID de su aplicación de GitHub

    <Note>
      Esta aplicación se usará con la acción [actions/create-github-app-token](https://github.com/actions/create-github-app-token) para generar tokens de autenticación en sus flujos de trabajo.
    </Note>

    **Alternativa para Claude API o si no desea configurar su propia aplicación de Github**: Use la aplicación oficial de Anthropic:

    1. Instale desde: [https://github.com/apps/claude](https://github.com/apps/claude)
    2. No se requiere configuración adicional para autenticación
  </Step>

  <Step title="Configurar autenticación del proveedor en la nube">
    Elija su proveedor en la nube y configure autenticación segura:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock">
        **Configure AWS para permitir que GitHub Actions se autentique de forma segura sin almacenar credenciales.**

        > **Nota de seguridad**: Use configuraciones específicas del repositorio y otorgue solo los permisos mínimos requeridos.

        **Configuración requerida**:

        1. **Habilitar Amazon Bedrock**:
           * Solicite acceso a modelos de Claude en Amazon Bedrock
           * Para modelos entre regiones, solicite acceso en todas las regiones requeridas

        2. **Configurar proveedor de identidad OIDC de GitHub**:
           * URL del proveedor: `https://token.actions.githubusercontent.com`
           * Audiencia: `sts.amazonaws.com`

        3. **Crear rol de IAM para GitHub Actions**:
           * Tipo de entidad de confianza: Identidad web
           * Proveedor de identidad: `token.actions.githubusercontent.com`
           * Permisos: política `AmazonBedrockFullAccess`
           * Configurar política de confianza para su repositorio específico

        **Valores requeridos**:

        Después de la configuración, necesitará:

        * **AWS\_ROLE\_TO\_ASSUME**: El ARN del rol de IAM que creó

        <Tip>
          OIDC es más seguro que usar claves de acceso estáticas de AWS porque las credenciales son temporales y se rotan automáticamente.
        </Tip>

        Consulte la [documentación de AWS](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) para obtener instrucciones detalladas de configuración de OIDC.
      </Accordion>

      <Accordion title="Google Cloud's Agent Platform">
        **Configure Google Cloud para permitir que GitHub Actions se autentique de forma segura sin almacenar credenciales.**

        > **Nota de seguridad**: Use configuraciones específicas del repositorio y otorgue solo los permisos mínimos requeridos.

        **Configuración requerida**:

        1. **Habilitar APIs** en su proyecto de Google Cloud:
           * API de credenciales de IAM
           * API de servicio de token de seguridad (STS)
           * API de Google Cloud's Agent Platform

        2. **Crear recursos de Federación de identidad de carga de trabajo**:
           * Crear un grupo de identidad de carga de trabajo
           * Agregar un proveedor OIDC de GitHub con:
             * Emisor: `https://token.actions.githubusercontent.com`
             * Asignaciones de atributos para repositorio y propietario
             * **Recomendación de seguridad**: Use condiciones de atributo específicas del repositorio

        3. **Crear una cuenta de servicio**:
           * Otorgue solo el rol `Vertex AI User`
           * **Recomendación de seguridad**: Cree una cuenta de servicio dedicada por repositorio

        4. **Configurar enlaces de IAM**:
           * Permitir que el grupo de identidad de carga de trabajo suplante la cuenta de servicio
           * **Recomendación de seguridad**: Use conjuntos de principios específicos del repositorio

        **Valores requeridos**:

        Después de la configuración, necesitará:

        * **GCP\_WORKLOAD\_IDENTITY\_PROVIDER**: El nombre completo del recurso del proveedor
        * **GCP\_SERVICE\_ACCOUNT**: La dirección de correo electrónico de la cuenta de servicio

        <Tip>
          Workload Identity Federation elimina la necesidad de claves de cuenta de servicio descargables, mejorando la seguridad.
        </Tip>

        Para obtener instrucciones de configuración detalladas, consulte la [documentación de Federación de identidad de carga de trabajo de Google Cloud](https://cloud.google.com/iam/docs/workload-identity-federation).
      </Accordion>
    </AccordionGroup>
  </Step>

  <Step title="Agregar secretos requeridos">
    Agregue los siguientes secretos a su repositorio (Configuración → Secretos y variables → Acciones):

    #### Para Claude API (Directo):

    1. **Para autenticación de API**:
       * `ANTHROPIC_API_KEY`: Su clave de API de Claude de [console.anthropic.com](https://console.anthropic.com)

    2. **Para aplicación de GitHub (si usa su propia aplicación)**:
       * `APP_ID`: El ID de su aplicación de GitHub
       * `APP_PRIVATE_KEY`: El contenido de la clave privada (.pem)

    #### Para Google Cloud's Agent Platform

    1. **Para autenticación de GCP**:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`
       * `GCP_SERVICE_ACCOUNT`

    2. **Para aplicación de GitHub (si usa su propia aplicación)**:
       * `APP_ID`: El ID de su aplicación de GitHub
       * `APP_PRIVATE_KEY`: El contenido de la clave privada (.pem)

    #### Para Amazon Bedrock

    1. **Para autenticación de AWS**:
       * `AWS_ROLE_TO_ASSUME`

    2. **Para aplicación de GitHub (si usa su propia aplicación)**:
       * `APP_ID`: El ID de su aplicación de GitHub
       * `APP_PRIVATE_KEY`: El contenido de la clave privada (.pem)
  </Step>

  <Step title="Crear archivos de flujo de trabajo">
    Cree archivos de flujo de trabajo de GitHub Actions que se integren con su proveedor en la nube. Los ejemplos a continuación muestran configuraciones completas tanto para Amazon Bedrock como para Google Cloud's Agent Platform:

    <AccordionGroup>
      <Accordion title="Flujo de trabajo de Amazon Bedrock">
        **Requisitos previos:**

        * Acceso a Amazon Bedrock habilitado con permisos de modelo de Claude
        * GitHub configurado como proveedor de identidad OIDC en AWS
        * Rol de IAM con permisos de Amazon Bedrock que confía en GitHub Actions

        **Secretos de GitHub requeridos:**

        | Nombre del secreto   | Descripción                                                          |
        | -------------------- | -------------------------------------------------------------------- |
        | `AWS_ROLE_TO_ASSUME` | ARN del rol de IAM para acceso a Amazon Bedrock                      |
        | `APP_ID`             | Su ID de aplicación de GitHub (de la configuración de la aplicación) |
        | `APP_PRIVATE_KEY`    | La clave privada que generó para su aplicación de GitHub             |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            env:
              AWS_REGION: us-west-2
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6 --max-turns 10'
        ```

        <Tip>
          El formato de ID de modelo para Amazon Bedrock incluye un prefijo de región (por ejemplo, `us.anthropic.claude-sonnet-4-6`).
        </Tip>
      </Accordion>

      <Accordion title="Flujo de trabajo de Google Cloud's Agent Platform">
        **Requisitos previos:**

        * API de Google Cloud's Agent Platform habilitada en su proyecto de GCP
        * Federación de identidad de carga de trabajo configurada para GitHub
        * Cuenta de servicio con permisos de Google Cloud's Agent Platform

        **Secretos de GitHub requeridos:**

        | Nombre del secreto               | Descripción                                                                            |
        | -------------------------------- | -------------------------------------------------------------------------------------- |
        | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Nombre del recurso del proveedor de identidad de carga de trabajo                      |
        | `GCP_SERVICE_ACCOUNT`            | Correo electrónico de la cuenta de servicio con acceso a Google Cloud's Agent Platform |
        | `APP_ID`                         | Su ID de aplicación de GitHub (de la configuración de la aplicación)                   |
        | `APP_PRIVATE_KEY`                | La clave privada que generó para su aplicación de GitHub                               |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  trigger_phrase: "@claude"
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-4-5@20250929 --max-turns 10'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
                  VERTEX_REGION_CLAUDE_4_5_SONNET: us-east5
        ```

        <Tip>
          El ID del proyecto se recupera automáticamente del paso de autenticación de Google Cloud, por lo que no necesita codificarlo.
        </Tip>
      </Accordion>
    </AccordionGroup>
  </Step>
</Steps>

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude no responde a comandos @claude
</h3>

Verifique que la aplicación de GitHub esté instalada correctamente, compruebe que los flujos de trabajo estén habilitados, asegúrese de que la clave API esté configurada en los secretos del repositorio y confirme que el comentario contenga `@claude` (no `/claude`).

<h3 id="ci-not-running-on-claude’s-commits">
  CI no se ejecuta en los commits de Claude
</h3>

Asegúrese de estar usando la aplicación de GitHub o una aplicación personalizada (no el usuario de Acciones), verifique que los disparadores de flujo de trabajo incluyan los eventos necesarios y confirme que los permisos de la aplicación incluyan disparadores de CI.

<h3 id="authentication-errors">
  Errores de autenticación
</h3>

Confirme que la clave API sea válida y tenga permisos suficientes. Para Amazon Bedrock o Google Cloud's Agent Platform, verifique la configuración de credenciales y asegúrese de que los secretos tengan los nombres correctos en los flujos de trabajo.

<h2 id="advanced-configuration">
  Configuración avanzada
</h2>

<h3 id="action-parameters">
  Parámetros de acción
</h3>

Claude Code Action v1 utiliza una configuración simplificada:

| Parámetro             | Descripción                                                                               | Requerido |
| --------------------- | ----------------------------------------------------------------------------------------- | --------- |
| `prompt`              | Instrucciones para Claude (texto sin formato o un nombre de [skill](/docs/es/skills))          | No\*      |
| `claude_args`         | Argumentos de CLI pasados a Claude Code                                                   | No        |
| `plugin_marketplaces` | Lista separada por saltos de línea de URLs de Git de marketplace de plugins               | No        |
| `plugins`             | Lista separada por saltos de línea de nombres de plugins a instalar antes de la ejecución | No        |
| `anthropic_api_key`   | Clave API de Claude                                                                       | Sí\*\*    |
| `github_token`        | Token de GitHub para acceso a API                                                         | No        |
| `trigger_phrase`      | Frase de disparo personalizada (predeterminado: "@claude")                                | No        |
| `use_bedrock`         | Usar Amazon Bedrock en lugar de Claude API                                                | No        |
| `use_vertex`          | Usar Google Cloud's Agent Platform en lugar de Claude API                                 | No        |

\*Prompt es opcional - cuando se omite para comentarios de problema/PR, Claude responde a la frase de disparo\
\*\*Requerido para Claude API directo, no para Amazon Bedrock o Google Cloud's Agent Platform

<h4 id="pass-cli-arguments">
  Pasar argumentos de CLI
</h4>

El parámetro `claude_args` acepta cualquier argumento de CLI de Claude Code:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argumentos comunes:

* `--max-turns`: Máximo de turnos de conversación (predeterminado: 10)
* `--model`: Modelo a usar (por ejemplo, `claude-sonnet-5`)
* `--mcp-config`: Ruta a la configuración de MCP
* `--allowedTools`: Lista separada por comas de herramientas permitidas. El alias `--allowed-tools` también funciona.
* `--debug`: Habilitar salida de depuración

<h3 id="alternative-integration-methods">
  Métodos de integración alternativos
</h3>

Aunque el comando `/install-github-app` es el enfoque recomendado, también puede:

* **Aplicación de GitHub personalizada**: Para organizaciones que necesitan nombres de usuario personalizados o flujos de autenticación personalizados. Cree su propia aplicación de GitHub con permisos requeridos (contenidos, problemas, solicitudes de extracción) y use la acción actions/create-github-app-token para generar tokens en sus flujos de trabajo.
* **GitHub Actions manual**: Configuración de flujo de trabajo directo para máxima flexibilidad
* **Configuración de MCP**: Carga dinámica de servidores del Protocolo de contexto del modelo

Consulte la [documentación de Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs) para obtener guías detalladas sobre autenticación, seguridad y configuración avanzada.

<h3 id="customizing-claude’s-behavior">
  Personalizar el comportamiento de Claude
</h3>

Puede configurar el comportamiento de Claude de dos formas:

1. **CLAUDE.md**: Defina estándares de codificación, criterios de revisión y reglas específicas del proyecto en un archivo `CLAUDE.md` en la raíz de su repositorio. Claude seguirá estas directrices al crear PR y responder a solicitudes. Consulte nuestra [documentación de Memory](/docs/es/memory) para más detalles.
2. **Solicitudes personalizadas**: Use el parámetro `prompt` en el archivo de flujo de trabajo para proporcionar instrucciones específicas del flujo de trabajo. Esto le permite personalizar el comportamiento de Claude para diferentes flujos de trabajo o tareas.

Claude seguirá estas directrices al crear PR y responder a solicitudes.
