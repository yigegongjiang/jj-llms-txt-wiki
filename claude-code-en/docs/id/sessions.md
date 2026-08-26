> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola sesi

> Beri nama, lanjutkan, cabang, dan beralih antar percakapan Claude Code. Mencakup `--continue`, `--resume`, `--from-pr`, pemilih `/resume`, penamaan sesi, ekspor transkrip, dan tempat penyimpanan transkrip.

Sesi adalah percakapan yang disimpan yang terikat pada direktori proyek. Claude Code menyimpannya secara lokal saat Anda bekerja, sehingga Anda dapat melanjutkan dari tempat Anda berhenti, membuat cabang untuk mencoba pendekatan berbeda, atau beralih antar tugas.

[Aplikasi desktop](/docs/id/desktop#work-in-parallel-with-sessions), [Claude Code di web](/docs/id/claude-code-on-the-web), dan [ekstensi VS Code](/docs/id/vs-code#resume-past-conversations) masing-masing mempertahankan riwayat sesi mereka sendiri. Halaman ini mencakup CLI.

<h2 id="resume-a-session">
  Lanjutkan sesi
</h2>

Sesi disimpan secara berkelanjutan ke [file transkrip lokal](#export-and-locate-session-data) saat Anda bekerja, sehingga Anda dapat kembali ke satu setelah keluar atau menjalankan `/clear`. Gunakan titik masuk ini:

| Perintah                    | Apa yang dilakukannya                                        |
| :-------------------------- | :----------------------------------------------------------- |
| `claude --continue`         | Melanjutkan sesi terbaru di direktori saat ini               |
| `claude --resume`           | Membuka [pemilih sesi](#use-the-session-picker)              |
| `claude --resume <name>`    | Melanjutkan sesi bernama secara langsung                     |
| `claude --from-pr <number>` | Melanjutkan sesi yang ditautkan ke permintaan tarik tersebut |
| `/resume`                   | Beralih ke percakapan berbeda dari dalam sesi aktif          |

Sesi yang dibuat dengan [`claude -p`](/docs/id/headless) atau [Agent SDK](/docs/id/agent-sdk/overview) tidak muncul di pemilih sesi, tetapi Anda masih dapat melanjutkan satu dengan meneruskan ID sesinya ke `claude --resume <session-id>`. Jalankan ini dari direktori tempat sesi dimulai: pencarian ID sesi dibatasi pada direktori proyek saat ini dan git worktrees-nya, jadi sesi yang dibuat di tempat lain melaporkan `No conversation found with session ID: <session-id>`.

<h3 id="where-the-session-picker-looks">
  Tempat pemilih sesi mencari
</h3>

Sesi disimpan per direktori proyek. Secara default, pemilih sesi menampilkan sesi interaktif dari worktree saat ini, ditambah sesi yang dimulai di tempat lain yang menambahkan direktori saat ini dengan `/add-dir`. Gunakan `Ctrl+W` untuk memperluas ke semua worktree repositori atau `Ctrl+A` untuk memperluas ke setiap proyek di mesin ini.

Dari v2.1.169, memindahkan sesi dengan [`/cd`](/docs/id/commands) memindahkannya ke penyimpanan proyek direktori baru, sehingga muncul di pemilih direktori tersebut setelahnya. Sejak v2.1.196, sesi yang dipindahkan tetap keluar dari pemilih direktori lama bahkan setelah kerusakan atau keluar paksa. Pada versi sebelumnya, sesi juga dapat muncul kembali di daftar direktori lama setelah keluar yang tidak bersih ketika jalur lama berisi karakter khusus seperti garis bawah.

Memilih sesi dari worktree lain dari repositori yang sama melanjutkannya di tempat. Memilih sesi dari proyek yang tidak terkait menyalin perintah `cd` dan resume ke clipboard Anda.

Melanjutkan berdasarkan nama diselesaikan di seluruh repositori saat ini dan worktree-nya. Kedua bentuk mencari kecocokan yang tepat dan melanjutkannya secara langsung bahkan jika berada di worktree berbeda:

| Perintah                 | Kecocokan tepat             | Nama ambigu                                                                       |
| :----------------------- | :-------------------------- | :-------------------------------------------------------------------------------- |
| `claude --resume <name>` | Melanjutkan secara langsung | Membuka pemilih sesi dengan nama yang sudah diisi sebagai istilah pencarian       |
| `/resume <name>`         | Melanjutkan secara langsung | Melaporkan kesalahan; jalankan `/resume` tanpa argumen untuk membuka pemilih sesi |

<h2 id="name-your-sessions">
  Beri nama sesi Anda
</h2>

Berikan sesi nama deskriptif sehingga dapat ditemukan di pemilih sesi dan dapat dilanjutkan berdasarkan nama. Ini paling penting ketika Anda mengerjakan beberapa tugas secara paralel.

| Kapan                   | Cara mengatur nama                                                                                                                                                          |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Saat startup            | `claude -n auth-refactor`                                                                                                                                                   |
| Selama sesi             | `/rename auth-refactor`. Nama juga muncul di bilah prompt                                                                                                                   |
| Dari pemilih sesi       | Sorot sesi dan tekan `Ctrl+R`                                                                                                                                               |
| Pada penerimaan rencana | Menerima rencana dalam [mode rencana](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) memberi nama sesi dari konten rencana kecuali Anda sudah menetapkan satu |

Setelah sesi diberi nama, kembali ke sesi dengan `claude --resume <name>` atau `/resume <name>`. Lihat [Lanjutkan sesi](#resume-a-session) untuk cara resolusi nama berperilaku di seluruh worktrees.

Sesi interaktif yang tidak pernah Anda beri nama masih mendapatkan nama tampilan default saat dimulai. Memerlukan Claude Code v2.1.196 atau lebih baru. Default menggabungkan nama direktori kerja dengan akhiran dua karakter, misalnya `my-app-3f`, dan mengidentifikasi sesi dalam daftar sesi yang sedang berjalan, seperti [tampilan agen](/docs/id/agent-view) dan output `claude agents --json`.

Default bukan pegangan resume: `claude --resume <name>`, `/resume <name>`, dan pemilih sesi hanya cocok dengan nama yang Anda tetapkan. Memberi nama sesi menggantikan default.

<h2 id="use-the-session-picker">
  Gunakan pemilih sesi
</h2>

Jalankan `/resume` di dalam sesi, atau `claude --resume` tanpa argumen, untuk membuka pemilih sesi interaktif. Gunakan pintasan keyboard ini untuk menavigasi, mencari, dan memperluas daftar:

| Pintasan                                                    | Tindakan                                                                                                                                                             |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                                   | Navigasi antar sesi                                                                                                                                                  |
| `→` / `←`                                                   | Perluas atau ciutkan sesi yang dikelompokkan                                                                                                                         |
| `Enter`                                                     | Lanjutkan sesi yang disorot                                                                                                                                          |
| `Space`                                                     | Pratinjau konten sesi. `Ctrl+V` juga berfungsi di terminal yang tidak menangkapnya sebagai tempel                                                                    |
| `Ctrl+R`                                                    | Ubah nama sesi yang disorot                                                                                                                                          |
| `/` atau karakter yang dapat dicetak lainnya selain `Space` | Masuk mode pencarian dan filter sesi. Tempel URL permintaan tarik atau gabung GitHub, GitHub Enterprise, GitLab, atau Bitbucket untuk menemukan sesi yang membuatnya |
| `Ctrl+A`                                                    | Tampilkan sesi dari semua proyek di mesin ini. Tekan lagi untuk kembali ke repositori saat ini                                                                       |
| `Ctrl+W`                                                    | Tampilkan sesi dari semua worktree repositori saat ini. Tekan lagi untuk kembali ke worktree saat ini. Hanya ditampilkan di repositori multi-worktree                |
| `Ctrl+B`                                                    | Filter ke sesi dari cabang git saat ini. Tekan lagi untuk menampilkan semua cabang                                                                                   |
| `Esc`                                                       | Keluar dari pemilih sesi atau mode pencarian                                                                                                                         |

Setiap baris menampilkan nama sesi jika diatur, jika tidak ringkasan percakapan atau prompt pertama, bersama dengan waktu sejak aktivitas terakhir, jumlah pesan, dan cabang git. Jalur proyek muncul setelah Anda memperluas ke semua proyek dengan `Ctrl+A`.

Sesi yang dicabangkan dibuat dengan `/branch`, `/rewind`, atau `--fork-session` dikelompokkan di bawah sesi akar mereka. Tekan `→` untuk memperluas grup.

<h2 id="branch-a-session">
  Cabang sesi
</h2>

Pencabangan membuat salinan percakapan sejauh ini dan beralih Anda ke dalamnya, meninggalkan yang asli tetap utuh. Gunakan untuk mencoba pendekatan berbeda tanpa kehilangan jalur yang Anda jalani.

Dari dalam sesi, jalankan `/branch` dengan nama opsional:

```text theme={null}
/branch try-streaming-approach
```

Jika Anda menghilangkan nama, Claude Code memberi nama cabang baru setelah prompt pertama dalam percakapan. Mulai dari v2.1.198 ini juga berlaku setelah [compaction](/docs/id/how-claude-code-works#when-context-fills-up); versi sebelumnya kembali ke nama literal `Branched conversation` alih-alih melihat melampaui ringkasan compaction ke prompt pertama asli.

Dari baris perintah, gabungkan `--continue` atau `--resume` dengan `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

Sesi asli tidak berubah dan tetap tersedia di pemilih sesi. Konfirmasi `/branch` mencetak dua ID sesi: cabang baru yang sekarang Anda masuki dan yang asli. Untuk kembali ke yang asli, teruskan ID-nya ke `/resume`, gunakan pemilih sesi, atau jalankan `/resume <original-name>`. Izin yang Anda setujui dengan "izinkan untuk sesi ini" tidak dibawa ke cabang baru. Jika Anda melanjutkan sesi yang sama di dua terminal tanpa forking, pesan dari keduanya saling bersisipan menjadi satu transkrip.

Untuk rewind berbasis checkpoint dalam satu sesi, lihat [Checkpointing](/docs/id/checkpointing).

<h2 id="manage-context-within-a-session">
  Kelola konteks dalam sesi
</h2>

Perintah ini mengontrol apa yang ada di jendela konteks tanpa meninggalkan sesi:

* **`/clear`**: mulai segar dengan konteks kosong. Percakapan sebelumnya disimpan dan dapat dilanjutkan dengan `/resume`, atau, dalam proses Claude Code yang sama, dari [entri sesi sebelumnya di menu rewind](/docs/id/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`**: ganti riwayat dengan ringkasan, secara opsional berfokus pada apa yang Anda tentukan
* **`/context`**: tampilkan apa yang saat ini mengonsumsi konteks

Untuk cara pemadatan berinteraksi dengan CLAUDE.md, skills, dan aturan, lihat [panduan jendela konteks](/docs/id/context-window). Untuk strategi tentang kapan harus menghapus versus memadatkan, lihat [Praktik terbaik](/docs/id/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Ekspor dan temukan data sesi
</h2>

Jalankan `/export` untuk membuka menu yang memungkinkan Anda menyalin percakapan saat ini ke clipboard atau menyimpannya sebagai file teks biasa, dengan pesan dan output alat dirender sebagai teks yang dapat dibaca. Teruskan nama file untuk melewati menu dan menulis langsung ke file tersebut.

<h3 id="access-conversations-from-scripts">
  Akses percakapan dari skrip
</h3>

`/export` menghasilkan transkrip yang dirender untuk dibaca oleh seseorang. Antarmuka di bawah ini menghasilkan data terstruktur untuk skrip diurai: hasil JSON dari suatu run, jalur ke file transkrip sesi, atau aliran peristiwa langsung. Pilih berdasarkan apa yang memicu skrip:

* **Jalankan Claude sekali dan tangkap hasilnya**: panggil `claude -p` dengan [`--output-format json` atau `stream-json`](/docs/id/headless#get-structured-output) untuk menangkap hasil, ID sesi, penggunaan, dan biaya dari run non-interaktif sebagai JSON terstruktur.
* **Tanyakan pertanyaan ke sesi yang ada**: teruskan ID sesi ke [`claude -p --resume`](/docs/id/headless#continue-conversations) untuk mengirim prompt tindak lanjut, seperti permintaan ringkasan, dan tangkap respons terstruktur.
* **Bereaksi terhadap peristiwa sesi**: baca bidang `transcript_path` yang diterima [hooks](/docs/id/hooks#common-input-fields) dan [perintah status line](/docs/id/statusline#available-data) sebagai input. Hook `SessionEnd` dapat mengarsipkan transkrip ketika sesi berakhir.
* **Sematkan Claude dalam aplikasi TypeScript atau Python**: gunakan [Agent SDK](/docs/id/agent-sdk/overview) untuk menerima setiap pesan secara terprogram.

Contoh di bawah menggunakan antarmuka kedua. Ini mengirim prompt tindak lanjut ke sesi yang ada dan membaca jawaban dengan `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Tempat transkrip disimpan
</h3>

Secara default, transkrip disimpan sebagai JSONL di `~/.claude/projects/<project>/<session-id>.jsonl`, di mana `<project>` adalah jalur direktori kerja Anda dengan karakter non-alfanumerik diganti dengan `-`. Setiap baris adalah objek JSON untuk pesan, penggunaan alat, atau entri metadata. Format entri bersifat internal untuk Claude Code dan berubah antar versi, jadi skrip yang mengurai file ini secara langsung dapat rusak pada rilis apa pun. Untuk membangun data sesi, gunakan `/export` atau [antarmuka skrip](#access-conversations-from-scripts) sebagai gantinya.

Lokasi, retensi, dan perilaku penulisan dapat dikonfigurasi:

| Untuk                                         | Atur                                                   | Di mana                     |
| --------------------------------------------- | ------------------------------------------------------ | --------------------------- |
| Pindahkan penyimpanan dari `~/.claude`        | [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars)                    | Variabel lingkungan         |
| Ubah retensi 30 hari                          | [`cleanupPeriodDays`](/docs/id/settings#available-settings) | `settings.json`             |
| Tekan penulisan transkrip di semua mode       | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/id/env-vars)      | Variabel lingkungan         |
| Tekan penulisan untuk satu run non-interaktif | [`--no-session-persistence`](/docs/id/cli-reference)        | Flag CLI dengan `claude -p` |

<h2 id="see-also">
  Lihat juga
</h2>

Halaman-halaman ini mencakup mekanik sesi dan paralelisme terkait:

* [Worktrees](/docs/id/worktrees): jalankan sesi paralel terisolasi di cabang terpisah
* [Checkpointing](/docs/id/checkpointing): putar ulang kode dan percakapan ke titik sebelumnya
* [Jendela konteks](/docs/id/context-window): apa yang mengisi konteks dan apa yang bertahan dari pemadatan
* [Mode non-interaktif](/docs/id/headless): perilaku sesi di bawah `claude -p`
