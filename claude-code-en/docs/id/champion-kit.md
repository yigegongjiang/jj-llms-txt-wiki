> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Champion kit

> Panduan untuk insinyur yang mengadvokasi Claude Code secara internal: apa yang harus dibagikan, cara menjawab pertanyaan, dan cara meningkatkan adopsi di tim Anda.

Halaman ini untuk insinyur individual yang sudah menggunakan Claude Code dan ingin membantu tim mereka mengadopsinya. Ini mencakup apa yang harus dibagikan, cara menjawab pertanyaan yang akan Anda terima, playbook tiga puluh hari, dan respons terhadap kekhawatiran umum.

Adopsi alat pengembang jarang terjadi karena pengumuman peluncuran. Ini terjadi karena seseorang di tim mulai menggunakan alat dengan baik, membicarakannya secara terbuka, dan memudahkan orang lain untuk mengikuti. Pekerjaan yang Anda lakukan sebagai champion memiliki efek yang tidak sebanding: setiap contoh yang Anda bagikan mempersingkat kurva pembelajaran bagi insinyur yang datang setelah Anda, dan setiap pertanyaan yang Anda jawab secara publik mengubah pengalaman satu orang menjadi sesuatu yang dapat dibangun oleh seluruh tim. Anda bertindak sebagai pengganda untuk tim Anda, bukan help desk, dan panduan ini dirancang untuk menjaga peran tetap berkelanjutan berdasarkan istilah-istilah tersebut.

<h2 id="the-champion-role">
  Peran champion
</h2>

Peran ini terdiri dari tiga perilaku yang saling memperkuat.

| Perilaku                         | Seperti apa dalam praktik                                                                                                                                                                  | Mengapa hal ini penting                                                                                                                                                                                                       |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bagikan apa yang Anda temukan    | Posting prompt, screenshot, dan kemenangan kecil dari pekerjaan Anda sendiri di tempat yang sudah dibaca tim Anda, seperti saluran teknik, thread standup, atau deskripsi pull-request.    | Contoh yang diambil dari codebase Anda sendiri lebih persuasif daripada dokumentasi eksternal apa pun, karena rekan kerja dapat melihat dengan tepat bagaimana alat ini berlaku pada masalah yang mereka bagikan dengan Anda. |
| Jadilah orang yang ditanya orang | Ketika rekan kerja bertanya bagaimana Anda mencapai sesuatu, berikan prompt sebenarnya yang Anda gunakan sehingga mereka dapat menerapkannya langsung ke tugas mereka sendiri.             | Contoh konkret yang dapat dijalankan menghilangkan kesenjangan antara rasa ingin tahu dan penggunaan pertama yang berhasil, yang merupakan tempat sebagian besar upaya adopsi terhenti.                                       |
| Perluas lingkaran                | Tetapkan sejumlah kecil kebiasaan berulang yang ringan, seperti saluran khusus atau thread mingguan, sehingga momentum terus berlanjut bahkan ketika perhatian Anda berada di tempat lain. | Adopsi yang bergantung pada satu orang rapuh. Adopsi yang dibawa oleh kebiasaan bersama terus berkembang dengan sendirinya.                                                                                                   |

Sebagian besar dari ini cocok secara alami dalam pekerjaan yang sudah Anda lakukan. Perbedaannya adalah sejumlah kecil niat tambahan tentang di mana penemuan Anda diposting dan bagaimana jawaban Anda menyebar.

<h3 id="what-this-should-cost-you">
  Apa yang seharusnya ini biayai Anda
</h3>

Tetapkan ekspektasi dengan diri sendiri dan dengan pemimpin Anda. Aktivitas di bawah ini dimaksudkan untuk cocok dalam minggu kerja normal, dan peran harus tetap menjadi pengganda pada pekerjaan Anda yang ada daripada tanggung jawab dukungan tambahan.

| Aktivitas                                | Waktu per minggu  | Panduan                                                                                                                                  |
| ---------------------------------------- | ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Posting kemenangan dan prompt            | Sekitar 15 menit  | Tangkap ini pada saat itu dengan screenshot dan satu atau dua kalimat; hindari mengubahnya menjadi write-up formal.                      |
| Menjawab pertanyaan di saluran bersama   | Sekitar 20 menit  | Jawab secara publik sekali, kemudian tautkan kembali ke jawaban itu ketika pertanyaan terulang.                                          |
| Mengadakan thread show-and-tell mingguan | Sekitar 5 menit   | Anda memposting prompt pembukaan; tim menyediakan konten.                                                                                |
| Pairing atau walkthrough opsional        | 0 hingga 30 menit | Cadangkan ini untuk rekan kerja yang benar-benar terhalang, dan tawarkan tautan [Quickstart](/docs/id/quickstart) sebelum menjadwalkan waktu. |

