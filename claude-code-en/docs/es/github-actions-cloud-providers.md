> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code GitHub Actions con proveedores en la nube

> Ejecute Claude Code GitHub Actions a través de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry en lugar de la API de Claude

[Claude Code GitHub Actions](/docs/es/github-actions) llama a la API de Claude de forma predeterminada. Para enrutar la inferencia a través de su propia cuenta en la nube, establezca la entrada del proveedor de Claude Code GitHub Action y configure su nube para confiar en el token de OpenID Connect (OIDC) del flujo de trabajo. El flujo de trabajo se autentica con ese token, por lo que no almacena ninguna credencial en la nube de larga duración en su repositorio.

<Info>
  Esta página se basa en la [configuración de GitHub Actions](/docs/es/github-actions#setup). Asume que ya conoce el archivo de flujo de trabajo y el paso `anthropics/claude-code-action`, y cubre solo lo que cambia un proveedor en la nube.
</Info>

<h2 id="choose-your-provider">
  Elija su proveedor
</h2>

Claude Code GitHub Action admite tres proveedores, y los pasos de configuración a continuación difieren solo en la configuración del lado de la nube. Use el que su organización ya tenga acceso a modelos de Claude. Indica a Claude Code GitHub Action qué proveedor usar con una entrada en el bloque `with:` del paso `anthropics/claude-code-action`:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Los ejemplos de flujo de trabajo completo en [Configurar la integración](#set-up-the-integration) ya incluyen la entrada para cada proveedor.

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de comenzar, necesita:

* Acceso de administrador al repositorio donde se ejecuta Claude Code GitHub Action, para instalar una aplicación de GitHub y agregar secretos
* Permiso para crear recursos de identidad en su cuenta en la nube: roles de IAM y proveedores de identidad OIDC en AWS, recursos de Workload Identity Federation y cuentas de servicio en Google Cloud, o aplicaciones de Microsoft Entra en Azure
* Acceso a modelos de Claude en su proveedor:
  * **Amazon Bedrock**: acceso otorgado a modelos de Claude. Los perfiles de inferencia entre regiones, como los ID de modelo `us.` en los ejemplos de esta página, necesitan acceso otorgado en cada región de su grupo de regiones. Consulte [Claude Code en Amazon Bedrock](/docs/es/amazon-bedrock)
  * **Google Cloud's Agent Platform**: un proyecto con la API de Agent Platform habilitada y acceso a modelos de Claude. Consulte [Claude Code en Google Cloud's Agent Platform](/docs/es/google-vertex-ai)
  * **Microsoft Foundry**: un recurso de Foundry con una implementación de modelo de Claude. Consulte [Claude Code en Microsoft Foundry](/docs/es/microsoft-foundry)

<h2 id="set-up-the-integration">
  Configurar la integración
</h2>

Más allá de los requisitos previos, crea una identidad de GitHub para Claude Code GitHub Action, la configuración de confianza del lado de la nube, los secretos del repositorio y el archivo de flujo de trabajo. Los pasos a continuación lo guían a través de cada uno.

<Steps>
  <Step title="Elija una identidad de GitHub">
    Claude Code GitHub Action envía confirmaciones y publica comentarios a través de una identidad de GitHub. La [configuración rápida](/docs/es/github-actions#quick-setup) instala la aplicación oficial de Claude GitHub para esto. Con un proveedor en la nube, elige la identidad usted mismo:

    * **[Aplicación oficial de Claude GitHub](https://github.com/apps/claude)**: instálela en el repositorio, o salte al siguiente paso si ya está instalada
    * **Aplicación personalizada de GitHub**: cree su propia aplicación cuando desee solo los tres permisos que usa Claude Code GitHub Action en lugar del [conjunto completo de la aplicación oficial](/docs/es/github-actions#github-app-permissions)
    * **Token automático `GITHUB_TOKEN` de GitHub**: sin aplicación para crear o instalar, pero GitHub no activa sus flujos de trabajo de CI en confirmaciones realizadas con él

    Los ejemplos de flujo de trabajo en el cuarto paso se autentican con una aplicación personalizada. Ese paso también dice qué cambiar para las otras dos opciones.

    Para crear una aplicación personalizada, [registre una nueva aplicación de GitHub](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) con webhooks deshabilitados, ya que esta integración no los usa. Otórguele tres permisos de repositorio:

    * **Contents**: lectura y escritura
    * **Issues**: lectura y escritura
    * **Pull requests**: lectura y escritura

    Después de registrar la aplicación, genere una clave privada y mantenga el archivo `.pem` descargado, anote el ID de la aplicación desde la página de configuración de la aplicación, e [instale la aplicación](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) en el repositorio donde se ejecuta Claude Code GitHub Action. Agregue la clave y el ID como secretos en el tercer paso.
  </Step>

  <Step title="Configurar la autenticación en la nube">
    Configure su nube para confiar en el token OIDC que GitHub emite al flujo de trabajo, de modo que cada ejecución del flujo de trabajo obtenga credenciales en la nube de corta duración. Los puntos en cada pestaña resumen lo que debe crear, y cada pestaña vincula la guía del proveedor de nube para los pasos a nivel de consola.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Cree la configuración de confianza en su cuenta de AWS, siguiendo la [guía de AWS para crear proveedores de identidad OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html):

        * Agregue un proveedor de identidad OIDC de GitHub con URL de proveedor `https://token.actions.githubusercontent.com` y audiencia `sts.amazonaws.com`
        * Cree un rol de IAM confiado por ese proveedor como una identidad web, y adjunte la política de invocación con alcance de [Configuración de IAM](/docs/es/amazon-bedrock#iam-configuration), que otorga `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles` y `bedrock:GetInferenceProfile`, junto con dos acciones de suscripción `aws-marketplace`
        * Limite la política de confianza del rol a su repositorio con una condición de asunto como `repo:your-org/your-repo:*`. Consulte la [guía de endurecimiento de OIDC de GitHub](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) para el formato de reclamación

        Anote el ARN del rol. Lo agregará como secreto en el siguiente paso.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Cree los recursos de federación en su proyecto de Google Cloud, siguiendo la [documentación de Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation):

        * Habilite tres API: IAM Credentials, Security Token Service (STS) y la API de Agent Platform, cuyo nombre de servicio es `aiplatform.googleapis.com`
        * Cree un Workload Identity Pool con un proveedor OIDC de GitHub cuyo emisor sea `https://token.actions.githubusercontent.com`, y agregue una condición de atributo que limite el grupo a su repositorio
        * Cree una cuenta de servicio dedicada con solo el rol `Vertex AI User`, que es `roles/aiplatform.user`, y permita que el grupo la suplante

        Anote el nombre de recurso completo del proveedor y la dirección de correo electrónico de la cuenta de servicio. Los agregará como secretos en el siguiente paso.
      </Tab>

      <Tab title="Microsoft Foundry">
        Cree una aplicación de Microsoft Entra con una credencial federada para su repositorio, siguiendo la [guía de Microsoft para autenticarse desde GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect):

        * Registre una aplicación de Microsoft Entra y agregue una credencial de identidad federada que confíe en los tokens que GitHub emite a su repositorio. Una identidad administrada asignada por el usuario funciona en lugar de una aplicación. Ambas tienen el ID de cliente que anota a continuación
        * Asigne a la aplicación el rol `Azure AI User` en su recurso de Foundry. Consulte [Configuración de RBAC de Azure](/docs/es/microsoft-foundry#azure-rbac-configuration) para un rol personalizado más estrecho

        Anote el ID de cliente de la aplicación, su ID de inquilino y su ID de suscripción. Los agregará como secretos en el siguiente paso.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Agregar secretos del repositorio">
    En el repositorio donde se ejecuta Claude Code GitHub Action, agregue los secretos para su proveedor, más los dos secretos de la aplicación si creó una aplicación personalizada de GitHub en el primer paso. Consulte la guía de GitHub sobre [uso de secretos en GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Secreto                          | Necesario para                     | Valor                                                       |
    | -------------------------------- | ---------------------------------- | ----------------------------------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                     | El ARN del rol de IAM                                       |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform      | El nombre de recurso completo del proveedor                 |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform      | La dirección de correo electrónico de la cuenta de servicio |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry                  | El ID de cliente de la aplicación de Entra                  |
    | `AZURE_TENANT_ID`                | Microsoft Foundry                  | Su ID de inquilino de Microsoft Entra                       |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry                  | Su ID de suscripción de Azure                               |
    | `APP_ID`                         | Aplicación personalizada de GitHub | El ID de la aplicación de GitHub                            |
    | `APP_PRIVATE_KEY`                | Aplicación personalizada de GitHub | El contenido del archivo de clave privada `.pem`            |
  </Step>

  <Step title="Crear el archivo de flujo de trabajo">
    Cree un archivo de flujo de trabajo para su proveedor, como `.github/workflows/claude.yml`. Cada ejemplo responde a menciones de `@claude`, se autentica en GitHub con una aplicación personalizada e incluye el permiso `id-token: write`, que GitHub requiere para emitir el token OIDC que su proveedor en la nube intercambia por credenciales.

    Si eligió una identidad de GitHub diferente en el primer paso, ajuste el ejemplo:

    * **Aplicación oficial de Claude GitHub**: elimine el paso Generar token de aplicación de GitHub y la línea `github_token`
    * **Token automático de GitHub**: elimine el paso de generación de token y cambie la línea `github_token` a `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      En repositorios públicos, un comentario que contenga la frase de activación de cualquier usuario inicia este flujo de trabajo. Los pasos de credenciales se ejecutan antes de que Claude Code GitHub Action verifique el acceso de escritura del comentarista, por lo que la acción rechaza a los usuarios no autorizados solo después de que el flujo de trabajo ha generado un token de aplicación e iniciado sesión en su proveedor en la nube, lo que deja entradas en el registro de auditoría y consume minutos de Actions. Para evitar esas ejecuciones, agregue un paso que verifique el acceso de escritura del comentarista antes de los pasos de credenciales.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Reemplace el valor `aws-region` con el suyo. El paso de credenciales lo exporta como `AWS_REGION` para el resto del trabajo.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          Los ID de modelo de Bedrock incluyen un prefijo de perfil de inferencia entre regiones como `us.`. Use el prefijo para el grupo de regiones donde otorgó acceso al modelo.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Reemplace el valor `CLOUD_ML_REGION` con el suyo. No necesita codificar el ID del proyecto, porque el flujo de trabajo lo lee de la salida del paso `auth`.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        Reemplace `your-resource-name` con el nombre de su recurso de Foundry. Claude Code construye la URL del punto de conexión a partir de él. El paso `azure/login` inicia sesión con el token OIDC del flujo de trabajo, y Claude Code recoge las credenciales a través de la [cadena de credenciales predeterminada](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) de Azure.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Use un ID de modelo que coincida con una implementación de Claude en su recurso de Foundry. Consulte [Claude Code en Microsoft Foundry](/docs/es/microsoft-foundry) para la configuración del modelo y el anclaje de versiones.
        </Tip>
      </Tab>
    </Tabs>

    Con cualquier proveedor, puede limitar la duración de la ejecución y el costo agregando `--max-turns` a `claude_args`. Consulte [Administrar costos](/docs/es/github-actions#manage-costs).
  </Step>

  <Step title="Probar la configuración">
    Mencione `@claude` en un comentario de problema o solicitud de extracción, luego observe la ejecución en la pestaña Actions del repositorio. Claude responde en un comentario en el mismo problema o solicitud de extracción.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Una ejecución fallida generalmente se interrumpe en uno de dos lugares:

* **Errores de autenticación**: generalmente una configuración incorrecta de OIDC. Verifique que el flujo de trabajo incluya el permiso `id-token: write`, que la condición del repositorio de la configuración de confianza coincida exactamente con su repositorio, y que los nombres de secreto en su flujo de trabajo coincidan con los que agregó
* **Problemas de activación e IC**: se comportan igual que cuando Claude Code GitHub Action llama a la API de Claude. Consulte la [sección de solución de problemas](/docs/es/github-actions#troubleshooting) de la página principal y las [Preguntas frecuentes](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) de Claude Code GitHub Action

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Claude Code GitHub Actions](/docs/es/github-actions) para ejemplos, parámetros y mejores prácticas
* [Claude Code en Amazon Bedrock](/docs/es/amazon-bedrock) para ID de modelo de Bedrock y regiones
* [Claude Code en Google Cloud's Agent Platform](/docs/es/google-vertex-ai) para ID de modelo de Agent Platform y regiones
* [Claude Code en Microsoft Foundry](/docs/es/microsoft-foundry) para configuración de modelo y punto de conexión de Foundry
