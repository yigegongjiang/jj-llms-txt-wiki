> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit komunikasi

> Luncurkan pengumuman, pesan kampanye bertahap, dan respons FAQ untuk meluncurkan Claude Code ke organisasi teknik Anda.

Halaman ini untuk administrator dan pemimpin teknik yang meluncurkan Claude Code ke tim. Halaman ini menyediakan pengumuman peluncuran siap pakai, kampanye tips-dan-trik bertahap, dan respons FAQ satu baris untuk pertanyaan yang paling sering Anda terima.

<Note>
  Perlakukan semuanya di sini sebagai naskah draf, bukan naskah final. Tulis ulang setiap pesan dengan suara organisasi Anda, tukar tugas contoh dengan bug dan modul nyata dari basis kode Anda sendiri, dan ganti `[placeholder dalam kurung]` sebelum mengirim. Pengumuman yang mendorong adopsi adalah yang terdengar seperti ditulis oleh seseorang di perusahaan Anda.
</Note>

<h2 id="launch-communications">
  Komunikasi peluncuran
</h2>

Satu pengumuman dalam dua format, ditambah dua varian opsional. Pilih yang paling sesuai dengan peluncuran Anda dan tulis ulang dari sana.

<h3 id="before-you-send">
  Sebelum Anda mengirim
</h3>

Kerjakan daftar periksa ini sebelum pengumuman dikirim. Setiap item menutup celah yang sebaliknya berubah menjadi utas dukungan hari peluncuran.

| Item                                                                                                   | Mengapa penting                                                                                                                                                  |
| ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Saluran `#claude-code` dibuat dan ditautkan dalam pesan                                                | Memberikan pertanyaan satu tempat untuk mendarat                                                                                                                 |
| Perintah instalasi diuji pada setidaknya satu mesin di lingkungan Anda                                 | Menangkap masalah proxy atau firewall sebelum semua orang mengalaminya sekaligus                                                                                 |
| Tautan keamanan dan penanganan data siap ([Penggunaan data](/docs/id/data-usage) atau setara internal Anda) | "Ke mana kode saya pergi?" akan menjadi balasan pertama                                                                                                          |
| Satu tugas pertama konkret dipilih, bug nyata atau file di basis kode Anda                             | Contoh generik tidak mengkonversi; "perbaiki tes yang tidak stabil di `auth_test.go`" melakukannya                                                               |
| Pemilik bernama untuk saluran selama 48 jam pertama                                                    | Pertanyaan hari peluncuran yang tidak terjawab membunuh momentum                                                                                                 |
| Sponsor C-suite yang siap mengirim atau menandatangani pengumuman                                      | Peluncuran yang dikirim dengan nama eksekutif secara konsisten melihat adopsi minggu pertama yang lebih tinggi daripada yang dikirim oleh admin atau tim tooling |

<h3 id="the-announcement">
  Pengumuman
</h3>

