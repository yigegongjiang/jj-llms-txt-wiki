> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop di Linux (beta)

> Instal dan perbarui aplikasi desktop Claude di Ubuntu dan Debian

<Note>
  Dukungan Linux untuk aplikasi desktop Claude sedang dalam beta. Tab Chat, Cowork, dan Code semuanya tersedia.
</Note>

Aplikasi desktop di Linux memberikan Anda pengalaman Chat, Cowork, dan Claude Code yang sama seperti macOS dan Windows: sesi paralel, tinjauan diff visual, terminal dan editor terintegrasi, dan pratinjau aplikasi langsung. Lihat [Gunakan Claude Code Desktop](/docs/id/desktop) untuk referensi fitur lengkap.

<h2 id="requirements">
  Persyaratan
</h2>

* Ubuntu 22.04 atau lebih baru, atau Debian 12 atau lebih baru
* x86\_64 atau arm64

Distribusi berbasis Debian lainnya yang memenuhi persyaratan ini mungkin berfungsi tetapi tidak diuji secara resmi.

<h2 id="install">
  Instal
</h2>

Instal dari repositori apt Anthropic sehingga pembaruan tiba melalui pembaruan paket reguler sistem Anda. Buka terminal dan jalankan perintah di setiap langkah.

<Steps>
  <Step title="Tambahkan repositori apt Anthropic">
    Langkah ini mengunduh kunci penandatanganan dengan `curl`, yang instalasi Debian dan Ubuntu yang baru mungkin tidak menyertakan. Jika perintah unduh gagal dengan `sudo: curl: command not found`, instal curl terlebih dahulu:

    ```bash theme={null}
    sudo apt install curl
    ```

    Unduh kunci penandatanganan Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

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

<Accordion title="Verifikasi kunci penandatanganan">
  Anda dapat mengonfirmasi bahwa kunci penandatanganan yang diunduh milik Anthropic:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  Sidik jari harus `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Instal dari file yang diunduh
</h3>

Jika Anda tidak dapat menginstal melalui repositori apt, unduh paket `.deb` secara langsung dari kumpulan paket repositori. Perintah ini mencari paket terbaru untuk arsitektur Anda di indeks repositori, kemudian mengunduhnya ke direktori saat ini:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Jika perintah gagal dengan `Remote file name has no length`, pencarian mengembalikan tidak ada jalur paket. Ini dapat berarti indeks repositori tidak dapat diambil, misalnya ketika jaringan Anda memblokir `downloads.claude.ai`, atau bahwa tidak ada paket yang ada untuk arsitektur Anda. Konfirmasi bahwa jaringan Anda dapat menjangkau `downloads.claude.ai` dan bahwa `dpkg --print-architecture` mencetak `amd64` atau `arm64`; repositori tidak menerbitkan paket untuk arsitektur lain.

Kemudian buka file yang diunduh dengan penginstal perangkat lunak Anda, seperti GNOME Software, atau instal dengan apt dari direktori yang berisi file yang diunduh:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Jika apt melaporkan `E: Unsupported file ./claude-desktop_*.deb given on commandline`, pola tidak cocok dengan file `.deb` di direktori saat ini. Konfirmasi bahwa unduhan selesai, kemudian jalankan perintah lagi dari direktori yang berisi file tersebut.

`.deb` yang diinstal dengan cara ini tidak menerima pembaruan. Untuk mendapatkan pembaruan melalui apt, daftarkan repositori dari langkah [Tambahkan repositori apt Anthropic](#install). Paket juga menulis entri repositori yang dikomentari ke `/etc/apt/sources.list.d/claude-desktop.list`; membuka komentar pada baris `deb` adalah setara.

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

Ini menghapus kunci penandatanganan bersama dengan aplikasi, jadi jika Anda menambahkan entri repositori selama instalasi, hapus juga:

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

* Konfirmasi entri repositori telah ditulis. `cat /etc/apt/sources.list.d/claude-desktop.list` harus menampilkan baris `deb` dari langkah [Add Anthropic's apt repository](#install). Jika file kosong atau hilang, jalankan langkah itu lagi.
* Konfirmasi arsitektur Anda didukung. `dpkg --print-architecture` harus mencetak `amd64` atau `arm64`. Repositori tidak menerbitkan paket untuk arsitektur lain.
* Jalankan `sudo apt update` lagi dan periksa outputnya untuk kesalahan yang terkait dengan `downloads.claude.ai`. Kesalahan jaringan atau kunci di sana berarti repositori ditambahkan tetapi tidak dapat dijangkau atau diverifikasi.

Jika repositori sudah ada dan dapat dijangkau dan paket masih tidak ditemukan, [install dari file yang diunduh](#install-from-a-downloaded-file) sebagai gantinya.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Apa yang belum ada di beta Linux
</h2>

* **Computer Use**: [kontrol aplikasi dan layar](/docs/id/desktop#let-claude-use-your-computer) tidak tersedia di Linux.
* **Dictation**: input suara tidak tersedia di aplikasi desktop Linux. Gunakan [dictation suara](/docs/id/voice-dictation) di CLI sebagai gantinya.
* **Quick Entry global hotkey**: berfungsi di X11. Di Wayland asli, ini memerlukan portal GlobalShortcuts lingkungan desktop Anda.
* **Fedora dan RHEL**: hanya distribusi berbasis Debian yang didukung hari ini. Dukungan untuk distribusi tambahan akan datang di masa depan.

Untuk apa pun yang belum tersedia di aplikasi desktop, [CLI](/docs/id/quickstart) menjalankan mesin Claude Code yang sama dan mendukung berbagai distribusi Linux yang lebih luas; lihat [persyaratan sistem](/docs/id/setup#system-requirements).
