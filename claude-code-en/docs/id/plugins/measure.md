> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ukur biaya dan penggunaan plugin

> Ukur biaya token plugin Claude Code, cari tahu apakah orang masih menggunakannya, dan pilih peristiwa telemetri untuk pertanyaan plugin di seluruh organisasi.

Setiap sesi di mana plugin diaktifkan mencakup nama dan deskripsi skills, agents, dan commands-nya dalam konteks Claude, dan token tersebut dihitung terhadap penggunaan pengguna terlepas dari apakah plugin digunakan atau tidak. Halaman ini menunjukkan cara melihat angka tersebut untuk plugin, cara menguranginya jika Anda memelihara plugin, dan di mana penggunaan ditampilkan sehingga Anda dapat mengetahui apakah plugin masih digunakan.

Halaman ini untuk penulis dan pemelihara plugin. Jika Anda mengelola Claude Code untuk organisasi, [Measure across a fleet](#measure-across-a-fleet) mencakup pertanyaan yang sama di setiap mesin.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Menguji seberapa andal plugin mengubah perilaku Claude**: lihat [Test plugins with evals](/docs/id/plugin-evals)
  * **Memangkas konteks sesi Anda sendiri**: lihat [Manage installed plugins](/docs/id/plugins/install#manage-installed-plugins) dan halaman [context window](/docs/id/context-window)
</Note>

Mulai dengan [Measure what a plugin costs](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Measure what a plugin costs
</h2>

Untuk melihat apa yang ditambahkan plugin ke konteks Claude, jalankan [`claude plugin details`](/docs/id/plugins/cli-reference#plugin-details) dengan nama plugin. Anda menjalankannya di shell Anda, bukan di prompt sesi Claude Code yang sedang berjalan. Plugin harus dimuat: diinstal, di direktori skills, atau diteruskan dengan `--plugin-dir` dalam perintah yang sama, seperti dalam `claude --plugin-dir ./formatter plugin details formatter`.

Contoh ini membaca plugin yang diinstal bernama `formatter` yang memiliki dua skills, satu command, satu agent, satu hook, dan satu server MCP:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Setiap bagian dari output menjawab pertanyaan yang berbeda:

* **Component inventory**: apa yang ditemukan Claude Code di plugin. Commands dihitung dengan skills, jadi `format-all` muncul di bawah `Skills`. Hooks dan server MCP tidak mendapatkan estimasi biaya dan tidak ada baris per-component; untuk melihat apa yang ditambahkan tools MCP plugin, jalankan `/context` dalam sesi dengan plugin diaktifkan dan baca kategori `MCP tools`.
* **Always-on**: token yang ditambahkan nama dan deskripsi skills, agents, dan commands plugin ke setiap sesi di mana plugin diaktifkan, terlepas dari apakah ada yang berjalan. Ini adalah angka yang dibawa setiap pengguna, dan yang harus dikurangi.
* **Per-component**: setiap baris membagi satu skill, agent, atau command menjadi bagian always-on dan biaya on-invoke, yang merupakan body yang dimuat hanya ketika component tersebut berjalan. Gunakan kolom always-on untuk menemukan component mana yang berkontribusi paling banyak.

<h3 id="lower-the-always-on-figure">
  Lower the always-on figure
</h3>

Jika Anda memelihara plugin, perubahan ini mengurangi apa yang ditambahkannya ke setiap sesi. Jika Anda hanya menggunakannya, pilihan Anda adalah menonaktifkan atau mencopot pemasangannya; lihat [Manage installed plugins](/docs/id/plugins/install#manage-installed-plugins).

Angka always-on menghitung nama setiap component ditambah `description` dan `when_to_use` frontmatter-nya. Untuk menurunkannya:

* Perpendek deskripsi skill dan agent.
* Pisahkan plugin besar sehingga pengguna hanya memasang component yang mereka butuhkan.

Deskripsi skill juga apa yang dicocokkan Claude terhadap permintaan, jadi yang lebih pendek dapat menghentikan skill dari pemicu. Setelah Anda memangkas deskripsi, periksa pemicu dengan [grader `tool_used: Skill`](/docs/id/plugin-evals#create-your-first-eval-suite) dalam suite eval Anda.

Untuk apa yang berkontribusi setiap jenis component, lihat [plugin components](/docs/id/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Cost shown to users before install
</h3>

Plugin di marketplace resmi menunjukkan biayanya kepada pengguna sebelum pemasangan. Di `/plugin`, ketika pengguna menjelajahi daftar plugin marketplace dan memilih plugin, pane detail menunjukkan bagian **Context cost** dengan baris `Every turn:` dan baris `When invoked:`. Ketika angka always-on adalah 2.000 token atau lebih, baris `Every turn:` muncul disorot.

Plugin di marketplace Anda sendiri tidak memiliki bagian **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Check whether a plugin is used
</h2>

Claude Code tidak melaporkan penggunaan plugin kembali ke pembuatnya. Penggunaan dicatat di mesin setiap orang yang memasang plugin, jadi apa yang dapat Anda pelajari tergantung pada hubungan Anda dengan orang-orang tersebut:

* **Anda mengelola Claude Code untuk organisasi mereka**: peristiwa OpenTelemetry dan Analytics API menghitung pemasangan dan aktivasi skill di setiap mesin. Lihat [Measure across a fleet](#measure-across-a-fleet).
* **Mereka adalah rekan kerja yang dapat Anda tanyai**: Claude Code setiap pengguna menunjukkan kepada mereka apakah mereka masih menggunakan plugin, di empat tempat: panel [`/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor), dan [`/usage`](#usage-share-in-/usage). Keempat perintah ini dijalankan pengguna di prompt Claude Code dalam sesi di mesin mereka sendiri.
* **Tidak satupun**: Anda tidak memiliki sinyal penggunaan dari Claude Code untuk plugin tersebut.

<h3 id="not-used-recently-in-/plugin">
  Not used recently in `/plugin`
</h3>

Di tab **Installed** dari `/plugin`, plugin yang dipasang pengguna dari marketplace bergerak di bawah header **Not used recently** setelah tidak digunakan selama minimal 14 hari dan 10 sesi. Detail plugin juga menunjukkan baris `Last used:`. Untuk apa yang dilakukan pengguna dengan header dan baris tersebut, lihat [Find plugins you no longer use](/docs/id/plugins/install#find-plugins-you-no-longer-use).

Header **Not used recently** tidak pernah muncul untuk:

* Plugin yang dimuat dengan `--plugin-dir` atau dari direktori skills
* Plugin yang diaktifkan melalui managed settings, atau dipasang dari [seed directory](/docs/id/plugins/org#seed-containers-and-ci)
* Plugin yang menyertakan tema, output style, monitor, atau workflow, karena ini digunakan tanpa invocation yang dilacak

[Language server](/docs/id/plugins/components#lsp-servers) plugin dihitung sebagai digunakan ketika memberikan diagnostik atau menjawab permintaan navigasi kode, jadi plugin LSP yang servernya aktif dalam sesi Anda tidak terdaftar sebagai tidak digunakan.

Ketika organisasi pengguna menetapkan [`strictKnownMarketplaces`](/docs/id/plugins/org#restrict-what-users-can-install), baik header maupun baris `Last used:` tidak muncul.

<h3 id="find-skills-that-never-run">
  Find skills that never run
</h3>

Jalankan `/skill-doctor` untuk melihat apa yang dikerjakan setiap skill Anda dan seberapa sering digunakan. Ini menandai skills yang ada dalam daftar skill Claude tetapi tidak pernah dijalankan, termasuk skills dari plugins.

Dalam sesi interaktif, laporan terbuka di tab **Stats** manajer `/plugin`. Lihat [Find unused skills](/docs/id/skills#find-unused-skills) untuk apa yang dicakup laporan dan di mana tersedia.

<h3 id="unused-plugins-in-/doctor">
  Unused plugins in `/doctor`
</h3>

Checkup `/doctor` mencantumkan setiap skill yang dipasang pengguna, server MCP, dan plugin, dan merekomendasikan menonaktifkan yang tidak digunakan. Lihat [`/doctor` dalam referensi commands](/docs/id/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Usage share in `/usage`
</h3>

Pada paket Pro, Max, Team, atau Enterprise, breakdown `/usage` mengatribusikan penggunaan terbaru kepada skills, subagents, plugins, dan server MCP sebagai bagian dari total. Lihat [Using the `/usage` command](/docs/id/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Measure across a fleet
</h2>

Jika Anda mengelola Claude Code untuk organisasi, Anda dapat mengukur biaya dan penggunaan plugin di setiap mesin dari salah satu sumber ini:

* **OpenTelemetry events**: Claude Code mengekspor ini ke backend Anda sendiri setelah Anda [configure an exporter](/docs/id/monitoring-usage). Lihat [OpenTelemetry events for plugin installs and use](#pick-the-opentelemetry-event-for-each-question).
* **Analytics API**: dilayani dari catatan Anthropic, tanpa exporter yang diperlukan. Lihat [Query the Analytics API](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  OpenTelemetry events for plugin installs and use
</h3>

Peristiwa OpenTelemetry dan atribut ini menjawab setiap pertanyaan plugin dari backend Anda:

| Pertanyaan                                                   | OpenTelemetry event atau atribut                                                                                                                  |
| :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| Plugin mana yang dipasang, dan dari mana                     | [`claude_code.plugin_installed`](/docs/id/monitoring-usage#plugin-installed-event), satu per pemasangan                                                |
| Plugin mana yang aktif dalam berapa banyak sesi              | [`claude_code.plugin_loaded`](/docs/id/monitoring-usage#plugin-loaded-event), satu per plugin yang diaktifkan pada awal sesi                           |
| Skill mana yang diaktifkan, dan plugin mana yang memilikinya | [`claude_code.skill_activated`](/docs/id/monitoring-usage#skill-activated-event), dengan `plugin.name` dan `marketplace.name` untuk plugin skills      |
| Apa yang dilaporkan hooks plugin                             | [`claude_code.hook_plugin_metrics`](/docs/id/monitoring-usage#hook-plugin-metrics-event), dipancarkan hanya untuk hooks di plugin marketplace resmi    |
| Apa yang dikerjakan plugin dalam pengeluaran API             | `plugin.name` dan `marketplace.name` pada [cost counter](/docs/id/monitoring-usage#cost-counter), diatur ketika skill atau subagent aktif milik plugin |

<h3 id="redacted-plugin-names-in-your-backend">
  Redacted plugin names in your backend
</h3>

Plugin dari marketplace resmi melaporkan nama plugin dan nama marketplace mereka ke backend Anda secara verbatim. Setiap plugin lainnya namanya diredaksi atau dihilangkan secara default, termasuk plugin dari marketplace organisasi Anda sendiri. [Trust tier](/docs/id/plugins/security#find-plugins-in-telemetry) plugin menentukan mana.

Untuk mendapatkan nama asli pada beberapa peristiwa, atur variabel lingkungan [`OTEL_LOG_TOOL_DETAILS`](/docs/id/monitoring-usage#common-configuration-variables) ke `1` pada mesin yang mengekspor telemetri, misalnya di blok `env` dari [managed settings](/docs/id/monitoring-usage#administrator-configuration) yang sama yang mengonfigurasi exporter:

| Event                                 | Default                                                                                                      | Dengan `OTEL_LOG_TOOL_DETAILS=1`                       |
| :------------------------------------ | :----------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` dan `marketplace.name` adalah string literal `third-party`                                     | Nama asli                                              |
| `plugin_installed`, `skill_activated` | `plugin.name` dan `marketplace.name` dihilangkan; pada `skill_activated`, `skill.name` adalah `custom_skill` | Nama asli                                              |
| Cost counter                          | `plugin.name` adalah `third-party`; `marketplace.name` tidak ada                                             | `plugin.name` asli; `marketplace.name` masih tidak ada |

Pada `plugin_loaded`, `plugin_id_hash` masih mengidentifikasi setiap plugin secara default, jadi Anda dapat menghitung plugin pihak ketiga yang berbeda.

<h3 id="query-the-analytics-api">
  Query the Analytics API
</h3>

Pada paket Enterprise, Analytics API menjawab "plugin mana yang dipasang dan dijalankan organisasi saya" dari catatan Anthropic, tanpa exporter yang diperlukan. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) mengembalikan per-plugin, per-hari install dan invocation counts di seluruh Claude Code dan Cowork, yang dapat Anda kelompokkan berdasarkan pengguna, RBAC group, atau produk.

Aktivitas plugin yang mencapai Anthropic tanpa nama plugin muncul dalam satu baris agregat `third-party`. [Find plugins in telemetry](/docs/id/plugins/security#find-plugins-in-telemetry) mengatakan plugin mana yang dilaporkan Claude Code berdasarkan nama.

Autentikasi permintaan dengan API key yang memiliki scope `read:analytics`, yang dibuat Primary Owner seperti dijelaskan di bawah [Access data programmatically](/docs/id/analytics#access-data-programmatically).

Lihat [endpoint reference](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) untuk parameter dan response fields.

<h2 id="next-steps">
  Next steps
</h2>

* [Test plugins with evals](/docs/id/plugin-evals): ukur seberapa andal plugin mengarahkan Claude, bukan hanya apa yang dikerjakan
* [Lower the always-on figure](#lower-the-always-on-figure): apa yang harus diubah dalam plugin untuk mengurangi biaya per-turn-nya
* [Plugin security and trust](/docs/id/plugins/security#find-plugins-in-telemetry): field telemetri mana yang membawa nama plugin dan kapan diredaksi
* [Monitoring usage](/docs/id/monitoring-usage): referensi peristiwa OpenTelemetry lengkap
