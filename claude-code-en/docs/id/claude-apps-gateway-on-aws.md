> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Terapkan gateway aplikasi Claude di AWS

> Contoh praktis menjalankan gateway aplikasi Claude di AWS: ECS Fargate atau EKS, Amazon RDS untuk PostgreSQL, AWS Secrets Manager, dan autentikasi berbasis peran IAM ke Amazon Bedrock.

<Note>
  Halaman ini menjelaskan satu cara untuk menjalankan gateway aplikasi Claude di AWS. Konfigurasi ini adalah contoh yang berfungsi untuk infrastruktur yang dikelola pelanggan daripada deployment produksi yang didukung; gunakan ini untuk melihat bagaimana potongan-potongan cocok bersama sebelum menyesuaikannya dengan lingkungan Anda sendiri. Untuk persyaratan yang tidak bergantung pada platform, lihat [panduan deployment](/docs/id/claude-apps-gateway-deploy).
</Note>

Contoh ini menyediakan gateway aplikasi Claude di AWS dengan Amazon Bedrock sebagai upstream model, menggunakan [Amazon ECS](https://aws.amazon.com/ecs/) di [AWS Fargate](https://aws.amazon.com/fargate/) atau [Amazon EKS](https://aws.amazon.com/eks/) untuk komputasi. [Okta](https://www.okta.com/) adalah penyedia identitas (IdP) contoh, tetapi penyedia IdP yang sesuai dengan OpenID Connect (OIDC) apa pun berfungsi; lihat [Pengaturan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk detail per-IdP.

<Note>
  Bedrock bukan satu-satunya upstream Claude di AWS. Gateway juga mendukung Claude Platform di AWS, API Claude yang dioperasikan Anthropic dengan autentikasi AWS dan penagihan AWS Marketplace, sebagai pengganti Bedrock atau bersama dengannya. Entri upstream, kredensial, dan izin IAM-nya berbeda dari yang berfokus pada Bedrock di halaman ini; [referensi upstream Claude Platform di AWS](/docs/id/claude-apps-gateway-config#claude-platform-on-aws) mencakup apa yang berubah, dan sisa halaman ini berlaku tanpa perubahan.
</Note>

<h2 id="architecture">
  Arsitektur
</h2>

<Frame caption="Arsitektur contoh, dengan Amazon Bedrock sebagai upstream model. Upstream Claude Platform di AWS menempati posisi yang sama.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Diagram gateway aplikasi Claude di AWS: Klien Claude Code terhubung melalui HTTPS ke Application Load Balancer internal yang menghadap gateway (ECS Fargate atau EKS), yang berjalan di subnet pribadi bersama dengan instans Amazon RDS untuk PostgreSQL untuk status sesi. Gateway memproses masuk pengguna melalui OIDC terhadap IdP perusahaan, membaca rahasia dari AWS Secrets Manager, meneruskan permintaan model ke Amazon Bedrock menggunakan peran IAM-nya, dan menarik gambarnya dari Amazon ECR saat deploy." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

Gateway berjalan sebagai endpoint HTTPS pribadi di jaringan Anda yang diakses pengembang melalui IdP Anda. Sesi Claude Code mereka mencapai model Claude di Amazon Bedrock melalui peran IAM gateway, jadi tidak ada kredensial model yang mendarat di mesin pengembang. Konfigurasi referensi menyediakan:

* Layanan **Amazon ECS di AWS Fargate** atau **Amazon EKS** Deployment yang menjalankan kontainer gateway
* Repositori **Amazon ECR** untuk gambar gateway
* Instans **Amazon RDS untuk PostgreSQL** di subnet pribadi, tidak dapat diakses secara publik, untuk [store](/docs/id/claude-apps-gateway-config#store) gateway
* Rahasia **AWS Secrets Manager** untuk kunci penandatanganan JWT, rahasia klien OIDC, dan URL Postgres
* **Peran IAM** dengan `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, dan `bedrock:CountTokens`, terpasang sebagai peran tugas ECS atau terikat melalui IAM Roles for Service Accounts (IRSA) di EKS
* **Application Load Balancer Internal** untuk HTTPS

<h2 id="prerequisites">
  Prasyarat
</h2>

Panduan ini membuat sumber daya gateway sendiri, tetapi dibangun di atas infrastruktur jaringan dan identitas yang sudah Anda miliki. Sebelum Anda mulai, Anda memerlukan:

* Akun AWS dengan izin untuk membuat [sumber daya di atas](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) terinstal dan [terauthentikasi](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), dan [Docker](https://docs.docker.com/get-started/get-docker/) terinstal secara lokal
* [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) dengan setidaknya dua [subnet pribadi](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) di Zona Ketersediaan berbeda, dengan akses internet keluar melalui [gateway NAT](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); load balancer internal memerlukan subnet di dua AZ, dan gateway memerlukan egress ke Bedrock dan IdP Anda
* Aplikasi web OIDC Okta dengan URI pengalihan `https://<gateway-host>/oauth/callback`; lihat [Pengaturan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup)
* Nama host TLS untuk gateway, biasanya nama DNS internal di [zona hosted pribadi Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html) yang menunjuk ke load balancer, dengan [sertifikat ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) untuk nama itu, diimpor atau dikeluarkan oleh [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Atur variabel lingkungan Anda
</h3>

Setiap perintah di halaman ini membaca empat nilai dari shell Anda: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID`, dan `PRIVATE_SUBNETS`.

Pilih wilayah US tempat Bedrock melayani model Claude yang Anda butuhkan. Panduan ini bergantung pada katalog model bawaan gateway, yang diselesaikan ke profil inferensi `us.anthropic.*`, dan kebijakan IAM memberikan ARN tersebut. Di wilayah non-US, tambahkan [blok `models:`](/docs/id/claude-apps-gateway-config#models) dengan ID profil inferensi geo itu dan ubah awalan ARN kebijakan IAM agar sesuai.

Jika Anda tidak memiliki ID VPC di tangan, daftarkan VPC Anda dengan `aws ec2 describe-vpcs`, kemudian daftarkan subnet VPC itu untuk menemukan dua subnet pribadi di Zona Ketersediaan berbeda:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Ekspor keempat sebelum melanjutkan:

```bash theme={null}
export AWS_REGION=us-east-1   # a US region where Bedrock serves the Claude models you need
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Terapkan gateway
</h2>

Langkah-langkah di bawah menyediakan deployment lengkap dengan perintah `aws`.

<Steps>
  <Step title="Buat grup keamanan">
    Tiga grup keamanan merantai jalur lalu lintas: jaringan perusahaan Anda mencapai load balancer di 443, load balancer mencapai gateway di 8080, dan gateway mencapai Postgres di 5432. Tidak ada yang lain yang dapat dijangkau. Cara Anda melampirkannya tergantung pada jalur komputasi:

    * Di ECS Fargate, langkah deploy melampirkan `$ALB_SG` ke load balancer dan `$GW_SG` ke layanan.
    * Di EKS, AWS Load Balancer Controller membuat grup keamanan frontend-nya sendiri, jadi `$ALB_SG` dan `$GW_SG` tidak digunakan: anotasi `inbound-cidrs` langkah deploy membatasi pendengar ke jaringan perusahaan Anda, dan grup keamanan database mengakui grup keamanan kluster sebagai gantinya.

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

  <Step title="Buat peran IAM dan kirimkan formulir kasus penggunaan">
    Gateway berjalan dengan peran tugas khusus yang satu-satunya izinnya adalah memanggil model Claude di Bedrock. Sesuai [referensi upstream Bedrock](/docs/id/claude-apps-gateway-config#amazon-bedrock), kebijakan harus mencakup baik ARN profil inferensi lintas wilayah maupun ARN model dasar yang mendasarinya:

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

    ECS juga memerlukan peran eksekusi, yang digunakan agen ECS sendiri untuk menarik gambar dari ECR dan menyuntikkan nilai Secrets Manager yang dibuat nanti. Ini terpisah dari peran tugas yang digunakan AWS SDK gateway saat runtime:

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

    Nama kebijakan satu ARN per rahasia daripada wildcard `gateway-*` telanjang, yang dalam akun bersama juga akan cocok dengan rahasia yang tidak terkait; akhiran `-??????` yang tertinggal cocok dengan tepat enam karakter acak yang ditambahkan Secrets Manager ke ARN setiap rahasia. Akhiran `-*` akan menjadi glob awalan biasa dan juga akan cocok dengan nama yang lebih panjang seperti `gateway-postgres-url-prod`.

    Kebijakan IAM memberikan gateway izin untuk memanggil Bedrock, dan Bedrock mengaktifkan akses model secara default di wilayah komersial. Gerbang tingkat akun yang tersisa adalah formulir kasus penggunaan Anthropic satu kali: jika tidak ada yang di akun Anda telah mengirimkannya, buka [konsol Amazon Bedrock](https://console.aws.amazon.com/bedrock/), pilih model Anthropic dari katalog Model, dan lengkapi formulir. Akses diberikan segera setelah pengajuan; lihat [Claude Code di Amazon Bedrock](/docs/id/amazon-bedrock#1-submit-use-case-details) untuk formulir AWS Organizations dan izin IAM yang dibutuhkan pengajuan.

    Trek EKS menggunakan kembali kedua dokumen kebijakan pada peran IRSA sebagai gantinya dari dua peran ECS; lihat langkah deploy.
  </Step>

  <Step title="Sediakan Amazon RDS untuk PostgreSQL">
    Instans berjalan di subnet pribadi tanpa alamat publik dan enkripsi penyimpanan aktif. Versi mesin disematkan ke Postgres 16, yang memenuhi lantai yang didukung gateway dari PostgreSQL 14 dan menjamin keluarga grup parameter di bawah cocok dengan instans.

    Pertama, buat grup subnet yang menempatkan database di subnet pribadi, dan grup parameter dengan `rds.force_ssl=1` sehingga server menolak koneksi plaintext. Versi mesin disematkan sekali karena keluarga grup parameter harus cocok dengan versi utama mesin yang dijalankan instans:

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

    Kemudian buat instans dengan kata sandi master yang dihasilkan:

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

    Argumen `--master-user-password` literal terlihat di tabel proses dan dalam log audit/EDR saat perintah berjalan, paparan yang sama yang dicakup catatan langkah rahasia. Di host bersama atau dipantau, teruskan kata sandi melalui `--cli-input-json` dari file `0600` sebagai gantinya, cara yang dilakukan `setup.sh` bundle.

    Tunggu instans naik, yang dapat memakan waktu beberapa menit, kemudian baca endpoint pribadinya dan kumpulkan string koneksi yang akan digunakan gateway:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` membuat gateway memverifikasi rantai sertifikat server RDS dan nama host, bukan hanya enkripsi. Jangkar kepercayaan adalah [bundel sertifikat AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), yang langkah build gambar di bawah menyalin ke `/etc/claude/rds-global-bundle.pem` dan mempercayai melalui `NODE_EXTRA_CA_CERTS`. Jangan menambahkan parameter `sslrootcert=` gaya libpq ke URL: driver gateway membaca hanya `sslmode` dari string kueri dan akan meneruskan `sslrootcert` ke Postgres sebagai parameter startup, yang ditolak server.

    Layanan ECS atau pod EKS harus berjalan di VPC ini sehingga mereka dapat mencapai endpoint pribadi instans, dan grup keamanan `claude-gateway-db` hanya mengakui grup keamanan gateway.
  </Step>

  <Step title="Tulis gateway.yaml">
    Blok `upstreams` menunjuk ke Bedrock dengan `auth: {}`, jadi gateway mengautentikasi melalui rantai kredensial default AWS dari peran tugas di ECS atau peran IRSA di EKS. Lihat [referensi konfigurasi](/docs/id/claude-apps-gateway-config) untuk setiap bidang.

    Dua bidang `listen` bergantung pada apa yang menghadap gateway:

    * `public_url`: asal `https://` eksternal, diperlukan untuk bind non-loopback apa pun; lihat [referensi `listen`](/docs/id/claude-apps-gateway-config#listen). Gateway membangun `redirect_uri` IdP dan dokumen penemuannya hanya dari nilai ini, tidak pernah dari header `X-Forwarded-*`.
    * `trusted_proxies`: rentang sumber front end. Gateway menghormati `X-Forwarded-For` hanya ketika peer TCP berada dalam daftar ini, kemudian berjalan di rantai melewati hop terpercaya, jadi batas laju sign-in per-IP dan acara audit mencatat IP pengembang daripada load balancer.

    Di kedua trek front end adalah ALB internal, baik dibuat langsung atau oleh AWS Load Balancer Controller, dan node ALB mengambil alamat dari subnet yang dilampirkannya, jadi atur `trusted_proxies` ke CIDR subnet tersebut. Ini mempercayai setiap host di subnet tersebut sebagai proxy. Jaga sumber ingress ALB, CIDR perusahaan Anda, dari tumpang tindih dengannya, dan jangan bagikan subnet dengan beban kerja yang tidak dipercaya yang dapat memalsukan IP klien melalui `X-Forwarded-For`.

    Atribut preservasi port klien ALB, `routing.http.xff_client_port.enabled`, dapat tetap di salah satu pengaturan: dengan itu aktif, ALB menulis klien sebagai `203.0.113.7:54321` atau `[2001:db8::1]:54321`, dan gateway membaca keduanya dengan port dijatuhkan.

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
      # The Okta org authorization server returns a thin id_token that omits
      # email and groups; the gateway fills them from /userinfo.
      userinfo_fallback: true
      # Okta emits groups only when the `groups` scope is requested and the
      # app's groups claim filter allows them.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # bounds deprovision latency; lower
    # toward 1 for tighter revocation

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # match $AWS_REGION so the IAM
    # policy's ARNs cover it
        auth: {} # AWS default credential chain:
    # ECS task role, or IRSA on EKS
    ```

    <Note>
      Hanya blok `oidc` yang spesifik Okta. Untuk menggunakan Microsoft Entra ID sebagai gantinya, atur `issuer` ke `https://login.microsoftonline.com/<tenant-id>/v2.0`, lepaskan `userinfo_fallback` dan cakupan `groups`, dan perhatikan bahwa Entra memancarkan Object ID grup daripada nama, jadi [`managed.policies`](/docs/id/claude-apps-gateway-config#managed) harus cocok pada GUID, atau pada App Roles dengan `oidc.groups_claim: roles`. Lihat [Pengaturan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Simpan rahasia di AWS Secrets Manager">
    Buat tiga rahasia; peran eksekusi dari langkah IAM sudah dapat membacanya:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Catat ARN yang dicetak setiap panggilan; definisi tugas ECS mereferensikan rahasia berdasarkan ARN.

    <Note>
      Argumen `--secret-string` literal terlihat di tabel proses dan dalam log audit/EDR saat setiap perintah berjalan. Di host bersama atau dipantau, masukkan nilai dalam file `0600` dan teruskan `--secret-string file://<path>` sebagai gantinya. `setup.sh` bundle menjaga nilai rahasia dari argv proses dengan cara yang sama, melewatkan file sementara `0600` ke `--cli-input-json`.
    </Note>

    Tidak seperti rahasia, `gateway.yaml` sendiri tidak berisi nilai rahasia, karena setiap kredensial diselesaikan saat boot melalui [ekspansi `${VAR}` atau `${file:...}`](/docs/id/claude-apps-gateway-config#secret-expansion). Bagaimana semuanya mencapai kontainer berbeda menurut trek:

    * Di ECS, build langkah berikutnya menyalin `gateway.yaml` ke dalam gambar di `/etc/claude/gateway.yaml`, dan definisi tugas menyuntikkan tiga rahasia sebagai variabel lingkungan melalui bidang `secrets`-nya, jadi YAML mereferensikan `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}`, dan `${GATEWAY_POSTGRES_URL}`.
    * Di EKS, pasang `gateway.yaml` dari ConfigMap dan rahasia sebagai file di `/secrets`, direferensikan sebagai `${file:/secrets/...}`. Sumber Kubernetes Secrets dari Secrets Manager dengan External Secrets Operator atau penyedia AWS driver Secrets Store CSI, atau buat langsung dengan `kubectl`.
  </Step>

  <Step title="Bangun dan dorong gambar ke Amazon ECR">
    Bangun gambar sesuai [persyaratan gambar kontainer](/docs/id/claude-apps-gateway-deploy#container-image), menempatkan biner glibc `linux-x64` di `./claude` dalam konteks build. Tulis Dockerfile Anda sendiri sesuai persyaratan tersebut atau mulai dari [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) bundle, yang menyalin `gateway.yaml` yang diisi dari langkah sebelumnya ke dalam gambar di `/etc/claude/gateway.yaml`. Di ECS salinan tertanam itu adalah cara konfigurasi mencapai kontainer, itulah mengapa build datang setelah file ditulis. Trek EKS sebagai gantinya memasang `gateway.yaml` dari ConfigMap saat deploy, jadi salinan tertanam tidak digunakan di sana.

    Gambar juga membawa bundel sertifikat AWS RDS sebagai jangkar kepercayaan untuk string koneksi `sslmode=verify-full`, jadi unduh ke dalam konteks build terlebih dahulu. AWS memutar bundel (CA regional baru ditambahkan), jadi unduh per build daripada menyematkan checksum atau melakukan commit:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Persyaratan gambar kontainer tidak mencakup bundel, jadi jika Anda menulis Dockerfile Anda sendiri, tambahkan dua baris yang menyalin dan mempercayainya; `Dockerfile` bundle sudah menyertakan keduanya:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Buat repositori ECR dan masuk Docker ke dalamnya. Tag yang tidak dapat diubah berarti tag `<version>` yang disematkan langkah deploy tidak dapat kemudian diarahkan ulang secara diam-diam ke gambar yang berbeda:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Bangun dan dorong gambar. Definisi tugas di bawah menjalankan `linux/amd64`, jadi platform harus cocok di sini; untuk Fargate di ARM64 (Graviton), bangun `linux/arm64` dengan biner `linux-arm64` dan atur `cpuArchitecture` ke `ARM64` sebagai gantinya:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Terapkan">
    <Tabs>
      <Tab title="ECS Fargate">
        Buat kluster dan grup log untuk stderr gateway, yang membawa acara audit dan log operasional. Retensi adalah panggilan terpisah, dan tanpa satu CloudWatch menyimpan log selamanya; selaraskan 90 hari dengan kebijakan retensi audit Anda:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Tulis definisi tugas. Peran tugas membawa izin Bedrock dan peran eksekusi menyuntikkan rahasia; gunakan ARN rahasia dari langkah Secrets Manager:

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

        Daftarkan:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Letakkan ALB internal di depan dengan grup target yang memeriksa kesehatan gateway. `--ip-address-type ipv4` penting: ALB dual-stack internal menerbitkan catatan AAAA jangkauan publik, yang pemeriksaan jaringan pribadi `/login` menolak:

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

        Tambahkan pendengar HTTPS. `--ssl-policy` menyematkan lantai TLS modern, karena menghilangkannya kembali ke default `ELBSecurityPolicy-2016-08` warisan, yang masih menerima TLS 1.0/1.1.

        ALB menutup koneksi setelah 60 detik tanpa data secara default. Ping keepalive gateway menjaga aliran di dalam default itu, jadi menaikkan timeout menambah margin di atas kecepatan ping; baris [Troubleshooting](#troubleshooting) pada aliran yang dijatuhkan mencakup mekanisme dan gateway yang lebih lama. Perintah di bawah menambahkan pendengar dan menaikkan timeout:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Buat layanan. Pemutus sirkuit deployment menggulung deployment yang tugasnya terus gagal, dari gambar buruk atau konfigurasi yang tidak dapat boot, kembali ke status stabil terakhir daripada meluncurkan tugas yang gagal selamanya:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        Periode ketenangan 60 detik memberi tugas dingin waktu untuk menarik gambar, terhubung ke toko, dan menjawab pemeriksaan kesehatan pertamanya sebelum ECS mulai menghitung kegagalan terhadap deployment. Pemeriksaan kesehatan grup target di `GET /readyz` memverifikasi toko dapat dijangkau, jadi tugas yang tidak dapat mencapai Postgres tidak pernah memasuki rotasi; lihat [Perilaku Pemadaman](/docs/id/claude-apps-gateway-deploy#outage-behavior) untuk tradeoff dan alternatif `/healthz`.

        Tugas berjalan di subnet pribadi tanpa IP publik, jadi semua egress (ke Bedrock, IdP Anda, Secrets Manager, ECR, dan CloudWatch Logs) melalui gateway NAT. Untuk menjaga lalu lintas Bedrock dari jalur publik, buat endpoint VPC antarmuka `bedrock-runtime` dan arahkan `base_url` upstream ke sana, seperti yang ditunjukkan dalam [referensi upstream Bedrock](/docs/id/claude-apps-gateway-config#amazon-bedrock); IdP masih memerlukan egress internet.

        Selesaikan dengan memberikan pengembang nama host yang dapat diselesaikan secara pribadi: di zona hosted pribadi Route 53, alias nama DNS internal gateway ke ALB, dan atur `listen.public_url` ke nama host itu. Nama `*.elb.amazonaws.com` ALB sendiri diselesaikan ke alamat pribadi di ALB internal, tetapi tidak dapat membawa sertifikat ACM Anda, jadi gunakan nama Anda sendiri.

        Perbarui URI pengalihan otorisasi klien OAuth ke `<public_url>/oauth/callback` sebelum sign-in pertama. Setelah mengubah `public_url`, bangun kembali dan dorong gambar di bawah tag baru, daftarkan revisi definisi tugas baru, dan terapkan kembali. Di ECS pengaturan hidup dalam `gateway.yaml` tertanam gambar, dan gateway membangun asal publik hanya dari pengaturan itu, mengabaikan `X-Forwarded-Host` dan `X-Forwarded-Proto`. `X-Forwarded-For` dihormati untuk IP klien hanya ketika `listen.trusted_proxies` diatur.
      </Tab>

      <Tab title="EKS">
        Trek ini memerlukan `kubectl` dan `eksctl` terinstal secara lokal, dan kluster EKS yang ada dengan penyedia OIDC IAM dan AWS Load Balancer Controller terinstal. Kluster harus berada di `$VPC_ID` sehingga pod dapat mencapai endpoint pribadi RDS, dan grup keamanan `claude-gateway-db` harus mengakui grup keamanan pod atau node kluster sebagai pengganti `$GW_SG`.

        Di EKS gateway mendapatkan kredensial Bedrock melalui IRSA daripada peran ECS. Kebijakan kepercayaan `ecs-tasks.amazonaws.com` dari langkah IAM tidak berlaku di sini; IRSA memerlukan peran yang kebijakan kepercayaannya berfederasi pada penyedia OIDC kluster, dibatasi pada `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` membuat peran itu, melampirkan kebijakan, dan memberi anotasi akun layanan Kubernetes dengan ARN peran dalam satu langkah. Ubah dua dokumen kebijakan dari langkah IAM menjadi kebijakan terkelola yang dapat dilampirkan:

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

        Kebijakan rahasia diperlukan hanya ketika pod membaca Secrets Manager sendiri, seperti yang dilakukan penyedia AWS driver Secrets Store CSI menggunakan akun layanan pod pemasangan; lepaskan jika Anda membuat Kubernetes Secrets dengan cara lain. Penyedia memerlukan kedua tindakan kebijakan: ia memanggil `DescribeSecret` ketika merekonsiliasi rahasia yang diputar, jadi hibah `GetSecretValue`-only memasang pada deploy pertama tetapi berhenti mengambil rotasi.

        Terapkan gateway sebagai Deployment standar plus Service dan Ingress, seperti yang dijelaskan dalam [deployment Kubernetes](/docs/id/claude-apps-gateway-deploy#kubernetes), dengan:

        * `serviceAccountName: gateway`
        * `gateway.yaml` dipasang dari ConfigMap dan rahasia dipasang di `/secrets`
        * probe kesiapan menunjuk ke `GET /readyz`

        Untuk front end, Ingress yang dikelola AWS Load Balancer Controller menyediakan ALB internal. Beri anotasi dengan:

        * `alb.ingress.kubernetes.io/scheme: internal` dan `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, jadi tidak ada catatan AAAA jangkauan publik yang diterbitkan untuk pemeriksaan jaringan pribadi `/login` [](/docs/id/claude-apps-gateway#prerequisites) menolak
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, jadi grup keamanan frontend yang dikelola pengontrol mengakui hanya jaringan perusahaan Anda sebagai pengganti default `0.0.0.0/0`-nya
        * `alb.ingress.kubernetes.io/certificate-arn` dengan sertifikat ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, jadi pendengar tidak kembali ke kebijakan default warisan yang menerima TLS 1.0 dan 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, margin di atas keepalive streaming gateway; lihat [Troubleshooting](#troubleshooting)

        Dengan IRSA, AWS SDK membaca token akun layanan yang diproyeksikan dan menukarnya dengan AWS STS, jadi pod tidak pernah memerlukan layanan metadata instans EC2; NetworkPolicy egress dapat memblokir `169.254.169.254` untuk pod gateway. Masalah batas hop node dalam [Troubleshooting](#troubleshooting) di bawah berlaku hanya untuk kluster yang melewati IRSA dan mengandalkan peran instans node.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Dorong URL gateway ke mesin pengembang">
    Gateway sekarang berjalan, tetapi pengembang tidak dapat menjangkaunya dari `/login` sampai URL gateway ada di mesin mereka. Atur `forceLoginMethod` dan `forceLoginGatewayUrl` dalam [file pengaturan terkelola](/docs/id/claude-apps-gateway#set-the-gateway-url) yang Anda terapkan ke setiap perangkat melalui MDM. Tidak ada opsi gateway dalam pemilih login untuk dipilih pengembang secara manual.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referensi Terraform
</h2>

Bundle pendamping di [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) mengemas halaman ini sebagai kode:

* **`setup.sh`** mengskrip panduan penyediaan di atas dengan perintah `aws` yang sama, di trek ECS Fargate. Ini adalah idempoten: sumber daya yang ada terdeteksi dan dilewati, jadi menjalankannya kembali aman, dan default apa pun dapat ditimpa melalui variabel lingkungan. Anda masih membuat rahasia klien OIDC Okta dan sertifikat ACM sendiri: jalankan tanpa mereka melewati deploy ECS/ALB, menamai input yang hilang, dan mencetak perintah `create-secret`; buat keduanya dan jalankan kembali. Formulir kasus penggunaan Bedrock dan alias Route 53 dicetak sebagai langkah berikutnya daripada berjalan secara otomatis, dan push MDM klien tetap menjadi langkah manual dari halaman ini.
* **`gateway.yaml.example`** adalah template konfigurasi dari langkah gateway.yaml, dengan kunci opsional disertakan berkomentar. Salin ke `gateway.yaml` dan ganti setiap `REPLACE_ME` sebelum membangun.
* **`Dockerfile`** membangun gambar runtime dari biner `linux-x64` yang telah dibangun sebelumnya dan menyalin `gateway.yaml` yang diisi di `/etc/claude/gateway.yaml`, ditambah bundel sertifikat AWS RDS yang menambatkan `sslmode=verify-full` toko. `setup.sh` mengunduh bundel hanya ketika belum ada dalam konteks build; hapus file dan bangun kembali di bawah tag baru untuk mengambil rotasi CA AWS. File konfigurasi tidak menyimpan nilai rahasia, karena setiap kredensial diselesaikan saat boot melalui ekspansi `${VAR}`. Edit konfigurasi oleh karena itu berarti rebuild di bawah tag baru; `setup.sh` mengotomatisasi ini dengan menandai gambar dengan hash file.
* **`terraform/`** menyediakan cakupan ECS Fargate yang sama secara deklaratif: grup keamanan, peran IAM, repositori ECR, instans RDS, rahasia Secrets Manager, dan layanan ECS di belakang ALB internal. VPC dan subnet pribadi tetap menjadi prasyarat, dilewatkan sebagai variabel. Terraform membuat repositori ECR tetapi tidak membangun gambar, dan definisi layanan mereferensikan gambar, jadi apply adalah dua pass: apply tertarget untuk repositori, kemudian build dan push, kemudian apply penuh. `terraform/README.md` bundle mencakup variabel, status jarak jauh, dan teardown.

Seperti halaman ini, bundle adalah contoh yang berfungsi untuk infrastruktur yang dikelola pelanggan daripada deployment produksi yang didukung; tinjau dan sesuaikan dengan lingkungan Anda sendiri sebelum mengandalkannya.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk boot gateway dan kesalahan login, lihat tabel [troubleshooting](/docs/id/claude-apps-gateway-deploy#troubleshooting) yang tidak bergantung pada platform. Entri di bawah khusus untuk AWS.

| Gejala                                                                                                                                     | Penyebab                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Perbaikan                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | Nama gateway diselesaikan ke setidaknya satu alamat publik. ALB dual-stack internal menerbitkan catatan AAAA jangkauan publik, dan [pemeriksaan jaringan pribadi](/docs/id/claude-apps-gateway#prerequisites) memerlukan setiap alamat yang diselesaikan menjadi pribadi                                                                                                                                                                                                                                                                                                  | Buat ALB dengan `--ip-address-type ipv4`, atau layani nama DNS internal-only terpisah tanpa catatan AAAA publik                                                                                                                                                                                                  |
| Setiap permintaan Bedrock mengembalikan 502; log menunjukkan `Could not load credentials from any providers`                               | Tugas berjalan pada jenis peluncuran ECS EC2 tanpa peran tugas, atau pod berjalan pada node EKS tanpa IRSA, jadi kredensial berasal dari metadata instans, yang batas hop default IMDSv2 sebesar 1 berhenti di dalam kontainer. Tidak ada trek di halaman ini yang terpengaruh: peran tugas Fargate dan IRSA tidak menggunakan metadata instans                                                                                                                                                                                                                      | Lebih suka peran tugas dan IRSA. Jika kredensial instans tidak dapat dihindari, naikkan batas hop dengan `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; [tabel tidak bergantung pada platform](/docs/id/claude-apps-gateway-deploy#troubleshooting) mencakup tradeoff |
| Permintaan Bedrock mengembalikan `403 AccessDeniedException`                                                                               | Akun belum mengirimkan formulir kasus penggunaan Anthropic satu kali, langganan AWS Marketplace otomatis yang dimulai pada invoke pertama akun belum selesai, atau kebijakan peran tugas kehilangan ARN profil inferensi atau model dasar                                                                                                                                                                                                                                                                                                                            | Kirimkan formulir kasus penggunaan dari katalog Model konsol Bedrock; jika baru saja dikirimkan atau ini adalah invoke pertama akun, coba lagi setelah beberapa menit. Berikan `bedrock:InvokeModel` dan `bedrock:InvokeModelWithResponseStream` pada kedua keluarga ARN.                                        |
| Bedrock mengembalikan `ValidationException` mengatakan throughput on-demand tidak didukung                                                 | Entri `models:` khusus memetakan ke ID model dasar telanjang yang hanya dilayani wilayah melalui profil inferensi                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Petakan model ke ID profil inferensi lintas wilayahnya (`us.anthropic.*`) sebagai gantinya; katalog bawaan sudah melakukan ini                                                                                                                                                                                   |
| Tugas ECS berhenti dengan `ResourceInitializationError` sebelum gateway mencatat apa pun                                                   | Peran eksekusi tidak dapat membaca rahasia Secrets Manager, atau subnet pribadi tidak memiliki jalur ke Secrets Manager atau ECR                                                                                                                                                                                                                                                                                                                                                                                                                                     | Berikan `secretsmanager:GetSecretValue` pada ARN tiga rahasia `gateway-` ke peran eksekusi, dan sediakan egress melalui gateway NAT, atau, tanpanya, endpoint antarmuka untuk Secrets Manager, ECR, dan CloudWatch Logs, yang driver `awslogs` butuhkan pada tahap yang sama, ditambah endpoint gateway S3       |
| Boot gateway keluar dengan kesalahan timeout koneksi Postgres                                                                              | Grup keamanan database tidak mengakui grup keamanan gateway di 5432, atau layanan berjalan di luar VPC database                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Izinkan 5432 dari grup keamanan gateway di database, dan jalankan layanan di VPC yang sama dengan grup subnet DB                                                                                                                                                                                                 |
| Boot gateway keluar dengan kesalahan verifikasi sertifikat TLS Postgres                                                                    | String koneksi menetapkan `sslmode=verify-full` tetapi gambar tidak mempercayai bundel RDS CA: bundel tidak disalin ke dalam gambar, atau `NODE_EXTRA_CA_CERTS` tidak menunjuk ke sana                                                                                                                                                                                                                                                                                                                                                                               | Tambahkan dua baris Dockerfile langkah build yang menyalin bundel dan menetapkan `NODE_EXTRA_CA_CERTS`, kemudian bangun kembali, dorong di bawah tag baru, dan terapkan kembali                                                                                                                                  |
| Respons streaming jatuh di tengah aliran setelah periode tenang                                                                            | Gateway yang lebih lama dari v2.1.229 di upstream Bedrock atau Claude Platform di AWS mengirimkan tidak ada saat upstream tenang, misalnya selama pemikiran diperpanjang tanpa output yang dialirkan. ALB menutup koneksi setelah 60 detik tanpa data secara default, jadi memotong aliran di celah itu. Gateway v2.1.229 dan yang lebih baru menjaga aliran tenang di bawah timeout itu: di upstream tersebut gateway memancarkan acara SSE `ping` sekali sekitar 15 detik berlalu tanpa data aliran, dan di upstream API Anthropic itu meneruskan ping API sendiri | Perbarui gateway ke v2.1.229 atau lebih baru, atau atur atribut `idle_timeout.timeout_seconds` ke `3600`, melalui `modify-load-balancer-attributes` atau anotasi `load-balancer-attributes` Ingress di EKS                                                                                                       |

<h2 id="telemetry">
  Telemetri
</h2>

Gateway memberi Anda metrik penggunaan per-pengembang tanpa konfigurasi OTEL per-mesin. Claude Code memancarkan metrik OpenTelemetry (OTLP), log, dan jejak opt-in; [Monitoring usage](/docs/id/monitoring-usage) mencakup semua yang dilaporkan CLI. Pada sesi gateway CLI memberi stempel setiap ekspor dengan atribut identitas IdP yang terauthentikasi `user.id`, `user.email`, dan `user.groups`, jadi penggunaan bergulir per pengembang tanpa pipa `OTEL_RESOURCE_ATTRIBUTES`.

Gateway sendiri adalah relai OTLP yang terauthentikasi. Atur [`telemetry.forward_to`](/docs/id/claude-apps-gateway-config#telemetry) bersama dengan `listen.public_url`, dan itu mendorong pengaturan pengekspor OTEL ke setiap klien yang terhubung dan meneruskan lalu lintas OTLP mereka verbatim ke setiap tujuan yang Anda daftarkan. Setiap tujuan memilih metrik, log, dan jejak secara independen, dan default adalah metrik saja; lihat [referensi `telemetry`](/docs/id/claude-apps-gateway-config#telemetry) untuk bidang per-sinyal dan tradeoff sensitivitas mereka. Gateway tidak membuffer, mengagregasi, atau menyimpan telemetri, jadi di mana data mendarat sepenuhnya adalah konfigurasi pengekspor pengumpul.

Telemetri klien dimatikan secara default; mengonfigurasi `telemetry.forward_to` adalah apa yang mengaktifkannya untuk pengembang yang terhubung, dan setiap klien interaktif menunjukkan dialog persetujuan keamanan satu kali untuk pengaturan yang didorong, seperti yang dijelaskan dalam [referensi konfigurasi](/docs/id/claude-apps-gateway-config#telemetry). Di AWS, setiap sinyal memetakan ke tujuan sebagai berikut.

<h3 id="client-metrics-logs-and-traces">
  Metrik, log, dan jejak klien
</h3>

Arahkan `telemetry.forward_to` ke pengumpul OpenTelemetry, seperti [AWS Distro untuk OpenTelemetry (ADOT) collector](https://aws-otel.github.io/), dan ekspor dari sana ke Amazon CloudWatch, Amazon Managed Service untuk Prometheus, atau backend OTLP apa pun.

Jalankan pengumpul sebagai layanan internal sendiri yang dapat dijangkau melalui `https://`; [referensi `telemetry`](/docs/id/claude-apps-gateway-config#telemetry) mencakup pengecualian loopback dan `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Log gateway
</h3>

Di ECS Fargate, tidak ada setup tambahan: driver `awslogs` mengirimkan stderr gateway, yang membawa acara audit dan log operasionalnya, ke grup log `/ecs/claude-gateway` yang dibuat di atas. Di EKS, log pod tidak mencapai CloudWatch secara default, jadi jejak audit hilang sampai Anda memasang pengumpulan log: add-on Amazon CloudWatch Observability dengan penangkapan log kontainer diaktifkan, atau DaemonSet Fluent Bit. Di trek mana pun, kueri log dengan CloudWatch Logs Insights dan drive alarm dari filter metrik.

<h3 id="container-metrics">
  Metrik kontainer
</h3>

Aktifkan Container Insights pada kluster dengan `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` untuk CPU, memori, dan jaringan per-tugas. Di EKS, pasang add-on Amazon CloudWatch Observability.

<h3 id="spend">
  Pengeluaran
</h3>

Telemetri menunjukkan penggunaan setelah fakta; [batas pengeluaran](/docs/id/claude-apps-gateway-spend-limits) adalah tampilan dan penegakan gateway per-pengembang langsung di atas kredensial upstream bersama.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Referensi konfigurasi](/docs/id/claude-apps-gateway-config): setiap opsi `gateway.yaml`, termasuk `managed.policies` dan `telemetry`
* [Deployment dan operasi](/docs/id/claude-apps-gateway-deploy): pengaturan IdP, pemeriksaan kesehatan, rotasi rahasia JWT, upgrade, dan model keamanan
* [Gambaran umum gateway aplikasi Claude](/docs/id/claude-apps-gateway): quickstart dan menghubungkan pengembang
* [Sampel AWS untuk gateway aplikasi Claude](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): sampel yang dipertahankan AWS mencakup berbagai lingkungan pelanggan
