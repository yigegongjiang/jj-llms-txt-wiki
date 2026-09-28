> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Aprenda a integrar Claude Code en su flujo de trabajo de desarrollo con GitLab CI/CD

<Info>
  Claude Code para GitLab CI/CD se encuentra actualmente en beta. Las características y funcionalidades pueden evolucionar a medida que refinamos la experiencia.

  Esta integración es mantenida por GitLab. Para obtener soporte, consulte el siguiente [problema de GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  Esta integración se basa en el [Claude Code CLI y Agent SDK](/docs/es/agent-sdk/overview), lo que permite el uso programático de Claude en sus trabajos de CI/CD y flujos de trabajo de automatización personalizados.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  ¿Por qué usar Claude Code con GitLab?
</h2>

* **Creación instantánea de MR**: Describa lo que necesita, y Claude propone un MR completo con cambios y explicación
* **Implementación automatizada**: Convierta problemas en código funcional con un único comando o mención
* **Consciente del proyecto**: Claude sigue sus directrices `CLAUDE.md` y patrones de código existentes
* **Configuración simple**: Agregue un trabajo a `.gitlab-ci.yml` y una variable de CI/CD enmascarada
* **Listo para empresas**: Elija Claude API, Amazon Bedrock o Google Cloud's Agent Platform para cumplir con los requisitos de residencia de datos y adquisición
* **Seguro por defecto**: Se ejecuta en sus ejecutores de GitLab con su protección de rama y aprobaciones

<h2 id="how-it-works">
  Cómo funciona
</h2>

Claude Code utiliza GitLab CI/CD para ejecutar tareas de IA en trabajos aislados y confirmar resultados a través de MRs:

1. **Orquestación impulsada por eventos**: GitLab escucha los desencadenantes elegidos (por ejemplo, un comentario que menciona `@claude` en un problema, MR o hilo de revisión). El trabajo recopila contexto del hilo y repositorio, construye indicaciones a partir de esa entrada y ejecuta Claude Code.

2. **Abstracción de proveedores**: Utilice el proveedor que se ajuste a su entorno:
   * Claude API (SaaS)
   * Amazon Bedrock (acceso basado en IAM, opciones entre regiones)
   * Google Cloud's Agent Platform (nativo de GCP, Federación de Identidad de Carga de Trabajo)

3. **Ejecución en sandbox**: Cada interacción se ejecuta en un contenedor con reglas estrictas de red y sistema de archivos. Claude Code aplica permisos con alcance de espacio de trabajo para restringir escrituras. Cada cambio fluye a través de un MR para que los revisores vean el diff y las aprobaciones sigan siendo aplicables.

Elija puntos finales regionales para reducir la latencia y cumplir con los requisitos de soberanía de datos mientras utiliza acuerdos en la nube existentes.

<h2 id="what-can-claude-do">
  ¿Qué puede hacer Claude?
</h2>

En una canalización de GitLab, Claude Code puede:

* Crear y actualizar MRs a partir de descripciones de problemas o comentarios
* Analizar regresiones de rendimiento y proponer optimizaciones
* Implementar características directamente en una rama y luego abrir un MR
* Corregir errores y regresiones identificados por pruebas o comentarios
* Responder a comentarios de seguimiento para iterar sobre los cambios solicitados

<h2 id="setup">
  Configuración
</h2>

<h3 id="quick-setup">
  Configuración rápida
</h3>

La forma más rápida de comenzar es agregar un trabajo mínimo a su `.gitlab-ci.yml` y establecer su clave API como una variable enmascarada.

1. **Agregue una variable CI/CD enmascarada**
   * Vaya a **Configuración** → **CI/CD** → **Variables**
   * Agregue `ANTHROPIC_API_KEY` (enmascarada, protegida según sea necesario)

2. **Agregue un trabajo Claude a `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Adjust rules to fit how you want to trigger the job:
  # - manual runs
  # - merge request events
  # - web/API triggers when a comment contains '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Optional: start a GitLab MCP server if your setup provides one
    - /bin/gitlab-mcp-server || true
    # Use AI_FLOW_* variables when invoking via web/API triggers with context payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Después de agregar el trabajo y su variable `ANTHROPIC_API_KEY`, pruebe ejecutando el trabajo manualmente desde **CI/CD** → **Pipelines**, o actívelo desde un MR para permitir que Claude proponga actualizaciones en una rama y abra un MR si es necesario.

<Note>
  Para ejecutar en Amazon Bedrock o en la plataforma de agentes de Google Cloud en lugar de la API de Claude, consulte la sección [Uso con Amazon Bedrock y Google Cloud](#using-with-amazon-bedrock-and-google-cloud) a continuación para la configuración de autenticación y entorno.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Configuración manual (recomendada para producción)
</h3>

Si prefiere una configuración más controlada o necesita proveedores empresariales:

1. **Configure el acceso del proveedor**:
   * **Claude API**: Cree y almacene `ANTHROPIC_API_KEY` como una variable CI/CD enmascarada
   * **Amazon Bedrock**: **Configure GitLab** → **AWS OIDC** y cree un rol de IAM para Amazon Bedrock
   * **Plataforma de agentes de Google Cloud**: **Configure la federación de identidades de carga de trabajo para GitLab** → **GCP**

2. **Agregue credenciales de proyecto para operaciones de API de GitLab**:
   * Use `CI_JOB_TOKEN` de forma predeterminada, o cree un token de acceso de proyecto con alcance `api`
   * Almacene como `GITLAB_ACCESS_TOKEN` (enmascarado) si utiliza un PAT

3. **Agregue el trabajo Claude a `.gitlab-ci.yml`**: use el trabajo de [Configuración rápida](#quick-setup) para la API de Claude, o un trabajo de proveedor de [Ejemplos de configuración](#configuration-examples)

4. **(Opcional) Habilite desencadenadores impulsados por menciones**:
   * Agregue un webhook de proyecto para "Comentarios (notas)" a su escucha de eventos (si utiliza uno)
   * Haga que el escucha llame a la API de desencadenador de canalización con variables como `AI_FLOW_INPUT` y `AI_FLOW_CONTEXT` cuando un comentario contenga `@claude`

<h2 id="example-use-cases">
  Casos de uso de ejemplo
</h2>

<h3 id="turn-issues-into-mrs">
  Convertir problemas en MRs
</h3>

En un comentario de problema:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude analiza el problema y la base de código, escribe cambios en una rama y abre un MR para revisión.

<h3 id="get-implementation-help">
  Obtener ayuda de implementación
</h3>

En una discusión de MR:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude propone cambios, añade código con almacenamiento en caché apropiado y actualiza el MR.

<h3 id="fix-bugs-quickly">
  Corregir errores rápidamente
</h3>

En un comentario de problema o MR:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude localiza el error, implementa una corrección y actualiza la rama o abre un nuevo MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Uso con Amazon Bedrock y Google Cloud
</h2>

Para entornos empresariales, puede ejecutar Claude Code completamente en su infraestructura en la nube con la misma experiencia de desarrollador.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Requisitos previos

    Antes de configurar Claude Code con Amazon Bedrock, necesita:

    1. Una cuenta de AWS con acceso a Amazon Bedrock para los modelos Claude deseados
    2. GitLab configurado como proveedor de identidad OIDC en AWS IAM
    3. Un rol de IAM con permisos de Amazon Bedrock y una política de confianza restringida a su proyecto/referencias de GitLab
    4. Variables de CI/CD de GitLab para la asunción de roles:
       * `AWS_ROLE_TO_ASSUME` (ARN del rol)
       * `AWS_REGION` (región de Amazon Bedrock)

    ### Instrucciones de configuración

    Configure AWS para permitir que los trabajos de CI de GitLab asuman un rol de IAM a través de OIDC (sin claves estáticas).

    **Configuración requerida:**

    1. Habilite Amazon Bedrock y solicite acceso a sus modelos Claude objetivo
    2. Cree un proveedor OIDC de IAM para GitLab si aún no existe
    3. Cree un rol de IAM de confianza del proveedor OIDC de GitLab, restringido a su proyecto y referencias protegidas
    4. Adjunte permisos de menor privilegio para las API de invocación de Amazon Bedrock

    Utilice el [ejemplo de trabajo de Amazon Bedrock](#configuration-examples) para intercambiar el token OIDC del trabajo por credenciales temporales de AWS en tiempo de ejecución.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Requisitos previos

    Antes de configurar Claude Code con Google Cloud's Agent Platform, necesita:

    1. Un proyecto de Google Cloud con:
       * API de Google Cloud's Agent Platform habilitada
       * Workload Identity Federation configurada para confiar en OIDC de GitLab
    2. Una cuenta de servicio dedicada solo con los roles requeridos de Google Cloud's Agent Platform
    3. Variables de CI/CD de GitLab:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nombre del recurso del proveedor sin el prefijo `//iam.googleapis.com/`, como `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (correo electrónico de la cuenta de servicio)
       * `GCP_PROJECT_ID` (ID del proyecto de Google Cloud)

    ### Instrucciones de configuración

    Configure Google Cloud para permitir que los trabajos de CI de GitLab suplanten una cuenta de servicio a través de Workload Identity Federation.

    **Configuración requerida:**

    1. Habilite la API de Credenciales de IAM, la API de STS y la API de Google Cloud's Agent Platform
    2. Cree un Workload Identity Pool y un proveedor para OIDC de GitLab
    3. Cree una cuenta de servicio dedicada con roles de Google Cloud's Agent Platform
    4. Otorgue al principal de WIF permiso para suplantar la cuenta de servicio

    Utilice el [ejemplo de trabajo de Agent Platform](#configuration-examples) para autenticarse sin almacenar claves.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Ejemplos de configuración
</h2>

A continuación se muestran fragmentos listos para usar que puede adaptar a su canalización.

<h3 id="amazon-bedrock-job-example-oidc">
  Ejemplo de trabajo de Amazon Bedrock (OIDC)
</h3>

**Requisitos previos:**

* Amazon Bedrock habilitado con acceso a su(s) modelo(s) Claude elegido(s)
* OIDC de GitLab configurado en AWS con un rol que confía en su proyecto y referencias de GitLab
* Rol de IAM con permisos de Amazon Bedrock (se recomienda privilegio mínimo)

**Variables de CI/CD requeridas:**

* `AWS_ROLE_TO_ASSUME`: ARN del rol de IAM para acceso a Amazon Bedrock
* `AWS_REGION`: región de Amazon Bedrock (por ejemplo, `us-west-2`)

GitLab genera el token OIDC del trabajo desde el bloque `id_tokens:` y lo expone como `GITLAB_OIDC_TOKEN`. Establezca `aud` en el valor de audiencia que configuró en el proveedor de identidad OIDC de IAM en AWS, por ejemplo, la URL de su instancia de GitLab.

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Los ID de modelo para Amazon Bedrock incluyen prefijos específicos de región (por ejemplo, `us.anthropic.claude-sonnet-4-6`). Pase el modelo deseado a través de su configuración de trabajo o indicación si su flujo de trabajo lo admite.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Ejemplo de trabajo de Agent Platform (Workload Identity Federation)
</h3>

**Requisitos previos:**

* API de Agent Platform de Google Cloud habilitada en su proyecto de GCP
* Workload Identity Federation configurada para confiar en OIDC de GitLab
* Una cuenta de servicio con permisos de Agent Platform de Google Cloud

**Variables de CI/CD requeridas:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: nombre del recurso del proveedor sin el prefijo `//iam.googleapis.com/`, como `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: correo electrónico de la cuenta de servicio
* `GCP_PROJECT_ID`: ID del proyecto de Google Cloud
* `CLOUD_ML_REGION`: región de Agent Platform de Google Cloud (por ejemplo, `us-east5`)

GitLab genera el token OIDC del trabajo desde el bloque `id_tokens:` y lo expone como `GITLAB_OIDC_TOKEN`. Establezca `aud` en el valor de audiencia que configuró en el proveedor de Workload Identity Pool, por ejemplo, la URL de su instancia de GitLab. El trabajo escribe el token en un archivo, y la entrada `credential_source` de la configuración de credenciales le indica a las bibliotecas de autenticación de Google que lo lean desde allí. Establecer `GOOGLE_APPLICATION_CREDENTIALS` en el archivo de configuración de credenciales lo hace disponible para Claude Code a través de [Application Default Credentials](/docs/es/google-vertex-ai#3-configure-gcp-credentials).

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  Con Workload Identity Federation, no necesita almacenar claves de cuenta de servicio. Utilice condiciones de confianza específicas del repositorio y cuentas de servicio con privilegio mínimo.
</Note>

<h2 id="best-practices">
  Mejores prácticas
</h2>

<h3 id="claude-md-configuration">
  Configuración de CLAUDE.md
</h3>

Cree un archivo `CLAUDE.md` en la raíz del repositorio para definir estándares de codificación, criterios de revisión y reglas específicas del proyecto. Claude lee este archivo durante las ejecuciones y sigue sus convenciones al proponer cambios.

<h3 id="security-considerations">
  Consideraciones de seguridad
</h3>

**Nunca confirme claves API o credenciales en la nube en su repositorio**. Siempre use variables de GitLab CI/CD:

* Agregue `ANTHROPIC_API_KEY` como una variable enmascarada (y protéjala si es necesario)
* Use OIDC específico del proveedor donde sea posible (sin claves de larga duración)
* Limite los permisos de trabajo y la salida de red
* Revise los MR de Claude como cualquier otro colaborador

<h3 id="optimizing-performance">
  Optimización del rendimiento
</h3>

* Mantenga `CLAUDE.md` enfocado y conciso
* Proporcione descripciones claras de problemas/MR para reducir iteraciones
* Almacene en caché npm e instalaciones de paquetes en ejecutores donde sea posible

<h3 id="ci-costs">
  Costos de CI
</h3>

Cuando use Claude Code con GitLab CI/CD, tenga en cuenta los costos asociados:

* **Tiempo de GitLab Runner**:
  * Claude se ejecuta en sus ejecutores de GitLab y consume minutos de cómputo
  * Consulte los detalles de facturación del ejecutor de su plan de GitLab

* **Costos de API**:
  * Cada interacción de Claude consume tokens según el tamaño del mensaje y la respuesta
  * El uso de tokens varía según la complejidad de la tarea y el tamaño de la base de código
  * Consulte [Precios de Anthropic](https://platform.claude.com/docs/en/about-claude/pricing) para obtener más detalles

* **Consejos de optimización de costos**:
  * Use comandos específicos `@claude` para reducir turnos innecesarios
  * Establezca valores apropiados de `--max-turns` y `timeout` de trabajo
  * Limite la concurrencia para controlar ejecuciones paralelas

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude no responde a comandos @claude
</h3>

* Verifique que su pipeline se esté activando (manualmente, evento de MR o mediante un escuchador de eventos de nota/webhook)
* Asegúrese de que sus variables `ANTHROPIC_API_KEY` o del proveedor de nube estén presentes
* Compruebe que el comentario contenga `@claude` (no `/claude`) y que su disparador de mención esté configurado

<h3 id="job-can’t-write-comments-or-open-mrs">
  El trabajo no puede escribir comentarios ni abrir MRs
</h3>

* Asegúrese de que `CI_JOB_TOKEN` tenga permisos suficientes para el proyecto, o utilice un Token de Acceso de Proyecto con alcance `api`
* Compruebe que la herramienta `mcp__gitlab` esté habilitada en `--allowedTools`
* Confirme que el trabajo se ejecuta en el contexto del MR o tiene suficiente contexto a través de variables `AI_FLOW_*`

<h3 id="authentication-errors">
  Errores de autenticación
</h3>

* **Para Claude API**: Confirme que `ANTHROPIC_API_KEY` sea válida y no haya expirado
* **Para Amazon Bedrock o la Plataforma de Agentes de Google Cloud**: Verifique la configuración de OIDC/WIF, la suplantación de roles y los nombres de secretos; confirme la disponibilidad de región y modelo

<h2 id="advanced-configuration">
  Configuración avanzada
</h2>

<h3 id="common-parameters-and-variables">
  Parámetros y variables comunes
</h3>

Controle las ejecuciones de Claude Code en sus trabajos con estas banderas CLI, palabras clave de GitLab y variables:

* `-p`: proporcione instrucciones en línea, por ejemplo `claude -p "Review this MR"`
* `--max-turns`: limite el número de iteraciones de ida y vuelta
* `timeout`: limite el tiempo total de ejecución del trabajo con la palabra clave `timeout` de nivel de trabajo de GitLab, por ejemplo `timeout: 30m`
* `ANTHROPIC_API_KEY`: requerida para la API de Claude (no se utiliza para Amazon Bedrock ni para la plataforma de agentes de Google Cloud)
* Entorno específico del proveedor: `AWS_REGION`, variables de proyecto/región para la plataforma de agentes de Google Cloud

<Note>
  Las banderas y parámetros exactos pueden variar según la versión de `@anthropic-ai/claude-code`. Ejecute `claude --help` en su trabajo para ver las opciones compatibles.
</Note>

<h3 id="customizing-claude’s-behavior">
  Personalización del comportamiento de Claude
</h3>

Puede guiar a Claude de dos formas principales:

1. **CLAUDE.md**: Defina estándares de codificación, requisitos de seguridad y convenciones del proyecto. Claude lee esto durante las ejecuciones y sigue sus reglas.
2. **Indicaciones personalizadas**: Pase instrucciones específicas de tareas a través de `-p` en el trabajo. Utilice diferentes indicaciones para diferentes trabajos (por ejemplo, revisión, implementación, refactorización).
