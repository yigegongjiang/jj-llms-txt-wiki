> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mengamankan penyebaran agen AI

> Panduan untuk mengamankan penyebaran Claude Code dan Agent SDK dengan isolasi, manajemen kredensial, dan kontrol jaringan

Claude Code dan Agent SDK adalah alat yang ampuh yang dapat menjalankan kode, mengakses file, dan berinteraksi dengan layanan eksternal atas nama Anda. Seperti alat apa pun dengan kemampuan ini, penyebaran yang cermat memastikan Anda mendapatkan manfaat sambil mempertahankan kontrol yang sesuai.

Tidak seperti perangkat lunak tradisional yang mengikuti jalur kode yang telah ditentukan sebelumnya, alat ini menghasilkan tindakan mereka secara dinamis berdasarkan konteks dan tujuan. Fleksibilitas inilah yang membuat mereka berguna, tetapi ini juga berarti perilaku mereka dapat dipengaruhi oleh konten yang mereka proses: file, halaman web, atau input pengguna. Ini kadang-kadang disebut prompt injection. Misalnya, jika README repositori berisi instruksi yang tidak biasa, Claude Code mungkin menggabungkan instruksi tersebut ke dalam tindakannya dengan cara yang tidak diantisipasi oleh operator. Panduan ini mencakup cara praktis untuk mengurangi risiko ini.

Berita baiknya adalah mengamankan penyebaran agen tidak memerlukan infrastruktur yang eksotis. Prinsip yang sama yang berlaku untuk menjalankan kode semi-terpercaya apa pun berlaku di sini: isolasi, privilege minimal, dan pertahanan berlapis. Claude Code mencakup beberapa fitur keamanan yang membantu dengan kekhawatiran umum, dan panduan ini memandu Anda melalui ini bersama dengan opsi pengerasan tambahan bagi mereka yang membutuhkannya.

Tidak setiap penyebaran memerlukan keamanan maksimal. Pengembang yang menjalankan Claude Code di laptop mereka memiliki persyaratan yang berbeda dari perusahaan yang memproses data pelanggan di lingkungan multi-tenant. Panduan ini menyajikan opsi mulai dari fitur keamanan bawaan Claude Code hingga arsitektur produksi yang dikeraskan, sehingga Anda dapat memilih apa yang sesuai dengan situasi Anda.

<h2 id="threat-model">
  Model ancaman
</h2>

