> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Terapkan gateway aplikasi Claude di Google Cloud

> Contoh praktis menjalankan gateway aplikasi Claude di Google Cloud: Cloud Run atau GKE, Cloud SQL untuk PostgreSQL, Secret Manager, dan autentikasi service-account ke Agent Platform Google Cloud.

<Note>
  Halaman ini menjelaskan satu cara untuk menjalankan gateway aplikasi Claude di Google Cloud. Konfigurasi ini adalah contoh kerja untuk infrastruktur yang dikelola pelanggan daripada penyebaran produksi yang didukung; gunakan ini untuk melihat bagaimana potongan-potongan cocok bersama sebelum menyesuaikannya dengan lingkungan Anda sendiri. Untuk persyaratan yang tidak bergantung pada platform, lihat [panduan penyebaran](/docs/id/claude-apps-gateway-deploy).
</Note>

Contoh ini menyediakan gateway aplikasi Claude di Google Cloud dengan Agent Platform Google Cloud sebagai upstream model, menggunakan Cloud Run atau GKE untuk komputasi. Google Workspace adalah penyedia identitas contoh (IdP), tetapi penyedia IdP yang sesuai dengan OpenID Connect (OIDC) apa pun berfungsi; hanya blok `oidc` yang berubah. Lihat [Penyiapan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup) untuk detail per-IdP.

<h2 id="what-you’ll-build">
  Apa yang akan Anda bangun
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Diagram gateway aplikasi Claude di Google Cloud: klien Claude Code terhubung melalui HTTPS ke gateway (Cloud Run atau GKE), yang berjalan di dalam VPC bersama database Cloud SQL IP pribadi untuk status sesi. Gateway menandatangani pengguna melalui OIDC terhadap Google Workspace, membaca konfigurasi dan rahasia dari Secret Manager, meneruskan permintaan model ke Agent Platform, dan menarik gambarnya dari Artifact Registry saat penyebaran." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

Konfigurasi referensi menyediakan:

* Layanan **Cloud Run** atau **GKE** Deployment yang menjalankan kontainer gateway
* Repositori **Artifact Registry** untuk gambar gateway
* Instans **Cloud SQL untuk PostgreSQL**, IP pribadi saja, untuk [store](/docs/id/claude-apps-gateway-config#store) gateway
* Rahasia **Secret Manager** untuk `gateway.yaml`, kunci penandatanganan JWT, rahasia klien OIDC, dan URL Postgres
* **Service account** dengan `roles/aiplatform.user`, terlampir langsung di Cloud Run atau terikat melalui Workload Identity di GKE
* **Internal Application Load Balancer** di Cloud Run, atau **GKE Ingress** internal dari kelas `gce-internal` di GKE, untuk HTTPS

<h2 id="prerequisites">
  Prasyarat
</h2>

* Proyek GCP dengan penagihan diaktifkan, dan izin untuk membuat sumber daya di atas
* CLI `gcloud`, diautentikasi dengan `gcloud auth login`, dan Docker dipasang secara lokal
* Untuk jalur GKE: `kubectl`, dan kluster GKE di VPC yang dibuat dalam panduan di bawah
* Akses ke model Claude yang Anda butuhkan di Model Garden, di wilayah yang menerbitkannya
* Klien aplikasi web OAuth 2.0 Google Workspace dengan URI pengalihan `https://<gateway-host>/oauth/callback`; lihat [Penyiapan penyedia identitas](/docs/id/claude-apps-gateway-deploy#identity-provider-setup)
* Nama host TLS untuk gateway, biasanya nama DNS internal yang menunjuk ke load balancer

Atur proyek dan wilayah sekali:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Terapkan gateway
</h2>

Langkah-langkah di bawah menyediakan penyebaran lengkap dengan perintah `gcloud`.

<Steps>
  <Step title="Aktifkan API">
    Aktifkan API layanan yang digunakan panduan:

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

    API yang Anda butuhkan bergantung pada jalur penyebaran:

    * `compute` dan `servicenetworking`: diperlukan untuk jalur Cloud SQL IP pribadi
    * `run`: Cloud Run saja
    * `container`: GKE saja
  </Step>

  <Step title="Buat service account dan berikan IAM">
    Gateway berjalan sebagai service account khusus dengan izin untuk memanggil Agent Platform. Ini menjangkau Cloud SQL melalui VPC dengan pengguna kata sandi, jadi tidak ada peran IAM Cloud SQL yang diperlukan:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Kemudian aktifkan model Claude untuk proyek di Model Garden; model menerbitkan ke wilayah tertentu, jadi periksa setiap kartu model.
  </Step>

  <Step title="Bangun dan dorong gambar ke Artifact Registry">
    Bangun gambar sesuai dengan [persyaratan gambar kontainer](/docs/id/claude-apps-gateway-deploy#container-image), menggunakan biner glibc `linux-x64`, dan dorong:

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

  <Step title="Sediakan Cloud SQL untuk PostgreSQL">
    Buat instans di VPC melalui Private Services Access sehingga tidak memiliki IP publik; ini juga memenuhi proyek di mana `constraints/sql.restrictPublicIp` diberlakukan:

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

    Runtime Cloud Run atau GKE harus berada di, atau dirutekan ke, VPC ini.
  </Step>

  <Step title="Tulis gateway.yaml">
    Blok `upstreams` menunjuk ke Agent Platform dengan `auth: {}`, jadi gateway mengautentikasi melalui Application Default Credentials dari service account runtime. Lihat [referensi konfigurasi](/docs/id/claude-apps-gateway-config) untuk setiap bidang.

    Dua bidang `listen` bergantung pada apa yang berada di depan gateway:

    * `public_url`: diperlukan di belakang Cloud Run atau GKE Ingress. Gateway membangun `redirect_uri` IdP dan dokumen penemuannya hanya dari nilai ini, tidak pernah dari header `X-Forwarded-*`.
    * `trusted_proxies`: rentang sumber front end. Gateway menghormati `X-Forwarded-For` hanya ketika peer TCP berada dalam daftar ini, kemudian berjalan melalui rantai melewati hop terpercaya, jadi batas laju sign-in per-IP dan acara audit mencatat IP pengembang daripada load balancer.

    Atur `trusted_proxies` agar sesuai dengan front end Anda. GKE Ingress eksternal dari kelas `gce` tidak terdaftar: ini menyediakan alamat forwarding-rule publik, yang pemeriksaan [jaringan pribadi](/docs/id/claude-apps-gateway#prerequisites) `/login` menolak.

    | Front end                                             | `trusted_proxies`                                     |
    | ----------------------------------------------------- | ----------------------------------------------------- |
    | Cloud Run dijangkau langsung, tidak ada load balancer | `[169.254.0.0/16]`                                    |
    | Internal Application Load Balancer di depan Cloud Run | `169.254.0.0/16` ditambah CIDR subnet proxy-only Anda |
    | GKE internal Ingress, kelas `gce-internal`            | CIDR subnet proxy-only Anda                           |

    Contoh di bawah menggunakan nilai internal-load-balancer-in-front-of-Cloud-Run.

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
      Token id Google tidak membawa klaim `groups`. Untuk menggunakan kebijakan berbasis grup dalam [`managed.policies`](/docs/id/claude-apps-gateway-config#managed) dengan Google Workspace sebagai IdP, konfigurasikan [`oidc.google_groups`](/docs/id/claude-apps-gateway-config#oidc), yang mencari grup setiap pengguna melalui Admin SDK Directory API menggunakan service account dengan delegasi domain-wide. Tanpa itu, cocokkan pada `email_domain` sebagai gantinya.
    </Note>
  </Step>

  <Step title="Simpan rahasia di Secret Manager">
    Buat empat rahasia dan berikan `roles/secretmanager.secretAccessor` ke service account `claude-gateway`:

    | Rahasia                      | Sumber                                         |
    | ---------------------------- | ---------------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                      |
    | `gateway-oidc-client-secret` | Google Cloud Console → OAuth client            |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` dari langkah Cloud SQL |
    | `gateway-config`             | `gateway.yaml` lengkap dari langkah sebelumnya |

    Bagaimana rahasia mencapai kontainer berbeda menurut jalur:

    * Di GKE mereka dipasang sebagai file melalui driver CSI Secret Manager, dan `gateway.yaml` mereferensikan `${file:/secrets/...}`.
    * Di Cloud Run, yang tidak dapat memasang beberapa rahasia ke satu direktori, `gateway.yaml` dipasang sebagai file dan tiga lainnya disuntikkan sebagai variabel lingkungan, jadi `gateway.yaml` mereferensikan `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}`, dan `${GATEWAY_POSTGRES_URL}` sebagai gantinya.
  </Step>

  <Step title="Terapkan">
    <Tabs>
      <Tab title="Cloud Run">
        Perintah di bawah menyebarkan untuk produksi di belakang load balancer internal.

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

        Egress VPC langsung, melalui `--network`, `--subnet`, dan `--vpc-egress=private-ranges-only`, memungkinkan layanan menjangkau IP pribadi Cloud SQL secara langsung. Egress publik ke endpoint Agent Platform dan `accounts.google.com` langsung ke internet daripada melalui VPC, jadi Cloud NAT tidak diperlukan.

        Pemeriksaan IAM invoker harus terbuka atau dinonaktifkan. Gateway menjalankan OIDC-nya sendiri dan kliennya tidak membawa token GCP, jadi pemeriksaan invoker Cloud Run harus mengakui permintaan yang tidak diautentikasi. Autentikasi OIDC gateway mengautentikasi permintaan setelah mencapai kontainer, dengan `allowed_email_domains` membatasi domain mana yang dapat masuk.

        Dua bendera mengakui permintaan yang tidak diautentikasi:

        * `--no-invoker-iam-check`: menonaktifkan pemeriksaan tanpa ikatan `allUsers` untuk dikelola, dan bekerja di bawah Domain Restricted Sharing
        * `--allow-unauthenticated`: memberikan peran `run.invoker` kepada `allUsers`; gunakan jika organisasi Anda tidak memungkinkan `--no-invoker-iam-check`

        Pembatasan ingress melalui `--ingress` adalah lapisan terpisah dan independen dari pemeriksaan invoker; tetap atur untuk membatasi layanan ke jaringan perusahaan Anda.

        Secara default URL Cloud Run `*.run.app` diselesaikan ke alamat publik, yang pemeriksaan [jaringan pribadi](/docs/id/claude-apps-gateway#prerequisites) `/login` menolak. Dua topologi memberikan pengembang nama host yang dapat diselesaikan secara pribadi, dan Cloud Run tidak menyediakan keduanya untuk Anda:

        * **Internal Application Load Balancer**, topologi yang diasumsikan perintah deploy di atas: terapkan dengan `--ingress=internal-and-cloud-load-balancing`, sediakan Internal Application Load Balancer di depan layanan dengan nama DNS internal dan sertifikat, dan atur `listen.public_url` ke nama host itu.
        * **Ingress internal saja tanpa load balancer**: terapkan dengan `--ingress=internal` dan biarkan `listen.public_url` sebagai URL `*.run.app`, default dalam [aset referensi](#terraform-reference) di bawah. Agar `*.run.app` diselesaikan secara pribadi, tim jaringan Anda harus sudah mengoperasikan endpoint Private Service Connect untuk Google APIs, zona DNS pribadi Cloud yang menyelesaikan `*.run.app` ke dalamnya, dan perutean on-premises ke endpoint itu.

        [Panduan jaringan pribadi Google untuk Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) mencakup infrastruktur yang kedua opsi butuhkan. Verifikasi sign-in setelah gateway melayani pada nama host pribadi; sampai saat itu, konfirmasi kontainer boot dari lognya di Cloud Run.

        Perbarui URI pengalihan otorisasi klien OAuth ke `<public_url>/oauth/callback` sebelum sign-in pertama. Terapkan ulang setelah mengubah `public_url`, karena gateway membangun asal publiknya hanya dari pengaturan itu dan mengabaikan `X-Forwarded-Host` dan `X-Forwarded-Proto`. `X-Forwarded-For` dihormati untuk IP klien hanya ketika `listen.trusted_proxies` diatur.
      </Tab>

      <Tab title="GKE">
        Kluster harus berada di `$VPC` yang dibuat dalam langkah Cloud SQL sehingga pod dapat menjangkau IP pribadi database; peering VPC saja tidak berfungsi, karena IP pribadi Cloud SQL itu sendiri adalah jaringan yang di-peer dan peering tidak transitif. Untuk membuat kluster baru di VPC itu, teruskan `--network="$VPC" --subnetwork=cc-gateway-subnet` ke `gcloud container clusters create`.

        Aktifkan Workload Identity di kluster dan pool node-nya, kemudian ikat service account Google ke service account Kubernetes sehingga pod mewarisi kredensialnya:

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

        Terapkan gateway sebagai Deployment standar ditambah Service dan Ingress internal, kelas `gce-internal`, seperti dijelaskan dalam [penyebaran Kubernetes](/docs/id/claude-apps-gateway-deploy#kubernetes), dengan:

        * `serviceAccountName: gateway`
        * driver CSI Secret Manager memasang rahasia di `/secrets`
        * probe kesiapan menunjuk ke `GET /readyz`

        Lampirkan BackendConfig dengan `timeoutSec` yang ditingkatkan ke Service gateway: layanan backend load balancer di belakang GKE Ingress default ke timeout 30 detik, yang memotong respons streaming panjang.

        Jangan terapkan NetworkPolicy egress yang memblokir `169.254.169.254` di kluster Workload Identity; pod harus menjangkau server metadata untuk kredensial. [Penjaga SSRF](/docs/id/claude-apps-gateway-deploy#threat-model-summary) gateway yang tertanam adalah pertahanan di sana.

        Gateway mencatat peringatan boot bahwa endpoint metadata dapat dijangkau dan menyarankan menerapkan NetworkPolicy egress. Di bawah Workload Identity peringatan itu diharapkan, karena pod membutuhkan endpoint.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Dorong URL gateway ke mesin pengembang">
    Gateway sekarang berjalan, tetapi pengembang tidak dapat menjangkaunya dari `/login` sampai URL gateway berada di mesin mereka. Atur `forceLoginMethod` dan `forceLoginGatewayUrl` dalam [file pengaturan terkelola](/docs/id/claude-apps-gateway#set-the-gateway-url) yang Anda terapkan ke setiap perangkat melalui MDM. Tidak ada opsi gateway di pemilih login untuk pengembang memilih secara manual.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Referensi Terraform
</h2>

[Aset penyebaran referensi](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) mengotomatisasi jalur Cloud Run di halaman ini; aset konfigurasi dan gambar berlaku untuk kedua jalur:

* `setup.sh`: penyedia `gcloud` idempoten yang berjalan melalui jalur Cloud Run lengkap, dari mengaktifkan API melalui penyebaran pertama
* `terraform/`: penyebaran yang sama sebagai infrastruktur-sebagai-kode, untuk penyebaran greenfield: penerapan bertarget untuk membuat repositori Artifact Registry, kemudian membangun dan mendorong gambar, kemudian penerapan penuh
* `gateway.yaml.example` dan `Dockerfile` untuk gambar runtime distroless

Artefak default ingress Cloud Run ke `internal`, jadi tidak ada load balancer yang diperlukan. Untuk mencocokkan penyebaran produksi-di-belakang-ALB halaman ini, jalankan `setup.sh` dengan `INGRESS=internal-and-cloud-load-balancing`, atau atur variabel Terraform `ingress` ke `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Artefak juga default lapisan invoker ke pemberian `run.invoker` `allUsers` daripada `--no-invoker-iam-check`, kebalikan dari panduan halaman ini; keduanya berfungsi, dan pilihan bergantung pada batasan kebijakan organisasi Anda.

Aset disediakan sebagai contoh kerja, bukan sebagai artefak produksi yang didukung; tinjau dan sesuaikan dengan lingkungan Anda.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Untuk boot gateway dan kesalahan login, lihat tabel [troubleshooting](/docs/id/claude-apps-gateway-deploy#troubleshooting) yang tidak bergantung pada platform. Entri di bawah khusus untuk Google Cloud.

| Gejala                                                                                    | Penyebab                                                                                                                           | Perbaikan                                                                                                                                                                                                                          |
| ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run mengembalikan `403 Forbidden` sebelum mencapai kontainer                        | Pemeriksaan IAM invoker masih diaktifkan                                                                                           | Terapkan dengan `--no-invoker-iam-check`, atau berikan peran `run.invoker` kepada `allUsers` dengan `--allow-unauthenticated`                                                                                                      |
| `--no-invoker-iam-check` ditolak dengan `invoker_iam_disabled is not currently available` | Diblokir oleh `constraints/run.managed.requireInvokerIam`                                                                          | Gunakan `--allow-unauthenticated`. Jika Domain Restricted Sharing melalui `constraints/iam.allowedPolicyMemberDomains` juga memblokir itu, gunakan jalur GKE, yang mengekspos gateway di lapisan jaringan tanpa ikatan `allUsers`. |
| `Container manifest type … must support amd64/linux` saat penyebaran                      | Gambar dibangun di host non-amd64, atau buildx memancarkan indeks gambar OCI                                                       | Bangun dengan `--platform=linux/amd64 --provenance=false`                                                                                                                                                                          |
| Boot gateway keluar dengan kesalahan timeout koneksi Postgres di Cloud Run                | Layanan tidak terlampir ke VPC, atau Cloud SQL tidak memiliki IP pribadi di VPC itu; store berhenti menunggu setelah 5 detik       | Terapkan dengan `--network` dan `--subnet` untuk egress VPC Langsung, dan buat instans Cloud SQL dengan `--no-assign-ip` dan `--network` menunjuk ke VPC yang sama                                                                 |
| Permintaan Agent Platform mengembalikan `403 PERMISSION_DENIED`                           | Runtime tidak menggunakan service account `claude-gateway`, atau model tidak diaktifkan di Model Garden untuk proyek               | Atur `--service-account` di Cloud Run atau ikat Workload Identity di GKE, dan aktifkan setiap model Claude di Model Garden untuk wilayah target                                                                                    |
| Respons streaming terpotong setelah durasi tetap                                          | Timeout permintaan front-end: layanan backend load balancer di belakang GKE Ingress default ke 30 detik dan Cloud Run ke 300 detik | Lampirkan BackendConfig dengan `timeoutSec` yang ditingkatkan di GKE, atau terapkan dengan `--timeout=3600` di Cloud Run                                                                                                           |

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Referensi konfigurasi](/docs/id/claude-apps-gateway-config): setiap opsi `gateway.yaml`, termasuk `managed.policies` dan `telemetry`
* [Penyebaran dan operasi](/docs/id/claude-apps-gateway-deploy): penyiapan IdP, pemeriksaan kesehatan, rotasi rahasia JWT, upgrade, dan model keamanan
* [Gambaran umum gateway aplikasi Claude](/docs/id/claude-apps-gateway): quickstart dan menghubungkan pengembang