<h2 id="share-what-you-discover">
  Bagikan apa yang Anda temukan
</h2>

Pengalaman Anda sendiri adalah materi paling persuasif yang akan dihadapi rekan kerja Anda, karena spesifik untuk codebase, alur kerja, dan masalah yang Anda semua bagikan. Dokumentasi memberi tahu orang apa yang mungkin; posting Anda menunjukkan kepada mereka apa yang benar-benar berfungsi di lingkungan Anda.

<h3 id="what-is-worth-sharing">
  Apa yang layak dibagikan
</h3>

Post yang paling berguna menggambarkan teknik yang dapat digunakan kembali oleh rekan kerja besok daripada hasil yang sudah selesai. Teknik berkembang saat menyebar melalui tim; pembaruan status tidak.

Contoh teknik yang dapat digunakan kembali:

* "Saya belajar bahwa @-mentioning direktori berfungsi. Mengarahkannya ke `@src/components/` dan bertanya komponen mana yang kehilangan tes mengungkapkan dua yang saya lewatkan."
* "Plan mode (`Shift+Tab`) menunjukkan dengan tepat file mana yang akan disentuh sebelum ada edit yang dibuat, itulah mengapa saya nyaman menggunakannya pada kode bersama."
* "Saya mengonfigurasi hook Stop sehingga saya menerima notifikasi desktop ketika tugas panjang selesai. Konfigurasi ada di thread."
* "Menjalankan `/init` menghasilkan `CLAUDE.md` dari repositori sehingga asisten berhenti menanyakan kembali tentang konvensi kami."

<h3 id="where-to-share-it">
  Di mana harus membagikannya
</h3>

Posting di mana pun tim Anda sudah membaca. Tujuannya adalah menempatkan contoh di jalur pekerjaan normal daripada membuat tujuan.

| Lokasi                                   | Paling cocok untuk                                                           | Format yang direkomendasikan                                                                         |
| ---------------------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Saluran `#claude-code` atau teknik umum  | Penemuan, prompt, dan momen "hari ini saya belajar"                          | Screenshot disertai satu atau dua kalimat konteks                                                    |
| Deskripsi pull-request                   | Mendemonstrasikan pendekatan pada kode nyata yang sudah dibaca oleh reviewer | Satu baris seperti "Claude dan saya melakukan refactor ini; senang untuk menjelaskan pendekatannya." |
| Standup atau pembaruan tertulis mingguan | Menormalkan penggunaan dengan pemimpin dan manajer skip-level                | Satu kalimat menggambarkan satu hasil konkret                                                        |
| Wiki tim atau dokumentasi internal       | Pola tahan lama, skill khusus, dan contoh `CLAUDE.md`                        | Halaman pendek, ditautkan dari topik saluran sehingga tetap dapat ditemukan                          |

<h3 id="the-format-that-works">
  Format yang berfungsi
</h3>

Screenshot disertai satu baris konteks, atau deskripsi before-and-after singkat, umumnya tingkat detail yang tepat. Jaga setiap post cukup pendek sehingga seseorang yang melewati masih menyerap poinnya. Write-up panjang cenderung disimpan untuk nanti dan dilupakan, sedangkan post pendek dengan screenshot cenderung disalin dan dicoba.

Post contoh di bawah ini mengilustrasikan nada dan panjang; sesuaikan daripada menyalin verbatim.

```text theme={null}
Belajar hari ini bahwa @-mentioning direktori berfungsi. Saya mengarahkannya ke
@src/components/ dan bertanya komponen mana yang kehilangan tes, dan itu
mengungkapkan dua yang saya lupakan.
```

```text theme={null}
Saya mengonfigurasi hook Stop sehingga saya menerima notifikasi desktop ketika
tugas panjang selesai. Saya memulai refactor, pergi, dan diberitahu ketika
selesai. Konfigurasi ada di thread.
```

```text theme={null}
Plan mode adalah alasan saya nyaman menggunakan ini pada kode yang penting.
Tekan Shift+Tab sampai Anda melihat "plan"; itu meletakkan dengan tepat file mana
yang dimaksudkan untuk disentuh sebelum mengubah apa pun.
```

<h2 id="be-the-person-people-ask">
  Jadilah orang yang ditanya orang
</h2>

