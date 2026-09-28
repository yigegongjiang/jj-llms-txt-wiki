> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gambaran Umum Agent SDK

> Bangun agen AI produksi dengan Claude Code sebagai perpustakaan

Agen adalah aplikasi yang menyelesaikan tugas dengan merencanakan langkah-langkahnya sendiri dan memanggil alat yang membaca file, menjalankan perintah, atau mengedit kode. Agent SDK memberi Anda alat yang sama, [agent loop](/docs/id/agent-sdk/agent-loop), dan manajemen konteks yang mendukung Claude Code, dapat diprogram dalam Python dan TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Bandingkan Agent SDK dengan alat Claude lainnya
</h2>

Agent SDK, CLI, Client SDK, dan Managed Agents berbeda dalam hal siapa yang menjalankan agen, apa yang sudah tertanam, dan bagaimana Anda mengaksesnya. Temukan baris yang sesuai dengan cara Anda ingin membangun dan menjalankannya.

| Anda ingin                                                                                                         | Gunakan                                                                           | Apa yang Anda dapatkan                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Menyematkan agen Claude Code dalam aplikasi Python atau TypeScript Anda sendiri, dalam proses yang Anda operasikan | **Agent SDK**                                                                     | Sebuah library yang menjalankan biner Claude Code, dengan [kemampuan](#capabilities) Claude Code, seperti alat bawaan, izin, sesi, dan hooks.                                                                                                                                                                                                                                                                   |
| Melakukan pengembangan interaktif atau menjalankan tugas sekali jadi dari terminal                                 | [**Claude Code CLI**](/docs/id/overview)                                               | Antarmuka terminal, dibangun untuk penggunaan interaktif sehari-hari.                                                                                                                                                                                                                                                                                                                                           |
| Memanggil API Claude secara langsung dari kode Anda sendiri                                                        | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Akses langsung ke API Claude dari salah satu bahasa Client SDK. Anda menulis loop alat sendiri, atau biarkan [tool runner](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) beta Client SDK menjalankannya.                                                                                                                                                                           |
| Memiliki Anthropic menjalankan agen, dikonfigurasi melalui API Claude                                              | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | Harness agen yang dihosting yang menjalankan loop agen, dengan sesi dalam sandbox cloud yang dikelola Anthropic atau [sandbox yang dihosting sendiri](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) pada infrastruktur Anda sendiri. Gunakan dari [SDK untuk bahasa Anda](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), CLI `ant`, atau REST API. |

Untuk menjalankan loop agen yang sama dari bahasa selain Python atau TypeScript, [jalankan CLI sebagai subprocess](/docs/id/headless) dengan flag `-p` dan `--output-format json`.

<h2 id="capabilities">
  Kemampuan
</h2>

Kemampuan Claude Code ini tersedia di SDK:

| Kemampuan                    | Apa yang dilakukannya                                                                            | Pelajari lebih lanjut                                                                                                                                                                                         |
| ---------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Alat bawaan                  | Baca, tulis, edit file, jalankan perintah, dan cari web                                          | [Referensi alat](/docs/id/tools-reference)                                                                                                                                                                         |
| Hooks                        | Jalankan kode khusus pada titik-titik kunci dalam siklus hidup agen                              | [Hooks](/docs/id/agent-sdk/hooks)                                                                                                                                                                                  |
| Subagents                    | Spawn agen khusus untuk menangani subtask yang terfokus                                          | [Subagents](/docs/id/agent-sdk/subagents)                                                                                                                                                                          |
| MCP                          | Terhubung ke alat eksternal dan sumber data melalui Model Context Protocol                       | [MCP](/docs/id/agent-sdk/mcp)                                                                                                                                                                                      |
| Izin                         | Kontrol alat mana yang berjalan secara otomatis, mana yang memerlukan persetujuan                | [Izin](/docs/id/agent-sdk/permissions)                                                                                                                                                                             |
| Sesi                         | Pertahankan konteks di seluruh pertukaran, lanjutkan atau fork nanti                             | [Sesi](/docs/id/agent-sdk/sessions)                                                                                                                                                                                |
| Skills, commands, dan memory | Muat secara otomatis dari `.claude/` proyek Anda dan dari `~/.claude/`, sama seperti Claude Code | [Skills](/docs/id/agent-sdk/skills), [Commands](/docs/id/agent-sdk/skills#commands-in-agent-sdk-sessions), [Memory](/docs/id/agent-sdk/modifying-system-prompts), [Pemuatan konfigurasi](/docs/id/agent-sdk/claude-code-features) |
| Plugins                      | Paket skills, agen, hooks, dan server MCP, dan muat mereka berdasarkan jalur lokal               | [Plugins](/docs/id/agent-sdk/plugins)                                                                                                                                                                              |

<h2 id="get-started">
  Mulai
</h2>

Ikuti [Quickstart](/docs/id/agent-sdk/quickstart) untuk memasang SDK, mengatur kunci API Anda, dan membangun agen pertama Anda, yang menemukan dan memperbaiki bug dalam kode yang ada.

<Note>
  Kecuali telah disetujui sebelumnya, Anthropic tidak mengizinkan pengembang pihak ketiga untuk menawarkan login claude.ai atau batas laju untuk produk mereka, termasuk agen yang dibangun di Agent SDK Claude. Gunakan metode autentikasi kunci API yang dijelaskan dalam [Quickstart](/docs/id/agent-sdk/quickstart) sebagai gantinya.
</Note>

<h2 id="changelog">
  Changelog
</h2>

Lihat changelog lengkap untuk pembaruan SDK, perbaikan bug, dan fitur baru:

* **TypeScript SDK**: [lihat CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [lihat CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Melaporkan bug
</h2>

Jika Anda mengalami bug atau masalah dengan Agent SDK:

* **TypeScript SDK**: [laporkan masalah di GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [laporkan masalah di GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Pedoman branding
</h2>

Untuk mitra yang mengintegrasikan Claude Agent SDK, penggunaan branding Claude bersifat opsional. Saat mereferensikan Claude dalam produk Anda:

**Diizinkan:**

* "Claude Agent", lebih disukai untuk menu dropdown
* "Claude", ketika sudah dalam menu berlabel "Agents"
* "\{YourAgentName} Powered by Claude", jika Anda memiliki nama agen yang ada

**Tidak diizinkan:**

* "Claude Code" atau "Claude Code Agent"
* Elemen visual atau ASCII art bermerek Claude Code yang meniru Claude Code

Produk Anda harus mempertahankan branding sendiri dan tidak boleh terlihat seperti Claude Code atau produk Anthropic apa pun. Untuk pertanyaan tentang kepatuhan branding, hubungi [tim penjualan](https://www.anthropic.com/contact-sales) Anthropic.

<h2 id="license-and-terms">
  Lisensi dan persyaratan
</h2>

Penggunaan Claude Agent SDK diatur oleh [Persyaratan Layanan Komersial Anthropic](https://www.anthropic.com/legal/commercial-terms), termasuk ketika Anda menggunakannya untuk memberdayakan produk dan layanan yang Anda buat tersedia untuk pelanggan dan pengguna akhir Anda sendiri, kecuali sejauh komponen atau dependensi tertentu dicakup oleh lisensi berbeda seperti yang ditunjukkan dalam file LICENSE komponen tersebut.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Sumber daya ini mencakup detail teknis yang lebih mendalam dan proyek contoh untuk membangun dengan Agent SDK.

* [Panduan Cepat](/docs/id/agent-sdk/quickstart): bangun agen pertama Anda yang menemukan dan memperbaiki bug
* [Panduan migrasi](/docs/id/agent-sdk/migration-guide): migrasi dari paket Claude Code SDK ke Agent SDK
* [Loop agen](/docs/id/agent-sdk/agent-loop): bagaimana Claude merencanakan, memanggil alat, dan memutuskan kapan tugas selesai
* [Agen contoh](https://github.com/anthropics/claude-agent-sdk-demos): aplikasi demo untuk pengembangan lokal
* [TypeScript SDK](/docs/id/agent-sdk/typescript): referensi API TypeScript lengkap dan contoh
* [Python SDK](/docs/id/agent-sdk/python): referensi API Python lengkap dan contoh
* [Desain harness agen](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): bagaimana tim Claude Code menggunakan alur kerja dinamis untuk mengorkestrasi banyak subagen sekaligus
