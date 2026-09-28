> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Keamanan dan kepercayaan plugin

> Tentukan apakah Anda mempercayai plugin sebelum menginstalnya, dari apa yang dapat dilakukan plugin di mesin Anda hingga cara meninjau dan menghapusnya.

Plugin Claude Code yang Anda instal dapat menjalankan kode arbitrer di mesin Anda dengan hak istimewa pengguna Anda.

Anda menginstal plugin dari marketplace, yang merupakan katalog yang Claude Code ambil darinya. Beberapa nama marketplace [dicadangkan untuk marketplace Anthropic sendiri](#marketplace-tiers), dan setiap marketplace lainnya adalah pihak ketiga. Nama marketplace memberi tahu Anda siapa yang menerbitkan katalog, bukan apa yang dilakukan setiap plugin di dalamnya, jadi [tinjau plugin sebelum Anda menginstalnya](#review-a-plugin-before-you-install) dari marketplace mana pun asalnya.

Baca halaman ini jika Anda memutuskan apakah akan menginstal plugin, atau jika Anda meninjau alat sebelum tim Anda dapat menggunakannya.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Model keamanan Claude Code sendiri**: lihat [Security](/docs/id/security)
  * **Membatasi atau memerlukan plugin untuk organisasi**: lihat [Manage plugins for your organization](/docs/id/plugins/org)
  * **Plugin `security-guidance` atau `claude-security`**: halaman ini bukan tentang mereka. Lihat [`security-guidance`](/docs/id/security-guidance) dan [`claude-security`](/docs/id/claude-security)
</Note>

Mulai dengan [apa yang dapat dilakukan plugin](#understand-what-a-plugin-can-do) dan [marketplace mana yang merupakan Anthropic](#marketplace-tiers), kemudian [tinjau plugin sebelum Anda menginstalnya](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Pahami apa yang dapat dilakukan plugin
</h2>

Plugin dapat membawa konten yang menjalankan kode di mesin Anda dengan hak istimewa pengguna Anda dan konten yang memasuki konteks Claude sebagai instruksi, jadi [tinjau plugin sebelum Anda menginstalnya](#review-a-plugin-before-you-install). Berikut adalah apa yang dapat dilakukan plugin yang diinstal:

* **Hooks**: [hooks](/docs/id/hooks) plugin berjalan sebagai perintah shell pada titik-titik dalam siklus hidup Claude Code, seperti sebelum atau sesudah panggilan alat.
* **Server MCP dan LSP**: Claude Code terhubung ke [server MCP](/docs/id/mcp) yang dideklarasikan plugin yang diaktifkan dan memberikan Claude alat mereka. Server MCP stdio berjalan sebagai proses yang dimulai Claude Code di mesin Anda. Claude Code juga memulai server bahasa yang dideklarasikan plugin.
* **Direktori `bin/`**: Claude Code menambahkan direktori `bin/` setiap plugin yang diaktifkan ke `PATH` shell alat Bash, sehingga perintah Bash Claude dapat menjalankan executable apa pun di sana.
* **Skills, commands, dan agents**: ini memasuki konteks Claude sebagai instruksi, sehingga mempengaruhi apa yang dilakukan Claude dengan alat yang sudah dimilikinya.
* **Updates**: ketika auto-update aktif untuk marketplace tempat Anda menginstal plugin, Claude Code memperbarui plugin itu di latar belakang, sehingga file yang Anda tinjau dapat berubah di disk. [When auto-update runs](/docs/id/plugins/loading#when-auto-update-runs) memiliki waktu. Untuk mengaktifkan atau menonaktifkan auto-update per marketplace, lihat [Keep plugins updated](/docs/id/plugins/install#keep-plugins-updated).

[Aturan izin](/docs/id/permissions) Claude Code dan [sandbox](/docs/id/sandboxing) mencakup panggilan alat yang dibuat Claude, bukan kode yang dijalankan plugin sendiri:

* **Hooks dan proses server**: command hooks menjalankan perintah shell dengan izin pengguna penuh Anda. Claude Code menjalankan hooks dan server MCP di luar sandbox.
* **Panggilan alat Claude**: panggilan ke salah satu alat MCP plugin, dan perintah Bash yang menjalankan executable dari `bin/` plugin, adalah panggilan alat, sehingga aturan izin Anda berlaku untuk mereka.

Menginstal plugin juga mengaktifkannya, kecuali manifest atau entri marketplace-nya menetapkan [`defaultEnabled: false`](/docs/id/plugins/install#choose-an-install-scope) dan Anda belum mengaktifkannya sendiri.

Untuk menghapus plugin yang tidak lagi Anda percayai, lihat [Remove a plugin you no longer trust](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identifikasi marketplace Anthropic berdasarkan nama
</h2>

Nama marketplace menempatkannya dalam salah satu dari tiga tingkat: official, community, atau third-party. Claude Code menerima nama official dan community hanya untuk marketplace yang bersumber dari repositori `github.com/anthropics/`, sehingga marketplace pihak ketiga tidak dapat menyajikan dirinya sebagai marketplace Anthropic. Marketplace yang diterbitkan rekan kerja atau organisasi Anda adalah pihak ketiga.

Tabel mencantumkan nama mana yang termasuk dalam setiap tingkat:

| Tingkat     | Marketplace mana                                                                            |
| :---------- | :------------------------------------------------------------------------------------------ |
| Official    | [Nama marketplace official](#official-marketplace-names), seperti `claude-plugins-official` |
| Community   | `claude-community`, `claude-plugins-community`, dan `healthcare`                            |
| Third-party | Setiap marketplace lainnya                                                                  |

Ketika katalog `claude-community` menyematkan plugin ke SHA commit, yang dilakukannya untuk hampir setiap entri, Claude Code menolak untuk menginstal commit yang berbeda.

<h3 id="official-marketplace-names">
  Nama marketplace official
</h3>

Nama marketplace ini membentuk tingkat official:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Untuk cara marketplace official, community, dan demo berbeda dan di mana untuk menjelajahi apa yang masing-masing daftarkan, lihat [Anthropic's marketplaces](/docs/id/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Tinjau plugin sebelum Anda menginstal
</h2>

Sebelum Anda menginstal plugin, lihat apa yang ditambahkannya dan dari mana asalnya.

<Steps>
  <Step title="Periksa sumber marketplace">
    Di shell Anda, jalankan `claude plugin marketplace list` untuk mencetak sumber setiap marketplace ditambahkan dari, seperti repositori GitHub atau direktori.
  </Step>

  <Step title="Baca panel detail">
    Dalam sesi Claude Code, jalankan `/plugin` dan pilih plugin. Panel detail menunjukkan bagian **Will install** yang mencantumkan commands, agents, skills, hooks, dan server MCP dan LSP plugin. Untuk plugin yang Anthropic tidak memiliki data komponen yang dipublikasikan, bagian menunjukkan apa yang dideklarasikan entri marketplace, atau catatan: `Components will be discovered at installation` untuk plugin yang disimpan di dalam marketplace, atau `Component summary not available for remote plugin` untuk yang diambil dari tempat lain.
  </Step>

  <Step title="Baca sumber plugin">
    Di panel detail, pilih **Open homepage** atau **View on GitHub** di bawah opsi install. Jika panel tidak menawarkan keduanya, buka repositori marketplace yang Anda temukan di langkah pertama. Temukan direktori plugin di sana. Bagian **Will install** menunjukkan bahwa hook ada tetapi bukan apa yang dijalankannya, jadi baca file-file ini di direktori plugin:

    * **`hooks/hooks.json`**: perintah yang dijalankan setiap hook
    * **`.mcp.json`**: perintah atau URL setiap server
    * **`bin/`**: setiap file dalam direktori
  </Step>

  <Step title="Daftar apa yang berisi plugin">
    Clone repositori yang menyimpan direktori plugin, kemudian jalankan `claude --plugin-dir <plugin directory> plugin details <plugin name>` di shell Anda untuk melihat apa yang ditemukan Claude Code di dalamnya. Perintah membaca file plugin tanpa memulai sesi dan mencetak `Component inventory` yang mencantumkan skills dan commands plugin, agents, hooks dengan event setiap hook, dan server MCP dan LSP.
  </Step>
</Steps>

Setelah Anda menginstal plugin, jalankan `claude plugin details <plugin name>` di shell Anda untuk mencetak `Component inventory` yang sama untuk salinan yang diinstal di bawah `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Hapus plugin yang tidak lagi Anda percayai
</h3>

Di shell Anda, jalankan [`claude plugin uninstall <plugin>`](/docs/id/plugins/cli-reference#plugin-uninstall) dengan `--scope` tempat Anda menginstalnya. Kemudian periksa apa yang dihapus uninstall dan apa yang ditinggalkannya:

* **Data persisten**: ketika itu adalah scope terakhir plugin diinstal, uninstalling juga menghapus direktori data persisten plugin, kecuali Anda melewatkan `--keep-data`.
* **File cache**: file plugin tetap di disk di bawah `~/.claude/plugins/cache/` selama 14 hari sebelum [background sweep menghapusnya](/docs/id/plugins/loading#cleanup-of-previous-versions). Setelah Anda menghapus plugin terakhir Anda, direktori yatim piatu tetap sampai Anda menginstal yang lain. Untuk menghapus file sekarang, hapus direktori plugin di bawah `~/.claude/plugins/cache/<marketplace>/<plugin>/` sendiri.
* **Marketplace**: jika Anda juga tidak mempercayai pemilik marketplace, [hapus marketplace](/docs/id/plugins/install#manage-marketplaces) juga, yang menghapus setiap plugin yang Anda instal darinya.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Kenali ketika Claude Code menolak atau memperingatkan
</h2>

Panel detail yang Anda buka dari tab **Discover** atau **Marketplaces** di `/plugin` menunjukkan peringatan kepercayaan yang sama untuk setiap plugin. Claude Code menolak alih-alih memperingatkan dalam kasus-kasus seperti yang ada di bawah [Untrusted marketplace sources and failed integrity checks](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Peringatan kepercayaan sebelum Anda menginstal
</h3>

Peringatan membaca sama apa pun marketplace plugin berasal dari:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Jika organisasi Anda menetapkan `pluginTrustMessage` dalam [managed settings](/docs/id/plugins/org), Claude Code menambahkan teks itu ke peringatan.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Sumber marketplace yang tidak dipercaya dan pemeriksaan integritas yang gagal
</h3>

Claude Code menolak untuk memuat marketplace atau menginstal plugin dalam kasus-kasus ini, masing-masing dengan pesan kesalahannya sendiri:

* **Sumber marketplace yang tidak dipercaya**: ketika marketplace menggunakan nama official atau community tetapi sumbernya berada di luar `github.com/anthropics/`, Claude Code berhenti memuat marketplace dan plugin yang Anda instal darinya. Kesalahannya adalah [Marketplace is registered from an untrusted source](/docs/id/errors#marketplace-is-registered-from-an-untrusted-source).
* **Integritas archive**: ketika entri marketplace menyematkan [`archive` source](/docs/id/plugins/marketplace-reference#archive-plugin-source) ke digest `sha256` dan digest file yang diunduh tidak cocok, Claude Code menolak install. Kesalahannya adalah [Plugin archive integrity check failed](/docs/id/errors#plugin-archive-integrity-check-failed).

Pin `sha256` terpisah dari pin SHA commit katalog community, yang memilih commit git untuk checkout.

<h2 id="enforce-plugin-controls-for-your-organization">
  Terapkan kontrol plugin untuk organisasi Anda
</h2>

Dengan [managed settings](/docs/id/plugins/org), administrator dapat menerapkan kontrol plugin ini:

* Allowlist atau blocklist sumber marketplace
* Force-enable plugins
* Matikan flag `--plugin-dir` dan `--plugin-url` dan variabel `CLAUDE_CODE_PLUGIN_DIRS`
* Batasi hooks ke yang dari managed settings dan force-enabled plugins
* Hentikan plugins dari akun claude.ai anggota dari loading di Claude Code, dengan [`syncClaudeAiPlugins`](/docs/id/plugins/org#control-matrix)

[Control matrix](/docs/id/plugins/org#control-matrix) mengatakan apa yang dilakukan setiap kunci dan tidak mencakup.

<h2 id="find-plugins-in-telemetry">
  Temukan plugins dalam telemetri
</h2>

Jika organisasi Anda mengekspor [OpenTelemetry events](/docs/id/monitoring-usage) Claude Code ke backend-nya sendiri, [marketplace tiers](#marketplace-tiers) memutuskan nama plugin mana yang muncul di sana:

* **[Plugin loaded event](/docs/id/monitoring-usage#plugin-loaded-event)**: event melaporkan nama plugin dan marketplace tingkat official seperti adanya. Untuk tingkat community dan third-party, `plugin.name` dan `marketplace.name` adalah string literal `third-party` kecuali Anda menetapkan `OTEL_LOG_TOOL_DETAILS=1`.
* **Plugin scope**: `plugin.scope` event loaded masih melaporkan dari mana plugin berasal, seperti `org` untuk plugin yang diaktifkan managed settings Anda atau `user-local` untuk plugin third-party lainnya. [Plugin loaded event](/docs/id/monitoring-usage#plugin-loaded-event) mencantumkan setiap nilai.
* **[Plugin installed event](/docs/id/monitoring-usage#plugin-installed-event)**: kecuali Anda menetapkan `OTEL_LOG_TOOL_DETAILS=1`, event menghilangkan field nama untuk plugin non-official alih-alih melaporkan `third-party`.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code melaporkan plugins dari tingkat official dan community berdasarkan nama dan melaporkan setiap plugin lainnya sebagai `third-party`.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Manage plugins for your organization](/docs/id/plugins/org): batasi marketplace mana yang dapat diinstal pengguna dan perlukan yang Anda percayai
* [Install and manage plugins](/docs/id/plugins/install): tinjau panel detail plugin sebelum Anda memilih scope
* [Anthropic's marketplaces](/docs/id/plugins/anthropic-marketplaces): nama marketplace mana yang merupakan Anthropic
* [Security](/docs/id/security): model keamanan Claude Code sendiri
