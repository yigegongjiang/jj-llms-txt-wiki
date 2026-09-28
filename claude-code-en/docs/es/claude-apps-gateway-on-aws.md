> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implementar Claude apps gateway en AWS

> Un ejemplo práctico de ejecutar Claude apps gateway en AWS: ECS Fargate o EKS, Amazon RDS para PostgreSQL, AWS Secrets Manager, y autenticación basada en roles de IAM a Amazon Bedrock.

<Note>
  Esta página le muestra una forma de ejecutar Claude apps gateway en AWS. La configuración es un ejemplo funcional para infraestructura administrada por el cliente en lugar de una implementación de producción compatible; úsela para ver cómo encajan las piezas antes de adaptarla a su propio entorno. Para los requisitos agnósticos de plataforma, consulte la [guía de implementación](/docs/es/claude-apps-gateway-deploy).
</Note>

Este ejemplo aprovisiona Claude apps gateway en AWS con Amazon Bedrock como el upstream del modelo, utilizando [Amazon ECS](https://aws.amazon.com/ecs/) en [AWS Fargate](https://aws.amazon.com/fargate/) o [Amazon EKS](https://aws.amazon.com/eks/) para la computación. [Okta](https://www.okta.com/) es el proveedor de identidad (IdP) de ejemplo, pero cualquier proveedor de identidad compatible con OpenID Connect (OIDC) funciona; consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup) para obtener detalles específicos de cada IdP.

<Note>
  Bedrock no es el único upstream de Claude en AWS. El gateway también admite Claude Platform on AWS, la API de Claude operada por Anthropic con autenticación de AWS y facturación de AWS Marketplace, en lugar de Bedrock o junto a él. Su entrada upstream, credenciales y permisos de IAM difieren de los específicos de Bedrock en esta página; la [referencia de upstream de Claude Platform on AWS](/docs/es/claude-apps-gateway-config#claude-platform-on-aws) cubre qué cambia, y el resto de esta página se aplica sin cambios.
</Note>

<h2 id="architecture">
  Arquitectura
</h2>

<Frame caption="La arquitectura de ejemplo, con Amazon Bedrock como el upstream del modelo. Un upstream de Claude Platform on AWS ocupa la misma posición.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagrama de Claude apps gateway en AWS: Los clientes de Claude Code se conectan a través de HTTPS a un Application Load Balancer interno que está delante del gateway (ECS Fargate o EKS), que se ejecuta en subredes privadas junto a una instancia de Amazon RDS para PostgreSQL para el estado de la sesión. El gateway inicia sesión de los usuarios a través de OIDC contra el IdP corporativo, lee secretos de AWS Secrets Manager, reenvía solicitudes de modelo a Amazon Bedrock usando su rol de IAM, y extrae su imagen de Amazon ECR en la implementación." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

El gateway se ejecuta como un endpoint HTTPS privado en su red al que los desarrolladores inician sesión a través de su IdP. Sus sesiones de Claude Code alcanzan los modelos de Claude en Amazon Bedrock a través del rol de IAM del gateway, por lo que ninguna credencial de modelo llega a las máquinas de los desarrolladores. La configuración de referencia aprovisiona:

* Servicio **Amazon ECS en AWS Fargate** o **Amazon EKS** Deployment ejecutando el contenedor del gateway
* Repositorio **Amazon ECR** para la imagen del gateway
* Instancia **Amazon RDS para PostgreSQL** en subredes privadas, no accesible públicamente, para el [almacén](/docs/es/claude-apps-gateway-config#store) del gateway
* Secretos de **AWS Secrets Manager** para la clave de firma JWT, el secreto del cliente OIDC y la URL de Postgres
* **Rol de IAM** con `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` y `bedrock:CountTokens`, adjunto como el rol de tarea de ECS o vinculado a través de Roles de IAM para Cuentas de Servicio (IRSA) en EKS
* **Application Load Balancer interno** para HTTPS

<h2 id="prerequisites">
  Requisitos previos
</h2>

El tutorial crea los recursos propios del gateway, pero se basa en la infraestructura de red e identidad que ya tiene. Antes de comenzar, necesita:

* Una cuenta de AWS con permiso para crear los [recursos anteriores](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) instalado y [autenticado](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), y [Docker](https://docs.docker.com/get-started/get-docker/) instalado localmente
* Una [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) con al menos dos [subredes privadas](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) en diferentes Zonas de Disponibilidad, con acceso a internet saliente a través de una [puerta de enlace NAT](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); el equilibrador de carga interno necesita subredes en dos AZ, y el gateway necesita salida a Bedrock y su IdP
* Una aplicación web OIDC de Okta con URI de redirección `https://<gateway-host>/oauth/callback`; consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup)
* Un nombre de host TLS para el gateway, típicamente un nombre DNS interno en una [zona alojada privada de Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html) que apunta al equilibrador de carga, con un [certificado ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) para ese nombre, importado o emitido por [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Establezca sus variables de entorno
</h3>

Cada comando en esta página lee cuatro valores de su shell: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` y `PRIVATE_SUBNETS`.

Elija una región de EE.UU. donde Bedrock sirva los modelos de Claude que necesita. El tutorial se basa en el catálogo de modelos integrado del gateway, que se resuelve en perfiles de inferencia `us.anthropic.*`, y la política de IAM otorga esos ARN. En una región que no sea de EE.UU., agregue un [bloque `models:`](/docs/es/claude-apps-gateway-config#models) con los ID de perfil de inferencia de esa geografía y cambie el prefijo ARN de la política de IAM para que coincida.

Si no tiene el ID de VPC a mano, enumere sus VPC con `aws ec2 describe-vpcs`, luego enumere las subredes de esa VPC para encontrar dos privadas en diferentes Zonas de Disponibilidad:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Exporte los cuatro antes de continuar:

```bash theme={null}
export AWS_REGION=us-east-1   # una región de EE.UU. donde Bedrock sirve los modelos de Claude que necesita
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Implemente el gateway
</h2>

Los pasos a continuación aprovisionan la implementación completa con comandos `aws`.

<Steps>
  <Step title="Cree los grupos de seguridad">
    Tres grupos de seguridad encadenan la ruta del tráfico: su red corporativa alcanza el equilibrador de carga en 443, el equilibrador de carga alcanza el gateway en 8080, y el gateway alcanza Postgres en 5432. Nada más es alcanzable. Cómo los adjunta depende de la pista de computación:

    * En ECS Fargate, el paso de implementación adjunta `$ALB_SG` al equilibrador de carga y `$GW_SG` al servicio.
    * En EKS, el Controlador de Equilibrador de Carga de AWS crea su propio grupo de seguridad frontend para el ALB, por lo que `$ALB_SG` y `$GW_SG` no se usan: la anotación `inbound-cidrs` del paso de implementación restringe el oyente a su red corporativa, y el grupo de seguridad de la base de datos admite el grupo de seguridad del clúster en su lugar.

    ```bash theme={null}
    ALB_SG="$(aws ec2 create-security-group --group-name claude-gateway-alb \
      --description "Claude gateway ALB" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    GW_SG="$(aws ec2 create-security-group --group-name claude-gateway-svc \
      --description "Claude gateway service" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    DB_SG="$(aws ec2 create-security-group --group-name claude-gateway-db \
      --description "Claude gateway Postgres" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"

    aws ec2 authorize-security-group-ingress --group-id "$ALB_SG" \
      --protocol tcp --port 443 --cidr <your-corporate-cidr>
    aws ec2 authorize-security-group-ingress --group-id "$GW_SG" \
      --protocol tcp --port 8080 --source-group "$ALB_SG"
    aws ec2 authorize-security-group-ingress --group-id "$DB_SG" \
      --protocol tcp --port 5432 --source-group "$GW_SG"
    ```
  </Step>

  <Step title="Cree los roles de IAM y envíe el formulario de caso de uso">
    El gateway se ejecuta con un rol de tarea dedicado cuyo único permiso es invocar modelos de Claude en Bedrock. Según la [referencia de upstream de Bedrock](/docs/es/claude-apps-gateway-config#amazon-bedrock), la política debe cubrir tanto los ARN de perfil de inferencia entre regiones como los ARN de modelo base subyacentes:

    ```bash theme={null}
    cat > bedrock-invoke.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["bedrock:InvokeModel", "bedrock:InvokeModelWithResponseStream", "bedrock:CountTokens"],
        "Resource": [
          "arn:aws:bedrock:${AWS_REGION}:${ACCOUNT_ID}:inference-profile/us.anthropic.*",
          "arn:aws:bedrock:*::foundation-model/anthropic.*"
        ]
      }]
    }
    EOF
    cat > ecs-trust.json <<'EOF'
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Principal": { "Service": "ecs-tasks.amazonaws.com" },
        "Action": "sts:AssumeRole"
      }]
    }
    EOF

    aws iam create-role --role-name claude-gateway-task \
      --assume-role-policy-document file://ecs-trust.json
    aws iam put-role-policy --role-name claude-gateway-task \
      --policy-name bedrock-invoke --policy-document file://bedrock-invoke.json
    ```

    ECS también necesita un rol de ejecución, que el agente de ECS mismo usa para extraer la imagen de ECR e inyectar los valores de Secrets Manager creados más adelante. Es separado del rol de tarea que el AWS SDK del gateway usa en tiempo de ejecución:

    ```bash theme={null}
    aws iam create-role --role-name claude-gateway-execution \
      --assume-role-policy-document file://ecs-trust.json
    aws iam attach-role-policy --role-name claude-gateway-execution \
      --policy-arn arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy
    cat > secrets-read.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["secretsmanager:GetSecretValue", "secretsmanager:DescribeSecret"],
        "Resource": [
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-jwt-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-oidc-client-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-postgres-url-??????"
        ]
      }]
    }
    EOF
    aws iam put-role-policy --role-name claude-gateway-execution \
      --policy-name read-gateway-secrets --policy-document file://secrets-read.json
    ```

    Los nombres de política especifican un ARN por secreto en lugar de un comodín simple `gateway-*`, que en una cuenta compartida también coincidiría con secretos no relacionados; el sufijo `-??????` al final coincide exactamente con el sufijo de seis caracteres aleatorios que Secrets Manager añade al ARN de cada secreto. Un `-*` al final sería un glob de prefijo simple y también coincidiría con nombres más largos como `gateway-postgres-url-prod`.

    La política de IAM otorga al gateway permiso para llamar a Bedrock, y Bedrock habilita el acceso al modelo de forma predeterminada en regiones comerciales. La puerta de nivel de cuenta restante es el formulario de caso de uso único de Anthropic: si nadie en su cuenta lo ha enviado, abra la [consola de Amazon Bedrock](https://console.aws.amazon.com/bedrock/), seleccione un modelo de Anthropic del catálogo de modelos y complete el formulario. El acceso se otorga inmediatamente después del envío; consulte [Claude Code en Amazon Bedrock](/docs/es/amazon-bedrock#1-submit-use-case-details) para el formulario de AWS Organizations y los permisos de IAM que el remitente necesita.

    La pista de EKS reutiliza ambos documentos de política en un rol de IRSA en lugar de los dos roles de ECS; consulte el paso de implementación.
  </Step>

  <Step title="Aprovisione Amazon RDS para PostgreSQL">
    La instancia se ejecuta en las subredes privadas sin dirección pública y con cifrado de almacenamiento activado. La versión del motor se fija en Postgres 16, que satisface el piso compatible del gateway de PostgreSQL 14 y garantiza que la familia del grupo de parámetros a continuación coincida con la instancia.

    Primero, cree el grupo de subredes que coloca la base de datos en las subredes privadas, y un grupo de parámetros con `rds.force_ssl=1` para que el servidor rechace las conexiones de texto sin formato. La versión del motor se fija una vez porque la familia del grupo de parámetros debe coincidir con la versión principal del motor que ejecuta la instancia:

    ```bash theme={null}
    aws rds create-db-subnet-group --db-subnet-group-name claude-gateway-db \
      --db-subnet-group-description "Claude gateway" --subnet-ids $PRIVATE_SUBNETS

    PG_VERSION=16
    PG_FAMILY="postgres${PG_VERSION}"
    aws rds create-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --db-parameter-group-family "$PG_FAMILY" \
      --description "Claude gateway - require TLS on every connection"
    aws rds modify-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --parameters "ParameterName=rds.force_ssl,ParameterValue=1,ApplyMethod=immediate"
    ```

    Luego cree la instancia con una contraseña maestra generada:

    ```bash theme={null}
    PGPASS="$(openssl rand -hex 24)"
    aws rds create-db-instance --db-instance-identifier claude-gateway-db \
      --engine postgres --engine-version "$PG_VERSION" \
      --db-instance-class db.t4g.micro \
      --allocated-storage 20 --db-name claude_gateway \
      --master-username gateway --master-user-password "$PGPASS" \
      --db-subnet-group-name claude-gateway-db \
      --db-parameter-group-name claude-gateway-db \
      --vpc-security-group-ids "$DB_SG" \
      --no-publicly-accessible --storage-encrypted
    ```

    El argumento literal `--master-user-password` es visible en la tabla de procesos y en registros de auditoría/EDR mientras se ejecuta el comando, la misma exposición que cubre la nota del paso de secretos. En un host compartido o monitoreado, pase la contraseña a través de `--cli-input-json` desde un archivo `0600` en su lugar, de la forma que lo hace `setup.sh` del paquete.

    Espere a que la instancia se levante, lo que puede tomar varios minutos, luego lea su endpoint privado y ensamble la cadena de conexión que usará el gateway:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` hace que el gateway verifique la cadena del certificado del servidor RDS y el nombre de host, no solo cifre. El ancla de confianza es el [paquete de certificados de AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), que el paso de compilación de imagen a continuación copia a `/etc/claude/rds-global-bundle.pem` y confía a través de `NODE_EXTRA_CA_CERTS`. No agregue un parámetro de estilo libpq `sslrootcert=` a la URL: el controlador del gateway lee solo `sslmode` de la cadena de consulta y reenviaría `sslrootcert` a Postgres como parámetro de inicio, que el servidor rechaza.

    El servicio de ECS o los pods de EKS deben ejecutarse en esta VPC para que puedan alcanzar el endpoint privado de la instancia, y el grupo de seguridad `claude-gateway-db` solo admite el grupo de seguridad del gateway.
  </Step>

  <Step title="Escriba gateway.yaml">
    El bloque `upstreams` apunta a Bedrock con `auth: {}`, por lo que el gateway se autentica a través de la cadena de credenciales predeterminada de AWS desde el rol de tarea en ECS o el rol de IRSA en EKS. Consulte la [referencia de configuración](/docs/es/claude-apps-gateway-config) para cada campo.

    Dos campos `listen` describen qué está delante del gateway:

    * `public_url`: el origen `https://` externo, requerido para cualquier enlace que no sea loopback; consulte la [referencia `listen`](/docs/es/claude-apps-gateway-config#listen). El gateway construye el `redirect_uri` del IdP y su documento de descubrimiento solo a partir de este valor, nunca a partir de encabezados `X-Forwarded-*`.
    * `trusted_proxies`: los rangos de origen del front end. El gateway honra `X-Forwarded-For` solo cuando el par TCP está en esta lista, luego camina la cadena pasada los saltos confiables, por lo que los límites de velocidad de inicio de sesión por IP y los eventos de auditoría registran las IP de los desarrolladores en lugar de la del equilibrador de carga.

    En ambas pistas el front end es un ALB interno, ya sea creado directamente o por el Controlador de Equilibrador de Carga de AWS, y los nodos de un ALB toman direcciones de las subredes a las que está adjunto, por lo que establezca `trusted_proxies` en los CIDR de esas subredes. Esto confía en cada host en esas subredes como un proxy. Mantenga la fuente de ingreso del ALB, su CIDR corporativo, sin superponerse a ellas, y no comparta las subredes con cargas de trabajo no confiables que podrían falsificar las IP de los clientes a través de `X-Forwarded-For`.

    El atributo de preservación del puerto del cliente del ALB, `routing.http.xff_client_port.enabled`, puede permanecer en cualquier configuración: con él activado, el ALB escribe el cliente como `203.0.113.7:54321` o `[2001:db8::1]:54321`, y el gateway lee ambos con el puerto descartado.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [<your-alb-subnet-cidrs>]

    oidc:
      issuer: https://example.okta.com
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}           # EKS: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # El servidor de autorización de la organización de Okta devuelve un id_token delgado que omite
      # correo electrónico y grupos; el gateway los completa desde /userinfo.
      userinfo_fallback: true
      # Okta emite grupos solo cuando se solicita el alcance `groups` y el
      # filtro de reclamación de grupos de la aplicación los permite.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # limita la latencia de desaprovisionamiento; baje
    # hacia 1 para una revocación más ajustada

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # coincida con $AWS_REGION para que los ARN de la política de IAM
    # lo cubran
        auth: {} # cadena de credenciales predeterminada de AWS:
    # rol de tarea de ECS, o IRSA en EKS
    ```

    <Note>
      Solo el bloque `oidc` es específico de Okta. Para usar Microsoft Entra ID en su lugar, establezca `issuer` en `https://login.microsoftonline.com/<tenant-id>/v2.0`, elimine `userinfo_fallback` y el alcance `groups`, y tenga en cuenta que Entra emite ID de objeto de grupo en lugar de nombres, por lo que [`managed.policies`](/docs/es/claude-apps-gateway-config#managed) debe coincidir en los GUID, o en Roles de Aplicación con `oidc.groups_claim: roles`. Consulte [Configuración del proveedor de identidad](/docs/es/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Almacene secretos en AWS Secrets Manager">
    Cree tres secretos; el rol de ejecución del paso de IAM ya puede leerlos:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Tenga en cuenta el ARN que imprime cada llamada; la definición de tarea de ECS hace referencia a los secretos por ARN.

    <Note>
      Los argumentos literales `--secret-string` son visibles en la tabla de procesos y en registros de auditoría/EDR mientras se ejecuta cada comando. En un host compartido o monitoreado, coloque el valor en un archivo `0600` y pase `--secret-string file://<path>` en su lugar. El `setup.sh` del paquete mantiene los valores secretos fuera de argv del proceso de la misma manera, pasando archivos temporales `0600` a `--cli-input-json`.
    </Note>

    A diferencia de los secretos, `gateway.yaml` en sí no contiene valores secretos, porque cada credencial se resuelve al arranque a través de [expansión `${VAR}` o `${file:...}`](/docs/es/claude-apps-gateway-config#secret-expansion). Cómo todo llega al contenedor difiere por pista:

    * En ECS, el paso siguiente copia `gateway.yaml` en la imagen en `/etc/claude/gateway.yaml`, y la definición de tarea inyecta los tres secretos como variables de entorno a través de su campo `secrets`, por lo que el YAML hace referencia a `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` y `${GATEWAY_POSTGRES_URL}`.
    * En EKS, monte `gateway.yaml` desde un ConfigMap y los secretos como archivos en `/secrets`, referenciados como `${file:/secrets/...}`. Obtenga los Secretos de Kubernetes de Secrets Manager con External Secrets Operator o el proveedor de AWS del controlador CSI de Secrets Store, o créelos directamente con `kubectl`.
  </Step>

  <Step title="Compile e inserte la imagen en Amazon ECR">
    Compile la imagen según los [requisitos de imagen de contenedor](/docs/es/claude-apps-gateway-deploy#container-image), colocando el binario glibc `linux-x64` en `./claude` en el contexto de compilación. Escriba su propio Dockerfile según esos requisitos o comience desde el [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) del paquete, que copia el `gateway.yaml` completado de los pasos anteriores en la imagen en `/etc/claude/gateway.yaml`. En ECS esa copia incrustada es cómo la configuración llega al contenedor, por eso la compilación viene después de que se escribe el archivo. La pista de EKS en su lugar monta `gateway.yaml` desde un ConfigMap en la implementación, por lo que la copia incrustada no se usa allí.

    La imagen también lleva el paquete de certificados de AWS RDS como el ancla de confianza para la cadena de conexión `sslmode=verify-full`, así que descárguelo en el contexto de compilación primero. AWS rota el paquete (se añaden nuevas CA regionales), así que descárguelo por compilación en lugar de fijar un checksum o confirmarlo:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Los requisitos de imagen de contenedor no cubren el paquete, así que si escribe su propio Dockerfile, agregue las dos líneas que lo copian y confían; el `Dockerfile` del paquete ya incluye ambas:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Cree el repositorio de ECR e inicie sesión en Docker. Las etiquetas inmutables significan que la etiqueta `<version>` que el paso de implementación fija no puede ser reapuntada silenciosamente a una imagen diferente más adelante:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Compile e inserte la imagen. La definición de tarea a continuación ejecuta `linux/amd64`, por lo que la plataforma debe coincidir aquí; para Fargate en ARM64 (Graviton), compile `linux/arm64` con el binario `linux-arm64` y establezca `cpuArchitecture` en `ARM64` en su lugar:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Implemente">
    <Tabs>
      <Tab title="ECS Fargate">
        Cree el clúster y un grupo de registros para stderr del gateway, que lleva tanto sus eventos de auditoría como registros operacionales. La retención es una llamada separada, y sin una CloudWatch mantiene los registros para siempre; alinee los 90 días con su política de retención de auditoría:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Escriba la definición de tarea. El rol de tarea lleva el permiso de Bedrock y el rol de ejecución inyecta los secretos; use los ARN de secretos del paso de Secrets Manager:

        ```json claude-gateway-task.json theme={null}
        {
          "family": "claude-gateway",
          "networkMode": "awsvpc",
          "requiresCompatibilities": ["FARGATE"],
          "cpu": "1024",
          "memory": "2048",
          "runtimePlatform": { "cpuArchitecture": "X86_64", "operatingSystemFamily": "LINUX" },
          "executionRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-execution",
          "taskRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-task",
          "containerDefinitions": [
            {
              "name": "gateway",
              "image": "<account-id>.dkr.ecr.<region>.amazonaws.com/claude-gateway:<version>",
              "portMappings": [{ "containerPort": 8080 }],
              "secrets": [
                { "name": "GATEWAY_JWT_SECRET",   "valueFrom": "<gateway-jwt-secret ARN>" },
                { "name": "OIDC_CLIENT_SECRET",   "valueFrom": "<gateway-oidc-client-secret ARN>" },
                { "name": "GATEWAY_POSTGRES_URL", "valueFrom": "<gateway-postgres-url ARN>" }
              ],
              "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                  "awslogs-group": "/ecs/claude-gateway",
                  "awslogs-region": "<region>",
                  "awslogs-stream-prefix": "gateway"
                }
              }
            }
          ]
        }
        ```

        Regístrelo:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Coloque un ALB interno al frente con un grupo de destino que verifique la salud del gateway. `--ip-address-type ipv4` importa: un ALB dual-stack interno publica registros AAAA de rango público, que la verificación de red privada de `/login` rechaza:

        ```bash theme={null}
        ALB_ARN="$(aws elbv2 create-load-balancer --name claude-gateway \
          --scheme internal --type application --ip-address-type ipv4 \
          --subnets $PRIVATE_SUBNETS --security-groups "$ALB_SG" \
          --query 'LoadBalancers[0].LoadBalancerArn' --output text)"

        TG_ARN="$(aws elbv2 create-target-group --name claude-gateway \
          --protocol HTTP --port 8080 --vpc-id "$VPC_ID" --target-type ip \
          --health-check-path /readyz \
          --query 'TargetGroups[0].TargetGroupArn' --output text)"
        ```

        Agregue el oyente HTTPS. `--ssl-policy` fija un piso TLS moderno, ya que omitirlo vuelve a la política predeterminada heredada `ELBSecurityPolicy-2016-08`, que aún acepta TLS 1.0/1.1.

        El ALB cierra una conexión después de 60 segundos sin datos de forma predeterminada. Los pings de keepalive del gateway mantienen los streams dentro de ese predeterminado, por lo que aumentar el tiempo de espera añade margen por encima de la cadencia de ping; la fila [Solución de problemas](#troubleshooting) sobre streams descartados cubre el mecanismo y gateways más antiguos. Los comandos a continuación añaden el oyente y aumentan el tiempo de espera:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Cree el servicio. El disyuntor de implementación revierte una implementación cuyos tareas siguen fallando, de una imagen mala o una configuración no arrancable, al último estado estable en lugar de relanzar tareas fallidas para siempre:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        El período de gracia de 60 segundos da a una tarea fría tiempo para extraer la imagen, conectarse al almacén y responder su primera verificación de salud antes de que ECS comience a contar fallos contra la implementación. La verificación de salud del grupo de destino en `GET /readyz` verifica que el almacén sea alcanzable, por lo que una tarea que no puede alcanzar Postgres nunca entra en rotación; consulte [Comportamiento de interrupción](/docs/es/claude-apps-gateway-deploy#outage-behavior) para el compromiso y la alternativa `/healthz`.

        Las tareas se ejecutan en subredes privadas sin IP pública, por lo que todo el tráfico saliente (a Bedrock, su IdP, Secrets Manager, ECR y CloudWatch Logs) va a través de la puerta de enlace NAT. Para mantener el tráfico de Bedrock fuera de la ruta pública, cree un endpoint de VPC de interfaz `bedrock-runtime` y apunte el `base_url` del upstream a él, como se muestra en la [referencia de upstream de Bedrock](/docs/es/claude-apps-gateway-config#amazon-bedrock); el IdP aún necesita salida a internet.

        Termine dando a los desarrolladores un nombre de host privadamente resoluble: en una zona alojada privada de Route 53, alias el nombre DNS interno del gateway al ALB, y establezca `listen.public_url` en ese nombre de host. El nombre `*.elb.amazonaws.com` propio del ALB se resuelve en direcciones privadas en un ALB interno, pero no puede llevar su certificado ACM, así que use su propio nombre.

        Actualice el URI de redirección autorizado del cliente OAuth a `<public_url>/oauth/callback` antes del primer inicio de sesión. Después de cambiar `public_url`, recompile e inserte la imagen bajo una etiqueta nueva, registre una nueva revisión de definición de tarea e reimplemente. En ECS la configuración vive en el `gateway.yaml` incrustado de la imagen, y el gateway construye su origen público solo a partir de esa configuración, ignorando `X-Forwarded-Host` y `X-Forwarded-Proto`. `X-Forwarded-For` se honra para las IP de los clientes solo cuando se establece `listen.trusted_proxies`.
      </Tab>

      <Tab title="EKS">
        Esta pista necesita `kubectl` y `eksctl` instalados localmente, y un clúster de EKS existente con un proveedor de OIDC de IAM y el Controlador de Equilibrador de Carga de AWS instalado. El clúster debe estar en `$VPC_ID` para que los pods puedan alcanzar el endpoint privado de RDS, y el grupo de seguridad `claude-gateway-db` debe admitir el grupo de seguridad del pod o nodo del clúster en lugar de `$GW_SG`.

        En EKS el gateway obtiene sus credenciales de Bedrock a través de IRSA en lugar de los roles de ECS. La política de confianza `ecs-tasks.amazonaws.com` del paso de IAM no se aplica aquí; IRSA necesita un rol cuya política de confianza se federe en el proveedor de OIDC del clúster, limitado a `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` crea ese rol, adjunta las políticas y anota la cuenta de servicio de Kubernetes con el ARN del rol en un paso. Convierta los dos documentos de política del paso de IAM en políticas administradas que pueda adjuntar:

        ```bash theme={null}
        BEDROCK_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-bedrock-invoke \
          --policy-document file://bedrock-invoke.json --query Policy.Arn --output text)"
        SECRETS_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-secrets-read \
          --policy-document file://secrets-read.json --query Policy.Arn --output text)"

        kubectl create namespace claude-gateway
        eksctl create iamserviceaccount --cluster <your-cluster> --region "$AWS_REGION" \
          --namespace claude-gateway --name gateway --role-name claude-gateway \
          --attach-policy-arn "$BEDROCK_POLICY_ARN" \
          --attach-policy-arn "$SECRETS_POLICY_ARN" \
          --approve
        ```

        La política de secretos es necesaria solo cuando los pods leen Secrets Manager ellos mismos, como lo hace el proveedor de AWS del controlador CSI de Secrets Store usando la cuenta de servicio del pod de montaje; elimínela si crea los Secretos de Kubernetes de otra manera. El proveedor necesita ambas acciones de la política: llama a `DescribeSecret` cuando reconcilia secretos rotados, por lo que una concesión de solo `GetSecretValue` monta en la primera implementación pero deja de recoger rotaciones.

        Implemente el gateway como un Deployment estándar más un Service e Ingress, como se describe en [Implementación de Kubernetes](/docs/es/claude-apps-gateway-deploy#kubernetes), con:

        * `serviceAccountName: gateway`
        * `gateway.yaml` montado desde un ConfigMap y los secretos montados en `/secrets`
        * la sonda de preparación apuntada a `GET /readyz`

        Para el front end, un Ingress administrado por el Controlador de Equilibrador de Carga de AWS aprovisiona el ALB interno. Anótelo con:

        * `alb.ingress.kubernetes.io/scheme: internal` y `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, para que no se publiquen registros AAAA de rango público para que la verificación de red privada de `/login` [rechace](/docs/es/claude-apps-gateway#prerequisites)
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, para que el grupo de seguridad frontend administrado por el controlador admita solo su red corporativa en lugar de su predeterminado `0.0.0.0/0`
        * `alb.ingress.kubernetes.io/certificate-arn` con el certificado ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, para que el oyente no vuelva a la política predeterminada heredada que acepta TLS 1.0 y 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, un margen por encima del keepalive de streaming del gateway; consulte [Solución de problemas](#troubleshooting)

        Con IRSA, el AWS SDK lee un token de cuenta de servicio proyectado e intercambia con AWS STS, por lo que el pod nunca necesita el servicio de metadatos de instancia de EC2; una NetworkPolicy de salida puede bloquear `169.254.169.254` para pods de gateway. El problema del límite de saltos de nodo en [Solución de problemas](#troubleshooting) a continuación se aplica solo a clústeres que omiten IRSA y se basan en roles de instancia de nodo.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Inserte la URL del gateway en las máquinas de los desarrolladores">
    El gateway ahora se está ejecutando, pero los desarrolladores no pueden alcanzarlo desde `/login` hasta que la URL del gateway esté en sus máquinas. Establezca `forceLoginMethod` y `forceLoginGatewayUrl` en el [archivo de configuración administrada](/docs/es/claude-apps-gateway#set-the-gateway-url) que implementa en cada dispositivo a través de MDM. No hay opción de gateway en el selector de inicio de sesión para que un desarrollador seleccione manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referencia de Terraform
</h2>

El paquete complementario en [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) empaqueta esta página como código:

* **`setup.sh`** secuencia el tutorial de aprovisionamiento anterior con los mismos comandos `aws`, en la pista de ECS Fargate. Es idempotente: los recursos existentes se detectan y se omiten, por lo que volver a ejecutarlo es seguro, y cualquier predeterminado se puede anular a través de variable de entorno. Aún crea el secreto del cliente OIDC de Okta y el certificado ACM usted mismo: una ejecución sin ellos omite la implementación de ECS/ALB, nombra las entradas faltantes e imprime el comando `create-secret`; cree ambos y vuelva a ejecutar. El formulario de caso de uso de Bedrock y el alias de Route 53 se imprimen como pasos siguientes en lugar de ejecutarse automáticamente, y el push de MDM del cliente permanece como un paso manual de esta página.
* **`gateway.yaml.example`** es la plantilla de configuración del paso gateway.yaml, con las claves opcionales incluidas comentadas. Cópielo a `gateway.yaml` y reemplace cada `REPLACE_ME` antes de compilar.
* **`Dockerfile`** compila la imagen de tiempo de ejecución desde el binario precompilado `linux-x64` y copia su `gateway.yaml` completado en `/etc/claude/gateway.yaml`, más el paquete de certificados de AWS RDS que ancla el `sslmode=verify-full` del almacén. `setup.sh` descarga el paquete solo cuando no está ya en el contexto de compilación; elimine el archivo y recompile bajo una etiqueta nueva para recoger una rotación de CA de AWS. El archivo de configuración no contiene valores secretos, ya que cada credencial se resuelve al arranque a través de expansión `${VAR}`. Una edición de configuración por lo tanto significa una recompilación bajo una etiqueta nueva; `setup.sh` automatiza esto etiquetando imágenes con un hash del archivo.
* **`terraform/`** aprovisiona el mismo alcance de ECS Fargate de forma declarativa: los grupos de seguridad, roles de IAM, repositorio de ECR, instancia de RDS, secretos de Secrets Manager y el servicio de ECS detrás del ALB interno. La VPC y las subredes privadas permanecen como requisitos previos, pasadas como variables. Terraform crea el repositorio de ECR pero no compila la imagen, y la definición del servicio hace referencia a la imagen, por lo que la aplicación es dos pasadas: una aplicación dirigida para el repositorio, luego la compilación e inserción, luego la aplicación completa. El `terraform/README.md` del paquete cubre las variables, estado remoto y desmontaje.

Como esta página, el paquete es un ejemplo funcional para infraestructura administrada por el cliente en lugar de una implementación de producción compatible; revíselo y adáptelo a su propio entorno antes de confiar en él.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para errores de arranque y inicio de sesión del gateway, consulte la tabla de [solución de problemas](/docs/es/claude-apps-gateway-deploy#troubleshooting) agnóstica de plataforma. Las entradas a continuación son específicas de AWS.

| Síntoma                                                                                                                                    | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Solución                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | El nombre del gateway se resuelve en al menos una dirección pública. Un ALB dual-stack interno publica registros AAAA de rango público, y la [verificación de red privada](/docs/es/claude-apps-gateway#prerequisites) requiere que cada dirección resuelta sea privada                                                                                                                                                                                                                                                                                                                                                                              | Cree el ALB con `--ip-address-type ipv4`, o sirva un nombre DNS interno separado sin registro AAAA público                                                                                                                                                                                                                                          |
| Cada solicitud de Bedrock devuelve 502; el registro muestra `Could not load credentials from any providers`                                | La tarea se ejecuta en el tipo de lanzamiento de ECS EC2 sin un rol de tarea, o el pod se ejecuta en un nodo de EKS sin IRSA, por lo que las credenciales provienen de metadatos de instancia, que el límite de saltos predeterminado de IMDSv2 de 1 detiene dentro de un contenedor. Ninguna pista en esta página se ve afectada: los roles de tarea de Fargate e IRSA no usan metadatos de instancia                                                                                                                                                                                                                                          | Prefiera roles de tarea e IRSA. Donde las credenciales de instancia son inevitables, aumente el límite de saltos con `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; la [tabla agnóstica de plataforma](/docs/es/claude-apps-gateway-deploy#troubleshooting) cubre los compromisos                        |
| Las solicitudes de Bedrock devuelven `403 AccessDeniedException`                                                                           | La cuenta no ha enviado el formulario de caso de uso único de Anthropic, la suscripción automática de AWS Marketplace que comienza en la primera invocación de la cuenta aún no ha terminado, o la política del rol de tarea carece de los ARN de perfil de inferencia o modelo base                                                                                                                                                                                                                                                                                                                                                            | Envíe el formulario de caso de uso desde el catálogo de modelos de la consola de Bedrock; si acaba de enviarse o esta es la primera invocación de la cuenta, reintente después de unos minutos. Otorgue `bedrock:InvokeModel` y `bedrock:InvokeModelWithResponseStream` en ambas familias de ARN.                                                   |
| Bedrock devuelve una `ValidationException` diciendo que el rendimiento bajo demanda no es compatible                                       | Una entrada `models:` personalizada se asigna a un ID de modelo base simple que la región sirve solo a través de perfiles de inferencia                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Asigne el modelo a su ID de perfil de inferencia entre regiones (`us.anthropic.*`) en su lugar; el catálogo integrado ya lo hace                                                                                                                                                                                                                    |
| La tarea de ECS se detiene con `ResourceInitializationError` antes de que el gateway registre algo                                         | El rol de ejecución no puede leer los secretos de Secrets Manager, o las subredes privadas no tienen ruta a Secrets Manager o ECR                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Otorgue `secretsmanager:GetSecretValue` en los ARN de los tres secretos `gateway-` al rol de ejecución, y proporcione salida a través de la puerta de enlace NAT, o, sin una, endpoints de interfaz para Secrets Manager, ECR y CloudWatch Logs, que el controlador `awslogs` necesita en la misma etapa, más un endpoint de puerta de enlace de S3 |
| El arranque del gateway sale con un error de tiempo de espera de conexión de Postgres                                                      | El grupo de seguridad de la base de datos no admite el grupo de seguridad del gateway en 5432, o el servicio se ejecuta fuera de la VPC de la base de datos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Permita 5432 desde el grupo de seguridad del gateway en el de la base de datos, y ejecute el servicio en la misma VPC que el grupo de subredes de la base de datos                                                                                                                                                                                  |
| El arranque del gateway sale con un error de verificación de certificado TLS de Postgres                                                   | La cadena de conexión establece `sslmode=verify-full` pero la imagen no confía en el paquete de CA de RDS: el paquete no se copió en la imagen, o `NODE_EXTRA_CA_CERTS` no apunta a él                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Agregue las dos líneas de Dockerfile del paso de compilación que copian el paquete y establecen `NODE_EXTRA_CA_CERTS`, luego recompile, inserte bajo una etiqueta nueva e reimplemente                                                                                                                                                              |
| Las respuestas de streaming se interrumpen a mitad de stream después de un período tranquilo                                               | Un gateway más antiguo que v2.1.229 en un upstream de Bedrock o Claude Platform on AWS no envía nada mientras el upstream está tranquilo, por ejemplo durante pensamiento extendido sin salida transmitida. El ALB cierra una conexión después de 60 segundos sin datos de forma predeterminada, por lo que corta el stream en esa brecha. Los gateways v2.1.229 y posteriores mantienen un stream tranquilo bajo ese tiempo de espera: en esos upstreams el gateway emite un evento SSE `ping` una vez que pasan aproximadamente 15 segundos sin datos de stream, y en un upstream de API de Anthropic retransmite los pings propios de la API | Actualice el gateway a v2.1.229 o posterior, o establezca el atributo `idle_timeout.timeout_seconds` en `3600`, a través de `modify-load-balancer-attributes` o la anotación `load-balancer-attributes` de Ingress en EKS                                                                                                                           |

<h2 id="telemetry">
  Telemetría
</h2>

El gateway le proporciona métricas de uso por desarrollador sin ninguna configuración de OTEL por máquina. Claude Code emite métricas, registros y trazas de OpenTelemetry (OTLP) opcionales; [Monitoreo de uso](/docs/es/monitoring-usage) cubre todo lo que el CLI reporta. En sesiones de gateway el CLI marca cada exportación con los atributos de identidad del IdP autenticado `user.id`, `user.email` y `user.groups`, por lo que el uso se acumula por desarrollador sin ningún cableado de `OTEL_RESOURCE_ATTRIBUTES`.

El gateway en sí es un relé OTLP autenticado. Establezca [`telemetry.forward_to`](/docs/es/claude-apps-gateway-config#telemetry) junto con `listen.public_url`, e inserta la configuración del exportador OTEL en cada cliente conectado y reenvía su tráfico OTLP verbatim a cada destino que enumere. Cada destino se suscribe a métricas, registros y trazas de forma independiente, y el predeterminado es solo métricas; consulte la [referencia `telemetry`](/docs/es/claude-apps-gateway-config#telemetry) para los campos por señal y sus compromisos de sensibilidad. El gateway no almacena en búfer, agrega ni almacena telemetría, por lo que dónde aterrizan los datos es enteramente la configuración del exportador del recopilador.

La telemetría del cliente está desactivada de forma predeterminada; configurar `telemetry.forward_to` es lo que la activa para desarrolladores conectados, y cada cliente interactivo muestra un diálogo de aprobación de seguridad único para la configuración insertada, como se describe en la [referencia de configuración](/docs/es/claude-apps-gateway-config#telemetry). En AWS, cada señal se asigna a un destino de la siguiente manera.

<h3 id="client-metrics-logs-and-traces">
  Métricas, registros y trazas del cliente
</h3>

Apunte `telemetry.forward_to` a un recopilador de OpenTelemetry, como el [recopilador de AWS Distro for OpenTelemetry (ADOT)](https://aws-otel.github.io/), y exporte desde allí a Amazon CloudWatch, Amazon Managed Service for Prometheus, o cualquier backend de OTLP.

Ejecute el recopilador como su propio servicio interno alcanzable sobre `https://`; la [referencia `telemetry`](/docs/es/claude-apps-gateway-config#telemetry) cubre la excepción de loopback y `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Registros del gateway
</h3>

En ECS Fargate, sin configuración adicional: el controlador `awslogs` entrega stderr del gateway, que lleva sus eventos de auditoría y registros operacionales, al grupo de registros `/ecs/claude-gateway` creado anteriormente. En EKS, los registros de pod no llegan a CloudWatch de forma predeterminada, por lo que el rastro de auditoría se pierde hasta que instale recopilación: el complemento de Observabilidad de Amazon CloudWatch con captura de registros de contenedor habilitada, o un DaemonSet de Fluent Bit. En cualquier pista, consulte los registros con CloudWatch Logs Insights e impulse alarmas desde filtros de métricas.

<h3 id="container-metrics">
  Métricas de contenedor
</h3>

Habilite Container Insights en el clúster con `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` para CPU, memoria y red por tarea. En EKS, instale el complemento de Observabilidad de Amazon CloudWatch.

<h3 id="spend">
  Gasto
</h3>

La telemetría muestra el uso después del hecho; los [límites de gasto](/docs/es/claude-apps-gateway-spend-limits) son la vista en vivo del gateway y la aplicación por desarrollador en la parte superior de la credencial upstream compartida.

<h2 id="next-steps">
  Pasos siguientes
</h2>

* [Referencia de configuración](/docs/es/claude-apps-gateway-config): cada opción de `gateway.yaml`, incluyendo `managed.policies` y `telemetry`
* [Implementación y operaciones](/docs/es/claude-apps-gateway-deploy): configuración de IdP, verificaciones de salud, rotación de secretos JWT, actualizaciones y el modelo de seguridad
* [Descripción general de Claude apps gateway](/docs/es/claude-apps-gateway): inicio rápido y conexión de desarrolladores
* [Ejemplos de AWS para Claude apps gateway](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): ejemplos de implementación mantenidos por AWS que cubren una variedad de entornos de clientes
