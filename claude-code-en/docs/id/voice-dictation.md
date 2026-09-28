> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dikte suara

> Ucapkan prompt Anda di Claude Code CLI dengan dikte suara tahan-untuk-merekam atau ketuk-untuk-merekam.

Ucapkan prompt Anda alih-alih mengetiknya di Claude Code CLI. Ucapan Anda ditranskripsikan secara langsung ke dalam input prompt, sehingga Anda dapat mencampur suara dan pengetikan dalam pesan yang sama. Aktifkan dikte dengan `/voice`, kemudian tahan kunci sambil Anda berbicara atau ketuk sekali untuk memulai dan lagi untuk mengirim.

Dikte juga berfungsi di [tampilan agen](/docs/id/agent-view#peek-and-reply). Tahan atau ketuk kunci push-to-talk Anda saat input pengiriman atau balasan panel intip difokuskan untuk mendikte ke sesi latar belakang.

<h2 id="requirements">
  Persyaratan
</h2>

Dikte suara mengalirkan audio yang direkam ke server Anthropic untuk transkripsi. Audio tidak diproses secara lokal. Layanan ini memerlukan semua hal berikut:

* **Akun Claude.ai**: layanan ucapan-ke-teks hanya tersedia saat Anda melakukan autentikasi dengan akun Claude.ai, dan tidak tersedia saat Claude Code dikonfigurasi untuk menggunakan kunci API Anthropic secara langsung, Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry.
* **Mikrofon lokal**: dikte suara tidak berfungsi di [sesi cloud](/docs/id/claude-code-on-the-web) atau sesi SSH.
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

Untuk tiga sesi pertama dengan dikte suara diaktifkan, footer input menampilkan petunjuk `hold space to speak` saat prompt kosong. Petunjuk mencerminkan pengikatan `voice:pushToTalk` saat ini Anda dan diperbarui jika Anda [mengikat ulang kunci dikte](#rebind-the-dictation-key). Teks petunjuk sama di kedua mode, dan tidak muncul jika Anda memiliki [baris status kustom](/docs/id/statusline) yang dikonfigurasi.

Transkripsi disesuaikan untuk kosakata pengkodean di kedua mode. Istilah pengembangan umum seperti `regex`, `OAuth`, `JSON`, dan `localhost` dikenali dengan benar, dan nama proyek saat ini dan nama cabang git Anda ditambahkan sebagai petunjuk pengenalan secara otomatis.

<h2 id="hold-to-record">
  Tahan untuk merekam
</h2>

Mode tahan adalah push-to-talk: perekaman berjalan saat Anda menahan kunci dan berhenti saat Anda melepasnya. Ini adalah mode default.

Tahan `Space` untuk mulai merekam. Claude Code mendeteksi kunci yang ditahan dengan memantau peristiwa pengulangan kunci cepat dari terminal Anda, jadi ada pemanasan singkat sebelum perekaman dimulai. Footer menampilkan `keep holding…` selama pemanasan, kemudian `listening…` setelah perekaman aktif. Saat merekam, kursor prompt menjadi batang yang naik dan turun sesuai dengan level mikrofon Anda, kecuali jika Anda memiliki [`prefersReducedMotion`](/docs/id/settings-reference#prefersreducedmotion) yang diaktifkan.

Beberapa karakter pengulangan kunci pertama mengetik ke dalam input selama pemanasan dan dihapus secara otomatis saat perekaman diaktifkan. Ketukan `Space` tunggal masih mengetik spasi, karena deteksi tahan hanya dipicu pada pengulangan cepat.

Menahan atau mengetuk `Space` memulai dikte hanya di tempat penekanan tombol akan mengetik ke dalam prompt. Di [penampil transkrip](/docs/id/interactive-mode#transcript-viewer), `Space` menavigasi percakapan, dan dalam [mode vim](/docs/id/interactive-mode#vim-editor-mode) di luar INSERT itu adalah perintah. Kombinasi pengubah yang [diikat ulang](#rebind-the-dictation-key) seperti `meta+k` tidak pernah mengetik teks, jadi itu juga memulai dikte dari tempat-tempat tersebut.

<Tip>
  Untuk melewati pemanasan, beralih ke [mode ketuk](#tap-to-record-and-send) dengan `/voice tap`, atau [ikat ulang ke kombinasi pengubah](#rebind-the-dictation-key) seperti `meta+k`. Kombinasi pengubah mulai merekam pada penekanan tombol pertama.
</Tip>

Ucapan Anda muncul dalam prompt saat Anda berbicara, redup sampai transkrip diselesaikan. Lepaskan `Space` untuk berhenti merekam dan menyelesaikan teks. Transkrip dimasukkan pada posisi kursor Anda dan kursor tetap di akhir teks yang dimasukkan, sehingga Anda dapat mencampur pengetikan dan dikte dalam urutan apa pun. Tahan `Space` lagi untuk menambahkan perekaman lain, atau pindahkan kursor terlebih dahulu untuk menyisipkan ucapan di tempat lain dalam prompt:

```
> refactor the auth middleware to ▮
  # hold space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Secara default, saat Anda melepaskan kunci, Claude Code menyisipkan transkrip dan menunggu Anda menekan `Enter`. Atur `"autoSubmit": true` dalam objek pengaturan `voice` untuk mengirim prompt secara otomatis saat Anda melepaskan kunci, asalkan transkrip setidaknya tiga kata panjang.

<h2 id="tap-to-record-and-send">
  Ketuk untuk merekam dan mengirim
</h2>

Mode ketuk mengalihkan perekaman dengan penekanan tombol tunggal: ketuk sekali untuk memulai, berbicara, kemudian ketuk lagi untuk mengirim prompt. Tidak ada pemanasan, dan Anda tidak perlu menahan kunci.

Aktifkan mode ketuk dengan `/voice tap`. Dengan input prompt kosong, ketuk `Space` untuk mulai merekam. Footer menampilkan `● REC · tap to send` saat merekam. Ketuk `Space` lagi untuk berhenti.

Claude Code menyisipkan transkrip dan mengirimkan prompt secara otomatis saat transkrip setidaknya tiga kata panjang. Transkrip yang lebih pendek dimasukkan tetapi tidak dikirim, sehingga ketukan yang tidak disengaja tidak mengirim kata yang tersesat.

Ambang batas tiga kata menghitung kata untuk bahasa yang ditulis tanpa spasi. Jepang, Cina, dan Thailand menghitung kata individual, sehingga mereka auto-submit dalam mode ketuk dan dalam mode tahan dengan `autoSubmit`.

Ketukan pertama hanya mulai merekam saat input prompt kosong, sehingga Anda masih dapat mengetik spasi secara normal saat menyusun pesan. Ketukan kedua menghentikan perekaman terlepas dari isi input. Perekaman juga berhenti secara otomatis setelah 15 detik keheningan atau dua menit total.

<h2 id="cancel-a-recording">
  Batalkan perekaman
</h2>

Tekan `Esc` atau `Ctrl+C` untuk membatalkan diksi alih-alih menyelesaikannya. Claude Code menghentikan mikrofon, membuang transkrip, dan mengembalikan prompt ke keadaan sebelum perekaman dimulai.

Kedua tombol juga membatalkan saat transkrip perekaman yang selesai masih diproses. Prompt yang Anda edit atau kirimkan selama pemrosesan tetap seperti yang Anda tinggalkan.

Tidak ada tombol yang melakukan apa pun di tekan lain yang membatalkan: `Esc` tidak mengganggu respons Claude, dan `Ctrl+C` tidak menghapus prompt atau dihitung sebagai yang pertama dari [dua penekanan yang keluar dari Claude Code](/docs/id/interactive-mode#general-controls).

<h2 id="change-the-dictation-language">
  Ubah bahasa dikte
</h2>

Dikte suara menggunakan pengaturan [`language`](/docs/id/settings-reference#language) yang sama yang mengontrol bahasa respons Claude. Jika pengaturan itu kosong, dikte default ke Bahasa Inggris. Di ekstensi VS Code, jika `language` kosong, dikte menggunakan pengaturan `accessibility.voice.speechLanguage` VS Code sebelum default ke Bahasa Inggris.

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
  Troubleshooting
</h2>

Masalah umum ketika voice dictation tidak aktif atau merekam:

* **`Voice mode requires a Claude.ai account`**: Anda diautentikasi dengan API key atau penyedia pihak ketiga. Jalankan `/login` untuk masuk dengan akun claude.ai.
* **`Voice mode is disabled by your organization's policy`**: kebijakan administrator untuk organisasi Anda mematikan voice dictation. Hubungi administrator organisasi Anda untuk mengonfirmasi apakah voice dictation tersedia untuk organisasi Anda.
* **`Microphone access is denied`**: berikan izin mikrofon ke terminal Anda di pengaturan sistem. Di macOS, buka System Settings → Privacy & Security → Microphone dan aktifkan aplikasi terminal Anda, kemudian jalankan `/voice` lagi. Di Windows, buka Settings → Privacy & security → Microphone dan aktifkan akses mikrofon untuk aplikasi desktop, kemudian jalankan `/voice` lagi. Jika terminal Anda tidak terdaftar dalam pengaturan macOS, lihat [Terminal not listed in macOS Microphone settings](#terminal-not-listed-in-macos-microphone-settings).
* **`Voice mode requires SoX for audio recording` on Linux**: modul audio native tidak dapat dimuat dan tidak ada fallback yang terinstal. Instal SoX dengan perintah yang ditampilkan dalam pesan kesalahan, misalnya `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX terinstal, tetapi host tidak memiliki perangkat penangkap audio, misalnya server headless atau container. Jalankan Claude Code di mesin yang memiliki mikrofon. Mulai dari v2.1.195, Claude Code di Linux melaporkan pesan ini dalam situasi tersebut; versi sebelumnya meminta Anda untuk menginstal SoX bahkan ketika sudah terinstal.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg merutekan audio melalui PulseAudio daripada perangkat ALSA, jadi SoX memerlukan backend PulseAudio-nya terinstal secara eksplisit. Jalankan `sudo apt install sox libsox-fmt-pulse`. Menginstal `sox` saja menarik backend ALSA, yang tidak dapat merekam di WSL karena tidak ada perangkat `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`**: voice dictation mengalami tiga kegagalan penangkapan dalam 10 detik. Claude Code menjeda dictation hingga 10 detik telah berlalu sejak yang pertama dari kegagalan tersebut. Kegagalan dihitung apakah mikrofon gagal dimulai atau perekam dimulai dan kemudian berhenti tanpa menghasilkan audio apa pun. Ini biasanya berarti mikrofon atau audio stack di host ini tidak dapat menangkap audio, misalnya server headless, shell jarak jauh tanpa passthrough audio, atau izin mikrofon ditolak. Konfirmasi perangkat input yang berfungsi, perbaiki penyebab mendasar dari entri di atas, kemudian picu voice lagi. Sebelum v2.1.202, hanya kegagalan start-up yang diperhitungkan menuju jeda.
* **Nothing happens when holding `Space` in hold mode**: perhatikan input prompt saat Anda menahan. Jika spasi terus bertambah, voice dictation kemungkinan besar mati; jalankan `/voice hold` untuk mengaktifkannya. Jika hanya satu atau dua spasi muncul dan kemudian tidak ada, voice dictation aktif tetapi deteksi hold tidak terpicu. Deteksi hold memerlukan terminal Anda untuk mengirim key-repeat events, jadi tidak dapat mendeteksi kunci yang ditahan jika key-repeat dinonaktifkan di tingkat OS. Beralih ke tap mode dengan `/voice tap` untuk menghindari persyaratan key-repeat.
* **Tapping `Space` types a space instead of recording in tap mode**: ketukan pertama hanya memulai perekaman ketika input prompt kosong. Hapus input terlebih dahulu, atau periksa bahwa Anda dalam tap mode dengan menjalankan `/voice tap`.
* **`No audio detected from microphone`**: perekaman dimulai tetapi menangkap kesunyian. Konfirmasi perangkat input yang benar diatur sebagai default sistem dan tingkat inputnya tidak dibisukan atau mendekati nol. Di Windows, buka Settings → System → Sound → Input dan pilih mikrofon Anda. Di macOS, buka System Settings → Sound → Input.
* **`Voice connection failed`**: perekaman Anda tidak pernah mencapai layanan transkripsi karena koneksi gagal. Periksa jaringan Anda dan coba lagi. Perekaman yang tidak menangkap audio melaporkan `No audio detected from microphone` alih-alih pesan ini. Sebelum v2.1.200, mikrofon senyap dapat melaporkan kegagalan koneksi, yang menyarankan masalah jaringan ketika masalah sebenarnya adalah perangkat input.
* **`Voice stream error: WebSocket upgrade rejected with HTTP <status>`**: server menolak koneksi Anda dengan status HTTP yang ditampilkan, jadi ini bukan pemadaman jaringan. Status dalam rentang 400 biasanya berarti sign-in basi, atau layanan proxy atau bot-protection menjawab sebagai pengganti layanan transkripsi. Jalankan `/login` untuk menyegarkan sign-in Anda, dan periksa VPN atau proxy di jalur jaringan Anda jika status berlanjut. Jika Anda masih merekam ketika penolakan tiba, Claude Code mencoba ulang status di luar rentang 400 sekali sebelum menampilkan pesan ini; tidak mencoba ulang status dalam rentang 400. Dalam v2.1.229 hingga v2.1.231, build native tidak menampilkan pesan ini: Claude Code terus merekam, footer hold-mode masih menampilkan `listening…`, dan melaporkan `Voice connection failed` setelah Anda berhenti merekam.
* **`No speech detected`**: audio mencapai layanan transkripsi tetapi tidak ada kata yang dikenali. Berbicara lebih dekat ke mikrofon, kurangi kebisingan latar belakang, dan konfirmasi [dictation language](#change-the-dictation-language) Anda cocok dengan bahasa yang Anda gunakan.
* **Transcription is garbled or in the wrong language**: dictation default ke English. Jika Anda mendiktekan dalam bahasa lain, atur terlebih dahulu di `/config`. Lihat [Change the dictation language](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal not listed in macOS Microphone settings
</h3>

Jika aplikasi terminal Anda tidak muncul di bawah System Settings → Privacy & Security → Microphone, tidak ada toggle yang dapat Anda aktifkan. Atur ulang status izin untuk terminal Anda sehingga `/voice` run berikutnya memicu prompt izin macOS yang segar.

<Steps>
  <Step title="Reset the microphone permission for your terminal">
    Jalankan `tccutil reset Microphone <bundle-id>`, mengganti `<bundle-id>` dengan identifier terminal Anda: `com.apple.Terminal` untuk Terminal bawaan, atau `com.googlecode.iterm2` untuk iTerm2. Untuk terminal lain, cari identifier dengan `osascript -e 'id of app "AppName"'`.

    <Warning>
      Anda dapat menjalankan `tccutil reset Microphone` tanpa bundle ID, tetapi ini mencabut akses mikrofon dari setiap aplikasi di Mac Anda, termasuk aplikasi seperti Zoom atau Slack. Setiap aplikasi akan perlu meminta akses lagi pada penggunaan berikutnya, jadi jangan jalankan selama panggilan aktif.
    </Warning>
  </Step>

  <Step title="Quit and relaunch your terminal">
    macOS tidak akan meminta ulang proses yang sudah berjalan. Keluar dari aplikasi terminal dengan Cmd+Q, bukan hanya menutup jendelanya, kemudian buka lagi.
  </Step>

  <Step title="Trigger a fresh prompt">
    Mulai Claude Code dan jalankan `/voice`. macOS meminta akses mikrofon; izinkan.
  </Step>
</Steps>

<h2 id="see-also">
  Lihat juga
</h2>

* [Sesuaikan pintasan keyboard](/docs/id/keybindings): ikat ulang `voice:pushToTalk` dan tindakan keyboard CLI lainnya
* [Semua pengaturan](/docs/id/settings-reference#voice): kunci pengaturan `voice`, `language`, dan lainnya
* [Mode interaktif](/docs/id/interactive-mode): pintasan keyboard, mode input, dan kontrol sesi
* [Perintah](/docs/id/commands): referensi untuk `/voice`, `/config`, dan semua perintah lainnya