Setelah Anda membagikan beberapa contoh, pertanyaan akan mengikuti. Di sinilah peran champion memiliki leverage terbesar, karena jawaban yang baik untuk satu orang sering kali membuka blokir beberapa orang lain yang menonton saluran yang sama.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Jawab dengan prompt daripada penjelasan
</h3>

Ketika rekan kerja bertanya bagaimana Anda mencapai sesuatu, respons paling berguna adalah prompt yang benar-benar Anda gunakan. Mereka akan belajar lebih banyak dari menjalankan prompt itu terhadap masalah mereka sendiri daripada dari deskripsi apa pun yang bisa Anda tulis, dan itu memberi mereka sesuatu yang bisa mereka tindaklanjuti segera.

```text theme={null}
Rekan kerja: Bagaimana Anda mendapatkannya untuk menemukan kondisi race itu?

Champion: Saya bertanya, "Test di @tests/scheduler.test.ts tidak stabil, cari tahu
mengapa," dan itu melacak dua promise yang tidak bergabung di scheduler. Coba
frasa yang sama pada test Anda.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Tunjuk ke fitur daripada dokumentasi
</h3>

Respons seperti "Coba plan mode, tekan `Shift+Tab` sampai Anda melihatnya" lebih berguna pada saat itu daripada tautan ke dokumentasi. Jika orang membutuhkan kedalaman lebih nanti mereka akan menemukannya sendiri; sekarang mereka membutuhkan satu hal yang membuka blokir mereka.

<h3 id="questions-you-are-likely-to-hear">
  Pertanyaan yang mungkin akan Anda dengar
</h3>

| Pertanyaan                                        | Respons yang disarankan                                                                                                                                                                                                                       | Sumber daya tindak lanjut                               |
| ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| "Apa yang harus saya coba pertama kali?"          | Rekomendasikan tugas nyata tetapi terbatas, idealnya bug atau chore yang telah ditunda orang karena membosankan daripada sulit.                                                                                                               | [Common workflows](/docs/id/common-workflows)                |
| "Bagaimana saya mempercayainya dengan kode saya?" | Perkenalkan plan mode: menekan `Shift+Tab` bersiklus ke dalamnya, Claude mengusulkan dengan tepat apa yang dimaksudkan untuk diubah, dan tidak ada yang dimodifikasi sampai pengguna menyetujui.                                              | [Permissions](/docs/id/permissions)                          |
| "Apakah setup layak untuk usaha?"                 | Instalasi membutuhkan waktu kira-kira dua menit, berjalan di terminal, dan tidak memerlukan ekstensi IDE. Menjalankan `/init` sekali sudah cukup untuk mulai bekerja.                                                                         | [Quickstart](/docs/id/quickstart)                            |
| "Itu menghasilkan hasil yang salah."              | Dorong mereka untuk memberikan kegagalan kembali ke Claude. Menempel pesan kesalahan atau test yang gagal jauh lebih efektif daripada mengganti frasa permintaan asli.                                                                        | [Common workflows](/docs/id/common-workflows)                |
| "Itu tidak memahami konvensi codebase kami."      | Sarankan menjalankan `/init` untuk menghasilkan file `CLAUDE.md`, kemudian tambahkan konvensi tim, perintah test, dan direktori apa pun yang harus dihindari.                                                                                 | [Memory](/docs/id/memory)                                    |
| "Apakah ini hanya autocomplete?"                  | Tawarkan demonstrasi singkat di mana Claude menjelaskan file yang tidak dikenal, melacak bug di seluruh layanan, atau menyusun rencana migrasi. Tugas-tugas ini memerlukan penalaran di seluruh repositori daripada menyelesaikan satu baris. | Demonstrasi langsung dua menit                          |
| "Bagaimana dengan keamanan dan penanganan data?"  | Rujuk pertanyaan ini ke administrator Anda. Kebijakan penerapan dan penanganan data organisasi Anda sudah dikonfigurasi, dan champion tidak boleh mengimprovasi jawaban ini.                                                                  | [Security](/docs/id/security) · [Data usage](/docs/id/data-usage) |

<h2 id="grow-the-circle">
  Perluas lingkaran
</h2>

Tujuannya bukan untuk membangun program atau memiliki peluncuran. Ini adalah untuk membangun sejumlah kecil kebiasaan ringan yang memungkinkan momentum terus berlanjut setelah Anda berhenti secara aktif mendorong. Ketika pertanyaan di saluran dijawab oleh orang selain Anda, peran telah melakukan tugasnya.

<h3 id="patterns-that-tend-to-work">
  Pola yang cenderung berfungsi
</h3>

| Pola                                                | Cara menjalankannya                                                                                                                                                                                                                                                                              | Usaha yang diperlukan                             |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------- |
| Saluran khusus                                      | Buat saluran `#claude-code` (atau thread berulang di saluran yang ada), pin tautan [Quickstart](/docs/id/quickstart) dan satu contoh kuat, dan jawab pertanyaan secara publik sehingga setiap jawaban menguntungkan semua orang yang menonton.                                                        | Sekitar lima menit untuk diatur, kemudian ambient |
| Thread show-and-tell mingguan                       | Setiap Jumat, posting "Apa yang membantu Claude Anda minggu ini?" Tidak ada persiapan, slide, atau pertemuan yang diperlukan; screenshot dan deskripsi pendek sudah cukup.                                                                                                                       | Sekitar dua menit per minggu                      |
| Bagikan skill khusus                                | Posting file `.claude/skills/<name>/SKILL.md` paling berguna Anda, misalnya skill `/ship` yang menjalankan test dan lint sebelum commit, dengan deskripsi satu baris. Karena skill adalah Markdown biasa, rekan kerja dapat mengadopsinya segera.                                                | Sekitar lima menit per skill                      |
| Hasilkan panduan setup dari penggunaan Anda sendiri | Jalankan `/team-onboarding` dalam proyek yang telah Anda habiskan waktu nyata. Claude memindai sesi, perintah, dan server MCP terbaru Anda, kemudian menghasilkan panduan yang dapat ditempel oleh rekan kerja baru sebagai pesan pertama mereka untuk memutar ulang setup Anda. Pin di saluran. | Sekitar dua menit                                 |
| Pair pada tugas pertama                             | Tawarkan sesi pairing lima belas menit tunggal kepada siapa pun yang memulai. Satu hasil yang berhasil pada kode mereka sendiri lebih persuasif daripada presentasi apa pun.                                                                                                                     | Sekitar lima belas menit per orang                |
| Identifikasi champion berikutnya                    | Rekan kerja yang paling banyak bertanya kepada Anda biasanya siap mengambil peran ini. Teruskan halaman ini kepada mereka dan bagi tanggung jawab saluran antara Anda.                                                                                                                           | Dapat diabaikan                                   |

