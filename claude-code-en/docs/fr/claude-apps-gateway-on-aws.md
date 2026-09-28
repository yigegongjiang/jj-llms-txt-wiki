> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déployer la passerelle Claude apps sur AWS

> Un exemple concret d'exécution de la passerelle Claude apps sur AWS : ECS Fargate ou EKS, Amazon RDS pour PostgreSQL, AWS Secrets Manager et authentification par rôle IAM vers Amazon Bedrock.

<Note>
  Cette page vous guide à travers une façon d'exécuter la passerelle Claude apps sur AWS. La configuration est un exemple fonctionnel pour une infrastructure gérée par le client plutôt qu'un déploiement de production pris en charge ; utilisez-la pour voir comment les éléments s'assemblent avant de l'adapter à votre propre environnement. Pour les exigences indépendantes de la plateforme, consultez le [guide de déploiement](/docs/fr/claude-apps-gateway-deploy).
</Note>

Cet exemple provisionne la passerelle Claude apps sur AWS avec Amazon Bedrock comme upstream de modèle, en utilisant soit [Amazon ECS](https://aws.amazon.com/ecs/) sur [AWS Fargate](https://aws.amazon.com/fargate/) soit [Amazon EKS](https://aws.amazon.com/eks/) pour le calcul. [Okta](https://www.okta.com/) est le fournisseur d'identité (IdP) d'exemple, mais tout IdP conforme à OpenID Connect (OIDC) fonctionne ; consultez [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup) pour les détails spécifiques à chaque IdP.

<Note>
  Bedrock n'est pas le seul upstream Claude sur AWS. La passerelle prend également en charge Claude Platform on AWS, l'API Claude exploitée par Anthropic avec authentification AWS et facturation AWS Marketplace, à la place de Bedrock ou en parallèle. Son entrée upstream, ses identifiants et ses permissions IAM diffèrent de ceux spécifiques à Bedrock de cette page ; la [référence upstream Claude Platform on AWS](/docs/fr/claude-apps-gateway-config#claude-platform-on-aws) couvre ce qui change, et le reste de cette page s'applique sans modification.
</Note>

<h2 id="architecture">
  Architecture
</h2>

<Frame caption="L'architecture d'exemple, avec Amazon Bedrock comme upstream de modèle. Un upstream Claude Platform on AWS occupe la même position.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagramme de la passerelle Claude apps sur AWS : les clients Claude Code se connectent via HTTPS à un équilibreur de charge d'application interne frontal de la passerelle (ECS Fargate ou EKS), qui s'exécute dans des sous-réseaux privés aux côtés d'une instance Amazon RDS pour PostgreSQL pour l'état de session. La passerelle connecte les utilisateurs via OIDC par rapport à l'IdP d'entreprise, lit les secrets d'AWS Secrets Manager, transfère les demandes de modèle à Amazon Bedrock en utilisant son rôle IAM, et extrait son image d'Amazon ECR au déploiement." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

La passerelle s'exécute en tant que point de terminaison HTTPS privé sur votre réseau auquel les développeurs se connectent via votre IdP. Leurs sessions Claude Code atteignent les modèles Claude sur Amazon Bedrock via le rôle IAM de la passerelle, donc aucun identifiant de modèle n'arrive sur les machines des développeurs. La configuration de référence provisionne :

* Un service **Amazon ECS sur AWS Fargate** ou un **Amazon EKS** Deployment exécutant le conteneur de la passerelle
* Un référentiel **Amazon ECR** pour l'image de la passerelle
* Une instance **Amazon RDS pour PostgreSQL** dans des sous-réseaux privés, non accessible publiquement, pour le [store](/docs/fr/claude-apps-gateway-config#store) de la passerelle
* Des secrets **AWS Secrets Manager** pour la clé de signature JWT, le secret client OIDC et l'URL Postgres
* Un **rôle IAM** avec `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` et `bedrock:CountTokens`, attaché en tant que rôle de tâche ECS ou lié via IAM Roles for Service Accounts (IRSA) sur EKS
* Un **équilibreur de charge d'application interne** pour HTTPS

<h2 id="prerequisites">
  Prérequis
</h2>

La procédure pas à pas crée les ressources propres de la passerelle, mais elle s'appuie sur une infrastructure réseau et d'identité que vous avez déjà. Avant de commencer, vous avez besoin de :

* Un compte AWS avec la permission de créer les [ressources ci-dessus](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) installée et [authentifiée](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), et [Docker](https://docs.docker.com/get-started/get-docker/) installé localement
* Un [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) avec au moins deux [sous-réseaux privés](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) dans différentes zones de disponibilité, avec accès Internet sortant via une [passerelle NAT](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html) ; l'équilibreur de charge interne a besoin de sous-réseaux dans deux zones de disponibilité, et la passerelle a besoin d'une sortie vers Bedrock et votre IdP
* Une application web OIDC Okta avec l'URI de redirection `https://<gateway-host>/oauth/callback` ; consultez [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup)
* Un nom d'hôte TLS pour la passerelle, généralement un nom DNS interne dans une [zone hébergée privée Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html) pointant vers l'équilibreur de charge, avec un [certificat ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) pour ce nom, importé ou émis par [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Définir vos variables d'environnement
</h3>

Chaque commande de cette page lit quatre valeurs de votre shell : `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` et `PRIVATE_SUBNETS`.

Choisissez une région US où Bedrock sert les modèles Claude dont vous avez besoin. La procédure pas à pas s'appuie sur le catalogue de modèles intégré de la passerelle, qui se résout en profils d'inférence `us.anthropic.*`, et la politique IAM accorde ces ARN. Dans une région non-US, ajoutez un [bloc `models:`](/docs/fr/claude-apps-gateway-config#models) avec les ID de profil d'inférence de cette géographie et modifiez le préfixe ARN de la politique IAM pour qu'il corresponde.

Si vous n'avez pas l'ID du VPC à portée de main, listez vos VPC avec `aws ec2 describe-vpcs`, puis listez les sous-réseaux de ce VPC pour trouver deux sous-réseaux privés dans différentes zones de disponibilité :

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Exportez les quatre avant de continuer :

```bash theme={null}
export AWS_REGION=us-east-1   # une région US où Bedrock sert les modèles Claude dont vous avez besoin
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Déployer la passerelle
</h2>

Les étapes ci-dessous provisionent le déploiement complet avec des commandes `aws`.

<Steps>
  <Step title="Créer les groupes de sécurité">
    Trois groupes de sécurité chaînent le chemin du trafic : votre réseau d'entreprise atteint l'équilibreur de charge sur le port 443, l'équilibreur de charge atteint la passerelle sur le port 8080, et la passerelle atteint Postgres sur le port 5432. Rien d'autre n'est accessible. La façon dont vous les attachez dépend de la piste de calcul :

    * Sur ECS Fargate, l'étape de déploiement attache `$ALB_SG` à l'équilibreur de charge et `$GW_SG` au service.
    * Sur EKS, le contrôleur AWS Load Balancer crée son propre groupe de sécurité frontal pour l'ALB, donc `$ALB_SG` et `$GW_SG` ne sont pas utilisés : l'annotation `inbound-cidrs` de l'étape de déploiement restreint l'écouteur à votre réseau d'entreprise, et le groupe de sécurité de la base de données admet le groupe de sécurité du cluster à la place.

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

  <Step title="Créer les rôles IAM et soumettre le formulaire de cas d'usage">
    La passerelle s'exécute avec un rôle de tâche dédié dont la seule permission est d'invoquer les modèles Claude sur Bedrock. Selon la [référence upstream Bedrock](/docs/fr/claude-apps-gateway-config#amazon-bedrock), la politique doit couvrir à la fois les ARN de profil d'inférence inter-régions et les ARN de modèle de base sous-jacents :

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

    ECS a également besoin d'un rôle d'exécution, que l'agent ECS lui-même utilise pour extraire l'image d'ECR et injecter les valeurs Secrets Manager créées ultérieurement. Il est séparé du rôle de tâche que le SDK AWS de la passerelle utilise à l'exécution :

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

    La politique nomme un ARN par secret plutôt qu'un wildcard nu `gateway-*`, qui dans un compte partagé correspondrait également à des secrets non liés ; le suffixe `-??????` à la fin correspond exactement au suffixe aléatoire de six caractères que Secrets Manager ajoute à l'ARN de chaque secret. Un `-*` à la fin serait un glob de préfixe simple et correspondrait également à des noms plus longs tels que `gateway-postgres-url-prod`.

    La politique IAM accorde à la passerelle la permission d'appeler Bedrock, et Bedrock active l'accès au modèle par défaut dans les régions commerciales. La porte au niveau du compte restante est le formulaire de cas d'usage unique d'Anthropic : si personne dans votre compte ne l'a soumis, ouvrez la [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/), sélectionnez un modèle Anthropic dans le catalogue de modèles et complétez le formulaire. L'accès est accordé immédiatement après la soumission ; consultez [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock#1-submit-use-case-details) pour le formulaire AWS Organizations et les permissions IAM dont le soumetteur a besoin.

    La piste EKS réutilise les deux documents de politique sur un rôle IRSA à la place des deux rôles ECS ; consultez l'étape de déploiement.
  </Step>

  <Step title="Provisionner Amazon RDS pour PostgreSQL">
    L'instance s'exécute dans les sous-réseaux privés sans adresse publique et avec le chiffrement du stockage activé. La version du moteur est épinglée à Postgres 16, ce qui satisfait le plancher pris en charge de PostgreSQL 14 de la passerelle et garantit que la famille du groupe de paramètres ci-dessous correspond à l'instance.

    Tout d'abord, créez le groupe de sous-réseaux qui place la base de données dans les sous-réseaux privés, et un groupe de paramètres avec `rds.force_ssl=1` pour que le serveur rejette les connexions en texte brut. La version du moteur est épinglée une fois car la famille du groupe de paramètres doit correspondre à la version majeure du moteur que l'instance exécute :

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

    Ensuite, créez l'instance avec un mot de passe maître généré :

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

    L'argument littéral `--master-user-password` est visible dans la table des processus et dans les journaux d'audit/EDR pendant l'exécution de la commande, la même exposition que celle couverte par la note de l'étape des secrets. Sur un hôte partagé ou surveillé, passez le mot de passe via `--cli-input-json` à partir d'un fichier `0600` à la place, de la même façon que le `setup.sh` du bundle.

    Attendez que l'instance soit opérationnelle, ce qui peut prendre plusieurs minutes, puis lisez son point de terminaison privé et assemblez la chaîne de connexion que la passerelle utilisera :

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` fait que la passerelle vérifie la chaîne du certificat du serveur RDS et le nom d'hôte, pas seulement le chiffrement. L'ancre de confiance est le [bundle de certificats AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), que l'étape de construction d'image ci-dessous copie à `/etc/claude/rds-global-bundle.pem` et approuve via `NODE_EXTRA_CA_CERTS`. N'ajoutez pas de paramètre `sslrootcert=` de style libpq à l'URL : le pilote de la passerelle lit uniquement `sslmode` à partir de la chaîne de requête et transmettrait `sslrootcert` à Postgres en tant que paramètre de démarrage, que le serveur rejette.

    Le service ECS ou les pods EKS doivent s'exécuter dans ce VPC pour pouvoir atteindre le point de terminaison privé de l'instance, et le groupe de sécurité `claude-gateway-db` n'admet que le groupe de sécurité de la passerelle.
  </Step>

  <Step title="Écrire gateway.yaml">
    Le bloc `upstreams` pointe vers Bedrock avec `auth: {}`, donc la passerelle s'authentifie via la chaîne de credentials par défaut d'AWS à partir du rôle de tâche sur ECS ou du rôle IRSA sur EKS. Consultez la [référence de configuration](/docs/fr/claude-apps-gateway-config) pour chaque champ.

    Deux champs `listen` dépendent de ce qui est en face de la passerelle :

    * `public_url` : l'origine `https://` externe, requise pour tout bind non-loopback ; consultez la [référence `listen`](/docs/fr/claude-apps-gateway-config#listen). La passerelle construit l'`redirect_uri` de l'IdP et son document de découverte uniquement à partir de cette valeur, jamais à partir des en-têtes `X-Forwarded-*`.
    * `trusted_proxies` : les plages source du frontal. La passerelle honore `X-Forwarded-For` uniquement lorsque le pair TCP est dans cette liste, puis parcourt la chaîne au-delà des sauts de confiance, donc les limites de taux de connexion par IP et les événements d'audit enregistrent les adresses IP des développeurs au lieu de celle de l'équilibreur de charge.

    Sur les deux pistes, le frontal est un ALB interne, qu'il soit créé directement ou par le contrôleur AWS Load Balancer, et les nœuds d'un ALB prennent des adresses à partir des sous-réseaux auxquels il est attaché, donc définissez `trusted_proxies` sur les CIDR de ces sous-réseaux. Cela approuve chaque hôte de ces sous-réseaux en tant que proxy. Gardez la source d'entrée de l'ALB, votre CIDR d'entreprise, de ne pas chevaucher, et ne partagez pas les sous-réseaux avec des charges de travail non fiables qui pourraient usurper les adresses IP des clients via `X-Forwarded-For`.

    L'attribut de préservation du port client de l'ALB, `routing.http.xff_client_port.enabled`, peut rester à l'un ou l'autre paramètre : avec lui activé, l'ALB écrit le client comme `203.0.113.7:54321` ou `[2001:db8::1]:54321`, et la passerelle lit les deux avec le port supprimé.

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
      # Le serveur d'autorisation org Okta retourne un id_token mince qui omet
      # l'email et les groupes ; la passerelle les remplit à partir de /userinfo.
      userinfo_fallback: true
      # Okta émet des groupes uniquement lorsque la portée `groups` est demandée et que
      # le filtre de revendication de groupes de l'application les autorise.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # limite la latence de déprovision ; réduire
    # vers 1 pour une révocation plus stricte

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # correspondre à $AWS_REGION pour que les ARN de la politique IAM
    # le couvrent
        auth: {} # chaîne de credentials par défaut d'AWS :
    # rôle de tâche ECS, ou IRSA sur EKS
    ```

    <Note>
      Seul le bloc `oidc` est spécifique à Okta. Pour utiliser Microsoft Entra ID à la place, définissez `issuer` sur `https://login.microsoftonline.com/<tenant-id>/v2.0`, supprimez `userinfo_fallback` et la portée `groups`, et notez qu'Entra émet des ID d'objet de groupe plutôt que des noms, donc [`managed.policies`](/docs/fr/claude-apps-gateway-config#managed) doit correspondre sur les GUID, ou sur les rôles d'application avec `oidc.groups_claim: roles`. Consultez [Configuration du fournisseur d'identité](/docs/fr/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Stocker les secrets dans AWS Secrets Manager">
    Créez trois secrets ; le rôle d'exécution de l'étape IAM peut déjà les lire :

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Notez l'ARN que chaque appel imprime ; la définition de tâche ECS référence les secrets par ARN.

    <Note>
      Les arguments littéraux `--secret-string` sont visibles dans la table des processus et dans les journaux d'audit/EDR pendant l'exécution de chaque commande. Sur un hôte partagé ou surveillé, mettez la valeur dans un fichier `0600` et passez `--secret-string file://<path>` à la place. Le `setup.sh` du bundle garde les valeurs secrètes hors de l'argv du processus de la même façon, en passant des fichiers temporaires `0600` à `--cli-input-json`.
    </Note>

    Contrairement aux secrets, `gateway.yaml` lui-même ne contient aucune valeur secrète, car chaque credential se résout au démarrage via l'expansion [`${VAR}` ou `${file:...}`](/docs/fr/claude-apps-gateway-config#secret-expansion). La façon dont tout atteint le conteneur diffère selon la piste :

    * Sur ECS, l'étape de construction suivante copie `gateway.yaml` dans l'image à `/etc/claude/gateway.yaml`, et la définition de tâche injecte les trois secrets en tant que variables d'environnement via son champ `secrets`, donc le YAML référence `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` et `${GATEWAY_POSTGRES_URL}`.
    * Sur EKS, montez `gateway.yaml` à partir d'une ConfigMap et les secrets en tant que fichiers à `/secrets`, référencés comme `${file:/secrets/...}`. Sourcez les secrets Kubernetes à partir de Secrets Manager avec External Secrets Operator ou le pilote AWS du pilote CSI Secrets Store, ou créez-les directement avec `kubectl`.
  </Step>

  <Step title="Construire et pousser l'image vers Amazon ECR">
    Construisez l'image selon les [exigences d'image de conteneur](/docs/fr/claude-apps-gateway-deploy#container-image), en plaçant le binaire glibc `linux-x64` à `./claude` dans le contexte de construction. Écrivez votre propre Dockerfile selon ces exigences ou commencez par le [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) du bundle, qui copie le `gateway.yaml` rempli des étapes précédentes dans l'image à `/etc/claude/gateway.yaml`. Sur ECS, cette copie intégrée est la façon dont la configuration atteint le conteneur, c'est pourquoi la construction vient après l'écriture du fichier. La piste EKS monte plutôt `gateway.yaml` à partir d'une ConfigMap au déploiement, donc la copie intégrée n'est pas utilisée là.

    L'image porte également le bundle de certificats AWS RDS comme ancre de confiance pour la chaîne de connexion `sslmode=verify-full`, donc téléchargez-le d'abord dans le contexte de construction. AWS fait tourner le bundle (les nouvelles autorités de certification régionales sont ajoutées), donc téléchargez-le par construction plutôt que d'épingler une somme de contrôle ou de le valider :

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Les exigences d'image de conteneur ne couvrent pas le bundle, donc si vous écrivez votre propre Dockerfile, ajoutez les deux lignes qui le copient et le font confiance ; le Dockerfile du bundle les inclut déjà :

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Créez le référentiel ECR et connectez Docker à celui-ci. Les balises immuables signifient que la balise `<version>` que l'étape de déploiement épingle ne peut pas être ultérieurement silencieusement réorientée vers une image différente :

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Construisez et poussez l'image. La définition de tâche ci-dessous exécute `linux/amd64`, donc la plateforme doit correspondre ici ; pour Fargate sur ARM64 (Graviton), construisez `linux/arm64` avec le binaire `linux-arm64` et définissez `cpuArchitecture` sur `ARM64` à la place :

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Déployer">
    <Tabs>
      <Tab title="ECS Fargate">
        Créez le cluster et un groupe de journaux pour la sortie d'erreur standard de la passerelle, qui porte à la fois ses événements d'audit et ses journaux opérationnels. La rétention est un appel séparé, et sans elle CloudWatch garde les journaux pour toujours ; alignez les 90 jours avec votre politique de rétention d'audit :

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Écrivez la définition de tâche. Le rôle de tâche porte la permission Bedrock et le rôle d'exécution injecte les secrets ; utilisez les ARN de secret de l'étape Secrets Manager :

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

        Enregistrez-le :

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Mettez un ALB interne en face avec un groupe cible qui vérifie l'état de santé de la passerelle. `--ip-address-type ipv4` est important : un ALB interne double pile publie des enregistrements AAAA de plage publique, que la vérification de réseau privé `/login` rejette :

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

        Ajoutez l'écouteur HTTPS. `--ssl-policy` épingle un plancher TLS moderne, car l'omettre revient à la politique par défaut héritée `ELBSecurityPolicy-2016-08`, qui accepte toujours TLS 1.0/1.1.

        L'ALB ferme une connexion après 60 secondes sans données par défaut. Les pings de maintien de la passerelle gardent les flux à l'intérieur de ce délai par défaut, donc augmenter le délai d'inactivité ajoute une marge au-dessus de la cadence de ping ; la ligne [Dépannage](#troubleshooting) sur les flux abandonnés couvre le mécanisme et les passerelles plus anciennes. Les commandes ci-dessous ajoutent l'écouteur et augmentent le délai d'inactivité :

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Créez le service. Le disjoncteur de déploiement annule un déploiement dont les tâches continuent d'échouer, à cause d'une mauvaise image ou d'une configuration non amorçable, au dernier état stable au lieu de relancer les tâches défaillantes pour toujours :

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        La période de grâce de 60 secondes donne à une tâche froide le temps de tirer l'image, de se connecter au store et de répondre à sa première vérification de santé avant qu'ECS ne commence à compter les défaillances par rapport au déploiement. La vérification de santé du groupe cible sur `GET /readyz` vérifie que le store est accessible, donc une tâche qui ne peut pas atteindre Postgres n'entre jamais en rotation ; consultez [Comportement en cas de panne](/docs/fr/claude-apps-gateway-deploy#outage-behavior) pour le compromis et l'alternative `/healthz`.

        Les tâches s'exécutent dans des sous-réseaux privés sans IP publique, donc tout le trafic sortant (vers Bedrock, votre IdP, Secrets Manager, ECR et CloudWatch Logs) passe par la passerelle NAT. Pour garder le trafic Bedrock hors du chemin public, créez un point de terminaison VPC d'interface `bedrock-runtime` et pointez l'`base_url` upstream vers celui-ci, comme indiqué dans la [référence upstream Bedrock](/docs/fr/claude-apps-gateway-config#amazon-bedrock) ; l'IdP a toujours besoin d'une sortie Internet.

        Terminez en donnant aux développeurs un nom d'hôte privé résolvable : dans une zone hébergée privée Route 53, aliasez le nom DNS interne de la passerelle à l'ALB, et définissez `listen.public_url` sur ce nom d'hôte. Le nom `*.elb.amazonaws.com` propre de l'ALB se résout en adresses privées sur un ALB interne, mais il ne peut pas porter votre certificat ACM, donc utilisez votre propre nom.

        Mettez à jour l'URI de redirection autorisée du client OAuth vers `<public_url>/oauth/callback` avant la première connexion. Après avoir modifié `public_url`, reconstruisez et poussez l'image sous une nouvelle balise, enregistrez une nouvelle révision de définition de tâche et redéployez. Sur ECS, le paramètre vit dans le `gateway.yaml` intégré de l'image, et la passerelle construit son origine publique uniquement à partir de ce paramètre, en ignorant `X-Forwarded-Host` et `X-Forwarded-Proto`. `X-Forwarded-For` est honoré pour les adresses IP des clients uniquement lorsque `listen.trusted_proxies` est défini.
      </Tab>

      <Tab title="EKS">
        Cette piste a besoin de `kubectl` et `eksctl` installés localement, et d'un cluster EKS existant avec un fournisseur OIDC IAM et le contrôleur AWS Load Balancer installé. Le cluster doit être sur `$VPC_ID` pour que les pods puissent atteindre le point de terminaison privé RDS, et le groupe de sécurité `claude-gateway-db` doit admettre le groupe de sécurité du pod ou du nœud du cluster à la place de `$GW_SG`.

        Sur EKS, la passerelle obtient ses credentials Bedrock via IRSA plutôt que les rôles ECS. La politique de confiance `ecs-tasks.amazonaws.com` de l'étape IAM ne s'applique pas ici ; IRSA a besoin d'un rôle dont la politique de confiance fédère sur le fournisseur OIDC du cluster, limité à `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` crée ce rôle, attache les politiques et annote le compte de service Kubernetes avec l'ARN du rôle en une seule étape. Transformez les deux documents de politique de l'étape IAM en politiques gérées qu'il peut attacher :

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

        La politique des secrets n'est nécessaire que lorsque les pods lisent eux-mêmes Secrets Manager, comme le fait le pilote AWS du pilote CSI Secrets Store en utilisant le compte de service du pod de montage ; supprimez-la si vous créez les secrets Kubernetes d'une autre façon. Le fournisseur a besoin des deux actions de la politique : il appelle `DescribeSecret` lorsqu'il réconcilie les secrets rotatés, donc une subvention `GetSecretValue`-uniquement monte au premier déploiement mais arrête de récupérer les rotations.

        Déployez la passerelle en tant que Deployment standard plus un Service et un Ingress, comme décrit dans [Déploiement Kubernetes](/docs/fr/claude-apps-gateway-deploy#kubernetes), avec :

        * `serviceAccountName: gateway`
        * `gateway.yaml` monté à partir d'une ConfigMap et les secrets montés à `/secrets`
        * la sonde de disponibilité pointée vers `GET /readyz`

        Pour le frontal, un Ingress géré par le contrôleur AWS Load Balancer provisionne l'ALB interne. Annotez-le avec :

        * `alb.ingress.kubernetes.io/scheme: internal` et `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, pour qu'aucun enregistrement AAAA de plage publique ne soit publié pour la vérification de [réseau privé](/docs/fr/claude-apps-gateway#prerequisites) `/login` à rejeter
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, pour que le groupe de sécurité frontal géré par le contrôleur n'admette que votre réseau d'entreprise à la place de sa valeur par défaut `0.0.0.0/0`
        * `alb.ingress.kubernetes.io/certificate-arn` avec le certificat ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, pour que l'écouteur ne revienne pas à la politique par défaut héritée qui accepte TLS 1.0 et 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, une marge au-dessus du maintien de la passerelle en streaming ; consultez [Dépannage](#troubleshooting)

        Avec IRSA, le SDK AWS lit un jeton de compte de service projeté et l'échange avec AWS STS, donc le pod n'a jamais besoin du service de métadonnées d'instance EC2 ; une NetworkPolicy de sortie peut bloquer `169.254.169.254` pour les pods de passerelle. Le problème de limite de saut de nœud dans [Dépannage](#troubleshooting) ci-dessous s'applique uniquement aux clusters qui ignorent IRSA et s'appuient sur les rôles d'instance de nœud.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Pousser l'URL de la passerelle vers les machines des développeurs">
    La passerelle s'exécute maintenant, mais les développeurs ne peuvent pas la atteindre à partir de `/login` jusqu'à ce que l'URL de la passerelle soit sur leurs machines. Définissez `forceLoginMethod` et `forceLoginGatewayUrl` dans le [fichier de paramètres gérés](/docs/fr/claude-apps-gateway#set-the-gateway-url) que vous déployez sur chaque appareil via MDM. Il n'y a pas d'option de passerelle dans le sélecteur de connexion pour qu'un développeur sélectionne manuellement.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Référence Terraform
</h2>

Le bundle compagnon à [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) empaquette cette page en tant que code :

* **`setup.sh`** script la procédure pas à pas de provisionnement ci-dessus avec les mêmes commandes `aws`, sur la piste ECS Fargate. Il est idempotent : les ressources existantes sont détectées et ignorées, donc le réexécuter est sûr, et tout défaut peut être remplacé via une variable d'environnement. Vous créez toujours le secret client OIDC Okta et le certificat ACM vous-même : une exécution sans eux ignore le déploiement ECS/ALB, nomme les entrées manquantes et imprime la commande `create-secret` ; créez les deux et réexécutez. Le formulaire de cas d'usage Bedrock et l'alias Route 53 s'impriment comme les prochaines étapes plutôt que de s'exécuter automatiquement, et la poussée MDM du client reste une étape manuelle de cette page.
* **`gateway.yaml.example`** est le modèle de configuration de l'étape gateway.yaml, avec les clés optionnelles incluses commentées. Copiez-le vers `gateway.yaml` et remplacez chaque `REPLACE_ME` avant de construire.
* **`Dockerfile`** construit l'image d'exécution à partir du binaire précompilé `linux-x64` et copie votre `gateway.yaml` rempli à `/etc/claude/gateway.yaml`, plus le bundle de certificats AWS RDS qui ancre le `sslmode=verify-full` du store. `setup.sh` télécharge le bundle uniquement lorsqu'il n'est pas déjà dans le contexte de construction ; supprimez le fichier et reconstruisez sous une nouvelle balise pour récupérer une rotation d'autorité de certification AWS. Le fichier de configuration ne contient aucune valeur secrète, car chaque credential se résout au démarrage via l'expansion `${VAR}`. Une modification de configuration signifie donc une reconstruction sous une nouvelle balise ; `setup.sh` automatise cela en marquant les images avec un hash du fichier.
* **`terraform/`** provisionne la même portée ECS Fargate de manière déclarative : les groupes de sécurité, les rôles IAM, le référentiel ECR, l'instance RDS, les secrets Secrets Manager et le service ECS derrière l'ALB interne. Le VPC et les sous-réseaux privés restent des prérequis, transmis en tant que variables. Terraform crée le référentiel ECR mais ne construit pas l'image, et la définition de service référence l'image, donc l'application est deux passes : une application ciblée pour le référentiel, puis la construction et la poussée, puis l'application complète. Le `terraform/README.md` du bundle couvre les variables, l'état distant et le démontage.

Comme cette page, le bundle est un exemple fonctionnel pour une infrastructure gérée par le client plutôt qu'un déploiement de production pris en charge ; examinez et adaptez-le à votre propre environnement avant de vous y fier.

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour les erreurs de démarrage et de connexion de la passerelle, consultez le [tableau de dépannage](/docs/fr/claude-apps-gateway-deploy#troubleshooting) indépendant de la plateforme. Les entrées ci-dessous sont spécifiques à AWS.

| Symptôme                                                                                                                                    | Cause                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Correction                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login` : `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | Le nom de la passerelle se résout en au moins une adresse publique. Un ALB interne double pile publie des enregistrements AAAA de plage publique, et la [vérification de réseau privé](/docs/fr/claude-apps-gateway#prerequisites) exige que chaque adresse résolue soit privée                                                                                                                                                                                                                                                                                                                              | Créez l'ALB avec `--ip-address-type ipv4`, ou servez un nom DNS interne séparé sans enregistrement AAAA public                                                                                                                                                                                                                                          |
| Chaque demande Bedrock retourne 502 ; le journal affiche `Could not load credentials from any providers`                                    | La tâche s'exécute sur le type de lancement ECS EC2 sans rôle de tâche, ou le pod s'exécute sur un nœud EKS sans IRSA, donc les credentials proviennent des métadonnées d'instance, que la limite de saut par défaut d'IMDSv2 de 1 arrête à l'intérieur d'un conteneur. Aucune des deux pistes de cette page n'est affectée : les rôles de tâche Fargate et IRSA n'utilisent pas les métadonnées d'instance                                                                                                                                                                                             | Préférez les rôles de tâche et IRSA. Lorsque les credentials d'instance sont inévitables, augmentez la limite de saut avec `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2` ; le [tableau indépendant de la plateforme](/docs/fr/claude-apps-gateway-deploy#troubleshooting) couvre les compromis               |
| Les demandes Bedrock retournent `403 AccessDeniedException`                                                                                 | Le compte n'a pas soumis le formulaire de cas d'usage unique d'Anthropic, l'abonnement AWS Marketplace automatique qui commence à la première invocation du compte n'a pas encore terminé, ou la politique du rôle de tâche manque les ARN de profil d'inférence ou de modèle de base                                                                                                                                                                                                                                                                                                                   | Soumettez le formulaire de cas d'usage à partir du catalogue de modèles de la console Bedrock ; s'il vient d'être soumis ou s'il s'agit de la première invocation du compte, réessayez après quelques minutes. Accordez `bedrock:InvokeModel` et `bedrock:InvokeModelWithResponseStream` sur les deux familles d'ARN.                                   |
| Bedrock retourne une `ValidationException` disant que le débit à la demande n'est pas pris en charge                                        | Une entrée `models:` personnalisée mappe à un ID de modèle de base nu que la région ne sert que via des profils d'inférence                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Mappez le modèle à son ID de profil d'inférence inter-régions (`us.anthropic.*`) à la place ; le catalogue intégré le fait déjà                                                                                                                                                                                                                         |
| La tâche ECS s'arrête avec `ResourceInitializationError` avant que la passerelle ne journalise quoi que ce soit                             | Le rôle d'exécution ne peut pas lire les secrets Secrets Manager, ou les sous-réseaux privés n'ont pas de chemin vers Secrets Manager ou ECR                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Accordez `secretsmanager:GetSecretValue` sur les ARN des trois secrets `gateway-` au rôle d'exécution, et fournissez une sortie via la passerelle NAT, ou, sans elle, des points de terminaison d'interface pour Secrets Manager, ECR et CloudWatch Logs, que le pilote `awslogs` a besoin au même stade, plus un point de terminaison de passerelle S3 |
| Le démarrage de la passerelle se termine avec une erreur de délai d'expiration de connexion Postgres                                        | Le groupe de sécurité de la base de données n'admet pas le groupe de sécurité de la passerelle sur le port 5432, ou le service s'exécute en dehors du VPC de la base de données                                                                                                                                                                                                                                                                                                                                                                                                                         | Autorisez le port 5432 à partir du groupe de sécurité de la passerelle sur celui de la base de données, et exécutez le service dans le même VPC que le groupe de sous-réseaux DB                                                                                                                                                                        |
| Le démarrage de la passerelle se termine avec une erreur de vérification du certificat TLS Postgres                                         | La chaîne de connexion définit `sslmode=verify-full` mais l'image ne fait pas confiance au bundle d'autorité de certification RDS : le bundle n'a pas été copié dans l'image, ou `NODE_EXTRA_CA_CERTS` ne le pointe pas                                                                                                                                                                                                                                                                                                                                                                                 | Ajoutez les deux lignes Dockerfile de l'étape de construction qui copient le bundle et définissent `NODE_EXTRA_CA_CERTS`, puis reconstruisez, poussez sous une nouvelle balise et redéployez                                                                                                                                                            |
| Les réponses de streaming se décrochent au milieu du flux après une période calme                                                           | Une passerelle plus ancienne que v2.1.229 sur un upstream Bedrock ou Claude Platform on AWS n'envoie rien pendant que l'upstream est calme, par exemple lors de la réflexion étendue sans sortie en flux. L'ALB ferme une connexion après 60 secondes sans données par défaut, donc il coupe le flux à cette lacune. Les passerelles v2.1.229 et ultérieures gardent un flux calme sous ce délai : sur ces upstreams, la passerelle émet un événement SSE `ping` une fois qu'environ 15 secondes passent sans données de flux, et sur un upstream API Anthropic, elle relaye les pings propres de l'API | Mettez à jour la passerelle vers v2.1.229 ou ultérieur, ou définissez l'attribut `idle_timeout.timeout_seconds` sur `3600`, via `modify-load-balancer-attributes` ou l'annotation `load-balancer-attributes` Ingress sur EKS                                                                                                                            |

<h2 id="telemetry">
  Télémétrie
</h2>

La passerelle vous donne des métriques d'utilisation par développeur sans aucune configuration OTEL par machine. Claude Code émet des métriques, des journaux et des traces OpenTelemetry (OTLP) optionnels ; [Surveillance de l'utilisation](/docs/fr/monitoring-usage) couvre tout ce que le CLI rapporte. Sur les sessions de passerelle, le CLI marque chaque export avec les attributs d'identité IdP authentifiés `user.id`, `user.email` et `user.groups`, donc l'utilisation s'accumule par développeur sans plomberie `OTEL_RESOURCE_ATTRIBUTES`.

La passerelle elle-même est un relais OTLP authentifié. Définissez [`telemetry.forward_to`](/docs/fr/claude-apps-gateway-config#telemetry) avec `listen.public_url`, et elle pousse les paramètres d'exportateur OTEL à chaque client connecté et transfère leur trafic OTLP verbatim à chaque destination que vous listez. Chaque destination opte pour les métriques, les journaux et les traces indépendamment, et la valeur par défaut est les métriques uniquement ; consultez la [référence `telemetry`](/docs/fr/claude-apps-gateway-config#telemetry) pour les champs par signal et leurs compromis de sensibilité. La passerelle ne met pas en mémoire tampon, n'agrège pas ou ne stocke pas la télémétrie, donc l'endroit où les données arrivent est entièrement la configuration d'exportateur du collecteur.

La télémétrie du client est désactivée par défaut ; configurer `telemetry.forward_to` est ce qui l'active pour les développeurs connectés, et chaque client interactif affiche une boîte de dialogue d'approbation de sécurité unique pour les paramètres poussés, comme décrit dans la [référence de configuration](/docs/fr/claude-apps-gateway-config#telemetry). Sur AWS, chaque signal mappe à une destination comme suit.

<h3 id="client-metrics-logs-and-traces">
  Métriques, journaux et traces du client
</h3>

Pointez `telemetry.forward_to` vers un collecteur OpenTelemetry, tel que le [collecteur AWS Distro for OpenTelemetry (ADOT)](https://aws-otel.github.io/), et exportez de là vers Amazon CloudWatch, Amazon Managed Service for Prometheus ou tout backend OTLP.

Exécutez le collecteur en tant que service interne séparé accessible via `https://` ; la [référence `telemetry`](/docs/fr/claude-apps-gateway-config#telemetry) couvre l'exception de loopback et `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Journaux de la passerelle
</h3>

Sur ECS Fargate, aucune configuration supplémentaire : le pilote `awslogs` livre la sortie d'erreur standard de la passerelle, qui porte ses événements d'audit et ses journaux opérationnels, au groupe de journaux `/ecs/claude-gateway` créé ci-dessus. Sur EKS, les journaux des pods n'arrivent pas à CloudWatch par défaut, donc la piste d'audit est perdue jusqu'à ce que vous installiez la collecte de journaux : le module complémentaire Amazon CloudWatch Observability avec capture de journaux de conteneur activée, ou un DaemonSet Fluent Bit. Sur l'une ou l'autre piste, interrogez les journaux avec CloudWatch Logs Insights et pilotez les alarmes à partir des filtres de métriques.

<h3 id="container-metrics">
  Métriques de conteneur
</h3>

Activez Container Insights sur le cluster avec `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` pour le CPU, la mémoire et le réseau par tâche. Sur EKS, installez le module complémentaire Amazon CloudWatch Observability.

<h3 id="spend">
  Dépenses
</h3>

La télémétrie affiche l'utilisation après le fait ; les [limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) sont la vue en direct de la passerelle et l'application par développeur en plus de la credential upstream partagée.

<h2 id="next-steps">
  Prochaines étapes
</h2>

* [Référence de configuration](/docs/fr/claude-apps-gateway-config) : chaque option `gateway.yaml`, y compris `managed.policies` et `telemetry`
* [Déploiement et opérations](/docs/fr/claude-apps-gateway-deploy) : configuration IdP, vérifications de santé, rotation de clé JWT secrète, mises à niveau et modèle de sécurité
* [Aperçu de la passerelle Claude apps](/docs/fr/claude-apps-gateway) : démarrage rapide et connexion des développeurs
* [Exemples AWS pour la passerelle Claude apps](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway) : exemples de déploiement maintenus par AWS couvrant une gamme d'environnements clients
