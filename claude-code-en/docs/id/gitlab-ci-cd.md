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

Claude Code memungkinkan alur kerja CI/CD yang kuat yang mengubah cara Anda bekerja dengan kode:

* Buat dan perbarui MR dari deskripsi masalah atau komentar
* Analisis regresi kinerja dan usulkan optimisasi
* Implementasikan fitur langsung di cabang, kemudian buka MR
* Perbaiki bug dan regresi yang diidentifikasi oleh tes atau komentar
* Merespons komentar lanjutan untuk mengulangi perubahan yang diminta

<h2 id="setup">
  Pengaturan
</h2>

<h3 id="quick-setup">
  Pengaturan cepat
</h3>

Cara tercepat untuk memulai adalah menambahkan pekerjaan minimal ke `.gitlab-ci.yml` Anda dan menetapkan kunci API Anda sebagai variabel yang disembunyikan.

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
  # Sesuaikan aturan agar sesuai dengan cara Anda ingin memicu pekerjaan:
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
  Untuk menjalankan di Amazon Bedrock atau Google Cloud's Agent Platform alih-alih Claude API, lihat bagian [Menggunakan dengan Amazon Bedrock dan Google Cloud](#using-with-amazon-bedrock-and-google-cloud) di bawah untuk pengaturan autentikasi dan lingkungan.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Pengaturan manual (direkomendasikan untuk produksi)
</h3>

Jika Anda lebih suka pengaturan yang lebih terkontrol atau memerlukan penyedia perusahaan:

1. **Konfigurasi akses penyedia**:
   * **Claude API**: Buat dan simpan `ANTHROPIC_API_KEY` sebagai variabel CI/CD yang disembunyikan
   * **Amazon Bedrock**: **Konfigurasi GitLab** → **AWS OIDC** dan buat peran IAM untuk Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Konfigurasi Workload Identity Federation untuk GitLab** → **GCP**

2. **Tambahkan kredensial proyek untuk operasi GitLab API**:
   * Gunakan `CI_JOB_TOKEN` secara default, atau buat Project Access Token dengan cakupan `api`
   * Simpan sebagai `GITLAB_ACCESS_TOKEN` (disembunyikan) jika menggunakan PAT

3. **Tambahkan pekerjaan Claude ke `.gitlab-ci.yml`** (lihat contoh di bawah)

4. **(Opsional) Aktifkan pemicu berbasis penyebutan**:
   * Tambahkan webhook proyek untuk "Comments (notes)" ke pendengar peristiwa Anda (jika Anda menggunakannya)
   * Buat pendengar memanggil API pemicu pipeline dengan variabel seperti `AI_FLOW_INPUT` dan `AI_FLOW_CONTEXT` ketika komentar berisi `@claude`

<h2 id="example-use-cases">
  Contoh kasus penggunaan
</h2>

<h3 id="turn-issues-into-mrs">
  Ubah masalah menjadi MR
</h3>

Dalam komentar masalah:

```text theme={null}
@claude implement this feature based on the issue description
```

Claude menganalisis masalah dan basis kode, menulis perubahan di cabang, dan membuka MR untuk ditinjau.

<h3 id="get-implementation-help">
  Dapatkan bantuan implementasi
</h3>

Dalam diskusi MR:

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude mengusulkan perubahan, menambahkan kode dengan caching yang sesuai, dan memperbarui MR.

<h3 id="fix-bugs-quickly">
  Perbaiki bug dengan cepat
</h3>

Dalam komentar masalah atau MR:

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude menemukan bug, mengimplementasikan perbaikan, dan memperbarui cabang atau membuka MR baru.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Menggunakan dengan Amazon Bedrock dan Google Cloud
</h2>

