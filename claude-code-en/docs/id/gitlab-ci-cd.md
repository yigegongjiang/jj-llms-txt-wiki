> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Pelajari tentang mengintegrasikan Claude Code ke dalam alur kerja pengembangan Anda dengan GitLab CI/CD

<Info>
  Claude Code untuk GitLab CI/CD saat ini dalam versi beta. Fitur dan fungsionalitas dapat berkembang saat kami menyempurnakan pengalaman.

  Integrasi ini dikelola oleh GitLab. Untuk dukungan, lihat [masalah GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776) berikut.
</Info>

<Note>
  Integrasi ini dibangun di atas [Claude Code CLI dan Agent SDK](/docs/id/agent-sdk/overview), memungkinkan penggunaan Claude secara terprogram dalam pekerjaan CI/CD dan alur kerja otomasi khusus Anda.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Mengapa menggunakan Claude Code dengan GitLab?
</h2>

* **Pembuatan MR instan**: Jelaskan apa yang Anda butuhkan, dan Claude mengusulkan MR lengkap dengan perubahan dan penjelasan
* **Implementasi otomatis**: Ubah masalah menjadi kode yang berfungsi dengan satu perintah atau penyebutan
* **Menyadari proyek**: Claude mengikuti panduan `CLAUDE.md` Anda dan pola kode yang ada
* **Pengaturan sederhana**: Tambahkan satu pekerjaan ke `.gitlab-ci.yml` dan satu variabel CI/CD yang disembunyikan
* **Siap untuk perusahaan**: Pilih Claude API, Amazon Bedrock, atau Google Cloud's Agent Platform untuk memenuhi kebutuhan residensi data dan pengadaan
* **Aman secara default**: Berjalan di runner GitLab Anda dengan perlindungan cabang dan persetujuan Anda

<h2 id="how-it-works">
  Cara kerjanya
</h2>

Claude Code menggunakan GitLab CI/CD untuk menjalankan tugas AI dalam pekerjaan terisolasi dan melakukan commit hasil kembali melalui MR:

1. **Orkestrasi berbasis peristiwa**: GitLab mendengarkan pemicu pilihan Anda (misalnya, komentar yang menyebutkan `@claude` dalam masalah, MR, atau utas ulasan). Pekerjaan mengumpulkan konteks dari utas dan repositori, membangun prompt dari input tersebut, dan menjalankan Claude Code.

2. **Abstraksi penyedia**: Gunakan penyedia yang sesuai dengan lingkungan Anda:
   * Claude API (SaaS)
   * Amazon Bedrock (akses berbasis IAM, opsi lintas wilayah)
   * Google Cloud's Agent Platform (asli GCP, Workload Identity Federation)

3. **Eksekusi bersandbox**: Setiap interaksi berjalan dalam kontainer dengan aturan jaringan dan sistem file yang ketat. Claude Code memberlakukan izin berskop ruang kerja untuk membatasi penulisan. Setiap perubahan mengalir melalui MR sehingga pengulas melihat diff dan persetujuan masih berlaku.

Pilih titik akhir regional untuk mengurangi latensi dan memenuhi persyaratan kedaulatan data sambil menggunakan perjanjian cloud yang ada.

<h2 id="what-can-claude-do">
  Apa yang dapat dilakukan Claude?
</h2>

Dalam pipeline GitLab, Claude Code dapat:

* Membuat dan memperbarui MR dari deskripsi atau komentar masalah
* Menganalisis regresi kinerja dan mengusulkan optimisasi
* Menerapkan fitur langsung di cabang, kemudian membuka MR
* Memperbaiki bug dan regresi yang diidentifikasi oleh tes atau komentar
* Merespons komentar lanjutan untuk mengulangi perubahan yang diminta

<h2 id="setup">
  Pengaturan
</h2>

<h3 id="quick-setup">
  Pengaturan cepat
</h3>

