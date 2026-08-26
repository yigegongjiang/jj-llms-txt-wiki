> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jadwalkan tugas berulang di Claude Code Desktop

> Atur tugas terjadwal di Claude Code Desktop untuk menjalankan Claude secara otomatis pada basis berulang untuk tinjauan kode harian, audit dependensi, atau briefing pagi.

Tugas terjadwal memulai sesi baru secara otomatis pada waktu dan frekuensi yang Anda pilih. Gunakan untuk pekerjaan berulang seperti tinjauan kode harian, pemeriksaan pembaruan dependensi, atau briefing pagi yang menarik dari kalender dan kotak masuk Anda.

Halaman **Routines** aplikasi Desktop memungkinkan Anda membuat tugas terjadwal lokal dan [routines](/docs/id/routines) jarak jauh. Tugas lokal berjalan di mesin Anda dengan akses langsung ke file dan alat Anda, tetapi hanya berfungsi saat aplikasi terbuka dan komputer Anda terjaga. Routine jarak jauh berjalan pada infrastruktur cloud yang dikelola Anthropic bahkan ketika komputer Anda mati, dan juga dapat dipicu oleh panggilan API atau acara GitHub. Halaman ini mencakup tugas terjadwal lokal; untuk routine jarak jauh dan opsi pemicu mereka, lihat [Routines](/docs/id/routines).