Gunakan ini sebagai pesan peluncuran organisasi luas standar Anda. Ini mencakup apa itu Claude Code, memberikan jalur instalasi dua menit, memberikan pembaca satu tugas konkret untuk dicoba, dan menjawab "ke mana kode saya pergi?" sebelum siapa pun harus bertanya.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Subject: Claude Code aktif untuk [Engineering / tim Anda]

    Tim,

    Mulai hari ini Anda memiliki akses ke Claude Code, agen pengkodean AI yang berjalan di
    terminal Anda, membaca basis kode aktual Anda, dan menyelesaikan tugas nyata dari awal hingga akhir:
    debugging, refactor, tes, PR. Ini bukan autocomplete dan bukan jendela chat. Ini mengedit file,
    menjalankan perintah Anda, dan meminta izin sebelum apa pun yang berisiko.

    Mulai berjalan dalam dua menit:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <repo-Anda>
        claude

    Kemudian jalankan /init sekali. Claude membaca proyek Anda dan menulis CLAUDE.md dengan
    perintah build dan konvensi Anda, sehingga Anda berhenti menjelaskan ulang dasar-dasarnya.

    Kemudian coba salah satu dari ini di repo yang sudah Anda gunakan:

      - "Tes di [file] tidak stabil. Cari tahu mengapa dan perbaiki"
      - "Jelaskan kepada saya bagaimana [modul] menangani [X]"
      - "Lihat diff kerja saya dan beri tahu saya apa yang berisiko sebelum saya push"

    Ke mana kode Anda pergi: Claude Code berjalan di terminal Anda dan berbicara langsung
    ke API Anthropic, tanpa server pihak ketiga dalam loop. Ini meminta sebelum
    mengedit file atau menjalankan perintah. Di bawah perjanjian Enterprise kami, Anthropic
    tidak menggunakan kode atau prompt Anda untuk melatih modelnya.
    Detail: https://code.claude.com/docs/en/data-usage
             https://code.claude.com/docs/en/security

    Di mana harus pergi dengan pertanyaan: #claude-code. [Nama pemilik] mengawasi
    minggu ini.

    - [Nama]

    P.S. Lebih suka editor Anda? Ada ekstensi VS Code dan plugin JetBrains.
    Agen yang sama, tidak perlu terminal.
    ```
  </Tab>

  <Tab title="Slack atau Teams">
    ```markdown theme={null}
    🚀 *Claude Code aktif untuk [tim]*

    Agen pengkodean AI, berjalan di terminal Anda, membaca repo Anda, melakukan pekerjaan nyata:
    bug, refactor, tes, PR. Meminta sebelum menyentuh apa pun.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd repo-Anda` → `claude`

    *Hal pertama untuk dicoba* → jalankan `/init`, kemudian: "tes di [file] tidak stabil,
    cari tahu mengapa dan perbaiki."

    🔒 Berjalan di terminal Anda, berbicara hanya ke API Anthropic. Di bawah paket
    Enterprise kami, kode dan prompt Anda tidak digunakan untuk melatih model.
    Penggunaan data → https://code.claude.com/docs/en/data-usage

    📚 Quickstart · VS Code · Kursus gratis 1 jam
       https://code.claude.com/docs/en/quickstart
       https://code.claude.com/docs/en/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Pertanyaan → utas ini. [Pemilik] sedang mengerjakan.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Varian sponsor eksekutif
</h3>

Kirim ini dari eksekutif sponsor Anda, seperti CTO, CIO, atau SVP Engineering, dengan nama mereka dan dari akun mereka. Peluncuran yang dikirim dengan nama eksekutif secara konsisten melihat tingkat pembukaan yang lebih tinggi dan aktivasi minggu pertama yang lebih cepat daripada pesan yang sama dari admin atau tim tooling. Ini menandakan prioritas perusahaan daripada eksperimen opsional.

Versi ini sengaja dikurangi menjadi satu permintaan: instal dan jalankan pada satu tugas nyata. Pekerjaan eksekutif adalah membuat permintaan mendarat; pengumuman standar dan `#claude-code` menangani caranya.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Subject: Satu hal yang ingin saya coba setiap insinyur minggu ini

    Tim,

    Kami telah mengaktifkan Claude Code untuk semua teknik. Ini adalah agen AI
    yang bekerja langsung di terminal Anda, pada basis kode aktual Anda, dan
    hasil awal dari tim yang sudah menggunakannya cukup kuat sehingga saya ingin
    semua orang menggunakannya minggu ini.

    Saya meminta sepuluh menit:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <repo-Anda>
        claude

    Kemudian berikan satu tugas nyata: bug yang telah Anda tunda, atau "jelaskan
    kepada saya bagaimana [modul] bekerja."

    Itu saja permintaannya. [Nama pemilik] dan tim ada di #claude-code untuk
    apa pun yang Anda alami.

    - [Nama Eksekutif]
      [Judul]
    ```
  </Tab>

  <Tab title="Slack atau Teams">
    ```markdown theme={null}
    📣 *Dari [Nama Eksekutif]: satu hal untuk dicoba minggu ini*

    Kami telah mengaktifkan *Claude Code* untuk semua teknik. Hasil awal cukup
    kuat sehingga saya meminta semua orang untuk memberikan sepuluh menit pada
    pekerjaan nyata minggu ini.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd repo-Anda` →
    `claude` → berikan satu tugas nyata.

    Itu saja. Pertanyaan → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Varian grup pilot
</h3>

Gunakan untuk peluncuran bertahap. Kirim hanya ke kohort pilot.

```text theme={null}
Subject: Anda dalam pilot Claude Code

