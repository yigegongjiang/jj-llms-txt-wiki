> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implementar Claude apps gateway en Google Cloud

> Un ejemplo práctico de ejecutar Claude apps gateway en Google Cloud: Cloud Run o GKE, Cloud SQL para PostgreSQL, Secret Manager y autenticación de cuenta de servicio en Agent Platform de Google Cloud.

<Note>
  Esta página describe una forma de ejecutar Claude apps gateway en Google Cloud. La configuración es un ejemplo funcional para infraestructura administrada por el cliente en lugar de una implementación de producción compatible; úsela para ver cómo encajan las piezas antes de adaptarla a su propio entorno. Para los requisitos independientes de la plataforma, consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy).
</Note>

Este ejemplo aprovisiona Claude apps gateway en Google Cloud con Google Cloud's Agent Platform como el upstream del modelo, utilizando Cloud Run o GKE para la computación. Google Workspace es el ejemplo del proveedor de identidad (IdP), pero cualquier proveedor de identidad compatible con OpenID Connect (OIDC) funciona; solo cambia el bloque `oidc`. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para obtener detalles específicos de cada IdP.

<h2 id="what-you’ll-build">
  Lo que construirá
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagrama de Claude apps gateway en Google Cloud: los clientes de Claude Code se conectan a través de HTTPS a la puerta de enlace (Cloud Run o GKE), que se ejecuta dentro de una VPC junto a una base de datos Cloud SQL de IP privada para el estado de la sesión. La puerta de enlace inicia sesión de usuarios a través de OIDC contra Google Workspace, lee la configuración y los secretos de Secret Manager, reenvía solicitudes de modelo a Agent Platform y extrae su imagen de Artifact Registry en la implementación." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

La configuración de referencia aprovisiona:

* Servicio **Cloud Run** o **GKE** Deployment ejecutando el contenedor de la puerta de enlace
* Repositorio de **Artifact Registry** para la imagen de la puerta de enlace
* Instancia de **Cloud SQL para PostgreSQL**, solo IP privada, para el [store](/docs/es/claude-apps-gateway-config#store) de la puerta de enlace
* Secretos de **Secret Manager** para `gateway.yaml`, la clave de firma JWT, el secreto del cliente OIDC y la URL de Postgres
* **Cuenta de servicio** con `roles/aiplatform.user`, adjunta directamente en Cloud Run o vinculada a través de Workload Identity en GKE
* **Equilibrador de carga de aplicaciones interno** en Cloud Run, o un **GKE Ingress** interno de clase `gce-internal` en GKE, para HTTPS

<h2 id="prerequisites">
  Requisitos previos
</h2>

* Un proyecto de GCP con facturación habilitada y permiso para crear los recursos anteriores
* La CLI `gcloud`, autenticada con `gcloud auth login`, y Docker instalado localmente
* Para la ruta de GKE: `kubectl` y un clúster de GKE en la VPC creada en el tutorial a continuación
* Acceso a los modelos de Claude que necesita en Model Garden, en una región que los publique
* Un cliente de aplicación web OAuth 2.0 de Google Workspace con URI de redirección `https://<gateway-host>/oauth/callback`; consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup)
* Un nombre de host TLS para la puerta de enlace, típicamente un nombre DNS interno que apunta al equilibrador de carga

Establezca el proyecto y la región una vez:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Implementar la puerta de enlace
</h2>

Los pasos a continuación aprovisionan la implementación completa con comandos `gcloud`.

