> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuire il gateway delle app Claude su AWS

> Un esempio pratico di esecuzione del gateway delle app Claude su AWS: ECS Fargate o EKS, Amazon RDS per PostgreSQL, AWS Secrets Manager e autenticazione basata su ruoli IAM ad Amazon Bedrock.

<Note>
  Questa pagina illustra un modo per eseguire il gateway delle app Claude su AWS. La configurazione è un esempio funzionante per infrastrutture gestite dal cliente piuttosto che una distribuzione di produzione supportata; utilizzatela per vedere come i componenti si incastrano insieme prima di adattarla al vostro ambiente. Per i requisiti indipendenti dalla piattaforma, consultate la [guida alla distribuzione](/docs/it/claude-apps-gateway-deploy).
</Note>

Questo esempio esegue il provisioning del gateway delle app Claude su AWS con Amazon Bedrock come upstream del modello, utilizzando [Amazon ECS](https://aws.amazon.com/ecs/) su [AWS Fargate](https://aws.amazon.com/fargate/) o [Amazon EKS](https://aws.amazon.com/eks/) per il calcolo. [Okta](https://www.okta.com/) è il provider di identità (IdP) di esempio, ma qualsiasi IdP conforme a OpenID Connect (OIDC) funziona; consultate [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup) per i dettagli specifici di ogni IdP.

<Note>
  Bedrock non è l'unico upstream Claude su AWS. Il gateway supporta anche Claude Platform su AWS, l'API Claude gestita da Anthropic con autenticazione AWS e fatturazione AWS Marketplace, al posto di Bedrock o insieme ad esso. La sua voce upstream, le credenziali e i permessi IAM differiscono da quelli specifici di Bedrock in questa pagina; il [riferimento upstream Claude Platform su AWS](/docs/it/claude-apps-gateway-config#claude-platform-on-aws) copre cosa cambia, e il resto di questa pagina si applica invariato.
</Note>

<h2 id="architecture">
  Architettura
</h2>

<Frame caption="L'architettura di esempio, con Amazon Bedrock come upstream del modello. Un upstream Claude Platform su AWS occupa la stessa posizione.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagramma del gateway delle app Claude su AWS: i client Claude Code si connettono tramite HTTPS a un Application Load Balancer interno che sta davanti al gateway (ECS Fargate o EKS), che viene eseguito in subnet private insieme a un'istanza Amazon RDS per PostgreSQL per lo stato della sessione. Il gateway accede gli utenti tramite OIDC rispetto all'IdP aziendale, legge i segreti da AWS Secrets Manager, inoltra le richieste del modello ad Amazon Bedrock utilizzando il suo ruolo IAM e estrae la sua immagine da Amazon ECR al momento della distribuzione." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

Il gateway viene eseguito come endpoint HTTPS privato sulla vostra rete a cui gli sviluppatori accedono tramite il vostro IdP. Le loro sessioni Claude Code raggiungono i modelli Claude su Amazon Bedrock attraverso il ruolo IAM del gateway, quindi nessuna credenziale del modello finisce sulle macchine degli sviluppatori. La configurazione di riferimento esegue il provisioning di:

* Servizio **Amazon ECS su AWS Fargate** o **Amazon EKS** Deployment che esegue il contenitore del gateway
* Repository **Amazon ECR** per l'immagine del gateway
* Istanza **Amazon RDS per PostgreSQL** in subnet private, non accessibile pubblicamente, per lo [store](/docs/it/claude-apps-gateway-config#store) del gateway
* Segreti **AWS Secrets Manager** per la chiave di firma JWT, il segreto del client OIDC e l'URL di Postgres
* **Ruolo IAM** con `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` e `bedrock:CountTokens`, allegato come ruolo di attività ECS o associato tramite IAM Roles for Service Accounts (IRSA) su EKS
* **Application Load Balancer interno** per HTTPS

<h2 id="prerequisites">
  Prerequisiti
</h2>

La procedura dettagliata crea le risorse proprie del gateway, ma si basa su infrastrutture di rete e identità che già possedete. Prima di iniziare, avete bisogno di:

* Un account AWS con autorizzazione per creare le [risorse sopra](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) installata e [autenticata](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), e [Docker](https://docs.docker.com/get-started/get-docker/) installato localmente
* Un [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) con almeno due [subnet private](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) in diverse zone di disponibilità, con accesso a Internet in uscita tramite un [gateway NAT](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); il load balancer interno ha bisogno di subnet in due AZ e il gateway ha bisogno di uscita verso Bedrock e il vostro IdP
* Un'applicazione web OIDC Okta con URI di reindirizzamento `https://<gateway-host>/oauth/callback`; consultate [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup)
* Un nome host TLS per il gateway, tipicamente un nome DNS interno in una [zona ospitata privata Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html) che punta al load balancer, con un [certificato ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) per quel nome, importato o emesso da [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Impostare le variabili di ambiente
</h3>

Ogni comando in questa pagina legge quattro valori dalla vostra shell: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` e `PRIVATE_SUBNETS`.

Scegliete una regione US dove Bedrock serve i modelli Claude di cui avete bisogno. La procedura dettagliata si basa sul catalogo dei modelli integrato del gateway, che si risolve in profili di inferenza `us.anthropic.*`, e la politica IAM concede quegli ARN. In una regione non-US, aggiungete un [blocco `models:`](/docs/it/claude-apps-gateway-config#models) con gli ID del profilo di inferenza di quella geo e cambiate il prefisso ARN della politica IAM per corrispondere.

Se non avete l'ID VPC a portata di mano, elencate i vostri VPC con `aws ec2 describe-vpcs`, quindi elencate le subnet di quel VPC per trovare due private in diverse zone di disponibilità:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Esportate tutti e quattro prima di continuare:

```bash theme={null}
export AWS_REGION=us-east-1   # una regione US dove Bedrock serve i modelli Claude di cui avete bisogno
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Distribuire il gateway
</h2>

I passaggi seguenti eseguono il provisioning della distribuzione completa con comandi `aws`.

<Steps>
  <Step title="Creare i gruppi di sicurezza">
    Tre gruppi di sicurezza concatenano il percorso del traffico: la vostra rete aziendale raggiunge il load balancer sulla porta 443, il load balancer raggiunge il gateway sulla porta 8080 e il gateway raggiunge Postgres sulla porta 5432. Nient'altro è raggiungibile. Come li collegate dipende dal percorso di calcolo:

    * Su ECS Fargate, il passaggio di distribuzione allega `$ALB_SG` al load balancer e `$GW_SG` al servizio.
    * Su EKS, AWS Load Balancer Controller crea il proprio gruppo di sicurezza frontend per l'ALB, quindi `$ALB_SG` e `$GW_SG` non vengono utilizzati: l'annotazione `inbound-cidrs` del passaggio di distribuzione limita il listener alla vostra rete aziendale e il gruppo di sicurezza del database ammette il gruppo di sicurezza del cluster al posto di `$GW_SG`.

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

  <Step title="Creare i ruoli IAM e inviare il modulo del caso d'uso">
    Il gateway viene eseguito con un ruolo di attività dedicato la cui unica autorizzazione è invocare i modelli Claude su Bedrock. Secondo il [riferimento upstream Bedrock](/docs/it/claude-apps-gateway-config#amazon-bedrock), la politica deve coprire sia gli ARN del profilo di inferenza cross-region che gli ARN del modello di base sottostante:

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

    ECS ha anche bisogno di un ruolo di esecuzione, che l'agente ECS stesso utilizza per estrarre l'immagine da ECR e iniettare i valori di Secrets Manager creati in seguito. È separato dal ruolo di attività che l'AWS SDK del gateway utilizza in fase di esecuzione:

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

    I nomi della politica specificano un ARN per segreto piuttosto che un wildcard semplice `gateway-*`, che in un account condiviso corrisponderebbe anche a segreti non correlati; il suffisso finale `-??????` corrisponde esattamente al suffisso di sei caratteri casuale che Secrets Manager aggiunge all'ARN di ogni segreto. Un `-*` finale sarebbe un glob di prefisso semplice e corrisponderebbe anche a nomi più lunghi come `gateway-postgres-url-prod`.

    La politica IAM concede al gateway il permesso di chiamare Bedrock, e Bedrock abilita l'accesso al modello per impostazione predefinita nelle regioni commerciali. Il gate rimanente a livello di account è il modulo del caso d'uso una tantum di Anthropic: se nessuno nel vostro account lo ha inviato, aprite la [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/), selezionate un modello Anthropic dal catalogo dei modelli e completate il modulo. L'accesso viene concesso immediatamente dopo l'invio; consultate [Claude Code su Amazon Bedrock](/docs/it/amazon-bedrock#1-submit-use-case-details) per il modulo AWS Organizations e i permessi IAM di cui il mittente ha bisogno.

    Il percorso EKS riutilizza entrambi i documenti della politica su un ruolo IRSA al posto dei due ruoli ECS; consultate il passaggio di distribuzione.
  </Step>

  <Step title="Eseguire il provisioning di Amazon RDS per PostgreSQL">
    L'istanza viene eseguita nelle subnet private senza indirizzo pubblico e con crittografia dell'archiviazione attivata. La versione del motore è fissata a Postgres 16, che soddisfa il limite supportato del gateway di PostgreSQL 14 e garantisce che la famiglia del gruppo di parametri sottostante corrisponda all'istanza.

    Per prima cosa, create il gruppo di subnet che posiziona il database nelle subnet private e un gruppo di parametri con `rds.force_ssl=1` in modo che il server rifiuti le connessioni in testo semplice. La versione del motore è fissata una volta perché la famiglia del gruppo di parametri deve corrispondere alla versione principale del motore che l'istanza esegue:

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

    Quindi create l'istanza con una password principale generata:

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

    L'argomento letterale `--master-user-password` è visibile nella tabella dei processi e nei log di audit/EDR mentre il comando viene eseguito, la stessa esposizione che la nota del passaggio dei segreti copre. Su un host condiviso o monitorato, passate la password tramite `--cli-input-json` da un file `0600` al posto, il modo in cui `setup.sh` del bundle lo fa.

    Attendete che l'istanza si avvii, il che può richiedere diversi minuti, quindi leggete il suo endpoint privato e assemblate la stringa di connessione che il gateway utilizzerà:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` fa sì che il gateway verifichi la catena del certificato del server RDS e il nome host, non solo crittografare. L'ancora di fiducia è il [bundle di certificati AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), che il passaggio di compilazione dell'immagine sottostante copia in `/etc/claude/rds-global-bundle.pem` e affida tramite `NODE_EXTRA_CA_CERTS`. Non aggiungete un parametro `sslrootcert=` in stile libpq all'URL: il driver del gateway legge solo `sslmode` dalla stringa di query e inoltrerebbe `sslrootcert` a Postgres come parametro di avvio, che il server rifiuta.

    Il servizio ECS o i pod EKS devono essere eseguiti in questo VPC in modo che possano raggiungere l'endpoint privato dell'istanza, e il gruppo di sicurezza `claude-gateway-db` ammette solo il gruppo di sicurezza del gateway.
  </Step>

  <Step title="Scrivere gateway.yaml">
    Il blocco `upstreams` punta a Bedrock con `auth: {}`, quindi il gateway si autentica tramite la catena di credenziali predefinita di AWS dal ruolo di attività su ECS o dal ruolo IRSA su EKS. Consultate il [riferimento di configurazione](/docs/it/claude-apps-gateway-config) per ogni campo.

    Due campi `listen` descrivono cosa sta davanti al gateway:

    * `public_url`: l'origine esterna `https://`, obbligatoria per qualsiasi bind non-loopback; consultate il [riferimento `listen`](/docs/it/claude-apps-gateway-config#listen). Il gateway costruisce l'`redirect_uri` dell'IdP e il suo documento di scoperta solo da questo valore, mai da intestazioni `X-Forwarded-*`.
    * `trusted_proxies`: gli intervalli di origine del front end. Il gateway onora `X-Forwarded-For` solo quando il peer TCP è in questo elenco, quindi cammina nella catena oltre i hop affidabili, in modo che i limiti di velocità di accesso per IP e gli eventi di audit registrino gli IP degli sviluppatori al posto di quello del load balancer.

    Su entrambi i percorsi il front end è un ALB interno, creato direttamente o da AWS Load Balancer Controller, e i nodi di un ALB prendono indirizzi dalle subnet a cui è collegato, quindi impostate `trusted_proxies` ai CIDR di quelle subnet. Questo affida ogni host in quelle subnet come proxy. Evitate che l'origine di ingresso dell'ALB, il vostro CIDR aziendale, si sovrapponga ad essi, e non condividete le subnet con carichi di lavoro non affidabili che potrebbero falsificare gli IP dei client tramite `X-Forwarded-For`.

    L'attributo di conservazione del client port dell'ALB, `routing.http.xff_client_port.enabled`, può rimanere a entrambe le impostazioni: con esso attivato, l'ALB scrive il client come `203.0.113.7:54321` o `[2001:db8::1]:54321`, e il gateway legge entrambi con la porta eliminata.

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
      # Il server di autorizzazione dell'organizzazione Okta restituisce un id_token sottile che omette
      # email e gruppi; il gateway li riempie da /userinfo.
      userinfo_fallback: true
      # Okta emette gruppi solo quando viene richiesto lo scope `groups` e il
      # filtro della rivendicazione dei gruppi dell'app lo consente.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # limita la latenza di deprovisioning; abbassate
    # verso 1 per una revoca più stretta

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # corrispondere a $AWS_REGION in modo che gli ARN della politica IAM
    # lo coprano
        auth: {} # catena di credenziali predefinita di AWS:
    # ruolo di attività ECS, o IRSA su EKS
    ```

    <Note>
      Solo il blocco `oidc` è specifico di Okta. Per utilizzare Microsoft Entra ID al posto, impostate `issuer` su `https://login.microsoftonline.com/<tenant-id>/v2.0`, eliminate `userinfo_fallback` e lo scope `groups`, e notate che Entra emette Object ID dei gruppi piuttosto che nomi, quindi [`managed.policies`](/docs/it/claude-apps-gateway-config#managed) deve corrispondere ai GUID, o su App Roles con `oidc.groups_claim: roles`. Consultate [Configurazione del provider di identità](/docs/it/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Archiviare i segreti in AWS Secrets Manager">
    Create tre segreti; il ruolo di esecuzione dal passaggio IAM può già leggerli:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Notate l'ARN che ogni chiamata stampa; la definizione di attività ECS fa riferimento ai segreti per ARN.

    <Note>
      Gli argomenti letterali `--secret-string` sono visibili nella tabella dei processi e nei log di audit/EDR mentre ogni comando viene eseguito. Su un host condiviso o monitorato, mettete il valore in un file `0600` e passate `--secret-string file://<path>` al posto. `setup.sh` del bundle mantiene i valori dei segreti fuori da argv del processo allo stesso modo, passando file temporanei `0600` a `--cli-input-json`.
    </Note>

    A differenza dei segreti, `gateway.yaml` stesso non contiene valori segreti, perché ogni credenziale si risolve all'avvio tramite l'espansione [`${VAR}` o `${file:...}`](/docs/it/claude-apps-gateway-config#secret-expansion). Come tutto raggiunge il contenitore differisce per percorso:

    * Su ECS, il passaggio di compilazione successivo copia `gateway.yaml` nell'immagine a `/etc/claude/gateway.yaml`, e la definizione di attività inietta i tre segreti come variabili di ambiente tramite il suo campo `secrets`, quindi lo YAML fa riferimento a `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` e `${GATEWAY_POSTGRES_URL}`.
    * Su EKS, montate `gateway.yaml` da una ConfigMap e i segreti come file a `/secrets`, referenziati come `${file:/secrets/...}`. Originare i Kubernetes Secrets da Secrets Manager con External Secrets Operator o il provider AWS del driver CSI Secrets Store, o crearli direttamente con `kubectl`.
  </Step>

  <Step title="Compilare e spingere l'immagine ad Amazon ECR">
    Compilate l'immagine secondo i [requisiti dell'immagine del contenitore](/docs/it/claude-apps-gateway-deploy#container-image), posizionando il binario glibc `linux-x64` a `./claude` nel contesto di compilazione. Scrivete il vostro Dockerfile secondo questi requisiti o iniziate dal [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) del bundle, che copia il `gateway.yaml` compilato dai passaggi precedenti nell'immagine a `/etc/claude/gateway.yaml`. Su ECS quella copia incorporata è come la configurazione raggiunge il contenitore, motivo per cui la compilazione viene dopo che il file è stato scritto. Il percorso EKS al posto monta `gateway.yaml` da una ConfigMap al momento della distribuzione, quindi la copia incorporata non viene utilizzata lì.

    L'immagine porta anche il bundle di certificati AWS RDS come ancora di fiducia per la stringa di connessione `sslmode=verify-full`, quindi scaricatelo nel contesto di compilazione per primo. AWS ruota il bundle (nuove CA regionali vengono aggiunte), quindi scaricatelo per compilazione piuttosto che fissare un checksum o impegnarlo:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    I requisiti dell'immagine del contenitore non coprono il bundle, quindi se scrivete il vostro Dockerfile, aggiungete le due righe che lo copiano e lo affidano; il `Dockerfile` del bundle include già entrambi:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Create il repository ECR e accedete Docker ad esso. I tag immutabili significano che il tag `<version>` che il passaggio di distribuzione fissa non può essere successivamente reindirizzato silenziosamente a un'immagine diversa:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Compilate e spingete l'immagine. La definizione di attività sottostante esegue `linux/amd64`, quindi la piattaforma deve corrispondere qui; per Fargate su ARM64 (Graviton), compilate `linux/arm64` con il binario `linux-arm64` e impostate `cpuArchitecture` su `ARM64` al posto:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Distribuire">
    <Tabs>
      <Tab title="ECS Fargate">
        Create il cluster e un gruppo di log per stderr del gateway, che porta sia i suoi eventi di audit che i log operazionali. La conservazione è una chiamata separata, e senza una CloudWatch mantiene i log per sempre; allineate i 90 giorni con la vostra politica di conservazione dell'audit:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Scrivete la definizione di attività. Il ruolo di attività porta il permesso Bedrock e il ruolo di esecuzione inietta i segreti; utilizzate gli ARN dei segreti dal passaggio Secrets Manager:

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

        Registratela:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Mettete un ALB interno davanti con un gruppo di destinazione che verifica lo stato del gateway. `--ip-address-type ipv4` è importante: un ALB interno dual-stack pubblica record AAAA di intervallo pubblico, che il controllo della rete privata `/login` rifiuta:

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

        Aggiungete il listener HTTPS. `--ssl-policy` fissa un limite TLS moderno, poiché ometterlo ricade nella politica predefinita legacy `ELBSecurityPolicy-2016-08`, che ancora accetta TLS 1.0/1.1.

        L'ALB chiude una connessione dopo 60 secondi senza dati per impostazione predefinita. I ping di keepalive del gateway mantengono i flussi entro quel default, quindi aumentare il timeout aggiunge margine sopra la cadenza del ping; la riga [Troubleshooting](#troubleshooting) sui flussi interrotti copre il meccanismo e i gateway più vecchi. I comandi sottostanti aggiungono il listener e aumentano il timeout:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Create il servizio. Il circuito di distribuzione del deployment fa rotolare una distribuzione le cui attività continuano a fallire, da un'immagine cattiva o una configurazione non avviabile, indietro allo stato stabile precedente al posto di rilanciare attività fallite per sempre:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        Il periodo di grazia di 60 secondi dà a un'attività fredda il tempo di estrarre l'immagine, connettersi allo store e rispondere al suo primo controllo di stato prima che ECS inizi a contare i fallimenti rispetto alla distribuzione. Il controllo di stato del gruppo di destinazione su `GET /readyz` verifica che lo store sia raggiungibile, quindi un'attività che non può raggiungere Postgres non entra mai in rotazione; consultate [Comportamento di interruzione](/docs/it/claude-apps-gateway-deploy#outage-behavior) per il compromesso e l'alternativa `/healthz`.

        Le attività vengono eseguite in subnet private senza IP pubblico, quindi tutto l'egresso (verso Bedrock, il vostro IdP, Secrets Manager, ECR e CloudWatch Logs) passa attraverso il gateway NAT. Per mantenere il traffico Bedrock fuori dal percorso pubblico, create un endpoint VPC dell'interfaccia `bedrock-runtime` e puntate l'`base_url` dell'upstream ad esso, come mostrato nel [riferimento upstream Bedrock](/docs/it/claude-apps-gateway-config#amazon-bedrock); l'IdP ha ancora bisogno di uscita a Internet.

        Finite dando agli sviluppatori un nome host risolvibile privatamente: in una zona ospitata privata Route 53, alias il nome DNS interno del gateway all'ALB, e impostate `listen.public_url` a quel nome host. Il nome `*.elb.amazonaws.com` dell'ALB stesso si risolve in indirizzi privati su un ALB interno, ma non può portare il vostro certificato ACM, quindi utilizzate il vostro nome.

        Aggiornate l'URI di reindirizzamento autorizzato del client OAuth a `<public_url>/oauth/callback` prima del primo accesso. Dopo aver cambiato `public_url`, ricompilate e spingete l'immagine sotto un nuovo tag, registrate una nuova revisione della definizione di attività e ridistribuite. Su ECS l'impostazione vive nel `gateway.yaml` incorporato dell'immagine, e il gateway costruisce la sua origine pubblica solo da quell'impostazione, ignorando `X-Forwarded-Host` e `X-Forwarded-Proto`. `X-Forwarded-For` è onorato per gli IP dei client solo quando `listen.trusted_proxies` è impostato.
      </Tab>

      <Tab title="EKS">
        Questo percorso ha bisogno di `kubectl` e `eksctl` installati localmente, e di un cluster EKS esistente con un provider OIDC IAM e AWS Load Balancer Controller installato. Il cluster deve essere su `$VPC_ID` in modo che i pod possano raggiungere l'endpoint privato RDS, e il gruppo di sicurezza `claude-gateway-db` deve ammettere il gruppo di sicurezza del pod o del nodo del cluster al posto di `$GW_SG`.

        Su EKS il gateway ottiene le sue credenziali Bedrock tramite IRSA piuttosto che i ruoli ECS. La politica di fiducia `ecs-tasks.amazonaws.com` dal passaggio IAM non si applica qui; IRSA ha bisogno di un ruolo la cui politica di fiducia si federi sul provider OIDC del cluster, scoped a `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` crea quel ruolo, allega le politiche e annota l'account di servizio Kubernetes con l'ARN del ruolo in un passaggio. Trasformate i due documenti della politica dal passaggio IAM in politiche gestite che può allegare:

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

        La politica dei segreti è necessaria solo quando i pod leggono Secrets Manager stessi, come fa il provider AWS del driver CSI Secrets Store utilizzando l'account di servizio del pod di montaggio; eliminatela se create i Kubernetes Secrets in un altro modo. Il provider ha bisogno di entrambe le azioni della politica: chiama `DescribeSecret` quando riconcilia i segreti ruotati, quindi una concessione `GetSecretValue`-only monta sulla prima distribuzione ma smette di raccogliere rotazioni.

        Distribuite il gateway come Deployment standard più un Service e un Ingress, come descritto in [Distribuzione Kubernetes](/docs/it/claude-apps-gateway-deploy#kubernetes), con:

        * `serviceAccountName: gateway`
        * `gateway.yaml` montato da una ConfigMap e i segreti montati a `/secrets`
        * il probe di prontezza puntato a `GET /readyz`

        Per il front end, un Ingress gestito da AWS Load Balancer Controller esegue il provisioning dell'ALB interno. Annotatelo con:

        * `alb.ingress.kubernetes.io/scheme: internal` e `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, in modo che nessun record AAAA di intervallo pubblico venga pubblicato per il controllo della rete privata `/login` [private-network check](/docs/it/claude-apps-gateway#prerequisites) da rifiutare
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, in modo che il gruppo di sicurezza gestito dal controller ammetta solo la vostra rete aziendale al posto del suo default `0.0.0.0/0`
        * `alb.ingress.kubernetes.io/certificate-arn` con il certificato ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, in modo che il listener non ricada nella politica predefinita legacy che accetta TLS 1.0 e 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, un margine sopra il keepalive di streaming del gateway; consultate [Troubleshooting](#troubleshooting)

        Con IRSA, l'AWS SDK legge un token dell'account di servizio proiettato e lo scambia con AWS STS, quindi il pod non ha mai bisogno del servizio di metadati dell'istanza EC2; una NetworkPolicy di egresso può bloccare `169.254.169.254` per i pod del gateway. Il problema del limite di hop del nodo in [Troubleshooting](#troubleshooting) sottostante si applica solo ai cluster che saltano IRSA e si affidano ai ruoli dell'istanza del nodo.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Spingere l'URL del gateway alle macchine degli sviluppatori">
    Il gateway è ora in esecuzione, ma gli sviluppatori non possono raggiungerlo da `/login` fino a quando l'URL del gateway non è sulle loro macchine. Impostate `forceLoginMethod` e `forceLoginGatewayUrl` nel [file delle impostazioni gestite](/docs/it/claude-apps-gateway#set-the-gateway-url) che distribuite a ogni dispositivo tramite MDM. Non c'è opzione di gateway nel selettore di accesso per uno sviluppatore da selezionare manualmente.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Riferimento Terraform
</h2>

Il bundle complementare a [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) pacchetti questa pagina come codice:

* **`setup.sh`** script la procedura dettagliata di provisioning sopra con gli stessi comandi `aws`, sul percorso ECS Fargate. È idempotente: le risorse esistenti vengono rilevate e saltate, quindi rieseguirlo è sicuro, e qualsiasi default può essere sovrascritto tramite variabile di ambiente. Voi create comunque il segreto del client OIDC Okta e il certificato ACM voi stessi: un'esecuzione senza di essi salta la distribuzione ECS/ALB, nomina gli input mancanti e stampa il comando `create-secret`; create entrambi e rieseguite. Il modulo del caso d'uso Bedrock e l'alias Route 53 vengono stampati come passaggi successivi piuttosto che eseguiti automaticamente, e il push MDM del client rimane un passaggio manuale da questa pagina.
* **`gateway.yaml.example`** è il modello di configurazione dal passaggio gateway.yaml, con le chiavi opzionali incluse commentate. Copiatelo in `gateway.yaml` e sostituite ogni `REPLACE_ME` prima di compilare.
* **`Dockerfile`** compila l'immagine di runtime dal binario precompilato `linux-x64` e copia il vostro `gateway.yaml` compilato a `/etc/claude/gateway.yaml`, più il bundle di certificati AWS RDS che ancora la connessione `sslmode=verify-full` dello store. `setup.sh` scarica il bundle solo quando non è già nel contesto di compilazione; eliminate il file e ricompilate sotto un nuovo tag per raccogliere una rotazione CA di AWS. Il file di configurazione non contiene valori segreti, poiché ogni credenziale si risolve all'avvio tramite l'espansione `${VAR}`. Una modifica della configurazione quindi significa una ricompilazione sotto un nuovo tag; `setup.sh` automatizza questo taggando le immagini con un hash del file.
* **`terraform/`** esegue il provisioning dello stesso ambito ECS Fargate in modo dichiarativo: i gruppi di sicurezza, i ruoli IAM, il repository ECR, l'istanza RDS, i segreti di Secrets Manager e il servizio ECS dietro l'ALB interno. Il VPC e le subnet private rimangono prerequisiti, passati come variabili. Terraform crea il repository ECR ma non compila l'immagine, e la definizione del servizio fa riferimento all'immagine, quindi l'apply è due passaggi: un apply mirato per il repository, quindi la compilazione e il push, quindi l'apply completo. Il `terraform/README.md` del bundle copre le variabili, lo stato remoto e lo smantellamento.

Come questa pagina, il bundle è un esempio funzionante per infrastrutture gestite dal cliente piuttosto che una distribuzione di produzione supportata; esaminate e adattate al vostro ambiente prima di affidarvi ad esso.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Per gli errori di avvio del gateway e di accesso, consultate la tabella di [troubleshooting](/docs/it/claude-apps-gateway-deploy#troubleshooting) indipendente dalla piattaforma. Le voci sottostanti sono specifiche di AWS.

| Sintomo                                                                                                                                    | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Correzione                                                                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | Il nome del gateway si risolve in almeno un indirizzo pubblico. Un ALB interno dual-stack pubblica record AAAA di intervallo pubblico, e il [controllo della rete privata](/docs/it/claude-apps-gateway#prerequisites) richiede che ogni indirizzo risolto sia privato                                                                                                                                                                                                                                                                                                                                    | Create l'ALB con `--ip-address-type ipv4`, o servite un nome DNS interno separato solo con nessun record AAAA pubblico                                                                                                                                                                                                                 |
| Ogni richiesta Bedrock restituisce 502; il log mostra `Could not load credentials from any providers`                                      | L'attività viene eseguita sul tipo di lancio ECS EC2 senza un ruolo di attività, o il pod viene eseguito su un nodo EKS senza IRSA, quindi le credenziali provengono dai metadati dell'istanza, che il limite di hop predefinito di IMDSv2 di 1 ferma dentro un contenitore. Nessuno dei due percorsi in questa pagina è interessato: i ruoli di attività Fargate e IRSA non utilizzano i metadati dell'istanza                                                                                                                                                                                      | Preferite i ruoli di attività e IRSA. Dove le credenziali dell'istanza sono inevitabili, aumentate il limite di hop con `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; la [tabella indipendente dalla piattaforma](/docs/it/claude-apps-gateway-deploy#troubleshooting) copre i compromessi |
| Le richieste Bedrock restituiscono `403 AccessDeniedException`                                                                             | L'account non ha inviato il modulo del caso d'uso una tantum di Anthropic, l'iscrizione automatica AWS Marketplace che inizia al primo invoke dell'account non ha ancora finito, o la politica del ruolo di attività manca gli ARN del profilo di inferenza o del modello di base                                                                                                                                                                                                                                                                                                                    | Inviate il modulo del caso d'uso dalla catalogo dei modelli della console Bedrock; se è stato appena inviato o questo è il primo invoke dell'account, riprovate dopo alcuni minuti. Concedete `bedrock:InvokeModel` e `bedrock:InvokeModelWithResponseStream` su entrambe le famiglie di ARN.                                          |
| Bedrock restituisce una `ValidationException` dicendo che la velocità effettiva on-demand non è supportata                                 | Una voce `models:` personalizzata mappa a un ID del modello di base semplice che la regione serve solo tramite profili di inferenza                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Mappate il modello all'ID del profilo di inferenza cross-region (`us.anthropic.*`) al posto; il catalogo integrato lo fa già                                                                                                                                                                                                           |
| L'attività ECS si ferma con `ResourceInitializationError` prima che il gateway registri qualcosa                                           | Il ruolo di esecuzione non può leggere i segreti di Secrets Manager, o le subnet private non hanno percorso verso Secrets Manager o ECR                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Concedete `secretsmanager:GetSecretValue` sui tre ARN dei segreti `gateway-` al ruolo di esecuzione, e fornite uscita tramite il gateway NAT, o, senza uno, endpoint dell'interfaccia per Secrets Manager, ECR e CloudWatch Logs, che il driver `awslogs` ha bisogno nella stessa fase, più un endpoint del gateway S3                 |
| L'avvio del gateway esce con un errore di timeout della connessione Postgres                                                               | Il gruppo di sicurezza del database non ammette il gruppo di sicurezza del gateway sulla porta 5432, o il servizio viene eseguito al di fuori del VPC del database                                                                                                                                                                                                                                                                                                                                                                                                                                   | Consentite 5432 dal gruppo di sicurezza del gateway su quello del database, ed eseguite il servizio nello stesso VPC del gruppo di subnet del DB                                                                                                                                                                                       |
| L'avvio del gateway esce con un errore di verifica del certificato TLS di Postgres                                                         | La stringa di connessione imposta `sslmode=verify-full` ma l'immagine non affida il bundle CA di RDS: il bundle non è stato copiato nell'immagine, o `NODE_EXTRA_CA_CERTS` non punta ad esso                                                                                                                                                                                                                                                                                                                                                                                                         | Aggiungete le due righe del Dockerfile del passaggio di compilazione che copiano il bundle e impostano `NODE_EXTRA_CA_CERTS`, quindi ricompilate, spingete sotto un nuovo tag e ridistribuite                                                                                                                                          |
| Le risposte di streaming si interrompono a metà flusso dopo un periodo tranquillo                                                          | Un gateway più vecchio di v2.1.229 su un upstream Bedrock o Claude Platform su AWS non invia nulla mentre l'upstream è tranquillo, ad esempio durante il pensiero esteso senza output trasmesso. L'ALB chiude una connessione dopo 60 secondi senza dati per impostazione predefinita, quindi taglia il flusso a quel gap. I gateway v2.1.229 e successivi mantengono un flusso tranquillo sotto quel timeout: su quegli upstream il gateway emette un evento SSE `ping` una volta che circa 15 secondi passano senza dati di flusso, e su un upstream API Anthropic rilancia i ping propri dell'API | Aggiornate il gateway a v2.1.229 o successivo, o impostate l'attributo `idle_timeout.timeout_seconds` su `3600`, tramite `modify-load-balancer-attributes` o l'annotazione `load-balancer-attributes` Ingress su EKS                                                                                                                   |

<h2 id="telemetry">
  Telemetria
</h2>

Il gateway vi fornisce metriche di utilizzo per sviluppatore senza alcuna configurazione OTEL per macchina. Claude Code emette metriche, log e tracce OpenTelemetry (OTLP) opt-in; [Monitoraggio dell'utilizzo](/docs/it/monitoring-usage) copre tutto ciò che il CLI segnala. Sulle sessioni del gateway il CLI marca ogni esportazione con gli attributi di identità IdP autenticati `user.id`, `user.email` e `user.groups`, quindi l'utilizzo si accumula per sviluppatore senza alcun plumbing `OTEL_RESOURCE_ATTRIBUTES`.

Il gateway stesso è un relè OTLP autenticato. Impostate [`telemetry.forward_to`](/docs/it/claude-apps-gateway-config#telemetry) insieme a `listen.public_url`, e spinge le impostazioni dell'esportatore OTEL a ogni client connesso e inoltra il loro traffico OTLP verbatim a ogni destinazione che elencate. Ogni destinazione opta per metriche, log e tracce indipendentemente, e l'impostazione predefinita è solo metriche; consultate il [riferimento `telemetry`](/docs/it/claude-apps-gateway-config#telemetry) per i campi per segnale e i loro compromessi di sensibilità. Il gateway non memorizza nel buffer, aggrega o archivia la telemetria, quindi dove i dati finiscono è interamente la configurazione dell'esportatore del collettore.

La telemetria del client è disattivata per impostazione predefinita; configurare `telemetry.forward_to` è ciò che la attiva per gli sviluppatori connessi, e ogni client interattivo mostra una finestra di dialogo di approvazione della sicurezza una tantum per le impostazioni spinte, come descritto nel [riferimento di configurazione](/docs/it/claude-apps-gateway-config#telemetry). Su AWS, ogni segnale mappa a una destinazione come segue.

<h3 id="client-metrics-logs-and-traces">
  Metriche, log e tracce del client
</h3>

Puntate `telemetry.forward_to` a un collettore OpenTelemetry, come il [collettore AWS Distro for OpenTelemetry (ADOT)](https://aws-otel.github.io/), ed esportate da lì ad Amazon CloudWatch, Amazon Managed Service for Prometheus, o qualsiasi backend OTLP.

Eseguite il collettore come suo proprio servizio interno raggiungibile su `https://`; il [riferimento `telemetry`](/docs/it/claude-apps-gateway-config#telemetry) copre l'eccezione di loopback e `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Log del gateway
</h3>

Su ECS Fargate, nessuna configurazione extra: il driver `awslogs` consegna stderr del gateway, che porta i suoi eventi di audit e log operazionali, al gruppo di log `/ecs/claude-gateway` creato sopra. Su EKS, i log dei pod non raggiungono CloudWatch per impostazione predefinita, quindi la traccia di audit viene persa fino a quando non installate la raccolta dei log: il componente aggiuntivo Amazon CloudWatch Observability con acquisizione dei log del contenitore abilitata, o un DaemonSet Fluent Bit. Su entrambi i percorsi, interrogate i log con CloudWatch Logs Insights e guidate gli allarmi dai filtri delle metriche.

<h3 id="container-metrics">
  Metriche del contenitore
</h3>

Abilitate Container Insights sul cluster con `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` per CPU, memoria e rete per attività. Su EKS, installate il componente aggiuntivo Amazon CloudWatch Observability.

<h3 id="spend">
  Spesa
</h3>

La telemetria mostra l'utilizzo dopo il fatto; i [limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) sono la vista live del gateway per sviluppatore e l'applicazione sulla credenziale upstream condivisa.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Riferimento di configurazione](/docs/it/claude-apps-gateway-config): ogni opzione `gateway.yaml`, inclusi `managed.policies` e `telemetry`
* [Distribuzione e operazioni](/docs/it/claude-apps-gateway-deploy): configurazione IdP, controlli di stato, rotazione della chiave segreta JWT, aggiornamenti e il modello di sicurezza
* [Panoramica del gateway delle app Claude](/docs/it/claude-apps-gateway): quickstart e connessione degli sviluppatori
* [Esempi AWS per il gateway delle app Claude](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): esempi di distribuzione mantenuti da AWS che coprono una gamma di ambienti dei clienti
