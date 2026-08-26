> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Gunakan Claude Code dengan JetBrains IDEs termasuk IntelliJ, PyCharm, WebStorm, dan lainnya

Claude Code terintegrasi dengan JetBrains IDEs melalui plugin khusus, menyediakan fitur seperti tampilan diff interaktif, berbagi konteks seleksi, dan lainnya.

<h2 id="supported-ides">
  IDE yang Didukung
</h2>

Plugin Claude Code bekerja dengan sebagian besar JetBrains IDEs, termasuk:

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Fitur
</h2>

* **Peluncuran cepat**: Gunakan `Cmd+Esc` (Mac) atau `Ctrl+Esc` (Windows/Linux) untuk membuka Claude Code langsung dari editor Anda, atau klik tombol Claude Code di UI
* **Tampilan diff**: Perubahan kode dapat ditampilkan langsung di penampil diff IDE alih-alih terminal
* **Konteks seleksi**: Seleksi atau tab saat ini di IDE secara otomatis dibagikan dengan Claude Code. Aturan penolakan [`Read`](/docs/id/permissions#read-and-edit) memblokir berbagi ini untuk file yang cocok
* **Pintasan referensi file**: Gunakan `Cmd+Option+K` (Mac) atau `Alt+Ctrl+K` (Linux/Windows) untuk menyisipkan referensi file seperti `@src/auth.ts#L1-99`
* **Berbagi diagnostik**: Kesalahan diagnostik dari IDE, seperti lint dan kesalahan sintaks, secara otomatis dibagikan dengan Claude saat Anda bekerja

<h2 id="installation">
  Instalasi
</h2>

Plugin menjalankan perintah `claude` di terminal terintegrasi IDE Anda dan terhubung dengannya. Plugin tidak menyertakan salinan CLI-nya sendiri, jadi instal kedua bagian:

<Steps>
  <Step title="Instal Claude Code CLI">
    Ikuti [quickstart](/docs/id/quickstart) untuk menginstal CLI jika Anda belum melakukannya. Plugin menampilkan notifikasi "Cannot launch Claude Code" ketika `claude` tidak ada di PATH Anda.
  </Step>

  <Step title="Instal plugin JetBrains">
    Instal [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) dari JetBrains Marketplace dan mulai ulang IDE Anda.
  </Step>
</Steps>

Jika `claude` diinstal di tempat yang tidak dapat ditemukan IDE Anda, atur jalur lengkap di [pengaturan perintah Claude](#general-settings) plugin.

Claude Code bekerja dengan langganan Claude berbayar apa pun (Pro, Max, Team, atau Enterprise) atau akun Claude Console, dan tidak ada kunci API yang diperlukan. Anda akan diminta untuk [masuk](/docs/id/authentication#log-in-to-claude-code) pertama kali Anda menjalankan `claude`.

<Note>
  Setelah menginstal plugin, Anda mungkin perlu memulai ulang IDE Anda sepenuhnya agar dapat diterapkan.
</Note>

<h2 id="usage">
  Penggunaan
</h2>

<h3 id="from-your-ide">
  Dari IDE Anda
</h3>

Jalankan `claude` dari terminal terintegrasi IDE Anda, dan semua fitur integrasi akan aktif.

<h3 id="from-external-terminals">
  Dari Terminal Eksternal
</h3>

Gunakan perintah `/ide` di terminal eksternal apa pun untuk menghubungkan Claude Code ke JetBrains IDE Anda dan mengaktifkan semua fitur:

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Jika Anda ingin Claude memiliki akses ke file yang sama dengan IDE Anda, mulai Claude Code dari direktori yang sama dengan root proyek IDE Anda.

<h2 id="configuration">
  Konfigurasi
</h2>

<h3 id="claude-code-settings">
  Pengaturan Claude Code
</h3>

Konfigurasikan integrasi IDE melalui pengaturan Claude Code:

1. Jalankan `claude`
2. Masukkan perintah `/config`
3. Atur alat diff ke `auto` untuk menampilkan diff di IDE, atau `terminal` untuk menyimpannya di terminal

<h3 id="plugin-settings">
  Pengaturan Plugin
</h3>

Konfigurasikan plugin Claude Code dengan membuka **Settings → Tools → Claude Code \[Beta]**:

<h4 id="general-settings">
  Pengaturan Umum
</h4>

* **Perintah Claude**: Tentukan perintah khusus untuk menjalankan Claude, misalnya `claude`, `/usr/local/bin/claude`, atau `npx @anthropic-ai/claude-code`
* **Tekan notifikasi untuk perintah Claude tidak ditemukan**: Lewati notifikasi tentang tidak menemukan perintah Claude
* **Aktifkan penggunaan Option+Enter untuk prompt multi-baris**: Hanya di macOS. Ketika diaktifkan, Option+Enter menyisipkan baris baru dalam prompt Claude Code. Nonaktifkan jika tombol Option ditangkap secara tidak terduga. Memerlukan restart terminal.
* **Aktifkan pembaruan otomatis**: Secara otomatis periksa dan instal pembaruan plugin, diterapkan saat restart

<Tip>
  Untuk pengguna WSL: Atur `wsl -d Ubuntu -- bash -lic "claude"` sebagai perintah Claude Anda (ganti `Ubuntu` dengan nama distribusi WSL Anda)
</Tip>

<h4 id="esc-key-configuration">
  Konfigurasi Tombol ESC
</h4>

Jika tombol ESC tidak menghentikan operasi Claude Code di terminal JetBrains:

1. Buka **Settings → Tools → Terminal**
2. Salah satu dari:
   * Batalkan centang "Pindahkan fokus ke editor dengan Escape", atau
   * Klik "Konfigurasikan pintasan keyboard terminal" dan hapus pintasan "Alihkan fokus ke Editor"
3. Terapkan perubahan

Ini memungkinkan tombol ESC untuk dengan benar menghentikan operasi Claude Code.

<h2 id="special-configurations">
  Konfigurasi Khusus
</h2>

<h3 id="remote-development">
  Pengembangan Jarak Jauh
</h3>

<Warning>
  Saat menggunakan JetBrains Remote Development, Anda harus menginstal plugin di host jarak jauh melalui **Settings → Plugin (Host)**.
</Warning>

Plugin harus diinstal di host jarak jauh, bukan di mesin klien lokal Anda.

<h3 id="wsl-configuration">
  Konfigurasi WSL
</h3>

Jika Anda menggunakan Claude Code di WSL2 dengan JetBrains IDE dan melihat "No available IDEs detected", penyebabnya biasanya adalah jaringan NAT WSL2 atau Windows Firewall yang memblokir koneksi antara WSL2 dan IDE yang berjalan di host Windows. WSL1 menggunakan jaringan host secara langsung dan tidak terpengaruh.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  Izinkan lalu lintas WSL2 melalui Windows Firewall
</h4>

Ini adalah perbaikan yang direkomendasikan karena mempertahankan mode jaringan WSL2 yang ada.

<Steps>
  <Step title="Temukan alamat IP WSL2 Anda">
    Dari dalam shell WSL Anda, jalankan:

    ```bash theme={null}
    hostname -I
    ```

    Catat subnet, misalnya `172.21.123.45` berada di `172.21.0.0/16`.
  </Step>

  <Step title="Buat aturan firewall">
    Buka PowerShell sebagai Administrator dan jalankan yang berikut, sesuaikan rentang IP untuk mencocokkan subnet Anda:

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Mulai ulang IDE dan Claude Code Anda">
    Tutup dan buka kembali keduanya agar aturan baru berlaku.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Alihkan WSL2 ke jaringan mirrored
</h4>

Jaringan mirrored memerlukan Windows 11 22H2 atau lebih baru. Jika Anda menggunakan Windows 10, gunakan aturan firewall di atas.

Tambahkan ini ke `.wslconfig` di direktori pengguna Windows Anda:

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Kemudian mulai ulang WSL dengan `wsl --shutdown` dari PowerShell.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="plugin-not-working">
  Plugin tidak berfungsi
</h3>

Jika plugin diinstal tetapi fitur Claude Code tidak muncul di IDE Anda:

* Pastikan Anda menjalankan Claude Code dari direktori root proyek
* Periksa bahwa plugin JetBrains diaktifkan dalam pengaturan IDE
* Mulai ulang IDE sepenuhnya (Anda mungkin perlu melakukan ini beberapa kali)
* Untuk Remote Development, pastikan plugin diinstal di host jarak jauh

<h3 id="ide-not-detected">
  IDE tidak terdeteksi
</h3>

Jika menjalankan `claude` menunjukkan "No available IDEs detected":

* Verifikasi plugin diinstal dan diaktifkan
* Mulai ulang IDE sepenuhnya
* Periksa bahwa Anda menjalankan Claude Code dari terminal terintegrasi
* Untuk pengguna WSL, lihat [konfigurasi WSL](#wsl-configuration) di atas

<h3 id="command-not-found">
  Perintah tidak ditemukan
</h3>

Jika mengklik ikon Claude menunjukkan "command not found":

1. Verifikasi Claude Code diinstal dengan menjalankan `claude --version` di terminal
2. Konfigurasikan jalur perintah Claude dalam pengaturan plugin
3. Untuk pengguna WSL, gunakan format perintah WSL yang disebutkan di bagian konfigurasi

<h2 id="security-considerations">
  Pertimbangan Keamanan
</h2>

Ketika Claude Code berjalan di JetBrains IDE dalam mode izin [`acceptEdits`](/docs/id/permission-modes#auto-approve-file-edits-with-acceptedits-mode), Claude Code mungkin dapat memodifikasi file konfigurasi IDE yang dapat dijalankan secara otomatis oleh IDE Anda. Ini dapat meningkatkan risiko menjalankan Claude Code dalam mode `acceptEdits` dan memungkinkan melewati prompt izin Claude Code untuk eksekusi bash.

Saat berjalan di JetBrains IDEs, pertimbangkan:

* Menggunakan mode persetujuan manual untuk edit
* Berhati-hati ekstra untuk memastikan Claude hanya digunakan dengan prompt terpercaya
* Menyadari file mana yang Claude Code memiliki akses untuk memodifikasi

Untuk masalah instalasi atau login Claude Code di luar IDE, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  Server MCP IDE bawaan
</h3>

Ketika plugin aktif, plugin menjalankan server MCP lokal yang CLI terhubung secara otomatis. Ini adalah cara CLI membuka diff di penampil diff asli IDE, membaca pilihan Anda saat ini untuk penyebutan `@`, dan menarik diagnostik inspeksi ke dalam percakapan.

Server bernama `ide` dan tersembunyi dari `/mcp` karena tidak ada yang perlu dikonfigurasi. Jika organisasi Anda menggunakan [`PreToolUse` hook](/docs/id/hooks#pretooluse) untuk membuat daftar putih alat MCP, bagaimanapun, Anda perlu mengetahui bahwa itu ada.

**Konteks pilihan dan file terbuka.** Saat terhubung, CLI menyertakan pilihan editor Anda saat ini dan jalur file aktif sebagai konteks pada setiap prompt yang Anda kirim. Transkrip menunjukkan baris `⧉ Selected N lines from <file>` ketika ini terjadi. Untuk mengecualikan file sensitif seperti `.env`, tambahkan [aturan penolakan `Read`](/docs/id/permissions#read-and-edit) untuk jalurnya. Aturan penolakan yang cocok mencegah baik teks yang dipilih maupun pemberitahuan file terbuka untuk file itu dari mencapai Claude.

**Transportasi dan autentikasi.** Server mendengarkan pada port ephemeral yang ditugaskan OS, dan port tidak dapat dikonfigurasi. Transportasi adalah `ws://` yang tidak terenkripsi; pada loopback, proses apa pun yang dapat menangkap lalu lintas juga dapat membaca token dari file kunci, jadi TLS tidak akan menambah perlindungan terhadap penyerang lokal. Setiap awal IDE menghasilkan token autentikasi acak segar, menulisnya ke file kunci di `~/.claude/ide/<port>.lock`, dan CLI harus menyajikannya sebagai header `X-Claude-Code-Ide-Authorization` untuk terhubung. Jika `CLAUDE_CONFIG_DIR` diatur, file kunci ditulis ke `$CLAUDE_CONFIG_DIR/ide/` sebagai gantinya.

**Alat yang diekspos ke model.** Server menampilkan beberapa alat, tetapi hanya satu yang terlihat oleh model. Sisanya adalah RPC internal yang CLI gunakan untuk UI-nya sendiri, seperti membuka diff dan membaca pilihan, dan disaring keluar sebelum daftar alat mencapai Claude.

| Nama alat (seperti yang terlihat oleh hooks) | Apa yang dilakukannya                                                                                                                | Hanya-baca |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ---------- |
| `mcp__ide__getDiagnostics`                   | Mengembalikan diagnostik inspeksi IDE, kesalahan dan peringatan yang ditampilkan di editor. Secara opsional dibatasi pada satu file. | Ya         |

Plugin JetBrains tidak mengekspos alat eksekusi kode ke model.

**Antarmuka mendengarkan.** Antarmuka jaringan mana yang server ikat dikontrol oleh **Accept connections from all network interfaces** di bawah **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)**. Dengan pengaturan dinonaktifkan, server mendengarkan hanya pada `127.0.0.1` dan tidak dapat dijangkau dari host lain. Dengan pengaturan diaktifkan, port dapat dijangkau dari jaringan lokal Anda. Pengaturan ada untuk kasus di mana CLI tidak dapat menjangkau IDE melalui loopback, seperti WSL2 dengan jaringan NAT default atau pengaturan IDE jarak jauh; lihat [Konfigurasi WSL](#wsl-configuration) untuk skenario itu.

<Warning>
  Mengaktifkan **Accept connections from all network interfaces** membuat port MCP IDE dapat dijangkau dari jaringan lokal Anda. Koneksi masih memerlukan token autentikasi dari file kunci, tetapi karena transportasi adalah `ws://` yang tidak terenkripsi, baik lalu lintas sesi maupun token itu melintasi jaringan dalam teks biasa ketika pengaturan aktif. Hanya aktifkan ketika loopback benar-benar tidak dapat bekerja. Untuk WSL2, lebih suka [jaringan cermin](#switch-wsl2-to-mirrored-networking) sehingga antarmuka loopback Windows dibagikan dengan VM Linux dan soket dapat tetap pada loopback.
</Warning>