<Steps>
  <Step title="Habilitar APIs">
    Habilite las API de servicio que utiliza el tutorial:

    ```bash theme={null}
    gcloud services enable \
      aiplatform.googleapis.com \
      artifactregistry.googleapis.com \
      sqladmin.googleapis.com \
      secretmanager.googleapis.com \
      iamcredentials.googleapis.com \
      iam.googleapis.com \
      compute.googleapis.com \
      servicenetworking.googleapis.com \
      run.googleapis.com \
      container.googleapis.com
    ```

    Las API que necesita dependen de la ruta de implementación:

    * `compute` y `servicenetworking`: necesarias para la ruta de Cloud SQL de IP privada
    * `run`: solo Cloud Run
    * `container`: solo GKE
  </Step>

  <Step title="Crear la cuenta de servicio y otorgar IAM">
    La puerta de enlace se ejecuta como una cuenta de servicio dedicada con permiso para llamar a Agent Platform. Alcanza Cloud SQL sobre la VPC con un usuario de contraseña, por lo que no se requiere ningún rol de IAM de Cloud SQL:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Luego habilite los modelos de Claude para el proyecto en Model Garden; los modelos se publican en regiones específicas, así que verifique cada tarjeta de modelo.
  </Step>

  <Step title="Construir e insertar la imagen en Artifact Registry">
    Construya la imagen según los [requisitos de imagen de contenedor](/docs/es/claude-apps-gateway-deploy#container-image), utilizando el binario glibc `linux-x64`, e insértela:

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run requires linux/amd64. --provenance=false avoids a buildx OCI
    # image index that Cloud Run rejects.
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Aprovisionar Cloud SQL para PostgreSQL">
    Cree la instancia en una VPC a través de Private Services Access para que no tenga IP pública; esto también satisface proyectos donde `constraints/sql.restrictPublicIp` se aplica:

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access: one-time per VPC
    gcloud compute addresses create "google-managed-services-${VPC}" \
      --global --purpose=VPC_PEERING --prefix-length=16 --network="$VPC"
    gcloud services vpc-peerings connect \
      --service=servicenetworking.googleapis.com \
      --ranges="google-managed-services-${VPC}" --network="$VPC"

    gcloud sql instances create claude-gateway-db \
      --database-version=POSTGRES_16 --tier=db-g1-small --region="$REGION" \
      --network="projects/${PROJECT_ID}/global/networks/${VPC}" --no-assign-ip
    gcloud sql databases create claude_gateway --instance=claude-gateway-db
    PGPASS="$(openssl rand -hex 24)"
    gcloud sql users create gateway --instance=claude-gateway-db --password="$PGPASS"

    PRIVATE_IP="$(gcloud sql instances describe claude-gateway-db \
      --format='value(ipAddresses[0].ipAddress)')"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${PRIVATE_IP}:5432/claude_gateway?sslmode=require"
    ```

    El tiempo de ejecución de Cloud Run o GKE debe estar en, o enrutado hacia, esta VPC.
  </Step>

  <Step title="Escribir gateway.yaml">
    El bloque `upstreams` apunta a Agent Platform con `auth: {}`, por lo que la puerta de enlace se autentica a través de Application Default Credentials desde la cuenta de servicio del tiempo de ejecución. Consulte la [referencia de configuración](/docs/es/claude-apps-gateway-config) para cada campo.

    Dos campos `listen` dependen de lo que está frente a la puerta de enlace:

    * `public_url`: requerido detrás de Cloud Run o un GKE Ingress. La puerta de enlace construye el `redirect_uri` del IdP y su documento de descubrimiento solo a partir de este valor, nunca a partir de encabezados `X-Forwarded-*`.
    * `trusted_proxies`: los rangos de origen del front end. La puerta de enlace honra `X-Forwarded-For` solo cuando el par TCP está en esta lista, luego camina la cadena pasando saltos confiables, por lo que los límites de velocidad de inicio de sesión por IP y los eventos de auditoría registran las IP de los desarrolladores en lugar de la del equilibrador de carga.

    Establezca `trusted_proxies` para que coincida con su front end. Un GKE Ingress externo de clase `gce` no está listado: aprovisiona una dirección de regla de reenvío público, que la [verificación de red privada](/docs/es/claude-apps-gateway#prerequisites) de `/login` rechaza.

    | Front end                                                        | `trusted_proxies`                                       |
    | ---------------------------------------------------------------- | ------------------------------------------------------- |
    | Cloud Run alcanzado directamente, sin equilibrador de carga      | `[169.254.0.0/16]`                                      |
    | Equilibrador de carga de aplicaciones interno frente a Cloud Run | `169.254.0.0/16` más el CIDR de su subred de solo proxy |
    | GKE Ingress interno, clase `gce-internal`                        | El CIDR de su subred de solo proxy                      |

    El ejemplo a continuación utiliza los valores del equilibrador de carga interno frente a Cloud Run.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [169.254.0.0/16, <your-proxy-only-subnet-cidr>]

    oidc:
      issuer: https://accounts.google.com
      client_id: <your-oauth-client-id>
      client_secret: ${OIDC_CLIENT_SECRET}           # GKE: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Google ignores offline_access; these yield refresh tokens:
      scopes: [openid, profile, email]
      extra_auth_params: { access_type: offline, prompt: consent }

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # GKE: ${file:/secrets/jwt-secret}

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # GKE: ${file:/secrets/postgres-url}

    upstreams:
      - provider: vertex
        region: <your-region>                        # must match $REGION
        project_id: <your-project>
        auth: {} # ADC via the runtime service account
    ```

    <Note>
      Los id\_tokens de Google no llevan ninguna reclamación `groups`. Para usar políticas basadas en grupos en [`managed.policies`](/docs/es/claude-apps-gateway-config#managed) con Google Workspace como IdP, configure [`oidc.google_groups`](/docs/es/claude-apps-gateway-config#oidc), que busca los grupos de cada usuario a través de la API de Admin SDK Directory usando una cuenta de servicio con delegación en todo el dominio. Sin ella, coincida con `email_domain` en su lugar.
    </Note>
  </Step>

  <Step title="Almacenar secretos en Secret Manager">
    Cree cuatro secretos y otorgue `roles/secretmanager.secretAccessor` a la cuenta de servicio `claude-gateway`:

    | Secreto                      | Origen                                        |
    | ---------------------------- | --------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                     |
    | `gateway-oidc-client-secret` | Google Cloud Console → Cliente OAuth          |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` del paso de Cloud SQL |
    | `gateway-config`             | el `gateway.yaml` completo del paso anterior  |

    La forma en que los secretos llegan al contenedor difiere según la ruta:

    * En GKE se montan como archivos a través del controlador CSI de Secret Manager, y `gateway.yaml` hace referencia a `${file:/secrets/...}`.
    * En Cloud Run, que no puede montar múltiples secretos en un directorio, `gateway.yaml` se monta como un archivo y los otros tres se inyectan como variables de entorno, por lo que `gateway.yaml` hace referencia a `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` y `${GATEWAY_POSTGRES_URL}` en su lugar.
  </Step>

  <Step title="Implementar">
    <Tabs>
      <Tab title="Cloud Run">
        El comando a continuación se implementa para producción detrás de un equilibrador de carga interno.

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --timeout=3600 \
          --ingress=internal-and-cloud-load-balancing \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        La salida directa de VPC, a través de `--network`, `--subnet` y `--vpc-egress=private-ranges-only`, permite que el servicio alcance la IP privada de Cloud SQL directamente. La salida pública a los puntos finales de Agent Platform y `accounts.google.com` va directamente a Internet en lugar de a través de la VPC, por lo que no se necesita Cloud NAT.

        La verificación de IAM del invocador debe estar abierta o deshabilitada. La puerta de enlace ejecuta su propio OIDC y sus clientes no llevan ningún token de GCP, por lo que la verificación del invocador de Cloud Run tiene que admitir solicitudes no autenticadas. La autenticación OIDC de la puerta de enlace autentica la solicitud una vez que llega al contenedor, con `allowed_email_domains` controlando qué dominios pueden iniciar sesión.

        Dos indicadores admiten solicitudes no autenticadas:

        * `--no-invoker-iam-check`: deshabilita la verificación sin ningún enlace `allUsers` para administrar, y funciona bajo Domain Restricted Sharing
        * `--allow-unauthenticated`: otorga al rol `run.invoker` a `allUsers`; úselo si su organización no permite `--no-invoker-iam-check`

        La restricción de ingreso a través de `--ingress` es una capa separada e independiente de la verificación del invocador; manténgala establecida para limitar el servicio a su red corporativa.

        Por defecto, la URL de Cloud Run `*.run.app` se resuelve a una dirección pública, que la [verificación de red privada](/docs/es/claude-apps-gateway#prerequisites) de `/login` rechaza. Dos topologías dan a los desarrolladores un nombre de host resolvible de forma privada, y Cloud Run no aprovisiona ninguno para usted:

        * **Equilibrador de carga de aplicaciones interno**, la topología que el comando de implementación anterior asume: implemente con `--ingress=internal-and-cloud-load-balancing`, aprovisione un equilibrador de carga de aplicaciones interno frente al servicio con un nombre DNS interno y certificado, y establezca `listen.public_url` en ese nombre de host.
        * **Ingreso solo interno sin equilibrador de carga**: implemente con `--ingress=internal` y deje `listen.public_url` como la URL `*.run.app`, el valor predeterminado en los [activos de referencia](#terraform-reference) a continuación. Para que `*.run.app` se resuelva de forma privada, su equipo de red debe operar un punto final de Private Service Connect para las API de Google, una zona privada de Cloud DNS que resuelva `*.run.app` a él, y enrutamiento local a ese punto final.

        La [guía de redes privadas de Google para Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) cubre la infraestructura que ambas opciones necesitan. Verifique el inicio de sesión una vez que la puerta de enlace esté sirviendo en un nombre de host privado; hasta entonces, confirme que el contenedor se inició desde sus registros en Cloud Run.

        Actualice el URI de redirección autorizado del cliente OAuth a `<public_url>/oauth/callback` antes del primer inicio de sesión. Reimplemente después de cambiar `public_url`, porque la puerta de enlace construye su origen público solo a partir de esa configuración e ignora `X-Forwarded-Host` y `X-Forwarded-Proto`. `X-Forwarded-For` se honra para las IP de cliente solo cuando `listen.trusted_proxies` está establecido.
      </Tab>

      <Tab title="GKE">
        El clúster debe estar en la `$VPC` creada en el paso de Cloud SQL para que los pods puedan alcanzar la IP privada de la base de datos; el emparejamiento de VPC solo no funciona, porque la IP privada de Cloud SQL es en sí misma una red emparejada y el emparejamiento no es transitivo. Para crear un nuevo clúster en esa VPC, pase `--network="$VPC" --subnetwork=cc-gateway-subnet` a `gcloud container clusters create`.

        Habilite Workload Identity en el clúster y sus grupos de nodos, luego vincule la cuenta de servicio de Google a la cuenta de servicio de Kubernetes para que los pods hereden sus credenciales:

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # On a Standard cluster, existing node pools also need GKE_METADATA;
        # Autopilot enables this by default.
        gcloud container node-pools update <pool> --cluster=<cluster> \
          --region="$REGION" --workload-metadata=GKE_METADATA

        kubectl create namespace claude-gateway
        kubectl create serviceaccount gateway -n claude-gateway

        gcloud iam service-accounts add-iam-policy-binding \
          "claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --role roles/iam.workloadIdentityUser \
          --member "serviceAccount:${PROJECT_ID}.svc.id.goog[claude-gateway/gateway]"

        kubectl annotate serviceaccount gateway -n claude-gateway \
          iam.gke.io/gcp-service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"
        ```

        Implemente la puerta de enlace como un Deployment estándar más un Service e un Ingress interno, clase `gce-internal`, como se describe en [Implementación de Kubernetes](/docs/es/claude-apps-gateway-deploy#kubernetes), con:

        * `serviceAccountName: gateway`
        * el controlador CSI de Secret Manager montando secretos en `/secrets`
        * la sonda de disponibilidad apuntando a `GET /readyz`

        Adjunte un BackendConfig con un `timeoutSec` elevado al Service de la puerta de enlace: el servicio backend del equilibrador de carga detrás de GKE Ingress tiene un tiempo de espera predeterminado de 30 segundos, que corta las respuestas de transmisión larga.

        No aplique una NetworkPolicy de salida que bloquee `169.254.169.254` en un clúster de Workload Identity; el pod debe alcanzar el servidor de metadatos para las credenciales. La [protección SSRF](/docs/es/claude-apps-gateway-deploy#threat-model-summary) integrada de la puerta de enlace es la defensa allí.

        La puerta de enlace registra una advertencia de arranque de que el punto final de metadatos es alcanzable y sugiere aplicar una NetworkPolicy de salida. Bajo Workload Identity esa advertencia es esperada, porque el pod necesita el punto final.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Insertar la URL de la puerta de enlace en máquinas de desarrolladores">
    La puerta de enlace ahora se está ejecutando, pero los desarrolladores no pueden alcanzarla desde `/login` hasta que la URL de la puerta de enlace esté en sus máquinas. Establezca `forceLoginMethod` y `forceLoginGatewayUrl` en el [archivo de configuración administrada](/docs/es/claude-apps-gateway#set-the-gateway-url) que implementa en cada dispositivo a través de MDM. No hay opción de puerta de enlace en el selector de inicio de sesión para que un desarrollador seleccione manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referencia de Terraform
</h2>

Los [activos de implementación de referencia](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) automatizan la ruta de Cloud Run en esta página; los activos de configuración e imagen se aplican a ambas rutas:

* `setup.sh`: un aprovisionador `gcloud` idempotente que recorre la ruta completa de Cloud Run, desde habilitar API hasta la primera implementación
* `terraform/`: la misma implementación como infraestructura como código, para una implementación de campo virgen: una aplicación dirigida para crear el repositorio de Artifact Registry, luego construir e insertar la imagen, luego una aplicación completa
* `gateway.yaml.example` y un `Dockerfile` para la imagen de tiempo de ejecución sin distro

Los artefactos establecen por defecto el ingreso de Cloud Run a `internal`, por lo que no se requiere equilibrador de carga. Para que coincida con la implementación de producción detrás de un ALB de esta página, ejecute `setup.sh` con `INGRESS=internal-and-cloud-load-balancing`, o establezca la variable de Terraform `ingress` a `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Los artefactos también establecen por defecto la capa del invocador a una concesión `run.invoker` de `allUsers` en lugar de `--no-invoker-iam-check`, lo inverso del tutorial de esta página; cualquiera funciona, y la elección depende de las restricciones de política de su organización.

Los activos se proporcionan como ejemplos funcionales, no como un artefacto de producción compatible; revíselos y adáptelos a su entorno.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para errores de arranque y inicio de sesión de la puerta de enlace, consulte la tabla de [solución de problemas](/docs/es/claude-apps-gateway-deploy#troubleshooting) independiente de la plataforma. Las entradas a continuación son específicas de Google Cloud.

| Síntoma                                                                                                       | Causa                                                                                                                                                                                                | Solución                                                                                                                                                                                                                                     |
| ------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run devuelve `403 Forbidden` antes de alcanzar el contenedor                                            | La verificación de IAM del invocador aún está habilitada                                                                                                                                             | Implemente con `--no-invoker-iam-check`, o otorgue a `allUsers` el rol `run.invoker` con `--allow-unauthenticated`                                                                                                                           |
| `--no-invoker-iam-check` rechazado con `invoker_iam_disabled is not currently available`                      | Bloqueado por `constraints/run.managed.requireInvokerIam`                                                                                                                                            | Use `--allow-unauthenticated`. Si Domain Restricted Sharing a través de `constraints/iam.allowedPolicyMemberDomains` también bloquea eso, use la ruta de GKE, que expone la puerta de enlace en la capa de red sin ningún enlace `allUsers`. |
| `Container manifest type … must support amd64/linux` en la implementación                                     | La imagen se construyó en un host que no es amd64, o buildx emitió un índice de imagen OCI                                                                                                           | Construya con `--platform=linux/amd64 --provenance=false`                                                                                                                                                                                    |
| El arranque de la puerta de enlace sale con un error de tiempo de espera de conexión de Postgres en Cloud Run | El servicio no está adjunto a la VPC, o Cloud SQL no tiene IP privada en esa VPC; el almacén deja de esperar después de 5 segundos                                                                   | Implemente con `--network` y `--subnet` para salida directa de VPC, y cree la instancia de Cloud SQL con `--no-assign-ip` y `--network` apuntando a la misma VPC                                                                             |
| Las solicitudes de Agent Platform devuelven `403 PERMISSION_DENIED`                                           | El tiempo de ejecución no está utilizando la cuenta de servicio `claude-gateway`, o el modelo no está habilitado en Model Garden para el proyecto                                                    | Establezca `--service-account` en Cloud Run o vincule Workload Identity en GKE, y habilite cada modelo de Claude en Model Garden para la región de destino                                                                                   |
| Las respuestas de transmisión se cortan después de una duración fija                                          | Tiempo de espera de solicitud del front end: el servicio backend del equilibrador de carga detrás de GKE Ingress tiene un tiempo de espera predeterminado de 30 segundos y Cloud Run de 300 segundos | Adjunte un BackendConfig con un `timeoutSec` elevado en GKE, o implemente con `--timeout=3600` en Cloud Run                                                                                                                                  |

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Referencia de configuración](/docs/es/claude-apps-gateway-config): cada opción de `gateway.yaml`, incluyendo `managed.policies` y `telemetry`
* [Implementación y operaciones](/docs/es/claude-apps-gateway-deploy): configuración de IdP, verificaciones de salud, rotación de secretos JWT, actualizaciones y el modelo de seguridad
* [Descripción general de Claude apps gateway](/docs/es/claude-apps-gateway): inicio rápido y conexión de desarrolladores
