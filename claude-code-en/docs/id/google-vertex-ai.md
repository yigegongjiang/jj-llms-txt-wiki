> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code di Platform Agen Google Cloud

> Pelajari tentang mengonfigurasi Claude Code melalui Platform Agen Google Cloud, yang sebelumnya bernama Vertex AI, termasuk pengaturan, konfigurasi IAM, dan pemecahan masalah.

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum mengonfigurasi Claude Code dengan Google Cloud's Agent Platform, yang sebelumnya dikenal sebagai Vertex AI, pastikan Anda memiliki:

* Akun Google Cloud Platform (GCP) dengan penagihan diaktifkan
* Proyek GCP dengan Google Cloud's Agent Platform API diaktifkan
* Akses ke model Claude yang diinginkan (misalnya, Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) terinstal dan dikonfigurasi
* Kuota dialokasikan di wilayah GCP yang diinginkan

Untuk masuk dengan kredensial Google Cloud's Agent Platform Anda sendiri, ikuti [Masuk dengan Google Cloud's Agent Platform](#sign-in-with-agent-platform) di bawah. Untuk menerapkan Claude Code di seluruh tim, gunakan langkah [pengaturan manual](#set-up-manually) dan [pin versi model Anda](#5-pin-model-versions) sebelum melakukan peluncuran.

<h2 id="sign-in-with-agent-platform">
  Masuk dengan Agent Platform
</h2>

Jika Anda memiliki kredensial Google Cloud dan ingin mulai menggunakan Claude Code melalui Agent Platform Google Cloud, wizard login akan memandu Anda. Anda menyelesaikan prasyarat sisi GCP sekali per proyek; wizard menangani sisi Claude Code.

<Steps>
  <Step title="Aktifkan model Claude di proyek GCP Anda">
    [Aktifkan API Agent Platform Google Cloud](#1-enable-agent-platform-api) untuk proyek Anda, kemudian minta akses ke model Claude yang Anda inginkan di [Model Garden Agent Platform Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden). Lihat [konfigurasi IAM](#iam-configuration) untuk izin yang akun Anda butuhkan.
  </Step>

  <Step title="Mulai Claude Code dan pilih Agent Platform Google Cloud">
    Jalankan `claude`. Pada prompt login, pilih **3rd-party platform**, kemudian **Google Vertex AI**, label yang masih digunakan prompt login untuk Agent Platform Google Cloud.
  </Step>

  <Step title="Ikuti prompt wizard">
    Pilih cara Anda melakukan autentikasi ke Google Cloud: Application Default Credentials dari `gcloud`, file kunci akun layanan, atau kredensial yang sudah ada di lingkungan Anda. Wizard mendeteksi proyek dan wilayah Anda, memverifikasi model Claude mana yang dapat dijalankan proyek Anda, dan memungkinkan Anda untuk mempinnya. Ini menyimpan hasilnya ke blok `env` dari [file pengaturan pengguna Anda](/docs/id/settings), jadi Anda tidak perlu mengekspor variabel lingkungan sendiri.
  </Step>
</Steps>

Setelah Anda masuk, jalankan `/setup-vertex` kapan saja untuk membuka kembali wizard dan mengubah kredensial, proyek, wilayah, atau pin model Anda. Langkah pin model dimulai dari model yang saat ini Anda pin. Wizard menulis ke `~/.claude/settings.json`, atau ke `$CLAUDE_CONFIG_DIR/settings.json` ketika [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars#variables) diatur.

<h2 id="region-configuration">
  Konfigurasi wilayah
</h2>

Claude Code mendukung Google Cloud's Agent Platform [global](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), multi-region, dan titik akhir regional. Atur `CLOUD_ML_REGION` ke `global`, lokasi multi-region seperti `eu` atau `us`, atau wilayah spesifik seperti `us-east5`. Claude Code memilih nama host Google Cloud's Agent Platform yang benar untuk setiap bentuk, termasuk host `aiplatform.eu.rep.googleapis.com` dan `aiplatform.us.rep.googleapis.com` untuk lokasi multi-region.

<Note>
  Google Cloud's Agent Platform mungkin tidak mendukung model default Claude Code di setiap jenis titik akhir. Ketersediaan model bervariasi di [wilayah spesifik](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), lokasi multi-region, dan [titik akhir global](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Anda mungkin perlu beralih ke lokasi yang didukung atau menentukan model yang didukung.
</Note>

<h2 id="set-up-manually">
  Pengaturan manual
</h2>

Untuk mengonfigurasi Google Cloud's Agent Platform melalui variabel lingkungan alih-alih wizard, misalnya di CI atau peluncuran perusahaan yang ditulis skrip, ikuti langkah-langkah di bawah.

<h3 id="1-enable-agent-platform-api">
  1. Aktifkan Agent Platform API
</h3>

Aktifkan Google Cloud's Agent Platform API di proyek GCP Anda:

```bash theme={null}
# Atur ID proyek Anda
gcloud config set project YOUR-PROJECT-ID

# Aktifkan Agent Platform API
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Minta akses model
</h3>

Minta akses ke model Claude di Google Cloud's Agent Platform:

1. Navigasikan ke [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Cari model "Claude"
3. Minta akses ke model Claude yang diinginkan (misalnya, Claude Sonnet 4.6)
4. Tunggu persetujuan (mungkin memakan waktu 24-48 jam)

<h3 id="3-configure-gcp-credentials">
  3) Konfigurasi kredensial GCP
</h3>

Claude Code menggunakan autentikasi Google Cloud standar.

Untuk informasi lebih lanjut, lihat [dokumentasi autentikasi Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code v2.1.121 atau lebih baru mendukung [Workload Identity Federation berbasis sertifikat X.509](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) melalui rantai Application Default Credentials yang sama. Atur `GOOGLE_APPLICATION_CREDENTIALS` ke jalur file konfigurasi kredensial Anda.

<Note>
  Claude Code menggunakan `ANTHROPIC_VERTEX_PROJECT_ID` sebagai ID proyek untuk permintaan Google Cloud's Agent Platform. Variabel lingkungan `GCLOUD_PROJECT` dan `GOOGLE_CLOUD_PROJECT` serta file kredensial yang dirujuk oleh `GOOGLE_APPLICATION_CREDENTIALS` memiliki prioritas lebih tinggi daripada itu. Jika tidak ada yang diatur, ID proyek diselesaikan dari konfigurasi `gcloud` Anda atau akun layanan yang terlampir.
</Note>

<h4 id="advanced-credential-configuration">
  Konfigurasi kredensial lanjutan
</h4>

Claude Code mendukung penyegaran kredensial otomatis untuk GCP melalui pengaturan `gcpAuthRefresh`. Ketika Claude Code mendeteksi bahwa kredensial GCP Anda telah kedaluwarsa atau tidak dapat dimuat, Claude Code menjalankan perintah yang dikonfigurasi untuk mendapatkan kredensial baru sebelum mencoba ulang permintaan.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Output perintah ditampilkan kepada pengguna, tetapi input interaktif tidak didukung. Ini berfungsi dengan baik untuk alur autentikasi berbasis browser di mana CLI menampilkan URL dan Anda menyelesaikan autentikasi di browser. Perintah penyegaran habis waktu setelah tiga menit jika autentikasi tidak selesai. Jika Anda mengatur `gcpAuthRefresh` dalam pengaturan proyek seperti `.claude/settings.json`, perintah hanya berjalan setelah Anda menerima prompt kepercayaan ruang kerja.

<h3 id="4-configure-claude-code">
  4. Konfigurasi Claude Code
</h3>

Atur variabel lingkungan berikut:

```bash theme={null}
# Aktifkan integrasi Agent Platform
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Opsional: Timpa URL titik akhir Agent Platform untuk titik akhir kustom atau gateway
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Opsional: Nonaktifkan prompt caching jika diperlukan
export DISABLE_PROMPT_CACHING=1

# Opsional: Minta TTL cache prompt 1 jam alih-alih default 5 menit
export ENABLE_PROMPT_CACHING_1H=1

# Ketika CLOUD_ML_REGION=global, timpa wilayah untuk model yang tidak mendukung titik akhir global
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

Sebagian besar versi model memiliki variabel `VERTEX_REGION_CLAUDE_*` yang sesuai. Lihat [referensi variabel lingkungan](/docs/id/env-vars) untuk daftar lengkap. Periksa [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) untuk menentukan model mana yang mendukung titik akhir global versus regional saja.

[Prompt caching](/docs/id/prompt-caching) diaktifkan secara otomatis. Untuk menonaktifkannya, atur `DISABLE_PROMPT_CACHING=1`. Untuk meminta TTL cache 1 jam alih-alih default 5 menit, atur `ENABLE_PROMPT_CACHING_1H=1`; penulisan cache dengan TTL 1 jam ditagih dengan tarif yang lebih tinggi. Untuk batas laju yang lebih tinggi, hubungi dukungan Google Cloud. Saat menggunakan Google Cloud's Agent Platform, perintah `/logout` tidak tersedia karena autentikasi ditangani melalui kredensial Google Cloud.

Claude Code menonaktifkan [pencarian alat MCP](/docs/id/mcp#scale-with-mcp-tool-search) secara default di Google Cloud's Agent Platform, sehingga definisi alat MCP dimuat di muka. Google Cloud's Agent Platform mendukung pencarian alat untuk Claude Sonnet 4.5 dan lebih baru serta Claude Opus 4.5 dan lebih baru. Atur `ENABLE_TOOL_SEARCH=true` untuk mengaktifkannya pada model tersebut. Model sebelumnya di Google Cloud's Agent Platform tidak menerima header beta yang diperlukan, dan permintaan gagal jika Anda mengaktifkan pencarian alat dengan model tersebut.

<h3 id="5-pin-model-versions">
  5. Pin versi model
</h3>

<Warning>
  Pin versi model spesifik saat menerapkan ke beberapa pengguna. Tanpa pinning, alias model seperti `sonnet` dan `opus` diselesaikan ke default bawaan Claude Code untuk Google Cloud's Agent Platform, yang dapat tertinggal dari rilis terbaru dan mungkin belum diaktifkan di proyek Anda. Claude Code [kembali](#startup-model-checks) ke versi sebelumnya atau model tingkat lebih rendah saat startup ketika default tidak tersedia, tetapi pinning memungkinkan Anda mengontrol kapan pengguna Anda pindah ke model baru.
</Warning>

Atur variabel lingkungan ini ke ID model Google Cloud's Agent Platform spesifik.

Tanpa `ANTHROPIC_DEFAULT_OPUS_MODEL`, alias `opus` di Google Cloud's Agent Platform diselesaikan ke Opus 4.8, dan tanpa `ANTHROPIC_DEFAULT_SONNET_MODEL`, alias `sonnet` diselesaikan ke Sonnet 4.5. Contoh ini pin setiap alias ke versi spesifik:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Untuk ID model saat ini dan warisan, lihat [Ikhtisar Model](https://platform.claude.com/docs/en/about-claude/models/overview). Lihat [Konfigurasi Model](/docs/id/model-config#pin-models-for-third-party-deployments) untuk daftar lengkap variabel lingkungan.

Claude Code menggunakan model default ini ketika tidak ada variabel pinning yang diatur:

| Jenis model       | Nilai default                |
| :---------------- | :--------------------------- |
| Model utama       | `claude-opus-4-8`            |
| Model kecil/cepat | `claude-sonnet-4-5@20250929` |

Tugas latar belakang seperti pembuatan judul sesi menggunakan model kecil/cepat, biasanya model kelas Haiku. Di Google Cloud's Agent Platform, Claude Code menggunakan model Sonnet default untuk tugas latar belakang karena Haiku mungkin tidak diaktifkan di setiap proyek atau wilayah. Dua pilihan mengubah model mana yang membawanya:

* Ketika Anda memilih model utama dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`, tugas latar belakang menggunakan model tersebut. Mengatur `ANTHROPIC_DEFAULT_OPUS_MODEL` tanpa `ANTHROPIC_DEFAULT_SONNET_MODEL` juga dihitung sebagai pilihan, karena model Sonnet bawaan mungkin tidak diaktifkan dalam proyek yang mengarahkan Opus-nya sendiri.
* Untuk menggunakan Haiku untuk tugas latar belakang, atur `ANTHROPIC_DEFAULT_HAIKU_MODEL` ke ID model yang tersedia di proyek Anda.

<Warning>
  Model Opus memiliki harga per-token yang lebih tinggi daripada model Sonnet, jadi penerapan yang tidak pin model utama ditagih dengan tarif Opus setelah diperbarui ke v2.1.207 atau lebih baru. Untuk menjaga Sonnet 4.5 sebagai model utama, atur `ANTHROPIC_MODEL` ke ID model lengkapnya. Penerapan yang mengarahkan default dengan `ANTHROPIC_DEFAULT_SONNET_MODEL` dan tidak mengatur `ANTHROPIC_DEFAULT_OPUS_MODEL` menjaga model Sonnet yang diarahkan sebagai default.
</Warning>

Sebelum v2.1.207, model utama di Google Cloud's Agent Platform default ke Sonnet 4.5, alias `opus` diselesaikan ke Opus 4.6, dan tugas latar belakang selalu menggunakan model utama.

Untuk menyesuaikan model lebih lanjut:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  Pemeriksaan model startup
</h2>

Ketika Claude Code dimulai dengan Google Cloud's Agent Platform dikonfigurasi, ia memverifikasi bahwa model yang dimaksudkan untuk digunakan dapat diakses di proyek Anda.

Jika Anda telah mempinkan versi model yang lebih lama dari default Claude Code saat ini, dan proyek Anda dapat memanggil versi yang lebih baru, Claude Code meminta Anda untuk memperbarui pin. Menerima menulis ID model baru ke [file pengaturan pengguna Anda](/docs/id/settings) dan memulai ulang Claude Code. Menolak diingat sampai perubahan versi default berikutnya.

Jika Anda belum mempinkan model dan default saat ini tidak tersedia di proyek Anda, Claude Code kembali untuk sesi saat ini dan menampilkan pemberitahuan. Ia mencoba versi sebelumnya dari model default terlebih dahulu dan, ketika default adalah model Opus dan tidak ada versi Opus yang tersedia, kembali ke model Sonnet default. Fallback tidak disimpan. Aktifkan model yang lebih baru di [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) atau [pin versi](#5-pin-model-versions) untuk membuat pilihan permanen.

<h2 id="iam-configuration">
  Konfigurasi IAM
</h2>

Tetapkan izin IAM yang diperlukan:

Peran `roles/aiplatform.user` mencakup izin yang diperlukan:

* `aiplatform.endpoints.predict` - Diperlukan untuk invokasi model dan penghitungan token

Untuk izin yang lebih ketat, buat peran kustom dengan hanya izin di atas.

Untuk detail, lihat [dokumentasi IAM Platform Agent Google Cloud](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Buat proyek GCP khusus untuk Claude Code untuk menyederhanakan pelacakan biaya dan kontrol akses.
</Note>

<h2 id="1m-token-context-window">
  Jendela konteks token 1M
</h2>

Claude Sonnet 5, Opus 4.6 dan yang lebih baru, serta Sonnet 4.6 mendukung [jendela konteks token 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) di Platform Agent Google Cloud. Sonnet 5 selalu berjalan dengan jendela 1M, tanpa varian `[1m]` untuk dipilih. Untuk model lainnya, Claude Code secara otomatis mengaktifkan jendela konteks yang diperluas ketika Anda memilih varian model 1M.

[Wizard pengaturan](#sign-in-with-agent-platform) menawarkan opsi konteks 1M ketika mempinkan model. Untuk mengaktifkannya untuk model yang dipinkan secara manual, tambahkan `[1m]` ke ID model. Lihat [Pin models for third-party deployments](/docs/id/model-config#pin-models-for-third-party-deployments) untuk detail.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

Jika Anda mengalami kesalahan "Could not load the default credentials":

* Jalankan `gcloud auth application-default login` untuk menyiapkan Application Default Credentials
* Atur `GOOGLE_APPLICATION_CREDENTIALS` ke jalur file kunci akun layanan
* Lihat [Configure GCP credentials](#3-configure-gcp-credentials) untuk semua opsi

Jika Anda mengalami masalah kuota:

* Periksa kuota saat ini atau minta peningkatan kuota melalui [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)

Jika Anda mengalami kesalahan "model not found" 404:

* Konfirmasi model diaktifkan di [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Verifikasi model tersedia di lokasi yang Anda tentukan. Beberapa model hanya ditawarkan di lokasi `global` atau multi-region seperti `eu` dan `us`, bukan di wilayah spesifik
* Jika menggunakan `CLOUD_ML_REGION=global`, periksa bahwa model Anda mendukung titik akhir global di [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) di bawah "Supported features". Untuk model yang tidak mendukung titik akhir global, baik:
  * Tentukan model yang didukung melalui `ANTHROPIC_MODEL` atau `ANTHROPIC_DEFAULT_HAIKU_MODEL`, atau
  * Atur wilayah atau lokasi multi-region menggunakan variabel lingkungan `VERTEX_REGION_<MODEL_NAME>`

Jika Anda mengalami kesalahan 429:

* Untuk titik akhir regional, pastikan model utama dan model kecil/cepat didukung di wilayah yang Anda pilih
* Pertimbangkan untuk beralih ke `CLOUD_ML_REGION=global` untuk ketersediaan yang lebih baik

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

* [Dokumentasi Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs)
* [Harga Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/pricing)
* [Kuota dan batas Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs/quotas)