Cara tercepat untuk memulai adalah dengan menambahkan pekerjaan minimal ke `.gitlab-ci.yml` Anda dan menetapkan kunci API Anda sebagai variabel yang disembunyikan.

1. **Tambahkan variabel CI/CD yang disembunyikan**
   * Buka **Settings** → **CI/CD** → **Variables**
   * Tambahkan `ANTHROPIC_API_KEY` (disembunyikan, dilindungi sesuai kebutuhan)

2. **Tambahkan pekerjaan Claude ke `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Sesuaikan aturan untuk menyesuaikan cara Anda ingin memicu pekerjaan:
  # - menjalankan secara manual
  # - peristiwa permintaan penggabungan
  # - pemicu web/API ketika komentar berisi '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # Penginstal menempatkan claude di ~/.local/bin, yang tidak ada di PATH dalam gambar ini
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Opsional: mulai server GitLab MCP jika pengaturan Anda menyediakannya
    - /bin/gitlab-mcp-server || true
    # Gunakan variabel AI_FLOW_* saat memanggil melalui pemicu web/API dengan muatan konteks
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Setelah menambahkan pekerjaan dan variabel `ANTHROPIC_API_KEY` Anda, uji dengan menjalankan pekerjaan secara manual dari **CI/CD** → **Pipelines**, atau picu dari MR untuk membiarkan Claude mengusulkan pembaruan di cabang dan membuka MR jika diperlukan.

<Note>
  Untuk menjalankan di Amazon Bedrock atau Platform Agent Google Cloud alih-alih Claude API, lihat bagian [Using with Amazon Bedrock and Google Cloud](#using-with-amazon-bedrock-and-google-cloud) di bawah untuk pengaturan autentikasi dan lingkungan.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Pengaturan manual (direkomendasikan untuk produksi)
</h3>

Jika Anda lebih suka pengaturan yang lebih terkontrol atau memerlukan penyedia perusahaan:

1. **Konfigurasi akses penyedia**:
   * **Claude API**: Buat dan simpan `ANTHROPIC_API_KEY` sebagai variabel CI/CD yang disembunyikan
   * **Amazon Bedrock**: **Konfigurasi GitLab** → **AWS OIDC** dan buat peran IAM untuk Amazon Bedrock
   * **Platform Agent Google Cloud**: **Konfigurasi Workload Identity Federation untuk GitLab** → **GCP**

2. **Tambahkan kredensial proyek untuk operasi GitLab API**:
   * Gunakan `CI_JOB_TOKEN` secara default, atau buat Project Access Token dengan cakupan `api`
   * Simpan sebagai `GITLAB_ACCESS_TOKEN` (disembunyikan) jika menggunakan PAT

3. **Tambahkan pekerjaan Claude ke `.gitlab-ci.yml`**: gunakan pekerjaan [Quick setup](#quick-setup) untuk Claude API, atau pekerjaan penyedia dari [Configuration examples](#configuration-examples)

4. **(Opsional) Aktifkan pemicu berbasis penyebutan**:
   * Tambahkan webhook proyek untuk "Comments (notes)" ke pendengar acara Anda (jika Anda menggunakannya)
   * Biarkan pendengar memanggil API pemicu pipeline dengan variabel seperti `AI_FLOW_INPUT` dan `AI_FLOW_CONTEXT` ketika komentar berisi `@claude`

<h2 id="example-use-cases">
  Contoh kasus penggunaan
</h2>

<h3 id="turn-issues-into-mrs">
  Ubah masalah menjadi MR
</h3>

Dalam komentar masalah:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude menganalisis masalah dan basis kode, menulis perubahan di cabang, dan membuka MR untuk ditinjau.

<h3 id="get-implementation-help">
  Dapatkan bantuan implementasi
</h3>

Dalam diskusi MR:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude mengusulkan perubahan, menambahkan kode dengan caching yang sesuai, dan memperbarui MR.

<h3 id="fix-bugs-quickly">
  Perbaiki bug dengan cepat
</h3>

Dalam komentar masalah atau MR:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude menemukan bug, mengimplementasikan perbaikan, dan memperbarui cabang atau membuka MR baru.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Menggunakan dengan Amazon Bedrock dan Google Cloud
</h2>

Untuk lingkungan perusahaan, Anda dapat menjalankan Claude Code sepenuhnya pada infrastruktur cloud Anda dengan pengalaman pengembang yang sama.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Prasyarat

    Sebelum menyiapkan Claude Code dengan Amazon Bedrock, Anda memerlukan:

    1. Akun AWS dengan akses Amazon Bedrock ke model Claude yang diinginkan
    2. GitLab dikonfigurasi sebagai penyedia identitas OIDC di AWS IAM
    3. Peran IAM dengan izin Amazon Bedrock dan kebijakan kepercayaan yang dibatasi pada proyek/ref GitLab Anda
    4. Variabel CI/CD GitLab untuk asumsi peran:
       * `AWS_ROLE_TO_ASSUME` (ARN peran)
       * `AWS_REGION` (wilayah Amazon Bedrock)

    ### Instruksi penyiapan

    Konfigurasikan AWS untuk memungkinkan pekerjaan CI GitLab mengasumsikan peran IAM melalui OIDC (tanpa kunci statis).

    **Penyiapan yang diperlukan:**

    1. Aktifkan Amazon Bedrock dan minta akses ke model Claude target Anda
    2. Buat penyedia OIDC IAM untuk GitLab jika belum ada
    3. Buat peran IAM yang dipercaya oleh penyedia OIDC GitLab, dibatasi pada proyek dan ref yang dilindungi Anda
    4. Lampirkan izin least-privilege untuk API invoke Amazon Bedrock

    Gunakan [contoh pekerjaan Amazon Bedrock](#configuration-examples) untuk menukar token OIDC pekerjaan dengan kredensial AWS sementara saat runtime.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Prasyarat

    Sebelum menyiapkan Claude Code dengan Google Cloud's Agent Platform, Anda memerlukan:

    1. Proyek Google Cloud dengan:
       * API Google Cloud's Agent Platform diaktifkan
       * Workload Identity Federation dikonfigurasi untuk mempercayai GitLab OIDC
    2. Akun layanan khusus dengan hanya peran Google Cloud's Agent Platform yang diperlukan
    3. Variabel CI/CD GitLab:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nama sumber daya penyedia tanpa awalan `//iam.googleapis.com/`, seperti `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (email akun layanan)
       * `GCP_PROJECT_ID` (ID proyek Google Cloud)

    ### Instruksi penyiapan

    Konfigurasikan Google Cloud untuk memungkinkan pekerjaan CI GitLab menyamar sebagai akun layanan melalui Workload Identity Federation.

    **Penyiapan yang diperlukan:**

    1. Aktifkan IAM Credentials API, STS API, dan Google Cloud's Agent Platform API
    2. Buat Workload Identity Pool dan penyedia untuk GitLab OIDC
    3. Buat akun layanan khusus dengan peran Google Cloud's Agent Platform
    4. Berikan izin prinsip WIF untuk menyamar sebagai akun layanan

    Gunakan [contoh pekerjaan Agent Platform](#configuration-examples) untuk autentikasi tanpa menyimpan kunci.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Contoh konfigurasi
</h2>

Di bawah ini adalah cuplikan siap pakai yang dapat Anda sesuaikan dengan pipeline Anda.

<h3 id="amazon-bedrock-job-example-oidc">
  Contoh pekerjaan Amazon Bedrock (OIDC)
</h3>

**Prasyarat:**

* Amazon Bedrock diaktifkan dengan akses ke model Claude pilihan Anda
* GitLab OIDC dikonfigurasi di AWS dengan peran yang mempercayai proyek dan refs GitLab Anda
* Peran IAM dengan izin Amazon Bedrock (least privilege direkomendasikan)

**Variabel CI/CD yang diperlukan:**

* `AWS_ROLE_TO_ASSUME`: ARN dari peran IAM untuk akses Amazon Bedrock
* `AWS_REGION`: Wilayah Amazon Bedrock (misalnya, `us-west-2`)

GitLab membuat token OIDC pekerjaan dari blok `id_tokens:` dan mengeksposnya sebagai `GITLAB_OIDC_TOKEN`. Atur `aud` ke nilai audiens yang Anda konfigurasi pada penyedia identitas OIDC IAM di AWS, misalnya URL instans GitLab Anda.

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
  ID model untuk Amazon Bedrock mencakup awalan khusus wilayah (misalnya, `us.anthropic.claude-sonnet-4-6`). Teruskan model yang diinginkan melalui konfigurasi pekerjaan atau prompt Anda jika alur kerja Anda mendukungnya.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Contoh pekerjaan Agent Platform (Workload Identity Federation)
</h3>

**Prasyarat:**

* API Agent Platform Google Cloud diaktifkan di proyek GCP Anda
* Workload Identity Federation dikonfigurasi untuk mempercayai GitLab OIDC
* Akun layanan dengan izin Agent Platform Google Cloud

**Variabel CI/CD yang diperlukan:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: nama sumber daya penyedia tanpa awalan `//iam.googleapis.com/`, seperti `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: email akun layanan
* `GCP_PROJECT_ID`: ID proyek Google Cloud
* `CLOUD_ML_REGION`: Wilayah Agent Platform Google Cloud (misalnya, `us-east5`)

GitLab membuat token OIDC pekerjaan dari blok `id_tokens:` dan mengeksposnya sebagai `GITLAB_OIDC_TOKEN`. Atur `aud` ke nilai audiens yang Anda konfigurasi pada penyedia Workload Identity Pool, misalnya URL instans GitLab Anda. Pekerjaan menulis token ke file, dan entri `credential_source` konfigurasi kredensial memberi tahu perpustakaan auth Google untuk membacanya dari sana. Mengatur `GOOGLE_APPLICATION_CREDENTIALS` ke file konfigurasi kredensial membuatnya tersedia untuk Claude Code melalui [Application Default Credentials](/docs/id/google-vertex-ai#3-configure-gcp-credentials).

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
  Dengan Workload Identity Federation, Anda tidak perlu menyimpan kunci akun layanan. Gunakan kondisi kepercayaan khusus repositori dan akun layanan dengan least-privilege.
</Note>

<h2 id="best-practices">
  Praktik terbaik
</h2>

<h3 id="claude-md-configuration">
  Konfigurasi CLAUDE.md
</h3>

Buat file `CLAUDE.md` di akar repositori untuk mendefinisikan standar pengkodean, kriteria tinjauan, dan aturan khusus proyek. Claude membaca file ini selama menjalankan dan mengikuti konvensi Anda saat mengusulkan perubahan.

<h3 id="security-considerations">
  Pertimbangan keamanan
</h3>

**Jangan pernah melakukan commit kunci API atau kredensial cloud ke repositori Anda**. Selalu gunakan variabel GitLab CI/CD:

* Tambahkan `ANTHROPIC_API_KEY` sebagai variabel yang disembunyikan (dan lindungi jika diperlukan)
* Gunakan OIDC khusus penyedia jika memungkinkan (tanpa kunci yang tahan lama)
* Batasi izin pekerjaan dan egress jaringan
* Tinjau MR Claude seperti kontributor lainnya

<h3 id="optimizing-performance">
  Mengoptimalkan kinerja
</h3>

* Jaga `CLAUDE.md` tetap fokus dan ringkas
* Berikan deskripsi masalah/MR yang jelas untuk mengurangi iterasi
* Cache npm dan instalasi paket di runner jika memungkinkan

<h3 id="ci-costs">
  Biaya CI
</h3>

Saat menggunakan Claude Code dengan GitLab CI/CD, waspadai biaya terkait:

* **Waktu GitLab Runner**:
  * Claude berjalan di runner GitLab Anda dan mengonsumsi menit komputasi
  * Lihat penagihan runner rencana GitLab Anda untuk detail

* **Biaya API**:
  * Setiap interaksi Claude mengonsumsi token berdasarkan ukuran prompt dan respons
  * Penggunaan token bervariasi menurut kompleksitas tugas dan ukuran basis kode
  * Lihat [harga Anthropic](https://platform.claude.com/docs/en/about-claude/pricing) untuk detail

* **Tips optimasi biaya**:
  * Gunakan perintah `@claude` spesifik untuk mengurangi putaran yang tidak perlu
  * Tetapkan nilai `--max-turns` dan `timeout` pekerjaan yang sesuai
  * Batasi konkurensi untuk mengontrol jalankan paralel

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude tidak merespons perintah @claude
</h3>

* Verifikasi pipeline Anda dipicu (secara manual, acara MR, atau melalui pendengar acara catatan/webhook)
* Pastikan `ANTHROPIC_API_KEY` atau variabel penyedia cloud Anda ada
* Periksa bahwa komentar berisi `@claude` (bukan `/claude`) dan bahwa pemicu penyebutan Anda dikonfigurasi

<h3 id="job-can’t-write-comments-or-open-mrs">
  Job tidak dapat menulis komentar atau membuka MR
</h3>

* Pastikan `CI_JOB_TOKEN` memiliki izin yang cukup untuk proyek, atau gunakan Project Access Token dengan cakupan `api`
* Periksa bahwa alat `mcp__gitlab` diaktifkan dalam `--allowedTools`
* Konfirmasi job berjalan dalam konteks MR atau memiliki konteks yang cukup melalui variabel `AI_FLOW_*`

<h3 id="authentication-errors">
  Kesalahan autentikasi
</h3>

* **Untuk Claude API**: Konfirmasi `ANTHROPIC_API_KEY` valid dan tidak kadaluarsa
* **Untuk Amazon Bedrock atau Platform Agent Google Cloud**: Verifikasi konfigurasi OIDC/WIF, impersonasi peran, dan nama rahasia; konfirmasi ketersediaan wilayah dan model

<h2 id="advanced-configuration">
  Konfigurasi lanjutan
</h2>

<h3 id="common-parameters-and-variables">
  Parameter dan variabel umum
</h3>

Kontrol Claude Code berjalan di pekerjaan Anda dengan bendera CLI, kata kunci GitLab, dan variabel ini:

* `-p`: berikan instruksi secara inline, misalnya `claude -p "Review this MR"`
* `--max-turns`: batasi jumlah iterasi bolak-balik
* `timeout`: batasi total waktu eksekusi pekerjaan dengan kata kunci `timeout` tingkat pekerjaan GitLab, misalnya `timeout: 30m`
* `ANTHROPIC_API_KEY`: diperlukan untuk Claude API (tidak digunakan untuk Amazon Bedrock atau Agent Platform Google Cloud)
* Lingkungan khusus penyedia: `AWS_REGION`, variabel proyek/region untuk Agent Platform Google Cloud

<Note>
  Bendera dan parameter yang tepat mungkin berbeda menurut versi `@anthropic-ai/claude-code`. Jalankan `claude --help` di pekerjaan Anda untuk melihat opsi yang didukung.
</Note>

<h3 id="customizing-claude’s-behavior">
  Menyesuaikan perilaku Claude
</h3>

Anda dapat memandu Claude dengan dua cara utama:

1. **CLAUDE.md**: Tentukan standar pengkodean, persyaratan keamanan, dan konvensi proyek. Claude membaca ini selama berjalan dan mengikuti aturan Anda.
2. **Prompt kustom**: Berikan instruksi khusus tugas melalui `-p` di pekerjaan. Gunakan prompt berbeda untuk pekerjaan berbeda (misalnya, review, implementasi, refactor).
