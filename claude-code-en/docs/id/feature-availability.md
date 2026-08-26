> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ketersediaan fitur

> Bandingkan fitur Claude Code mana yang tersedia di seluruh paket langganan Anthropic, Anthropic Console, Amazon Bedrock, Claude Platform di AWS, Platform Agent Google Cloud, dan Microsoft Foundry.

CLI Claude Code dan semua yang berjalan secara lokal bekerja di setiap penyedia. Untuk instruksi setup per penyedia, lihat [Ringkasan deployment enterprise](/docs/id/third-party-integrations). Untuk langsung ke apa yang hilang di penyedia Anda, lihat tab [ringkasan per penyedia](#summary-by-provider).

Dalam tabel di bawah, ✓ berarti tersedia, ✗ berarti tidak tersedia, dan "Lihat catatan" menghubungkan ke catatan kaki untuk dukungan parsial. Kualifikasi setelah ✓ mempersempit ketersediaan ke subset tersebut, dan "Admin-enabled" berarti fitur dimatikan sampai admin organisasi menyalakannya.

<h2 id="availability-by-model-provider">
  Ketersediaan per penyedia model
</h2>

Cara Anda melakukan autentikasi menentukan fitur mana yang dapat dijangkau Claude Code. Untuk daftar tunggal apa yang hilang di penyedia Anda, lihat tab [ringkasan per penyedia](#summary-by-provider). Untuk menemukan kolom Anda di tabel:

* **Langganan Claude**: Anda masuk dengan akun claude.ai di paket Pro, Max, Team, atau Enterprise
* **Anthropic Console**: Anda melakukan autentikasi dengan kunci API Anthropic
* **Amazon Bedrock**: Anda menggunakan model Claude dari katalog model Bedrock dan menetapkan `CLAUDE_CODE_USE_BEDROCK`. [Endpoint Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) tercakup oleh kolom ini
* **Claude Platform di AWS**: Anda membeli Claude melalui AWS Marketplace tetapi memanggil API Anthropic, dan menetapkan `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Platform Agent Google Cloud**: Dioperasikan Google; Anda menetapkan `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: Dioperasikan Anthropic di Azure; Anda menetapkan `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Fitur yang tersedia di setiap penyedia
</h3>

Ini bekerja di setiap penyedia:

* [CLI](/docs/id/quickstart) dan [Agent SDK](/docs/id/agent-sdk/overview)
* Ekstensi [VS Code](/docs/id/vs-code) dan [JetBrains](/docs/id/jetbrains)
* [Subagents](/docs/id/sub-agents), [hooks](/docs/id/hooks-guide), [commands](/docs/id/commands), dan [skills](/docs/id/skills)
* Memori [CLAUDE.md](/docs/id/memory), [plugins](/docs/id/plugins), dan [server MCP](/docs/id/mcp)
* [Checkpoints](/docs/id/checkpointing), [sandboxing](/docs/id/sandboxing), dan [Workflows](/docs/id/workflows)
* Metrik [OpenTelemetry](/docs/id/monitoring-usage) dan [file pengaturan terkelola](/docs/id/settings#settings-files)

Tiga di antaranya memiliki perbedaan khusus penyedia:

* **Server MCP**: [konektor dari claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) dimuat hanya ketika langganan claude.ai Anda adalah metode autentikasi aktif, dan [pencarian alat](/docs/id/mcp#configure-tool-search) dimatikan secara default di Platform Agent Google Cloud dan ketika `ANTHROPIC_BASE_URL` menunjuk ke host non-pihak pertama
* **Subagents**: [subagent Explore](/docs/id/sub-agents#built-in-subagents) bawaan membatasi model yang diwariskan pada Opus di Claude API, dan mewarisi model percakapan utama secara langsung di penyedia lain mana pun, termasuk Claude Platform di AWS
* **[Commands](/docs/id/commands#all-commands)**: `/design-sync` dan `/radio` tidak tersedia di Amazon Bedrock, Platform Agent Google Cloud, Microsoft Foundry, dan Claude Platform di AWS, dan `/voice` memerlukan akun claude.ai

<h3 id="features-that-require-a-claude-subscription">
  Fitur yang memerlukan langganan Claude
</h3>

Ini memerlukan masuk dengan akun claude.ai dan tidak dapat dijangkau dengan kunci API Anthropic Console atau dari penyedia pihak ketiga:

* [Claude Code di web](/docs/id/claude-code-on-the-web), Claude Code di mobile, dan [Claude Code di Slack](/docs/id/slack)
* [Claude Code Desktop](/docs/id/desktop)
* [Routines](/docs/id/routines) (`/schedule`)
* [Ultraplan](/docs/id/ultraplan) dan [Ultrareview](/docs/id/ultrareview)
* [Code Review](/docs/id/code-review): paket Team dan Enterprise
* [Remote Control](/docs/id/remote-control)
* [Ekstensi Chrome](/docs/id/chrome)
* [Computer use](/docs/id/computer-use): paket Pro dan Max
* [Artifacts](/docs/id/artifacts): paket Pro, Max, Team, dan Enterprise
* [Voice dictation](/docs/id/voice-dictation)

Desktop adalah pengecualian parsial: [perutean gateway dapat dikonfigurasi di aplikasi atau oleh administrator](/docs/id/llm-gateway-connect#desktop-app), deployment Enterprise dapat merutekan Desktop ke Platform Agent Google Cloud atau penyedia gateway melalui [pengaturan terkelola](https://claude.com/docs/third-party/claude-desktop/configuration), dan [Claude Desktop pada 3P](https://claude.com/docs/third-party/claude-desktop/overview) menjalankan tab Code di Amazon Bedrock, Platform Agent Google Cloud, Microsoft Foundry, atau gateway LLM yang di-host sendiri. Untuk ketersediaan per-paket fitur ini, lihat [Ketersediaan per paket langganan](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Kemampuan CLI yang bervariasi per penyedia
</h3>

Fitur ini bekerja di CLI lokal tetapi bergantung pada kemampuan sisi server yang tidak setiap penyedia paparkan.

<table>
  <thead>
    <tr>
      <th>Fitur</th>
      <th>Langganan Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform di AWS</th>
      <th>Platform Agent Google Cloud</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/id/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Lihat catatan <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/id/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/id/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Lihat catatan <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Lihat catatan <sup><a href="#fn2">2</a></sup></td>
      <td>Lihat catatan <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/id/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/id/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` scheduled tasks](/docs/id/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Lihat catatan <sup><a href="#fn3">3</a></sup></td>
      <td>Lihat catatan <sup><a href="#fn3">3</a></sup></td>
      <td>Lihat catatan <sup><a href="#fn3">3</a></sup></td>
      <td>Lihat catatan <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/id/github-actions) dan [GitLab CI/CD](/docs/id/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Admin dan analitik
</h3>

Kontrol tingkat organisasi dan visibilitas penggunaan.

<table>
  <thead>
    <tr>
      <th>Fitur</th>
      <th>Langganan Claude</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform di AWS</th>
      <th>Platform Agent Google Cloud</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Dashboard analitik dan API](/docs/id/analytics)</td>
      <td>✓ (dashboard: Team dan Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Pengaturan terkelola server](/docs/id/server-managed-settings)</td>
      <td>✓ (Team dan Enterprise)</td>
      <td>✓ (Team dan Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/id/zero-data-retention)</td>
      <td>✓ (akun Enterprise yang memenuhi syarat)</td>
      <td>✓ (akun yang memenuhi syarat)</td>
      <td>Lihat catatan <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (akun yang memenuhi syarat)</td>
      <td>Lihat catatan <sup><a href="#fn4">4</a></sup></td>
      <td>Lihat catatan <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> Di Platform Agent Google Cloud, web search tersedia untuk model Claude 4 dan yang lebih baru.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> Di penyedia ini, auto mode mendukung hanya Claude Sonnet 5, Opus 4.7, dan Opus 4.8. Lihat [Konfigurasi Auto mode](/docs/id/auto-mode-config). Di v2.1.158 hingga v2.1.206, auto mode di penyedia ini juga memerlukan pengaturan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 menghapus persyaratan.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Interval eksplisit seperti `/loop every 2 hours` bekerja di setiap penyedia. Di Amazon Bedrock, Claude Platform di AWS, Platform Agent Google Cloud, dan Microsoft Foundry, `/loop` tidak dapat memilih interval sendiri atau menyediakan prompt pemeliharaan default, jadi prompt tanpa interval berjalan setiap 10 menit, dan `/loop` tanpa argumen menampilkan pesan penggunaan. Lihat [Scheduled tasks](/docs/id/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Tunduk pada perjanjian Anda dengan penyedia cloud.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Dashboard dan API saja. [Metrik kontribusi](/docs/id/analytics#enable-contribution-metrics) memerlukan organisasi Team atau Enterprise claude.ai.

<Note>
  Jika Anda melakukan autentikasi melalui [gateway LLM](/docs/id/llm-gateway), ketersediaan fitur cocok dengan penyedia yang mendasarinya yang gateway teruskan. Beberapa fitur khusus Anthropic seperti [Advisor](/docs/id/advisor) hanya bekerja jika gateway meneruskan permintaan utuh ke API Anthropic.
</Note>

<h3 id="summary-by-provider">
  Ringkasan per penyedia
</h3>

Setiap tab mencantumkan apa yang tidak tersedia atau didukung sebagian di penyedia tersebut, dengan alternatif jika ada. Semua yang tidak tercantum bekerja sama seperti di langganan Claude, terlepas dari [perbedaan khusus penyedia](#features-available-on-every-provider) yang dicatat di atas. Di Amazon Bedrock, Platform Agent Google Cloud, Microsoft Foundry, dan Claude Platform di AWS, pelaporan kesalahan dan telemetri ke Anthropic dimatikan secara default. Lihat [perilaku default per penyedia API](/docs/id/data-usage#default-behaviors-by-api-provider) untuk lalu lintas apa yang masih mencapai Anthropic dan cara untuk tidak berpartisipasi.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Tidak tersedia:** semua [fitur yang memerlukan langganan Claude](#features-that-require-a-claude-subscription), ditambah [web search](/docs/id/tools-reference#websearch-tool-behavior), [fast mode](/docs/id/fast-mode), [Advisor](/docs/id/advisor), [Channels](/docs/id/channels), [dashboard analitik](/docs/id/analytics), [pengaturan terkelola server](/docs/id/server-managed-settings), dan [perintah `/design-sync` dan `/radio`](/docs/id/commands#all-commands).

    **Dukungan parsial:**

    * [Desktop](/docs/id/desktop): hanya melalui [Claude Desktop pada 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/id/auto-mode-config): Sonnet 5, Opus 4.7, dan Opus 4.8 saja
    * [`/loop`](/docs/id/scheduled-tasks): interval eksplisit saja
    * [Zero Data Retention](/docs/id/zero-data-retention): tunduk pada perjanjian AWS Anda

    **Alternatif:** untuk penjadwalan, gunakan [`/loop`](/docs/id/scheduled-tasks) dengan interval eksplisit alih-alih `/schedule`. Untuk sesi cloud, gunakan [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd). Untuk pencarian web, gunakan [alat WebFetch](/docs/id/tools-reference#webfetch-tool-behavior) dengan URL spesifik.
  </Tab>

  <Tab title="Claude Platform di AWS">
    **Tidak tersedia:** semua [fitur yang memerlukan langganan Claude](#features-that-require-a-claude-subscription), ditambah [fast mode](/docs/id/fast-mode), [Advisor](/docs/id/advisor), [Channels](/docs/id/channels), [dashboard analitik](/docs/id/analytics), [pengaturan terkelola server](/docs/id/server-managed-settings), dan [perintah `/design-sync` dan `/radio`](/docs/id/commands#all-commands).

    **Tersedia di mana Amazon Bedrock tidak:** [web search](/docs/id/tools-reference#websearch-tool-behavior).

    **Dukungan parsial:**

    * [`/loop`](/docs/id/scheduled-tasks): interval eksplisit saja

    **Alternatif:** untuk penjadwalan, gunakan [`/loop`](/docs/id/scheduled-tasks) dengan interval eksplisit alih-alih `/schedule`. Untuk sesi cloud, gunakan [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd).
  </Tab>

  <Tab title="Platform Agent Google Cloud">
    **Tidak tersedia:** semua [fitur yang memerlukan langganan Claude](#features-that-require-a-claude-subscription), ditambah [fast mode](/docs/id/fast-mode), [Advisor](/docs/id/advisor), [Channels](/docs/id/channels), [dashboard analitik](/docs/id/analytics), [pengaturan terkelola server](/docs/id/server-managed-settings), dan [perintah `/design-sync` dan `/radio`](/docs/id/commands#all-commands).

    **Dukungan parsial:**

    * [Desktop](/docs/id/desktop): melalui [pengaturan terkelola](https://claude.com/docs/third-party/claude-desktop/configuration) atau [Claude Desktop pada 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/id/tools-reference#websearch-tool-behavior): model Claude 4 dan yang lebih baru
    * [Auto mode](/docs/id/auto-mode-config): Sonnet 5, Opus 4.7, dan Opus 4.8 saja
    * [`/loop`](/docs/id/scheduled-tasks): interval eksplisit saja
    * [Zero Data Retention](/docs/id/zero-data-retention): tunduk pada perjanjian Google Cloud Anda

    **Alternatif:** untuk penjadwalan, gunakan [`/loop`](/docs/id/scheduled-tasks) dengan interval eksplisit alih-alih `/schedule`. Untuk sesi cloud, gunakan [GitHub Actions](/docs/id/github-actions) atau [GitLab CI/CD](/docs/id/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Tidak tersedia:** semua [fitur yang memerlukan langganan Claude](#features-that-require-a-claude-subscription), ditambah [fast mode](/docs/id/fast-mode), [Advisor](/docs/id/advisor), [Channels](/docs/id/channels), [GitHub Actions](/docs/id/github-actions) dan [GitLab CI/CD](/docs/id/gitlab-ci-cd), [dashboard analitik](/docs/id/analytics), [pengaturan terkelola server](/docs/id/server-managed-settings), dan [perintah `/design-sync` dan `/radio`](/docs/id/commands#all-commands).

    **Dukungan parsial:**

    * [Desktop](/docs/id/desktop): hanya melalui [Claude Desktop pada 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/id/auto-mode-config): Sonnet 5, Opus 4.7, dan Opus 4.8 saja
    * [`/loop`](/docs/id/scheduled-tasks): interval eksplisit saja
    * [Zero Data Retention](/docs/id/zero-data-retention): tunduk pada perjanjian Azure Anda

    **Alternatif:** untuk penjadwalan, gunakan [`/loop`](/docs/id/scheduled-tasks) dengan interval eksplisit alih-alih `/schedule`.
  </Tab>

  <Tab title="Anthropic Console">
    **Tidak tersedia:** semua [fitur yang memerlukan langganan Claude](#features-that-require-a-claude-subscription).

    Semua yang ada di [kemampuan CLI yang bervariasi per penyedia](#cli-capabilities-that-vary-by-provider) tersedia, seperti [pengaturan terkelola server](/docs/id/server-managed-settings) ketika kunci API milik organisasi Team atau Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Ketersediaan per paket langganan
</h2>

Jika Anda melakukan autentikasi melalui Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau kunci API Anthropic Console, bagian ini tidak berlaku untuk Anda. Ketika Anda masuk dengan akun claude.ai, paket Anda menentukan fitur mana di bawah ini yang tersedia.

| Fitur                                                                       | Pro | Max | Team          | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------------ | :-------------------------------- |
| [Claude Code di web](/docs/id/claude-code-on-the-web)                            | ✓   | ✓   | ✓             | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/id/routines)                                                    | ✓   | ✓   | ✓             | ✓                                 |
| [Remote Control](/docs/id/remote-control)                                        | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Channels](/docs/id/channels)                                                    | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Computer use](/docs/id/computer-use)                                            | ✓   | ✓   | ✗             | ✗                                 |
| Dispatch ([Desktop](/docs/id/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗             | ✗                                 |
| [Code Review](/docs/id/code-review)                                              | ✗   | ✗   | ✓             | ✓                                 |
| [Artifacts](/docs/id/artifacts)                                                  | ✓   | ✓   | ✓             | Admin-enabled                     |
| [Dashboard analitik dan metrik kontribusi](/docs/id/analytics)                   | ✗   | ✗   | ✓             | ✓                                 |
| [Enterprise Analytics API](/docs/id/analytics#access-data-programmatically)      | ✗   | ✗   | ✗             | ✓                                 |
| [Pengaturan terkelola server](/docs/id/server-managed-settings)                  | ✗   | ✗   | ✓             | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓             | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗             | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗             | ✓                                 |
| [Zero Data Retention](/docs/id/zero-data-retention)                              | ✗   | ✗   | ✗             | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> Di Enterprise, memerlukan kursi premium atau kursi Chat + Claude Code. Lihat [Claude Code di web](/docs/id/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Tidak termasuk dalam paket Enterprise standar. Memerlukan aktivasi terpisah oleh Anthropic untuk akun yang memenuhi syarat. Lihat [Zero Data Retention](/docs/id/zero-data-retention).

Untuk harga dan perbandingan paket lengkap, lihat [Paket Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) dan [Paket Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Ketersediaan model
</h2>

Untuk model Claude mana dan ukuran jendela konteks yang tersedia per penyedia dan wilayah, lihat [Konfigurasi model](/docs/id/model-config) dan [Ringkasan Model](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, input PDF, dan extended thinking adalah kemampuan model daripada fitur Claude Code dan bekerja di setiap penyedia yang menawarkan model. [Prompt caching](/docs/id/prompt-caching) bekerja dengan cara yang sama di sebagian besar penyedia; di Amazon Bedrock, dukungan bervariasi menurut model.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Ringkasan deployment enterprise](/docs/id/third-party-integrations): bandingkan autentikasi, penagihan, dan wilayah di seluruh penyedia
* Panduan setup penyedia: [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform di AWS](/docs/id/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry)
* [Platform dan integrasi](/docs/id/platforms): di mana Claude Code berjalan, termasuk CLI, Desktop, ekstensi IDE, web, mobile, dan CI/CD