Agen dapat mengambil tindakan yang tidak diinginkan karena prompt injection (instruksi yang tertanam dalam konten yang mereka proses) atau kesalahan model. Model Claude dirancang untuk menahan ini; lihat [model overview](https://platform.claude.com/docs/id/about-claude/models/overview) dan kartu sistem untuk model yang Anda sebarkan untuk detail evaluasi.

Pertahanan berlapis masih merupakan praktik yang baik. Misalnya, jika agen memproses file berbahaya yang menginstruksikannya untuk mengirim data pelanggan ke server eksternal, kontrol jaringan dapat memblokir permintaan itu sepenuhnya.

<h2 id="built-in-security-features">
  Fitur keamanan bawaan
</h2>

Claude Code mencakup beberapa fitur keamanan yang mengatasi kekhawatiran umum. Lihat [dokumentasi keamanan](/docs/id/security) untuk detail lengkap.

* **Sistem izin**: Setiap alat dan perintah bash dapat dikonfigurasi untuk mengizinkan, memblokir, atau meminta persetujuan pengguna. Gunakan pola glob untuk membuat aturan seperti "izinkan semua perintah npm" atau "blokir perintah apa pun dengan sudo". Organisasi dapat menetapkan kebijakan yang berlaku di semua pengguna. Lihat [permissions](/docs/id/permissions).
* **Parsing perintah untuk izin**: Sebelum menjalankan perintah bash, Claude Code menguraikannya menjadi AST dan mencocokkan hasilnya dengan aturan izin Anda. Perintah yang tidak dapat diuraikan dengan bersih, atau yang tidak cocok dengan aturan izin, memerlukan persetujuan eksplisit. Serangkaian kecil konstruksi seperti `eval` selalu memerlukan persetujuan terlepas dari aturan izin. Ini adalah gerbang izin, bukan sandbox; ini tidak menyimpulkan apakah perintah berbahaya dari jalur target atau efeknya.
* **Ringkasan pencarian web**: Hasil pencarian diringkas daripada melewatkan konten mentah langsung ke dalam konteks, mengurangi risiko prompt injection dari konten web berbahaya.
* **Mode sandbox**: Perintah Bash dapat berjalan di lingkungan sandbox yang membatasi akses filesystem dan jaringan. Lihat [dokumentasi sandboxing](/docs/id/sandboxing) untuk detail.

<h2 id="security-principles">
  Prinsip keamanan
</h2>

Untuk penyebaran yang memerlukan pengerasan tambahan di luar default Claude Code, prinsip-prinsip ini memandu opsi yang tersedia.

<h3 id="security-boundaries">
  Batas keamanan
</h3>

Batas keamanan memisahkan komponen dengan tingkat kepercayaan yang berbeda. Untuk penyebaran keamanan tinggi, Anda dapat menempatkan sumber daya sensitif (seperti kredensial) di luar batas yang berisi agen. Jika ada yang salah di lingkungan agen, sumber daya di luar batas itu tetap terlindungi.

Misalnya, daripada memberikan agen akses langsung ke kunci API, Anda dapat menjalankan proxy di luar lingkungan agen yang menyuntikkan kunci ke dalam permintaan. Agen dapat melakukan panggilan API, tetapi tidak pernah melihat kredensial itu sendiri. Pola ini berguna untuk penyebaran multi-tenant atau saat memproses konten yang tidak terpercaya.

<h3 id="least-privilege">
  Privilege minimal
</h3>

Jika diperlukan, Anda dapat membatasi agen hanya ke kemampuan yang diperlukan untuk tugas spesifiknya:

| Sumber Daya      | Opsi Pembatasan                                              |
| ---------------- | ------------------------------------------------------------ |
| Filesystem       | Pasang hanya direktori yang diperlukan, lebih suka read-only |
| Jaringan         | Batasi ke endpoint spesifik melalui proxy                    |
| Kredensial       | Suntikkan melalui proxy daripada mengekspos secara langsung  |
| Kemampuan sistem | Lepaskan kemampuan Linux di kontainer                        |

<h3 id="defense-in-depth">
  Pertahanan berlapis
</h3>

Untuk lingkungan keamanan tinggi, melapisi beberapa kontrol memberikan perlindungan tambahan. Opsi termasuk:

* Isolasi kontainer
* Pembatasan jaringan
* Kontrol filesystem
* Validasi permintaan di proxy

Kombinasi yang tepat tergantung pada model ancaman dan persyaratan operasional Anda.

<h2 id="isolation-technologies">
  Teknologi isolasi
</h2>

Teknologi isolasi yang berbeda menawarkan trade-off yang berbeda antara kekuatan keamanan, kinerja, dan kompleksitas operasional.

<Info>
  Dalam semua konfigurasi ini, Claude Code (atau aplikasi Agent SDK Anda) berjalan di dalam batas isolasi (sandbox, kontainer, atau VM). Kontrol keamanan yang dijelaskan di bawah membatasi apa yang dapat diakses agen dari dalam batas itu.
</Info>

| Teknologi              | Kekuatan Isolasi                      | Overhead Kinerja | Kompleksitas  |
| ---------------------- | ------------------------------------- | ---------------- | ------------- |
| Sandbox runtime        | Baik (default aman)                   | Sangat rendah    | Rendah        |
| Kontainer (Docker)     | Tergantung setup                      | Rendah           | Sedang        |
| gVisor                 | Sangat baik (dengan setup yang benar) | Sedang/Tinggi    | Sedang        |
| VM (Firecracker, QEMU) | Sangat baik (dengan setup yang benar) | Tinggi           | Sedang/Tinggi |

<h3 id="sandbox-runtime">
  Sandbox runtime
</h3>

Untuk isolasi ringan tanpa kontainer, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) memberlakukan pembatasan filesystem dan jaringan di tingkat OS.

