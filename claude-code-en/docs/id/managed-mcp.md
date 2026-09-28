> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kontrol akses server MCP untuk organisasi Anda

> Batasi server MCP mana yang dapat ditambahkan atau dihubungkan pengguna, atau sediakan server untuk setiap pengguna, dengan file konfigurasi yang dikelola, pengaturan yang dikelola, daftar izin, dan daftar penolakan.

Secara default, siapa pun yang menjalankan Claude Code dapat menghubungkan server [MCP](/docs/id/mcp) apa pun yang mereka pilih. Anthropic meninjau konektor terhadap [kriteria pendaftarannya](https://claude.com/docs/connectors/building/review-criteria) sebelum menambahkannya ke [Direktori Anthropic](https://claude.ai/directory), tetapi tidak melakukan audit keamanan atau mengelola server MCP apa pun. Sebagai administrator, Anda dapat membatasi server mana yang berjalan di organisasi Anda, mulai dari menerapkan set yang disetujui tetap hingga menonaktifkan MCP sepenuhnya, dan Anda dapat menyediakan server untuk setiap pengguna.

Pembatasan ini mencakup server yang dimuat Claude Code sendiri, termasuk konektor yang diambilnya dari claude.ai. Konektor yang dikirimkan aplikasi desktop ke sesi lokal dan SSH-nya tiba dalam proses dan diatur dari pengaturan organisasi claude.ai Anda sebagai gantinya; [Bagaimana konektor mencapai Claude Code](/docs/id/mcp#how-connectors-reach-claude-code) menunjukkan kontrol mana yang berlaku untuk konektor di setiap jenis sesi, termasuk sesi cloud.

Halaman ini mencakup cara untuk:

* [Pilih pola](#choose-a-pattern) yang sesuai dengan seberapa banyak kontrol yang Anda butuhkan
* [Terapkan set server tetap dengan `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), termasuk cara [menonaktifkan MCP sepenuhnya](#disable-mcp-entirely)
* [Sediakan server melalui pengaturan yang dikelola](#provide-servers-through-managed-settings) sementara pengguna menyimpan milik mereka sendiri
* [Kontrol server dengan daftar izin dan daftar penolakan](#policy-based-control-with-allowlists-and-denylists)
* [Beri tahu pengguna apa yang diharapkan](#how-restrictions-appear-to-users) ketika pembatasan memblokir server
* [Pantau server mana yang benar-benar digunakan organisasi Anda](#monitor-mcp-usage)

<Note>
  Halaman [Security](/docs/id/security) mencakup model ancaman MCP dan cara mengevaluasi server sebelum menyetujuinya. [Tentukan apa yang akan diterapkan](/docs/id/admin-setup#decide-what-to-enforce) mencakup pembatasan MCP bersama dengan kontrol administratif lainnya.
</Note>

<h2 id="choose-a-pattern">
  Pilih pola
</h2>

Claude Code mendukung berbagai tingkat pembatasan. Setiap pola menggunakan satu atau lebih mekanisme yang tercakup di bawah: `managed-mcp.json` untuk menerapkan set tetap, pengaturan terkelola `managedMcpServers` untuk menyediakan server bersama dengan yang ditambahkan pengguna, dan `allowedMcpServers`/`deniedMcpServers` untuk memfilter apa yang dikonfigurasi pengguna.

| Pola                       | Apa yang dilakukan                                                                                                                                                                                                                                     | Konfigurasi                                                                                                       |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| **Nonaktifkan MCP**        | Tidak ada server yang dimuat, kecuali [server dalam proses yang didaftarkan oleh aplikasi yang memulai sesi](#exclusive-control-with-managed-mcp-json) dan yang [Anda sediakan melalui `managedMcpServers`](#provide-servers-through-managed-settings) | `managed-mcp.json` dengan peta server kosong                                                                      |
| **Penerapan tetap**        | Setiap pengguna mendapatkan server yang sama dan tidak dapat menambah server lain                                                                                                                                                                      | `managed-mcp.json` dengan server yang Anda inginkan                                                               |
| **Server yang disediakan** | Setiap pengguna mendapatkan server jarak jauh yang Anda daftarkan dan mempertahankan server mereka sendiri                                                                                                                                             | `managedMcpServers` dalam pengaturan terkelola                                                                    |
| **Katalog yang disetujui** | Publikasikan daftar server yang disetujui; pengguna menambahkan yang mereka inginkan, yang lain diblokir                                                                                                                                               | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                          |
| **Hanya server plugin**    | Pengguna tidak dapat menambah server melalui `~/.claude.json` atau `.mcp.json`; server plugin masih dimuat                                                                                                                                             | [`strictPluginOnlyCustomization`](/docs/id/settings-reference#strictpluginonlycustomization) dengan `mcp` dalam daftar |
| **Daftar izin lunak**      | Terapkan daftar izin yang dapat diperluas pengguna dalam pengaturan mereka sendiri                                                                                                                                                                     | `allowedMcpServers` tanpa `allowManagedMcpServersOnly`                                                            |
| **Hanya daftar penolakan** | Blokir server yang diketahui buruk, izinkan yang lain                                                                                                                                                                                                  | `deniedMcpServers`                                                                                                |
| **Tanpa pembatasan**       | Pengguna menambahkan apa pun                                                                                                                                                                                                                           | Jangan terapkan konfigurasi MCP terkelola apa pun                                                                 |

<Note>
  Claude Code tidak memiliki registri server MCP bawaan yang dapat dijelajahi dan diinstal pengguna. Untuk pola katalog yang disetujui, bagikan daftar yang disetujui dan perintah `claude mcp add` di tempat pengguna Anda akan menemukannya, seperti wiki internal, atau distribusikan server sebagai plugin melalui [marketplace plugin terkelola](/docs/id/plugins/org#restrict-what-users-can-install) sehingga pengguna dapat menjelajahi dan menginstalnya dari `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Kontrol eksklusif dengan managed-mcp.json
</h2>

Ketika Anda menerapkan file `managed-mcp.json`, Claude Code hanya memuat server MCP berikut:

* Server yang didefinisikan file
* Server yang Anda [sediakan melalui `managedMcpServers`](#provide-servers-through-managed-settings)
* Server dalam proses yang aplikasi yang memulai sesi mendaftarkan, seperti server Claude Code sendiri dari ekstensi VS Code atau [konektor yang dikirimkan aplikasi desktop](/docs/id/mcp#how-connectors-reach-claude-code)

Pengguna tidak dapat menambah, memodifikasi, atau menggunakan server MCP lainnya, termasuk server yang disediakan plugin dan server yang diteruskan dengan [flag CLI `--mcp-config`](/docs/id/cli-reference#cli-flags). File ini juga menekan konektor claude.ai yang Claude Code ambil sendiri kecuali Anda [mengizinkannya bersama set yang dikelola](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  Terapkan managed-mcp.json
</h3>

`managed-mcp.json` adalah file mandiri, jadi tidak dapat dikirimkan melalui [pengaturan yang dikelola server](/docs/id/server-managed-settings). Untuk mengirimkan server melalui pengaturan yang dikelola sebagai gantinya, tanpa kontrol eksklusif, gunakan [`managedMcpServers`](#provide-servers-through-managed-settings).

Proses apa pun yang dapat menulis ke jalur sistem dengan hak istimewa administrator dapat menerapkan file. Di seluruh armada, itu biasanya melalui alat manajemen perangkat, seperti Jamf atau profil konfigurasi di macOS, Kebijakan Grup atau Intune di Windows, atau manajemen armada pilihan Anda di Linux. Claude Code mencari file di salah satu jalur berikut:

| Platform      | Jalur                                                      |
| :------------ | :--------------------------------------------------------- |
| macOS         | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux dan WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows       | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

File ini menggunakan format yang sama dengan file proyek [`.mcp.json`](/docs/id/mcp#project-scope):

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Autentikasi dengan kredensial per pengguna
</h3>

Pengguna apa pun di mesin dapat membaca file ini, jadi jangan simpan kunci API atau kredensial lainnya di blok `env`. Teruskan kredensial per pengguna dengan salah satu dari ini:

* [Ekspansi `${VAR}`](/docs/id/mcp#environment-variable-expansion-in-mcp-json) untuk membaca rahasia dari lingkungan setiap pengguna.
* [OAuth atau header per pengguna](/docs/id/mcp#authenticate-with-remote-mcp-servers) sehingga setiap pengguna mengautentikasi sebagai diri mereka sendiri.
* [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) untuk menghasilkan kredensial pada waktu koneksi.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Server yang diteruskan dengan `--mcp-config` atau `--strict-mcp-config`
</h3>

Ketika sesi menerima server melalui `--mcp-config` sementara `managed-mcp.json` yang dapat dibaca dan diurai Claude Code diterapkan, apa yang dilihat pengguna berbeda antara workstation dan sesi cloud:

* Di workstation, Claude Code keluar saat startup dengan `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Dalam [sesi cloud](/docs/id/claude-code-on-the-web) di host tempat file diterapkan, seperti [runner yang di-host sendiri](/docs/id/self-hosted-environments-configuration#mcp-servers), Claude Code dimulai dengan server yang dikelola saja dan melewati konektor claude.ai dan server lainnya yang host cloud kirimkan melalui `--mcp-config`. Tidak ada dalam sesi yang memberi tahu pengguna server mana yang ditinggalkan. Claude Code menamakannya dalam peringatan di stderr-nya, yang runner yang di-host sendiri catat pada tingkat log `debug`.

Flag `--strict-mcp-config` meminta untuk mengganti set yang dikelola. Jika pengguna meneruskannya sementara file seperti itu diterapkan, Claude Code keluar saat startup di workstation dan dalam sesi cloud.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Bagaimana allowlist dan denylist berlaku untuk set yang dikelola
</h3>

Denylist dapat lebih lanjut memfilter server di `managed-mcp.json`:

* `deniedMcpServers` berlaku untuk server yang dikelola juga, jadi server yang dikelola yang cocok dengan entri tidak akan dimuat.
* `deniedMcpServers` pengguna sendiri bergabung dari pengaturan mereka, jadi pengguna dapat memblokir server yang dikelola untuk diri mereka sendiri.

`allowedMcpServers` tidak berlaku untuk server di `managed-mcp.json`, dengan satu pengecualian: Claude Code masih memeriksa server yang definisinya menggunakan [ekspansi `${VAR}`](/docs/id/mcp#environment-variable-expansion-in-mcp-json) terhadap allowlist, karena konfigurasi efektif server itu berasal dari lingkungan setiap pengguna daripada dari file saja. Sebelum v2.1.259, setiap server yang dikelola harus melewati allowlist kapan pun satu diatur. Lihat [Bagaimana server dievaluasi](#how-a-server-is-evaluated) untuk bidang mana yang memicu pemeriksaan `${VAR}` dan urutan lengkap pemeriksaan.

Jika Anda menggunakan `allowedMcpServers` untuk mencegah beberapa server `managed-mcp.json` Anda sendiri dari dimuat, server tersebut mulai dimuat pada peluncuran pertama setiap pengguna dari v2.1.259 atau lebih baru kecuali mereka menggunakan ekspansi `${VAR}`, tanpa prompt atau pemberitahuan: hanya `deniedMcpServers` yang masih mengurangi dari server tersebut. Tambahkan entri denylist untuk mereka, atau terapkan `managed-mcp.json` terpisah per grup, sebelum pengguna Anda upgrade.

<h3 id="validate-the-configuration">
  Validasi konfigurasi
</h3>

Untuk mengonfirmasi file berlaku, jalankan dua pemeriksaan pada mesin yang dikelola:

1. `claude mcp list` menampilkan hanya server di `managed-mcp.json`, ditambah yang Anda sediakan melalui `managedMcpServers`. Dua hasil lainnya berarti ada yang salah:
   * Jika server pengguna sendiri masih muncul, Claude Code tidak membaca file, jadi periksa jalurnya dan izin pada direktori induknya.
   * Jika server file tidak muncul dan bagian `MCP config diagnostics` menandai konfigurasi perusahaan sebagai gagal diurai, Claude Code tidak dapat membaca atau mengurai file. Perbaiki kesalahan yang bagian itu namai, kemudian minta pengguna untuk memulai ulang Claude Code.
2. `claude mcp add --transport http test https://example.com/mcp` gagal dengan `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. URL tidak perlu menjadi server nyata, karena pemeriksaan kebijakan menolak perintah sebelum apa pun dihubungi.

<h3 id="disable-mcp-entirely">
  Nonaktifkan MCP sepenuhnya
</h3>

Terapkan `managed-mcp.json` yang berisi peta server kosong untuk memblokir setiap server MCP selain [server dalam proses yang aplikasi yang memulai sesi mendaftarkan](#exclusive-control-with-managed-mcp-json):

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` gagal dengan kesalahan kebijakan perusahaan di atas. Server yang pengguna konfigurasi sebelumnya berhenti dimuat saat mereka memulai sesi berikutnya, tanpa peringatan bahwa kebijakan adalah alasannya. Server yang Anda sediakan melalui `managedMcpServers` masih dimuat di bawah peta kosong, jadi biarkan kunci itu tidak diatur juga untuk menonaktifkan MCP sepenuhnya.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Izinkan konektor claude.ai bersama set yang dikelola
</h3>

Secara default, menerapkan `managed-mcp.json` menekan [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) yang Claude Code ambil sendiri, termasuk konektor yang administrator konfigurasi untuk organisasi di konsol admin claude.ai. Untuk memuat konektor tersebut bersama server di `managed-mcp.json`, atur `"allowAllClaudeAiMcps": true` dalam [sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices).

Dengan pengaturan diaktifkan, Claude Code memuat konektor claude.ai yang sama yang akan dimuat jika `managed-mcp.json` tidak diterapkan. [Allowlist dan denylist](#policy-based-control-with-allowlists-and-denylists) masih berlaku untuk konektor tersebut, jadi Anda dapat memblokir yang spesifik dengan `deniedMcpServers`. Pengaturan hanya mempengaruhi konektor claude.ai yang Claude Code ambil sendiri; server yang disediakan plugin tetap ditekan.

Sesi cloud dan sesi lokal dan SSH aplikasi desktop menerima konektor dengan cara lain, dijelaskan dalam [Bagaimana konektor mencapai Claude Code](/docs/id/mcp#how-connectors-reach-claude-code). `managed-mcp.json` pada host yang menjalankan sesi cloud, seperti [host runner yang di-host sendiri](/docs/id/self-hosted-environments-configuration#mcp-servers), menekan konektor sesi itu terlepas dari apakah Anda mengatur `allowAllClaudeAiMcps`. Tidak ada `managed-mcp.json` yang mencapai konektor yang aplikasi desktop kirimkan ke sesi lokal dan SSH-nya.

Claude Code membaca `allowAllClaudeAiMcps` hanya dari tingkat kebijakan yang dikendalikan admin: pengaturan yang dikelola server, kunci plist yang diterapkan MDM atau kunci registri HKLM, atau file `managed-settings.json` sistem. Menempatkannya dalam pengaturan pengguna atau proyek tidak berpengaruh, jadi pengguna tidak dapat mengaktifkan kembali konektor yang kontrol eksklusif tekan.

<h2 id="provide-servers-through-managed-settings">
  Sediakan server melalui pengaturan terkelola
</h2>

Untuk memberikan setiap pengguna satu set server MCP jarak jauh tanpa mengambil kontrol eksklusif atas MCP, daftarkan mereka di bawah `managedMcpServers` dalam [sumber pengaturan terkelola](/docs/id/admin-setup#decide-how-settings-reach-devices): pengaturan yang dikelola server, [gateway aplikasi Claude](/docs/id/claude-apps-gateway-config#what-goes-in-cli), profil MDM atau kebijakan registry, atau `managed-settings.json`. Pengguna mempertahankan server yang mereka tambahkan sendiri dan menerima server Anda sebagai tambahan. Memerlukan Claude Code v2.1.259 atau lebih baru. Klien yang lebih lama mengabaikan kunci ini.

Nilainya adalah objek yang dikunci berdasarkan nama server. Setiap entri memiliki bentuk yang sama dengan server HTTP atau SSE dalam file [`.mcp.json`](/docs/id/mcp#project-scope) proyek, termasuk anggota `headers` dan `oauth` opsional yang dijelaskan dalam [Autentikasi dengan server MCP jarak jauh](/docs/id/mcp#authenticate-with-remote-mcp-servers). Contoh ini menyediakan server pencarian yang setiap pengguna masuk dengannya menggunakan OAuth, dan server catatan yang mengirimkan header yang dikeluarkan organisasi Anda:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Siapa pun yang dapat membaca pengaturan terkelola di mesin, termasuk pengguna, dapat membaca nilai header yang Anda tetapkan di sini. Gunakan kredensial yang dikeluarkan untuk seluruh audiens itu, atau tinggalkan `headers` dan biarkan setiap pengguna masuk dengan OAuth.

<h3 id="what-an-entry-can-contain">
  Apa yang dapat dimuat entri
</h3>

Claude Code memuat entri hanya ketika melewati setiap pemeriksaan di bawah ini. Entri yang gagal satu pemeriksaan dijatuhkan, catatan dicatat yang dapat Anda baca dengan `/status`, dan entri lainnya tetap dimuat:

* `type` adalah `http` atau `sse`. Seperti dalam `.mcp.json`, `streamable-http` diterima sebagai alias untuk `http`.
* `url` adalah URL `https://`. Claude Code menolak URL `http://` biasa, termasuk yang menunjuk ke `localhost`.
* Entri tidak memiliki anggota `command`, `args`, `env`, atau `headersHelper`, jadi dokumen pengaturan terkelola tidak pernah menyebutkan program untuk dijalankan di mesin pengguna.
* Tidak ada nilai yang berisi referensi `${VAR}`. Claude Code tidak memperluas variabel lingkungan dalam entri ini, jadi tulis nilai literal.
* Nama server hanya berisi huruf, angka, tanda hubung, dan garis bawah, dan tidak ada kunci atau nilai yang berisi karakter kontrol atau pemformatan yang tidak terlihat.

Claude Desktop memiliki pengaturan terkelola dengan nama yang sama yang nilainya adalah array dari bentuk entri yang berbeda, jadi jangan salin satu ke yang lain. Claude Code tidak menerima bentuk array dan mencatat catatan alih-alih memuatnya.

Gateway aplikasi Claude menjalankan pemeriksaan yang sama saat boot; lihat [Server MCP dalam kebijakan](/docs/id/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Bagaimana server yang disediakan dimuat
</h3>

Aturan ini menentukan apa yang dimuat ketika server yang disediakan tumpang tindih dengan definisi server lain atau dengan pengaturan lain di halaman ini:

* Server yang disediakan memiliki prioritas atas server dengan nama yang sama dalam cakupan lokal, proyek, atau pengguna, dan atas server plugin atau konektor claude.ai yang menunjuk ke URL yang sama.
* Jika Anda juga menerapkan `managed-mcp.json`, Claude Code memuat server dan server yang disediakan bersama-sama, dan entri file memiliki prioritas ketika keduanya mendefinisikan nama.
* Server yang disediakan terus dimuat ketika [`strictPluginOnlyCustomization`](/docs/id/settings-reference#strictpluginonlycustomization) mengunci permukaan `mcp`.
* `deniedMcpServers` berlaku untuk server yang disediakan, termasuk entri dari pengaturan pengguna mereka sendiri, jadi pengguna dapat memblokir satu untuk diri mereka sendiri. Server yang disediakan tidak memerlukan entri `allowedMcpServers`.

Ketika Anda belum menerapkan `managed-mcp.json`, bendera per-run mempertahankan makna mereka:

* Server yang diteruskan pengguna dengan `--mcp-config` dengan nama yang sama menggantikan yang disediakan untuk run itu dan diperiksa terhadap `allowedMcpServers`.
* `--strict-mcp-config` meninggalkan server yang disediakan bersama dengan setiap server yang dikonfigurasi lainnya.

Dengan `managed-mcp.json` yang diterapkan, kedua bendera berperilaku seperti [Kontrol eksklusif dengan managed-mcp.json](#exclusive-control-with-managed-mcp-json) menjelaskan.

<h3 id="what-users-can-see-and-change">
  Apa yang dapat dilihat dan diubah pengguna
</h3>

Pengguna tidak dapat mengedit atau menghapus server yang disediakan:

* `claude mcp remove` melaporkan bahwa server disediakan oleh organisasi.
* Ketika Anda belum menerapkan `managed-mcp.json`, entri yang ditambahkan pengguna dengan nama yang sama disimpan tetapi tidak digunakan saat server Anda ada.
* Pengguna masih dapat mematikan server yang disediakan untuk diri mereka sendiri di [`/mcp`](/docs/id/mcp#disable-a-server-without-removing-it), yang mencantumkan server yang disediakan di bawah **Managed MCPs**.

`claude mcp get` dan `/mcp` menampilkan URL server yang disediakan hanya sebagai hostnya, misalnya `https://mcp.example.com/…`, dan `claude mcp get` menampilkan nama headernya tanpa nilainya.

<h3 id="where-managedmcpservers-applies">
  Di mana `managedMcpServers` berlaku
</h3>

Claude Code membaca `managedMcpServers` dari sumber terkelola yang dipilihnya di bawah [Bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources). Ketika sumber itu menetapkan [`managedSourcesBehavior`](/docs/id/settings-reference#managedsourcesbehavior) ke `"merge"`, Claude Code menyediakan server dari setiap sumber admin sebagai gantinya, dan ketika dua sumber mendefinisikan nama yang sama, entri sumber yang lebih tinggi peringkatnya berlaku sepenuhnya. Tidak pernah membaca kunci dari registry HKCU yang dapat ditulis pengguna, dari [pengaturan induk yang disediakan host penyematan](/docs/id/managed-settings#parent-settings-from-embedding-hosts), atau dari file pengaturan pengguna, proyek, atau lokal, di mana kunci dijatuhkan dengan peringatan.

Claude Code tidak membaca kunci di tab Kode aplikasi Claude Desktop pada penerapan pihak ketiga atau dalam sesi Cowork aplikasi, karena Claude Desktop menyediakan dan mengunci server MCP sesi tersebut sendiri. `/status` dan `claude doctor` mengatakan demikian ketika pengaturan terkelola Anda membawa kunci di sana.

<h3 id="when-provided-servers-connect">
  Ketika server yang disediakan terhubung
</h3>

Ketika `managedMcpServers` tiba melalui pengaturan yang dikelola server, waktu mengikuti [Perilaku pengambilan dan caching](/docs/id/server-managed-settings#fetch-and-caching-behavior):

* Pada mesin dengan pengaturan cache, Claude Code menahan salinan cache kunci ini sampai server mengonfirmasi pengaturan untuk sesi, dan menunggu konfirmasi itu sebelum memuat server MCP. Jika konfirmasi gagal, sesi berlanjut tanpa server yang disediakan dan `/status` mengatakan mereka ditahan.
* Pada peluncuran pertama mesin, tanpa apa pun yang di-cache, sesi interaktif yang dimulai sebelum pengaturan tiba menghubungkan server yang disediakan segera setelah mereka tiba, dan run `claude -p` yang sudah dimulai dapat selesai tanpanya.

Dengan [sign-in gateway](/docs/id/claude-apps-gateway-config#precedence-with-other-managed-sources), Claude Code memuat kebijakan sebelum sesi dimulai, jadi tidak ada kasus yang menunda atau melewatkan server yang disediakan.

Sesi interaktif yang sudah berjalan menerapkan edit Anda ke kunci:

* **Tambahkan server**: Claude Code menghubungkannya ketika pengaturan yang diperbarui tiba, tanpa restart.
* **Ubah entri server**: sesi tersebut terhubung kembali dengannya dengan definisi baru.
* **Hapus server**: sesi interaktif yang sedang berjalan memutusnya setelah membaca pengaturan yang berubah. Run non-interaktif (`-p`) menyimpannya sampai berakhir.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Kontrol berbasis kebijakan dengan allowlist dan denylist
</h2>

Allowlist dan denylist memfilter server mana yang dikonfigurasi yang diizinkan untuk dimuat. Mereka bukan registri: server masih harus ditambahkan oleh pengguna, plugin, atau organisasi Anda sebelum salah satu daftar berlaku padanya.

Server yang organisasi Anda berikan melalui `managedMcpServers` dimuat tanpa entri allowlist, dan [Bagaimana server dievaluasi](#how-a-server-is-evaluated) mencakup server `managed-mcp.json`. Denylist berlaku untuk setiap server terlepas dari asalnya, kecuali entri `type: "sdk"` dalam proses.

Untuk menerapkan server kepada pengguna, gunakan [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) atau [`managedMcpServers`](#provide-servers-through-managed-settings). Kedua daftar juga memfilter server yang dilewatkan dengan flag CLI [`--mcp-config`](/docs/id/cli-reference#cli-flags), kecuali entri `type: "sdk"` dalam proses; `--strict-mcp-config` membatasi file konfigurasi mana yang dimuat dan tidak melewati salah satu daftar.

Untuk membuat allowlist berwenang, atur `allowedMcpServers` dan `allowManagedMcpServersOnly: true` bersama-sama dalam [sumber pengaturan terkelola](/docs/id/admin-setup#decide-how-settings-reach-devices), seperti pengaturan yang dikelola server atau file `managed-settings.json` yang diterapkan.

Kunci berlaku dari setiap sumber terkelola admin, jadi penguncian dalam file yang diterapkan masih berlaku ketika pengaturan yang dikelola server yang tidak menyebutkan MCP juga digunakan. Saat kunci aktif, allowlist terkelola berasal dari sumber admin dengan peringkat tertinggi yang menetapkan satu. Membaca kunci dan allowlist di seluruh sumber memerlukan Claude Code v2.1.273 atau lebih baru.

[Batasi allowlist ke pengaturan terkelola saja](#restrict-the-allowlist-to-managed-settings-only) menunjukkan konfigurasi.

Tanpa `allowManagedMcpServersOnly`, allowlist dari setiap cakupan pengaturan bergabung, termasuk `~/.claude/settings.json` pengguna sendiri, jadi pengguna dapat memperluas apa yang allowlist Anda izinkan. Denylist bergabung dari setiap cakupan terlepas.

<Note>
  `allowManagedMcpServersOnly` terpisah dari `allowManagedPermissionRulesOnly`, yang mengunci [aturan izin](/docs/id/permissions#managed-settings) saja. Menetapkan flag itu tidak memberlakukan allowlist MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Cocokkan server berdasarkan URL, perintah, atau nama
</h3>

`allowedMcpServers` dan `deniedMcpServers` adalah daftar entri. Setiap entri adalah objek dengan satu kunci yang mengidentifikasi server berdasarkan URL, perintah, atau nama mereka:

| Kunci           | Cocok dengan                                                                   | Gunakan untuk                                      |
| :-------------- | :----------------------------------------------------------------------------- | :------------------------------------------------- |
| `serverUrl`     | URL server jarak jauh, tepat atau dengan wildcard `*`                          | Server HTTP dan SSE                                |
| `serverCommand` | Perintah dan argumen yang tepat yang memulai server stdio                      | Server stdio                                       |
| `serverName`    | Label yang ditetapkan pengguna. Kecocokan tepat saja; wildcard tidak diperluas | Salah satu jenis, tetapi lihat Peringatan di bawah |

Membiarkan `allowedMcpServers` tidak diatur berbeda dari menetapkannya ke array kosong:

| Pengaturan          | Tidak diatur (default)         | Array kosong `[]`                                                                             | Diisi                                                                                           |
| :------------------ | :----------------------------- | :-------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| `allowedMcpServers` | Semua server diizinkan         | Tidak ada server yang diizinkan, terlepas dari [milik organisasi](#how-a-server-is-evaluated) | Hanya server yang cocok diizinkan, terlepas dari [milik organisasi](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | Tidak ada server yang diblokir | Tidak ada server yang diblokir                                                                | Server yang cocok diblokir                                                                      |

Lihat [Entri tidak valid dalam pengaturan terkelola](/docs/id/managed-settings#invalid-entries-in-managed-settings) untuk mengetahui apa yang terjadi ketika entri gagal validasi skema.

<Warning>
  Entri `serverName`, di salah satu daftar, bukan kontrol keamanan. Nama adalah label yang ditetapkan pengguna saat menjalankan `claude mcp add` atau mengedit file konfigurasi, bukan server yang mendasar, jadi pengguna dapat memanggil server apa pun `github`. Untuk konektor claude.ai, nama adalah nama tampilan yang dikembalikan oleh claude.ai, yang dapat berubah. Untuk memberlakukan server mana yang benar-benar berjalan, tambahkan entri `serverCommand` atau `serverUrl`.
</Warning>

Validasi `serverName` berbeda antara dua daftar:

* Di `deniedMcpServers`, `serverName` menerima string apa pun yang tidak kosong tanpa spasi di awal atau akhir, jadi Anda dapat memblokir [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) berdasarkan nama tampilan mereka. Misalnya, `{ "serverName": "claude.ai Slack" }` memblokir konektor Slack. Lebih suka entri `serverUrl` ketika Anda memerlukan penolakan yang kuat terhadap penggantian nama, atau ketika nama konektor bertabrakan dan mendapatkan akhiran ` (N)`.
* Di `allowedMcpServers`, `serverName` terbatas pada huruf, angka, tanda hubung, dan garis bawah. Gunakan `serverUrl` untuk allowlist konektor claude.ai yang Claude Code ambil sendiri; untuk konektor yang host cloud berikan ke sesi yang di-host sendiri, gunakan entri yang tercantum di bawah [Lalu lintas konektor meninggalkan jaringan Anda](/docs/id/self-hosted-environments-deploy#connector-traffic-leaves-your-network) sebagai gantinya.

Untuk mematikan semua konektor claude.ai yang Claude Code ambil sendiri, lihat [`disableClaudeAiConnectors`](/docs/id/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Bagaimana server dievaluasi
</h3>

Sebelum memuat server, termasuk yang dari `managed-mcp.json`, Claude Code menjalankan tiga pemeriksaan di bawah ini secara berurutan. Ini menjalankannya lagi ketika pengguna menghubungkan kembali server atau menghidupkan kembali yang dinonaktifkan di `/mcp`. Server `type: "sdk"` dalam proses, yang [aplikasi yang memulai sesi mendaftarkan](/docs/id/mcp#how-connectors-reach-claude-code), melewati ketiganya.

1. **Gabungkan daftarnya.** Entri allowlist dan denylist dari setiap cakupan pengaturan bergabung menjadi satu allowlist dan satu denylist. Ketika `allowManagedMcpServersOnly` adalah `true`, hanya allowlist terkelola yang disimpan; denylist selalu bergabung dari setiap cakupan. Ketika lebih dari satu sumber terkelola ada, [Kunci dibaca dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source) mengatakan yang mana dari mereka yang memasok daftar cakupan terkelola.
2. **Periksa denylist.** Server yang cocok dengan entri denylist apa pun, berdasarkan URL, perintah, atau nama, diblokir. Tidak ada yang menggantikan kecocokan denylist.
3. **Periksa allowlist.** Jika `allowedMcpServers` tidak diatur di mana pun, setiap server yang melewati denylist dimuat. Jika diatur, apa yang harus cocok dengan server tergantung pada jenisnya, ditampilkan dalam tabel di bawah.

   Server organisasi sendiri melewati pemeriksaan ini: setiap entri `managedMcpServers`, dan entri `managed-mcp.json` apa pun yang nilainya tidak menggunakan ekspansi `${VAR}`. Server bawaan juga melewatinya, seperti Claude di Chrome, server `ide` yang Claude Code sambungkan ke IDE VS Code atau JetBrains yang sedang berjalan, dan server yang CLI sendiri konfigurasi.

   Server `managed-mcp.json` yang menggunakan ekspansi `${VAR}` dalam perintah, argumen, `env`, URL, atau header masih diperiksa, seperti halnya setiap server yang ditambahkan pengguna, plugin, `--mcp-config`, atau claude.ai.

| Jenis server               | Diizinkan ketika cocok                                                                                           |
| :------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| Jarak jauh (HTTP atau SSE) | Entri `serverUrl`. Kecocokan `serverName` hanya dihitung ketika allowlist tidak berisi entri `serverUrl`         |
| Stdio                      | Entri `serverCommand`. Kecocokan `serverName` hanya dihitung ketika allowlist tidak berisi entri `serverCommand` |

Tiga aturan pencocokan berlaku dalam pemeriksaan tersebut:

* **Perintah cocok dengan tepat.** Setiap argumen, secara berurutan. `["npx", "-y", "server"]` tidak cocok dengan `["npx", "server"]` atau `["npx", "-y", "server", "--flag"]`.
* **Nilai `serverCommand` dan `serverUrl` diperluas sebelum pencocokan.** Baik entri kebijakan maupun nilai yang dikonfigurasi server melalui ekspansi [`${VAR}` dan `${VAR:-default}`](/docs/id/mcp#environment-variable-expansion-in-mcp-json), jadi entri yang ditulis sebagai `["${HOME}/bin/server"]` cocok dengan konfigurasi server yang menggunakan referensi yang sama atau jalur yang diperluas. Di Windows, referensikan variabel lingkungan yang diatur di sana, seperti `${USERPROFILE}` bukan `${HOME}`. Nilai `serverName` cocok secara harfiah dan tidak pernah diperluas. Kedua belah pihak membaca lingkungan yang berbeda; [Bagaimana entri kebijakan diperluas](#how-policy-entries-expand) mencakup yang mana, dan bagaimana entri allowlist dan denylist berbeda.
* **URL mendukung wildcard `*`** di mana pun dalam pola, termasuk skema. Pencocokan nama host tidak peka huruf besar-kecil dan mengabaikan titik FQDN yang tertinggal, jadi `https://Mcp.Example.com/*` cocok dengan `https://mcp.example.com/api`. Jalur tetap peka huruf besar-kecil.

| Pola                        | Mengizinkan                                                                 |
| :-------------------------- | :-------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Semua jalur pada domain tertentu                                            |
| `https://mcp.example.com`   | Juga semua jalur di domain itu. Pola tanpa jalur cocok dengan jalur apa pun |
| `https://*.example.com/*`   | Subdomain apa pun dari `example.com`                                        |
| `http://localhost:*/*`      | Port apa pun di localhost                                                   |
| `*://mcp.example.com/*`     | Skema apa pun ke domain tertentu                                            |

<h4 id="how-policy-entries-expand">
  Bagaimana entri kebijakan diperluas
</h4>

Nilai yang dikonfigurasi server diperluas dari lingkungan proses langsung, seperti sisa `.mcp.json`. Entri kebijakan diperluas dari lingkungan yang disematkan sebagai gantinya, jadi variabel yang ditetapkan oleh file pengaturan proyek atau pengguna tidak dapat mengubah apa yang berarti entri allowlist. Karena entri kebijakan masih bergantung pada nilai shell peluncur untuk variabel apa pun yang dirujuknya, gunakan URL dan perintah harfiah untuk entri yang Anda andalkan untuk penegakan.

| Daftar entri        | Diperluas dari                                                                                                                                                                                                     | Ekspansi yang akan mengubah skema, host, atau cakupan jalur entri URL |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| `allowedMcpServers` | Lingkungan yang Claude Code mulai dengan, ditambah nilai `env` dari pengaturan terkelola                                                                                                                           | Claude Code mengabaikan entri                                         |
| `deniedMcpServers`  | Yang sama, dan variabel tanpa nilai startup dan tidak ada `:-default` diisi dari file pengaturan di luar repositori, seperti pengaturan pengguna atau terkelola, yang hanya memperluas apa yang cocok dengan entri | Entri masih cocok                                                     |

Memerlukan Claude Code v2.1.219 atau lebih baru.

<h3 id="example-configuration">
  Contoh konfigurasi
</h3>

Konfigurasi di bawah ini menyiapkan allowlist keras dengan denylist. Baris yang disorot mengubah cara sisa daftar dievaluasi, dan callout setelah blok menjelaskan masing-masing:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Baris 3**: entri `serverUrl` pertama. Setelah satu ada, setiap server jarak jauh harus cocok dengan pola URL, jadi pengguna tidak dapat mendapatkan server jarak jauh yang tidak terdaftar dengan memberikannya nama yang diizinkan.
* **Baris 5**: entri `serverCommand` pertama. Efek yang sama untuk server stdio, jadi setiap server lokal harus cocok dengan perintah yang terdaftar dengan tepat.
* **Baris 11**: entri `serverName` dalam denylist. Entri denylist selalu berlaku, jadi server apa pun yang bernama `dangerous-server` diblokir terlepas dari URL atau perintahnya.

Entri `serverName` dalam allowlist ini tidak akan pernah cocok dengan apa pun, karena kedua jenis transportasi sudah memiliki entri yang lebih ketat.

Accordion di bawah ini menjelaskan bagaimana server dievaluasi terhadap kombinasi allowlist dan denylist lainnya.

<Accordion title="Allowlist hanya URL">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Server                                                | Hasil                                                    |
  | :---------------------------------------------------- | :------------------------------------------------------- |
  | Server HTTP di `https://mcp.example.com/api`          | Diizinkan: cocok dengan pola URL                         |
  | Server HTTP di `https://api.internal.example.com/mcp` | Diizinkan: cocok dengan subdomain wildcard               |
  | Server HTTP di `https://external.example.com/mcp`     | Diblokir: tidak cocok dengan pola URL apa pun            |
  | Server stdio dengan perintah apa pun                  | Diblokir: tidak ada entri nama atau perintah untuk cocok |
</Accordion>

<Accordion title="Allowlist hanya perintah">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                  | Hasil                                      |
  | :------------------------------------------------------ | :----------------------------------------- |
  | Server stdio dengan `["npx", "-y", "approved-package"]` | Diizinkan: cocok dengan perintah           |
  | Server stdio dengan `["node", "server.js"]`             | Diblokir: tidak cocok dengan perintah      |
  | Server HTTP bernama `my-api`                            | Diblokir: tidak ada entri nama untuk cocok |
</Accordion>

<Accordion title="Allowlist nama dan perintah campuran">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                                       | Hasil                                                                        |
  | :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
  | Server stdio bernama `local-tool` dengan `["npx", "-y", "approved-package"]` | Diizinkan: cocok dengan perintah                                             |
  | Server stdio bernama `local-tool` dengan `["node", "server.js"]`             | Diblokir: entri perintah ada tetapi tidak cocok                              |
  | Server stdio bernama `github` dengan `["node", "server.js"]`                 | Diblokir: server stdio harus cocok dengan perintah ketika entri perintah ada |
  | Server HTTP bernama `github`                                                 | Diizinkan: cocok dengan nama                                                 |
  | Server HTTP bernama `other-api`                                              | Diblokir: nama tidak cocok                                                   |
</Accordion>

<Accordion title="Allowlist hanya nama">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Server                                                       | Hasil                                    |
  | :----------------------------------------------------------- | :--------------------------------------- |
  | Server stdio bernama `github` dengan perintah apa pun        | Diizinkan: tidak ada pembatasan perintah |
  | Server stdio bernama `internal-tool` dengan perintah apa pun | Diizinkan: tidak ada pembatasan perintah |
  | Server HTTP bernama `github`                                 | Diizinkan: cocok dengan nama             |
  | Server apa pun bernama `other`                               | Diblokir: nama tidak cocok               |
</Accordion>

<Accordion title="Allowlist dengan penggantian denylist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Server                                           | Hasil                                                                    |
  | :----------------------------------------------- | :----------------------------------------------------------------------- |
  | Server HTTP di `https://mcp.example.com/api`     | Diizinkan: cocok dengan pola URL allowlist, tidak ada kecocokan denylist |
  | Server HTTP di `https://staging.example.com/api` | Diblokir: cocok dengan keduanya, tetapi denylist memiliki prioritas      |
  | Server HTTP di `https://other.com/mcp`           | Diblokir: tidak cocok dengan allowlist                                   |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Batasi allowlist ke pengaturan terkelola saja
</h3>

Untuk membuat allowlist terkelola satu-satunya yang berlaku, atur `allowManagedMcpServersOnly` dalam file pengaturan terkelola:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Ketika `allowManagedMcpServersOnly` adalah `true`, allowlist dari pengaturan pengguna, proyek, dan lokal diabaikan. Denylist masih bergabung dari setiap cakupan pengaturan, jadi pengguna selalu dapat memblokir server untuk diri mereka sendiri.

<h2 id="how-restrictions-appear-to-users">
  Bagaimana pembatasan muncul kepada pengguna
</h2>

Untuk melihat apa yang pengguna lihat saat startup ketika `managed-mcp.json` digunakan dan sesi juga memiliki server `--mcp-config`, lihat [Kontrol eksklusif dengan managed-mcp.json](#exclusive-control-with-managed-mcp-json). Gunakan tabel ini untuk mengenali laporan lainnya dan untuk memberitahu pengguna apa yang diharapkan sebelum Anda meluncurkan perubahan:

| Pembatasan                                                                                                         | Apa yang dilihat pengguna                                                                                                    |
| :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` ada dan pengguna menjalankan `claude mcp add`                                                   | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| Server ada di daftar penolakan dan pengguna menjalankan `claude mcp add`                                           | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| Server tidak ada di daftar izin dan pengguna menjalankan `claude mcp add`                                          | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| Pengguna menjalankan `claude mcp remove` pada server dari `managedMcpServers`                                      | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Server yang sebelumnya dikonfigurasi sekarang diblokir oleh kebijakan                                              | Server menghilang dari `/mcp` dan `claude mcp list`                                                                          |
| Server diblokir saat sesi sedang berjalan, dan pengguna memilih **Reconnect** atau menyalakannya kembali di `/mcp` | [`MCP server <name> is blocked by enterprise managed policy`](/docs/id/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Ketika server menghilang secara diam-diam, pengguna tidak mendapatkan sinyal bahwa kebijakan adalah alasannya, jadi beritahu pengguna yang terkena dampak server mana yang diblokir ketika Anda meluncurkan pembatasan baru.

<h2 id="monitor-mcp-usage">
  Pantau penggunaan MCP
</h2>

Ketika [ekspor OpenTelemetry](/docs/id/monitoring-usage) dikonfigurasi, Claude Code dapat merekam server MCP dan alat mana yang digunakan pengguna. Atur `OTEL_LOG_TOOL_DETAILS=1` untuk menyertakan nama server dan alat MCP dalam acara alat, kemudian agregasikan di kolektor Anda untuk melihat server mana yang benar-benar dihubungkan pengguna Anda. Lihat [Monitoring](/docs/id/monitoring-usage) untuk menyiapkan pengekspor dan untuk skema acara lengkap.

<h2 id="configuration-summary">
  Ringkasan konfigurasi
</h2>

Setiap file dan pengaturan yang dibahas halaman ini, apa yang dikontrolnya, dan cara mengirimkannya:

| Permukaan                    | Apa yang dikontrol                                                                                                                                                                                                                             | Di mana itu berada                                                                                                                                                                                                                                         | Cara mengirimkan                                                                                                                                                                                  |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `managed-mcp.json`           | Set server tetap, kontrol eksklusif                                                                                                                                                                                                            | Jalur sistem: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, atau `C:\Program Files\ClaudeCode\`                                                                                                                                         | MDM, GPO, manajemen armada, atau proses apa pun dengan hak istimewa administrator. Tidak dapat diatur melalui pengaturan yang dikelola server                                                     |
| `managedMcpServers`          | Server jarak jauh yang disediakan untuk setiap pengguna bersama dengan server mereka sendiri                                                                                                                                                   | Hanya sumber pengaturan yang dikelola; pengaturan tidak berpengaruh di tempat lain                                                                                                                                                                         | [Sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices): pengaturan yang dikelola server, kebijakan gateway, `managed-settings.json`, profil MDM, atau registri HKLM |
| `allowedMcpServers`          | Daftar izin server yang diizinkan                                                                                                                                                                                                              | Cakupan [pengaturan](/docs/id/settings#where-settings-live) apa pun; [Cara server dievaluasi](#how-a-server-is-evaluated) menjelaskan bagaimana daftar dari beberapa cakupan dan sumber yang dikelola digabungkan                                               | Untuk penegakan, [sumber pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices): pengaturan yang dikelola server, `managed-settings.json`, profil MDM, atau registri        |
| `deniedMcpServers`           | Daftar penolakan server yang diblokir                                                                                                                                                                                                          | Cakupan pengaturan apa pun; [Cara server dievaluasi](#how-a-server-is-evaluated) menjelaskan bagaimana daftar dari beberapa cakupan dan sumber yang dikelola digabungkan                                                                                   | Sama seperti `allowedMcpServers`                                                                                                                                                                  |
| `allowManagedMcpServersOnly` | Mengunci daftar izin ke sumber yang dikelola saja                                                                                                                                                                                              | Hanya sumber pengaturan yang dikelola; [Kunci yang dibaca dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source) menjelaskan sumber yang dikelola mana yang dapat mengaktifkannya. Pengaturan tidak berpengaruh di cakupan lain | Sama seperti `allowedMcpServers`                                                                                                                                                                  |
| `allowAllClaudeAiMcps`       | Memuat konektor claude.ai yang Claude Code ambil sendiri bersama `managed-mcp.json`. [File `managed-mcp.json` di host yang menjalankan sesi cloud masih menekan konektor sesi tersebut](#allow-claude-ai-connectors-alongside-the-managed-set) | Hanya sumber pengaturan yang dikelola; pengaturan tidak berpengaruh di tempat lain                                                                                                                                                                         | Sama seperti `allowedMcpServers`                                                                                                                                                                  |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Tentukan apa yang akan diterapkan](/docs/id/admin-setup#decide-what-to-enforce): pembatasan MCP bersama dengan aturan izin, sandboxing, dan kontrol admin lainnya
* [Hubungkan Claude Code ke alat melalui MCP](/docs/id/mcp): referensi MCP lengkap, termasuk transportasi, cakupan, dan autentikasi
* [Pengaturan](/docs/id/settings): hierarki pengaturan dan bagaimana pengaturan yang dikelola memiliki prioritas
* [Pengaturan yang dikelola server](/docs/id/server-managed-settings): kirimkan `allowedMcpServers` dan `deniedMcpServers` dari konsol admin Claude.ai
* [Keamanan](/docs/id/security): model ancaman yang dilindungi kontrol ini
* [Panduan Administrator Perusahaan Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, manajemen kursi, dan playbook peluncuran
