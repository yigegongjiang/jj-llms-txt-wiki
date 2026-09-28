> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshoot the Agent SDK

> Perbaiki kesalahan Agent SDK ketika Claude Code CLI gagal dimulai, proses CLI keluar, atau hasil yang berhasil tiba tanpa output terstruktur.

Halaman ini mencakup kesalahan Agent SDK dalam startup CLI, exit proses CLI, dan output terstruktur. Entri di halaman ini dikunci ke kesalahan yang Anda lihat. Masing-masing menyebutkan penyebab dan apa yang harus dilakukan.

Gejala yang terikat pada fitur, seperti hook tidak aktif atau skill tidak digunakan, memiliki bagian troubleshooting di halaman fitur tersebut. Tabel menyebutkan bagian atau halaman yang mencakup setiap gejala:

| Gejala                                                                                                                                                                                                                                                                                                                           | Buka                                                                                                                             |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Skills tidak ditemukan, skill tidak digunakan, kesalahan `Invalid skill name`                                                                                                                                                                                                                                                    | [Skills troubleshooting](/docs/id/agent-sdk/skills#troubleshooting)                                                                   |
| Server MCP menunjukkan status `failed`, tools tidak dipanggil, connection timeouts, tool output yang melebihi token maksimal yang diizinkan                                                                                                                                                                                      | [MCP troubleshooting](/docs/id/agent-sdk/mcp#troubleshooting)                                                                         |
| Plugin tidak memuat, plugin skills tidak muncul                                                                                                                                                                                                                                                                                  | [Plugins troubleshooting](/docs/id/agent-sdk/plugins#troubleshooting)                                                                 |
| Claude tidak mendelegasikan ke subagents, filesystem-based agents tidak memuat                                                                                                                                                                                                                                                   | [Subagents troubleshooting](/docs/id/agent-sdk/subagents#troubleshooting)                                                             |
| Opsi checkpointing tidak dikenali, pesan pengguna tanpa UUIDs, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                          | [File checkpointing troubleshooting](/docs/id/agent-sdk/file-checkpointing#troubleshooting)                                           |
| Hook tidak aktif, matcher tidak memfilter seperti yang diharapkan, hook timeout, tool diblokir secara tidak terduga, input yang dimodifikasi tidak diterapkan, session hooks tidak tersedia di Python, subagent permission prompts berlipat ganda, recursive hook loops dengan subagents, `systemMessage` tidak muncul di output | [Fix common issues](/docs/id/agent-sdk/hooks#fix-common-issues) di halaman hooks                                                      |
| Agent yang bekerja di mesin Anda gagal dalam layanan yang diterapkan atau container                                                                                                                                                                                                                                              | [Troubleshoot deployment failures](/docs/id/agent-sdk/hosting#troubleshoot-deployment-failures)                                       |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                               | [Error reference](/docs/id/errors#find-your-error)                                                                                    |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` adalah `None`                                                                                                                                                    | [CLI startup](#cli-startup), [CLI process exit](#cli-process-exit), dan [Structured outputs](#structured-outputs) di halaman ini |

<h2 id="cli-startup">
  CLI startup
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

Python SDK meluncurkan Claude Code CLI sebagai subprocess. Ketika tidak dapat menemukan executable `claude`, koneksi gagal dengan `CLINotFoundError`:

```
Claude Code not found at: /your/configured/path
```

Pesan mencakup jalur yang dikonfigurasi ketika Anda menetapkan `ClaudeAgentOptions(cli_path=...)` dan menunjuk ke file yang hilang. Tanpa `cli_path`, SDK mencari `PATH` Anda dan lokasi instalasi umum, dan pesan mencakup instruksi instalasi untuk platform Anda.

Untuk memperbaikinya:

* Instal Claude Code jika belum diinstal. Lihat [Install Claude Code](/docs/id/setup#install-claude-code) untuk perintah di platform Anda.
* Jika Anda menetapkan `cli_path`, konfirmasi file ada dan merupakan executable `claude`.
* Jika Anda mengandalkan resolusi `PATH`, konfirmasi `claude --version` berfungsi di lingkungan yang sama tempat aplikasi Anda berjalan. Proses yang Anda luncurkan di luar shell Anda, seperti dari IDE atau manajer layanan, sering kali berjalan dengan `PATH` yang berbeda.

TypeScript SDK mencari CLI di paket platform bundel-nya dan jalur yang Anda tetapkan di `pathToClaudeCodeExecutable`. Cocokkan pesan yang Anda lihat:

* `Native CLI binary for <platform>-<arch> not found`: paket platform bundel hilang, paling sering karena instalasi melewatkan dependensi opsional. Instal ulang `@anthropic-ai/claude-agent-sdk` tanpa melewatkan dependensi opsional, atau arahkan `pathToClaudeCodeExecutable` ke [instalasi native](/docs/id/setup#install-claude-code). Dalam executable file tunggal yang dibangun dengan `bun build --compile`, pesan yang sama memiliki penyebab dan solusi yang berbeda. Lihat [Compile to a single executable](/docs/id/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` atau `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: file di jalur yang diselesaikan hilang, atau proses tidak dapat mengaksesnya. Konfirmasi file ada di jalur itu dan bahwa proses dapat mengaksesnya.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

Di Windows, koneksi gagal dengan `CLIConnectionError` ketika jalur CLI yang digunakan Python SDK adalah skrip batch `.bat` atau `.cmd`, termasuk shim `claude.cmd` yang dibuat instalasi npm:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

Penolakan ini adalah pengerasan keamanan yang disengaja, bukan instalasi yang rusak. Windows menjalankan skrip batch dengan menulis ulang spawn menjadi invokasi `cmd.exe /c`, dan `cmd.exe` mem-parse ulang seluruh baris perintah pada waktu eksekusi, jadi nilai argumen dapat menjalankan perintah yang disuntikkan.

Sebagian besar instalasi Windows tidak pernah mencapai kesalahan ini. Wheel Windows x64 dari `claude-agent-sdk` membundel `claude.exe`, dan SDK lebih memilih CLI bundel, kemudian executable `claude.exe` native apa pun yang dapat ditemukannya, sebelum kembali ke shim batch. Anda melihat penolakan dalam dua kasus:

* Anda menetapkan `ClaudeAgentOptions(cli_path=...)` ke file `.bat` atau `.cmd`, seperti shim `claude.cmd` npm.
* Instalasi Anda tidak memiliki `claude.exe` bundel atau native, misalnya instalasi sumber di ARM64 Windows di mana satu-satunya `claude` di `PATH` Anda adalah shim npm.

Untuk memperbaikinya, berikan SDK executable native alih-alih skrip batch:

* Jika Anda menetapkan `ClaudeAgentOptions(cli_path=...)`, arahkan ke `claude.exe` atau hapus opsi. SDK melewatkan penemuan saat `cli_path` diatur, jadi instalasi native saja tidak dapat berlaku.
* Instal Claude Code secara native di PowerShell: `irm https://claude.ai/install.ps1 | iex`
* Di Windows x64, instal wheel `claude-agent-sdk`, yang membundel `claude.exe`.

Sebelum `claude-agent-sdk` 0.2.124, Python SDK menjalankan skrip batch melalui `cmd.exe` tanpa pemeriksaan ini.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

SDK menemukan file di jalur yang diselesaikan tetapi tidak dapat meluncurkannya. Python menaikkan kegagalan ini sebagai `CLIConnectionError`. TypeScript menolak iterasi pesan dengan kesalahan yang tidak membawa kelas SDK. Tabel di bawah memetakan setiap pesan ke apa yang diberitahukannya. Cocokkan pesan yang Anda lihat:

| Pesan                                                             | SDK        | Apa yang diberitahukannya                                             |
| ----------------------------------------------------------------- | ---------- | --------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | Sisa pesan adalah kesalahan sistem operasi itu sendiri                |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | Skrip di jalur yang dikonfigurasi tidak dapat dijalankan              |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | Binary tidak dapat dijalankan, dengan saran libc ditambahkan ke pesan |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Kegagalan peluncuran lainnya                                          |

Di kedua SDK, penyebab umum adalah jalur yang diselesaikan yang menunjuk ke sesuatu yang tidak dapat dijalankan, seperti file teks, direktori, atau file tanpa izin eksekusi. Baca saran libc pesan binary native sebagai salah satu kemungkinan penyebab.

Untuk memperbaikinya di salah satu SDK:

* Konfirmasi jalur yang dikonfigurasi menunjuk ke executable `claude` itu sendiri dan bahwa file memiliki izin eksekusi.
* Jika Anda tidak memerlukan jalur kustom, hapus `cli_path` di Python atau `pathToClaudeCodeExecutable` di TypeScript sehingga SDK menemukan CLI sendiri, lebih memilih salinan bundel-nya.
* Ketika binary yang gagal adalah salinan bundel SDK dalam image kontainer, instal ulang SDK selama build image sehingga binary bundel cocok dengan platform kontainer, atau bangun ulang image untuk arsitektur yang dijalankannya. Penyebab umum adalah binary yang tidak cocok dengan arsitektur atau libc kontainer, atau yang kehilangan izin eksekusi dalam build image.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Memanggil metode `ClaudeSDKClient` di Python sebelum klien terhubung, atau setelah terputus, menaikkan `CLIConnectionError` dengan pesan ini:

```
Not connected. Call connect() first.
```

Lakukan apa yang dikatakan pesan. Baik panggil `await client.connect()` sebelum metode klien lainnya, atau buka klien dengan `async with ClaudeSDKClient() as client:`, yang terhubung saat masuk.

<h2 id="cli-process-exit">
  CLI process exit
</h2>

Entri di bagian ini berarti proses Claude Code berakhir saat aplikasi Anda menggunakannya. Kesalahan mana yang Anda lihat tergantung pada bahasa SDK dan apakah CLI melaporkan hasil kesalahan sebelum keluar.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

Python SDK menaikkan `ProcessError` ketika proses Claude Code keluar dengan kode bukan nol:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

Pesan menyatakan kode keluar dua kali, dan baris `Error output` adalah teks tetap daripada output kesalahan proses Anda. Teks tetap yang sama mengisi atribut `stderr` pengecualian. Atribut `exit_code` pengecualian membawa kode. Untuk menangkap apa yang benar-benar ditulis CLI ke stderr, berikan callback `stderr` di `ClaudeAgentOptions` dan catat apa yang diterimanya.

`ProcessError` telanjang berarti CLI keluar tanpa melaporkan hasil kesalahan. Ketika CLI melaporkan satu, SDK menaikkan [`ResultError`](/docs/id/agent-sdk/python#resulterror) sebagai gantinya, tercakup dalam [Claude Code returned an error result](#claude-code-returned-an-error-result). `ResultError` subkelas `ProcessError`, jadi `except ProcessError` menangkap keduanya. Untuk menanganinya secara berbeda, letakkan klausa `except ResultError` terlebih dahulu.

Sebelum `claude-agent-sdk` 0.2.140, Python SDK menaikkan keluar hasil kesalahan sebagai `Exception` biasa daripada `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

Pembungkus IDE juga mencetak pesan ini, dan [referensi kesalahan](/docs/id/errors#claude-code-process-exited-with-code-n) mencakupnya untuk VS Code dan peluncur lainnya. Entri ini mencakup apa yang diterima kode TypeScript SDK Anda. SDK menampilkan keluar CLI bukan nol sebagai `Error` biasa yang menolak loop `for await` atas pesan `query()`. Tidak ada kelas kesalahan SDK untuk ditangkap, jadi bungkus loop dalam `try`/`catch` dan cocokkan pada pesan:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Ketika CLI menulis ke stderr, pesan berakhir dengan ekor itu. Untuk menangkap aliran penuh, berikan callback `stderr` dalam opsi query. Proses yang dibunuh oleh sinyal melaporkan `Claude Code process terminated by signal <name>` dalam bentuk yang sama.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Kedua SDK mengganti kesalahan keluar proses dengan pesan ini ketika CLI melaporkan hasil kesalahan sebelum keluar:

```
Claude Code returned an error result: <the CLI's own error report>
```

Teks setelah titik dua adalah laporan CLI tentang apa yang salah, jadi mulai dari sana daripada dengan keluar itu sendiri. Python menaikkan ini sebagai [`ResultError`](/docs/id/agent-sdk/python#resulterror), yang atribut `data`-nya membawa hasil kesalahan penuh. TypeScript menolak loop pesan dengan `Error` biasa yang membawa bentuk pesan yang sama.

<h2 id="structured-outputs">
  Structured outputs
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output is None but the result says success
</h3>

Pesan hasil dapat berakhir dengan `subtype: "success"` sementara `structured_output` adalah `None` di Python atau `undefined` di TypeScript. Jalankan selesai, tetapi tidak ada output yang divalidasi. Salah satu cara untuk mencapai ini adalah skema yang tidak dapat dipenuhi output apa pun, misalnya batasan panjang yang bertentangan. Jalankan berakhir tanpa kesalahan validasi, dan satu-satunya sinyal adalah `structured_output` yang hilang.

Perlakukan hasil ini sebagai kegagalan dalam kode aplikasi. Periksa baik bahwa `subtype` adalah `success` dan bahwa `structured_output` ada sebelum menggunakannya. Bagian [Error handling](/docs/id/agent-sdk/structured-outputs#error-handling) menunjukkan pola ini untuk kedua SDK.

Jika terjadi berulang kali dengan skema yang Anda percaya benar, verifikasi skema dapat dipenuhi, kemudian sederhanakan sampai output divalidasi, dan perkenalkan kembali batasan satu per satu.

<h2 id="report-a-new-issue">
  Report a new issue
</h2>

Jika kesalahan Anda tidak tercakup di sini, periksa masalah terbuka atau ajukan yang baru di repositori SDK: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) atau [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Sertakan teks kesalahan penuh dan versi SDK Anda.
