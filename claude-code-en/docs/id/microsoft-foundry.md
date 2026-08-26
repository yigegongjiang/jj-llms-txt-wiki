> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code di Microsoft Foundry

> Pelajari tentang mengonfigurasi Claude Code melalui Microsoft Foundry, termasuk setup, konfigurasi, dan pemecahan masalah.

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

<ContactSalesCard surface="foundry" />

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum mengonfigurasi Claude Code dengan Microsoft Foundry, pastikan Anda memiliki:

* Langganan Azure dengan akses ke Microsoft Foundry
* Izin RBAC untuk membuat sumber daya dan deployment Microsoft Foundry
* Azure CLI diinstal dan dikonfigurasi (opsional - hanya diperlukan jika Anda tidak memiliki mekanisme lain untuk mendapatkan kredensial)

<Note>
  Jika Anda menerapkan Claude Code ke beberapa pengguna, [pin versi model Anda](#4-pin-model-versions) sebelum melakukan peluncuran.
</Note>

<h2 id="setup">
  Setup
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Menyediakan sumber daya Microsoft Foundry
</h3>

Pertama, buat sumber daya Claude di Azure:

1. Navigasikan ke [portal Microsoft Foundry](https://ai.azure.com/)
2. Buat sumber daya baru, catat nama sumber daya Anda
3. Buat deployment untuk model Claude, catat nama deployment yang Anda berikan untuk masing-masing; Anda akan menetapkan nama-nama ini sebagai variabel model di langkah 4:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Konfigurasi kredensial Azure
</h3>

Claude Code mendukung tiga metode autentikasi untuk Microsoft Foundry. Pilih metode yang paling sesuai dengan persyaratan keamanan Anda.

**Opsi A: Autentikasi kunci API**

1. Navigasikan ke sumber daya Anda di portal Microsoft Foundry
2. Buka bagian **Endpoints and keys**
3. Salin **API Key**
4. Atur variabel lingkungan, ganti `your-azure-api-key` dengan kunci yang Anda salin:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**Opsi B: Autentikasi Microsoft Entra ID**

Ketika `ANTHROPIC_FOUNDRY_API_KEY` dan `ANTHROPIC_FOUNDRY_AUTH_TOKEN` tidak diatur, Claude Code secara otomatis menggunakan Azure SDK [rantai kredensial default](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview).
Ini mendukung berbagai metode untuk mengautentikasi beban kerja lokal dan jarak jauh.

Di lingkungan lokal, Anda biasanya dapat menggunakan Azure CLI:

```bash theme={null}
az login
```

**Opsi C: Autentikasi bearer token**

Claude Code mengirimkan nilai `ANTHROPIC_FOUNDRY_AUTH_TOKEN` pada setiap permintaan sebagai header `Authorization: Bearer`. Gunakan opsi ini ketika proses lain, seperti aplikasi host atau skrip sign-in, telah memperoleh token akses untuk Anda. Memerlukan Claude Code v2.1.203 atau lebih baru.

Atur variabel ke bearer token yang dikeluarkan Microsoft Entra ID untuk sumber daya Anda:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` memiliki prioritas lebih tinggi daripada `ANTHROPIC_FOUNDRY_API_KEY` dan rantai kredensial default.

<Note>
  Saat menggunakan Microsoft Foundry, perintah `/logout` tidak tersedia karena autentikasi ditangani melalui kredensial Azure.
</Note>

<h3 id="3-configure-claude-code">
  3. Konfigurasi Claude Code
</h3>

Atur variabel lingkungan berikut untuk mengaktifkan Microsoft Foundry:

```bash theme={null}
# Aktifkan integrasi Microsoft Foundry
export CLAUDE_CODE_USE_FOUNDRY=1

# Nama sumber daya Azure (ganti {resource} dengan nama sumber daya Anda)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# Atau berikan URL dasar lengkap:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. Pin model versions
</h3>

<Warning>
  Tetapkan versi model spesifik untuk setiap deployment. Tanpa penentuan, alias model seperti `sonnet` dan `opus` diselesaikan ke default bawaan Claude Code untuk Microsoft Foundry, yang dapat tertinggal dari rilis terbaru dan mungkin belum tersedia di akun Anda. Microsoft Foundry tidak memiliki pemeriksaan model startup, jadi permintaan gagal ketika default tidak tersedia. Ketika Anda membuat deployment Azure, pilih versi model spesifik daripada "auto-update to latest."
</Warning>

Atur variabel model agar sesuai dengan nama deployment yang Anda buat di langkah 1.

Tanpa `ANTHROPIC_DEFAULT_OPUS_MODEL`, alias `opus` di Microsoft Foundry diselesaikan ke Opus 4.6. Aturnya ke ID Opus 4.8 untuk menggunakan model terbaru:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

Tugas latar belakang seperti pembuatan judul sesi menggunakan model kecil/cepat, biasanya model kelas Haiku. Di Microsoft Foundry, Claude Code secara default menggunakan model utama karena tidak setiap akun memiliki deployment Haiku. Untuk menggunakan Haiku untuk tugas latar belakang, atur `ANTHROPIC_DEFAULT_HAIKU_MODEL` ke deployment Haiku yang tersedia di akun Anda, seperti yang ditunjukkan di atas.

Untuk ID model saat ini dan legacy, lihat [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Lihat [Model configuration](/docs/id/model-config#pin-models-for-third-party-deployments) untuk daftar lengkap variabel lingkungan.

[Prompt caching](/docs/id/prompt-caching) diaktifkan secara otomatis. Untuk meminta TTL cache 1 jam alih-alih default 5 menit, atur variabel berikut; cache writes dengan TTL 1 jam ditagih dengan tarif yang lebih tinggi:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Jalankan Claude Code
</h3>

Dengan variabel lingkungan yang diatur, mulai Claude Code dari direktori proyek Anda:

```bash theme={null}
claude
```

Claude Code membaca `CLAUDE_CODE_USE_FOUNDRY` dan variabel Microsoft Foundry lainnya dari lingkungan dan terhubung ke sumber daya Azure Anda pada prompt pertama. Tidak seperti Amazon Bedrock dan Google Cloud's Agent Platform, Microsoft Foundry tidak memiliki wizard setup interaktif, jadi variabel lingkungan di langkah 3 dan 4 adalah satu-satunya jalur konfigurasi.

Untuk memverifikasi setup Anda, jalankan `/status` di dalam Claude Code. Baris penyedia API menunjukkan `Microsoft Foundry`, bersama dengan nama sumber daya atau URL dasar yang Anda konfigurasi.

<h2 id="azure-rbac-configuration">
  Konfigurasi Azure RBAC
</h2>

Peran default `Azure AI User` dan `Cognitive Services User` mencakup semua izin yang diperlukan untuk memanggil model Claude.

Untuk izin yang lebih ketat, buat peran khusus dengan yang berikut:

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

Untuk detail, lihat [dokumentasi RBAC Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry).

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

Jika Anda menerima kesalahan "Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed":

* Konfigurasi Entra ID di lingkungan, atau atur `ANTHROPIC_FOUNDRY_API_KEY`.

Jika permintaan gagal dengan kesalahan koneksi berulang pada prompt pertama:

* Periksa bahwa `ANTHROPIC_FOUNDRY_RESOURCE` diatur ke nama sumber daya aktual Anda daripada placeholder. Claude Code membangun URL endpoint dari nilai ini, jadi nama yang salah menunjuk ke host yang tidak ada.

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

* [Dokumentasi Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Model Microsoft Foundry](https://ai.azure.com/explore/models)
* [Harga Microsoft Foundry](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
