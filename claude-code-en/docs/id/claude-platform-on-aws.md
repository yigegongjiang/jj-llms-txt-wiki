> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code pada Claude Platform on AWS

> Konfigurasi Claude Code untuk menggunakan Claude API yang dioperasikan Anthropic dengan autentikasi AWS, kontrol akses IAM, dan penagihan AWS Marketplace.

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

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

Claude Platform on AWS adalah Claude API yang dioperasikan Anthropic dengan autentikasi AWS, kontrol akses IAM, dan penagihan AWS Marketplace. Permintaan mencapai API Anthropic secara langsung, sehingga Anda mendapatkan model dan fitur API yang sama seperti [Claude API](https://platform.claude.com/docs) dengan jadwal rilis yang sama. Fitur sisi klien yang Claude Code aktifkan melalui layanan feature-flag Anthropic, seperti [`/loop` pacing mandiri](/docs/id/scheduled-tasks#let-claude-choose-the-interval), dimatikan secara default, dan [alat advisor](/docs/id/advisor) tidak tersedia. Lihat [matriks ketersediaan fitur](/docs/id/feature-availability#summary-by-provider) untuk daftar lengkapnya. Anda melakukan autentikasi dengan kredensial AWS atau kunci API workspace, dan Anda membayar melalui AWS Marketplace.

Gunakan panduan ini untuk mengarahkan Claude Code ke workspace yang telah Anda sediakan melalui Claude Platform on AWS. Untuk langganan AWS dan penyiapan workspace yang dilakukan sebelumnya, lihat [dokumentasi Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  Berlangganan melalui AWS Marketplace menyediakan organisasi Anthropic baru yang terikat ke akun AWS Anda. Organisasi ini terpisah dari organisasi apa pun yang sudah Anda miliki dengan Anthropic, dan kredensial tidak ditransfer di antara keduanya. Gunakan ID workspace dan kunci API dari organisasi yang terhubung dengan AWS, bukan dari akun Claude Console yang sudah ada sebelumnya.
</Note>

<h2 id="prerequisites">
  Prasyarat
</h2>

Sebelum mengonfigurasi Claude Code, Anda memerlukan:

* Langganan Claude Platform on AWS aktif melalui AWS Marketplace
* Workspace di organisasi Anthropic yang terhubung dengan AWS Anda, dengan ID workspace-nya
* Prinsipal IAM dengan izin untuk memanggil layanan Anthropic, atau kunci API yang dibatasi pada workspace
* Kredensial AWS di lingkungan Anda, di `~/.aws/credentials`, atau dari peran IAM yang terpasang jika Anda ingin autentikasi SigV4. AWS CLI hanya diperlukan untuk alur login SSO.

<h2 id="setup">
  Penyiapan
</h2>

<h3 id="1-configure-aws-credentials">
  1. Konfigurasi kredensial AWS
</h3>

Claude Code mendukung dua metode autentikasi untuk Claude Platform on AWS. Pilih metode yang sesuai dengan cara tim Anda mengelola akses.

**Opsi A: Kredensial AWS dengan SigV4**

Claude Code menandatangani permintaan dengan SigV4 menggunakan rantai kredensial AWS standar: variabel lingkungan, kredensial bersama di `~/.aws/credentials`, peran IAM, sesi AWS SSO, dan sumber lain apa pun yang didukung AWS SDK.

Untuk penggunaan lokal, masuk dengan AWS CLI sebelum memulai Claude Code. Contoh di bawah menggunakan profil SSO, tetapi metode apa pun yang menghasilkan kredensial di lokasi standar berfungsi.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Untuk CI dan otomasi, berikan runner peran IAM dengan izin untuk memanggil layanan Anthropic dan atur `AWS_REGION`. Rantai kredensial mengambil peran secara otomatis.

Jika kredensial SSO Anda kedaluwarsa di tengah sesi, konfigurasi [`awsAuthRefresh`](/docs/id/amazon-bedrock#advanced-credential-configuration) sehingga Claude Code menjalankan kembali perintah login Anda dan mencoba lagi alih-alih gagal. Penyegaran otomatis pada Claude Platform on AWS memerlukan Claude Code v2.1.198 atau lebih baru; versi sebelumnya berhenti dengan prompt untuk menjalankan `/login`, yang tidak dapat menyegarkan kredensial AWS. Tambahkan perintah ke `settings.json` Anda:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Dengan `awsAuthRefresh` dikonfigurasi, `/login` menampilkan opsi **Claude Platform on AWS · refresh credentials** di bawah **Using 3rd-party platforms**. Memilihnya menjalankan perintah yang dikonfigurasi dan membaca ulang kredensial AWS Anda tanpa memulai ulang Claude Code.

**Opsi B: Kunci API Workspace**

Kunci API workspace adalah rahasia yang tahan lama, berguna ketika Anda tidak ingin mengelola kredensial AWS yang terfederasi. Buat satu di AWS Console di bawah **Claude Platform on AWS → API keys** dan aturnya sebagai `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

Kunci dikirim sebagai `x-api-key` dan mengambil prioritas atas SigV4, sehingga kredensial AWS apa pun di lingkungan Anda diabaikan. Kunci API dari organisasi Claude Console terpisah tidak akan berfungsi di sini.

Perlakukan kunci API workspace seperti kredensial produksi lainnya. Blok `env` [file pengaturan pengguna](/docs/id/settings) adalah cara yang nyaman untuk membatasi kunci ke mesin Anda tanpa mengekspornya secara global.

<Note>
  Perintah `/login` dan `/logout` tidak menandatangani Anda ke langganan Claude.ai untuk Claude Platform on AWS. Autentikasi berjalan melalui kredensial AWS Anda atau kunci API workspace. Pengecualiannya adalah opsi **refresh credentials** yang ditampilkan `/login` ketika `awsAuthRefresh` dikonfigurasi, yang membaca ulang kredensial AWS Anda seperti yang dijelaskan di atas.
</Note>

<h3 id="2-configure-claude-code">
  2. Konfigurasi Claude Code
</h3>

Atur variabel lingkungan yang mengarahkan Claude Code melalui Claude Platform on AWS alih-alih API Anthropic default.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` diperlukan dan dikirim pada setiap permintaan sebagai header `anthropic-workspace-id`. URL dasar dihitung dari `AWS_REGION` sebagai `https://aws-external-anthropic.{region}.api.aws`. Untuk mengganti URL secara langsung, atur `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS bersifat opt-in bahkan ketika kredensial AWS ada di lingkungan Anda. Amazon Bedrock dan Microsoft Foundry mengambil prioritas dalam perutean penyedia, jadi batalkan `CLAUDE_CODE_USE_BEDROCK` dan `CLAUDE_CODE_USE_FOUNDRY` jika diatur.

<h3 id="3-pin-model-versions">
  3. Sematkan versi model
</h3>

Claude Platform on AWS menggunakan ID model yang sama seperti Claude API langsung.

Alias default `fable`, `opus`, `sonnet`, dan `haiku` diselesaikan ke default bawaan Claude Code untuk Claude Platform on AWS, yang dapat tertinggal dari rilis terbaru. Tanpa `ANTHROPIC_DEFAULT_OPUS_MODEL`, alias `opus` diselesaikan ke Opus 4.8. Sebelum v2.1.207, alias tersebut diselesaikan ke Opus 4.7.

Jika Anda menerapkan Claude Code ke tim, sematkan ID model secara eksplisit sehingga rilis baru tidak memindahkan semua orang sekaligus:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Untuk daftar lengkap ID model dan alias, lihat [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk variabel terkait model lainnya, lihat [Model configuration](/docs/id/model-config).

[Prompt caching](/docs/id/prompt-caching) diaktifkan secara otomatis. Untuk meminta TTL cache 1 jam alih-alih default 5 menit, atur `ENABLE_PROMPT_CACHING_1H=1`. API menagih penulisan cache 1 jam dengan tarif lebih tinggi. Lihat [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) untuk tarifnya.

<h2 id="use-the-agent-sdk">
  Gunakan Agent SDK
</h2>

[Agent SDK](/docs/id/agent-sdk/overview) membaca variabel lingkungan yang sama dengan CLI, sehingga program apa pun yang menjalankan subprocess Claude Code dapat menargetkan Claude Platform on AWS dengan mengekspor `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID`, dan baik `ANTHROPIC_AWS_API_KEY` atau kredensial AWS sebelum panggilan.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Contoh ini bergantung pada rantai kredensial AWS ambient untuk SigV4. Untuk melakukan autentikasi dengan kunci API workspace sebagai gantinya, atur `ANTHROPIC_AWS_API_KEY` dengan cara yang sama. Untuk permukaan Agent SDK yang lebih luas, lihat [Agent SDK overview](/docs/id/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Rute melalui proxy korporat
</h2>

Untuk mengarahkan lalu lintas melalui proxy atau [LLM gateway](/docs/id/llm-gateway), atur `ANTHROPIC_AWS_BASE_URL` ke alamat proxy. Claude Code mengirim permintaan ke URL tersebut dengan header workspace dan autentikasi yang sama, sehingga gateway apa pun yang meneruskannya tanpa perubahan berfungsi.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Jika gateway Anda menandatangani permintaan sendiri, atur `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1` sehingga Claude Code mengirim permintaan yang tidak ditandatangani dan membiarkan gateway menambahkan header SigV4 sebelum meneruskan ke AWS. Jika gateway memerlukan token sendiri, aturnya di `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Jalankan `/status` untuk melihat penyedia yang diselesaikan dan ID workspace, region, penggantian URL dasar, dan pengaturan skip-auth yang dikonfigurasi secara eksplisit. Ini adalah cara tercepat untuk mengonfirmasi Claude Code menargetkan Claude Platform on AWS sama sekali.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` atau `AccessDenied` pada setiap permintaan
</h3>

Prinsipal IAM yang diselesaikan Claude Code kemungkinan besar tidak memiliki izin untuk memanggil layanan Anthropic di workspace Anda. Periksa peran yang terpasang pada profil AWS Anda atau runner yang memulai Claude Code, dan verifikasi bahwa ia memiliki tindakan `aws-external-anthropic` yang didokumentasikan dalam [referensi tindakan IAM](https://platform.claude.com/docs/id/api/claude-platform-on-aws-iam-actions).

Jika Anda menetapkan `ANTHROPIC_AWS_API_KEY`, kunci mengambil prioritas atas SigV4 dan kunci yang sudah usang menghasilkan kesalahan yang sama. Buat ulang kunci di AWS Console di bawah **Claude Platform on AWS → API keys** atau batalkan variabel untuk kembali ke kredensial AWS Anda.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Permintaan gagal dengan kesalahan workspace yang hilang
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` kemungkinan besar tidak diatur atau kosong. Setiap permintaan Claude Platform on AWS harus menyertakan ID workspace. Ini tidak tersirat oleh kredensial AWS Anda. Temukan ID di bawah **Workspaces** pada halaman layanan AWS Console dan ekspor sebelum memulai Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Permintaan masih pergi ke `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` kemungkinan besar tidak diatur atau diatur ke nilai yang tidak diuraikan sebagai truthy. Aturnya ke `1` dan jalankan `/status` untuk mengonfirmasi penyedia yang diselesaikan. Jika `CLAUDE_CODE_USE_BEDROCK` atau `CLAUDE_CODE_USE_FOUNDRY` juga diatur, keduanya mengambil prioritas atas Claude Platform on AWS.

<h2 id="additional-resources">
  Sumber daya tambahan
</h2>

Langganan Claude Platform on AWS, workspace, dan penyiapan IAM yang dilakukan sebelum mengonfigurasi Claude Code tercakup dalam dokumentasi platform:

* [Claude Platform on AWS overview](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws): langganan, penyiapan workspace, dan referensi produk
* [IAM action reference](https://platform.claude.com/docs/en/api/claude-platform-on-aws-iam-actions): izin dan kebijakan terkelola
