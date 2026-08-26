> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pilih lingkungan sandbox

> Bandingkan opsi sandbox Claude Code: alat Bash bersandbox bawaan, runtime sandbox, dev container, Docker, dan VM. Pilih isolasi yang tepat untuk model ancaman Anda.

Mengisolasi Claude Code membatasi apa yang dapat dibaca, ditulis, dan dijangkau sesi pada jaringan. Hal ini paling penting ketika Anda membiarkan Claude bekerja dengan lebih sedikit prompt izin, menjalankannya tanpa pengawasan, atau mengarahkannya ke kode yang tidak sepenuhnya Anda percayai.

Claude Code dapat berjalan di beberapa jenis lingkungan terisolasi, mulai dari sandbox per-perintah ringan hingga mesin virtual yang sepenuhnya terpisah. Halaman ini membandingkan mereka berdasarkan apa yang mereka isolasi dan apa yang mereka butuhkan, membantu Anda memilih satu untuk model ancaman Anda, dan menunjukkan cara menegakkan pilihan itu di seluruh organisasi.

<Info>
  Untuk model keamanan yang lebih luas, lihat [Security](/docs/id/security). Untuk penerapan Agent SDK, lihat [Secure deployment](/docs/id/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Bandingkan pendekatan sandboxing
</h2>

Dua pendekatan pertama dalam tabel di bawah berjalan pada sistem operasi host tanpa container. Sisanya menempatkan Claude Code di dalam container atau mesin virtual.

| Pendekatan                                        | Apa yang diisolasi                                                    | Memerlukan Docker | Upaya pengaturan                                  |
| :------------------------------------------------ | :-------------------------------------------------------------------- | :---------------- | :------------------------------------------------ |
| [Sandboxed Bash tool](#sandboxed-bash-tool)       | Perintah Bash dan proses anak mereka                                  | Tidak             | Minimal di macOS; rendah di Linux dan WSL2        |
| [Sandbox runtime](#sandbox-runtime)               | Seluruh proses Claude Code, termasuk alat file, server MCP, dan hooks | Tidak             | Rendah                                            |
| [Dev container](#dev-containers)                  | Lingkungan pengembangan lengkap                                       | Ya                | Sedang                                            |
| [Custom container](#custom-container)             | Lingkungan pengembangan lengkap                                       | Ya                | Sedang hingga tinggi                              |
| [Virtual machine](#virtual-machine)               | Sistem operasi lengkap                                                | Tidak             | Tinggi                                            |
| [Claude Code on the web](#claude-code-on-the-web) | Sistem operasi lengkap, dihosting oleh Anthropic                      | Tidak             | Tidak ada; memerlukan langganan Claude dan GitHub |

[Sandboxed Bash tool](/docs/id/sandboxing) bawaan di Claude Code dan hanya membatasi perintah Bash. Alat file bawaan, server MCP, dan hooks masih berjalan langsung di host Anda. Setiap pendekatan lain dalam tabel menempatkan seluruh proses Claude Code di dalam batas isolasi, sehingga alat file, server MCP, dan hooks juga dibatasi.

<Warning>
  Isolasi sandbox mengurangi dampak pelanggaran, tetapi tidak menghilangkan risiko. Pendekatan apa pun yang memungkinkan egress jaringan masih dapat membocorkan data yang dapat dibaca agen, dan pendekatan apa pun yang memasang direktori proyek Anda yang dapat ditulis masih dapat memodifikasi kode tersebut. Tinjau [batasan keamanan](/docs/id/sandboxing#security-limitations) sebelum mengandalkan sandbox sebagai kontrol keras.

  Isolasi juga tidak mengubah apa yang dikirim ke model. Prompt Anda dan file yang dibaca Claude ditransmisikan ke API Anthropic atau penyedia yang dikonfigurasi dengan atau tanpa sandbox. Lihat [Data usage](/docs/id/data-usage) untuk apa yang dikirim Claude Code dan cara menguranginya.
</Warning>

<h2 id="choose-an-approach">
  Pilih pendekatan
</h2>

Cocokkan tujuan Anda dengan baris di bawah, kemudian baca bagian detail yang mengikuti.

| Anda ingin                                                                                         | Mulai dengan                                                                                                                                       |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mengurangi prompt izin selama pekerjaan sehari-hari di mesin Anda sendiri                          | [Sandboxed Bash tool](/docs/id/sandboxing), diaktifkan dengan `/sandbox`                                                                                |
| Biarkan Claude bekerja tanpa pengawasan dengan `--dangerously-skip-permissions` atau mode otomatis | [Dev container](/docs/id/devcontainer) yang telah dikonfigurasi sebelumnya, container atau VM apa pun, atau [sandbox runtime](#sandbox-runtime)         |
| Isolasi server MCP dan hooks serta Bash, tanpa Docker                                              | Runtime sandbox                                                                                                                                    |
| Bekerja pada repositori yang tidak terpercaya                                                      | Mesin virtual khusus, atau [Claude Code on the web](/docs/id/claude-code-on-the-web) jika Anda memiliki langganan Claude dan akun GitHub yang terhubung |
| Standardisasi lingkungan bersandbox di seluruh tim                                                 | [Dev container](/docs/id/devcontainer) yang telah dikonfigurasi sebelumnya, disalin ke repositori Anda                                                  |
| Gunakan Claude Code dari perangkat tanpa pengaturan lokal                                          | [Claude Code on the web](/docs/id/claude-code-on-the-web), yang memerlukan langganan Claude dan akun GitHub yang terhubung                              |
| Memerlukan isolasi untuk setiap pengembang di organisasi Anda                                      | [Enforce isolation across an organization](#enforce-isolation-across-an-organization)                                                              |
| Bekerja pada host Windows asli                                                                     | Container atau VM, atau jalankan sandbox Bash di dalam WSL2                                                                                        |

<h3 id="how-isolation-relates-to-permission-modes">
  Bagaimana isolasi berhubungan dengan mode izin
</h3>

[Mode izin](/docs/id/permission-modes) memutuskan apakah panggilan alat berjalan dan apakah Anda diminta terlebih dahulu. Isolasi membatasi apa yang dapat diakses perintah setelah berjalan. Keduanya bekerja bersama: ketika mode izin membiarkan tindakan berjalan tanpa bertanya kepada Anda, batas isolasi membatasi apa yang dapat dijangkau tindakan tersebut.

Ketika Anda melewatkan `--dangerously-skip-permissions`, Claude bertindak tanpa bertanya kepada Anda terlebih dahulu; Anda hanya diminta untuk [aturan ask](/docs/id/permissions#manage-permissions) yang eksplisit, alat konektor [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan penghapusan yang menargetkan `/` atau direktori home Anda. Tanpa prompt untuk menangkap kesalahan, batas isolasi yang Anda pilih adalah apa yang melindungi sistem Anda. Selalu jalankan sesi `--dangerously-skip-permissions` di dalam container, VM, atau [sandbox runtime](#sandbox-runtime), sehingga alat file, server MCP, dan hooks juga berada di dalam batas.

[Mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) menggantikan prompt dengan pengklasifikasi yang meninjau tindakan dan memblokir yang melampaui permintaan, menargetkan infrastruktur yang tidak dikenali, atau tampak didorong oleh konten bermusuhan yang dibaca Claude. Pengklasifikasi adalah kontrol per-tindakan, bukan batas isolasi, sehingga batas isolasi masih menambah pertahanan berlapis untuk berjalan tanpa pengawasan, dan tidak diperlukan seperti halnya untuk `--dangerously-skip-permissions`.

[Sandboxed Bash tool](#sandboxed-bash-tool) sendiri hanya membatasi Bash, sehingga tidak cukup untuk berjalan sepenuhnya tanpa pengawasan di kedua mode. Anda dapat melapisi pendekatan: menjalankan sandboxed Bash tool di dalam container atau VM memberi Anda pembatasan perintah tingkat OS di atas batas lingkungan luar. Untuk bagaimana sandbox Bash itu sendiri berinteraksi dengan aturan izin dan mode, lihat [How sandboxing relates to permissions and permission modes](/docs/id/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Sandboxed Bash tool
</h2>

<Note>
  Opsi ini tidak mendukung Windows asli. Pada host Windows, gunakan WSL2 atau salah satu pendekatan container atau VM di bawah.
</Note>

Sandboxed Bash tool bawaan di Claude Code. Ini menggunakan primitif sistem operasi untuk membatasi akses filesystem dan jaringan dari setiap perintah Bash yang dijalankan Claude: Seatbelt, sandbox macOS bawaan, dan [bubblewrap](https://github.com/containers/bubblewrap) di Linux dan WSL2. Secara default, ini memungkinkan penulisan ke direktori kerja dan meminta pertama kali perintah memerlukan domain jaringan baru.

Aktifkan dengan perintah `/sandbox`. Panduan [Sandboxing](/docs/id/sandboxing) mencakup mode persetujuan, batas default, dan cara memperluas atau mempersempit.

Sandbox per-perintah tidak mencakup semua yang berjalan dalam sesi:

* [Alat bawaan](/docs/id/tools-reference) lainnya seperti Read, Edit, dan WebFetch berjalan di dalam proses Claude Code dan tidak menjalankan kode arbitrer. [Aturan izin](/docs/id/permissions) untuk path atau domain membatasi mereka sebagai gantinya.
* Server [MCP](/docs/id/mcp) dan hooks adalah proses terpisah yang berjalan tanpa batasan pada host.

Untuk menempatkan alat bawaan, server MCP, dan hooks semua di belakang satu batas OS, jalankan seluruh proses Claude Code di dalam [sandbox runtime](#sandbox-runtime), [dev container](#dev-containers), atau [custom container](#custom-container).

<h2 id="sandbox-runtime">
  Sandbox runtime
</h2>

Paket [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) membungkus seluruh proses dalam isolasi Seatbelt atau bubblewrap yang sama yang digunakan sandbox Bash bawaan. Menjalankan Claude Code melaluinya membatasi setiap alat, hook, dan server MCP dalam sesi, bukan hanya Bash. Runtime adalah pratinjau penelitian beta, dan format konfigurasinya mungkin berubah seiring paket berkembang.

Runtime menolak semua akses tulis dan jaringan secara default, jadi konfigurasikan sebelum meluncurkan Claude Code melaluinya. Di `~/.srt-settings.json`, atau file yang Anda berikan dengan `--settings`, izinkan akses tulis ke setidaknya direktori proyek Anda dan jalur konfigurasi Claude Code `~/.claude` dan `~/.claude.json`. Izinkan domain jaringan yang dibutuhkan sesi Anda, termasuk `api.anthropic.com` atau endpoint penyedia yang dikonfigurasi. Lihat [README](https://github.com/anthropic-experimental/sandbox-runtime) paket untuk skema konfigurasi lengkap.

Setelah file pengaturan sudah ada, luncurkan Claude Code dengan `npx` dan berikan `claude` sebagai perintah untuk dibungkus:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code dimulai di dalam sandbox dengan batas filesystem dan jaringan yang Anda konfigurasikan. Perintah yang sama berfungsi untuk sandboxing server MCP mandiri atau proses pembantu lainnya.

<h2 id="dev-containers">
  Dev containers
</h2>

Dev container menjalankan Claude Code di dalam Docker container yang dikelola VS Code atau editor kompatibel, dengan proyek Anda dipasang di dalamnya. Anda dapat menentukan milik Anda sendiri dengan direktori `.devcontainer/` di repositori Anda.

Repositori claude-code menerbitkan [contoh dev container](/docs/id/devcontainer) dengan firewall iptables default-deny sebagai titik awal. Salin ke repositori Anda dan sesuaikan daftar allowlist firewall, gambar dasar, dan versi Claude Code yang disematkan agar sesuai dengan lingkungan Anda. Karena firewall memblokir egress yang tidak disetujui, konfigurasi seperti ini mendukung menjalankan Claude Code dengan `--dangerously-skip-permissions` untuk pekerjaan tanpa pengawasan.

<h2 id="custom-container">
  Custom container
</h2>

Anda dapat menjalankan Claude Code di gambar Docker atau OCI container apa pun dengan kebijakan jaringan Anda sendiri, volume yang dipasang, dan profil seccomp. Ini adalah jalur paling umum untuk organisasi dengan infrastruktur container yang ada atau runner CI.

Beberapa layanan sandbox terkelola dan eksekusi jarak jauh dapat menampung container untuk Anda. Daftar periksa yang sama berlaku seperti untuk container apa pun yang Anda operasikan: tinjau apa yang dipasang dapat ditulis, kredensial dan token apa yang dapat dijangkau di dalamnya, dan apa yang diizinkan kebijakan egress jaringan.

Anda dapat melapisi sandbox Bash bawaan di dalam container untuk pembatasan per-perintah. Container yang tidak istimewa memerlukan pengaturan nested-sandbox yang dijelaskan dalam [Sandboxing troubleshooting](/docs/id/sandboxing#troubleshooting).

<h2 id="virtual-machine">
  Virtual machine
</h2>

Mesin virtual khusus menyediakan pemisahan terkuat, dengan kernel sendiri dan, dalam penerapan cloud atau microVM, hardware virtual sendiri. Opsi termasuk instance cloud, hypervisor lokal, dan microVM seperti Firecracker.

Gunakan pendekatan ini ketika Anda mengevaluasi kode yang tidak terpercaya, ketika kebijakan keamanan Anda memerlukan pemisahan tingkat kernel antara agen dan host, atau ketika tidak ada pendekatan tingkat host yang memenuhi persyaratan kepatuhan Anda. Fitur [sandboxes](https://docs.docker.com/ai/sandboxes/) Docker Desktop menyediakan microVM dengan daemon Docker sendiri dan sinkronisasi workspace, yang dapat menjalankan Claude Code pada host yang sudah memiliki Docker Desktop.

<h2 id="claude-code-on-the-web">
  Claude Code on the web
</h2>

[Claude Code on the web](/docs/id/claude-code-on-the-web) menjalankan setiap sesi dalam mesin virtual terisolasi yang dikelola Anthropic. Proxy jaringan menerapkan daftar allowlist default, dan proxy terpisah menyimpan token GitHub Anda di luar sandbox sambil mengeluarkan kredensial berscopeduntuk akses repositori di dalamnya.

Gunakan pendekatan ini ketika Anda menginginkan isolasi VM penuh tanpa menyediakan infrastruktur sendiri, atau ketika Anda mendelegasikan tugas dari perangkat yang tidak memiliki lingkungan pengembangan lokal. Ini memerlukan langganan Claude dan akun GitHub yang terhubung, dan sesi mengkloning repositori Anda dari GitHub. Lihat [Claude Code on the web](/docs/id/claude-code-on-the-web) untuk ketersediaan paket dan opsi autentikasi GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Enforce isolation across an organization
</h2>

Pengembang individual dapat memilih pendekatan apa pun di atas. Apa yang dapat diterapkan organisasi, dan dengan alat mana, tergantung pada pendekatan:

* **Built-in Bash sandbox**: satu-satunya pendekatan yang diterapkan Claude Code sendiri. Berikan kunci pengaturan `sandbox` melalui [managed settings](/docs/id/settings#settings-files), baik sebagai file yang dikelola oleh MDM Anda atau melalui [server-managed settings](/docs/id/server-managed-settings) di Claude.ai. Lihat [Enforce sandboxing with managed settings](/docs/id/sandboxing#enforce-sandboxing-with-managed-settings) untuk kunci yang akan digunakan dan cara mencegah pengembang memperluas kebijakan.
* **Dev containers**: komit [contoh dev container](/docs/id/devcontainer) ke repositori Anda untuk standardisasi lingkungan di seluruh tim. Ini adalah konvensi daripada batas penegakan, karena Claude Code tidak memerlukan container. Jika pengembang tidak boleh dapat menjalankan Claude Code di luar, terapkan dengan alat manajemen perangkat organisasi Anda atau alat allowlisting perangkat lunak.
* **Custom containers and VMs**: distribusikan Claude Code melalui gambar yang disetujui dan gunakan alat manajemen perangkat organisasi Anda atau alat allowlisting perangkat lunak untuk mencegah instalasi di luar.

<h2 id="see-also">
  Lihat juga
</h2>

Halaman-halaman ini mencakup detail konfigurasi dan kebijakan untuk pendekatan di atas.

* [Sandboxing](/docs/id/sandboxing): konfigurasikan alat Bash bersandbox bawaan
* [Dev container](/docs/id/devcontainer): container pengembangan Docker yang telah dikonfigurasi sebelumnya
* [Security](/docs/id/security): model keamanan Claude Code lengkap
* [Secure deployment](/docs/id/agent-sdk/secure-deployment): panduan isolasi untuk aplikasi Agent SDK
* [Settings](/docs/id/settings#sandbox-settings): semua kunci konfigurasi sandbox, termasuk pengiriman pengaturan terkelola
