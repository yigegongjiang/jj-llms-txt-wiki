> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code di Amazon Bedrock

> Pelajari tentang mengonfigurasi Claude Code melalui Amazon Bedrock, termasuk pengaturan, konfigurasi IAM, dan pemecahan masalah.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum mengonfigurasi Claude Code dengan Amazon Bedrock, pastikan Anda memiliki:

* Akun AWS dengan akses Amazon Bedrock yang diaktifkan
* Akses ke model Claude yang diinginkan (misalnya, Claude Sonnet 4.6) di Amazon Bedrock
* AWS CLI terinstal dan dikonfigurasi (opsional - hanya diperlukan jika Anda tidak memiliki mekanisme lain untuk mendapatkan kredensial)
* Izin IAM yang sesuai

Untuk masuk dengan kredensial Amazon Bedrock Anda sendiri, ikuti [Masuk dengan Amazon Bedrock](#sign-in-with-bedrock) di bawah ini. Untuk menerapkan Claude Code di seluruh tim, gunakan langkah [pengaturan manual](#set-up-manually) dan [pin versi model Anda](#4-pin-model-versions) sebelum melakukan peluncuran.

<h2 id="sign-in-with-bedrock">
  Masuk dengan Bedrock
</h2>

Jika Anda memiliki kredensial AWS dan ingin mulai menggunakan Claude Code melalui Amazon Bedrock, wizard login akan memandu Anda melaluinya. Anda menyelesaikan prasyarat sisi AWS sekali per akun; wizard menangani sisi Claude Code.

<Steps>
  <Step title="Aktifkan model Anthropic di akun AWS Anda">
    Di [konsol Amazon Bedrock](https://console.aws.amazon.com/bedrock/), buka katalog Model, pilih model Anthropic, dan kirimkan formulir kasus penggunaan. Akses diberikan segera setelah pengiriman. Lihat [Kirimkan detail kasus penggunaan](#1-submit-use-case-details) untuk AWS Organizations dan [konfigurasi IAM](#iam-configuration) untuk izin yang dibutuhkan peran Anda.
  </Step>

  <Step title="Mulai Claude Code dan pilih Amazon Bedrock">
    Jalankan `claude`. Pada prompt login, pilih **platform pihak ketiga**, kemudian **Amazon Bedrock**. Jika Anda sudah masuk dan melihat prompt chat sebagai gantinya, jalankan `/setup-bedrock` untuk membuka wizard. Sampai `CLAUDE_CODE_USE_BEDROCK=1` diatur, Claude Code [menyembunyikan perintah dari menu perintah](/docs/id/commands#how-the-command-menu-matches-what-you-type); ketikkan secara lengkap.
  </Step>

  <Step title="Ikuti prompt wizard">
    Pilih cara Anda mengautentikasi ke AWS: profil AWS yang terdeteksi dari direktori `~/.aws` Anda, kunci API Amazon Bedrock, kunci akses dan rahasia, atau kredensial yang sudah ada di lingkungan Anda. Wizard menanyakan wilayah Anda, memverifikasi model Claude mana yang dapat dijalankan akun Anda, dan memungkinkan Anda menyematkannya. Ini menyimpan hasilnya ke blok `env` dari [file pengaturan pengguna Anda](/docs/id/settings), sehingga Anda tidak perlu mengekspor variabel lingkungan sendiri.
  </Step>
</Steps>

Setelah Anda masuk, jalankan `/setup-bedrock` kapan saja untuk membuka kembali wizard dan mengubah kredensial, wilayah, atau pin model Anda. Langkah pin model dimulai dari model yang saat ini disematkan. Wizard menulis ke `~/.claude/settings.json`, atau ke `$CLAUDE_CONFIG_DIR/settings.json` ketika [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars#variables) diatur.

<h2 id="set-up-manually">
  Atur secara manual
</h2>

Untuk mengonfigurasi Amazon Bedrock melalui variabel lingkungan alih-alih wizard, misalnya dalam CI atau peluncuran perusahaan yang disertakan skrip, ikuti langkah-langkah di bawah ini.

<h3 id="1-submit-use-case-details">
  1. Kirimkan detail kasus penggunaan
</h3>

Sebelum Anda memanggil model Anthropic untuk pertama kalinya, kirimkan detail kasus penggunaan. Anda melakukan ini sekali per akun AWS.

1. Pastikan Anda memiliki izin IAM yang tepat seperti dijelaskan di bawah
2. Navigasikan ke [konsol Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Pilih model Anthropic dari **Model catalog**
4. Lengkapi formulir kasus penggunaan. Akses diberikan segera setelah pengajuan.

Jika Anda menggunakan AWS Organizations, Anda dapat mengirimkan formulir sekali dari akun manajemen menggunakan [`PutUseCaseForModelAccess` API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Panggilan ini memerlukan izin IAM `bedrock:PutUseCaseForModelAccess`. Persetujuan diperluas ke akun anak secara otomatis.

<h3 id="2-configure-aws-credentials">
  2. Konfigurasi kredensial AWS
</h3>

Claude Code menggunakan rantai kredensial SDK AWS default. Atur kredensial Anda menggunakan salah satu metode berikut:

**Opsi A: Konfigurasi AWS CLI**

```bash theme={null}
aws configure
```

**Opsi B: Variabel lingkungan (kunci akses)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Opsi C: Variabel lingkungan (profil SSO)**

Ganti `your-profile-name` dengan nama profil AWS Anda sebelum menjalankan perintah ini.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code meminta kredensial peran dari wilayah IAM Identity Center yang dinamai oleh `sso_region` profil, yang tidak perlu cocok dengan wilayah tempat Anda menjalankan Amazon Bedrock. Dalam v2.1.207, wilayah Amazon Bedrock menimpa `sso_region`, jadi profil yang instance IAM Identity Center-nya berada di wilayah berbeda gagal untuk mengautentikasi dengan kesalahan `Session token not found or invalid`.

**Opsi D: Kredensial Konsol Manajemen AWS**

```bash theme={null}
aws login
```

[Pelajari lebih lanjut](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) tentang `aws login`.

**Opsi E: Kunci API Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Kunci API Amazon Bedrock menyediakan metode autentikasi yang lebih sederhana tanpa memerlukan kredensial AWS lengkap. [Pelajari lebih lanjut tentang kunci API Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Penyimpanan kredensial dan batas waktu resolusi
</h4>

Claude Code menyelesaikan rantai penyedia kredensial default AWS sekali dan menyimpan kredensial yang diselesaikan dalam memori. Kredensial tersebut digunakan kembali hingga lima menit sebelum kedaluwarsa, atau selama satu jam ketika tidak memiliki kedaluwarsa, jadi profil yang didukung SSO meminta kredensial dari IAM Identity Center sekitar sekali per masa hidup kredensial. Kesalahan kredensial dari API menghapus cache, dan percobaan ulang menyelesaikan kredensial segar. Memerlukan Claude Code v2.1.207 atau lebih baru.

Cache mencakup setiap opsi kredensial di atas kecuali kunci API Amazon Bedrock, yang tidak menggunakan rantai penyedia. Untuk menyelesaikan rantai pada setiap permintaan, atur [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/id/env-vars).

Setiap resolusi rantai habis waktu setelah 60 detik. Jika langkah dalam rantai terhenti, misalnya pembantu `credential_process` yang menunggu input yang tidak dapat diterima, permintaan gagal dengan [`AWS default-chain credential resolve timed out`](/docs/id/errors#aws-default-chain-credential-resolve-timed-out). Jika rantai Anda menjalankan masuk interaktif yang secara sah memerlukan waktu lebih lama, seperti SSO berbasis browser dengan MFA melalui pembungkus seperti `aws-vault`, naikkan batas dalam milidetik dengan [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/id/env-vars). Sebelum v2.1.207, resolusi kredensial yang terhenti membiarkan permintaan menunggu tanpa batas.

Kecuali ketika Anda mengautentikasi dengan kunci API Amazon Bedrock, [wizard pengaturan](#sign-in-with-bedrock) menerapkan batas yang sama untuk setiap panggilan AWS yang dilakukannya saat memverifikasi kredensial Anda, dan untuk pencarian kredensial sebelum setiap pemeriksaan model. Selama verifikasi kredensial, pemeriksaan yang melampaui batas gagal dengan [`Timed out after 60s waiting for AWS`](/docs/id/errors#bedrock-setup-verification-timed-out-waiting-for-aws).

<h4 id="advanced-credential-configuration">
  Konfigurasi kredensial lanjutan
</h4>

Claude Code mendukung penyegaran kredensial otomatis untuk AWS SSO dan penyedia identitas perusahaan. Tambahkan pengaturan ini ke file pengaturan Claude Code Anda (lihat [Settings](/docs/id/settings) untuk lokasi file).

Dua pengaturan ini memiliki kondisi pemicu yang berbeda:

* **`awsAuthRefresh`**: berjalan hanya ketika Claude Code mendeteksi bahwa kredensial AWS Anda kedaluwarsa, baik secara lokal berdasarkan stempel waktu mereka atau ketika API mengembalikan kesalahan kredensial, kemudian mencoba ulang permintaan dengan kredensial yang disegarkan.
* **`awsCredentialExport`**: berjalan pada awal sesi dan pada setiap pemuatan ulang kredensial, bahkan ketika kredensial dalam rantai penyedia kredensial default AWS Anda masih berlaku. Gunakan ini ketika akun Amazon Bedrock Anda memerlukan kredensial lintas akun yang berbeda dari yang akan diselesaikan oleh rantai penyedia default.

Sebelum menjalankan perintah `awsAuthRefresh`, Claude Code membuat panggilan STS `GetCallerIdentity` untuk mengonfirmasi bahwa kredensial Anda benar-benar kedaluwarsa, dan melewati perintah ketika kredensial masih berfungsi. Claude Code mengirim pemeriksaan ini melalui [konfigurasi proxy](/docs/id/network-config#proxy-configuration) Anda, menghormati `HTTPS_PROXY` dan `NO_PROXY`. Sebelum v2.1.239, Claude Code mengirim pemeriksaan ini secara langsung dan tergantung pada startup di jaringan yang hanya memungkinkan keluar melalui proxy.

<h5 id="example-configuration">
  Contoh konfigurasi
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Pengaturan konfigurasi dijelaskan
</h5>

**`awsAuthRefresh`**: Gunakan ini untuk perintah yang memodifikasi direktori `.aws`, seperti memperbarui kredensial, cache SSO, atau file konfigurasi. Output perintah ditampilkan kepada pengguna, tetapi input interaktif tidak didukung. Ini bekerja dengan baik untuk alur SSO berbasis browser di mana CLI menampilkan URL atau kode dan Anda menyelesaikan autentikasi di browser.

**`awsCredentialExport`**: Hanya gunakan ini jika Anda tidak dapat memodifikasi `.aws` dan harus secara langsung mengembalikan kredensial. Output ditangkap secara diam-diam dan tidak ditampilkan kepada pengguna. Perintah harus menampilkan JSON dalam format ini:

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

Output datar dari `aws configure export-credentials --format process` juga diterima, dengan kunci yang sama di tingkat atas alih-alih bersarang di bawah `Credentials`.

`Expiration` bersifat opsional. Ketika perintah mengembalikan `Expiration` ISO 8601 yang valid, Claude Code menyimpan kredensial dalam cache hingga lima menit sebelum waktu itu. Tanpanya, kredensial disimpan dalam cache selama satu jam.

Ketika Anda mengonfigurasi `awsCredentialExport` tanpa `awsAuthRefresh`, Claude Code menggunakan kredensial yang diekspor secara langsung dan tidak menyelesaikan kembali rantai penyedia kredensial default AWS pada startup. Memerlukan Claude Code v2.1.206 atau lebih baru.

<h3 id="3-configure-claude-code">
  3. Konfigurasi Claude Code
</h3>

Atur variabel lingkungan berikut untuk mengaktifkan Amazon Bedrock:

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Saat mengaktifkan Amazon Bedrock untuk Claude Code, perhatikan hal berikut:

* Anda hanya perlu mengatur `AWS_REGION` untuk menimpa wilayah profil AWS Anda atau ketika profil Anda tidak memiliki wilayah. Claude Code menyelesaikan wilayah dalam urutan ini:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * `region` yang ditetapkan pada profil AWS aktif Anda, dibaca dari file kredensial bersama AWS terlebih dahulu dan kemudian file konfigurasi bersama, mencocokkan prioritas SDK AWS
  * `us-east-1`

  Jika nilai dari salah satu sumber ini tidak berbentuk seperti nama wilayah, Claude Code memperlakukannya sebagai tidak diatur dan melanjutkan ke bawah urutan. Misalnya, Claude Code memperlakukan nilai yang berisi garis miring, titik, atau spasi sebagai tidak diatur.

  Profil aktif adalah `AWS_PROFILE` jika diatur, jika tidak `default`. Atur `AWS_SHARED_CREDENTIALS_FILE` atau `AWS_CONFIG_FILE` untuk menunjuk ke jalur file non-default.

  Jalankan `/status` untuk melihat wilayah yang diselesaikan. Ketika wilayah berasal dari file konfigurasi AWS Anda atau fallback default, Claude Code juga mencatat sumber dalam output `/status`.
* Saat menggunakan Amazon Bedrock, perintah `/logout` tidak tersedia karena autentikasi ditangani melalui kredensial AWS.
* Alat WebSearch tidak tersedia di Amazon Bedrock. Lihat [perilaku alat WebSearch](/docs/id/tools-reference#websearch-tool-behavior).
* Anda dapat menggunakan file pengaturan untuk variabel lingkungan seperti `AWS_PROFILE` yang tidak ingin Anda bocorkan ke proses lain. Lihat [Settings](/docs/id/settings) untuk informasi lebih lanjut.

<h3 id="4-pin-model-versions">
  4. Pin versi model
</h3>

<Warning>
  Pin versi model spesifik saat menerapkan ke beberapa pengguna. Tanpa pinning, alias model seperti `sonnet` dan `opus` diselesaikan ke default bawaan Claude Code untuk Amazon Bedrock, yang dapat tertinggal dari rilis terbaru dan mungkin belum tersedia di akun Anda. Claude Code [kembali](#startup-model-checks) ke model yang lebih awal atau tingkat lebih rendah pada startup ketika default tidak tersedia, tetapi pinning memungkinkan Anda mengontrol kapan pengguna Anda pindah ke model baru.
</Warning>

Atur variabel lingkungan ini ke ID model Amazon Bedrock spesifik.

Tanpa `ANTHROPIC_DEFAULT_OPUS_MODEL`, alias `opus` di Amazon Bedrock diselesaikan ke Opus 5.5, dan tanpa `ANTHROPIC_DEFAULT_SONNET_MODEL`, alias `sonnet` diselesaikan ke Sonnet 4.5. Contoh ini pin setiap alias ke versi spesifik:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

ID ini menggunakan awalan profil inferensi lintas wilayah `us.`. Jika Anda menggunakan awalan wilayah berbeda atau profil inferensi aplikasi, sesuaikan sesuai kebutuhan. Di wilayah AWS GovCloud, gunakan awalan `us-gov.`.

Untuk menyimpan model default bawaan dan mengubah hanya awalan pilihan mereka, atur [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes) alih-alih pinning. Perbedaannya ditunjukkan dalam apa yang alias `opus` diselesaikan ke:

| Anda atur                                                     | Alias `opus` diselesaikan ke                                              |
| :------------------------------------------------------------ | :------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`, ID tepat yang Anda pin                    |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`, default bawaan dengan awalan pilihan Anda |

Untuk ID model saat ini dan warisan, lihat [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk daftar lengkap variabel lingkungan pinning, lihat [Model configuration](/docs/id/model-config#pin-models-for-third-party-deployments).

Claude Code menggunakan model default ini ketika tidak ada variabel pinning yang diatur:

| Jenis model       | Model default                                                                         |
| :---------------- | :------------------------------------------------------------------------------------ |
| Model utama       | Opus 5.5, misalnya `us.anthropic.claude-opus-5-5` di wilayah `us-*`                   |
| Model kecil/cepat | Sonnet 4.5, misalnya `us.anthropic.claude-sonnet-4-5-20250929-v1:0` di wilayah `us-*` |

Tugas latar belakang seperti pembuatan judul sesi menggunakan model kecil/cepat, biasanya model kelas Haiku. Di Amazon Bedrock, Claude Code menggunakan model Sonnet default untuk tugas latar belakang karena Haiku mungkin tidak diaktifkan di setiap akun atau wilayah. Dua pilihan mengubah model mana yang membawanya:

* Ketika Anda memilih model utama dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`, tugas latar belakang menggunakan model itu. Ketika Claude Code memulai sesi pada model yang Anda atur dengan [`ANTHROPIC_DEFAULT_MODEL`](/docs/id/model-config#set-a-default-model-for-new-sessions), tugas latar belakang juga menggunakan model itu. Pengaturan `ANTHROPIC_DEFAULT_OPUS_MODEL` tanpa `ANTHROPIC_DEFAULT_SONNET_MODEL` juga dihitung sebagai pilihan, karena model Sonnet bawaan mungkin tidak diaktifkan di akun yang mengarahkan Opus-nya sendiri.
* Untuk menggunakan Haiku untuk tugas latar belakang, atur `ANTHROPIC_DEFAULT_HAIKU_MODEL` ke ID model yang tersedia di akun Anda.

<Warning>
  Model Opus memiliki harga per-token yang lebih tinggi daripada model Sonnet, jadi penerapan yang tidak pin model utama ditagih dengan tarif Opus setelah diperbarui ke v2.1.207 atau lebih baru. Untuk menyimpan Sonnet 4.5 sebagai model utama, atur `ANTHROPIC_MODEL` ke ID model lengkapnya. Penerapan yang mengarahkan default dengan `ANTHROPIC_DEFAULT_SONNET_MODEL` dan tidak mengatur `ANTHROPIC_DEFAULT_OPUS_MODEL` menyimpan model Sonnet yang diarahkan sebagai default.
</Warning>

Sebelum v2.1.280, model utama di Amazon Bedrock default ke Opus 5 dan alias `opus` diselesaikan ke Opus 5 dari v2.1.219. Pada v2.1.207 melalui v2.1.218, model utama di Amazon Bedrock default ke Opus 4.8 dan alias `opus` diselesaikan ke Opus 4.8. Sebelum v2.1.207, model utama default ke Sonnet 4.5, alias `opus` diselesaikan ke Opus 4.6, dan tugas latar belakang selalu menggunakan model utama.

Untuk menyesuaikan model lebih lanjut, gunakan salah satu metode ini:

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
# export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
# export ENABLE_PROMPT_CACHING_1H=1
```

TTL cache 1 jam ditagih dengan tarif lebih tinggi daripada default 5 menit. Lihat [cache lifetime](/docs/id/prompt-caching#cache-lifetime). Untuk menetapkan TTL berbeda untuk percakapan utama Anda dan untuk permintaan yang Claude Code buat di luar itu, [pilih TTL sendiri](/docs/id/prompt-caching#choose-the-ttl-yourself).

<Note>Prompt caching mungkin tidak tersedia di semua wilayah Amazon Bedrock. Jika hitungan token cache tetap di nol, periksa [model, wilayah, dan batas yang didukung](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dalam dokumentasi Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Petakan setiap versi model ke profil inferensi
</h4>

Variabel lingkungan `ANTHROPIC_DEFAULT_*_MODEL` mengonfigurasi satu profil inferensi per keluarga model. Jika organisasi Anda perlu mengekspos beberapa versi keluarga yang sama di pemilih `/model`, masing-masing dirutekan ke ARN profil inferensi aplikasi sendiri, gunakan pengaturan `modelOverrides` dalam [file pengaturan](/docs/id/settings#where-settings-live) Anda.

Contoh ini memetakan empat versi Opus ke ARN yang berbeda sehingga pengguna dapat beralih di antara mereka tanpa melewati profil inferensi organisasi Anda:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Ketika pengguna memilih salah satu versi ini di `/model`, Claude Code memanggil Amazon Bedrock dengan ARN yang dipetakan. Pemetaan yang sama berlaku ketika Anda meneruskan ID model Anthropic secara langsung melalui `--model` atau `ANTHROPIC_MODEL`. Versi tanpa override kembali ke ID model Amazon Bedrock bawaan atau profil inferensi yang cocok yang ditemukan pada startup. Sebelum v2.1.200, nilai `--model` dan `ANTHROPIC_MODEL` mencapai Amazon Bedrock apa adanya tanpa melewati peta override. Lihat [Override model IDs per version](/docs/id/model-config#override-model-ids-per-version) untuk detail tentang bagaimana override berinteraksi dengan `availableModels` dan pengaturan model lainnya.

<h2 id="startup-model-checks">
  Pemeriksaan model startup
</h2>

Ketika Claude Code dimulai dengan Amazon Bedrock yang dikonfigurasi, Claude Code memverifikasi bahwa model yang dimaksudkan untuk digunakan dapat diakses di akun Anda.

Jika Anda telah menyematkan versi model yang lebih lama dari default Claude Code saat ini, dan akun Anda dapat memanggil versi yang lebih baru, Claude Code akan meminta Anda untuk memperbarui pin. Menerima akan menulis ID model baru ke [file pengaturan pengguna Anda](/docs/id/settings) dan memulai ulang Claude Code. Menolak akan diingat hingga perubahan versi default berikutnya. Pin yang menunjuk ke [profil inferensi aplikasi ARN](#map-each-model-version-to-an-inference-profile) dilewati, karena dikelola oleh administrator Anda.

Jika Anda belum menyematkan model dan default saat ini tidak tersedia di akun Anda, Claude Code akan kembali untuk sesi saat ini dan menampilkan pemberitahuan. Claude Code mencoba versi sebelumnya dari model default terlebih dahulu dan, ketika default adalah model Opus dan tidak ada versi Opus yang tersedia, kembali ke model Sonnet default. Fallback tidak disimpan. Aktifkan model yang lebih baru di akun Amazon Bedrock Anda atau [sematkan versi](#4-pin-model-versions) untuk membuat pilihan permanen.

Ketika Anda memulai sesi pada versi Sonnet atau Opus tertentu, misalnya dengan `--model`, `ANTHROPIC_MODEL`, atau [pengaturan `model`](/docs/id/settings-reference#model), versi tersebut bertindak sebagai default yang disematkan sesi untuk alias `sonnet` atau `opus` yang cocok. Claude Code melewati pemeriksaan ketersediaan untuk default bawaan yang diganti model Anda dan dimulai pada model yang Anda konfigurasi, tanpa pemberitahuan fallback.

Alias model seperti `opus` tidak bertindak sebagai pin, begitu juga ID model yang tidak dikenali Claude Code, seperti profil inferensi aplikasi ARN.

<h2 id="cross-region-inference-profile-prefixes">
  Prefiks profil inferensi lintas wilayah
</h2>

Pada Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html), Claude Code menyelesaikan model default bawaan ke ID [profil inferensi lintas wilayah](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html); untuk merutekan versi model melalui profil inferensi Anda sendiri, lihat [Petakan setiap versi model ke profil inferensi](#map-each-model-version-to-an-inference-profile). Tabel ini menunjukkan prefiks yang disukai Claude Code untuk setiap wilayah AWS yang diselesaikan:

| Wilayah AWS               | Prefiks   |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| Semua wilayah lainnya     | `global.` |

Atur `ANTHROPIC_BEDROCK_REGION_PREFIX` untuk memilih prefiks yang dicoba Claude Code terlebih dahulu; ketika Claude Code dapat memeriksa ketersediaan profil dan menemukan tidak ada profil yang cocok untuk model, Claude Code kembali seperti yang dijelaskan dalam urutan resolusi di bawah. Nilai yang valid adalah `us`, `eu`, `apac`, `jp`, `au`, dan `global`. Misalnya, atur ke `global` ketika akun Anda memiliki profil `global.` diaktifkan tetapi Claude Code akan menurunkan satu yang spesifik geografis dari wilayah AWS Anda. Memerlukan Claude Code v2.1.224 atau lebih baru.

Contoh ini merutekan model default melalui profil `global.`:

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# Di wilayah us-*, model utama sekarang diselesaikan ke
# global.anthropic.claude-opus-5-5 bukan us.anthropic.claude-opus-5-5
```

Prefiks yang disukai adalah preferensi, bukan jaminan, baik itu berasal dari wilayah Anda atau dari variabel. Bagaimana Claude Code menerapkannya tergantung pada apakah Claude Code dapat memeriksa ketersediaan profil di akun Anda:

* Ketika Claude Code dapat [mencantumkan profil inferensi](#iam-configuration) di akun Anda, Claude Code menyelesaikan setiap model dalam urutan ini:
  1. Profil dengan prefiks pilihan Anda.
  2. Profil yang cocok, untuk model yang tidak memiliki profil dengan prefiks tersebut.
  3. ID model bawaan dengan prefiks pilihan Anda, untuk model yang tidak memiliki profil yang cocok sama sekali. Claude Code menerapkan ID ini tanpa memeriksa ketersediaan pada langkah ini; [pemeriksaan model startup](#startup-model-checks) masih mencakup model default sesi.
* Ketika penemuan profil tidak tersedia, Claude Code menerapkan prefiks tanpa memeriksa ketersediaan. Jika akun Anda tidak memiliki profil inferensi dengan prefiks tersebut diaktifkan, permintaan gagal dengan kesalahan 400.

Claude Code tidak menulis ulang ID profil inferensi Amazon Bedrock atau ARN yang Anda konfigurasi sendiri, atau nilai [`modelOverrides`](#map-each-model-version-to-an-inference-profile); ID model format Anthropic diselesaikan melalui [pemetaan yang sama seperti pemilih `/model`](#map-each-model-version-to-an-inference-profile). Claude Code juga mengabaikan variabel dalam dua kasus:

* Di wilayah AWS GovCloud, Claude Code selalu menggunakan `us-gov.`, satu-satunya prefiks yang merutekan dalam partisi GovCloud.
* Ketika Anda menetapkan nilai yang bukan salah satu dari nilai yang valid, Claude Code kembali ke prefiks pilihan yang diturunkan dari wilayah.

<h2 id="iam-configuration">
  Konfigurasi IAM
</h2>

Buat kebijakan IAM dengan izin yang diperlukan untuk Claude Code:

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Untuk izin yang lebih ketat, Anda dapat membatasi Resource ke ARN profil inferensi spesifik.

`bedrock:GetInferenceProfile` memungkinkan Claude Code menyelesaikan [ARN profil inferensi aplikasi](#map-each-model-version-to-an-inference-profile) ke model fondasi pendukungnya, yang digunakan untuk memilih bentuk permintaan yang benar untuk model tersebut.

Jika token tidak memiliki izin ini, Claude Code pulih secara otomatis dengan mencoba ulang sekali dengan bentuk alternatif, sehingga permintaan tetap berhasil tetapi setiap model baru menambahkan perjalanan bolak-balik ekstra. Memberikan izin menghindari percobaan ulang. Ini paling sering berlaku untuk penyebaran `AWS_BEARER_TOKEN_BEDROCK`, di mana kebijakan token biasanya lebih sempit daripada peran IAM penuh.

Untuk detail, lihat [dokumentasi IAM Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Buat akun AWS khusus untuk Claude Code untuk menyederhanakan pelacakan biaya dan kontrol akses.
</Note>

<h2 id="1m-token-context-window">
  Jendela konteks token 1M
</h2>

Claude Sonnet 5, Opus 4.6 dan yang lebih baru, serta Sonnet 4.6 mendukung [jendela konteks token 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) di Amazon Bedrock. Sonnet 5 selalu berjalan dengan jendela 1M di kedua API Invoke dan [endpoint Mantle](#use-the-mantle-endpoint), tanpa varian `[1m]` untuk dipilih. Untuk model lainnya di API Invoke, Claude Code secara otomatis mengaktifkan jendela konteks yang diperluas ketika Anda memilih varian model 1M.

[Wizard pengaturan](#sign-in-with-bedrock) menawarkan opsi konteks 1M ketika mempin model. Untuk mengaktifkannya untuk model yang dipinnya secara manual, tambahkan `[1m]` ke ID model. Lihat [Pin models for third-party deployments](/docs/id/model-config#pin-models-for-third-party-deployments) untuk detail.

<h2 id="service-tiers">
  Tingkat layanan
</h2>

[Tingkat layanan Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) memungkinkan Anda menukar biaya terhadap latensi. Atur `ANTHROPIC_BEDROCK_SERVICE_TIER` ke `default`, `flex`, atau `priority`:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code mengirimkan ini sebagai header `X-Amzn-Bedrock-Service-Tier` pada setiap permintaan. Ketersediaan tingkat bervariasi menurut model dan wilayah. Kapasitas yang dicadangkan menggunakan [throughput yang disediakan](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) ARN sebagai ID model alih-alih pengaturan ini.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) memungkinkan Anda menerapkan penyaringan konten untuk Claude Code. Buat Guardrail di [konsol Amazon Bedrock](https://console.aws.amazon.com/bedrock/), publikasikan versi, kemudian tambahkan header Guardrail ke [file pengaturan](/docs/id/settings) Anda. Aktifkan inferensi Cross-Region pada Guardrail Anda jika Anda menggunakan profil inferensi lintas wilayah.

Contoh konfigurasi:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

Jika organisasi Anda mengirimkan header guardrail melalui kebijakan [gateway aplikasi Claude](/docs/id/claude-apps-gateway) sebagai gantinya, mereka dihitung sebagai [pengaturan yang memerlukan persetujuan](/docs/id/server-managed-settings#environment-variables-and-the-approval-dialog).

<h2 id="use-the-mantle-endpoint">
  Gunakan endpoint Mantle
</h2>

Mantle adalah endpoint Amazon Bedrock yang melayani model Claude melalui bentuk API Anthropic asli daripada Amazon Bedrock Invoke API. Ini menggunakan kredensial AWS yang sama [](#2-configure-aws-credentials) dan [konfigurasi `awsAuthRefresh`](#advanced-credential-configuration).

Mantle memiliki tindakan IAM-nya sendiri di bawah awalan `bedrock-mantle:`, jadi tindakan `bedrock:` dalam [konfigurasi IAM](#iam-configuration) tidak mencakupnya. Berikan identitas IAM Anda `bedrock-mantle:CreateInference` untuk inferensi dan `bedrock-mantle:CountTokens` untuk penghitungan token. Lihat [Making inference requests](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) dan [Counting tokens](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html) dalam dokumentasi AWS, dan [service authorization reference](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) untuk setiap tindakan Mantle.

<h3 id="enable-mantle">
  Aktifkan Mantle
</h3>

Dengan kredensial AWS sudah dikonfigurasi, atur `CLAUDE_CODE_USE_MANTLE` untuk merutekan permintaan ke endpoint Mantle:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code membuat URL endpoint dari wilayah AWS, diselesaikan dengan prioritas yang sama seperti [Amazon Bedrock di atas](#3-configure-claude-code). Untuk mengganti URL untuk endpoint khusus atau gateway, atur `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Jalankan `/status` di dalam Claude Code untuk mengonfirmasi. Baris penyedia menunjukkan `Amazon Bedrock (Mantle)` ketika Mantle aktif.

<h3 id="select-a-mantle-model">
  Pilih model Mantle
</h3>

Mantle menggunakan ID model dengan awalan `anthropic.` dan tanpa akhiran versi, misalnya `anthropic.claude-sonnet-5` atau `anthropic.claude-haiku-4-5`. Model yang tersedia untuk akun Anda tergantung pada apa yang telah diberikan organisasi Anda; ID model tambahan tercantum dalam materi onboarding Anda dari AWS. Hubungi tim akun AWS Anda untuk meminta akses ke model yang diizinkan.

Atur model dengan flag `--model` atau dengan `/model` di dalam Claude Code:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Jalankan Mantle bersama Invoke API
</h3>

Model yang tersedia untuk Anda di Mantle mungkin tidak mencakup setiap model yang Anda gunakan hari ini. Menetapkan `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_MANTLE` memungkinkan Claude Code memanggil kedua endpoint dari sesi yang sama. ID model yang cocok dengan format Mantle dirutekan ke Mantle, dan semua ID model lainnya pergi ke Amazon Bedrock Invoke API.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Untuk menampilkan model Mantle di pemilih `/model`, daftarkan ID-nya di `availableModels` di [file pengaturan](/docs/id/settings) Anda. Pengaturan ini juga membatasi pemilih ke entri yang terdaftar. Mendaftarkan `anthropic.claude-haiku-4-5` menghapus alias `haiku` biasa dari pemilih, jadi juga daftarkan awalan versi atau ID lengkap untuk versi yang ingin Anda tetap dapat dipilih. ID Mantle dan alias `haiku` diselesaikan ke keluarga model yang sama, jadi penggabungan hanya menyimpan entri yang lebih spesifik. Lihat [Merge behavior](/docs/id/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Entri dengan awalan `anthropic.` ditambahkan sebagai opsi pemilih khusus dan dirutekan ke Mantle. Ganti `anthropic.claude-haiku-4-5` dengan ID model yang telah diberikan akun Anda. Lihat [Restrict model selection](/docs/id/model-config#restrict-model-selection) untuk cara `availableModels` berinteraksi dengan pengaturan model lainnya.

Ketika kedua penyedia aktif, `/status` menunjukkan `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Rutekan Mantle melalui gateway
</h3>

Jika organisasi Anda merutekan lalu lintas model melalui [LLM gateway](/docs/id/llm-gateway) terpusat yang menyuntikkan kredensial AWS sisi server, nonaktifkan autentikasi sisi klien sehingga Claude Code mengirim permintaan tanpa tanda tangan SigV4 atau header `x-api-key`:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Variabel lingkungan Mantle
</h3>

Variabel ini khusus untuk endpoint Mantle. Lihat [Environment variables](/docs/id/env-vars) untuk daftar lengkap.

| Variabel                                | Tujuan                                                                      |
| :-------------------------------------- | :-------------------------------------------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | Aktifkan endpoint Mantle. Atur ke `1` atau `true`.                          |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Ganti URL endpoint Mantle default                                           |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Lewati autentikasi sisi klien untuk pengaturan proxy                        |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Ganti wilayah AWS untuk model kelas Haiku (dibagikan dengan Amazon Bedrock) |

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Loop autentikasi dengan SSO dan proxy perusahaan
</h3>

Jika tab browser muncul berulang kali saat menggunakan AWS SSO, hapus pengaturan `awsAuthRefresh` dari [file pengaturan](/docs/id/settings) Anda. Ini dapat terjadi ketika VPN perusahaan atau proxy inspeksi TLS mengganggu alur browser SSO. Claude Code memperlakukan koneksi yang terputus sebagai kegagalan autentikasi, menjalankan kembali `awsAuthRefresh`, dan loop tanpa batas.

Jika lingkungan jaringan Anda mengganggu alur SSO berbasis browser otomatis, gunakan `aws sso login` secara manual sebelum memulai Claude Code alih-alih mengandalkan `awsAuthRefresh`.

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  Kesalahan sertifikat di belakang proxy yang melakukan inspeksi TLS
</h3>

Claude Code menerapkan konfigurasi [penyimpanan sertifikat CA](/docs/id/network-config#ca-certificate-store) Anda ke permintaannya ke AWS, termasuk:

* Penemuan model
* Penghitungan token
* Panggilan kredensial peran STS dan SSO yang menyelesaikan kredensial AWS Anda
* Verifikasi kredensial dan pemeriksaan model [wizard penyiapan](#sign-in-with-bedrock)

Untuk permintaan ini, sertifikat akar perusahaan di penyimpanan kepercayaan OS Anda atau bundel `NODE_EXTRA_CA_CERTS` tidak memerlukan penyiapan khusus Amazon Bedrock.

Sebelum v2.1.260, Claude Code menerapkan konfigurasi CA Anda ke permintaan ini hanya ketika mereka melewati proxy yang dikonfigurasi, dan pada koneksi langsung mereka hanya mempercayai penyimpanan sertifikat default runtime.

Sebelum v2.1.261, pencarian kredensial di belakang pemeriksaan model wizard penyiapan dengan opsi **Use credentials already in my environment** masih hanya mempercayai penyimpanan sertifikat default runtime. Di belakang proxy yang melakukan inspeksi TLS yang sertifikat akarnya hanya ada di penyimpanan OS, permintaan yang terpengaruh gagal dengan `unable to get local issuer certificate`, atau wizard menunjukkan model sebagai `unreachable`, sementara permintaan inferensi berhasil. Perbarui ke v2.1.261 atau lebih baru.

<h3 id="region-issues">
  Masalah wilayah
</h3>

Jika Anda mengalami masalah wilayah:

* Periksa ketersediaan model: `aws bedrock list-inference-profiles --region your-region`
* Beralih ke wilayah yang didukung: `export AWS_REGION=us-east-1`
* Pertimbangkan menggunakan profil inferensi untuk akses lintas wilayah

Jika Anda menerima kesalahan "on-demand throughput isn't supported":

* Tentukan model sebagai ID [profil inferensi](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code menggunakan Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) dan tidak mendukung Converse API.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Kesalahan streaming di belakang gateway atau proxy
</h3>

Amazon Bedrock melakukan streaming respons `InvokeModelWithResponseStream` dalam format event-stream biner dengan header `Content-Type: application/vnd.amazon.eventstream`. Gateway atau proxy antara Claude Code dan Amazon Bedrock harus meneruskan badan respons dan headernya, termasuk `Content-Type`, seperti yang dikirim Amazon Bedrock.

Jika gateway menulis ulang `Content-Type` ke nilai lain, Claude Code menolak respons dengan kesalahan yang dimulai dengan `Bedrock streaming response has content-type`, menyebutkan nilai yang diterima. Penulisan ulang umum adalah `text/event-stream`, dari integrasi yang memancarkan kembali aliran sebagai server-sent events.

Jika gateway menghapus atau mengosongkan header sebagai gantinya, Claude Code mengasumsikan badan adalah event stream Amazon Bedrock dan mendekodenya, jadi badan yang gateway teruskan tanpa diubah terus melakukan streaming.

Jika gateway yang menghapus header juga memancarkan kembali aliran sebagai server-sent events, Claude Code tidak dapat mendekode badan dan kembali ke jalur non-streaming yang lebih lambat pada setiap giliran: setiap respons muncul hanya setelah selesai alih-alih melakukan streaming. Dalam hal itu, atur [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/id/env-vars) sehingga Claude Code membaca badan sebagai server-sent events sebagai gantinya.

Untuk memperbaiki kesalahan atau fallback, konfigurasikan gateway untuk meneruskan badan respons `InvokeModelWithResponseStream` dan header `Content-Type`-nya tanpa diubah.

Gateway yang mengonversi aliran ke server-sent events tidak lagi melayani Amazon Bedrock API. Jika juga menerima permintaan Anthropic Messages API, hubungkan sebagai [gateway LLM](/docs/id/llm-gateway-connect) dengan `ANTHROPIC_BASE_URL` alih-alih `CLAUDE_CODE_USE_BEDROCK`.

<h3 id="zero-token-counts-in-/context">
  Penghitungan token nol dalam /context
</h3>

Perintah `/context` menghitung token untuk setiap grup alat dengan mengirimkan skema alat ke API count-tokens Amazon Bedrock. Pada versi Claude Code sebelum v2.1.196, Amazon Bedrock menolak permintaan itu karena skema membawa bidang yang tidak diterima API count-tokens-nya, jadi setiap grup alat menunjukkan 0 token. Baris lain dalam rincian, seperti pesan dan file memori, tidak terpengaruh.

Perbarui ke v2.1.196 atau lebih baru.

<h3 id="mantle-endpoint-errors">
  Kesalahan endpoint Mantle
</h3>

Jika `/status` tidak menunjukkan `Amazon Bedrock (Mantle)` setelah Anda menetapkan `CLAUDE_CODE_USE_MANTLE`, variabel tidak mencapai proses. Konfirmasi bahwa variabel diekspor di shell tempat Anda meluncurkan `claude`, atau atur di blok `env` dari [file pengaturan](/docs/id/settings) Anda.

Apa yang berarti `403` dari endpoint Mantle tergantung pada apakah kesalahan menyebutkan tindakan IAM:

* Jika kesalahan menyebutkan tindakan `bedrock-mantle:`, berikan identitas IAM Anda tindakan itu.
* Jika kesalahan tidak menyebutkan tindakan dan kredensial Anda valid, akun AWS Anda belum diberikan akses ke model yang Anda minta. Hubungi tim akun AWS Anda untuk meminta akses.

A `400` yang menyebutkan ID model berarti model itu tidak dilayani di Mantle. Mantle memiliki lineup model sendiri yang terpisah dari katalog Bedrock standar, jadi ID profil inferensi seperti `us.anthropic.claude-sonnet-4-6` tidak akan berfungsi. Gunakan ID format Mantle, atau aktifkan [kedua endpoint](#run-mantle-alongside-the-invoke-api) sehingga Claude Code merutekan setiap permintaan ke endpoint tempat model tersedia.

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

* [Dokumentasi Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Harga Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Profil inferensi Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Burndown token Amazon Bedrock dan kuota](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code di Amazon Bedrock: Panduan Pengaturan Cepat](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Implementasi Pemantauan Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