<h3 id="thirty-day-playbook">
  Playbook tiga puluh hari
</h3>

Jika rencana longgar membantu, urutan di bawah ini mencerminkan apa yang cenderung berfungsi di sebagian besar tim. Sesuaikan dengan bebas agar sesuai dengan konteks Anda.

<Steps>
  <Step title="Minggu 1: Semai saluran">
    Buat saluran, pin [Quickstart](/docs/id/quickstart), dan posting dua atau tiga contoh Anda sendiri dengan prompt yang disertakan.

    **Sinyal bahwa itu berfungsi:** beberapa rekan kerja bereaksi atau membalas, dan setidaknya satu pertanyaan diajukan di saluran.
  </Step>

  <Step title="Minggu 2: Mulai ritme">
    Mulai thread show-and-tell mingguan, jawab setiap pertanyaan secara publik, dan bagikan satu skill khusus atau snippet `CLAUDE.md`.

    **Sinyal bahwa itu berfungsi:** seseorang selain Anda memposting contoh mereka sendiri.
  </Step>

  <Step title="Minggu 3: Pair dan konsolidasikan">
    Tawarkan dua atau tiga sesi pairing pendek dan konsolidasikan pertanyaan dan jawaban paling umum ke dalam pesan FAQ yang disematkan.

    **Sinyal bahwa itu berfungsi:** Anda melihat penggunaan berulang, dengan rekan kerja yang sama kembali daripada mencoba sekali dan berhenti.
  </Step>

  <Step title="Minggu 4: Serahkan">
    Identifikasi champion kedua dan bagikan ringkasan singkat tentang apa yang berfungsi dan apa yang tidak dengan pemimpin atau administrator Anda.

    **Sinyal bahwa itu berfungsi:** pertanyaan di saluran dijawab oleh orang selain Anda.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Ketika seseorang ingin menggali lebih dalam
</h3>

Anda adalah pengenalan hangat daripada program onboarding. Ketika rekan kerja bergerak melampaui "haruskah saya mencoba ini" menjadi "bagaimana saya menjadi efektif dengannya," arahkan mereka ke halaman [Quickstart](/docs/id/quickstart) dan [Common workflows](/docs/id/common-workflows). Mereka berisi bagian pendek yang mencakup fitur yang benar-benar berguna tetapi sulit ditemukan sendiri.

<h2 id="respond-to-common-concerns">
  Merespons kekhawatiran umum
</h2>