[Nama / tim],

Anda berada dalam gelombang pertama Claude Code di [perusahaan]. Kami memilih grup ini
karena Anda akan menggunakannya pada masalah nyata dan memberi tahu kami kebenaran tentangnya.

Permintaan: gunakan pada setidaknya satu tugas nyata minggu ini, kemudian tulis catatan di
#claude-code-pilot yang mencakup apa yang berhasil, apa yang mengganggu, dan apa yang
mengejutkan Anda. Umpan balik itu menentukan bagaimana kami meluncurkannya ke semua orang.

[Lanjutkan dengan "Mulai berjalan dalam dua menit" dari pengumuman standar]

Satu hal ekstra untuk pilot: pada perubahan multi-file pertama Anda, tekan Shift+Tab
sampai Anda melihat "plan". Claude akan meletakkan persis apa yang dimaksudkan untuk dilakukan
sebelum menyentuh file. Ini adalah cara tercepat untuk mengkalibrasi berapa banyak yang harus
dipercayai.
```

<h3 id="champion-recruitment-dm">
  DM perekrutan juara
</h3>

Setelah peluncuran, DM dua atau tiga orang yang paling aktif di `#claude-code`.

```text theme={null}
Hei [nama], posting #claude-code Anda melakukan lebih banyak untuk adopsi daripada
pengumuman saya. Beberapa orang memberi tahu saya bahwa [utas / tangkapan layar] Anda
adalah alasan mereka benar-benar mencobanya.

Ingin membuat itu semi-resmi? Beban ringan: sebagian besar terus memposting apa yang
Anda posting, ditambah akses pertama ke fitur baru dan jalur langsung ke tim Anthropic.
Saya dapat berbagi playbook singkat jika Anda tertarik.
```

<h2 id="tips-and-tricks-campaign">
  Kampanye tips dan trik
</h2>

Pesan Slack atau Teams siap tempel yang dirancang untuk mendorong aktivasi fitur setelah peluncuran. Masing-masing mengikuti pola yang sama: hook, hasil, prompt "coba sekarang", dan tautan docs. Teteskan satu atau dua per minggu di `#claude-code`, atau pilih segelintir yang cocok dengan celah tim Anda. Mereka berdiri sendiri tanpa urutan yang diperlukan.

Salin badan pesan dari setiap blok langsung ke Slack atau Teams. Ganti `[placeholder dalam kurung]` sebelum mengirim.

<h3 id="get-started">
  Memulai
</h3>

**Memilih model yang tepat**

```markdown theme={null}
🎯 *Tip: Cocokkan model dengan momen*

Menggunakan Opus untuk memperbaiki typo membakar komputasi. Menggunakan Haiku untuk refactor 12 file
adalah meminta pengerjaan ulang.

Claude Code berjalan pada model yang sama dengan aplikasi Claude, dan Anda dapat beralih
di tengah sesi. *Sonnet* adalah default workhorse untuk pekerjaan fitur sehari-hari,
bug, tes, dan review. Gunakan *Opus* untuk refactor besar, debugging rumit, atau apa pun
yang berisiko tinggi. Turun ke *Haiku* untuk pertanyaan cepat, pemformatan, dan edit mekanis
di mana kecepatan menang. *Fable 5* adalah model paling mampu untuk tugas-tugas tersulit dan
paling lama; itu bukan default, jadi pilih dengan `/model fable`, dan perhatikan bahwa konten
cybersecurity dan biology kembali ke Opus secara otomatis.

*Coba sekarang:* ketik `/model` dan pilih Sonnet jika Anda belum melakukannya. Ini adalah
default yang tepat untuk sebagian besar tugas.

📖 Konfigurasi model → https://code.claude.com/docs/id/model-config
```

