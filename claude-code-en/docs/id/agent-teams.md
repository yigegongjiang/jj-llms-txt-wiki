> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Koordinasikan tim Claude Code sessions

> Koordinasikan beberapa instance Claude Code yang bekerja bersama sebagai tim, dengan tugas bersama, pesan antar-agent, dan manajemen terpusat.

<Warning>
  Tim agent bersifat eksperimental dan dinonaktifkan secara default. Aktifkan dengan menambahkan `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` ke [settings.json](/docs/id/settings) atau environment Anda. Tanpa variabel tersebut, tidak ada tim yang diatur pada awal session, tidak ada direktori tim yang ditulis, dan Claude tidak menspawn atau mengusulkan rekan tim. Tim agent memiliki [keterbatasan yang diketahui](#limitations) seputar session resumption, koordinasi tugas, dan perilaku shutdown.
</Warning>

Tim agent memungkinkan Anda mengoordinasikan beberapa instance Claude Code yang bekerja bersama. Satu session bertindak sebagai team lead, mengoordinasikan pekerjaan, menugaskan tugas, dan mensintesis hasil. Rekan tim bekerja secara independen, masing-masing dalam context window-nya sendiri, dan berkomunikasi langsung satu sama lain.

Tidak seperti [subagents](/docs/id/sub-agents), yang berjalan dalam satu session dan hanya dapat melaporkan kembali ke agent utama, Anda juga dapat berinteraksi dengan rekan tim individual secara langsung tanpa melalui lead.

<Note>
  Halaman ini menjelaskan tim agent per v2.1.178. Dengan `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` diatur, menspawn rekan tim tidak lagi memerlukan langkah setup, dan cleanup terjadi secara otomatis ketika session keluar. Sebelum v2.1.178, Anda meminta Claude untuk membuat dan memberi nama tim terlebih dahulu, dan Claude menggunakan tools `TeamCreate` dan `TeamDelete` untuk mengaturnya dan menghapusnya. Kedua tools tidak lagi ada. Input `team_name` pada tool Agent diterima tetapi diabaikan, dan field `team_name` dalam [hook payloads](/docs/id/hooks#taskcreated) `TaskCreated`, `TaskCompleted`, dan `TeammateIdle` membawa nama yang diturunkan dari session dan sudah usang.
</Note>

<h2 id="when-to-use-agent-teams">
  Kapan menggunakan tim agent
</h2>

Tim agent paling efektif untuk tugas di mana eksplorasi paralel menambah nilai nyata. Lihat [contoh use case](#use-case-examples) untuk skenario lengkap. Use case terkuat adalah:

* **Penelitian dan review**: beberapa rekan tim dapat menyelidiki aspek berbeda dari masalah secara bersamaan, kemudian berbagi dan menantang temuan satu sama lain
* **Modul atau fitur baru**: rekan tim dapat masing-masing memiliki bagian terpisah tanpa saling mengganggu
* **Debugging dengan hipotesis bersaing**: rekan tim menguji teori berbeda secara paralel dan berkumpul pada jawaban lebih cepat
* **Koordinasi lintas-layer**: perubahan yang mencakup frontend, backend, dan tests, masing-masing dimiliki oleh rekan tim berbeda

Tim agent menambah overhead koordinasi dan menggunakan token secara signifikan lebih banyak daripada satu session. Mereka bekerja paling baik ketika rekan tim dapat beroperasi secara independen. Untuk tugas sekuensial, edit file yang sama, atau pekerjaan dengan banyak dependensi, satu session atau [subagents](/docs/id/sub-agents) lebih efektif.

<h3 id="compare-with-subagents">
  Bandingkan dengan subagents
</h3>

Baik tim agent maupun [subagents](/docs/id/sub-agents) memungkinkan Anda memparalelkan pekerjaan, tetapi mereka beroperasi berbeda. Pilih berdasarkan apakah pekerja Anda perlu berkomunikasi satu sama lain:

<Frame caption="Subagents hanya melaporkan hasil kembali ke agent utama dan tidak pernah berbicara satu sama lain. Dalam tim agent, rekan tim berbagi daftar tugas, mengklaim pekerjaan, dan berkomunikasi langsung satu sama lain.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagram membandingkan arsitektur subagent dan tim agent. Subagents dihasilkan oleh agent utama, melakukan pekerjaan, dan melaporkan hasil kembali. Tim agent berkoordinasi melalui daftar tugas bersama, dengan rekan tim berkomunikasi langsung satu sama lain." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagram membandingkan arsitektur subagent dan tim agent. Subagents dihasilkan oleh agent utama, melakukan pekerjaan, dan melaporkan hasil kembali. Tim agent berkoordinasi melalui daftar tugas bersama, dengan rekan tim berkomunikasi langsung satu sama lain." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                   | Subagents                                              | Tim agent                                                      |
| :---------------- | :----------------------------------------------------- | :------------------------------------------------------------- |
| **Context**       | Context window sendiri; hasil kembali ke pemanggil     | Context window sendiri; sepenuhnya independen                  |
| **Komunikasi**    | Melaporkan hasil kembali ke agent utama saja           | Rekan tim saling mengirim pesan secara langsung                |
| **Koordinasi**    | Agent utama mengelola semua pekerjaan                  | Daftar tugas bersama dengan self-coordination                  |
| **Terbaik untuk** | Tugas terfokus di mana hanya hasil yang penting        | Pekerjaan kompleks yang memerlukan diskusi dan kolaborasi      |
| **Biaya token**   | Lebih rendah: hasil diringkas kembali ke context utama | Lebih tinggi: setiap rekan tim adalah instance Claude terpisah |

Gunakan subagents ketika Anda membutuhkan pekerja cepat dan terfokus yang melaporkan kembali. Gunakan tim agent ketika rekan tim perlu berbagi temuan, menantang satu sama lain, dan berkoordinasi sendiri.

<h2 id="enable-agent-teams">
  Aktifkan tim agent
</h2>

Tim agent dinonaktifkan secara default. Aktifkan dengan mengatur variabel environment `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` ke `1`, baik di environment shell Anda atau melalui [settings.json](/docs/id/settings):

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

<h2 id="start-your-first-agent-team">
  Mulai tim agent pertama Anda
</h2>

Setelah mengaktifkan tim agent, jelaskan tugas dan rekan tim yang Anda inginkan dalam bahasa alami. Claude menelurkan mereka dan mengoordinasikan pekerjaan berdasarkan prompt Anda.

Contoh ini bekerja dengan baik karena tiga peran independen dan dapat mengeksplorasi masalah tanpa menunggu satu sama lain:

```text theme={null}
Saya merancang alat CLI yang membantu developer melacak komentar TODO di seluruh
codebase mereka. Buat tiga rekan tim untuk mengeksplorasi ini dari sudut berbeda:
satu pada UX, satu pada arsitektur teknis, satu memainkan devil's advocate.
```

Dari sana, Claude mengisi [daftar tugas bersama](/docs/id/interactive-mode#task-list), menelurkan rekan tim untuk setiap perspektif, membuat mereka mengeksplorasi masalah, dan mensintesis temuan ketika selesai.

Panel lead mencantumkan rekan tim di panel agent di bawah input prompt. Dari panel:

* **Panah atas dan bawah**: pilih rekan tim
* **Enter**: buka transkrip rekan tim yang dipilih dan kirim pesan langsung kepadanya
* **Escape**: hentikan giliran saat ini rekan tim yang dipilih

Mulai dari v2.1.199, baris rekan tim yang idle tetap berada di panel sementara rekan tim atau subagent mana pun masih bekerja, sehingga Anda dapat memilihnya untuk meninjau transkrip atau memberikan pekerjaan lebih lanjut. Setelah setiap agent di panel idle, baris idle bersembunyi setelah 30 detik dan muncul kembali pada giliran berikutnya rekan tim; rekan tim tetap berjalan dan dapat dialamatkan saat tersembunyi. Di v2.1.181 hingga v2.1.198, baris idle bersembunyi 30 detik setelah giliran sendirinya berakhir, bahkan sementara rekan tim lain masih bekerja; baris idle tidak disembunyikan di versi sebelum v2.1.181.

Ketika lebih dari tiga rekan tim idle sekaligus, baris di luar tiga pertama runtuh menjadi satu baris yang menghitung rekan tim yang runtuh, seperti `2 idle agents` ketika lima idle. Pilih dan tekan Enter untuk memperluas baris yang runtuh, atau tekan Esc untuk meruntuhkannya lagi. Rekan tim yang bekerja, rekan tim yang gagal, dan rekan tim yang Anda lihat selalu mempertahankan baris mereka sendiri.

Jika Anda ingin setiap rekan tim di split pane-nya sendiri, lihat [Pilih mode tampilan](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Kontrol tim agent Anda
</h2>

Beri tahu lead apa yang Anda inginkan dalam bahasa alami. Ini menangani koordinasi tim, penugasan tugas, dan delegasi berdasarkan instruksi Anda.

<h3 id="choose-a-display-mode">
  Pilih mode tampilan
</h3>

Tim agent mendukung dua mode tampilan:

* **In-process**: semua rekan tim berjalan di dalam terminal utama Anda. Gunakan tombol panah atas dan bawah di panel agent untuk memilih rekan tim, kemudian tekan Enter untuk melihatnya dan ketik untuk mengirim pesan kepadanya secara langsung. Bekerja di terminal apa pun, tidak ada setup tambahan yang diperlukan.
* **Split panes**: setiap rekan tim mendapat pane-nya sendiri. Anda dapat melihat output semua orang sekaligus dan klik ke dalam pane untuk berinteraksi secara langsung. Memerlukan tmux, atau iTerm2.

<Note>
  `tmux` memiliki keterbatasan yang diketahui pada sistem operasi tertentu dan secara tradisional bekerja paling baik di macOS. Menggunakan `tmux -CC` di iTerm2 adalah entrypoint yang disarankan ke `tmux`.
</Note>

Default adalah `"in-process"`. Sebelum v2.1.179 default adalah `"auto"`, jadi session yang ditingkatkan yang sebelumnya membuka split panes sekarang tetap dalam satu terminal kecuali Anda menetapkan mode secara eksplisit. Atur `"auto"` untuk mengaktifkan split panes ketika Anda sudah berjalan di dalam session tmux, atau ketika terminal Anda adalah iTerm2 dengan CLI `it2` yang terinstal, jatuh kembali ke in-process sebaliknya. Pengaturan `"tmux"` mengaktifkan mode split-pane dan auto-detects apakah akan menggunakan tmux atau iTerm2 berdasarkan terminal Anda.

Mulai dari v2.1.186, atur `"iterm2"` untuk menggunakan iTerm2 native split panes secara eksplisit. Mode ini memerlukan [`it2` CLI](https://github.com/mkusaka/it2) dan menampilkan error dengan perintah install jika `it2` hilang. Prompt setup yang menawarkan untuk menginstal `it2` atau beralih ke tmux muncul di bawah `"auto"` atau `"tmux"` ketika terminal Anda adalah iTerm2 dan tmux tersedia sebagai fallback.

Untuk mengganti default, atur [`teammateMode`](/docs/id/settings#available-settings) di `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Untuk menetapkan mode untuk satu session, teruskan sebagai flag:

```bash theme={null}
claude --teammate-mode auto
```

Mode split-pane memerlukan baik [tmux](https://github.com/tmux/tmux/wiki) atau iTerm2 dengan [`it2` CLI](https://github.com/mkusaka/it2). Untuk menginstal secara manual:

* **tmux**: instal melalui package manager sistem Anda. Lihat [tmux wiki](https://github.com/tmux/tmux/wiki/Installing) untuk instruksi spesifik platform.
* **iTerm2**: instal [`it2` CLI](https://github.com/mkusaka/it2), kemudian aktifkan Python API di **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Tentukan rekan tim dan model
</h3>

Claude memutuskan jumlah rekan tim untuk dihasilkan berdasarkan tugas Anda, atau Anda dapat menentukan dengan tepat apa yang Anda inginkan:

```text theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Rekan tim tidak mewarisi pilihan `/model` lead secara default. Untuk mengubah model yang digunakan ketika prompt tidak menentukan satu, atur **Default teammate model** di `/config`. Pilih **Default (leader's model)** untuk membuat rekan tim mengikuti model saat ini lead.

Rekan tim mewarisi [effort level](/docs/id/model-config#adjust-effort-level) lead. Dalam mode split-pane ini berlaku dari v2.1.186; versi sebelumnya tidak meneruskan effort session lead ke rekan tim split-pane.

<h3 id="require-plan-approval-for-teammates">
  Perlukan persetujuan rencana untuk rekan tim
</h3>

Untuk tugas kompleks atau berisiko, Anda dapat memerlukan rekan tim untuk merencanakan sebelum mengimplementasikan. Rekan tim bekerja dalam mode rencana read-only sampai lead menyetujui pendekatan mereka:

```text theme={null}
Spawn an architect teammate to refactor the authentication module.
Require plan approval before they make any changes.
```

Ketika rekan tim selesai merencanakan, mereka mengirim permintaan persetujuan rencana ke lead. Lead meninjau rencana dan baik menyetujuinya atau menolaknya dengan umpan balik. Jika ditolak, rekan tim tetap dalam mode rencana, merevisi berdasarkan umpan balik, dan mengirimkan kembali. Setelah disetujui, rekan tim keluar dari mode rencana dan mulai implementasi.

Lead membuat keputusan persetujuan secara otonom. Untuk mempengaruhi penilaian lead, berikan kriteria dalam prompt Anda, seperti "hanya setujui rencana yang mencakup test coverage" atau "tolak rencana yang memodifikasi skema database."

<h3 id="talk-to-teammates-directly">
  Berbicara dengan rekan tim secara langsung
</h3>

Setiap rekan tim adalah session Claude Code penuh dan independen. Anda dapat mengirim pesan ke rekan tim mana pun secara langsung untuk memberikan instruksi tambahan, mengajukan pertanyaan lanjutan, atau mengalihkan pendekatan mereka.

* **Mode in-process**: gunakan tombol panah atas dan bawah di panel agent untuk memilih rekan tim, kemudian tekan Enter untuk melihat sessionnya dan ketik untuk mengirim pesan kepadanya. Tekan `x` pada rekan tim yang dipilih untuk menghentikannya. Tekan Ctrl+T untuk toggle daftar tugas.
* **Mode split-pane**: klik ke dalam pane rekan tim untuk berinteraksi dengan session mereka secara langsung. Setiap rekan tim memiliki tampilan penuh dari terminal mereka sendiri.

Saat Anda melihat rekan tim in-process, teks biasa dan [skills](/docs/id/skills) pergi ke rekan tim itu, tetapi perintah built-in masih berjalan dalam session lead.

Model rekan tim dan fast mode diperbaiki ketika mereka spawn, jadi `/model` dan `/fast` hanya mengubah pengaturan lead. Mulai dari v2.1.199, mengetik salah satu perintah saat melihat rekan tim menampilkan pemberitahuan bahwa perubahan berlaku untuk lead; versi sebelumnya menerapkannya ke lead tanpa indikasi. `/effort` masih berlaku untuk giliran rekan tim yang lebih baru, karena rekan tim mengikuti [effort level](/docs/id/model-config#adjust-effort-level) lead.

<h3 id="assign-and-claim-tasks">
  Tetapkan dan klaim tugas
</h3>

Daftar tugas bersama mengoordinasikan pekerjaan di seluruh tim. Lead membuat tugas dan rekan tim mengerjakannya. Tugas memiliki tiga status: pending, in progress, dan completed. Tugas juga dapat bergantung pada tugas lain: tugas pending dengan dependensi yang tidak terselesaikan tidak dapat diklaim sampai dependensi tersebut selesai.

Lead dapat menugaskan tugas secara eksplisit, atau rekan tim dapat self-claim:

* **Lead menugaskan**: beri tahu lead tugas mana yang diberikan kepada rekan tim mana
* **Self-claim**: setelah menyelesaikan tugas, rekan tim mengambil tugas unassigned, unblocked berikutnya sendiri

Klaim tugas menggunakan file locking untuk mencegah race conditions ketika beberapa rekan tim mencoba mengklaim tugas yang sama secara bersamaan.

<h3 id="shut-down-teammates">
  Matikan rekan tim
</h3>

Untuk mengakhiri session rekan tim dengan baik, rujuk dengan nama. Misalnya, dengan rekan tim bernama researcher:

```text theme={null}
Ask the researcher teammate to shut down
```

Lead mengirim permintaan shutdown. Rekan tim dapat menyetujui, keluar dengan baik, atau menolak dengan penjelasan.

Direktori bersama tim dibersihkan secara otomatis ketika session berakhir, jadi tidak ada langkah pembersihan terpisah. Lihat [Architecture](#architecture) untuk direktori mana yang dihapus dan mana yang bertahan untuk session yang dilanjutkan.

<h3 id="enforce-quality-gates-with-hooks">
  Terapkan quality gates dengan hooks
</h3>

Gunakan [hooks](/docs/id/hooks) untuk menerapkan aturan ketika rekan tim menyelesaikan pekerjaan atau tugas dibuat atau diselesaikan:

* [`TeammateIdle`](/docs/id/hooks#teammateidle): berjalan ketika rekan tim akan idle. Keluar dengan kode 2 untuk mengirim umpan balik dan membuat rekan tim tetap bekerja.
* [`TaskCreated`](/docs/id/hooks#taskcreated): berjalan ketika tugas sedang dibuat. Keluar dengan kode 2 untuk mencegah pembuatan dan mengirim umpan balik.
* [`TaskCompleted`](/docs/id/hooks#taskcompleted): berjalan ketika tugas ditandai selesai. Keluar dengan kode 2 untuk mencegah penyelesaian dan mengirim umpan balik.

<h2 id="how-agent-teams-work">
  Bagaimana tim agent bekerja
</h2>

Bagian ini mencakup arsitektur dan mekanik di balik tim agent. Jika Anda ingin mulai menggunakannya, lihat [Kontrol tim agent Anda](#control-your-agent-team) di atas.

<h3 id="how-claude-starts-agent-teams">
  Bagaimana Claude memulai tim agent
</h3>

Tim agent terbentuk ketika rekan tim pertama dihasilkan, dengan sesi utama bertindak sebagai pemimpin. Ada dua cara rekan tim dihasilkan:

* **Anda meminta rekan tim**: berikan Claude tugas yang menguntungkan dari pekerjaan paralel dan secara eksplisit minta rekan tim. Claude menghasilkannya berdasarkan instruksi Anda.
* **Claude mengusulkan rekan tim**: jika Claude menentukan tugas Anda akan menguntungkan dari pekerjaan paralel, mungkin menyarankan untuk menghasilkan rekan tim. Anda mengonfirmasi sebelum melanjutkan.

Dalam kedua kasus, Anda tetap mengendalikan. Claude tidak akan menghasilkan rekan tim tanpa persetujuan Anda.

<h3 id="architecture">
  Arsitektur
</h3>

Tim agent terdiri dari:

| Komponen         | Peran                                                                               |
| :--------------- | :---------------------------------------------------------------------------------- |
| **Team lead**    | Sesi Claude Code utama yang menghasilkan rekan tim dan mengoordinasikan pekerjaan   |
| **Rekan tim**    | Instance Claude Code terpisah yang masing-masing bekerja pada tugas yang ditugaskan |
| **Daftar tugas** | Daftar item pekerjaan bersama yang diklaim dan diselesaikan rekan tim               |
| **Mailbox**      | Sistem pesan untuk komunikasi antar agent                                           |

Lihat [Pilih mode tampilan](#choose-a-display-mode) untuk opsi konfigurasi tampilan. Pesan rekan tim tiba di lead secara otomatis.

Mailbox setiap agent adalah file JSON di `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code memvalidasi setiap entri ketika membaca file mailbox. Entri yang tidak sesuai dengan format pesan dilaporkan sebagai kesalahan dan dihapus dari file; pesan yang valid masih dikirimkan. Sebelum v2.1.207, satu entri mailbox yang salah format menyebabkan kesalahan berulang setiap detik dan memblokir pengiriman untuk mailbox itu sampai Anda menghapus file secara manual.

Sistem mengelola dependensi tugas secara otomatis. Ketika rekan tim menyelesaikan tugas yang tugas lain bergantung padanya, tugas yang diblokir membuka tanpa intervensi manual.

Tim dan tugas disimpan secara lokal dengan nama yang diturunkan dari sesi. Nama adalah `session-` diikuti oleh delapan karakter pertama dari session ID:

* **Konfigurasi tim**: `~/.claude/teams/{team-name}/config.json`
* **Daftar tugas**: `~/.claude/tasks/{team-name}/`

Claude Code menghasilkan keduanya secara otomatis saat startup sesi dan memperbarui mereka saat rekan tim bergabung, idle, atau pergi. Direktori konfigurasi tim dihapus ketika sesi berakhir. Direktori daftar tugas tetap ada secara lokal dan tidak pernah diunggah, jadi sesi yang dilanjutkan menyimpan tugas mereka. Retensi diatur oleh [`cleanupPeriodDays`](/docs/id/settings#available-settings) yang sama yang sudah Anda kontrol untuk transkrip sesi.

Konfigurasi tim menyimpan status runtime seperti session IDs dan tmux pane IDs, jadi jangan mengeditnya dengan tangan atau pre-author: perubahan Anda ditimpa pada update status berikutnya.

Untuk mendefinisikan peran rekan tim yang dapat digunakan kembali, gunakan [subagent definitions](#use-subagent-definitions-for-teammates) sebagai gantinya.

Konfigurasi tim berisi array `members` dengan nama setiap rekan tim, agent ID, dan tipe agent. Rekan tim dapat membaca file ini untuk menemukan anggota tim lainnya.

Tidak ada padanan tingkat proyek dari konfigurasi tim. File seperti `.claude/teams/teams.json` di direktori proyek Anda tidak dikenali sebagai konfigurasi; Claude memperlakukannya sebagai file biasa.

<h3 id="use-subagent-definitions-for-teammates">
  Gunakan subagent definitions untuk rekan tim
</h3>

Ketika menghasilkan rekan tim, Anda dapat mereferensikan tipe [subagent](/docs/id/sub-agents) dari [subagent scope](/docs/id/sub-agents#choose-the-subagent-scope) apa pun: proyek, pengguna, plugin, atau CLI-defined. Ini memungkinkan Anda mendefinisikan peran sekali, seperti security-reviewer atau test-runner, dan menggunakannya kembali baik sebagai subagent yang didelegasikan maupun sebagai rekan tim agent team.

Untuk menggunakan subagent definition, sebutkan berdasarkan nama ketika meminta Claude untuk menghasilkan rekan tim:

```text theme={null}
Hasilkan rekan tim menggunakan tipe agent security-reviewer untuk mengaudit modul auth.
```

Rekan tim menghormati allowlist `tools` definisi itu dan `model`, dan body definisi ditambahkan ke system prompt rekan tim sebagai instruksi tambahan daripada menggantinya. Team coordination tools seperti `SendMessage` dan task management tools selalu tersedia untuk rekan tim bahkan ketika `tools` membatasi tools lain.

<Note>
  Field frontmatter `skills` dan `mcpServers` dalam subagent definition tidak diterapkan ketika definisi itu berjalan sebagai rekan tim. Rekan tim memuat skills dan MCP servers dari pengaturan proyek dan pengguna Anda, sama seperti session reguler.
</Note>

<h3 id="permissions">
  Izin
</h3>

Rekan tim dimulai dengan pengaturan izin lead. Jika lead berjalan dengan `--dangerously-skip-permissions`, semua rekan tim juga demikian. Setelah dihasilkan, Anda dapat mengubah mode rekan tim individual, tetapi Anda tidak dapat mengatur mode per-rekan tim pada waktu spawn.

Ketika satu agent mengirim agent lain pesan melalui `SendMessage`, agent penerima diberitahu bahwa itu berasal dari sesi Claude lain, bukan dari Anda. Rekan tim tidak dapat menyetujui prompt izin atau memberikan persetujuan atas nama Anda, dan rekan tim yang ditolak tindakannya tidak dapat menyampaikannya ke rekan tim lain untuk melewati pemeriksaan. Dalam [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), classifier memperlakukan klaim persetujuan yang disalurkan dari agent lain sebagai input yang tidak dipercaya daripada konfirmasi dari Anda.

Prompt izin rekan tim naik ke sesi lead, jadi setujui mereka di sana sendiri. [Persetujuan rencana](#require-plan-approval-for-teammates) adalah pengecualian yang dirancang: sesi lead memberikan persetujuan rencana rekan tim tanpa prompt terpisah kepada Anda.

<h3 id="context-and-communication">
  Context dan komunikasi
</h3>

Setiap rekan tim memiliki context window-nya sendiri. Ketika dihasilkan, rekan tim memuat konteks proyek yang sama seperti session reguler: CLAUDE.md, MCP servers, dan skills. Mereka juga menerima spawn prompt dari lead. Riwayat percakapan lead tidak terbawa.

**Bagaimana rekan tim berbagi informasi:**

* **Pengiriman pesan otomatis**: ketika rekan tim mengirim pesan, mereka dikirimkan secara otomatis ke penerima. Lead tidak perlu polling untuk update.
* **Notifikasi idle**: ketika rekan tim selesai dan berhenti, mereka secara otomatis memberi tahu lead. Mulai dari v2.1.198, rekan tim yang giliran berakhir pada kesalahan API memberi tahu lead bahwa itu gagal dan menyertakan teks kesalahan, daripada tampak selesai secara normal.
* **Daftar tugas bersama**: semua agent dapat melihat status tugas dan mengklaim pekerjaan yang tersedia.
* **Pesan rekan tim**: kirim pesan ke satu rekan tim spesifik berdasarkan nama. Untuk menjangkau semua orang, kirim satu pesan per penerima.

Lead menugaskan setiap rekan tim nama ketika menghasilkannya, dan rekan tim mana pun dapat mengirim pesan ke yang lain berdasarkan nama itu. Untuk mendapatkan nama yang dapat diprediksi yang dapat Anda referensikan dalam prompt kemudian, beri tahu lead apa yang harus dipanggil setiap rekan tim dalam instruksi spawn Anda.

<h3 id="token-usage">
  Penggunaan token
</h3>

Tim agent menggunakan token secara signifikan lebih banyak daripada satu session. Setiap rekan tim memiliki context window-nya sendiri, dan penggunaan token skala dengan jumlah rekan tim aktif. Untuk penelitian, review, dan pekerjaan fitur baru, token tambahan biasanya berharga. Untuk tugas rutin, satu session lebih cost-effective. Lihat [biaya token tim agent](/docs/id/costs#agent-team-token-costs) untuk panduan penggunaan.

<h2 id="use-case-examples">
  Contoh use case
</h2>

Contoh-contoh ini menunjukkan bagaimana tim agent menangani tugas di mana eksplorasi paralel menambah nilai.

<h3 id="run-a-parallel-code-review">
  Jalankan code review paralel
</h3>

Seorang reviewer tunggal cenderung tertarik pada satu jenis masalah pada satu waktu. Membagi kriteria review menjadi domain independen berarti keamanan, kinerja, dan test coverage semuanya mendapat perhatian menyeluruh secara bersamaan. Prompt menugaskan setiap rekan tim lensa yang berbeda sehingga mereka tidak tumpang tindih:

```text theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Setiap reviewer bekerja dari PR yang sama tetapi menerapkan filter berbeda. Lead mensintesis temuan di ketiga setelah mereka selesai.

<h3 id="investigate-with-competing-hypotheses">
  Investigasi dengan hipotesis bersaing
</h3>

Ketika akar penyebab tidak jelas, satu agent cenderung menemukan satu penjelasan yang masuk akal dan berhenti mencari. Prompt melawan ini dengan membuat rekan tim secara eksplisit adversarial: pekerjaan setiap orang bukan hanya menyelidiki teori mereka sendiri tetapi menantang yang lain.

```text theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

Struktur debat adalah mekanisme kunci di sini. Investigasi sekuensial menderita dari anchoring: setelah satu teori dieksplorasi, investigasi berikutnya bias terhadapnya.

Dengan beberapa investigator independen secara aktif mencoba membantah satu sama lain, teori yang bertahan jauh lebih mungkin menjadi akar penyebab sebenarnya.

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="give-teammates-enough-context">
  Berikan rekan tim konteks yang cukup
</h3>

Rekan tim memuat konteks proyek secara otomatis, termasuk CLAUDE.md, MCP servers, dan skills, tetapi mereka tidak mewarisi riwayat percakapan lead. Lihat [Context dan komunikasi](#context-and-communication) untuk detail. Sertakan detail spesifik tugas dalam spawn prompt:

```text theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  Pilih ukuran tim yang sesuai
</h3>

Tidak ada batas keras pada jumlah rekan tim, tetapi batasan praktis berlaku:

* **Biaya token skala linear**: setiap rekan tim memiliki context window-nya sendiri dan mengkonsumsi token secara independen. Lihat [biaya token tim agent](/docs/id/costs#agent-team-token-costs) untuk detail.
* **Overhead koordinasi meningkat**: lebih banyak rekan tim berarti lebih banyak komunikasi, koordinasi tugas, dan potensi konflik
* **Diminishing returns**: di luar titik tertentu, rekan tim tambahan tidak mempercepat pekerjaan secara proporsional

Mulai dengan 3-5 rekan tim untuk sebagian besar workflow. Ini menyeimbangkan pekerjaan paralel dengan koordinasi yang dapat dikelola. Contoh-contoh dalam panduan ini menggunakan 3-5 rekan tim karena rentang itu bekerja dengan baik di berbagai jenis tugas.

Memiliki 5-6 [tasks](/docs/id/agent-teams#architecture) per rekan tim membuat semua orang produktif tanpa context switching yang berlebihan. Jika Anda memiliki 15 tugas independen, 3 rekan tim adalah titik awal yang baik.

Skala naik hanya ketika pekerjaan benar-benar menguntungkan dari rekan tim bekerja secara bersamaan. Tiga rekan tim terfokus sering mengungguli lima yang tersebar.

<h3 id="size-tasks-appropriately">
  Ukuran tugas dengan tepat
</h3>

* **Terlalu kecil**: overhead koordinasi melebihi manfaat
* **Terlalu besar**: rekan tim bekerja terlalu lama tanpa check-in, meningkatkan risiko usaha yang terbuang
* **Tepat**: unit self-contained yang menghasilkan deliverable yang jelas, seperti fungsi, file test, atau review

<Tip>
  Lead memecah pekerjaan menjadi tugas dan menugaskan mereka ke rekan tim secara otomatis. Jika tidak membuat cukup tugas, minta untuk membagi pekerjaan menjadi potongan yang lebih kecil. Memiliki 5-6 tugas per rekan tim membuat semua orang produktif dan memungkinkan lead untuk menugaskan kembali pekerjaan jika seseorang terjebak.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Tunggu rekan tim selesai
</h3>

Kadang-kadang lead mulai mengimplementasikan tugas sendiri alih-alih menunggu rekan tim. Jika Anda memperhatikan ini:

```text theme={null}
Tunggu rekan tim Anda menyelesaikan tugas mereka sebelum melanjutkan
```

<h3 id="start-with-research-and-review">
  Mulai dengan penelitian dan review
</h3>

Jika Anda baru mengenal tim agent, mulai dengan tugas yang memiliki batas yang jelas dan tidak memerlukan penulisan kode: review PR, penelitian library, atau investigasi bug. Tugas-tugas ini menunjukkan nilai eksplorasi paralel tanpa tantangan koordinasi yang datang dengan implementasi paralel.

<h3 id="avoid-file-conflicts">
  Hindari konflik file
</h3>

Dua rekan tim mengedit file yang sama menyebabkan overwrites. Pecah pekerjaan sehingga setiap rekan tim memiliki set file berbeda.

<h3 id="monitor-and-steer">
  Monitor dan kemudi
</h3>

Periksa kemajuan rekan tim, alihkan pendekatan yang tidak berfungsi, dan sintesis temuan saat tiba. Membiarkan tim berjalan tanpa diawasi terlalu lama meningkatkan risiko usaha yang terbuang.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="teammates-not-appearing">
  Rekan tim tidak muncul
</h3>

Jika rekan tim tidak muncul setelah Anda meminta Claude untuk membuat mereka:

* Dalam mode in-process, rekan tim muncul di panel agen di bawah input prompt. Gunakan tombol panah atas dan bawah untuk memilih satu, kemudian tekan Enter untuk melihatnya.
* Baris rekan tim yang hilang setelah diam telah disembunyikan, bukan dihentikan. Baris idle menyembunyikan 30 detik setelah seluruh panel menjadi idle dan muncul kembali pada giliran rekan tim berikutnya. Ketika lebih dari tiga rekan tim idle, baris surplus mereka runtuh menjadi satu baris `N idle agents` yang Enter perluas. Kirim pesan ke rekan tim berdasarkan nama untuk membawa baris yang disembunyikan kembali.
* Periksa bahwa tugas yang Anda berikan Claude cukup kompleks untuk menjamin rekan tim. Claude memutuskan apakah akan menelurkan mereka berdasarkan tugas.
* Jika Anda secara eksplisit meminta split panes, pastikan tmux diinstal dan tersedia di PATH Anda:
  ```bash theme={null}
  which tmux
  ```
* Untuk iTerm2, verifikasi `it2` CLI diinstal dan Python API diaktifkan di preferensi iTerm2.

<h3 id="too-many-permission-prompts">
  Terlalu banyak permission prompts
</h3>

Permintaan izin rekan tim naik ke lead, yang dapat menciptakan gesekan. Pre-approve operasi umum di [pengaturan izin](/docs/id/permissions) Anda sebelum menelurkan rekan tim untuk mengurangi gangguan.

<h3 id="teammates-stopping-on-errors">
  Rekan tim berhenti pada errors
</h3>

Rekan tim dapat berhenti setelah mengalami errors alih-alih pulih. Periksa output mereka dengan memilih rekan tim di panel agen dan menekan Enter dalam mode in-process, atau dengan mengklik pane dalam mode split, kemudian baik:

* Berikan instruksi tambahan kepada mereka secara langsung
* Hasilkan rekan tim pengganti untuk melanjutkan pekerjaan

Mulai dari v2.1.198, pesan dari lead atau rekan tim lain membangunkan rekan tim in-process yang menunggu untuk mencoba ulang permintaan API yang gagal, sehingga mencoba ulang segera alih-alih menunggu penundaan percobaan ulang penuh.

<h3 id="lead-shuts-down-before-work-is-done">
  Lead shuts down sebelum pekerjaan selesai
</h3>

Lead dapat memutuskan tim selesai sebelum semua tugas benar-benar selesai. Jika ini terjadi, beri tahu untuk terus. Anda juga dapat memberi tahu lead untuk menunggu rekan tim selesai sebelum melanjutkan jika mulai melakukan pekerjaan alih-alih mendelegasikan.

<h3 id="orphaned-tmux-sessions">
  Orphaned tmux sessions
</h3>

Jika session tmux bertahan setelah sesi Claude Code berakhir, mungkin tidak sepenuhnya dibersihkan. Daftar session dan bunuh yang dibuat oleh tim:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Keterbatasan
</h2>

Tim agent bersifat eksperimental. Keterbatasan saat ini untuk diketahui:

* **Tidak ada session resumption dengan rekan tim in-process**: `/resume` dan `/rewind` tidak mengembalikan rekan tim in-process. Setelah melanjutkan session, lead dapat mencoba mengirim pesan ke rekan tim yang tidak lagi ada. Jika ini terjadi, beri tahu lead untuk menelurkan rekan tim baru.
* **Status tugas dapat tertinggal**: rekan tim kadang-kadang gagal menandai tugas sebagai selesai, yang memblokir tugas dependen. Jika tugas tampak terjebak, periksa apakah pekerjaan benar-benar selesai dan perbarui status tugas secara manual atau beri tahu lead untuk mendorong rekan tim.
* **Shutdown dapat lambat**: rekan tim menyelesaikan permintaan atau tool call saat ini sebelum shutdown, yang dapat memakan waktu.
* **Satu tim per session**: sebuah session memiliki tepat satu tim, yang dibatasi pada session tersebut. Anda tidak dapat membuat tim bernama tambahan atau berbagi tim di seluruh session.
* **Tidak ada tim bersarang**: rekan tim tidak dapat menelurkan rekan tim mereka sendiri. Hanya lead yang dapat mengelola tim.
* **Tidak ada background subagents dari rekan tim in-process**: subagents rekan tim in-process sendiri berjalan di foreground. Meminta yang background, baik dengan `run_in_background` atau definisi subagent yang menetapkan `background: true`, mengembalikan error, karena pekerjaan background rekan tim tidak dapat bertahan lebih lama dari proses lead. Subagents yang diluncurkan dari percakapan utama mengikuti [default background](/docs/id/sub-agents#run-subagents-in-foreground-or-background).
* **Lead tetap**: session utama adalah lead seumur hidupnya. Anda tidak dapat mempromosikan rekan tim ke lead atau mentransfer kepemimpinan.
* **Izin ditetapkan pada spawn**: semua rekan tim dimulai dengan mode izin lead. Anda dapat mengubah mode rekan tim individual setelah spawn, tetapi Anda tidak dapat mengatur mode per-rekan tim pada waktu spawn.
* **Split panes memerlukan tmux atau iTerm2**: mode in-process default bekerja di terminal apa pun. Mode split-pane tidak didukung di integrated terminal VS Code, Windows Terminal, atau Ghostty.

<Tip>
  **`CLAUDE.md` bekerja secara normal**: rekan tim membaca file `CLAUDE.md` dari direktori kerja mereka. Gunakan ini untuk memberikan panduan spesifik proyek ke semua rekan tim.
</Tip>

<h2 id="next-steps">
  Langkah berikutnya
</h2>

Jelajahi pendekatan terkait untuk pekerjaan paralel dan delegasi:

* **Delegasi ringan**: [subagents](/docs/id/sub-agents) menelurkan agent pembantu untuk penelitian atau verifikasi dalam session Anda, lebih baik untuk tugas yang tidak memerlukan koordinasi inter-agent
* **Session paralel manual**: [Git worktrees](/docs/id/worktrees) memungkinkan Anda menjalankan beberapa session Claude Code sendiri tanpa koordinasi tim otomatis
* **Bandingkan pendekatan**: lihat perbandingan [subagent vs tim agent](/docs/id/features-overview#compare-similar-features) untuk rincian side-by-side
