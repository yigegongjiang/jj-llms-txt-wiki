> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop di Linux (beta)

> Instal dan perbarui aplikasi desktop Claude di Ubuntu dan Debian

<Note>
  Dukungan Linux untuk aplikasi desktop Claude sedang dalam beta.
</Note>

Aplikasi desktop di Linux memberikan Anda pengalaman Chat, Cowork, dan Claude Code yang sama seperti di macOS dan Windows: sesi paralel, tinjauan diff visual, terminal dan editor terintegrasi, dan pratinjau aplikasi langsung. Lihat [Gunakan Claude Code Desktop](/docs/id/desktop) untuk referensi fitur.

<h2 id="requirements">
  Persyaratan
</h2>

* Distribusi berbasis Debian: Ubuntu 22.04 atau lebih baru, atau Debian 12 atau lebih baru
* x86\_64 atau arm64

Distribusi berbasis Debian lainnya yang memenuhi persyaratan ini mungkin berfungsi tetapi tidak diuji secara resmi. Pada distribusi yang bukan berbasis Debian, seperti Fedora atau Arch, jalankan [CLI](/docs/id/setup#system-requirements) sebagai gantinya. Jika Anda bekerja di Windows dengan WSL 2, instal aplikasi desktop Windows dan jalankan sesi di dalam distribusi Anda; lihat [Claude Code Desktop di WSL](/docs/id/desktop-wsl).

<h3 id="cowork-requirements">
  Persyaratan Cowork
</h3>

Cowork adalah tab desktop untuk [Dispatch dan pekerjaan agentic yang lebih lama](https://claude.com/docs/cowork/overview). Di Linux, Cowork menjalankan tugas-tugas tersebut di mesin virtual yang dihosting oleh aplikasi desktop dengan QEMU dan KVM. Untuk menggunakan Cowork, mesin Anda memerlukan:

* **Virtualisasi hardware**: diaktifkan dalam pengaturan firmware Anda. Tanpa itu, tab Cowork melaporkan "Cowork requires hardware virtualization (KVM)".
* **QEMU dan firmware UEFI**: `qemu-system-x86`, `ovmf`, dan `virtiofsd` pada x86\_64, atau `qemu-system-arm`, `qemu-efi-aarch64`, dan `virtiofsd` pada arm64. `apt install claude-desktop` menginstalnya secara default sebagai paket yang direkomendasikan. Jika Anda menginstal dengan `--no-install-recommends`, atau sistem Anda adalah gambar minimal yang melewatkan paket yang direkomendasikan, tab Cowork melaporkan "Cowork requires QEMU" dan menampilkan perintah `apt install` untuk dijalankan. Ubuntu 22.04 tidak memiliki paket `virtiofsd`; aplikasi menggunakan salinan bundel di sana.
* **Akses ke `/dev/kvm`**: tambahkan pengguna Anda ke grup `kvm` dengan `sudo usermod -aG kvm $USER`, kemudian keluar dan masuk kembali. Beberapa lingkungan desktop memberikan akses pengguna yang masuk ke `/dev/kvm` tanpa grup, tetapi Cowork juga memerlukan `/dev/vhost-vsock`, yang hanya dapat dibuka oleh anggota grup `kvm`. Bergabunglah dengan grup bahkan jika `/dev/kvm` sudah berfungsi untuk Anda.

Aplikasi memeriksa persyaratan ini sekali saat peluncuran: mulai ulang setelah menginstal paket, dan keluar serta masuk kembali setelah bergabung dengan grup. Jika `/dev/vhost-vsock` hilang dan kernel yang sedang berjalan tidak memiliki direktori modul di bawah `/lib/modules`, tab Cowork melaporkan bahwa kernel tidak menyertakan dukungan virtualisasi yang Cowork butuhkan dan bahwa itu tidak dapat ditambahkan secara manual. Kombinasi ini umum di ChromeOS dan di lingkungan Linux berbasis kontainer.

<h2 id="install">
  Instal
</h2>

Instal dari repositori apt Anthropic sehingga pembaruan tiba melalui pembaruan paket reguler sistem Anda. Buka terminal dan jalankan perintah di setiap langkah.

<Steps>
  <Step title="Tambahkan repositori apt Anthropic">
    Langkah ini mengunduh kunci penandatanganan dengan `curl` dan memverifikasinya dengan `gpg`, yang instalasi Debian dan Ubuntu yang baru mungkin tidak menyertakan. Jika salah satu perintah melaporkan `command not found`, instal keduanya terlebih dahulu:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Unduh kunci penandatanganan Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Perintah tidak mencetak apa pun ketika berhasil dan kesalahan `curl:` ketika tidak. Kunci yang hilang atau salah membuat `apt update` gagal nanti dengan `NO_PUBKEY BAA929FF1A7ECACE`, jadi konfirmasi kunci telah diunduh dan milik Anthropic sebelum melanjutkan:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    Sidik jari yang dicetak gpg harus `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Jika gpg melaporkan bahwa file tidak dapat dibuka atau tidak berisi data OpenPGP yang valid, unduhan gagal atau mengembalikan konten yang salah: konfirmasi jaringan Anda dapat menjangkau `downloads.claude.ai`, kemudian jalankan kembali perintah unduh.

    Daftarkan repositori:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Instal paket">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Luncurkan dan masuk">
    Luncurkan **Claude** dari peluncur aplikasi Anda, atau jalankan `claude-desktop` dari terminal, dan masuk dengan akun Anthropic Anda.

    Aplikasi Linux masuk dengan cara yang sama seperti di macOS dan Windows: dengan langganan claude.ai, atau melalui SSO organisasi Anda. Desktop tidak menerima kunci API Claude Console secara langsung; gunakan [CLI](/docs/id/quickstart) untuk autentikasi kunci API. Untuk penyebaran enterprise yang merutekan Desktop ke Agent Platform Google Cloud atau gateway LLM, lihat [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) dan [konfigurasi jaringan](/docs/id/network-config).
  </Step>
</Steps>

<h3 id="install-from-a-downloaded-file">
  Instal dari file yang diunduh
</h3>

Jika Anda tidak dapat menginstal melalui repositori apt, unduh paket `.deb` secara langsung dari kumpulan paket repositori. Perintah ini mencari paket terbaru untuk arsitektur Anda di indeks repositori, kemudian mengunduhnya ke direktori saat ini:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Jika perintah gagal dengan `Remote file name has no length`, pencarian mengembalikan tidak ada jalur paket. Ini dapat berarti indeks repositori tidak dapat diambil, misalnya ketika jaringan Anda memblokir `downloads.claude.ai`, atau bahwa tidak ada paket yang ada untuk arsitektur Anda. Konfirmasi bahwa jaringan Anda dapat menjangkau `downloads.claude.ai` dan bahwa `dpkg --print-architecture` mencetak `amd64` atau `arm64`; repositori tidak menerbitkan paket untuk arsitektur lain.

Untuk menginstal tanpa mendaftarkan repositori apt Anthropic, pertama buat `/etc/default/claude-desktop` dengan baris `CLAUDE_DESKTOP_ADD_REPO="false"`. Tanpa repositori, apt tidak mengirimkan versi baru; untuk memperbarui, jalankan kembali perintah unduh dan instal ulang, atau [daftarkan repositori](#install) nanti.

Kemudian buka file yang diunduh dengan penginstal perangkat lunak Anda, seperti GNOME Software, atau instal dengan apt dari direktori yang berisi file yang diunduh:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Jika apt melaporkan `E: Unsupported file ./claude-desktop_*.deb given on commandline`, pola tidak cocok dengan file `.deb` di direktori saat ini. Konfirmasi bahwa unduhan selesai, kemudian jalankan perintah lagi dari direktori yang berisi file tersebut.

Menginstal `.deb` juga mendaftarkan repositori apt Anthropic di `/etc/apt/sources.list.d/claude-desktop.list`, jadi pembaruan di masa depan tiba dengan [pembaruan paket reguler](#update) sistem Anda.

<h2 id="update">
  Perbarui
</h2>

Aplikasi desktop tidak memperbarui dirinya sendiri di Linux. Pembaruan tiba dengan pembaruan paket reguler sistem Anda:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

Pembarui perangkat lunak grafis distribusi Anda juga akan mengambil versi baru.

<h2 id="uninstall">
  Copot
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Mencopot paket juga menghapus entri repositori dan kunci penandatanganan yang didaftarkannya. Jika Anda menambahkan entri repositori sendiri dengan langkah [Tambahkan repositori apt Anthropic](#install), hapus juga:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Troubleshooting
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Tidak dapat menemukan paket claude-desktop
</h3>

Jika `sudo apt install claude-desktop` gagal dengan `E: Unable to locate package claude-desktop`, apt tidak menemukan repositori yang Anda tambahkan. Periksa hal berikut:

* Jalankan `sudo apt update` setelah menambahkan repositori. `apt install` sendiri tidak melihat repositori yang Anda tambahkan setelah terakhir kali Anda menjalankan `apt update`.
* Konfirmasi entri repositori telah ditulis. `cat /etc/apt/sources.list.d/claude-desktop.list` harus menampilkan baris `deb` dari langkah [Add Anthropic's apt repository](#install). Jika file kosong atau hilang, jalankan langkah itu lagi.
* Konfirmasi arsitektur Anda didukung. `dpkg --print-architecture` harus mencetak `amd64` atau `arm64`. Repositori tidak menerbitkan paket untuk arsitektur lain.
* Jalankan `sudo apt update` lagi dan periksa outputnya untuk kesalahan yang terkait dengan `downloads.claude.ai`. Kesalahan jaringan atau kunci di sana berarti repositori ditambahkan tetapi tidak dapat dijangkau atau diverifikasi.

Jika repositori sudah ada dan dapat dijangkau dan paket masih tidak ditemukan, [install dari file yang diunduh](#install-from-a-downloaded-file) sebagai gantinya.

<h3 id="unmet-dependencies">
  Dependensi yang tidak terpenuhi
</h3>

Jika `apt` berhenti dengan `The following packages have unmet dependencies` atau `Unsatisfied dependencies`, baca dependensi mana yang dinamainya:

* `libc6 (>= 2.34)`: distribusi Anda lebih lama daripada yang didukung paket. Ubuntu 20.04 mengirimkan `libc6` 2.31. Tingkatkan ke Ubuntu 22.04 atau lebih baru, atau Debian 12 atau lebih baru.
* Semua dependensi yang hilang menampilkan `not installable` dengan akhiran `:amd64` atau `:arm64`: Anda mengunduh `.deb` untuk arsitektur yang berbeda dari mesin Anda. Jalankan `dpkg --print-architecture` dan unduh `.deb` yang cocok, atau [install dari repositori apt](#install), yang memilih paket untuk arsitektur Anda.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  Menjalankan sebagai root tanpa --no-sandbox tidak didukung
</h3>

Jika `claude-desktop` keluar dengan pesan ini, Anda meluncurkannya sebagai root. Masuk sebagai pengguna biasa dan luncurkan dari sana.

<h3 id="cowork-isn’t-available">
  Cowork tidak tersedia
</h3>

Jika tab Cowork menampilkan salah satu pesan ini, perbaiki persyaratan yang dinamainya, kemudian mulai ulang aplikasi:

* **Cowork memerlukan QEMU**: instal paket [QEMU dan UEFI firmware](#cowork-requirements) yang tercantum dalam pesan.
* **Cowork memerlukan virtualisasi hardware (KVM)**: aktifkan [virtualisasi hardware](#cowork-requirements) di pengaturan firmware Anda.
* **Claude tidak memiliki izin untuk menggunakan virtualisasi (/dev/kvm)**: tambahkan pengguna Anda ke [grup `kvm`](#cowork-requirements), kemudian keluar dan masuk kembali.
* **Cowork memerlukan modul kernel `vhost_vsock`**: jalankan `sudo modprobe vhost_vsock`, kemudian mulai ulang aplikasi. Itu memuat modul hanya untuk boot saat ini. Untuk memuatnya di setiap boot, jalankan `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Apa yang belum ada di beta Linux
</h2>

* **Computer Use**: [kontrol aplikasi dan layar](/docs/id/desktop#let-claude-use-your-computer) tidak tersedia di Linux.
* **Dictation**: input suara tidak tersedia di aplikasi desktop Linux. Gunakan [dictation suara](/docs/id/voice-dictation) di CLI sebagai gantinya.
* **Quick Entry global hotkey**: berfungsi di X11. Di Wayland asli, ini memerlukan portal GlobalShortcuts lingkungan desktop Anda.
* **Fedora dan RHEL**: hanya distribusi berbasis Debian yang didukung hari ini. Dukungan untuk distribusi tambahan akan datang di masa depan.

Untuk apa pun yang belum tersedia di aplikasi desktop, [CLI](/docs/id/quickstart) menjalankan mesin Claude Code yang sama dan mendukung berbagai distribusi Linux yang lebih luas; lihat [persyaratan sistem](/docs/id/setup#system-requirements).