| Model   | Terbaik untuk                                                                                                                                                                  |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Fable 5 | Tugas-tugas tersulit dan paling lama. Opt-in saja: pilih dengan `/model fable`. Konten cybersecurity atau biology [kembali ke Opus](/docs/id/model-config#automatic-model-fallback) |
| Opus    | Refactor skala besar, debugging kompleks, keputusan arsitektur, perubahan berisiko tinggi                                                                                      |
| Sonnet  | Pekerjaan fitur sehari-hari, perbaikan bug, tes, dokumentasi, review kode. Default yang direkomendasikan.                                                                      |
| Haiku   | Pertanyaan cepat, pemformatan, edit mekanis, iterasi cepat                                                                                                                     |

**Kemenangan cepat untuk dicoba terlebih dahulu**

```markdown theme={null}
🚀 *Tip: Tiga hal untuk dicoba dalam 10 menit pertama Anda*

Menginstal Claude Code tetapi tidak yakin apa yang sebenarnya harus diminta? Mulai dengan
hal yang telah mengganggu Anda sepanjang minggu.

  - Perbaiki sesuatu yang mengganggu: "tes di [file] tidak stabil, cari tahu mengapa"
  - Orientasikan diri di kode yang tidak Anda tulis: "jelaskan kepada saya bagaimana [modul] bekerja"
  - Sanity-check sebelum Anda push: "lihat diff kerja saya dan beri tahu saya apa yang
    terlihat berisiko"

Tidak ada yang memerlukan setup. Cukup `cd` ke repo Anda dan jalankan `claude`.

*Coba sekarang:* pilih bug yang telah Anda hindari dan tempel pesan kesalahan.

📖 Quickstart → https://code.claude.com/docs/id/quickstart
```

<h3 id="project-memory">
  Memori proyek
</h3>

**`/init` dan CLAUDE.md**

```markdown theme={null}
📁 *Tip: Berhenti menjelaskan ulang repo Anda setiap sesi*

Mengatakan Claude "kami menggunakan pnpm, bukan npm" untuk kelima kalinya? Ada
perbaikan satu kali.

Jalankan `/init` sekali per repo. Claude membaca struktur proyek Anda dan menulis file
CLAUDE.md dengan perintah build, arsitektur, dan konvensi Anda. Setiap sesi masa depan
di repo itu dimulai dari file ini secara otomatis. Simpan di bawah dua layar. Ini adalah
lembar cheat, bukan dokumentasi.

*Coba sekarang:* buka repo utama Anda, jalankan `claude`, ketik `/init`. Tiga puluh
detik, membayar setiap sesi setelahnya.

📖 CLAUDE.md dan memori proyek → https://code.claude.com/docs/id/memory
```

**Referensi @**

```markdown theme={null}
📎 *Tip: Berhenti menempel konten file ke dalam chat*

Menyalin 200 baris komponen ke prompt Anda sehingga Claude dapat "melihatnya"?
Anda tidak harus.

Ketik `@` kemudian jalur file. Claude menarik file langsung ke konteks.
Bekerja untuk seluruh direktori juga.

> gaya di @src/components/Button.tsx terlihat salah, periksa terhadap
> @docs/design-system.md

*Coba sekarang:* ketik `@` kemudian Tab. Autocomplete menunjukkan Anda setiap file dalam jangkauan.

📖 Mereferensikan file → https://code.claude.com/docs/id/common-workflows
```

<h3 id="control-and-safety">
  Kontrol dan keamanan
</h3>

**Mode izin**

```markdown theme={null}
🛡️ *Tip: Satu keystroke antara "lihat tetapi jangan sentuh" dan "lakukan saja"*

Kadang-kadang Anda ingin Claude meminta sebelum setiap edit. Kadang-kadang Anda hanya ingin
itu dikirim. Anda tidak harus memilih satu selamanya.

*Shift+Tab* bersiklus melalui berapa banyak yang dapat dilakukan Claude tanpa bertanya: *Manual* (nilai
pengaturan `default`) meminta sebelum setiap tindakan, *acceptEdits* membiarkan edit file dan perintah
filesystem umum mengalir sementara masih memeriksa sebelum perintah shell lainnya, dan *plan*
mengusulkan perubahan untuk persetujuan Anda sebelum apa pun disentuh. Mode plan adalah pembangun
kepercayaan, jadi mulai dari sana untuk apa pun yang menyentuh beberapa file.

*Coba sekarang:* pada refactor berikutnya, tekan Shift+Tab sampai Anda melihat "plan",
kemudian jelaskan perubahan. Anda akan mendapatkan proposal lengkap sebelum file tunggal bergerak.

📖 Mode izin → https://code.claude.com/docs/id/permissions
```

**Checkpointing dan `/rewind`**

```markdown theme={null}
⏪ *Tip: Ada tombol undo untuk seluruh percakapan*

Claude pergi ke jalur yang salah tiga putaran yang lalu dan sekarang Anda membongkarnya?
Anda tidak harus memperbaiki maju.

`/rewind` menggulung kembali ke titik sebelumnya dalam percakapan, termasuk
perubahan file yang dibuat Claude. Checkpointing otomatis; Anda tidak mengatur apa pun.

*Coba sekarang:* tekan *Esc* dua kali untuk membuka menu rewind, atau ketik `/rewind`.
Pilih titik sebelum hal-hal menjadi miring.

📖 Checkpointing → https://code.claude.com/docs/id/checkpointing
```

<h3 id="connect-your-tools">
  Hubungkan alat Anda
</h3>

**Konektor MCP**

```markdown theme={null}
🔌 *Tip: Biarkan Claude membaca pelacak masalah Anda sehingga Anda tidak harus menempel tiket*

Menyalin-menempel tiket Jira ke terminal terasa seperti langkah mundur.
Memang demikian.

Satu file konfigurasi (`.mcp.json` di root proyek Anda) menghubungkan Claude ke GitHub,
Jira, Linear, atau tracker apa pun yang Anda gunakan. Kemudian "apa masalah prioritas teratas
yang ditugaskan kepada saya?" dan "lanjutkan dan perbaiki" terjadi dalam percakapan yang sama.

*Coba sekarang:* tanya Claude "atur konektor MCP untuk [GitHub/Jira/Linear]
di repo ini". Itu akan menulis konfigurasi untuk Anda.

📖 Konektor MCP → https://code.claude.com/docs/id/mcp
```

<h3 id="automate-your-workflows">
  Otomatisasi alur kerja Anda
</h3>

**Skills**

```markdown theme={null}
⚡ *Tip: Ubah prompt yang terus Anda ketik ulang menjadi perintah*

Mengetik "ringkas apa yang saya kerjakan hari ini dari git log, format untuk standup"
tiga kali minggu ini? Itu adalah slash command yang menunggu untuk terjadi.

File SKILL.md di `.claude/skills/<name>/` menjadi prompt yang dapat digunakan kembali; ketik
`/name` untuk menjalankannya. Buat satu kali kedua Anda mengetik prompt multi-langkah yang
telah Anda ketik sebelumnya. Jalur termudah: minta Claude untuk membuatnya untuk Anda.

*Coba sekarang:* ketik "buat saya skill /standup yang merangkum apa yang saya kerjakan
hari ini dari git log", kemudian jalankan `/standup` besok pagi.

📖 Skills → https://code.claude.com/docs/id/skills
```

**Hooks**

```markdown theme={null}
🔔 *Tip: Dapatkan ping saat refactor Anda selesai*

Duduk di meja Anda menonton Claude menyelesaikan tugas panjang? Anda memiliki
hal yang lebih baik untuk dilakukan selama delapan menit itu.

Hooks adalah perintah shell yang dijalankan pada acara Claude Code. Hook Stop yang
mengirim notifikasi desktop berarti Anda dapat memulai refactor panjang, pergi,
dan mendapatkan ping saat selesai.

*Coba sekarang:* tanya Claude "tambahkan Hook Stop yang mengirim notifikasi desktop
saat Anda selesai". Itu akan menulis skrip dan menghubungkannya.

📖 Panduan hooks → https://code.claude.com/docs/id/hooks-guide
```

<h3 id="day-to-day-development">
  Pengembangan sehari-hari
</h3>

**Tangkapan layar dan gambar**

```markdown theme={null}
📸 *Tip: Berhenti menjelaskan dialog kesalahan. Tunjukkan saja.*

Mengetik "ada kotak merah yang mengatakan sesuatu tentang referensi null
dan menunjuk ke baris 47-ish"? Tangkap layarnya.

Seret tangkapan layar langsung ke terminal dan Claude melihatnya: dialog kesalahan, mockup UI,
foto whiteboard, ekspor Figma. *Ctrl+V* menempel dari clipboard (gunakan Ctrl+V di macOS juga,
bukan Cmd+V).

*Coba sekarang:* lain kali sesuatu yang visual rusak, tangkap layarnya dan tempel langsung
ke prompt. Kemudian cukup ketik "apa yang salah di sini?"

📖 Bekerja dengan gambar → https://code.claude.com/docs/id/common-workflows
```

**Alur kerja Git**

```markdown theme={null}
🌿 *Tip: Serahkan seluruh upacara git*

Perbaikan memakan waktu 5 menit. Pesan commit, branch, dan deskripsi PR
memakan waktu 15. Rasio itu salah.

Claude menangani alur git lengkap: commit dengan pesan konvensional,
branch, PR dengan ringkasan yang tepat. Satu permintaan: "perbaiki off-by-one, commit
dengan pesan commit konvensional, dan buka PR." Meninjau pekerjaan orang lain?
Tempel URL PR dan minta Claude untuk menjelaskan diff kepada Anda.

*Coba sekarang:* setelah perbaikan berikutnya, alih-alih beralih ke klien git Anda,
cukup ketik "commit ini dengan pesan yang baik dan buka PR".

📖 Membuat pull request → https://code.claude.com/docs/id/common-workflows
```

<h3 id="share-and-scale">
  Bagikan dan skalakan
</h3>

**Plugins**

```markdown theme={null}
📦 *Tip: Seseorang mungkin sudah membangun skill itu*

Tentang untuk menghabiskan satu jam membangun perintah `/deploy`? Periksa apakah
sudah ada.

Skills dikemas dan dibagikan sebagai plugin. `/plugin` menelusuri apa yang
tersedia dan menginstal dalam satu langkah. Lima menit browsing dapat menghemat
satu jam pembangunan.

*Coba sekarang:* ketik `/plugin` dan gulir. Anda akan menemukan setidaknya satu
hal yang tidak Anda tahu Anda inginkan.

📖 Plugins → https://code.claude.com/docs/id/plugins
```

<h3 id="security-and-admin">
  Keamanan dan admin
</h3>

**Arsitektur keamanan**

```markdown theme={null}
🔐 *Tip: Jawaban untuk "apakah ini aman?" untuk lain kali Anda ditanya*

Seseorang di tim Anda akan bertanya "tunggu, ke mana kode saya pergi?"
Berikut versi singkat yang dapat Anda tempel.

Izin-pertama dengan desain. Setiap edit file, perintah shell, dan panggilan eksternal
dijaga oleh persetujuan Anda. CLI berjalan di terminal Anda dan berbicara langsung
ke API Anthropic, tanpa server pihak ketiga, dan mendukung sandboxing tingkat OS opsional
untuk perintah shell. Di bawah paket Enterprise kami, Anthropic tidak menggunakan kode
atau prompt Anda untuk melatih modelnya.

*Coba sekarang:* simpan dua tautan ini untuk lain kali pertanyaan muncul.
Mereka menjawab sebagian besar pertanyaan review keamanan.

📖 https://code.claude.com/docs/id/security
📖 https://code.claude.com/docs/id/data-usage
```

**Praktik terbaik**

```markdown theme={null}
✅ *Tip: 4 kebiasaan yang memisahkan "mencoba sekali" dari "gunakan setiap hari"*

Sebagian besar orang yang terpental dari Claude Code melewatkan salah satu dari ini.
Sebagian besar orang yang bertahan melakukan keempat-empatnya dalam minggu pertama.

  - Mulai dalam mode plan untuk apa pun yang menyentuh beberapa file
  - Jalankan /init lebih awal; konteks bertambah
  - Tinjau diff sebelum commit; Claude dapat dengan percaya diri salah
  - Verifikasi perubahan yang menyentuh jalur kritis; perlakukan seperti junior
    yang tajam, bukan oracle

*Coba sekarang:* jika Anda hanya melakukan satu atau dua dari ini, pilih yang Anda
lewatkan dan lakukan pada tugas berikutnya. Posting apa yang berubah di #claude-code.

📖 Praktik terbaik → https://code.claude.com/docs/id/best-practices
```

<h2 id="quick-reference">
  Referensi cepat
</h2>

<h3 id="faq-responses">
  Respons FAQ
</h3>

Balasan satu baris untuk pertanyaan yang paling sering Anda terima.

| Pertanyaan                                                  | Respons                                                                                                                                                                                                                                                                                                                                                                                                          |
| ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Apakah itu bekerja di VS Code?"                            | Ya. Ada ekstensi VS Code dan plugin JetBrains dengan fitur yang sama, tertanam di editor Anda. [VS Code →](/docs/id/vs-code)                                                                                                                                                                                                                                                                                          |
| "Apakah saya harus mengonfigurasi apa pun terlebih dahulu?" | Tidak. Instal, kemudian jalankan `claude` di repo apa pun. Jalankan `/init` sekali dan Anda siap. [Quickstart →](/docs/id/quickstart)                                                                                                                                                                                                                                                                                 |
| "Di mana kode saya pergi?"                                  | CLI berjalan di terminal Anda dan mengirim konteks ke API Anthropic untuk inferensi, tanpa server pihak ketiga. Di bawah paket Enterprise Anda, kode dan prompt Anda tidak digunakan untuk melatih model. [Penggunaan data →](/docs/id/data-usage)                                                                                                                                                                    |
| "Bisakah itu melihat seluruh repo saya?"                    | Itu membaca apa yang Anda berikan akses. Pembacaan file di dalam direktori kerja Anda tidak meminta; prompt izin menjaga edit, perintah shell non-read-only, dan pembacaan file-tool di luar direktori itu. Serangkaian perintah shell read-only bawaan seperti `ls` dan `cat` berjalan tanpa meminta; batasi dengan [aturan sandbox `denyRead`](/docs/id/sandboxing#filesystem-isolation). [Izin →](/docs/id/permissions) |
| "Bagaimana ini berbeda dari Copilot?"                       | Copilot melengkapi baris. Claude Code adalah agen yang membaca file, menjalankan perintah, dan membuat edit multi-file. [Ikhtisar →](/docs/id/overview)                                                                                                                                                                                                                                                               |
| "Apa yang harus saya coba terlebih dahulu?"                 | Bug yang telah Anda tunda karena membosankan. "Tes di \[file] tidak stabil, cari tahu mengapa." [Quickstart →](/docs/id/quickstart)                                                                                                                                                                                                                                                                                   |

<h3 id="prompt-templates">
  Template prompt
</h3>

Bagikan prompt pemula ini dengan insinyur yang telah menginstal tetapi tidak yakin apa yang harus diminta. Masing-masing ditulis dengan cara yang akan diketik ke sesi nyata; ganti bagian dalam kurung dengan file dari repo Anda sendiri.

| Tugas                 | Prompt                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------- |
| Perbaiki bug          | "tes di \[file] gagal, cari tahu mengapa dan perbaiki"                                            |
| Pahami kode           | "jelaskan kepada saya bagaimana \[modul] bekerja, kemudian beri tahu saya di mana titik masuknya" |
| Refactor aman         | "refactor \[modul] ke \[tujuan], gunakan mode plan sehingga saya dapat meninjau terlebih dahulu"  |
| Tulis tes             | "tulis tes untuk \[file] yang mencakup kasus tepi di sekitar \[skenario]"                         |
| Tinjau sebelum commit | "lihat diff kerja saya dan beri tahu saya apa yang terlihat berisiko"                             |
| Buka PR               | "perbaiki \[masalah], tulis commit konvensional, dan buka PR dengan ringkasan"                    |
| Buat skill            | "buat saya skill /ship yang menjalankan tes dan lint sebelum commit"                              |
| Debug stack trace     | "berikut stack trace, temukan akar penyebabnya, jangan hanya menutupinya"                         |

<Tip>
  Claude Code dikirim dengan sering. Verifikasi detail spesifik versi terhadap [halaman beranda dokumentasi](/docs/id/overview) sebelum mendistribusikan secara internal.
</Tip>
