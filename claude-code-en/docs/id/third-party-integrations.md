> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ikhtisar penyebaran enterprise

> Pelajari bagaimana Claude Code dapat terintegrasi dengan berbagai layanan pihak ketiga dan infrastruktur untuk memenuhi persyaratan penyebaran enterprise.

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

Organisasi dapat menyebarkan Claude Code melalui Anthropic secara langsung atau melalui penyedia cloud. Halaman ini membantu Anda memilih konfigurasi yang tepat.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Bandingkan opsi penyebaran
</h2>

Untuk sebagian besar organisasi, Claude for Teams atau Claude for Enterprise memberikan pengalaman terbaik. Anggota tim mendapatkan akses ke Claude Code dan Claude di web dengan satu langganan, penagihan terpusat, dan tidak ada setup infrastruktur yang diperlukan.

**Claude for Teams** adalah self-service dan mencakup fitur kolaborasi, alat admin, dan manajemen penagihan. Terbaik untuk tim yang lebih kecil yang perlu memulai dengan cepat.

**Claude for Enterprise** menambahkan SSO dan domain capture, izin berbasis peran, akses API kepatuhan, dan pengaturan kebijakan terkelola untuk menyebarkan konfigurasi Claude Code di seluruh organisasi. Terbaik untuk organisasi yang lebih besar dengan persyaratan keamanan dan kepatuhan.