Untuk lingkungan perusahaan, Anda dapat menjalankan Claude Code sepenuhnya di infrastruktur cloud Anda dengan pengalaman pengembang yang sama.

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

    ### Instruksi pengaturan

    Konfigurasi AWS untuk memungkinkan pekerjaan GitLab CI mengasumsikan peran IAM melalui OIDC (tanpa kunci statis).

    **Pengaturan yang diperlukan:**

    1. Aktifkan Amazon Bedrock dan minta akses ke model Claude target Anda
    2. Buat penyedia OIDC IAM untuk GitLab jika belum ada
    3. Buat peran IAM yang dipercaya oleh penyedia OIDC GitLab, dibatasi pada proyek dan ref yang dilindungi Anda
    4. Lampirkan izin hak istimewa minimal untuk API invoke Amazon Bedrock

    **Nilai yang diperlukan untuk disimpan dalam variabel CI/CD:**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    Tambahkan variabel di Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Untuk Amazon Bedrock:
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    Gunakan contoh pekerjaan Amazon Bedrock di atas untuk menukar token pekerjaan GitLab dengan kredensial AWS sementara saat runtime.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Prasyarat

    Sebelum menyiapkan Claude Code dengan Google Cloud's Agent Platform, Anda memerlukan:

    1. Proyek Google Cloud dengan:
       * Google Cloud's Agent Platform API diaktifkan
       * Workload Identity Federation dikonfigurasi untuk mempercayai GitLab OIDC
    2. Akun layanan khusus dengan hanya peran Google Cloud's Agent Platform yang diperlukan
    3. Variabel CI/CD GitLab untuk WIF:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (nama sumber daya lengkap)
       * `GCP_SERVICE_ACCOUNT` (email akun layanan)

    ### Instruksi pengaturan

    Konfigurasi Google Cloud untuk memungkinkan pekerjaan GitLab CI menyamar sebagai akun layanan melalui Workload Identity Federation.

    **Pengaturan yang diperlukan:**

    1. Aktifkan IAM Credentials API, STS API, dan Google Cloud's Agent Platform API
    2. Buat Workload Identity Pool dan penyedia untuk GitLab OIDC
    3. Buat akun layanan khusus dengan peran Google Cloud's Agent Platform
    4. Berikan izin principal WIF untuk menyamar sebagai akun layanan

    **Nilai yang diperlukan untuk disimpan dalam variabel CI/CD:**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    Tambahkan variabel di Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Untuk Google Cloud's Agent Platform:
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (misalnya, us-east5)
    ```

    Gunakan contoh pekerjaan Google Cloud's Agent Platform di atas untuk autentikasi tanpa menyimpan kunci.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Contoh konfigurasi
</h2>

Di bawah ini adalah cuplikan siap pakai yang dapat Anda sesuaikan dengan pipeline Anda.

<h3 id="basic-gitlab-ci-yml-claude-api">
  .gitlab-ci.yml dasar (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code akan menggunakan ANTHROPIC_API_KEY dari variabel CI/CD
```

<h3 id="amazon-bedrock-job-example-oidc">
  Contoh pekerjaan Amazon Bedrock (OIDC)
</h3>

**Prasyarat:**

* Amazon Bedrock diaktifkan dengan akses ke model Claude pilihan Anda
* GitLab OIDC dikonfigurasi di AWS dengan peran yang mempercayai proyek dan ref GitLab Anda
* Peran IAM dengan izin Amazon Bedrock (hak istimewa minimal direkomendasikan)

**Variabel CI/CD yang diperlukan:**

