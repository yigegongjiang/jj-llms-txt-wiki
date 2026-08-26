> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tangkap masalah keamanan saat Claude menulis kode

> Instal plugin security-guidance untuk membuat Claude meninjau perubahan kodenya sendiri untuk kerentanan dan memperbaikinya dalam sesi yang sama.

Plugin security guidance membuat Claude meninjau perubahan kodenya sendiri untuk kerentanan umum saat bekerja dan memperbaiki apa yang ditemukannya dalam sesi yang sama. Plugin menangkap masalah seperti injection, unsafe deserialization, dan unsafe DOM APIs sebelum kode mencapai pull request, mengurangi berapa banyak review keamanan yang jatuh ke reviewer manusia di hilir.

Setelah diinstal, plugin berjalan secara otomatis. Tidak ada yang perlu dipanggil dan tidak ada perintah terpisah yang perlu diingat.

Plugin adalah pendamping dalam-sesi untuk [Code Review](/docs/id/code-review), yang berjalan pada pull request. Plugin ini mengurangi apa yang mencapai PR. Code Review menangkap apa yang ada. Untuk bagaimana plugin berlapis dengan review on-demand dan CI scanning, lihat [Bagaimana ini cocok dengan alat keamanan lainnya](#how-this-fits-with-other-security-tools).

<h2 id="prerequisites">
  Prasyarat
</h2>

* Claude Code CLI versi 2.1.144 atau lebih baru
* Python 3.8 atau lebih baru di `PATH` Anda. Plugin mencoba `python3`, `python`, dan `py -3` dalam urutan itu
* Repositori git untuk direktori tempat Anda bekerja. Review end-of-turn dan commit melakukan diff terhadap status git dan melewati secara diam-diam di luar repositori. Pemeriksaan pola per-edit berfungsi di mana saja

Pada run pertama, plugin membuat virtual environment di bawah `~/.claude/security/` dan menginstal Claude Agent SDK ke dalamnya, yang memerlukan `pip` dan akses jaringan. Jika instalasi itu gagal, review commit kembali ke review single-shot daripada yang agentic. Di Windows, langkah virtual environment dilewati, jadi review commit agentic hanya berjalan jika `claude-agent-sdk` sudah dapat diimpor dan sebaliknya kembali dengan cara yang sama.

<h2 id="install-the-plugin">
  Instal plugin
</h2>

Dalam sesi Claude Code, instal dari [marketplace resmi Anthropic](/docs/id/discover-plugins#official-anthropic-marketplace):

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

Instalasi meminta scope. Pilih user scope untuk menulis plugin ke pengaturan pengguna Anda, sehingga dimuat di setiap sesi lokal baru yang Anda mulai di mesin ini. Jika Claude Code melaporkan bahwa marketplace tidak ditemukan, jalankan `/plugin marketplace add anthropics/claude-plugins-official` terlebih dahulu, kemudian coba lagi instalnya.

Kemudian aktifkan dalam sesi saat ini dengan `/reload-plugins`, yang menerapkan perubahan plugin yang tertunda tanpa restart:

```text theme={null}
/reload-plugins
```

<h3 id="enable-in-cloud-sessions-and-shared-repositories">
  Aktifkan dalam sesi cloud dan repositori bersama
</h3>

Plugin yang dibatasi pengguna tidak masuk ke [Claude Code di web](/docs/id/claude-code-on-the-web), karena sesi tersebut berjalan di infrastruktur Anthropic daripada mesin Anda. Untuk mengaktifkan plugin di sana, atau untuk mengaktifkannya bagi semua orang yang mengkloning repositori, deklarasikan dalam pengaturan proyek yang diperiksa:

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

Administrator dapat mengaktifkan plugin di seluruh organisasi dengan menetapkan [`enabledPlugins`](/docs/id/settings#plugin-settings) dalam [pengaturan terkelola](/docs/id/admin-setup).

<h2 id="what-the-plugin-checks">
  Apa yang diperiksa plugin
</h2>

Plugin meninjau pekerjaan Claude di tiga titik, masing-masing pada kedalaman yang berbeda:

* [Pada setiap edit file](#on-each-file-edit): pencocokan pola cepat untuk panggilan berisiko, tanpa panggilan model
* [Di akhir setiap giliran](#at-the-end-of-each-turn): review model latar belakang dari semua yang berubah giliran itu
* [Pada setiap commit atau push yang dilakukan Claude](#on-each-commit-or-push-claude-makes): review agentic yang lebih dalam yang membaca kode sekitarnya

Anda dapat memperluas setiap lapisan dengan [menambahkan aturan Anda sendiri](#add-your-own-rules). Pemeriksaan bawaan tidak dapat dihapus secara individual, tetapi Anda dapat [menonaktifkan setiap lapisan](#disable-or-uninstall) secara independen.

<h3 id="on-each-file-edit">
  Pada setiap edit file
</h3>

Ketika Claude menulis ke file, plugin memindai konten baru untuk pola berisiko yang diketahui. Ini adalah pencocokan pola tanpa panggilan model, jadi tidak menambah biaya penggunaan.

Kategori pola contoh:

* Eksekusi kode dinamis: `eval(`, `new Function`, `os.system`, `child_process.exec`
* Unsafe deserialization: `pickle`
* DOM injection: `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* File workflow: edit di bawah `.github/workflows/`, yang dapat memberikan izin tingkat repositori

Pemeriksaan berjalan setelah edit mendarat dan menambahkan peringatan ke konteks Claude untuk langkah berikutnya. Setiap peringatan menyala sekali per pola per file per sesi, jadi kecocokan berulang dalam file yang sama tidak membanjiri percakapan.

Anda dapat [menambahkan pola Anda sendiri](#add-custom-per-edit-patterns) ke lapisan ini dengan file `security-patterns.yaml`.

<h3 id="at-the-end-of-each-turn">
  Di akhir setiap giliran
</h3>

Giliran adalah satu putaran Claude merespons: Anda mengirim pesan, Claude bekerja dan membalas, dan giliran berakhir. Setelah setiap giliran, plugin menghitung git diff dari semua yang berubah di pohon kerja selama giliran, termasuk perubahan dari alat edit Claude, perintah Bash, dan subagent, dan mengirimnya ke review Claude terpisah yang berfokus pada keamanan. Review berjalan di latar belakang, jadi balasan Claude tidak tertunda. Jika review menemukan masalah, Claude diminta kembali dengan temuan dan mengatasinya sebagai tindak lanjut.

Ini menangkap masalah yang tidak dapat dilakukan pencocokan string, seperti:

* Authorization bypass
* Insecure direct object references
* Injection
* Server-side request forgery
* Weak cryptography

Anda melihat temuan dan resolusi Claude secara langsung dalam sesi Anda. Review mencakup hingga 30 file yang berubah per giliran dan menyala paling banyak tiga kali berturut-turut sebelum menghasilkan kembali kepada Anda.

<h3 id="on-each-commit-or-push-claude-makes">
  Pada setiap commit atau push yang dilakukan Claude
</h3>

Ketika Claude menjalankan `git commit` atau `git push` melalui alat Bash-nya, plugin menjalankan review agentic yang lebih dalam dari perubahan di latar belakang. Review ini membaca kode sekitarnya, termasuk pemanggil, sanitizer, dan file terkait, untuk memutuskan apakah temuan itu nyata sebelum melaporkannya. Konteks tambahan membuat false positive tetap rendah pada pola yang terlihat berbahaya secara terisolasi tetapi aman di codebase Anda.

Lapisan ini hanya menyala pada commit dan push yang dilakukan Claude melalui alat Bash-nya. Commit yang Anda jalankan dari shell Anda sendiri, termasuk escape shell `!` di dalam sesi, tidak ditinjau. Review commit dan push dibatasi pada 20 per jam bergulir. Jika temuan review commit menggandakan apa yang sudah dilaporkan review end-of-turn, Claude tidak diminta kembali, jadi commit yang bersih tidak menghasilkan output yang terlihat dari lapisan ini.

<h3 id="review-independence-and-limits">
  Review independence dan limits
</h3>

Plugin tidak meminta instance Claude yang sama yang menulis kode untuk menilai dirinya sendiri. Pemeriksaan per-edit adalah pencocokan string deterministik tanpa model yang terlibat. Review end-of-turn dan commit berjalan sebagai panggilan Claude terpisah dengan konteks segar dan prompt yang berfokus pada keamanan: reviewer dimulai dari diff, tidak memiliki investasi dalam pendekatan asli, dan hanya diinstruksikan untuk menemukan masalah.

Tidak ada lapisan yang memblokir penulisan atau commit. Temuan mencapai Claude yang menulis sebagai instruksi, Claude mengatasinya dalam percakapan, dan model review dapat melewatkan masalah. Perlakukan plugin sebagai satu lapisan pertahanan mendalam, bukan solusi keamanan lengkap. Lihat [Bagaimana ini cocok dengan alat keamanan lainnya](#how-this-fits-with-other-security-tools).

<h2 id="add-your-own-rules">
  Tambahkan aturan Anda sendiri
</h2>

Plugin memiliki dua titik ekstensi: file panduan Markdown untuk review yang didukung model, dan file pola YAML atau JSON untuk pencocokan string per-edit. Keduanya bersifat aditif. Anda dapat menambahkan pemeriksaan tetapi tidak dapat menonaktifkan yang bawaan dari file ini.

<h3 id="add-guidance-for-the-model-backed-reviews">
  Tambahkan panduan untuk review yang didukung model
</h3>

Buat `.claude/claude-security-guidance.md` di proyek Anda dan jelaskan model ancaman dan daftar periksa review Anda dalam bahasa biasa. Review yang didukung model memuatnya sebagai konteks tambahan bersama dengan daftar periksa kerentanan bawaan.

Contoh berikut adalah untuk layanan web dengan rute admin yang gated peran dan kebijakan logging data pelanggan:

```markdown .claude/claude-security-guidance.md theme={null}
# Security guidance for this repo

- Do not log `customer_id` or `account_number` at INFO level or above.
- All routes under `/admin` must call `require_role("admin")` before any database read.
- Use `crypto.timingSafeEqual` for token comparison instead of `===`.
```

Aturan ini adalah panduan untuk reviewer, bukan guardrail deterministik. Plugin menampilkan pelanggaran sebagai temuan untuk Claude perbaiki, tetapi tidak memblokir penulisan atau menjamin setiap pelanggaran ditangkap. Panduan bersifat aditif saja: aturan yang mengatakan untuk mengabaikan kelas kerentanan tidak menekan temuan tersebut. Untuk penegakan keras, pasangkan plugin dengan [hook yang memblokir edit](/docs/id/hooks-guide#block-edits-to-protected-files) atau pemeriksaan CI.

<h3 id="add-custom-per-edit-patterns">
  Tambahkan pola per-edit kustom
</h3>

Buat `.claude/security-patterns.yaml` untuk menambahkan aturan regex atau substring ke [pemeriksaan pola per-edit](#on-each-file-edit). Ini berjalan sebagai pencocokan string deterministik bersama dengan pola bawaan:

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "Hardcoded API key prefix. Load credentials from the secret manager."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "Multi-tenant code must filter by org_id."
```

| Field           | Type   | Description                                                                                                                                             |
| :-------------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `rule_name`     | string | Identifier shown in the warning                                                                                                                         |
| `reminder`      | string | Warning text appended to Claude's context, capped at 1 KB                                                                                               |
| `regex`         | string | Python regex matched against the edited content                                                                                                         |
| `substrings`    | list   | Literal substrings; provide this or `regex`                                                                                                             |
| `paths`         | list   | Optional glob patterns; the rule applies only to matching files. Globs match against the full file path, so prefix project-relative patterns with `**/` |
| `exclude_paths` | list   | Optional glob patterns to skip; same matching as `paths`                                                                                                |

Plugin juga membaca `.claude/security-patterns.yml` dan `.claude/security-patterns.json` dengan skema yang sama. JSON bekerja pada instalasi Python apa pun. Bentuk YAML memerlukan PyYAML untuk dapat diimpor, yang tidak diinstal plugin untuk Anda. Plugin memuat hingga 50 aturan kustom dan melewati regex yang terlihat rentan terhadap catastrophic backtracking.

<h3 id="rule-file-lookup-locations">
  Lokasi pencarian file aturan
</h3>

Plugin mencari `claude-security-guidance.md` dan `security-patterns.yaml` di lokasi yang sama, terlepas dari bagaimana plugin diaktifkan:

| Scope         | Path                                        | Notes                                    |
| :------------ | :------------------------------------------ | :--------------------------------------- |
| User          | `~/.claude/claude-security-guidance.md`     | Applies to every project on your machine |
| Project       | `.claude/claude-security-guidance.md`       | Checked in with the repository           |
| Project local | `.claude/claude-security-guidance.local.md` | Gitignored, for personal overrides       |

Plugin memuat semua lokasi yang ada dan menggabungkannya, dengan batas gabungan 8 KB untuk file panduan. Administrator dapat mendistribusikan aturan di seluruh organisasi dengan mendorong file lingkup pengguna ke `~/.claude/` melalui manajemen perangkat. Jalur yang sama berlaku untuk `security-patterns.yaml`.

<h2 id="usage-cost">
  Biaya penggunaan
</h2>

[Pemeriksaan pola per-edit](#on-each-file-edit) tidak membuat panggilan model dan tidak menambah biaya. Review [end-of-turn](#at-the-end-of-each-turn) dan [commit](#on-each-commit-or-push-claude-makes) masing-masing menghabiskan penggunaan model tambahan yang dihitung terhadap [penggunaan](/docs/id/costs) Anda seperti permintaan Claude lainnya. Review commit bersifat agentic dan mungkin memerlukan beberapa putaran model per commit, dibatasi pada 20 review per jam bergulir. Harapkan kira-kira satu panggilan review per giliran yang mengubah file dan satu review yang lebih dalam per commit, keduanya tunduk pada batas di atas.

Kedua review yang didukung model menggunakan Claude Opus 4.7 secara default. Atur `SECURITY_REVIEW_MODEL` untuk memilih model berbeda untuk review end-of-turn dan `SG_AGENTIC_MODEL` untuk review commit.

Plugin tersedia di semua paket.

<h2 id="disable-or-uninstall">
  Nonaktifkan atau uninstall
</h2>

Untuk mematikan lapisan individual sambil menjaga sisanya, atur variabel lingkungan yang cocok:

| Variable                        | Effect                                                                     |
| :------------------------------ | :------------------------------------------------------------------------- |
| `ENABLE_PATTERN_RULES=0`        | Disable the [per-edit pattern check](#on-each-file-edit)                   |
| `ENABLE_STOP_REVIEW=0`          | Disable the [end-of-turn diff review](#at-the-end-of-each-turn)            |
| `ENABLE_COMMIT_REVIEW=0`        | Disable the [commit and push review](#on-each-commit-or-push-claude-makes) |
| `ENABLE_CODE_SECURITY_REVIEW=0` | Disable all model-backed reviews at once                                   |
| `SECURITY_GUIDANCE_DISABLE=1`   | Disable the plugin entirely without uninstalling                           |

Untuk menjeda plugin dalam lingkup pengguna Anda:

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

Untuk menghapusnya dari lingkup pengguna Anda:

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

Jika plugin diaktifkan melalui `.claude/settings.json` proyek, menonaktifkannya dari `/plugin` menulis override ke `.claude/settings.local.json` Anda daripada mengedit file yang diperiksa, jadi plugin tetap mati untuk Anda sementara rekan kerja tidak terpengaruh. Dialog yang sama juga menawarkan untuk uninstall plugin untuk semua orang dengan menghapusnya dari `.claude/settings.json` bersama; opsi tersebut memerlukan Claude Code v2.1.203 atau lebih baru. Jika diaktifkan melalui [pengaturan terkelola](/docs/id/admin-setup), hanya administrator yang dapat menonaktifkannya.

<h2 id="how-the-plugin-integrates-with-claude-code">
  Bagaimana plugin terintegrasi dengan Claude Code
</h2>

Plugin dibangun sepenuhnya di [hooks](/docs/id/hooks), mekanisme untuk menjalankan kode Anda sendiri pada titik tertentu dalam loop Claude. Ini mendaftarkan:

| Hook event                                                         | Tujuan                                                                                      |
| :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `SessionStart`                                                     | Bootstrap lingkungan Python plugin                                                          |
| `UserPromptSubmit`                                                 | Menangkap baseline working-tree yang digunakan oleh review akhir giliran untuk membuat diff |
| `PostToolUse` pada `Edit`, `Write`, dan `NotebookEdit`             | Pencocokan pola per-edit                                                                    |
| `Stop`                                                             | Review diff akhir giliran, berjalan di latar belakang                                       |
| `PostToolUse` pada `Bash`, disaring ke `git commit` dan `git push` | Review commit dan push, berjalan di latar belakang                                          |

Jika Anda membangun hook Anda sendiri, [sumber plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance) adalah contoh kerja menjalankan panggilan model terpisah dari hook dan memberi makan hasilnya kembali ke sesi.

<h2 id="how-this-fits-with-other-security-tools">
  Bagaimana ini cocok dengan alat keamanan lainnya
</h2>

Plugin adalah satu lapisan dalam pendekatan pertahanan mendalam. Ini menangkap masalah paling awal, saat kode masih dalam editor, tetapi bukan jaminan dan tidak menggantikan pemeriksaan kemudian. Stack tipikal:

| Stage           | Tool                                                      | What it covers                                                                                   |
| :-------------- | :-------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| In session      | Security guidance plugin                                  | Common vulnerabilities in code Claude writes, fixed in the same session                          |
| On demand       | [`/security-review`](/docs/id/commands#all-commands)           | One-time security pass on the current branch, run when you ask                                   |
| On pull request | [Code Review](/docs/id/code-review), Team and Enterprise plans | Multi-agent correctness and security review with full codebase context                           |
| In CI           | Your existing static analysis and dependency scanners     | Language-specific rules, supply-chain checks, and policy enforcement the plugin does not attempt |

Setiap tahap kemudian menangkap apa yang dilewatkan yang sebelumnya. Nilai plugin adalah mengurangi volume yang mencapainya, bukan menghilangkan kebutuhan akan mereka.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Plugin menulis diagnostik runtime ke `~/.claude/security/log.txt`. Periksa di sana terlebih dahulu jika review tidak muncul.

Alasan umum lapisan review melewati tanpa pesan dalam percakapan:

* Direktori bukan repositori git: review end-of-turn dan commit memerlukan status git dan melewati di luar repositori
* Sesi tidak memiliki autentikasi Anthropic: review yang didukung model melewati dan hanya pemeriksaan pola per-edit yang berjalan
* File `security-patterns.yaml` ada tetapi PyYAML tidak dapat diimpor: file diabaikan. Gunakan `security-patterns.json` sebagai gantinya

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk mendalami bagian yang halaman ini sentuh:

* [Code Review](/docs/id/code-review): atur review multi-agent waktu PR
* [Otomatiskan alur kerja dengan hooks](/docs/id/hooks-guide): bangun pemeriksaan Anda sendiri pada titik siklus hidup yang sama
* [Temukan dan instal plugin](/docs/id/discover-plugins#official-anthropic-marketplace): telusuri plugin resmi lainnya
