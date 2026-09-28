> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gunakan Claude Code GitHub Actions dengan penyedia cloud

> Jalankan Claude Code GitHub Actions melalui Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry alih-alih Claude API

[Claude Code GitHub Actions](/docs/id/github-actions) memanggil Claude API secara default. Untuk merutekan inferensi melalui akun cloud Anda sendiri, atur input penyedia Claude Code GitHub Action dan konfigurasikan cloud Anda untuk mempercayai token OpenID Connect (OIDC) alur kerja. Alur kerja melakukan autentikasi dengan token tersebut, sehingga Anda tidak menyimpan kredensial cloud jangka panjang di repositori Anda.

<Info>
  Halaman ini dibangun berdasarkan [pengaturan GitHub Actions](/docs/id/github-actions#setup). Ini mengasumsikan Anda sudah mengetahui file alur kerja dan langkah `anthropics/claude-code-action`, dan hanya mencakup apa yang diubah oleh penyedia cloud.
</Info>

<h2 id="choose-your-provider">
  Pilih penyedia Anda
</h2>

Claude Code GitHub Action mendukung tiga penyedia, dan langkah pengaturan di bawah hanya berbeda dalam konfigurasi sisi cloud. Gunakan yang sudah memiliki akses model Claude di organisasi Anda. Anda memberi tahu Claude Code GitHub Action penyedia mana yang akan digunakan dengan satu input dalam blok `with:` langkah `anthropics/claude-code-action`:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Contoh alur kerja lengkap di bawah [Atur integrasi](#set-up-the-integration) sudah menyertakan input untuk setiap penyedia.

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum Anda memulai, Anda memerlukan:

* Akses admin ke repositori tempat Claude Code GitHub Action berjalan, untuk memasang GitHub App dan menambahkan rahasia
* Izin untuk membuat sumber daya identitas di akun cloud Anda: peran IAM dan penyedia identitas OIDC di AWS, sumber daya Workload Identity Federation dan akun layanan di Google Cloud, atau aplikasi Microsoft Entra di Azure
* Akses model Claude pada penyedia Anda:
  * **Amazon Bedrock**: akses diberikan ke model Claude. Profil inferensi lintas wilayah, seperti ID model `us.` dalam contoh halaman ini, memerlukan akses yang diberikan di setiap wilayah grup wilayah mereka. Lihat [Claude Code on Amazon Bedrock](/docs/id/amazon-bedrock)
  * **Google Cloud's Agent Platform**: proyek dengan Agent Platform API diaktifkan dan akses ke model Claude. Lihat [Claude Code on Google Cloud's Agent Platform](/docs/id/google-vertex-ai)
  * **Microsoft Foundry**: sumber daya Foundry dengan penerapan model Claude. Lihat [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry)

<h2 id="set-up-the-integration">
  Atur integrasi
</h2>

Selain prasyarat, Anda membuat identitas GitHub untuk Claude Code GitHub Action, konfigurasi kepercayaan sisi cloud, rahasia repositori, dan file alur kerja. Langkah-langkah di bawah memandu melalui masing-masing.

<Steps>
  <Step title="Pilih identitas GitHub">
    Claude Code GitHub Action mendorong komit dan memposting komentar melalui identitas GitHub. [Pengaturan cepat](/docs/id/github-actions#quick-setup) memasang Claude GitHub App resmi untuk ini. Dengan penyedia cloud, Anda memilih identitas sendiri:

    * **[Claude GitHub App](https://github.com/apps/claude) resmi**: pasang di repositori, atau lewati ke langkah berikutnya jika sudah dipasang
    * **GitHub App kustom**: buat aplikasi Anda sendiri ketika Anda menginginkan hanya tiga izin yang digunakan Claude Code GitHub Action daripada [set lengkap app resmi](/docs/id/github-actions#github-app-permissions)
    * **Token `GITHUB_TOKEN` otomatis GitHub**: tidak ada aplikasi untuk dibuat atau dipasang, tetapi GitHub tidak memicu alur kerja CI Anda pada komit yang dibuat dengannya

    Contoh alur kerja di langkah keempat mengautentikasi dengan aplikasi kustom. Langkah itu juga mengatakan apa yang harus diubah untuk dua opsi lainnya.

    Untuk membuat aplikasi kustom, [daftarkan GitHub App baru](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) dengan webhook dinonaktifkan, karena integrasi ini tidak menggunakannya. Berikan tiga izin repositori:

    * **Contents**: baca dan tulis
    * **Issues**: baca dan tulis
    * **Pull requests**: baca dan tulis

    Setelah mendaftarkan aplikasi, hasilkan kunci pribadi dan simpan file `.pem` yang diunduh, catat ID Aplikasi dari halaman pengaturan aplikasi, dan [pasang aplikasi](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) di repositori tempat Claude Code GitHub Action berjalan. Anda menambahkan kunci dan ID sebagai rahasia di langkah ketiga.
  </Step>

  <Step title="Konfigurasikan autentikasi cloud">
    Konfigurasikan cloud Anda untuk mempercayai token OIDC yang dikeluarkan GitHub ke alur kerja, sehingga setiap jalankan alur kerja mendapatkan kredensial cloud jangka pendek. Poin-poin dalam setiap tab merangkum apa yang harus dibuat, dan setiap tab menautkan panduan vendor cloud sendiri untuk langkah-langkah tingkat konsol.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Buat konfigurasi kepercayaan di akun AWS Anda, mengikuti [panduan AWS untuk membuat penyedia identitas OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html):

        * Tambahkan penyedia identitas OIDC GitHub dengan URL penyedia `https://token.actions.githubusercontent.com` dan audiens `sts.amazonaws.com`
        * Buat peran IAM yang dipercaya oleh penyedia tersebut sebagai identitas web, dan lampirkan kebijakan invokasi yang dibatasi dari [konfigurasi IAM](/docs/id/amazon-bedrock#iam-configuration), yang memberikan `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles`, dan `bedrock:GetInferenceProfile`, bersama dengan dua tindakan langganan `aws-marketplace`
        * Batasi kebijakan kepercayaan peran ke repositori Anda dengan kondisi subjek seperti `repo:your-org/your-repo:*`. Lihat [panduan pengerasan OIDC GitHub](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) untuk format klaim

        Catat ARN peran. Anda menambahkannya sebagai rahasia di langkah berikutnya.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Buat sumber daya federasi di proyek Google Cloud Anda, mengikuti [dokumentasi Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation):

        * Aktifkan tiga API: IAM Credentials, Security Token Service (STS), dan Agent Platform API, yang nama layanannya adalah `aiplatform.googleapis.com`
        * Buat Workload Identity Pool dengan penyedia OIDC GitHub yang penerbitnya adalah `https://token.actions.githubusercontent.com`, dan tambahkan kondisi atribut yang membatasi pool ke repositori Anda
        * Buat akun layanan khusus dengan hanya peran `Vertex AI User`, yaitu `roles/aiplatform.user`, dan izinkan pool untuk menyamakannya

        Catat nama sumber daya lengkap penyedia dan alamat email akun layanan. Anda menambahkannya sebagai rahasia di langkah berikutnya.
      </Tab>

      <Tab title="Microsoft Foundry">
        Buat aplikasi Microsoft Entra dengan kredensial terfederasi untuk repositori Anda, mengikuti [panduan Microsoft untuk mengautentikasi dari GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect):

        * Daftarkan aplikasi Microsoft Entra dan tambahkan kredensial identitas terfederasi yang mempercayai token yang dikeluarkan GitHub ke repositori Anda. Identitas terkelola yang ditetapkan pengguna berfungsi sebagai pengganti aplikasi. Keduanya memiliki ID klien yang Anda catat di bawah
        * Tetapkan aplikasi peran `Azure AI User` pada sumber daya Foundry Anda. Lihat [konfigurasi Azure RBAC](/docs/id/microsoft-foundry#azure-rbac-configuration) untuk peran kustom yang lebih sempit

        Catat ID klien aplikasi, ID penyewa Anda, dan ID langganan Anda. Anda menambahkannya sebagai rahasia di langkah berikutnya.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Tambahkan rahasia repositori">
    Di repositori tempat Claude Code GitHub Action berjalan, tambahkan rahasia untuk penyedia Anda, ditambah dua rahasia aplikasi jika Anda membuat GitHub App kustom di langkah pertama. Lihat panduan GitHub untuk [menggunakan rahasia di GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Rahasia                          | Diperlukan untuk              | Nilai                             |
    | -------------------------------- | ----------------------------- | --------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                | ARN peran IAM                     |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform | Nama sumber daya lengkap penyedia |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform | Alamat email akun layanan         |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry             | ID klien aplikasi Entra           |
    | `AZURE_TENANT_ID`                | Microsoft Foundry             | ID penyewa Microsoft Entra Anda   |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry             | ID langganan Azure Anda           |
    | `APP_ID`                         | GitHub App Kustom             | ID GitHub App                     |
    | `APP_PRIVATE_KEY`                | GitHub App Kustom             | Isi file kunci pribadi `.pem`     |
  </Step>

  <Step title="Buat file alur kerja">
    Buat file alur kerja untuk penyedia Anda, seperti `.github/workflows/claude.yml`. Setiap contoh merespons penyebutan `@claude`, mengautentikasi ke GitHub dengan aplikasi kustom, dan menyertakan izin `id-token: write`, yang diperlukan GitHub untuk mengeluarkan token OIDC yang ditukar penyedia cloud Anda dengan kredensial.

    Jika Anda memilih identitas GitHub yang berbeda di langkah pertama, sesuaikan contohnya:

    * **Claude GitHub App resmi**: hapus langkah Generate GitHub App token dan baris `github_token`
    * **Token otomatis GitHub**: hapus langkah pembuatan token dan ubah baris `github_token` menjadi `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      Di repositori publik, komentar yang berisi frasa pemicu dari pengguna mana pun memulai alur kerja ini. Langkah kredensial berjalan sebelum Claude Code GitHub Action memeriksa akses tulis komentator, sehingga tindakan menolak pengguna yang tidak sah hanya setelah alur kerja telah menghasilkan token App dan masuk ke penyedia cloud Anda, yang meninggalkan entri log audit dan mengonsumsi menit Actions. Untuk menghindari jalankan tersebut, tambahkan langkah yang memverifikasi akses tulis komentator sebelum langkah kredensial.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Ganti nilai `aws-region` dengan milik Anda sendiri. Langkah kredensial mengekspornya sebagai `AWS_REGION` untuk sisa pekerjaan.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          ID model Bedrock menyertakan awalan profil inferensi lintas wilayah seperti `us.`. Gunakan awalan untuk grup wilayah tempat Anda memberikan akses model.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Ganti nilai `CLOUD_ML_REGION` dengan milik Anda sendiri. Anda tidak perlu mengkode keras ID proyek, karena alur kerja membacanya dari output langkah `auth`.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        Ganti `your-resource-name` dengan nama sumber daya Foundry Anda. Claude Code membangun URL titik akhir darinya. Langkah `azure/login` masuk dengan token OIDC alur kerja, dan Claude Code mengambil kredensial melalui [rantai kredensial default](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) Azure.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Gunakan ID model yang cocok dengan penerapan Claude di sumber daya Foundry Anda. Lihat [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry) untuk konfigurasi model dan penentuan versi.
        </Tip>
      </Tab>
    </Tabs>

    Dengan penyedia apa pun, Anda dapat membatasi durasi jalankan dan biaya dengan menambahkan `--max-turns` ke `claude_args`. Lihat [Kelola biaya](/docs/id/github-actions#manage-costs).
  </Step>

  <Step title="Uji pengaturan">
    Sebutkan `@claude` dalam komentar masalah atau PR, kemudian tonton jalankan di tab Actions repositori. Claude membalas dalam komentar pada masalah atau PR yang sama.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

Jalankan yang gagal biasanya rusak di salah satu dari dua tempat:

* **Kesalahan autentikasi**: biasanya kesalahan konfigurasi OIDC. Periksa bahwa alur kerja menyertakan izin `id-token: write`, bahwa kondisi repositori konfigurasi kepercayaan cocok dengan repositori Anda dengan tepat, dan bahwa nama rahasia dalam alur kerja Anda cocok dengan yang Anda tambahkan
* **Masalah pemicu dan CI**: ini berperilaku sama seperti ketika Claude Code GitHub Action memanggil Claude API. Lihat [bagian pemecahan masalah](/docs/id/github-actions#troubleshooting) halaman utama dan [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) Claude Code GitHub Action

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Claude Code GitHub Actions](/docs/id/github-actions) untuk contoh, parameter, dan praktik terbaik
* [Claude Code on Amazon Bedrock](/docs/id/amazon-bedrock) untuk ID model Bedrock dan wilayah
* [Claude Code on Google Cloud's Agent Platform](/docs/id/google-vertex-ai) untuk ID model Agent Platform dan wilayah
* [Claude Code on Microsoft Foundry](/docs/id/microsoft-foundry) untuk konfigurasi model dan titik akhir Foundry