* `AWS_ROLE_TO_ASSUME`: ARN peran IAM untuk akses Amazon Bedrock
* `AWS_REGION`: Wilayah Amazon Bedrock (misalnya, `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # Tukar token OIDC GitLab dengan kredensial AWS
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
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
```

<Note>
  ID model untuk Amazon Bedrock mencakup awalan khusus wilayah (misalnya, `us.anthropic.claude-sonnet-4-6`). Teruskan model yang diinginkan melalui konfigurasi pekerjaan atau prompt Anda jika alur kerja Anda mendukungnya.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Contoh pekerjaan Agent Platform (Workload Identity Federation)
</h3>

**Prasyarat:**

* Google Cloud's Agent Platform API diaktifkan di proyek GCP Anda
* Workload Identity Federation dikonfigurasi untuk mempercayai GitLab OIDC
* Akun layanan dengan izin Google Cloud's Agent Platform

**Variabel CI/CD yang diperlukan:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Nama sumber daya penyedia lengkap
* `GCP_SERVICE_ACCOUNT`: Email akun layanan
* `CLOUD_ML_REGION`: Wilayah Google Cloud's Agent Platform (misalnya, `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # Autentikasi ke Google Cloud melalui WIF (tanpa kunci yang diunduh)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
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
```

<Note>
  Dengan Workload Identity Federation, Anda tidak perlu menyimpan kunci akun layanan. Gunakan kondisi kepercayaan khusus repositori dan akun layanan dengan hak istimewa minimal.
</Note>

<h2 id="best-practices">
  Praktik terbaik
</h2>

<h3 id="claude-md-configuration">
  Konfigurasi CLAUDE.md
</h3>

Buat file `CLAUDE.md` di akar repositori untuk menentukan standar pengkodean, kriteria ulasan, dan aturan khusus proyek. Claude membaca file ini selama berjalan dan mengikuti konvensi Anda saat mengusulkan perubahan.

<h3 id="security-considerations">
  Pertimbangan keamanan
</h3>

**Jangan pernah melakukan commit kunci API atau kredensial cloud ke repositori Anda**. Selalu gunakan variabel GitLab CI/CD:

* Tambahkan `ANTHROPIC_API_KEY` sebagai variabel yang disembunyikan (dan lindungi jika diperlukan)
* Gunakan OIDC khusus penyedia jika memungkinkan (tanpa kunci jangka panjang)
* Batasi izin pekerjaan dan egress jaringan
* Tinjau MR Claude seperti kontributor lainnya

<h3 id="optimizing-performance">
  Mengoptimalkan kinerja
</h3>

* Jaga `CLAUDE.md` tetap fokus dan ringkas
* Berikan deskripsi masalah/MR yang jelas untuk mengurangi iterasi
* Konfigurasi timeout pekerjaan yang masuk akal untuk menghindari lari liar
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
  * Lihat [harga Anthropic](https://platform.claude.com/docs/id/about-claude/pricing) untuk detail

* **Tips optimisasi biaya**:
  * Gunakan perintah `@claude` spesifik untuk mengurangi putaran yang tidak perlu
  * Tetapkan nilai `max_turns` dan timeout pekerjaan yang sesuai
  * Batasi keselarasan untuk mengontrol lari paralel

<h2 id="security-and-governance">
  Keamanan dan tata kelola
</h2>

* Setiap pekerjaan berjalan dalam kontainer terisolasi dengan akses jaringan terbatas
* Perubahan Claude mengalir melalui MR sehingga pengulas melihat setiap diff
* Perlindungan cabang dan aturan persetujuan berlaku untuk kode yang dihasilkan AI
* Claude Code menggunakan izin berskop ruang kerja untuk membatasi penulisan
* Biaya tetap di bawah kontrol Anda karena Anda membawa kredensial penyedia Anda sendiri

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude tidak merespons perintah @claude
</h3>

* Verifikasi pipeline Anda dipicu (secara manual, peristiwa MR, atau melalui pendengar peristiwa catatan/webhook)
* Pastikan variabel CI/CD (`ANTHROPIC_API_KEY` atau pengaturan penyedia cloud) ada dan tidak disembunyikan
* Periksa bahwa komentar berisi `@claude` (bukan `/claude`) dan pemicu penyebutan Anda dikonfigurasi

<h3 id="job-can’t-write-comments-or-open-mrs">
  Pekerjaan tidak dapat menulis komentar atau membuka MR
</h3>

* Pastikan `CI_JOB_TOKEN` memiliki izin yang cukup untuk proyek, atau gunakan Project Access Token dengan cakupan `api`
* Periksa alat `mcp__gitlab` diaktifkan dalam `--allowedTools`
* Konfirmasi pekerjaan berjalan dalam konteks MR atau memiliki konteks yang cukup melalui variabel `AI_FLOW_*`

<h3 id="authentication-errors">
  Kesalahan autentikasi
</h3>

* **Untuk Claude API**: Konfirmasi `ANTHROPIC_API_KEY` valid dan tidak kedaluwarsa
* **Untuk Amazon Bedrock atau Platform Agent Google Cloud**: Verifikasi konfigurasi OIDC/WIF, impersonasi peran, dan nama rahasia; konfirmasi ketersediaan wilayah dan model

<h2 id="advanced-configuration">
  Konfigurasi lanjutan
</h2>

<h3 id="common-parameters-and-variables">
  Parameter dan variabel umum
</h3>

Claude Code mendukung input yang umum digunakan ini:

* `prompt` / `prompt_file`: Berikan instruksi inline (`-p`) atau melalui file
* `max_turns`: Batasi jumlah iterasi bolak-balik
* `timeout_minutes`: Batasi waktu eksekusi total
* `ANTHROPIC_API_KEY`: Diperlukan untuk Claude API (tidak digunakan untuk Amazon Bedrock atau Platform Agen Google Cloud)
* Lingkungan khusus penyedia: `AWS_REGION`, variabel proyek/wilayah untuk Platform Agen Google Cloud

<Note>
  Bendera dan parameter yang tepat dapat bervariasi menurut versi `@anthropic-ai/claude-code`. Jalankan `claude --help` dalam pekerjaan Anda untuk melihat opsi yang didukung.
</Note>

<h3 id="customizing-claude’s-behavior">
  Menyesuaikan perilaku Claude
</h3>

Anda dapat memandu Claude dengan dua cara utama:

1. **CLAUDE.md**: Tentukan standar pengkodean, persyaratan keamanan, dan konvensi proyek. Claude membaca ini selama berjalan dan mengikuti aturan Anda.
2. **Prompt khusus**: Teruskan instruksi khusus tugas melalui `prompt`/`prompt_file` dalam pekerjaan. Gunakan prompt berbeda untuk pekerjaan berbeda (misalnya, ulasan, implementasi, refaktor).