<h2 id="compare-scheduling-options">
  Bandingkan opsi penjadwalan
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  Secara default, tugas terjadwal berjalan terhadap status apapun yang ada di direktori kerja Anda, termasuk perubahan yang belum di-commit. Aktifkan toggle worktree saat membuat tugas untuk memberikan setiap run worktree Git terisolasi sendiri, dengan cara yang sama seperti [sesi paralel](/docs/id/desktop#work-in-parallel-with-sessions) bekerja.
</Note>

<h2 id="create-a-scheduled-task">
  Buat tugas terjadwal
</h2>

Klik **Routines** di sidebar, kemudian klik **New routine** dan pilih **Local**. Konfigurasikan bidang-bidang ini:

| Bidang       | Deskripsi                                                                                                                                                                                                                                                                                       |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Name         | Pengenal untuk tugas. Dikonversi ke kebab-case huruf kecil dan digunakan sebagai nama folder di disk. Harus unik di seluruh tugas Anda.                                                                                                                                                         |
| Description  | Ringkasan singkat yang ditampilkan dalam daftar tugas.                                                                                                                                                                                                                                          |
| Instructions | Apa yang harus dilakukan Claude saat tugas berjalan. Tulis ini dengan cara yang sama seperti Anda menulis pesan apa pun di kotak prompt. Input instruksi mencakup pemilih untuk mode izin dan model, dan di bawahnya Anda memilih folder kerja dan apakah akan berjalan di worktree terisolasi. |
| Schedule     | Seberapa sering tugas berjalan. Lihat [opsi jadwal](#schedule-options) di bawah.                                                                                                                                                                                                                |

Folder diperlukan sebelum Anda dapat menyimpan tugas. Jika Anda belum mempercayai folder itu, Desktop akan meminta Anda untuk mempercayainya sebelum menyimpan.

Anda juga dapat membuat tugas dengan mendeskripsikan apa yang Anda inginkan dalam sesi apa pun. Misalnya, "set up a daily code review that runs every morning at 9am" membuat tugas berulang, dan "remind me at 3pm tomorrow to check the deploy" membuat tugas satu kali yang menonaktifkan dirinya sendiri setelah berfungsi.

<h2 id="schedule-options">
  Opsi jadwal
</h2>

Pilih preset dari kontrol Schedule:

* **Manual**: tidak ada jadwal, hanya berjalan saat Anda mengklik **Run now**. Berguna untuk menyimpan prompt yang Anda picu sesuai permintaan
* **Hourly**: berjalan setiap jam
* **Daily**: menampilkan pemilih waktu, default ke 9:00 AM waktu lokal
* **Weekdays**: sama dengan Daily tetapi melewati Sabtu dan Minggu
* **Weekly**: menampilkan pemilih waktu dan pemilih hari

Untuk interval yang tidak ditawarkan pemilih, seperti setiap 15 menit, hari pertama setiap bulan, atau run tunggal pada waktu masa depan tertentu, minta Claude dalam sesi Desktop apa pun untuk mengatur jadwal. Gunakan bahasa biasa; misalnya, "schedule a task to run all the tests every 6 hours."

<h2 id="how-scheduled-tasks-run">
  Cara tugas terjadwal berjalan
</h2>

Tugas terjadwal berjalan di mesin Anda. Desktop memeriksa jadwal setiap menit saat aplikasi terbuka dan memulai sesi segar saat tugas jatuh tempo, independen dari sesi manual apa pun yang Anda buka. Setiap tugas mendapat penundaan kecil beberapa menit setelah waktu terjadwal untuk membagi lalu lintas API. Penundaan bersifat deterministik: tugas yang sama selalu dimulai pada offset yang sama.

Saat tugas berfungsi, Anda mendapatkan notifikasi desktop dan sesi baru muncul di bawah bagian **Scheduled** di sidebar. Buka untuk melihat apa yang dilakukan Claude, tinjau perubahan, atau respons ke prompt izin. Sesi bekerja seperti yang lain: Claude dapat mengedit file, menjalankan perintah, membuat commit, dan membuka pull request.

Tugas hanya berjalan saat aplikasi desktop berjalan dan komputer Anda terjaga. Jika komputer Anda tidur melalui waktu terjadwal, run dilewati. Untuk mencegah idle-sleep, aktifkan **Keep computer awake** di Settings di bawah **Desktop app → General**. Menutup laptop lid masih membuatnya tidur. Untuk tugas yang perlu berjalan bahkan ketika komputer Anda mati, atau yang harus dipicu oleh panggilan API atau acara GitHub, buat [routine](/docs/id/routines) jarak jauh sebagai gantinya.

<h2 id="missed-runs">
  Run yang terlewat
</h2>

Saat aplikasi dimulai atau komputer Anda bangun, Desktop memeriksa apakah setiap tugas melewatkan run apa pun dalam tujuh hari terakhir. Jika demikian, Desktop memulai tepat satu run catch-up untuk waktu yang paling baru terlewat dan membuang apa pun yang lebih lama. Tugas harian yang melewatkan enam hari berjalan sekali saat bangun. Desktop menampilkan notifikasi saat run catch-up dimulai.

Ingat ini saat menulis prompt. Tugas yang dijadwalkan untuk 9am mungkin berjalan pada 11pm jika komputer Anda tidur sepanjang hari. Jika waktu penting, tambahkan guardrail ke prompt itu sendiri, misalnya: "Only review today's commits. If it's after 5pm, skip the review and just post a summary of what was missed."

<h2 id="permissions-for-scheduled-tasks">
  Izin untuk tugas terjadwal
</h2>

Setiap tugas memiliki mode izin sendiri, yang Anda atur saat membuat atau mengedit tugas. Aturan izin dari `~/.claude/settings.json` juga berlaku untuk sesi tugas terjadwal. Jika tugas berjalan dalam mode Ask dan perlu menjalankan alat yang tidak memiliki izin, run terhenti sampai Anda menyetujuinya. Sesi tetap terbuka di sidebar sehingga Anda dapat menjawab nanti.

Untuk menghindari stall, klik **Run now** setelah membuat tugas, perhatikan prompt izin, dan pilih "always allow" untuk masing-masing. Run masa depan dari tugas itu auto-approve alat yang sama tanpa meminta. Anda dapat meninjau dan mencabut persetujuan ini dari halaman detail tugas.

Alat connector [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) meminta pada setiap panggilan dan tidak menawarkan opsi always-allow. Run yang memanggil alat ini terhenti setiap kali.

<h2 id="manage-scheduled-tasks">
  Kelola tugas terjadwal
</h2>

Klik tugas dalam daftar **Routines** untuk membuka halaman detailnya. Dari sini Anda dapat:

* **Run now**: mulai tugas segera tanpa menunggu waktu terjadwal berikutnya
* **Status**: toggle antara Active dan Paused untuk menjeda atau melanjutkan run terjadwal tanpa menghapus tugas
* **Edit**: ubah instruksi, jadwal, folder, atau pengaturan lainnya
* **Review history**: lihat setiap run masa lalu, termasuk run yang dilewati. Arahkan entri yang dilewati untuk melihat mengapa: komputer Anda tidur, run sebelumnya masih berlangsung, atau tugas terjadwal lainnya sudah berjalan. Klik **Show more** untuk memuat entri yang lebih lama.
* **Review allowed permissions**: lihat dan cabut persetujuan alat yang disimpan untuk tugas ini dari panel **Always allowed**
* **Delete**: hapus tugas dan arsipkan semua sesi yang dibuatnya. Kotak centang **Also delete files on disk** muncul di dialog konfirmasi; centang untuk juga menghapus file `SKILL.md` tugas dan data terkait dari `~/.claude/scheduled-tasks/`.

Anda juga dapat membuat daftar, membuat, mengedit, dan menjeda tugas dengan meminta Claude dalam sesi Desktop apa pun. Misalnya, "pause my dependency-audit task" atau "show me my scheduled tasks." Untuk menghapus tugas, gunakan tombol **Delete** di halaman detailnya.

Tugas terjadwal juga dapat memodifikasi jadwal atau prompt sendiri dari dalam sesi yang berjalan menggunakan alat MCP `update_scheduled_task`. Ini memungkinkan tugas untuk menjadwalkan ulang dirinya sendiri berdasarkan apa yang ditemukannya, misalnya, menjadwalkan ulang tinjauan kode untuk berjalan lebih awal ketika mendeteksi cabang rilis telah dibuat.

Untuk mengedit prompt tugas di disk, buka `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (atau di bawah [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars) jika diatur). File menggunakan frontmatter YAML untuk `name` dan `description`, dengan prompt sebagai body. Perubahan berlaku pada run berikutnya. Schedule, folder, model, dan enabled state tidak ada dalam file ini: ubah melalui formulir Edit atau minta Claude.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Routines](/docs/id/routines): jalankan tugas pada infrastruktur yang dikelola Anthropic pada jadwal, melalui panggilan API, atau sebagai respons terhadap acara GitHub, bahkan ketika komputer Anda mati
* [Run prompts on a schedule](/docs/id/scheduled-tasks): penjadwalan scoped sesi dengan `/loop` di CLI
* [Claude Code GitHub Actions](/docs/id/github-actions): jalankan Claude pada jadwal di CI alih-alih di mesin Anda
* [Use Claude Code Desktop](/docs/id/desktop): panduan aplikasi Desktop lengkap
