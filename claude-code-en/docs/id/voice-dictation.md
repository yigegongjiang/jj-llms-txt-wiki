> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dikte suara

> Ucapkan prompt Anda di Claude Code CLI dengan dikte suara tahan-untuk-merekam atau ketuk-untuk-merekam.

Ucapkan prompt Anda alih-alih mengetiknya di Claude Code CLI. Ucapan Anda ditranskripsikan secara langsung ke dalam input prompt, sehingga Anda dapat mencampur suara dan pengetikan dalam pesan yang sama. Aktifkan dikte dengan `/voice`, kemudian tahan kunci sambil Anda berbicara atau ketuk sekali untuk memulai dan lagi untuk mengirim.

<Note>
  Mode ketuk memerlukan Claude Code v2.1.116 atau lebih baru. Periksa versi Anda dengan `claude --version`.
</Note>

Dikte juga berfungsi di [tampilan agen](/docs/id/agent-view#peek-and-reply). Tahan atau ketuk kunci push-to-talk Anda saat input pengiriman atau balasan panel intip difokuskan untuk mendikte ke sesi latar belakang.

<h2 id="requirements">
  Persyaratan
</h2>

Dikte suara mengalirkan audio yang direkam ke server Anthropic untuk transkripsi. Audio tidak diproses secara lokal. Layanan ini memerlukan semua hal berikut:

* **Akun Claude.ai**: layanan ucapan-ke-teks hanya tersedia saat Anda melakukan autentikasi dengan akun Claude.ai, dan tidak tersedia saat Claude Code dikonfigurasi untuk menggunakan kunci API Anthropic secara langsung, Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.
* **Organisasi tanpa kepatuhan HIPAA yang diaktifkan**: `/voice` menampilkan `Voice mode is disabled by your organization's policy` saat pembatasan ini berlaku.
* **Mikrofon lokal**: dikte suara tidak berfungsi di lingkungan jarak jauh seperti [Claude Code di web](/docs/id/claude-code-on-the-web) atau sesi SSH.
* **WSLg, jika Anda menjalankan Claude Code di WSL**: WSLg disertakan dengan WSL2 saat diinstal dari Microsoft Store di Windows 10 atau 11. Jika WSLg tidak tersedia, misalnya di WSL1, jalankan Claude Code di Windows asli sebagai gantinya.

Transkripsi tidak menggunakan pesan Claude atau token dan tidak dihitung terhadap batas yang ditampilkan di `/usage`. Lihat [penggunaan data](/docs/id/data-usage) untuk mengetahui bagaimana Anthropic menangani data Anda.

Perekaman audio menggunakan modul asli bawaan di macOS, Linux, dan Windows. Di Linux, jika modul asli tidak dapat dimuat, Claude Code kembali ke `arecord` dari ALSA utils atau `rec` dari SoX. Jika tidak ada yang tersedia, `/voice` mencetak perintah instalasi untuk manajer paket Anda.

[Ekstensi VS Code](/docs/id/vs-code) Claude Code juga mendukung dikte suara dengan persyaratan akun Claude.ai yang sama. Ini tidak tersedia di sesi VS Code Remote, termasuk SSH, Dev Containers, dan Codespaces, karena mikrofon berada di mesin lokal Anda dan ekstensi berjalan di host jarak jauh.

<h2 id="enable-voice-dictation">
  Aktifkan dikte suara
</h2>

Jalankan `/voice` untuk mengaktifkan dikte. Pertama kali Anda mengaktifkannya, Claude Code menjalankan pemeriksaan mikrofon. Di macOS, ini memicu prompt izin mikrofon sistem untuk terminal Anda jika belum pernah diberikan.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` menerima argumen mode opsional:

| Perintah      | Efek                                                 |
| :------------ | :--------------------------------------------------- |
| `/voice`      | Alihkan aktif atau mati, pertahankan mode saat ini   |
| `/voice hold` | Aktifkan dalam [mode tahan](#hold-to-record)         |
| `/voice tap`  | Aktifkan dalam [mode ketuk](#tap-to-record-and-send) |
| `/voice off`  | Nonaktifkan                                          |

Dikte suara bertahan di seluruh sesi. Atur langsung di [file pengaturan pengguna](/docs/id/settings) Anda alih-alih menjalankan `/voice`:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Saat dikte suara diaktifkan, footer input menampilkan petunjuk `hold space to speak` saat prompt kosong. Petunjuk mencerminkan pengikatan `voice:pushToTalk` saat ini Anda dan diperbarui jika Anda [mengikat ulang kunci dikte](#rebind-the-dictation-key). Teks petunjuk sama di kedua mode, dan tidak muncul jika Anda memiliki [baris status kustom](/docs/id/statusline) yang dikonfigurasi.

Transkripsi disesuaikan untuk kosakata pengkodean di kedua mode. Istilah pengembangan umum seperti `regex`, `OAuth`, `JSON`, dan `localhost` dikenali dengan benar, dan nama proyek saat ini dan nama cabang git Anda ditambahkan sebagai petunjuk pengenalan secara otomatis.

<h2 id="hold-to-record">
  Tahan untuk merekam
</h2>

Mode tahan adalah push-to-talk: perekaman berjalan saat Anda menahan kunci dan berhenti saat Anda melepasnya. Ini adalah mode default.

Tahan `Space` untuk mulai merekam. Claude Code mendeteksi kunci yang ditahan dengan memantau peristiwa pengulangan kunci cepat dari terminal Anda, jadi ada pemanasan singkat sebelum perekaman dimulai. Footer menampilkan `keep holding…` selama pemanasan, kemudian beralih ke bentuk gelombang langsung setelah perekaman aktif.

Beberapa karakter pengulangan kunci pertama mengetik ke dalam input selama pemanasan dan dihapus secara otomatis saat perekaman diaktifkan. Ketukan `Space` tunggal masih mengetik spasi, karena deteksi tahan hanya dipicu pada pengulangan cepat.

<Tip>
  Untuk melewati pemanasan, beralih ke [mode ketuk](#tap-to-record-and-send) dengan `/voice tap`, atau [ikat ulang ke kombinasi pengubah](#rebind-the-dictation-key) seperti `meta+k`. Kombinasi pengubah mulai merekam pada penekanan tombol pertama.
</Tip>

Ucapan Anda muncul dalam prompt saat Anda berbicara, redup sampai transkrip diselesaikan. Lepaskan `Space` untuk berhenti merekam dan menyelesaikan teks. Transkrip dimasukkan pada posisi kursor Anda dan kursor tetap di akhir teks yang dimasukkan, sehingga Anda dapat mencampur pengetikan dan dikte dalam urutan apa pun. Tahan `Space` lagi untuk menambahkan perekaman lain, atau pindahkan kursor terlebih dahulu untuk menyisipkan ucapan di tempat lain dalam prompt:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Secara default, melepaskan kunci menyisipkan transkrip dan menunggu Anda menekan `Enter`. Atur `"autoSubmit": true` dalam objek pengaturan `voice` untuk mengirim prompt secara otomatis saat Anda melepaskan kunci, asalkan transkrip setidaknya tiga kata panjang.

<h2 id="tap-to-record-and-send">
  Ketuk untuk merekam dan mengirim
</h2>

Mode ketuk mengalihkan perekaman dengan penekanan tombol tunggal: ketuk sekali untuk memulai, berbicara, kemudian ketuk lagi untuk mengirim prompt. Tidak ada pemanasan, dan Anda tidak perlu menahan kunci.

Aktifkan mode ketuk dengan `/voice tap`. Dengan input prompt kosong, ketuk `Space` untuk mulai merekam. Footer menampilkan bentuk gelombang langsung saat merekam. Ketuk `Space` lagi untuk berhenti.

Claude Code menyisipkan transkrip dan mengirimkan prompt secara otomatis saat transkrip setidaknya tiga kata panjang. Transkrip yang lebih pendek dimasukkan tetapi tidak dikirim, sehingga ketukan yang tidak disengaja tidak mengirim kata yang tersesat.

Ambang batas tiga kata menghitung kata untuk bahasa yang ditulis tanpa spasi. Mulai dari v2.1.195, transkrip Jepang, Cina, dan Thailand menghitung kata individual, sehingga mereka auto-submit dalam mode ketuk dan dalam mode tahan dengan `autoSubmit`. Versi sebelumnya menghitung transkrip tanpa spasi sebagai satu kata dan tidak pernah mengirimnya secara otomatis.

Ketukan pertama hanya mulai merekam saat input prompt kosong, sehingga Anda masih dapat mengetik spasi secara normal saat menyusun pesan. Ketukan kedua menghentikan perekaman terlepas dari isi input. Perekaman juga berhenti secara otomatis setelah 15 detik keheningan atau dua menit total.

<h2 id="change-the-dictation-language">
  Ubah bahasa dikte
</h2>

Dikte suara menggunakan pengaturan [`language`](/docs/id/settings) yang sama yang mengontrol bahasa respons Claude. Jika pengaturan itu kosong, dikte default ke Bahasa Inggris. Di ekstensi VS Code, jika `language` kosong, dikte menggunakan pengaturan `accessibility.voice.speechLanguage` VS Code sebelum default ke Bahasa Inggris.

<Accordion title="Bahasa dikte yang didukung">
  | Bahasa    | Kode |
  | :-------- | :--- |
  | Ceko      | `cs` |
  | Denmark   | `da` |
  | Belanda   | `nl` |
  | Inggris   | `en` |
  | Prancis   | `fr` |
  | Jerman    | `de` |
  | Yunani    | `el` |
  | Hindi     | `hi` |
  | Indonesia | `id` |
  | Italia    | `it` |
  | Jepang    | `ja` |
  | Korea     | `ko` |
  | Norwegia  | `no` |
  | Polandia  | `pl` |
  | Portugis  | `pt` |
  | Rusia     | `ru` |
  | Spanyol   | `es` |
  | Swedia    | `sv` |
  | Turki     | `tr` |
  | Ukraina   | `uk` |
</Accordion>

Atur bahasa di `/config` atau langsung di pengaturan. Anda dapat menggunakan [kode bahasa BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) atau nama bahasa:

```json theme={null}
{
  "language": "japanese"
}
```

Jika pengaturan `language` Anda tidak ada dalam daftar yang didukung, `/voice` memperingatkan Anda saat diaktifkan dan kembali ke Bahasa Inggris untuk dikte. Respons teks Claude tidak terpengaruh oleh fallback ini.

<h2 id="rebind-the-dictation-key">
  Ikat ulang kunci dikte
</h2>

Kunci dikte terikat pada `voice:pushToTalk` dalam konteks `Chat` dan default ke `Space`. Pengikatan yang sama mengontrol mode tahan dan ketuk. Ikat ulang di [`~/.claude/keybindings.json`](/docs/id/keybindings):

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

Aksi `voice:pushToTalk` menggunakan satu kunci pada satu waktu. Ketika Anda mengikat kunci khusus, itu menggantikan pengikatan `Space` default daripada menambahkan pemicu kedua, jadi baris `"space": null` dalam contoh ini untuk kejelasan dan dapat dihilangkan tanpa mengubah perilaku.

Dalam mode tahan, hindari mengikat kunci huruf telanjang seperti `v` karena deteksi tahan bergantung pada pengulangan kunci dan huruf mengetik ke dalam prompt selama pemanasan. Gunakan `Space`, atau gunakan kombinasi pengubah seperti `meta+k` untuk mulai merekam pada penekanan tombol pertama tanpa pemanasan. Mode ketuk tidak memiliki pemanasan, jadi sebagian besar kunci berfungsi.

Beberapa kunci tidak dikirimkan ke aplikasi terminal dan tidak dapat diikat sama sekali. Misalnya, `Caps Lock` menampilkan kesalahan jika Anda mencoba mengikatnya. Lihat [sesuaikan pintasan keyboard](/docs/id/keybindings) untuk sintaks keybinding lengkap dan daftar pintasan yang dicadangkan.

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

Masalah umum saat dikte suara tidak diaktifkan atau merekam:

* **`Voice mode requires a Claude.ai account`**: Anda diautentikasi dengan kunci API atau penyedia pihak ketiga. Jalankan `/login` untuk masuk dengan akun Claude.ai.
* **`Voice mode is disabled by your organization's policy`**: konfigurasi kepatuhan organisasi Anda menonaktifkan dikte suara, seperti yang dijelaskan dalam [Persyaratan](#requirements). Hubungi administrator organisasi Anda untuk mengonfirmasi apakah dikte suara tersedia untuk organisasi Anda.
* **`Microphone access is denied`**: berikan izin mikrofon ke terminal Anda di pengaturan sistem. Di macOS, buka System Settings → Privacy & Security → Microphone dan aktifkan aplikasi terminal Anda, kemudian jalankan `/voice` lagi. Di Windows, buka Settings → Privacy & security → Microphone dan aktifkan akses mikrofon untuk aplikasi desktop, kemudian jalankan `/voice` lagi. Jika terminal Anda tidak terdaftar dalam pengaturan macOS, lihat [Terminal tidak terdaftar dalam pengaturan Mikrofon macOS](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` di Linux**: modul audio asli tidak dapat dimuat dan tidak ada fallback yang diinstal. Instal SoX dengan perintah yang ditampilkan dalam pesan kesalahan, misalnya `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX diinstal, tetapi host tidak memiliki perangkat penangkap audio, misalnya server headless atau kontainer. Jalankan Claude Code pada mesin dengan mikrofon. Mulai dari v2.1.195, Claude Code di Linux melaporkan pesan ini dalam situasi itu; versi sebelumnya meminta Anda untuk menginstal SoX bahkan ketika sudah diinstal.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg merutekan audio melalui PulseAudio daripada perangkat ALSA, jadi SoX memerlukan backend PulseAudio-nya diinstal secara eksplisit. Jalankan `sudo apt install sox libsox-fmt-pulse`. Menginstal `sox` saja menarik backend ALSA, yang tidak dapat merekam di WSL karena tidak ada perangkat `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`**: dikte suara mengalami beberapa kegagalan penangkapan berturut-turut dan berhenti mencoba sesi baru sampai satu berhasil. Kegagalan dihitung apakah mikrofon gagal dimulai atau perekam dimulai dan kemudian berhenti tanpa menghasilkan audio apa pun. Ini biasanya berarti mikrofon atau tumpukan audio di host ini tidak dapat menangkap audio, misalnya server headless, shell jarak jauh tanpa passthrough audio, atau izin mikrofon yang ditolak. Konfirmasi perangkat input yang berfungsi, perbaiki penyebab mendasar dari entri di atas, kemudian picu suara lagi. Sebelum v2.1.202, hanya kegagalan awal yang diperhitungkan menuju jeda.
* **Tidak ada yang terjadi saat menahan `Space` dalam mode tahan**: perhatikan input prompt saat Anda menahan. Jika spasi terus menumpuk, dikte suara kemungkinan mati; jalankan `/voice hold` untuk mengaktifkannya. Jika hanya satu atau dua spasi muncul dan kemudian tidak ada, dikte suara aktif tetapi deteksi tahan tidak dipicu. Deteksi tahan memerlukan terminal Anda untuk mengirim peristiwa pengulangan kunci, sehingga tidak dapat mendeteksi kunci yang ditahan jika pengulangan kunci dinonaktifkan di tingkat OS. Beralih ke mode ketuk dengan `/voice tap` untuk menghindari persyaratan pengulangan kunci.
* **Mengetuk `Space` mengetik spasi alih-alih merekam dalam mode ketuk**: ketukan pertama hanya mulai merekam saat input prompt kosong. Hapus input terlebih dahulu, atau periksa bahwa Anda dalam mode ketuk dengan menjalankan `/voice tap`.
* **`No audio detected from microphone`**: perekaman dimulai tetapi menangkap keheningan. Konfirmasi perangkat input yang benar diatur sebagai default sistem dan tingkat inputnya tidak dibisukan atau mendekati nol. Di Windows, buka Settings → System → Sound → Input dan pilih mikrofon Anda. Di macOS, buka System Settings → Sound → Input.
* **`Voice connection failed`**: perekaman Anda tidak pernah mencapai layanan transkripsi karena koneksi gagal. Periksa jaringan Anda dan coba lagi. Perekaman yang menangkap audio tidak melaporkan `No audio detected from microphone` alih-alih pesan ini. Sebelum v2.1.200, mikrofon senyap dapat melaporkan kegagalan koneksi, yang menyarankan masalah jaringan ketika masalah sebenarnya adalah perangkat input.
* **`No speech detected`**: audio mencapai layanan transkripsi tetapi tidak ada kata yang dikenali. Berbicara lebih dekat ke mikrofon, kurangi kebisingan latar belakang, dan konfirmasi [bahasa dikte](#change-the-dictation-language) Anda cocok dengan bahasa yang Anda gunakan.
* **Transkripsi berantakan atau dalam bahasa yang salah**: dikte default ke Bahasa Inggris. Jika Anda mendikte dalam bahasa lain, atur di `/config` terlebih dahulu. Lihat [Ubah bahasa dikte](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal tidak terdaftar dalam pengaturan Mikrofon macOS
</h3>

Jika aplikasi terminal Anda tidak muncul di bawah System Settings → Privacy & Security → Microphone, tidak ada toggle yang dapat Anda aktifkan. Atur ulang status izin untuk terminal Anda sehingga `/voice` berikutnya menjalankan prompt izin macOS yang segar.

<Steps>
  <Step title="Atur ulang izin mikrofon untuk terminal Anda">
    Jalankan `tccutil reset Microphone <bundle-id>`, mengganti `<bundle-id>` dengan pengenal terminal Anda: `com.apple.Terminal` untuk Terminal bawaan, atau `com.googlecode.iterm2` untuk iTerm2. Untuk terminal lain, cari pengenal dengan `osascript -e 'id of app "AppName"'`.

    <Warning>
      Anda dapat menjalankan `tccutil reset Microphone` tanpa ID bundle, tetapi ini mencabut akses mikrofon dari setiap aplikasi di Mac Anda, termasuk aplikasi seperti Zoom atau Slack. Setiap aplikasi perlu meminta akses lagi pada penggunaan berikutnya, jadi jangan jalankan selama panggilan aktif.
    </Warning>
  </Step>

  <Step title="Keluar dan luncurkan ulang terminal Anda">
    macOS tidak akan meminta ulang proses yang sudah berjalan. Keluar dari aplikasi terminal dengan Cmd+Q, bukan hanya tutup jendelanya, kemudian buka lagi.
  </Step>

  <Step title="Picu prompt segar">
    Mulai Claude Code dan jalankan `/voice`. macOS meminta akses mikrofon; izinkan.
  </Step>
</Steps>

<h2 id="see-also">
  Lihat juga
</h2>

* [Sesuaikan pintasan keyboard](/docs/id/keybindings): ikat ulang `voice:pushToTalk` dan tindakan keyboard CLI lainnya
* [Konfigurasi pengaturan](/docs/id/settings): referensi lengkap untuk kunci pengaturan `voice`, `language`, dan lainnya
* [Mode interaktif](/docs/id/interactive-mode): pintasan keyboard, mode input, dan kontrol sesi
* [Perintah](/docs/id/commands): referensi untuk `/voice`, `/config`, dan semua perintah lainnya