Keuntungan utama adalah kesederhanaan: tidak ada konfigurasi Docker, citra kontainer, atau setup jaringan yang diperlukan. Proxy dan pembatasan filesystem sudah tertanam. Anda menyediakan file pengaturan yang menentukan domain dan jalur yang diizinkan.

**Cara kerjanya:**

* **Filesystem**: Menggunakan primitif OS (`bubblewrap` di Linux, `sandbox-exec` di macOS) untuk membatasi akses baca/tulis ke jalur yang dikonfigurasi
* **Jaringan**: Menghapus namespace jaringan (Linux) atau menggunakan profil Seatbelt (macOS) untuk merutekan lalu lintas jaringan melalui proxy bawaan
* **Konfigurasi**: Daftar izin berbasis JSON untuk domain dan jalur filesystem

**Setup:**

```bash theme={null}
npm install @anthropic-ai/sandbox-runtime
```

Kemudian buat file konfigurasi yang menentukan jalur dan domain yang diizinkan.

**Pertimbangan keamanan:**

1. **Kernel host yang sama**: Tidak seperti VM, proses sandbox berbagi kernel host. Kerentanan kernel secara teoritis dapat memungkinkan escape. Untuk beberapa model ancaman ini dapat diterima, tetapi jika Anda memerlukan isolasi tingkat kernel, gunakan gVisor atau VM terpisah.

