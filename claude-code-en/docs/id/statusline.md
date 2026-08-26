> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sesuaikan baris status Anda

> Konfigurasikan bilah status khusus untuk memantau penggunaan jendela konteks, biaya, dan status git di Claude Code

Baris status adalah bilah yang dapat disesuaikan di bagian bawah Claude Code yang menjalankan skrip shell apa pun yang Anda konfigurasikan. Skrip menerima data sesi JSON di stdin dan menampilkan apa pun yang dicetak skrip Anda, memberikan Anda tampilan yang persisten dan sekilas dari penggunaan konteks, biaya, status git, atau apa pun yang ingin Anda lacak.

Baris status berguna ketika Anda:

* Ingin memantau penggunaan jendela konteks saat bekerja
* Perlu melacak biaya sesi
* Bekerja di berbagai sesi dan perlu membedakannya
* Ingin cabang git dan status selalu terlihat

Baris status dirender di baris tersendiri di atas lencana footer bawaan dan tidak menggantikannya. Untuk menambahkan lencana tautan yang dapat diklik ke footer ketika ID muncul dalam percakapan, tanpa menulis skrip, konfigurasikan [`footerLinksRegexes`](/docs/id/settings#footer-link-badges) sebagai gantinya.

Berikut adalah contoh [baris status multi-baris](#display-multiple-lines) yang menampilkan informasi git di baris pertama dan bilah konteks berkode warna di baris kedua.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Baris status multi-baris yang menampilkan nama model, direktori, cabang git di baris pertama, dan bilah kemajuan penggunaan konteks dengan biaya dan durasi di baris kedua" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Halaman ini memandu Anda melalui [pengaturan baris status dasar](#set-up-a-status-line), menjelaskan [bagaimana aliran data](#how-status-lines-work) dari Claude Code ke skrip Anda, mencantumkan [semua bidang yang dapat Anda tampilkan](#available-data), dan menyediakan [contoh siap pakai](#examples) untuk pola umum seperti status git, pelacakan biaya, dan bilah kemajuan.

<h2 id="set-up-a-status-line">
  Atur baris status
</h2>

Gunakan [perintah `/statusline`](#use-the-%2Fstatusline-command) untuk membuat Claude Code menghasilkan skrip untuk Anda, atau [buat skrip secara manual](#manually-configure-a-status-line) dan tambahkan ke pengaturan Anda.

<h3 id="use-the-/statusline-command">
  Gunakan perintah /statusline
</h3>

Perintah `/statusline` menerima instruksi bahasa alami yang menjelaskan apa yang ingin Anda tampilkan. Claude Code menghasilkan file skrip di `~/.claude/` dan memperbarui pengaturan Anda secara otomatis:

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

<h3 id="manually-configure-a-status-line">
  Konfigurasikan baris status secara manual
</h3>

Tambahkan bidang `statusLine` ke pengaturan pengguna Anda (`~/.claude/settings.json`, di mana `~` adalah direktori home Anda) atau [pengaturan proyek](/docs/id/settings#settings-files). Atur `type` ke `"command"` dan arahkan `command` ke jalur skrip atau perintah shell inline. Untuk panduan lengkap membuat skrip, lihat [Bangun baris status langkah demi langkah](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

Bidang `command` berjalan di shell, jadi Anda juga dapat menggunakan perintah inline alih-alih file skrip. Contoh ini menggunakan `jq` untuk mengurai input JSON dan menampilkan nama model dan persentase konteks:

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

Bidang `padding` opsional menambahkan spasi horizontal ekstra (dalam karakter) ke konten baris status. Default ke `0`. Padding ini selain spasi bawaan antarmuka, jadi mengontrol indentasi relatif daripada jarak absolut dari tepi terminal.

Bidang `refreshInterval` opsional menjalankan kembali perintah Anda setiap N detik selain [pembaruan berbasis peristiwa](#how-status-lines-work). Minimum adalah `1`. Atur ini ketika baris status Anda menampilkan data berbasis waktu seperti jam, atau ketika subagen latar belakang mengubah keadaan git sementara sesi utama menganggur. Biarkan tidak diatur untuk hanya berjalan pada peristiwa.

Bidang `hideVimModeIndicator` opsional menekan teks `-- INSERT --` bawaan di bawah prompt. Atur ini ke `true` ketika skrip Anda merender [`vim.mode`](#available-data) itu sendiri, sehingga mode tidak ditampilkan dua kali.

<h3 id="disable-the-status-line">
  Nonaktifkan baris status
</h3>

Jalankan `/statusline` dan minta untuk menghapus atau menghapus baris status Anda (misalnya, `/statusline delete`, `/statusline clear`, `/statusline remove it`). Anda juga dapat secara manual menghapus bidang `statusLine` dari settings.json Anda.

<h2 id="build-a-status-line-step-by-step">
  Bangun baris status langkah demi langkah
</h2>

Panduan ini menunjukkan apa yang terjadi di balik layar dengan membuat baris status secara manual yang menampilkan model saat ini, direktori kerja, dan persentase penggunaan jendela konteks.

<Note>Menjalankan [`/statusline`](#use-the-%2Fstatusline-command) dengan deskripsi apa yang Anda inginkan mengonfigurasi semua ini untuk Anda secara otomatis.</Note>

Contoh-contoh ini menggunakan skrip Bash, yang berfungsi di macOS dan Linux. Di Windows, lihat [Konfigurasi Windows](#windows-configuration) untuk contoh PowerShell dan Git Bash.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Baris status yang menampilkan nama model, direktori, dan persentase konteks" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Buat skrip yang membaca JSON dan mencetak output">
    Claude Code mengirim data JSON ke skrip Anda melalui stdin. Skrip ini menggunakan [`jq`](https://jqlang.github.io/jq/), pengurai JSON baris perintah yang mungkin perlu Anda instal, untuk mengekstrak nama model, direktori, dan persentase konteks, kemudian mencetak baris yang diformat.

    Simpan ini ke `~/.claude/statusline.sh` (di mana `~` adalah direktori home Anda, seperti `/Users/username` di macOS atau `/home/username` di Linux):

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Buat dapat dieksekusi">
    Tandai skrip sebagai dapat dieksekusi sehingga shell Anda dapat menjalankannya:

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Tambahkan ke pengaturan">
    Beri tahu Claude Code untuk menjalankan skrip Anda sebagai baris status. Tambahkan konfigurasi ini ke `~/.claude/settings.json`, yang menetapkan `type` ke `"command"` (berarti "jalankan perintah shell ini") dan menunjuk `command` ke skrip Anda:

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    Baris status Anda muncul di bagian bawah antarmuka. Pengaturan dimuat ulang secara otomatis, tetapi perubahan tidak akan muncul sampai interaksi berikutnya Anda dengan Claude Code.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Bagaimana baris status bekerja
</h2>

Claude Code menjalankan skrip Anda dan menyalurkan [data sesi JSON](#available-data) ke dalamnya melalui stdin. Skrip Anda membaca JSON, mengekstrak apa yang dibutuhkan, dan mencetak teks ke stdout. Claude Code menampilkan apa pun yang dicetak skrip Anda.

**Kapan itu diperbarui**

Skrip Anda berjalan setelah setiap pesan asisten baru, setelah `/compact` selesai, ketika mode izin berubah, atau ketika vim mode beralih. Pembaruan dibatasi pada 300ms, berarti perubahan cepat dikumpulkan bersama dan skrip Anda berjalan sekali semuanya stabil. Jika pembaruan baru dipicu saat skrip Anda masih berjalan, eksekusi yang sedang berlangsung dibatalkan. Jika Anda mengedit skrip Anda, perubahan tidak akan muncul sampai interaksi berikutnya Anda dengan Claude Code memicu pembaruan.

Pemicu ini dapat menjadi senyap ketika sesi utama menganggur, misalnya saat koordinator menunggu subagen latar belakang. Untuk menjaga segmen berbasis waktu atau bersumber eksternal tetap terkini selama periode menganggur, atur [`refreshInterval`](#manually-configure-a-status-line) untuk juga menjalankan kembali perintah pada timer tetap.

**Apa yang dapat dicetak skrip Anda**

* **Beberapa baris**: setiap pernyataan `echo` atau `print` ditampilkan sebagai baris terpisah. Lihat [contoh multi-baris](#display-multiple-lines).
* **Warna**: gunakan [kode escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) seperti `\033[32m` untuk hijau (terminal harus mendukungnya). Lihat [contoh status git](#git-status-with-colors).
* **Tautan**: gunakan [urutan escape OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) untuk membuat teks dapat diklik (Cmd+klik di macOS, Ctrl+klik di Windows/Linux). Memerlukan terminal yang mendukung hyperlink seperti iTerm2, Kitty, atau WezTerm. Lihat [contoh tautan yang dapat diklik](#clickable-links).

**Mengukur output ke terminal**

Claude Code menangkap output skrip Anda alih-alih menghubungkannya langsung ke terminal, jadi `tput cols` dan deteksi lebar tingkat bahasa tidak dapat membaca ukuran terminal dari dalam skrip. Baca variabel lingkungan `COLUMNS` dan `LINES` sebagai gantinya. Claude Code menetapkan ini ke dimensi terminal saat ini sebelum menjalankan skrip Anda. Memerlukan Claude Code v2.1.153 atau lebih baru.

<Note>Baris status berjalan secara lokal dan tidak menggunakan token API. Itu sementara bersembunyi selama interaksi UI tertentu, termasuk saran pelengkapan otomatis, menu bantuan, dan prompt izin.</Note>

<h2 id="available-data">
  Data yang tersedia
</h2>

Claude Code mengirim bidang JSON berikut ke skrip Anda melalui stdin:

| Bidang                                                                           | Deskripsi                                                                                                                                                                                                                                                                                              |
| -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `model.id`, `model.display_name`                                                 | Pengidentifikasi model saat ini dan nama tampilan                                                                                                                                                                                                                                                      |
| `cwd`, `workspace.current_dir`                                                   | Direktori kerja saat ini. Kedua bidang berisi nilai yang sama; `workspace.current_dir` lebih disukai untuk konsistensi dengan `workspace.project_dir`.                                                                                                                                                 |
| `workspace.project_dir`                                                          | Direktori tempat Claude Code diluncurkan, yang mungkin berbeda dari `cwd` jika direktori kerja berubah selama sesi                                                                                                                                                                                     |
| `workspace.added_dirs`                                                           | Direktori tambahan yang ditambahkan melalui `/add-dir` atau `--add-dir`. Array kosong jika tidak ada yang telah ditambahkan                                                                                                                                                                            |
| `workspace.git_worktree`                                                         | Nama git worktree ketika direktori saat ini berada di dalam linked worktree yang dibuat dengan `git worktree add`. Tidak ada di main working tree. Diisi untuk git worktree apa pun, tidak seperti `worktree.*` yang hanya berlaku untuk sesi `--worktree`                                             |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Identitas repositori yang diuraikan dari remote `origin`, misalnya `"github.com"`, `"anthropics"`, `"claude-code"`. Tidak ada di luar repositori git atau ketika tidak ada remote `origin` yang dikonfigurasi                                                                                          |
| `cost.total_cost_usd`                                                            | Perkiraan biaya sesi dalam USD, dihitung sisi klien. Mungkin berbeda dari tagihan aktual Anda                                                                                                                                                                                                          |
| `cost.total_duration_ms`                                                         | Total waktu dinding jam sejak sesi dimulai, dalam milidetik                                                                                                                                                                                                                                            |
| `cost.total_api_duration_ms`                                                     | Total waktu yang dihabiskan menunggu respons API dalam milidetik                                                                                                                                                                                                                                       |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Baris kode yang diubah                                                                                                                                                                                                                                                                                 |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Jumlah token saat ini dalam jendela konteks, dari respons API terbaru. Input mencakup pembacaan dan penulisan cache. Sebelum v2.1.132 ini adalah total sesi kumulatif                                                                                                                                  |
| `context_window.context_window_size`                                             | Ukuran jendela konteks maksimum dalam token. 200000 secara default, atau 1000000 untuk model dengan konteks diperpanjang.                                                                                                                                                                              |
| `context_window.used_percentage`                                                 | Persentase jendela konteks yang digunakan yang telah dihitung sebelumnya                                                                                                                                                                                                                               |
| `context_window.remaining_percentage`                                            | Persentase jendela konteks yang tersisa yang telah dihitung sebelumnya                                                                                                                                                                                                                                 |
| `context_window.current_usage`                                                   | Jumlah token dari panggilan API terakhir, dijelaskan dalam [bidang jendela konteks](#context-window-fields)                                                                                                                                                                                            |
| `exceeds_200k_tokens`                                                            | Apakah jumlah token total (input, cache, dan output token digabungkan) dari respons API terbaru melebihi 200k. Ini adalah ambang batas tetap terlepas dari ukuran jendela konteks aktual.                                                                                                              |
| `effort.level`                                                                   | Tingkat upaya penalaran saat ini (`low`, `medium`, `high`, `xhigh`, atau `max`). Mencerminkan nilai sesi langsung, termasuk perubahan `/effort` pertengahan sesi. Ultracode bukan tingkat yang berbeda dan melaporkan sebagai `xhigh`. Tidak ada ketika model saat ini tidak mendukung parameter upaya |
| `thinking.enabled`                                                               | Apakah pemikiran diperpanjang diaktifkan untuk sesi                                                                                                                                                                                                                                                    |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Persentase batas laju 5 jam atau 7 hari yang dikonsumsi, dari 0 hingga 100                                                                                                                                                                                                                             |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Detik epoch Unix ketika jendela batas laju 5 jam atau 7 hari direset                                                                                                                                                                                                                                   |
| `session_id`                                                                     | Pengidentifikasi sesi unik                                                                                                                                                                                                                                                                             |
| `session_name`                                                                   | Nama sesi khusus yang ditetapkan dengan bendera `--name` atau `/rename`. Tidak ada jika tidak ada nama khusus yang telah ditetapkan                                                                                                                                                                    |
| `prompt_id`                                                                      | UUID yang mengidentifikasi prompt pengguna yang sedang diproses. Cocok dengan atribut [`prompt.id` pada acara OpenTelemetry](/docs/id/monitoring-usage#event-correlation-attributes). Tidak ada sampai input pengguna pertama. Memerlukan Claude Code v2.1.196 atau lebih baru                              |
| `transcript_path`                                                                | Jalur ke file transkrip percakapan                                                                                                                                                                                                                                                                     |
| `version`                                                                        | Versi Claude Code                                                                                                                                                                                                                                                                                      |
| `output_style.name`                                                              | Nama gaya output saat ini                                                                                                                                                                                                                                                                              |
| `vim.mode`                                                                       | Mode vim saat ini (`NORMAL`, `INSERT`, `VISUAL`, atau `VISUAL LINE`) ketika [vim mode](/docs/id/interactive-mode#vim-editor-mode) diaktifkan                                                                                                                                                                |
| `agent.name`                                                                     | Nama agen saat menjalankan dengan bendera `--agent` atau pengaturan agen dikonfigurasi                                                                                                                                                                                                                 |
| `pr.number`, `pr.url`                                                            | Buka permintaan tarik untuk cabang saat ini. Mencerminkan lencana PR di bilah status bawah. Tidak ada sampai PR ditemukan, ketika tidak dalam repositori git, atau setelah PR digabungkan atau ditutup                                                                                                 |
| `pr.review_state`                                                                | Status tinjauan PR terbuka: `approved`, `pending`, `changes_requested`, atau `draft`. Mungkin secara independen tidak ada bahkan ketika `pr` ada                                                                                                                                                       |
| `worktree.name`                                                                  | Nama worktree aktif. Hadir hanya selama sesi `--worktree`                                                                                                                                                                                                                                              |
| `worktree.path`                                                                  | Jalur absolut ke direktori worktree                                                                                                                                                                                                                                                                    |
| `worktree.branch`                                                                | Nama cabang Git untuk worktree (misalnya, `"worktree-my-feature"`). Tidak ada untuk worktree berbasis hook                                                                                                                                                                                             |
| `worktree.original_cwd`                                                          | Direktori tempat Claude berada sebelum memasuki worktree                                                                                                                                                                                                                                               |
| `worktree.original_branch`                                                       | Cabang Git yang diperiksa sebelum memasuki worktree. Tidak ada untuk worktree berbasis hook                                                                                                                                                                                                            |

<Accordion title="Skema JSON lengkap">
  Perintah baris status Anda menerima struktur JSON ini melalui stdin:

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-4-8",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Bidang yang mungkin tidak ada** (tidak ada dalam JSON):

  * `session_name`: muncul hanya ketika nama khusus telah ditetapkan dengan `--name` atau `/rename`
  * `prompt_id`: muncul hanya setelah input pengguna pertama
  * `workspace.git_worktree`: muncul hanya ketika direktori saat ini berada di dalam linked git worktree
  * `workspace.repo`: muncul hanya di dalam repositori git dengan remote `origin` yang dikonfigurasi
  * `effort`: muncul hanya ketika model saat ini mendukung parameter upaya penalaran
  * `vim`: muncul hanya ketika vim mode diaktifkan
  * `agent`: muncul hanya saat menjalankan dengan bendera `--agent` atau pengaturan agen dikonfigurasi
  * `pr`: muncul hanya saat PR terbuka ditemukan untuk cabang saat ini, dan dihapus setelah PR digabungkan atau ditutup. `pr.review_state` mungkin secara independen tidak ada
  * `worktree`: muncul hanya selama sesi `--worktree`. Ketika ada, `branch` dan `original_branch` juga mungkin tidak ada untuk worktree berbasis hook
  * `rate_limits`: muncul hanya untuk pelanggan Claude.ai (Pro/Max) setelah respons API pertama dalam sesi. Setiap jendela (`five_hour`, `seven_day`) mungkin secara independen tidak ada. Gunakan `jq -r '.rate_limits.five_hour.used_percentage // empty'` untuk menangani ketiadaan dengan anggun.

  **Bidang yang mungkin `null`**:

  * `context_window.current_usage`: `null` sebelum panggilan API pertama dalam sesi, dan lagi setelah `/compact` hingga panggilan API berikutnya mengisinya kembali
  * `context_window.used_percentage`, `context_window.remaining_percentage`: mungkin `null` awal dalam sesi

  Tangani bidang yang hilang dengan akses bersyarat dan nilai null dengan default fallback dalam skrip Anda.
</Accordion>

<h3 id="context-window-fields">
  Bidang jendela konteks
</h3>

Objek `context_window` menjelaskan jendela konteks langsung dari respons API terbaru. Mulai dari v2.1.132, `total_input_tokens` dan `total_output_tokens` mencerminkan penggunaan konteks saat ini, bukan total sesi kumulatif.

* **Total gabungan** (`total_input_tokens`, `total_output_tokens`): token saat ini dalam jendela konteks. `total_input_tokens` adalah jumlah dari `input_tokens`, `cache_creation_input_tokens`, dan `cache_read_input_tokens`; `total_output_tokens` adalah token output dari respons terbaru. Keduanya adalah `0` sebelum respons API pertama.
* **Penggunaan per komponen** (`current_usage`): jumlah token yang sama dipecah menurut kategori. Gunakan ini ketika Anda memerlukan cache hits terpisah dari input segar.

Objek `current_usage` berisi:

* `input_tokens`: token input dalam konteks saat ini
* `output_tokens`: token output yang dihasilkan
* `cache_creation_input_tokens`: token yang ditulis ke cache
* `cache_read_input_tokens`: token yang dibaca dari cache

Untuk apa arti bidang cache dan bagaimana cara penagihan, lihat [periksa kinerja cache](/docs/id/prompt-caching#check-cache-performance).

Bidang `used_percentage` dihitung dari token input saja: `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. Itu tidak termasuk `output_tokens`.

Jika Anda menghitung persentase konteks secara manual dari `current_usage`, gunakan formula input-only yang sama untuk mencocokkan `used_percentage`.

Objek `current_usage` adalah `null` sebelum panggilan API pertama dalam sesi, dan lagi segera setelah `/compact` hingga panggilan API berikutnya mengisinya kembali.

<h2 id="examples">
  Contoh
</h2>

Contoh-contoh ini menunjukkan pola baris status umum. Untuk menggunakan contoh apa pun:

1. Simpan skrip ke file seperti `~/.claude/statusline.sh` (atau `.py`/`.js`)
2. Buat dapat dieksekusi: `chmod +x ~/.claude/statusline.sh`
3. Tambahkan jalur ke [pengaturan](#manually-configure-a-status-line) Anda

Contoh Bash menggunakan [`jq`](https://jqlang.github.io/jq/) untuk mengurai JSON. Python dan Node.js memiliki penguraian JSON bawaan.

<h3 id="context-window-usage">
  Penggunaan jendela konteks
</h3>

Tampilkan model saat ini dan penggunaan jendela konteks dengan bilah kemajuan visual. Setiap skrip membaca JSON dari stdin, mengekstrak bidang `used_percentage`, dan membangun bilah 10 karakter di mana blok yang diisi (▓) mewakili penggunaan:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Baris status yang menampilkan nama model dan bilah kemajuan dengan persentase" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  Status git dengan warna
</h3>

Tampilkan cabang git dengan indikator berkode warna untuk file yang dipentingkan dan dimodifikasi. Skrip ini menggunakan [kode escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) untuk warna terminal: `\033[32m` adalah hijau, `\033[33m` adalah kuning, dan `\033[0m` mengatur ulang ke default.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Baris status yang menampilkan model, direktori, cabang git, dan indikator berkode warna untuk file yang dipentingkan dan dimodifikasi" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Setiap skrip memeriksa apakah direktori saat ini adalah repositori git, menghitung file yang dipentingkan dan dimodifikasi, dan menampilkan indikator berkode warna:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Pelacakan biaya dan durasi
</h3>

Lacak biaya API sesi dan waktu yang telah berlalu. Bidang `cost.total_cost_usd` mengakumulasi biaya semua panggilan API dalam sesi saat ini. Bidang `cost.total_duration_ms` mengukur total waktu yang telah berlalu sejak sesi dimulai, sementara `cost.total_api_duration_ms` melacak hanya waktu yang dihabiskan menunggu respons API.

Setiap skrip memformat biaya sebagai mata uang dan mengonversi milidetik ke menit dan detik:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Baris status yang menampilkan nama model, biaya sesi, dan durasi" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Tampilkan beberapa baris
</h3>

Skrip Anda dapat menampilkan beberapa baris untuk membuat tampilan yang lebih kaya. Setiap pernyataan `echo` menghasilkan baris terpisah di area status.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Baris status multi-baris yang menampilkan nama model, direktori, cabang git di baris pertama, dan bilah kemajuan penggunaan konteks dengan biaya dan durasi di baris kedua" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Contoh ini menggabungkan beberapa teknik: warna berbasis ambang batas (hijau di bawah 70%, kuning 70-89%, merah 90%+), bilah kemajuan, dan informasi cabang git. Setiap pernyataan `print` atau `echo` membuat baris terpisah:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Tautan yang dapat diklik
</h3>

Contoh ini membuat tautan yang dapat diklik ke repositori GitHub Anda. Itu membaca URL remote git, mengonversi format SSH ke HTTPS dengan `sed`, dan membungkus nama repo dalam kode escape OSC 8. Tahan Cmd (macOS) atau Ctrl (Windows/Linux) dan klik untuk membuka tautan di browser Anda.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Baris status yang menampilkan tautan yang dapat diklik ke repositori GitHub" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Setiap skrip mendapatkan URL remote git, mengonversi format SSH ke HTTPS, dan membungkus nama repo dalam kode escape OSC 8. Versi Bash menggunakan `printf '%b'` yang menginterpretasikan escape backslash lebih andal daripada `echo -e` di berbagai shell:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Penggunaan batas laju
</h3>

Tampilkan penggunaan batas laju langganan Claude.ai dalam baris status. Objek `rate_limits` berisi `five_hour` (jendela bergulir 5 jam) dan `seven_day` (jendela mingguan). Setiap jendela menyediakan `used_percentage` (0-100) dan `resets_at` (detik epoch Unix ketika jendela direset).

Bidang ini hanya ada untuk pelanggan Claude.ai (Pro/Max) setelah respons API pertama. Setiap skrip menangani bidang yang tidak ada dengan anggun:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Cache operasi yang mahal
</h3>

Skrip baris status Anda berjalan sering selama sesi aktif. Perintah seperti `git status` atau `git diff` dapat lambat, terutama di repositori besar. Contoh ini menyimpan informasi git ke file temp dan hanya menyegarkannya setiap 5 detik.

Nama file cache perlu stabil di seluruh invokasi baris status dalam sesi, tetapi unik di seluruh sesi sehingga sesi bersamaan di repositori berbeda tidak membaca keadaan git cache satu sama lain. Pengidentifikasi berbasis proses seperti `$$`, `os.getpid()`, atau `process.pid` berubah pada setiap invokasi dan mengalahkan cache. Gunakan `session_id` dari input JSON sebagai gantinya: itu stabil untuk seumur hidup sesi dan unik per sesi.

Setiap skrip memeriksa apakah file cache hilang atau lebih lama dari 5 detik sebelum menjalankan perintah git:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -f %m is macOS, stat -c %Y is Linux
      [ $(($(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Konfigurasi Windows
</h3>

Di Windows, Claude Code menjalankan perintah baris status melalui Git Bash ketika Git Bash diinstal, atau melalui PowerShell ketika Git Bash tidak ada.

Git Bash memperlakukan backslash yang tidak dikutip sebagai karakter escape, jadi jalur gaya Windows seperti `C:\Users\username\script.mjs` mencapai runner skrip dengan pemisahnya dihapus dan perintah gagal tanpa kesalahan yang terlihat. Tulis jalur file dalam string `command` dengan garis miring ke depan, seperti yang ditunjukkan dalam contoh di bawah ini. Pintasan `~` juga berfungsi dan berkembang ke direktori home Windows Anda.

Untuk menjalankan skrip PowerShell sebagai baris status Anda, panggil melalui `powershell`. Ini berfungsi apakah Claude Code merutekan perintah melalui Git Bash atau PowerShell:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

Atau, ketika Git Bash diinstal, jalankan skrip Bash secara langsung:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Baris status subagen
</h2>

Pengaturan `subagentStatusLine` merender badan baris khusus untuk setiap [subagen](/docs/id/sub-agents) yang ditampilkan di panel agen di bawah prompt. Gunakan untuk mengganti baris default `name · description · token count` dengan pemformatan Anda sendiri.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

Perintah berjalan sekali per tick refresh dengan semua baris subagen yang terlihat diteruskan sebagai objek JSON tunggal di stdin. Input mencakup [bidang hook umum](/docs/id/hooks#common-input-fields), bidang `columns` dengan lebar baris yang dapat digunakan, dan array `tasks`. Setiap tugas memiliki `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `contextWindowSize`, `tokenCount`, `tokenSamples`, dan `cwd`.

Bidang `model` per-tugas adalah ID model yang diselesaikan tempat tugas berjalan. `contextWindowSize` adalah jendela konteks model tersebut dalam token, dihitung dengan cara yang sama seperti `context_window.context_window_size` baris status utama, sehingga Anda dapat merender persentase per-baris dari `tokenCount`. Kedua bidang memerlukan Claude Code v2.1.205 atau lebih baru dan dihilangkan untuk tugas yang modelnya belum diselesaikan.

Tulis satu baris JSON ke stdout per baris yang ingin Anda ganti, dalam bentuk `{"id": "<task id>", "content": "<row body>"}`. String `content` dirender apa adanya, termasuk warna ANSI dan hyperlink OSC 8. Hilangkan `id` tugas untuk menjaga rendering default untuk baris itu; keluarkan string `content` kosong untuk menyembunyikannya.

Gerbang kepercayaan dan `disableAllHooks` yang sama yang berlaku untuk `statusLine` berlaku di sini. Plugin dapat mengirimkan `subagentStatusLine` default dalam [`settings.json`](/docs/id/plugins-reference#standard-plugin-layout) mereka.

<h2 id="tips">
  Tips
</h2>

* **Uji dengan input mock**: `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Jaga output tetap pendek**: bilah status memiliki lebar terbatas, jadi output panjang mungkin dipotong atau membungkus dengan canggung
* **Cache operasi lambat**: skrip Anda berjalan sering selama sesi aktif, jadi perintah seperti `git status` dapat menyebabkan lag. Lihat [contoh caching](#cache-expensive-operations) untuk cara menangani ini.

Proyek komunitas seperti [ccstatusline](https://github.com/sirmalloc/ccstatusline) dan [starship-claude](https://github.com/martinemde/starship-claude) menyediakan konfigurasi pra-bangun dengan tema dan fitur tambahan.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

**Baris status tidak muncul**

* Verifikasi skrip Anda dapat dieksekusi: `chmod +x ~/.claude/statusline.sh`
* Periksa bahwa skrip Anda menampilkan ke stdout, bukan stderr
* Jalankan skrip Anda secara manual untuk memverifikasi itu menghasilkan output
* Di Windows dengan Git Bash terinstal, garis miring terbalik dalam jalur `command` kemungkinan besar dikonsumsi sebagai karakter escape sebelum skrip berjalan. Gunakan garis miring ke depan dalam jalur. Lihat [Konfigurasi Windows](#windows-configuration).
* Jika `disableAllHooks` diatur ke `true` dalam pengaturan Anda, baris status juga dinonaktifkan. Hapus pengaturan ini atau atur ke `false` untuk mengaktifkan kembali.
* Jalankan `claude --debug` untuk mencatat kode keluar dan stderr dari invokasi baris status pertama dalam sesi
* Minta Claude untuk membaca file pengaturan Anda dan jalankan perintah `statusLine` secara langsung untuk mengungkap kesalahan

**Baris status menampilkan `--` atau nilai kosong**

* Bidang mungkin `null` sebelum respons API pertama selesai
* Tangani nilai null dalam skrip Anda dengan fallback seperti `// 0` dalam jq
* Mulai ulang Claude Code jika nilai tetap kosong setelah beberapa pesan

**Persentase konteks menampilkan nilai yang tidak terduga**

* Gunakan `used_percentage` untuk keadaan konteks yang paling akurat dan sederhana
* Persentase konteks mungkin berbeda dari output `/context` karena kapan masing-masing dihitung

**Tautan OSC 8 tidak dapat diklik**

* Verifikasi terminal Anda mendukung hyperlink OSC 8 (iTerm2, Kitty, WezTerm)

* Terminal.app tidak mendukung tautan yang dapat diklik

* Jika teks tautan muncul tetapi tidak dapat diklik, Claude Code mungkin tidak mendeteksi dukungan hyperlink di terminal Anda. Ini biasanya mempengaruhi Windows Terminal dan emulator lain yang tidak ada dalam daftar deteksi otomatis. Atur variabel lingkungan `FORCE_HYPERLINK` untuk mengganti deteksi sebelum meluncurkan Claude Code:

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  Di PowerShell, atur variabel dalam sesi saat ini terlebih dahulu:

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* Sesi SSH dan tmux mungkin menghapus urutan OSC tergantung pada konfigurasi

* Jika urutan escape muncul sebagai teks literal seperti `\e]8;;`, gunakan `printf '%b'` alih-alih `echo -e` untuk penanganan escape yang lebih andal

**Glitch tampilan dengan urutan escape**

* Urutan escape kompleks (warna ANSI, tautan OSC 8) dapat sesekali menyebabkan output berantakan jika tumpang tindih dengan pembaruan UI lainnya
* Jika Anda melihat teks yang rusak, coba sederhanakan skrip Anda ke output teks biasa
* Baris status multi-baris dengan kode escape lebih rentan terhadap masalah rendering daripada teks biasa satu baris

**Kepercayaan ruang kerja diperlukan**

* Perintah baris status hanya berjalan jika Anda telah menerima dialog kepercayaan ruang kerja untuk direktori saat ini. Karena `statusLine` mengeksekusi perintah shell, itu memerlukan penerimaan kepercayaan yang sama seperti hooks dan pengaturan lain yang mengeksekusi shell.
* Jika kepercayaan tidak diterima, Anda akan melihat notifikasi `statusline skipped · restart to fix` alih-alih output baris status Anda. Mulai ulang Claude Code dan terima prompt kepercayaan untuk mengaktifkannya.

**Kesalahan skrip atau hang**

* Skrip yang keluar dengan kode non-nol atau tidak menghasilkan output menyebabkan baris status menjadi kosong
* Skrip lambat memblokir baris status dari pembaruan sampai selesai. Jaga skrip tetap cepat untuk menghindari output basi.
* Jika pembaruan baru dipicu saat skrip lambat berjalan, skrip yang sedang berlangsung dibatalkan
* Uji skrip Anda secara independen dengan input mock sebelum mengonfigurasinya

**Notifikasi berbagi baris status**

* Notifikasi sistem seperti kesalahan server MCP dan pembaruan otomatis ditampilkan di sisi kanan baris yang sama dengan baris status Anda. Notifikasi sementara seperti peringatan konteks-rendah juga bersiklus melalui area ini.
* Mengaktifkan mode verbose menambahkan penghitung token ke area ini
* Di terminal sempit, notifikasi ini mungkin memotong output baris status Anda