Pelajari lebih lanjut tentang [rencana Tim](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) dan [rencana Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Jika organisasi Anda memiliki persyaratan infrastruktur khusus, bandingkan opsi di bawah ini:

<table>
  <thead>
    <tr>
      <th>Fitur</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, formerly Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Terbaik untuk</td>
      <td>Sebagian besar organisasi (direkomendasikan)</td>
      <td>Pengembang individual</td>
      <td>Penyebaran native AWS</td>
      <td>Penagihan AWS Marketplace dengan fitur Claude API</td>
      <td>Penyebaran native GCP</td>
      <td>Penyebaran native Azure</td>
    </tr>

    <tr>
      <td>Penagihan</td>
      <td><strong>Teams:</strong> \$150/seat (Premium) dengan PAYG tersedia<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Hubungi Penjualan</a></td>
      <td>PAYG</td>
      <td>PAYG melalui AWS</td>
      <td>PAYG melalui AWS Marketplace</td>
      <td>PAYG melalui GCP</td>
      <td>PAYG melalui Azure</td>
    </tr>

    <tr>
      <td>Wilayah</td>
      <td>Didukung [negara](https://www.anthropic.com/supported-countries)</td>
      <td>Didukung [negara](https://www.anthropic.com/supported-countries)</td>
      <td>Beragam [wilayah](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) AWS</td>
      <td>Beragam wilayah AWS</td>
      <td>Beragam [wilayah](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) GCP</td>
      <td>Beragam [wilayah](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/) Azure</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Diaktifkan secara default</td>
      <td>Diaktifkan secara default</td>
      <td>Diaktifkan secara default</td>
      <td>Diaktifkan secara default</td>
      <td>Diaktifkan secara default</td>
      <td>Diaktifkan secara default</td>
    </tr>

    <tr>
      <td>Autentikasi</td>
      <td>Claude.ai SSO atau email</td>
      <td>Kunci API</td>
      <td>Kunci API atau kredensial AWS</td>
      <td>Kunci API atau kredensial AWS</td>
      <td>Kredensial GCP</td>
      <td>Kunci API atau Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Pelacakan biaya</td>
      <td>Dashboard penggunaan</td>
      <td>Dashboard penggunaan</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP Billing</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>Termasuk Claude di web</td>
      <td>Ya</td>
      <td>Tidak</td>
      <td>Tidak</td>
      <td>Tidak</td>
      <td>Tidak</td>
      <td>Tidak</td>
    </tr>

    <tr>
      <td>Fitur enterprise</td>
      <td>Manajemen tim, SSO, pemantauan penggunaan</td>
      <td>Tidak ada</td>
      <td>Kebijakan IAM, CloudTrail</td>
      <td>Kebijakan IAM, CloudTrail</td>
      <td>Peran IAM, Cloud Audit Logs</td>
      <td>Kebijakan RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Untuk rincian fitur demi fitur tentang apa yang tersedia di setiap opsi, lihat [Ketersediaan fitur](/docs/id/feature-availability).

Pilih opsi penyebaran untuk melihat instruksi setup:

* [Claude for Teams atau Enterprise](/docs/id/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/id/authentication#claude-console-authentication)
* [Claude apps gateway](/docs/id/claude-apps-gateway), gateway yang di-host sendiri yang menambahkan sign-in IdP di depan Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry, atau Anthropic API
* [Amazon Bedrock](/docs/id/amazon-bedrock)
* [Claude Platform on AWS](/docs/id/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/id/google-vertex-ai)
* [Microsoft Foundry](/docs/id/microsoft-foundry)

Untuk Amazon Bedrock dan Google Vertex AI, Anda juga dapat menjalankan `claude` dan memilih **3rd-party platform** di prompt login untuk meluncurkan wizard setup interaktif.

<h2 id="configure-proxies-and-gateways">
  Konfigurasi proxy dan gateway
</h2>

Sebagian besar organisasi dapat menggunakan penyedia cloud secara langsung tanpa konfigurasi tambahan. Namun, Anda mungkin perlu mengonfigurasi proxy perusahaan atau gateway LLM jika organisasi Anda memiliki persyaratan jaringan atau manajemen khusus. Ini adalah konfigurasi berbeda yang dapat digunakan bersama:

* **Corporate proxy**: Merutekan lalu lintas melalui proxy HTTP/HTTPS. Gunakan ini jika organisasi Anda memerlukan semua lalu lintas keluar untuk melewati server proxy untuk pemantauan keamanan, kepatuhan, atau penegakan kebijakan jaringan. Konfigurasi dengan variabel lingkungan `HTTPS_PROXY` atau `HTTP_PROXY`. Pelajari lebih lanjut di [Konfigurasi jaringan enterprise](/docs/id/network-config).
* **LLM Gateway**: Layanan yang berada di antara Claude Code dan penyedia cloud untuk menangani autentikasi dan perutean. Gunakan ini jika Anda memerlukan pelacakan penggunaan terpusat di seluruh tim, pembatasan laju kustom atau anggaran, atau manajemen autentikasi terpusat. Konfigurasi dengan variabel lingkungan `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, atau `ANTHROPIC_FOUNDRY_BASE_URL`. Pelajari lebih lanjut di [Gateway LLM](/docs/id/llm-gateway).

Contoh berikut menunjukkan variabel lingkungan yang harus diatur di shell atau profil shell Anda (`.bashrc`, `.zshrc`). Lihat [Pengaturan](/docs/id/settings) untuk metode konfigurasi lainnya.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Corporate proxy">
    Rutekan lalu lintas Amazon Bedrock melalui proxy perusahaan Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Rutekan lalu lintas Amazon Bedrock melalui gateway LLM Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1

    # Configure LLM gateway
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # If gateway handles AWS auth
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="Corporate proxy">
    Rutekan lalu lintas Microsoft Foundry melalui proxy perusahaan Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Or omit for Entra ID auth

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Rutekan lalu lintas Microsoft Foundry melalui gateway LLM Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1

    # Configure LLM gateway
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # Sent as x-api-key
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="Corporate proxy">
    Rutekan lalu lintas Google Cloud's Agent Platform melalui proxy perusahaan Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Rutekan lalu lintas Google Cloud's Agent Platform melalui gateway LLM Anda dengan mengatur [variabel lingkungan](/docs/id/env-vars) berikut:

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1

    # Configure LLM gateway
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # If gateway handles GCP auth
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Gunakan `/status` di Claude Code untuk memverifikasi bahwa konfigurasi proxy dan gateway Anda diterapkan dengan benar. Misalnya, dengan konfigurasi gateway Bedrock di atas, output mencakup baris seperti:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Jika Anda mengonfigurasi proxy perusahaan, `/status` juga menampilkan baris `Proxy` dengan URL proxy Anda.
</Tip>

<h2 id="best-practices-for-organizations">
  Praktik terbaik untuk organisasi
</h2>

<h3 id="invest-in-documentation-and-memory">
  Investasi dalam dokumentasi dan memori
</h3>

Kami sangat merekomendasikan investasi dalam dokumentasi sehingga Claude Code memahami basis kode Anda. Organisasi dapat menyebarkan file CLAUDE.md di berbagai tingkat:

* **Seluruh organisasi**: Sebarkan ke direktori sistem seperti `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux dan WSL), atau `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) untuk standar perusahaan
* **Tingkat repositori**: Buat file `CLAUDE.md` di akar repositori yang berisi arsitektur proyek, perintah build, dan panduan kontribusi. Periksa ini ke dalam kontrol sumber sehingga semua pengguna mendapat manfaat

Pelajari lebih lanjut di [Memori dan file CLAUDE.md](/docs/id/memory).

<h3 id="simplify-deployment">
  Sederhanakan penyebaran
</h3>

Jika Anda memiliki lingkungan pengembangan kustom, kami menemukan bahwa membuat cara "satu klik" untuk menginstal Claude Code adalah kunci untuk meningkatkan adopsi di seluruh organisasi.

<h3 id="start-with-guided-usage">
  Mulai dengan penggunaan terpandu
</h3>

Dorong pengguna baru untuk mencoba Claude Code untuk Q\&A basis kode, atau pada perbaikan bug yang lebih kecil atau permintaan fitur. Minta Claude Code untuk membuat rencana. Periksa saran Claude dan berikan umpan balik jika tidak sesuai. Seiring waktu, ketika pengguna memahami paradigma baru ini dengan lebih baik, mereka akan lebih efektif dalam membiarkan Claude Code berjalan lebih agentik.

<h3 id="pin-model-versions-for-cloud-providers">
  Versi model pin untuk penyedia cloud
</h3>

Jika Anda menyebarkan melalui [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws), pin versi model tertentu menggunakan `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, dan `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Tanpa pinning, alias model menyelesaikan ke default bawaan Claude Code untuk penyedia tersebut, yang dapat tertinggal dari rilis terbaru dan mungkin belum diaktifkan di akun Anda. Pinning memungkinkan Anda mengontrol kapan pengguna Anda pindah ke model baru. Lihat [Konfigurasi model](/docs/id/model-config#pin-models-for-third-party-deployments) untuk apa yang dilakukan setiap penyedia ketika default tidak tersedia.

<h3 id="configure-security-policies">
  Konfigurasi kebijakan keamanan
</h3>

Tim keamanan dapat mengonfigurasi izin terkelola untuk apa yang Claude Code diizinkan dan tidak diizinkan untuk lakukan, yang tidak dapat ditimpa oleh konfigurasi lokal. [Pelajari lebih lanjut](/docs/id/security).

<h3 id="leverage-mcp-for-integrations">
  Manfaatkan MCP untuk integrasi
</h3>

MCP adalah cara yang bagus untuk memberikan Claude Code lebih banyak informasi, seperti menghubungkan ke sistem manajemen tiket atau log kesalahan. Kami merekomendasikan bahwa satu tim pusat mengonfigurasi server MCP dan memeriksa konfigurasi `.mcp.json` ke dalam basis kode sehingga semua pengguna mendapat manfaat. [Pelajari lebih lanjut](/docs/id/mcp).

Di Anthropic, kami mempercayai Claude Code untuk mendorong pengembangan di seluruh setiap basis kode Anthropic. Kami harap Anda menikmati menggunakan Claude Code sebanyak yang kami lakukan.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Setelah Anda memilih opsi penyebaran dan mengonfigurasi akses untuk tim Anda:

1. **Luncurkan ke tim Anda**: Bagikan instruksi instalasi dan minta anggota tim [menginstal Claude Code](/docs/id/setup) dan autentikasi dengan kredensial mereka.
2. **Atur konfigurasi bersama**: Buat [file CLAUDE.md](/docs/id/memory) di repositori Anda untuk membantu Claude Code memahami basis kode dan standar pengkodean Anda.
3. **Konfigurasi izin**: Tinjau [pengaturan keamanan](/docs/id/security) untuk menentukan apa yang dapat dan tidak dapat dilakukan Claude Code di lingkungan Anda.