Skeptisisme yang sehat diharapkan; insinyur harus berhati-hati tentang alat yang menyentuh kode mereka. Respons paling efektif jarang untuk membantah kasus umum. Sebaliknya, akui kekhawatiran, tawarkan reframe singkat, dan usulkan satu demonstrasi konkret pada kode orang itu sendiri. Sebagian besar kekhawatiran diselesaikan oleh satu pengalaman yang berhasil.

| Kekhawatiran                                               | Respons yang disarankan                                                                                                                                                                                                                     | Bukti untuk ditawarkan                                             |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| "Saya lebih cepat tanpanya."                               | Itu mungkin benar untuk kode yang biasanya ditulis orang. Sarankan mencobanya pada pekerjaan yang mereka cenderung hindari: file legacy, layanan yang tidak dikenal, atau test scaffolding, di mana leverage tertinggi.                     | Waktu satu tugas membosankan kedua cara dan bandingkan.            |
| "Saya tidak mempercayai AI untuk menyentuh kode produksi." | Setuju bahwa tidak ada perubahan yang harus mendarat tanpa dibaca. Plan mode dikombinasikan dengan review diff normal berarti tidak ada yang diterapkan yang belum diperiksa oleh insinyur, standar yang sama seperti pull request apa pun. | Demonstrasikan plan mode pada file nyata.                          |
| "Itu akan membuat insinyur junior lebih lemah."            | Digunakan dengan baik, itu adalah penjelasan yang efektif. Dorong insinyur junior untuk meminta Claude menjelaskan file dan situs panggilannya sebelum meminta untuk mengubah apa pun.                                                      | Jalankan "Jelaskan @file dan di mana itu dipanggil dari" bersama.  |
| "Saya mencobanya sekali dan itu mengalami halusinasi."     | Ini biasanya masalah konteks daripada masalah model. @-mentioning file yang relevan, menjalankan `/init`, dan memberikan output kesalahan aktual biasanya menyelesaikannya.                                                                 | Jalankan kembali prompt asli mereka dengan konteks `@` yang tepat. |
| "Kami tidak memiliki waktu untuk mempelajari alat lain."   | Claude Code adalah perintah terminal daripada platform. Jika itu tidak mengembalikan nilai dalam sesi pertama, masuk akal untuk menyisihkannya.                                                                                             | Instalasi dua menit diikuti oleh satu bug nyata.                   |

<h2 id="quick-reference-sheet">
  Lembar referensi cepat
</h2>

Teknik di bawah ini adalah yang paling dapat diandalkan untuk memindahkan seseorang dari uji coba pertama ke penggunaan harian. Pin tabel ini di saluran atau bagikan sendiri.

| Teknik                                  | Cara menerapkannya                                                                                                                                                                      |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Berikan konteks yang tepat              | Gunakan referensi `@file` atau `@directory/`, atau tempel output kesalahan atau log secara langsung. Menyediakan konteks yang relevan lebih efektif daripada prompting yang rumit.      |
| Tinjau rencana sebelum edit             | Tekan `Shift+Tab` untuk memasuki plan mode. Claude akan menggambarkan perubahan yang dimaksudkan untuk persetujuan Anda sebelum menjalankannya.                                         |
| Ajarkan repositori Anda                 | Jalankan `/init` untuk menghasilkan file `CLAUDE.md`, kemudian tambahkan konvensi Anda, perintah test, dan direktori apa pun yang tidak boleh dimodifikasi. Lihat [Memory](/docs/id/memory). |
| Gunakan kembali alur kerja              | Simpan file `SKILL.md` di `.claude/skills/<name>/` untuk membuat skill `/name` yang dapat digunakan oleh seluruh tim. Lihat [Skills](/docs/id/skills).                                       |
| Tetap terinformasi selama tugas panjang | Konfigurasikan hook Stop untuk menerima notifikasi desktop ketika tugas yang berjalan lama selesai. Lihat [Hooks](/docs/id/hooks-guide).                                                     |
| Pulih dari hasil yang salah             | Daripada mengganti frasa permintaan, tempel test yang gagal atau stack trace kembali ke Claude dan minta untuk mengatasi kegagalan spesifik itu.                                        |
| Jaga edit tetap bedah                   | Minta diff, atau tentukan "hanya ubah X." Claude menghormati scope ketika scope dinyatakan.                                                                                             |

<Tip>
  Claude Code diperbarui secara sering. Verifikasi detail spesifik versi terhadap [halaman beranda dokumentasi](/docs/id/overview) sebelum mendistribusikan materi ini secara internal.
</Tip>
