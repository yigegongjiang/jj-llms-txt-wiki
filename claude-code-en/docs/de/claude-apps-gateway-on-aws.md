> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude-Apps-Gateway auf AWS bereitstellen

> Ein praktisches Beispiel für die Ausführung von Claude-Apps-Gateway auf AWS: ECS Fargate oder EKS, Amazon RDS für PostgreSQL, AWS Secrets Manager und IAM-rollenbasierte Authentifizierung bei Amazon Bedrock.

<Note>
  Diese Seite zeigt eine Möglichkeit, Claude-Apps-Gateway auf AWS auszuführen. Die Konfiguration ist ein funktionierendes Beispiel für kundenverwaltete Infrastruktur und keine unterstützte Produktionsbereitstellung. Nutzen Sie sie, um zu verstehen, wie die einzelnen Komponenten zusammenpassen, bevor Sie sie an Ihre eigene Umgebung anpassen. Für die plattformunabhängigen Anforderungen siehe den [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy).
</Note>

Dieses Beispiel stellt Claude-Apps-Gateway auf AWS mit Amazon Bedrock als Modell-Upstream bereit und nutzt entweder [Amazon ECS](https://aws.amazon.com/ecs/) auf [AWS Fargate](https://aws.amazon.com/fargate/) oder [Amazon EKS](https://aws.amazon.com/eks/) für die Berechnung. [Okta](https://www.okta.com/) ist der Beispiel-Identitätsanbieter (IdP), aber jeder OpenID Connect (OIDC) konforme IdP funktioniert. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für Details pro IdP.

<Note>
  Bedrock ist nicht der einzige Claude-Upstream auf AWS. Das Gateway unterstützt auch Claude Platform on AWS, die von Anthropic betriebene Claude-API mit AWS-Authentifizierung und AWS-Marketplace-Abrechnung, anstelle von Bedrock oder neben ihm. Der Upstream-Eintrag, die Anmeldedaten und die IAM-Berechtigungen unterscheiden sich von den auf dieser Seite beschriebenen Bedrock-spezifischen; die [Claude Platform on AWS Upstream-Referenz](/docs/de/claude-apps-gateway-config#claude-platform-on-aws) behandelt, was sich ändert, und der Rest dieser Seite gilt unverändert.
</Note>

<h2 id="architecture">
  Architektur
</h2>

<Frame caption="Die Beispielarchitektur mit Amazon Bedrock als Modell-Upstream. Ein Claude Platform on AWS Upstream nimmt die gleiche Position ein.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagramm von Claude-Apps-Gateway auf AWS: Claude Code Clients verbinden sich über HTTPS mit einem internen Application Load Balancer, der das Gateway (ECS Fargate oder EKS) frontet, das in privaten Subnetzen neben einer Amazon RDS für PostgreSQL Instanz für Sitzungszustand läuft. Das Gateway meldet Benutzer über OIDC gegen den Unternehmens-IdP an, liest Geheimnisse aus AWS Secrets Manager, leitet Modellanfragen an Amazon Bedrock mit seiner IAM-Rolle weiter und zieht sein Image bei der Bereitstellung aus Amazon ECR." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

Das Gateway läuft als privater HTTPS-Endpunkt in Ihrem Netzwerk, bei dem sich Entwickler über Ihren IdP anmelden. Ihre Claude Code Sitzungen erreichen Claude-Modelle auf Amazon Bedrock über die IAM-Rolle des Gateways, sodass keine Modellanmeldedaten auf Entwicklermaschinen landen. Die Referenzkonfiguration stellt bereit:

* **Amazon ECS auf AWS Fargate** Service oder **Amazon EKS** Deployment, das den Gateway-Container ausführt
* **Amazon ECR** Repository für das Gateway-Image
* **Amazon RDS für PostgreSQL** Instanz in privaten Subnetzen, nicht öffentlich zugänglich, für den [Store](/docs/de/claude-apps-gateway-config#store) des Gateways
* **AWS Secrets Manager** Geheimnisse für den JWT-Signaturschlüssel, das OIDC-Client-Geheimnis und die Postgres-URL
* **IAM-Rolle** mit `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` und `bedrock:CountTokens`, angehängt als ECS-Task-Rolle oder gebunden über IAM Roles for Service Accounts (IRSA) auf EKS
* **Interner Application Load Balancer** für HTTPS

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Die Anleitung erstellt die eigenen Ressourcen des Gateways, basiert aber auf Netzwerk- und Identitätsinfrastruktur, die Sie bereits haben. Bevor Sie beginnen, benötigen Sie:

* Ein AWS-Konto mit Berechtigung zum Erstellen der [oben genannten Ressourcen](#architecture)
* Die [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) installiert und [authentifiziert](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), sowie [Docker](https://docs.docker.com/get-started/get-docker/) lokal installiert
* Ein [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) mit mindestens zwei [privaten Subnetzen](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) in verschiedenen Verfügbarkeitszonen mit ausgehendem Internetzugang über ein [NAT-Gateway](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); der interne Load Balancer benötigt Subnetze in zwei AZs, und das Gateway benötigt Egress zu Bedrock und Ihrem IdP
* Eine Okta OIDC-Webanwendung mit Redirect-URI `https://<gateway-host>/oauth/callback`; siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup)
* Ein TLS-Hostname für das Gateway, typischerweise ein interner DNS-Name in einer [Route 53 privaten gehosteten Zone](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html), der auf den Load Balancer zeigt, mit einem [ACM-Zertifikat](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) für diesen Namen, importiert oder ausgestellt von [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Legen Sie Ihre Umgebungsvariablen fest
</h3>

Jeder Befehl auf dieser Seite liest vier Werte aus Ihrer Shell: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` und `PRIVATE_SUBNETS`.

Wählen Sie eine US-Region, in der Bedrock die Claude-Modelle bereitstellt, die Sie benötigen. Die Anleitung basiert auf dem integrierten Modellkatalog des Gateways, der zu `us.anthropic.*` Inferenzprofilen aufgelöst wird, und die IAM-Richtlinie gewährt diese ARNs. In einer nicht-US-Region fügen Sie einen [`models:` Block](/docs/de/claude-apps-gateway-config#models) mit den Inferenzprofil-IDs dieser Region hinzu und ändern das ARN-Präfix der IAM-Richtlinie entsprechend.

Wenn Sie die VPC-ID nicht zur Hand haben, listen Sie Ihre VPCs mit `aws ec2 describe-vpcs` auf und listen Sie dann die Subnetze dieser VPC auf, um zwei private in verschiedenen Verfügbarkeitszonen zu finden:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Exportieren Sie alle vier, bevor Sie fortfahren:

```bash theme={null}
export AWS_REGION=us-east-1   # eine US-Region, in der Bedrock die Claude-Modelle bereitstellt, die Sie benötigen
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Stellen Sie das Gateway bereit
</h2>

Die folgenden Schritte stellen die vollständige Bereitstellung mit `aws` Befehlen bereit.

<Steps>
  <Step title="Erstellen Sie die Sicherheitsgruppen">
    Drei Sicherheitsgruppen verketten den Verkehrspfad: Ihr Unternehmensnetzwerk erreicht den Load Balancer auf 443, der Load Balancer erreicht das Gateway auf 8080, und das Gateway erreicht Postgres auf 5432. Nichts anderes ist erreichbar. Wie Sie sie anhängen, hängt vom Compute-Pfad ab:

    * Auf ECS Fargate hängt der Bereitstellungsschritt `$ALB_SG` an den Load Balancer und `$GW_SG` an den Service an.
    * Auf EKS erstellt der AWS Load Balancer Controller seine eigene Frontend-Sicherheitsgruppe für den ALB, daher werden `$ALB_SG` und `$GW_SG` nicht verwendet: die Annotation `inbound-cidrs` des Bereitstellungsschritts beschränkt den Listener auf Ihr Unternehmensnetzwerk, und die Datenbanksicherheitsgruppe lässt stattdessen die Sicherheitsgruppe des Clusters zu.

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

  <Step title="Erstellen Sie die IAM-Rollen und reichen Sie das Use-Case-Formular ein">
    Das Gateway läuft mit einer dedizierten Task-Rolle, deren einzige Berechtigung das Aufrufen von Claude-Modellen auf Bedrock ist. Gemäß der [Bedrock Upstream-Referenz](/docs/de/claude-apps-gateway-config#amazon-bedrock) muss die Richtlinie sowohl die Cross-Region-Inferenzprofil-ARNs als auch die zugrunde liegenden Foundation-Model-ARNs abdecken:

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

    ECS benötigt auch eine Ausführungsrolle, die der ECS-Agent selbst verwendet, um das Image aus ECR zu ziehen und die später erstellten Secrets Manager-Werte einzuspritzen. Sie ist getrennt von der Task-Rolle, die das Gateway zur Laufzeit mit dem AWS SDK verwendet:

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

    Die Richtlinie nennt eine ARN pro Geheimnis statt eines bloßen `gateway-*` Wildcards, das in einem gemeinsamen Konto auch nicht verwandte Geheimnisse abgleichen würde; das nachfolgende `-??????` gleicht genau das zufällige sechsstellige Suffix ab, das Secrets Manager an jede Geheimnis-ARN anhängt. Ein nachfolgendes `-*` wäre ein einfaches Präfix-Glob und würde auch längere Namen wie `gateway-postgres-url-prod` abgleichen.

    Die IAM-Richtlinie gewährt dem Gateway die Berechtigung, Bedrock aufzurufen, und Bedrock ermöglicht den Modellzugriff standardmäßig in kommerziellen Regionen. Das verbleibende Konto-Level-Gate ist Anthropics einmaliges Use-Case-Formular: Wenn niemand in Ihrem Konto es eingereicht hat, öffnen Sie die [Amazon Bedrock Konsole](https://console.aws.amazon.com/bedrock/), wählen Sie ein Anthropic-Modell aus dem Modellkatalog und füllen Sie das Formular aus. Der Zugriff wird unmittelbar nach der Einreichung gewährt; siehe [Claude Code auf Amazon Bedrock](/docs/de/amazon-bedrock#1-submit-use-case-details) für das AWS Organizations Formular und die IAM-Berechtigungen, die der Einreicher benötigt.

    Der EKS-Pfad verwendet beide Richtliniendokumente stattdessen auf einer IRSA-Rolle anstelle der zwei ECS-Rollen; siehe den Bereitstellungsschritt.
  </Step>

  <Step title="Stellen Sie Amazon RDS für PostgreSQL bereit">
    Die Instanz läuft in den privaten Subnetzen ohne öffentliche Adresse und mit aktivierter Speicherverschlüsselung. Die Engine-Version ist auf Postgres 16 festgelegt, was den unterstützten Boden des Gateways von PostgreSQL 14 erfüllt und garantiert, dass die Parametergruppe unten mit der Instanz übereinstimmt.

    Erstellen Sie zunächst die Subnet-Gruppe, die die Datenbank in den privaten Subnetzen platziert, und eine Parametergruppe mit `rds.force_ssl=1`, damit der Server Klartextverbindungen ablehnt. Die Engine-Version ist einmal festgelegt, da die Parametergruppen-Familie mit der Engine-Hauptversion übereinstimmen muss, die die Instanz ausführt:

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

    Erstellen Sie dann die Instanz mit einem generierten Master-Passwort:

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

    Das Literal `--master-user-password` Argument ist in der Prozesstabelle und in Audit-/EDR-Protokollen sichtbar, während der Befehl ausgeführt wird, die gleiche Exposition, die der Geheimnisse-Schritt behandelt. Auf einem gemeinsamen oder überwachten Host übergeben Sie das Passwort stattdessen über `--cli-input-json` aus einer `0600` Datei, wie es das `setup.sh` des Bundles tut.

    Warten Sie, bis die Instanz hochfährt, was mehrere Minuten dauern kann, lesen Sie dann ihren privaten Endpunkt und stellen Sie die Verbindungszeichenfolge zusammen, die das Gateway verwendet:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` lässt das Gateway das RDS-Serverzertifikat und den Hostnamen überprüfen, nicht nur verschlüsseln. Der Vertrauensanker ist das [AWS RDS Zertifikat-Bundle](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), das der Image-Build-Schritt unten zu `/etc/claude/rds-global-bundle.pem` kopiert und über `NODE_EXTRA_CA_CERTS` vertraut. Hängen Sie keinen libpq-Stil `sslrootcert=` Parameter an die URL an: Der Gateway-Treiber liest nur `sslmode` aus der Abfragezeichenfolge und würde `sslrootcert` als Startup-Parameter an Postgres weiterleiten, das der Server ablehnt.

    Der ECS-Service oder die EKS-Pods müssen in diesem VPC ausgeführt werden, damit sie den privaten Endpunkt der Instanz erreichen können, und die `claude-gateway-db` Sicherheitsgruppe lässt nur die Sicherheitsgruppe des Gateways zu.
  </Step>

  <Step title="Schreiben Sie gateway.yaml">
    Der `upstreams` Block zeigt auf Bedrock mit `auth: {}`, daher authentifiziert sich das Gateway über die AWS-Standard-Anmeldekette aus der Task-Rolle auf ECS oder der IRSA-Rolle auf EKS. Siehe die [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config) für jedes Feld.

    Zwei `listen` Felder beschreiben, was das Gateway frontet:

    * `public_url`: die externe `https://` Herkunft, erforderlich für jeden nicht-Loopback-Bind; siehe die [`listen` Referenz](/docs/de/claude-apps-gateway-config#listen). Das Gateway erstellt den IdP `redirect_uri` und sein Discovery-Dokument nur aus diesem Wert, niemals aus `X-Forwarded-*` Headern.
    * `trusted_proxies`: die Quellbereiche des Front-End. Das Gateway berücksichtigt `X-Forwarded-For` nur, wenn der TCP-Peer in dieser Liste ist, geht dann die Kette über vertrauenswürdige Hops, sodass Anmelderate-Limits pro IP und Audit-Events Entwickler-IPs statt der Load-Balancer-IP aufzeichnen.

    Auf beiden Pfaden ist das Front-End ein interner ALB, ob direkt erstellt oder vom AWS Load Balancer Controller, und ALB-Knoten nehmen Adressen aus den Subnetzen, an die sie angehängt sind, daher setzen Sie `trusted_proxies` auf die CIDRs dieser Subnetze. Dies vertraut jedem Host in diesen Subnetzen als Proxy. Halten Sie die Ingress-Quelle des ALB, Ihre Unternehmens-CIDR, davon ab, sich zu überlappen, und teilen Sie die Subnetze nicht mit nicht vertrauenswürdigen Workloads, die Client-IPs über `X-Forwarded-For` fälschen könnten.

    Das ALB-Attribut zur Beibehaltung des Client-Ports, `routing.http.xff_client_port.enabled`, kann bei beiden Einstellungen bleiben: Wenn es aktiviert ist, schreibt der ALB den Client als `203.0.113.7:54321` oder `[2001:db8::1]:54321`, und das Gateway liest beide mit dem Port gelöscht.

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
      # Der Okta-Org-Autorisierungsserver gibt ein dünnes id_token zurück, das
      # E-Mail und Gruppen auslässt; das Gateway füllt sie aus /userinfo.
      userinfo_fallback: true
      # Okta gibt Gruppen nur aus, wenn der `groups` Scope angefordert wird und
      # der Gruppen-Anspruchsfilter der App sie zulässt.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # begrenzt die Deprovisionierungs-Latenz; senken Sie
    # gegen 1 für straffere Sperrung

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # stimmen Sie mit $AWS_REGION überein, damit die IAM
    # Richtlinien-ARNs es abdecken
        auth: {} # AWS Standard-Anmeldekette:
    # ECS Task-Rolle oder IRSA auf EKS
    ```

    <Note>
      Nur der `oidc` Block ist Okta-spezifisch. Um stattdessen Microsoft Entra ID zu verwenden, setzen Sie `issuer` auf `https://login.microsoftonline.com/<tenant-id>/v2.0`, lassen Sie `userinfo_fallback` und den `groups` Scope weg, und beachten Sie, dass Entra Gruppen-Objekt-IDs statt Namen ausgibt, daher müssen [`managed.policies`](/docs/de/claude-apps-gateway-config#managed) auf den GUIDs abgleichen, oder auf App-Rollen mit `oidc.groups_claim: roles`. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Speichern Sie Geheimnisse in AWS Secrets Manager">
    Erstellen Sie drei Geheimnisse; die Ausführungsrolle aus dem IAM-Schritt kann sie bereits lesen:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Beachten Sie die ARN, die jeder Aufruf ausgibt; die ECS-Task-Definition referenziert Geheimnisse nach ARN.

    <Note>
      Literal `--secret-string` Argumente sind in der Prozesstabelle und in Audit-/EDR-Protokollen sichtbar, während jeder Befehl ausgeführt wird. Auf einem gemeinsamen oder überwachten Host legen Sie den Wert in eine `0600` Datei und übergeben Sie stattdessen `--secret-string file://<path>`. Das `setup.sh` des Bundles hält Geheimniswerte auf die gleiche Weise aus dem Prozess-argv, indem es `0600` temporäre Dateien an `--cli-input-json` übergibt.
    </Note>

    Im Gegensatz zu den Geheimnissen enthält `gateway.yaml` selbst keine Geheimniswerte, da jede Anmeldedaten beim Start über [`${VAR}` oder `${file:...}` Erweiterung](/docs/de/claude-apps-gateway-config#secret-expansion) aufgelöst wird. Wie alles den Container erreicht, unterscheidet sich je nach Pfad:

    * Auf ECS kopiert der Build des nächsten Schritts `gateway.yaml` in das Image bei `/etc/claude/gateway.yaml`, und die Task-Definition injiziert die drei Geheimnisse als Umgebungsvariablen über sein `secrets` Feld, daher referenziert die YAML `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` und `${GATEWAY_POSTGRES_URL}`.
    * Auf EKS mounten Sie `gateway.yaml` aus einer ConfigMap und die Geheimnisse als Dateien bei `/secrets`, referenziert als `${file:/secrets/...}`. Beziehen Sie die Kubernetes Secrets aus Secrets Manager mit dem External Secrets Operator oder dem AWS-Provider des Secrets Store CSI-Treibers, oder erstellen Sie sie direkt mit `kubectl`.
  </Step>

  <Step title="Erstellen Sie das Image und pushen Sie es zu Amazon ECR">
    Erstellen Sie das Image gemäß den [Container-Image-Anforderungen](/docs/de/claude-apps-gateway-deploy#container-image), wobei Sie die `linux-x64` glibc-Binärdatei bei `./claude` im Build-Kontext platzieren. Schreiben Sie Ihr eigenes Dockerfile gemäß diesen Anforderungen oder beginnen Sie mit dem [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) des Bundles, das die ausgefüllte `gateway.yaml` aus den vorherigen Schritten in das Image bei `/etc/claude/gateway.yaml` kopiert. Auf ECS ist diese eingebettete Kopie, wie die Konfiguration den Container erreicht, weshalb der Build nach dem Schreiben der Datei kommt. Der EKS-Pfad mountet stattdessen `gateway.yaml` aus einer ConfigMap bei der Bereitstellung, daher ist die eingebettete Kopie dort ungenutzt.

    Das Image trägt auch das AWS RDS Zertifikat-Bundle als Vertrauensanker für das `sslmode=verify-full` der Verbindungszeichenfolge, daher laden Sie es zunächst in den Build-Kontext herunter. AWS rotiert das Bundle (neue regionale CAs werden angehängt), daher laden Sie es pro Build herunter, statt einen Checksum zu pinnen oder es zu committen:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Die Container-Image-Anforderungen decken das Bundle nicht ab, daher müssen Sie, wenn Sie Ihr eigenes Dockerfile schreiben, die zwei Zeilen hinzufügen, die es kopieren und vertrauen; das `Dockerfile` des Bundles enthält bereits beide:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Erstellen Sie das ECR-Repository und melden Sie Docker darin an. Unveränderliche Tags bedeuten, dass das `<version>` Tag, das der Bereitstellungsschritt pinnt, später nicht stillschweigend auf ein anderes Image umgeleitet werden kann:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Erstellen und pushen Sie das Image. Die Task-Definition unten führt `linux/amd64` aus, daher muss die Plattform hier übereinstimmen; für Fargate auf ARM64 (Graviton) erstellen Sie `linux/arm64` mit der `linux-arm64` Binärdatei und setzen Sie `cpuArchitecture` stattdessen auf `ARM64`:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Bereitstellen">
    <Tabs>
      <Tab title="ECS Fargate">
        Erstellen Sie den Cluster und eine Log-Gruppe für die stderr des Gateways, die sowohl seine Audit-Events als auch Betriebsprotokolle trägt. Die Aufbewahrung ist ein separater Aufruf, und ohne eine CloudWatch behält die Protokolle für immer; richten Sie die 90 Tage auf Ihre Audit-Aufbewahrungsrichtlinie aus:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Schreiben Sie die Task-Definition. Die Task-Rolle trägt die Bedrock-Berechtigung und die Ausführungsrolle injiziert die Geheimnisse; verwenden Sie die Geheimnis-ARNs aus dem Secrets Manager-Schritt:

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

        Registrieren Sie es:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Setzen Sie einen internen ALB davor mit einer Zielgruppe, die das Gateway health-checkt. `--ip-address-type ipv4` ist wichtig: Ein interner Dual-Stack-ALB veröffentlicht öffentliche AAAA-Datensätze, die die `/login` private-Netzwerk-Prüfung ablehnt:

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

        Fügen Sie den HTTPS-Listener hinzu. `--ssl-policy` pinnt einen modernen TLS-Boden, da das Weglassen auf die Legacy-Standard-Richtlinie `ELBSecurityPolicy-2016-08` zurückfällt, die immer noch TLS 1.0/1.1 akzeptiert.

        Der ALB schließt eine Verbindung nach 60 Sekunden ohne Daten standardmäßig. Die Keepalive-Pings des Gateways halten Streams innerhalb dieses Standards, daher erhöht das Erhöhen des Timeouts die Marge über der Ping-Kadenz; die [Troubleshooting](#troubleshooting) Zeile auf abgebrochenen Streams behandelt den Mechanismus und ältere Gateways. Die folgenden Befehle fügen den Listener hinzu und erhöhen das Timeout:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Erstellen Sie den Service. Der Deployment-Schalter rollt eine Bereitstellung, deren Tasks weiterhin fehlschlagen, von einem schlechten Image oder einer nicht bootfähigen Konfiguration, zurück zum letzten stabilen Zustand, statt fehlgeschlagene Tasks für immer neu zu starten:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        Die 60-Sekunden-Gnadenfrist gibt einer kalten Task Zeit, das Image zu ziehen, sich mit dem Store zu verbinden und seinen ersten Health-Check zu beantworten, bevor ECS beginnt, Fehler gegen die Bereitstellung zu zählen. Der Health-Check der Zielgruppe auf `GET /readyz` überprüft, ob der Store erreichbar ist, daher kommt eine Task, die Postgres nicht erreichen kann, nie in Rotation; siehe [Ausfallverhalten](/docs/de/claude-apps-gateway-deploy#outage-behavior) für den Tradeoff und die `/healthz` Alternative.

        Die Tasks laufen in privaten Subnetzen ohne öffentliche IP, daher geht der gesamte Egress (zu Bedrock, Ihrem IdP, Secrets Manager, ECR und CloudWatch Logs) durch das NAT-Gateway. Um Bedrock-Verkehr vom öffentlichen Pfad zu halten, erstellen Sie einen `bedrock-runtime` Interface VPC-Endpunkt und zeigen Sie die `base_url` des Upstream darauf, wie in der [Bedrock Upstream-Referenz](/docs/de/claude-apps-gateway-config#amazon-bedrock) gezeigt; der IdP benötigt immer noch Internet-Egress.

        Beenden Sie, indem Sie Entwicklern einen privat auflösbaren Hostnamen geben: In einer Route 53 privaten gehosteten Zone, alias den internen DNS-Namen des Gateways zum ALB, und setzen Sie `listen.public_url` auf diesen Hostnamen. Der eigene `*.elb.amazonaws.com` Name des ALB wird zu privaten Adressen auf einem internen ALB aufgelöst, kann aber Ihr ACM-Zertifikat nicht tragen, daher verwenden Sie Ihren eigenen Namen.

        Aktualisieren Sie die autorisierte Redirect-URI des OAuth-Clients auf `<public_url>/oauth/callback`, bevor die erste Anmeldung. Nach dem Ändern von `public_url` erstellen Sie das Image unter einem neuen Tag neu, registrieren Sie eine neue Task-Definition-Revision und stellen Sie erneut bereit. Auf ECS lebt die Einstellung in der eingebetteten `gateway.yaml` des Images, und das Gateway erstellt seinen öffentlichen Ursprung nur aus dieser Einstellung, ignoriert `X-Forwarded-Host` und `X-Forwarded-Proto`. `X-Forwarded-For` wird nur berücksichtigt, wenn `listen.trusted_proxies` gesetzt ist.
      </Tab>

      <Tab title="EKS">
        Dieser Pfad benötigt `kubectl` und `eksctl` lokal installiert, und einen bestehenden EKS-Cluster mit einem IAM OIDC-Provider und dem AWS Load Balancer Controller installiert. Der Cluster muss auf `$VPC_ID` sein, damit Pods den RDS-Privatendpunkt erreichen können, und die `claude-gateway-db` Sicherheitsgruppe muss die Sicherheitsgruppe des Clusters oder des Pods des Clusters anstelle von `$GW_SG` zulassen.

        Auf EKS erhält das Gateway seine Bedrock-Anmeldedaten über IRSA statt der ECS-Rollen. Die `ecs-tasks.amazonaws.com` Vertrauensrichtlinie aus dem IAM-Schritt gilt hier nicht; IRSA benötigt eine Rolle, deren Vertrauensrichtlinie auf dem OIDC-Provider des Clusters föderiert ist, begrenzt auf `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` erstellt diese Rolle, hängt die Richtlinien an und kommentiert das Kubernetes-Dienstkonto mit der Rollen-ARN in einem Schritt. Verwandeln Sie die zwei Richtliniendokumente aus dem IAM-Schritt in verwaltete Richtlinien, die es anhängen kann:

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

        Die Geheimnisse-Richtlinie wird nur benötigt, wenn die Pods Secrets Manager selbst lesen, wie es der AWS-Provider des Secrets Store CSI-Treibers mit dem Dienstkonto des Mounting-Pods tut; lassen Sie sie weg, wenn Sie die Kubernetes Secrets auf andere Weise erstellen. Der Provider benötigt beide Aktionen der Richtlinie: Er ruft `DescribeSecret` auf, wenn er rotierte Geheimnisse abstimmt, daher gewährt ein `GetSecretValue`-only Grant Mounts beim ersten Deploy, stoppt aber das Abholen von Rotationen.

        Stellen Sie das Gateway als Standard-Deployment plus Service und Ingress bereit, wie in [Kubernetes-Bereitstellung](/docs/de/claude-apps-gateway-deploy#kubernetes) beschrieben, mit:

        * `serviceAccountName: gateway`
        * `gateway.yaml` gemountet aus einer ConfigMap und die Geheimnisse als Dateien bei `/secrets` gemountet
        * die Readiness-Probe auf `GET /readyz` gerichtet

        Für das Front-End, ein Ingress, das vom AWS Load Balancer Controller verwaltet wird, stellt den internen ALB bereit. Kommentieren Sie es mit:

        * `alb.ingress.kubernetes.io/scheme: internal` und `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, daher werden keine öffentlichen AAAA-Datensätze für die `/login` [private-Netzwerk-Prüfung](/docs/de/claude-apps-gateway#prerequisites) veröffentlicht, die sie ablehnt
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, daher lässt die Controller-verwaltete Frontend-Sicherheitsgruppe nur Ihr Unternehmensnetzwerk anstelle des `0.0.0.0/0` Standards zu
        * `alb.ingress.kubernetes.io/certificate-arn` mit dem ACM-Zertifikat
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, daher fällt der Listener nicht auf die Legacy-Standard-Richtlinie zurück, die TLS 1.0 und 1.1 akzeptiert
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, eine Marge über dem Streaming-Keepalive des Gateways; siehe [Troubleshooting](#troubleshooting)

        Mit IRSA liest das AWS SDK ein projiziertes Service-Account-Token und tauscht es mit AWS STS aus, daher benötigt der Pod niemals den EC2-Instanz-Metadaten-Service; eine Egress-NetworkPolicy kann `169.254.169.254` für Gateway-Pods blockieren. Das Node-Hop-Limit-Problem in [Troubleshooting](#troubleshooting) unten gilt nur für Cluster, die IRSA überspringen und sich auf Node-Instanzrollen verlassen.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Pushen Sie die Gateway-URL zu Entwicklermaschinen">
    Das Gateway läuft jetzt, aber Entwickler können es von `/login` nicht erreichen, bis die Gateway-URL auf ihren Maschinen ist. Setzen Sie `forceLoginMethod` und `forceLoginGatewayUrl` in der [verwalteten Einstellungsdatei](/docs/de/claude-apps-gateway#set-the-gateway-url), die Sie über MDM auf jedes Gerät bereitstellen. Es gibt keine Gateway-Option im Login-Picker für einen Entwickler, um manuell auszuwählen.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform-Referenz
</h2>

Das Begleit-Bundle bei [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) packt diese Seite als Code:

* **`setup.sh`** skriptet die Bereitstellungs-Anleitung oben mit den gleichen `aws` Befehlen auf dem ECS Fargate-Pfad. Es ist idempotent: Bestehende Ressourcen werden erkannt und übersprungen, daher ist das erneute Ausführen sicher, und jeder Standard kann über Umgebungsvariable überschrieben werden. Sie erstellen immer noch das Okta OIDC-Client-Geheimnis und das ACM-Zertifikat selbst: Ein Lauf ohne sie überspringt die ECS/ALB-Bereitstellung, nennt die fehlenden Eingaben und druckt den `create-secret` Befehl; erstellen Sie beide und führen Sie erneut aus. Das Bedrock-Use-Case-Formular und der Route 53-Alias werden als nächste Schritte statt automatisch ausgeführt, und der Client-MDM-Push bleibt ein manueller Schritt von dieser Seite.
* **`gateway.yaml.example`** ist die Konfigurationsvorlage aus dem gateway.yaml-Schritt, mit den optionalen Schlüsseln kommentiert. Kopieren Sie sie zu `gateway.yaml` und ersetzen Sie jeden `REPLACE_ME`, bevor Sie erstellen.
* **`Dockerfile`** erstellt das Runtime-Image aus der vorkompilierten `linux-x64` Binärdatei und kopiert Ihre ausgefüllte `gateway.yaml` bei `/etc/claude/gateway.yaml`, plus das AWS RDS Zertifikat-Bundle, das das `sslmode=verify-full` des Stores verankert. `setup.sh` lädt das Bundle nur herunter, wenn es nicht bereits im Build-Kontext ist; löschen Sie die Datei und erstellen Sie unter einem neuen Tag neu, um eine AWS CA-Rotation zu erhalten. Die Konfigurationsdatei enthält keine Geheimniswerte, da jede Anmeldedaten beim Start über `${VAR}` Erweiterung aufgelöst wird. Eine Konfigurationsbearbeitung bedeutet daher einen Rebuild unter einem neuen Tag; `setup.sh` automatisiert dies durch Tagging-Images mit einem Hash der Datei.
* **`terraform/`** stellt den gleichen ECS Fargate-Umfang deklarativ bereit: die Sicherheitsgruppen, IAM-Rollen, ECR-Repository, RDS-Instanz, Secrets Manager-Geheimnisse und den ECS-Service hinter dem internen ALB. Das VPC und die privaten Subnetze bleiben Voraussetzungen, die als Variablen übergeben werden. Terraform erstellt das ECR-Repository, erstellt aber nicht das Image, und die Service-Definition referenziert das Image, daher ist die Anwendung zwei Durchläufe: eine gezielte Anwendung für das Repository, dann der Build und Push, dann die vollständige Anwendung. Das `terraform/README.md` des Bundles behandelt die Variablen, den Remote-State und den Abbau.

Wie diese Seite ist das Bundle ein funktionierendes Beispiel für kundenverwaltete Infrastruktur statt einer unterstützten Produktionsbereitstellung; überprüfen und passen Sie es an Ihre eigene Umgebung an, bevor Sie sich darauf verlassen.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Für Gateway-Boot- und Login-Fehler siehe die plattformunabhängige [Troubleshooting-Tabelle](/docs/de/claude-apps-gateway-deploy#troubleshooting). Die Einträge unten sind spezifisch für AWS.

| Symptom                                                                                                                                    | Ursache                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Behebung                                                                                                                                                                                                                                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | Der Gateway-Name wird zu mindestens einer öffentlichen Adresse aufgelöst. Ein Dual-Stack-interner ALB veröffentlicht öffentliche AAAA-Datensätze, und die [private-Netzwerk-Prüfung](/docs/de/claude-apps-gateway#prerequisites) erfordert, dass jede aufgelöste Adresse privat ist                                                                                                                                                                                                                                                                                                                                                 | Erstellen Sie den ALB mit `--ip-address-type ipv4`, oder bedienen Sie einen separaten internen DNS-Namen ohne öffentlichen AAAA-Datensatz                                                                                                                                                                                                 |
| Jede Bedrock-Anfrage gibt 502 zurück; Log zeigt `Could not load credentials from any providers`                                            | Die Task läuft auf dem ECS EC2-Start-Typ ohne Task-Rolle, oder der Pod läuft auf einem EKS-Knoten ohne IRSA, daher kommen Anmeldedaten aus Instanz-Metadaten, die IMDSv2's Standard-Hop-Limit von 1 innerhalb eines Containers stoppt. Keiner der Pfade auf dieser Seite ist betroffen: Fargate-Task-Rollen und IRSA verwenden keine Instanz-Metadaten                                                                                                                                                                                                                                                                         | Bevorzugen Sie Task-Rollen und IRSA. Wo Instanz-Anmeldedaten unvermeidlich sind, erhöhen Sie das Hop-Limit mit `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; die [plattformunabhängige Tabelle](/docs/de/claude-apps-gateway-deploy#troubleshooting) behandelt die Tradeoffs                  |
| Bedrock-Anfragen geben `403 AccessDeniedException` zurück                                                                                  | Das Konto hat das einmalige Use-Case-Formular von Anthropic nicht eingereicht, das automatische AWS Marketplace-Abonnement, das beim ersten Invoke des Kontos beginnt, ist noch nicht abgeschlossen, oder die Task-Rollen-Richtlinie fehlen die Inferenzprofil- oder Foundation-Model-ARNs                                                                                                                                                                                                                                                                                                                                     | Reichen Sie das Use-Case-Formular aus dem Modellkatalog der Bedrock-Konsole ein; wenn es gerade eingereicht wurde oder dies der erste Invoke des Kontos ist, versuchen Sie es nach ein paar Minuten erneut. Gewähren Sie `bedrock:InvokeModel` und `bedrock:InvokeModelWithResponseStream` auf beiden ARN-Familien.                       |
| Bedrock gibt eine `ValidationException` zurück, die besagt, dass On-Demand-Durchsatz nicht unterstützt wird                                | Ein benutzerdefinierter `models:` Eintrag wird zu einer bloßen Foundation-Model-ID zugeordnet, die die Region nur über Inferenzprofile bedient                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Ordnen Sie das Modell stattdessen seiner Cross-Region-Inferenzprofil-ID (`us.anthropic.*`) zu; der integrierte Katalog tut dies bereits                                                                                                                                                                                                   |
| ECS-Task stoppt mit `ResourceInitializationError`, bevor das Gateway etwas protokolliert                                                   | Die Ausführungsrolle kann die Secrets Manager-Geheimnisse nicht lesen, oder die privaten Subnetze haben keinen Pfad zu Secrets Manager oder ECR                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Gewähren Sie `secretsmanager:GetSecretValue` auf den drei `gateway-` Geheimnis-ARNs der Ausführungsrolle, und stellen Sie Egress über das NAT-Gateway bereit, oder ohne eines, Interface-Endpunkte für Secrets Manager, ECR und CloudWatch Logs, die der `awslogs` Treiber in der gleichen Phase benötigt, plus einen S3-Gateway-Endpunkt |
| Gateway-Boot beendet mit einem Postgres-Verbindungs-Timeout-Fehler                                                                         | Die Datenbanksicherheitsgruppe lässt die Sicherheitsgruppe des Gateways nicht auf 5432 zu, oder der Service läuft außerhalb des VPC der Datenbank                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Erlauben Sie 5432 von der Sicherheitsgruppe des Gateways auf der Datenbank, und führen Sie den Service im gleichen VPC wie die DB-Subnet-Gruppe aus                                                                                                                                                                                       |
| Gateway-Boot beendet mit einem Postgres TLS-Zertifikat-Verifizierungsfehler                                                                | Die Verbindungszeichenfolge setzt `sslmode=verify-full`, aber das Image vertraut dem RDS CA-Bundle nicht: Das Bundle wurde nicht in das Image kopiert, oder `NODE_EXTRA_CA_CERTS` zeigt nicht darauf                                                                                                                                                                                                                                                                                                                                                                                                                           | Fügen Sie die zwei Dockerfile-Zeilen des Build-Schritts hinzu, die das Bundle kopieren und `NODE_EXTRA_CA_CERTS` setzen, erstellen Sie dann neu, pushen Sie unter einem neuen Tag und stellen Sie erneut bereit                                                                                                                           |
| Streaming-Antworten brechen während einer ruhigen Periode ab                                                                               | Ein Gateway älter als v2.1.229 auf einem Bedrock- oder Claude Platform on AWS-Upstream sendet nichts, während der Upstream ruhig ist, zum Beispiel während erweitertem Denken ohne gestreamte Ausgabe. Das ALB schließt eine Verbindung nach 60 Sekunden ohne Daten standardmäßig, daher schneidet es den Stream bei dieser Lücke ab. Gateways v2.1.229 und später halten einen ruhigen Stream unter diesem Timeout: auf diesen Upstreams gibt das Gateway ein SSE `ping` Event aus, sobald etwa 15 Sekunden ohne Stream-Daten vergangen sind, und auf einem Anthropic API-Upstream leitet es die eigenen Pings der API weiter | Aktualisieren Sie das Gateway auf v2.1.229 oder später, oder setzen Sie das `idle_timeout.timeout_seconds` Attribut auf `3600`, über `modify-load-balancer-attributes` oder die `load-balancer-attributes` Ingress-Annotation auf EKS                                                                                                     |

<h2 id="telemetry">
  Telemetrie
</h2>

Das Gateway gibt Ihnen pro-Entwickler Nutzungsmetriken ohne jede pro-Maschinen OTEL-Konfiguration. Claude Code gibt OpenTelemetry (OTLP) Metriken, Protokolle und Opt-in-Traces aus; [Überwachung der Nutzung](/docs/de/monitoring-usage) behandelt alles, was die CLI meldet. Bei Gateway-Sitzungen stempelt die CLI jeden Export mit den authentifizierten IdP-Identitätsattributen `user.id`, `user.email` und `user.groups`, daher wird die Nutzung pro Entwickler ohne `OTEL_RESOURCE_ATTRIBUTES` Rohrleitungen zusammengefasst.

Das Gateway selbst ist ein authentifiziertes OTLP-Relais. Setzen Sie [`telemetry.forward_to`](/docs/de/claude-apps-gateway-config#telemetry) zusammen mit `listen.public_url`, und es pusht die OTEL-Exporter-Einstellungen zu jedem verbundenen Client und leitet seinen OTLP-Verkehr wörtlich zu jedem Ziel weiter, das Sie auflisten. Jedes Ziel entscheidet sich unabhängig für Metriken, Protokolle und Traces, und der Standard ist nur Metriken; siehe die [`telemetry` Referenz](/docs/de/claude-apps-gateway-config#telemetry) für die pro-Signal-Felder und ihre Empfindlichkeits-Tradeoffs. Das Gateway puffert, aggregiert oder speichert keine Telemetrie, daher ist, wo die Daten landen, vollständig die Exporter-Konfiguration des Collectors.

Client-Telemetrie ist standardmäßig aus; das Konfigurieren von `telemetry.forward_to` ist, was sie für verbundene Entwickler einschaltet, und jeder interaktive Client zeigt einen einmaligen Sicherheitsgenehmigungsdialog für die gepushten Einstellungen, wie in der [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config#telemetry) beschrieben. Auf AWS wird jedes Signal wie folgt einem Ziel zugeordnet.

<h3 id="client-metrics-logs-and-traces">
  Client-Metriken, Protokolle und Traces
</h3>

Zeigen Sie `telemetry.forward_to` auf einen OpenTelemetry-Collector, wie den [AWS Distro for OpenTelemetry (ADOT) Collector](https://aws-otel.github.io/), und exportieren Sie von dort zu Amazon CloudWatch, Amazon Managed Service for Prometheus oder einem beliebigen OTLP-Backend.

Führen Sie den Collector als seinen eigenen internen Service aus, der über `https://` erreichbar ist; die [`telemetry` Referenz](/docs/de/claude-apps-gateway-config#telemetry) behandelt die Loopback-Ausnahme und `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Gateway-Protokolle
</h3>

Auf ECS Fargate, kein zusätzliches Setup: Der `awslogs` Treiber liefert die stderr des Gateways, die seine Audit-Events und Betriebsprotokolle trägt, zur `/ecs/claude-gateway` Log-Gruppe, die oben erstellt wurde. Auf EKS erreichen Pod-Protokolle CloudWatch nicht standardmäßig, daher geht die Audit-Spur verloren, bis Sie Log-Erfassung installieren: Das Amazon CloudWatch Observability Add-on mit aktivierter Container-Log-Erfassung, oder ein Fluent Bit DaemonSet. Auf beiden Pfaden fragen Sie die Protokolle mit CloudWatch Logs Insights ab und fahren Alarme von Metrik-Filtern.

<h3 id="container-metrics">
  Container-Metriken
</h3>

Aktivieren Sie Container Insights auf dem Cluster mit `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` für pro-Task CPU, Speicher und Netzwerk. Auf EKS installieren Sie das Amazon CloudWatch Observability Add-on.

<h3 id="spend">
  Ausgaben
</h3>

Telemetrie zeigt Nutzung im Nachhinein; [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) sind die Live-Ansicht des Gateways pro Entwickler und Durchsetzung auf der gemeinsamen Upstream-Anmeldedaten.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Konfigurationsreferenz](/docs/de/claude-apps-gateway-config): jede `gateway.yaml` Option, einschließlich `managed.policies` und `telemetry`
* [Bereitstellung und Betrieb](/docs/de/claude-apps-gateway-deploy): IdP-Setup, Health-Checks, JWT-Geheimnis-Rotation, Upgrades und das Sicherheitsmodell
* [Claude-Apps-Gateway Übersicht](/docs/de/claude-apps-gateway): Schnellstart und Verbindung von Entwicklern
* [AWS-Beispiele für Claude-Apps-Gateway](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): Von AWS verwaltete Bereitstellungsbeispiele, die eine Reihe von Kundenumgebungen abdecken