2. **Tidak ada inspeksi TLS**: Proxy daftar izin domain berdasarkan nama host yang disediakan klien dan tidak menghentikan atau memeriksa lalu lintas terenkripsi. Kode yang berjalan di dalam sandbox dapat berpotensi menggunakan [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) atau teknik serupa untuk menjangkau host di luar daftar izin. Jika model ancaman Anda memerlukan jaminan yang lebih kuat, konfigurasikan [proxy yang menghentikan TLS](#traffic-forwarding). Lihat [batasan keamanan sandboxing](/docs/id/sandboxing#security-limitations) untuk detail lebih lanjut. Secara terpisah, jika agen memiliki kredensial permisif untuk domain yang diizinkan, pastikan itu tidak dapat menggunakan domain itu untuk memicu permintaan jaringan lain atau untuk mengeksfiltrasi data.

Untuk banyak kasus penggunaan pengembang tunggal dan CI/CD, sandbox-runtime meningkatkan standar secara signifikan dengan setup minimal. Bagian di bawah mencakup kontainer dan VM untuk penyebaran yang memerlukan isolasi yang lebih kuat.

<h3 id="containers">
  Kontainer
</h3>

Kontainer menyediakan isolasi melalui namespace Linux. Setiap kontainer memiliki pandangan sendiri tentang filesystem, pohon proses, dan stack jaringan, sambil berbagi kernel host.

Konfigurasi kontainer yang dikeraskan keamanan mungkin terlihat seperti ini:

```bash theme={null}
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Berikut adalah apa yang dilakukan setiap opsi:

| Opsi                               | Tujuan                                                                                                                                                                           |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--cap-drop ALL`                   | Menghapus kemampuan Linux seperti `NET_ADMIN` dan `SYS_ADMIN` yang dapat memungkinkan eskalasi privilege                                                                         |
| `--security-opt no-new-privileges` | Mencegah proses mendapatkan privilege melalui binari setuid                                                                                                                      |
| `--security-opt seccomp=...`       | Membatasi syscall yang tersedia; default Docker memblokir \~44, profil kustom dapat memblokir lebih banyak                                                                       |
| `--read-only`                      | Membuat filesystem root kontainer tidak dapat diubah, mencegah agen dari mempertahankan perubahan                                                                                |
| `--tmpfs /tmp:...`                 | Menyediakan direktori sementara yang dapat ditulis yang dihapus saat kontainer berhenti                                                                                          |
| `--network none`                   | Menghapus semua antarmuka jaringan; agen berkomunikasi melalui soket Unix yang dipasang di bawah                                                                                 |
| `--memory 2g`                      | Membatasi penggunaan memori untuk mencegah kehabisan sumber daya                                                                                                                 |
| `--pids-limit 100`                 | Membatasi jumlah proses untuk mencegah fork bombs                                                                                                                                |
| `--user 1000:1000`                 | Berjalan sebagai pengguna non-root                                                                                                                                               |
| `-v ...:/workspace:ro`             | Memasang kode read-only sehingga agen dapat menganalisis tetapi tidak memodifikasinya. **Hindari memasang direktori host sensitif seperti `~/.ssh`, `~/.aws`, atau `~/.config`** |
| `-v .../proxy.sock:...`            | Memasang soket Unix yang terhubung ke proxy yang berjalan di luar kontainer (lihat di bawah)                                                                                     |

**Arsitektur soket Unix:**

Dengan `--network none`, kontainer tidak memiliki antarmuka jaringan sama sekali. Satu-satunya cara bagi agen untuk menjangkau dunia luar adalah melalui soket Unix yang dipasang, yang terhubung ke proxy yang berjalan di host. Proxy ini dapat memberlakukan daftar izin domain, menyuntikkan kredensial, dan mencatat semua lalu lintas.

Ini adalah arsitektur yang sama yang digunakan oleh [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Bahkan jika agen dikompromikan melalui prompt injection, itu tidak dapat mengeksfiltrasi data ke server arbitrer. Itu hanya dapat berkomunikasi melalui proxy, yang mengontrol domain mana yang dapat dijangkau. Untuk detail lebih lanjut, lihat [posting blog sandboxing Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opsi pengerasan tambahan:**

| Opsi             | Tujuan                                                                                                                                                  |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--userns-remap` | Memetakan root kontainer ke pengguna host yang tidak memiliki privilege; memerlukan konfigurasi daemon tetapi membatasi kerusakan dari escape kontainer |
| `--ipc private`  | Mengisolasi komunikasi antar-proses untuk mencegah serangan lintas-kontainer                                                                            |

<h3 id="gvisor">
  gVisor
</h3>

Kontainer standar berbagi kernel host: ketika kode di dalam kontainer membuat system call, itu langsung ke kernel yang sama yang menjalankan host. Ini berarti kerentanan kernel dapat memungkinkan escape kontainer. gVisor mengatasi ini dengan mengintersepsi system call di userspace sebelum mereka mencapai kernel host, mengimplementasikan lapisan kompatibilitas sendiri yang menangani sebagian besar syscall tanpa melibatkan kernel nyata.

Jika agen menjalankan kode berbahaya (mungkin karena prompt injection), kode itu berjalan di kontainer dan dapat mencoba exploit kernel. Dengan gVisor, permukaan serangan jauh lebih kecil: kode berbahaya perlu mengeksploitasi implementasi userspace gVisor terlebih dahulu dan akan memiliki akses terbatas ke kernel nyata.

Untuk menggunakan gVisor dengan Docker, instal runtime `runsc` dan konfigurasikan daemon:

```json theme={null}
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Kemudian jalankan kontainer dengan:

```bash theme={null}
docker run --runtime=runsc agent-image
```

**Pertimbangan kinerja:**

| Beban Kerja           | Overhead                                                |
| --------------------- | ------------------------------------------------------- |
| Komputasi terikat CPU | \~0% (tidak ada intersepsi syscall)                     |
| Syscall sederhana     | \~2× lebih lambat                                       |
| I/O file intensif     | Hingga 10-200× lebih lambat untuk pola open/close berat |

Untuk lingkungan multi-tenant atau saat memproses konten yang tidak terpercaya, isolasi tambahan sering kali sepadan dengan overhead.

<h3 id="virtual-machines">
  Mesin virtual
</h3>

VM menyediakan isolasi tingkat hardware melalui ekstensi virtualisasi CPU. Setiap VM menjalankan kernel sendiri, menciptakan batas yang kuat. Kerentanan di kernel guest tidak secara langsung mengkompromikan host. Namun, VM tidak secara otomatis "lebih aman" daripada alternatif seperti gVisor. Keamanan VM sangat bergantung pada hypervisor dan kode emulasi perangkat.

Firecracker dirancang untuk isolasi microVM yang ringan. Itu dapat boot VM dalam waktu kurang dari 125ms dengan overhead memori kurang dari 5 MiB, menghilangkan emulasi perangkat yang tidak perlu untuk mengurangi permukaan serangan.

Dengan pendekatan ini, VM agen tidak memiliki antarmuka jaringan eksternal. Sebaliknya, itu berkomunikasi melalui `vsock` (soket virtual). Semua lalu lintas merutekan melalui vsock ke proxy di host, yang memberlakukan daftar izin dan menyuntikkan kredensial sebelum meneruskan permintaan.

<h3 id="cloud-deployments">
  Penyebaran cloud
</h3>

Untuk penyebaran cloud, Anda dapat menggabungkan teknologi isolasi apa pun di atas dengan kontrol jaringan asli cloud:

1. Jalankan kontainer agen di subnet pribadi tanpa internet gateway
2. Konfigurasikan aturan firewall cloud (AWS Security Groups, GCP VPC firewall) untuk memblokir semua egress kecuali ke proxy Anda
3. Jalankan proxy (seperti [Envoy](https://www.envoyproxy.io/) dengan filter `credential_injector` nya) yang memvalidasi permintaan, memberlakukan daftar izin domain, menyuntikkan kredensial, dan meneruskan ke API eksternal
4. Tetapkan izin IAM minimal ke akun layanan agen, merutekan akses sensitif melalui proxy jika memungkinkan
5. Catat semua lalu lintas di proxy untuk tujuan audit

<h2 id="credential-management">
  Manajemen kredensial
</h2>

Agen sering memerlukan kredensial untuk memanggil API, mengakses repositori, atau berinteraksi dengan layanan cloud. Tantangannya adalah memberikan akses ini tanpa mengekspos kredensial itu sendiri.

<h3 id="the-proxy-pattern">
  Pola proxy
</h3>

Pendekatan yang direkomendasikan adalah menjalankan proxy di luar batas keamanan agen yang menyuntikkan kredensial ke dalam permintaan keluar. Agen mengirim permintaan tanpa kredensial, proxy menambahkannya, dan meneruskan permintaan ke tujuannya.

Pola ini memiliki beberapa manfaat:

1. Agen tidak pernah melihat kredensial aktual
2. Proxy dapat memberlakukan daftar izin endpoint yang diizinkan
3. Proxy dapat mencatat semua permintaan untuk audit
4. Kredensial disimpan di satu lokasi aman daripada didistribusikan ke setiap agen

<h3 id="configuring-claude-code-to-use-a-proxy">
  Mengonfigurasi Claude Code untuk menggunakan proxy
</h3>

Claude Code mendukung dua metode untuk merutekan permintaan sampling melalui proxy:

**Opsi 1: ANTHROPIC\_BASE\_URL (sederhana tetapi hanya untuk permintaan API sampling)**

```bash theme={null}
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Ini memberitahu Claude Code dan Agent SDK untuk mengirim permintaan sampling ke proxy Anda daripada langsung ke API Claude. Proxy Anda menerima permintaan HTTP plaintext, dapat memeriksa dan memodifikasinya (termasuk menyuntikkan kredensial), kemudian meneruskan ke API nyata.

**Opsi 2: HTTP\_PROXY / HTTPS\_PROXY (system-wide)**

```bash theme={null}
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code dan Agent SDK menghormati variabel lingkungan standar ini, merutekan semua lalu lintas HTTP melalui proxy. Untuk HTTPS, proxy membuat terowongan CONNECT terenkripsi: itu tidak dapat melihat atau memodifikasi konten permintaan tanpa intersepsi TLS.

<h3 id="implementing-a-proxy">
  Mengimplementasikan proxy
</h3>

Anda dapat membangun proxy Anda sendiri atau menggunakan yang sudah ada:

* [Envoy Proxy](https://www.envoyproxy.io/): proxy tingkat produksi dengan filter `credential_injector` untuk menambahkan header auth
* [mitmproxy](https://mitmproxy.org/): proxy yang menghentikan TLS untuk memeriksa dan memodifikasi lalu lintas HTTPS
* [Squid](http://www.squid-cache.org/): proxy caching dengan daftar kontrol akses
* [LiteLLM](https://github.com/BerriAI/litellm): gateway LLM dengan injeksi kredensial dan pembatasan laju

<h3 id="credentials-for-other-services">
  Kredensial untuk layanan lain
</h3>

Di luar sampling dari API Claude, agen sering memerlukan akses terautentikasi ke layanan lain, seperti repositori git, database, dan API internal. Ada dua pendekatan utama:

<h4 id="custom-tools">
  Alat kustom
</h4>

Sediakan akses melalui server MCP atau alat kustom yang merutekan permintaan ke layanan yang berjalan di luar batas keamanan agen. Agen memanggil alat, tetapi permintaan terautentikasi aktual terjadi di luar. Panggilan alat ke proxy yang menyuntikkan kredensial.

Misalnya, server MCP git dapat menerima perintah dari agen tetapi meneruskannya ke proxy git yang berjalan di host, yang menambahkan autentikasi sebelum menghubungi repositori jarak jauh. Agen tidak pernah melihat kredensial.

Keuntungan:

* **Tidak ada intersepsi TLS**: Layanan eksternal membuat permintaan terautentikasi secara langsung
* **Kredensial tetap di luar**: Agen hanya melihat antarmuka alat, bukan kredensial yang mendasarinya

<h4 id="traffic-forwarding">
  Penerusan lalu lintas
</h4>

Untuk panggilan API Claude, `ANTHROPIC_BASE_URL` memungkinkan Anda merutekan permintaan ke proxy yang dapat memeriksa dan memodifikasinya dalam plaintext. Tetapi untuk layanan HTTPS lainnya (GitHub, registri npm, API internal), lalu lintas sering dienkripsi end-to-end. Bahkan jika Anda merutekannya melalui proxy melalui `HTTP_PROXY`, proxy hanya melihat terowongan TLS yang buram dan tidak dapat menyuntikkan kredensial.

Untuk memodifikasi lalu lintas HTTPS ke layanan arbitrer, tanpa menggunakan alat kustom, Anda memerlukan proxy yang menghentikan TLS yang mendekripsi lalu lintas, memeriksa atau memodifikasinya, kemudian mengenkripsi ulang sebelum meneruskan. Ini memerlukan:

1. Menjalankan proxy di luar kontainer agen
2. Memasang sertifikat CA proxy di toko kepercayaan agen (sehingga agen mempercayai sertifikat proxy)
3. Mengonfigurasi `HTTP_PROXY`/`HTTPS_PROXY` untuk merutekan lalu lintas melalui proxy

Pendekatan ini menangani layanan berbasis HTTP apa pun tanpa menulis alat kustom, tetapi menambah kompleksitas di sekitar manajemen sertifikat.

Perhatikan bahwa tidak semua program menghormati `HTTP_PROXY`/`HTTPS_PROXY`. Sebagian besar alat (curl, pip, npm, git) melakukannya, tetapi beberapa mungkin melewati variabel ini dan terhubung langsung. Misalnya, Node.js `fetch()` mengabaikan variabel ini secara default; di Node 24+ Anda dapat mengatur `NODE_USE_ENV_PROXY=1` untuk mengaktifkan dukungan. Untuk cakupan komprehensif, Anda dapat menggunakan [proxychains](https://github.com/haad/proxychains) untuk mengintersepsi panggilan jaringan, atau mengonfigurasi iptables untuk mengarahkan lalu lintas keluar ke proxy transparan.

<Info>
  **Proxy transparan** mengintersepsi lalu lintas di tingkat jaringan, sehingga klien tidak perlu dikonfigurasi untuk menggunakannya. Proxy reguler memerlukan klien untuk secara eksplisit terhubung dan berbicara HTTP CONNECT atau SOCKS. Proxy transparan (seperti Squid atau mitmproxy dalam mode transparan) dapat menangani koneksi TCP yang dialihkan mentah.
</Info>

Kedua pendekatan masih memerlukan proxy yang menghentikan TLS dan sertifikat CA terpercaya. Mereka hanya memastikan lalu lintas benar-benar mencapai proxy.

<h2 id="filesystem-configuration">
  Konfigurasi filesystem
</h2>

Kontrol filesystem menentukan file apa yang dapat dibaca dan ditulis agen.

<h3 id="read-only-code-mounting">
  Pemasangan kode read-only
</h3>

Ketika agen perlu menganalisis kode tetapi tidak memodifikasinya, pasang direktori read-only:

```bash theme={null}
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
  Bahkan akses read-only ke direktori kode dapat mengekspos kredensial. File umum untuk dikecualikan atau disanitasi sebelum pemasangan:

  | File                                                    | Risiko                                  |
  | ------------------------------------------------------- | --------------------------------------- |
  | `.env`, `.env.local`                                    | Kunci API, kata sandi database, rahasia |
  | `~/.git-credentials`                                    | Kata sandi/token Git dalam plaintext    |
  | `~/.aws/credentials`                                    | Kunci akses AWS                         |
  | `~/.config/gcloud/application_default_credentials.json` | Token ADC Google Cloud                  |
  | `~/.azure/`                                             | Kredensial CLI Azure                    |
  | `~/.docker/config.json`                                 | Token auth registri Docker              |
  | `~/.kube/config`                                        | Kredensial kluster Kubernetes           |
  | `.npmrc`, `.pypirc`                                     | Token registri paket                    |
  | `*-service-account.json`                                | Kunci akun layanan GCP                  |
  | `*.pem`, `*.key`                                        | Kunci pribadi                           |

  Pertimbangkan untuk menyalin hanya file sumber yang diperlukan, atau menggunakan penyaringan gaya `.dockerignore`.
</Warning>

<h3 id="writable-locations">
  Lokasi yang dapat ditulis
</h3>

Jika agen perlu menulis file, Anda memiliki beberapa opsi tergantung pada apakah Anda ingin perubahan bertahan:

Untuk ruang kerja ephemeral di kontainer, gunakan pemasangan `tmpfs` yang hanya ada di memori dan dihapus saat kontainer berhenti:

```bash theme={null}
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Jika Anda ingin meninjau perubahan sebelum mempertahankannya, filesystem overlay memungkinkan agen menulis tanpa memodifikasi file yang mendasarinya. Perubahan disimpan di lapisan terpisah yang dapat Anda periksa, terapkan, atau buang. Untuk output yang sepenuhnya persisten, pasang volume khusus tetapi simpan terpisah dari direktori sensitif.

<h2 id="further-reading">
  Bacaan lebih lanjut
</h2>

* [Dokumentasi keamanan Claude Code](/docs/id/security)
* [Hosting Agent SDK](/docs/id/agent-sdk/hosting)
* [Menangani izin](/docs/id/agent-sdk/permissions)
* [Sandbox runtime](https://github.com/anthropic-experimental/sandbox-runtime)
* [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
* [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
* [Docker Security Best Practices](https://docs.docker.com/engine/security/)
* [gVisor Documentation](https://gvisor.dev/docs/)
* [Firecracker Documentation](https://firecracker-microvm.github.io/)
