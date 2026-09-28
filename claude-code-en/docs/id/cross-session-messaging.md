> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pesan sesi Claude Code Anda yang lain

> Biarkan Claude mencantumkan dan mengirim pesan ke sesi Claude Code Anda yang lain di mesin ini, dan jangkau sesi Anda di mesin lain atau di web.

<Note>
  Pesan lintas sesi memerlukan Claude Code v2.1.224 atau lebih baru di macOS dan Linux, termasuk Linux di dalam WSL 2. Di Windows asli, diperlukan Claude Code v2.1.234 atau lebih baru. Ketika sesi memenuhi persyaratan, pesan aktif tanpa perlu diaktifkan. Lihat [Ketersediaan](#availability) untuk persyaratan penyedia dan cara mengonfirmasi bahwa sesi memilikinya.
</Note>

Pesan lintas sesi memungkinkan Claude mengirimkan pesan dari salah satu sesi Claude Code Anda ke sesi lainnya. Ketika perubahan dalam satu sesi merusak apa yang sedang dibangun sesi lain, Claude dapat memperingatkan sesi tersebut sebelum Anda menyadarinya. Ketika satu sesi menyelesaikan pertanyaan yang memblokir sesi lain, Claude dapat mengirimkan jawaban lintas sesi.

Pesan adalah sepotong teks yang ditulis satu Claude ke Claude lainnya, tidak pernah riwayat percakapan atau file pengirim. Untuk memindahkan seluruh percakapan atau konteksnya, [lanjutkan sesi](/docs/id/sessions#resume-a-session) sebagai gantinya.

Claude menggunakan dua alat untuk ini: `ListAgents` untuk menemukan agen mana yang dapat dijangkaunya, dan `SendMessage` untuk mengirimkan pesan ke salah satunya berdasarkan nama. Dengan alat `SendMessage` yang sama, Claude juga dapat mengirim pesan ke [subagen](/docs/id/sub-agents#resume-subagents) dan rekan tim [tim agen](/docs/id/agent-teams) dalam satu sesi atau tim. Halaman ini mencakup pesan antara sesi independen Anda.

<h2 id="when-to-use-cross-session-messaging">
  Kapan menggunakan cross-session messaging
</h2>

Gunakan messaging ketika salah satu sesi Anda memiliki sesuatu yang dibutuhkan sesi lain di tengah tugas. Claude dapat mengirimkan pesan sendiri ketika melihat kebutuhan, misalnya setelah membuat perubahan yang mempengaruhi pekerjaan sesi lain, atau Anda dapat memintanya mengirimkan satu. Kasus umum:

* **Serahkan temuan**: ketika satu sesi menemukan perubahan yang merusak atau membuat keputusan, Claude merangkumnya untuk sesi yang bekerja di area yang terpengaruh, alih-alih Anda menjelaskannya ulang di sana.
* **Koordinasikan worktrees paralel**: ketika sesi bekerja di repositori yang sama di [worktrees](/docs/id/worktrees) terpisah, Claude dapat memberi tahu sesi lain apa yang mendarat.
* **Dapatkan status dari pekerjaan yang berjalan lama**: buat migrasi atau pengujian melaporkan kembali ke sesi yang Anda tonton, atau minta sendiri dari sana. Jika sesi itu ada di mesin ini, Claude juga dapat [memintanya untuk satu pemberitahuan ketika sesi berikutnya idle atau keluar](#get-a-notice-when-another-session-goes-idle).
* **Kirim pesan lintas mesin**: jangkau salah satu sesi Anda di mesin lain atau di web.

Gunakan messaging antara sesi independen yang Anda mulai dan arahkan sendiri. Claude Code memiliki fitur khusus untuk setiap cara lain menjalankan atau menjangkau beberapa sesi, jadi gunakan yang dibangun untuk apa yang Anda lakukan:

* Untuk melanjutkan satu percakapan di terminal lain, atau berbagi konteksnya dengan sesi baru, [lanjutkan sesi](/docs/id/sessions#resume-a-session)
* Untuk tim sesi terkoordinasi yang Claude spawn dan supervisi, gunakan [agent teams](/docs/id/agent-teams)
* Untuk menonton dan mengarahkan banyak sesi dari satu tempat, gunakan [agent view](/docs/id/agent-view)
* Untuk mengarahkan sesi sendiri dari ponsel atau perangkat lain, daripada membuat sesi saling mengirim pesan, gunakan [Remote Control](/docs/id/remote-control)
* Untuk mendorong peristiwa eksternal, seperti hasil CI atau pesan chat, ke dalam sesi, gunakan [channels](/docs/id/channels)

<h2 id="message-another-session">
  Kirim pesan ke sesi lain
</h2>

Ketika salah satu sesi Anda mempelajari sesuatu yang dibutuhkan sesi lain, seperti temuan, status, atau keputusan, Claude meneruskannya alih-alih Anda menyalin-menempel antar terminal. Claude menemukan target dengan `ListAgents` dan mengirim dengan `SendMessage`, jadi Anda tidak pernah memanggil tool apa pun sendiri. Claude dapat memutuskan untuk mengirimkan pesan tanpa diminta, dan Anda juga dapat meminta satu.

Untuk meminta satu sendiri, beri tahu Claude apa yang ingin Anda ketahui atau lakukan sesi lain. Contoh ini adalah prompt yang Anda ketik, bukan pesan yang Claude kirim:

```text wrap theme={null}
Tanyakan ke sesi yang berjalan di terminal lain saya apakah migrasi selesai
```

Claude menulis pesan aktual sendiri, jadi prompt Anda dapat meninggalkan konten kepada Claude. Prompt ini meminta ringkasan tanpa mendikte kata-katanya, dan apa yang Claude kirim bervariasi:

```text wrap theme={null}
Jelaskan apa yang baru saja kami lakukan ke sesi yang bekerja pada API pembayaran
```

Untuk menamai target sendiri, sebutkan sesi dalam prompt Anda: ketik `@` diikuti oleh huruf pertama nama sesi dan pilih sesi dari typeahead, dengan cara yang sama Anda [@-mention subagent](/docs/id/sub-agents#invoke-subagents-explicitly). Memerlukan Claude Code v2.1.232 atau lebih baru. Claude Code menyisipkan penyebutan, seperti `@api-worker`, dan memberi tahu Claude sesi mana yang dinamainya, jadi Claude dapat mengirim pesan ke sesi itu tanpa membuat daftar sesi Anda terlebih dahulu. Prompt ini menamai target dengan penyebutan:

```text wrap theme={null}
Biarkan @api-worker tahu migrasi skema selesai
```

Typeahead mencantumkan sesi live lain Anda di mesin ini. Dua kasus memerlukan lebih dari huruf pertama nama:

* **Sesi di luar mesin ini**: sesi cloud atau Remote Control muncul di typeahead hanya setelah Claude telah membuat daftar atau mengirim pesan ke sesi Anda di luar mesin ini, jadi minta Claude untuk membuat daftar mereka terlebih dahulu.
* **Nama dengan spasi atau karakter lain di luar huruf, digit, tanda hubung, dan garis bawah**: ketiknya dalam tanda kutip ganda, seperti `@"release notes"`. Ketika Anda memilih sesi dari typeahead, Claude Code menyisipkan tanda kutip untuk Anda.

Anda juga dapat mengetik penyebutan tanpa pemilih. Ketika lebih dari satu sesi live menjawab nama yang disebutkan, Claude meminta Anda sesi mana yang Anda maksud sebelum mengirim.

Untuk apa pesan yang Claude tulis terlihat seperti ketika tiba, termasuk contoh satu, lihat [apa pesan terlihat seperti](#what-a-message-looks-like).

<h3 id="message-delivery">
  Pengiriman pesan
</h3>

Claude penerima membaca pesan antara pemanggilan tools selama giliran aktif, jadi tools yang berjalan tidak pernah terputus. Ketika sesi penerima idle, Claude Code memulai giliran baru dengan pesan.

Pesan dari sesi lain tiba sebagai teks biasa. Jika menyebutkan file atau [MCP resource](/docs/id/mcp#use-mcp-resources) dengan `@`, Claude melihat penyebutan seperti yang ditulis dan Claude Code tidak melampirkan apa pun, apakah pesan memulai giliran baru atau tiba selama satu. Claude masih dapat membuka jalur yang disebutkan di mesin penerima dengan tools-nya sendiri, tunduk pada izin sesi itu. Sebelum v2.1.251, penyebutan `@` dalam pesan yang memulai giliran baru melampirkan file atau MCP resource di sisi penerima.

Claude Code menolak pesan dalam kasus berikut:

* Pesan [melebihi batas ukuran](#limitations). Claude Code menolaknya di sesi pengirim, sebelum meninggalkan.
* Ledakan cepat ke sesi di mesin ini telah mencapai [apa yang diterima inbox sesi itu](#limitations). Claude Code menolak pesan lebih lanjut ke sesi itu.
* Target balasan di mesin ini gagal pemeriksaan keamanan, seperti target yang disimbolkan atau endpoint yang bukan proses yang diharapkan. [Menolak mengirim pesan cross-session](/docs/id/errors#refusing-to-send-a-cross-session-message) mencantumkan pemeriksaan ini.
* Claude mengatasi pesan ke nama sesi ini sendiri, seperti dijelaskan di bawah [Lihat sesi mana yang dapat dijangkau Claude](#see-which-sessions-claude-can-reach).

Sesi penerima memeriksa setiap pesan yang tiba terhadap [kontrol inbound](#control-inbound-messages) miliknya sendiri, dan pemeriksaan berakhir dalam salah satu dari tiga hasil:

* **Delivered**: Claude Code meneruskan pesan ke Claude penerima.
* **Held**: Claude Code menyisihkan pesan tanpa dikirim. Pesan yang ditahan mencapai Claude hanya ketika Anda menyetujuinya atau perubahan mode atau pengaturan yang lebih baru memungkinkannya.
* **Refused**: Claude Code menjatuhkan pesan tanpa mengirimkannya.

Setelah dikirim, pesan dihitung terhadap [usage](/docs/id/costs) seperti prompt yang Anda ketik, dan Claude penerima dapat membalas ke pengirim dengan cara yang sama, kecuali dalam [kasus cross-machine satu arah](#message-sessions-on-other-machines).

Batas izin tetap per-sesi. Claude diinstruksikan untuk tidak pernah meminta sesi lain untuk tindakan yang ditolak atau diblokir di sesi miliknya sendiri, atau yang akan diblokir pengaturan izin miliknya sendiri, dan untuk mengarahkan pekerjaan itu kembali ke Anda. Di sisi penerima, [prompt izin sesi penerima sendiri dan aturan masih berlaku](#how-a-session-treats-an-incoming-message) untuk apa pun yang diminta pesan.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Dapatkan pemberitahuan ketika sesi lain idle
</h3>

Claude dapat meminta salah satu sesi Anda di mesin ini untuk mengirimkan kembali satu pemberitahuan ketika sesi itu berikutnya idle atau keluar. Idle di sini berarti sesi menyelesaikan giliran tanpa apa pun dalam antrian. Gunakan ketika Anda menunggu tugas panjang di sesi lain dan ingin mendengar ketika selesai alih-alih memeriksa. Memerlukan Claude Code v2.1.236 atau lebih baru di kedua sesi.

<h4 id="ask-for-a-notice">
  Minta pemberitahuan
</h4>

Beri tahu Claude apa yang Anda tunggu. Prompt ini meminta pemberitahuan dari sesi migrasi:

```text wrap theme={null}
Beri tahu saya ketika sesi migrasi selesai dengan apa yang sedang dikerjakan
```

Claude berlangganan dengan input `notify_when_idle` tools `SendMessage`, baik dilampirkan ke pesan yang sedang dikirimnya atau sendiri. Sendiri, Claude Code berlangganan tanpa memulai giliran atau menghabiskan token di sesi yang ditonton, dan mengirimkan pemberitahuan segera jika sesi itu sudah idle. Dilampirkan ke pesan, Claude Code mengirimkan pesan terlebih dahulu dan mengirimkan pemberitahuan nanti.

<h4 id="what-each-session-shows">
  Apa yang ditunjukkan setiap sesi
</h4>

Sesi yang ditonton menunjukkan baris yang mengatakan proses lain meminta untuk diberitahu ketika sesi berikutnya idle. Sesi yang bertanya menunjukkan pemberitahuan sebagai baris yang menamai sesi yang ditonton. Baris dapat mencakup waktu giliran sesi itu selesai dan status satu baris dari giliran itu. Jika sesi yang bertanya idle, Claude Code memulai giliran baru dengan pemberitahuan.

<h4 id="limits">
  Batas
</h4>

Pemberitahuan adalah satu kali: Claude Code mengirimnya sekali dari sesi yang ditonton, dan tidak ada sesi yang menanyakan yang lain. Jika tidak ada pemberitahuan yang tiba dalam 12 jam, Claude Code menjatuhkan langganan dan memberi tahu Claude, jadi tidak terus menunggu.

[Kontrol inbound](#control-inbound-messages) setiap sisi berlaku untuk pemberitahuan seperti pesan:

* **`refuse` di kedua sisi**: tidak ada yang tiba. Sesi yang ditonton menjatuhkan permintaan tanpa merekam atau menjawabnya, jadi langganan berakhir tanpa jawaban setelah 12 jam, dan sesi yang bertanya dengan `refuse` tidak pernah berlangganan.
* **`hold` di kedua sisi**: pemberitahuan tiba dengan lebih sedikit. Sesi yang ditonton meninggalkan status satu baris, dan sesi yang bertanya menunjukkan pemberitahuan dalam transkrip Anda tanpa mengirimkannya ke Claude.

Hanya Claude dalam percakapan utama Anda yang dapat berlangganan, dan hanya ke sesi Anda di mesin ini. Ketika subagent atau rekan kerja agent team menetapkan `notify_when_idle`, Claude Code tidak membuat langganan dan memberi tahu demikian. Ketika Claude meminta pemberitahuan dari agen lain apa pun, seperti rekan kerja, subagent, atau sesi di luar mesin ini, Claude Code menolak seluruh panggilan, termasuk pesan apa pun yang dilampirkan padanya, dan melaporkan penolakan ke Claude sehingga dapat mengirim ulang pesan tanpa permintaan.

<h3 id="see-which-sessions-claude-can-reach">
  Lihat sesi mana yang dapat dijangkau Claude
</h3>

Claude menemukan target pesan sendiri, jadi Anda tidak perlu menjalankan apa pun sebelum memintanya mengirim. Untuk melihat sendiri sesi mana yang dapat dijangkau Claude, jalankan perintah `/list-agents`. Baris pertama, ketika ada, adalah nama sesi ini sendiri, yang digunakan sesi lain Anda untuk mengirim pesan padanya. Baris di bawahnya adalah sesi yang dapat dijangkau Claude:

* **Subagents**: agen yang berjalan di dalam sesi saat ini.
* **Teammates**: rekan kerja [agent team](/docs/id/agent-teams) sesi ini sendiri. Sebelum v2.1.239, rekan kerja tidak muncul dalam daftar, meskipun Claude sudah dapat mengirim pesan kepada mereka berdasarkan nama.
* **Sesi lokal Anda yang lain**: sesi Claude Code yang berjalan di mesin yang sama, termasuk [background sessions](/docs/id/agent-view). Sesi muncul hanya ketika mengikat [inbox socket](#the-sessions-inbox-socket).
* **Sesi cloud Anda**: sesi [Claude Code on the web](/docs/id/claude-code-on-the-web) Anda, ditampilkan saat sesi ini terhubung ke [Remote Control](/docs/id/remote-control). Claude Code memberi label `cloud` pada mereka dalam daftar.
* **Sesi Remote Control Anda di mesin lain**: ditampilkan saat sesi ini terhubung ke [Remote Control](/docs/id/remote-control), dan diberi label `Remote Control`. Claude Code menunjukkan `offline` sebagai status sesi yang koneksi Remote Control-nya telah putus.

Sesi ini bukan salah satu baris. Jika Claude mengatasi pesan ke nama sesi ini sendiri, Claude Code menolaknya dan memberi tahu Claude target adalah sesi saat ini. Sebelum v2.1.239, daftar tidak menunjukkan nama sesi ini, dan Claude Code melaporkan pesan yang dikirim padanya sebagai agen yang tidak dapat ditemukannya.

Saat sesi ini terhubung ke [Remote Control](/docs/id/remote-control), Claude Code menahan beberapa detail sesi lokal Anda dari output `/list-agents`, tanpa mengubah apa yang Claude sendiri lihat ketika mencari sesi untuk mengirim pesan:

* **Direktori kerja**: meninggalkan direktori kerja setiap sesi lokal.
* **Nama sesi**: meninggalkan nama sesi apa pun yang tidak dapat diatribusikan ke orang, jadi baris yang tersisa tanpa nama berbunyi `(unnamed session)`.
* **Baris pertama**: meninggalkan baris dengan nama sesi ini sendiri kecuali Anda mengetik nama itu di terminal ini, dengan `--name` atau dengan `/rename` dan nama, sejak Anda meluncurkan atau terakhir melanjutkan sesi.

Ketika output mencantumkan apa pun, itu berakhir dengan catatan yang mengatakan detail ditahan. Menjalankan `/rename` diikuti dengan nama yang tidak digunakan di keyboard sesi sendiri memberikan sesi itu nama yang muncul dalam output.

Claude Code membaca daftar sesi cloud dan Remote Control Anda terbaru terlebih dahulu dan berhenti setelah jumlah halaman terbatas untuk masing-masing. Jika akun Anda memiliki lebih banyak sesi itu daripada yang cocok, Claude Code tidak membuat daftar yang lebih lama, dan Claude tidak dapat mengirim pesan kepada mereka berdasarkan nama. Ketika ini terjadi, Claude Code mengatakan demikian dalam daftar, dan Claude melihat catatan yang sama ketika mengirim pesan.

Claude mengatasi sesi di luar mesin ini berdasarkan nama, sama seperti sesi lokal. Lihat [Message sessions on other machines](#message-sessions-on-other-machines) untuk bagaimana pesan itu bepergian.

Sesi menjawab nama yang Anda tetapkan dengan perintah [`/rename`](/docs/id/commands) atau bendera [`--name`](/docs/id/cli-reference#cli-flags). Ketika Anda tidak menetapkan satu, Claude Code memberi nama sesi itu sendiri. Untuk sesi interaktif, itu adalah nama yang ditampilkan dalam [daftar sesi yang berjalan](/docs/id/sessions#name-your-sessions).

Ketika Anda mengganti nama sesi, Claude Code juga memperbarui catatan bersama yang digunakan sesi lain Anda untuk mencari nama sesi. Jika tidak dapat memperbarui catatan itu, itu memperingatkan Anda dalam output `/rename` bahwa sesi lain mungkin masih menunjukkan nama lama. Jalankan sesi dengan [`--debug`](/docs/id/cli-reference#cli-flags), dan Claude Code mencatat penyebab pembaruan yang gagal.

Ketika Anda mengganti nama sesi, atau memulai atau melanjutkan sesi interaktif, dengan nama yang sudah digunakan sesi live lain di mesin ini, Claude Code meninggalkan nama dengan sesi yang sudah memilikinya dan [mengganti nama Anda menjadi varian](/docs/id/sessions#name-your-sessions). Sesi masih dapat berbagi nama, misalnya ketika salah satunya menjalankan versi Claude Code yang lebih awal atau nama bersama adalah yang dihasilkan Claude Code. Kecuali sesi ini terhubung ke Remote Control, Claude Code menunjukkan direktori kerja setiap sesi lokal dalam output `/list-agents`, jadi Anda dapat membedakan sesi dengan nama yang sama ketika berjalan di direktori berbeda. Claude mengatasi pesan dalam salah satu dari dua cara, tergantung pada berapa banyak sesi live yang menjawab nama:

* **Satu sesi menjawab nama**: Claude Code mengirimkan pesan hanya pada nama.
* **Beberapa sesi berbagi nama, atau Claude Code tidak dapat memeriksa di mana pun sesi Anda berjalan**: Claude menambahkan pengidentifikasi pendek ke setiap baris daftarnya dan menggunakan pengidentifikasi dalam alamat.

<h3 id="message-sessions-on-other-machines">
  Kirim pesan ke sesi di mesin lain
</h3>

Bagaimana pesan bepergian, dan apakah melewati server Anthropic, tergantung di mana sesi target berjalan:

| Di mana sesi lain berjalan                              | Bagaimana pesan bepergian                                                                                                      |
| :------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------- |
| Di mesin ini                                            | Melalui soket per-sesi di macOS dan Linux, atau pipa bernama per-sesi di Windows native, tidak pernah melalui server Anthropic |
| Di mesin lain Anda                                      | Melalui server Anthropic, tiba melalui koneksi [Remote Control](/docs/id/remote-control) mesin itu                                  |
| Di [Claude Code on the web](/docs/id/claude-code-on-the-web) | Melalui server Anthropic, langsung ke sesi cloud                                                                               |

Memulai percakapan dengan sesi di mesin lain Anda memerlukan Claude Code v2.1.225 atau lebih baru dan target yang [muncul dalam daftar](#see-which-sessions-claude-can-reach). Sebelum v2.1.225, Claude hanya dapat membalas pesan yang tiba dari satu.

Anda dapat mengirim pesan ke sesi yang ditampilkan sebagai `offline` dalam [daftar](#see-which-sessions-claude-can-reach), sesi yang koneksi Remote Control-nya telah putus. Pengiriman melewati, tetapi pesan tiba hanya setelah mesin sesi itu terhubung kembali. Claude diberitahu demikian ketika mengirim.

Pengiriman mesin yang sama bekerja di mana pun fitur diaktifkan. Setiap sesi mendaftarkan dirinya dalam file di disk. Ketika Claude membuat daftar atau mengirim pesan ke sesi lokal Anda, Claude Code membaca file itu untuk menemukan sesi, jadi dua sesi dapat menjangkau satu sama lain hanya ketika dapat melihat file yang sama.

Kontainer memiliki sistem file sendiri, jadi sesi di dalamnya dan sesi di host tidak dapat menjangkau satu sama lain. Dua sesi di dalam kontainer yang sama masih dapat mengirim pesan satu sama lain, termasuk di [self-hosted runner](/docs/id/self-hosted-environments). Sesi di dalam WSL 2 dan sesi Windows native di komputer yang sama juga tidak dapat menjangkau satu sama lain, karena mendaftarkan di bawah direktori home berbeda dan mendengarkan pada jenis soket berbeda.

Saat sesi ini terhubung ke Remote Control, ketika Anda mengirim pesan ke sesi di mesin lain Anda, Claude Code menunjukkan pesan dalam percakapan sesi itu di bawah nama Remote Control sesi ini. Claude di mesin itu dapat membalas nama itu. Misalnya, ketika sesi ini terhubung ke Remote Control sebagai `laptop-graceful-unicorn` dan Anda mengirim pesan ke desktop Anda, Anda melihat pesan di sesi desktop di bawah `laptop-graceful-unicorn`.

Jika sesi ini tidak terhubung ke Remote Control ketika Claude mengirim ke sesi di luar mesin ini, pesan masih melewati, tetapi tanpa [alamat balasan](#what-a-message-looks-like), jadi Claude penerima tidak dapat menjawabnya. Claude diberitahu demikian ketika mengirim.

Untuk memerlukan persetujuan Anda sebelum pesan apa pun melampaui mesin ini, atur [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Bagaimana sesi memperlakukan pesan yang masuk
</h2>

Ketika sesi A mengirim pesan ke sesi B, Claude Code memberi tahu Claude B bahwa pesan berasal dari sesi lain, bukan dari Anda, dan membatasi apa yang dapat dilakukan pesan:

* **Tidak dapat menyetujui apa pun**: pesan dari sesi lain tidak pernah dihitung sebagai persetujuan Anda, jadi tidak dapat menjawab prompt izin yang tertunda atas nama Anda.
* **Tidak dapat mengubah konfigurasi**: Claude Code menginstruksikan Claude penerima untuk tidak pernah mengubah pengaturan izin, `CLAUDE.md`, atau konfigurasi lain karena sesi lain meminta.
* **Perintah tidak berjalan**: perintah dalam teks pesan, seperti `/compact`, tiba sebagai teks biasa. Claude Code tidak pernah menjalankannya.
* **Prompt izin masih aktif**: jika bertindak atas pesan memerlukan izin yang tidak dimiliki sesi penerima, Anda melihat prompt yang sama seperti untuk pekerjaan lain apa pun.

<h3 id="what-a-message-looks-like">
  Apa pesan terlihat seperti
</h3>

Ketika pesan tiba, Claude Code menunjukkannya dalam percakapan sebagai pratinjau satu baris yang redup, dan baris pratinjau tetap dalam percakapan sesudahnya. Pratinjau membawa nama pengirim dan baris pertama pesan, dipotong dengan `…` ketika panjang, seperti `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Sebelum v2.1.247, Claude Code menunjukkan pesan yang tiba secara penuh alih-alih pratinjau.

Salah satu dari ini menunjukkan Anda teks lengkap:

* Tekan `Ctrl+O` untuk membuka [transcript viewer](/docs/id/interactive-mode#transcript-viewer) dan baca teks lengkap di bawah nama sesi pengirim.
* Dalam sesi yang dimulai dengan [`--verbose`](/docs/id/cli-reference#cli-flags), Claude Code menunjukkan teks lengkap alih-alih pratinjau.

Pratinjau mempersingkat hanya apa yang Anda lihat. Apakah Anda memperluas atau tidak, Claude membaca pesan lengkap.

Claude menerima pesan dengan nama pengirim dan alamat balasan, kecuali untuk [pesan cross-machine satu arah](#message-sessions-on-other-machines), yang tidak membawa alamat balasan. Di luar nama dan alamat balasan, Claude penerima mendapatkan teks pesan, tidak pernah riwayat percakapan atau file pengirim. [Message delivery](#message-delivery) mencakup penyebutan `@` dalam teks.

Pesan yang ditulis [subagent](/docs/id/sub-agents) tiba di bawah nama sesi pengirim, dengan subagent diidentifikasi dalam teks pesan. Balasan padanya mencapai percakapan utama sesi itu, bukan subagent.

Contoh ini adalah pesan yang ditulis satu Claude ke Claude lain, seperti teks lengkapnya dibaca ketika Anda memperluas:

```text wrap theme={null}
Migrasi skema selesai
Kolom baru adalah tenant_id, dan rebase pada main aman sekarang.
```

<h3 id="control-inbound-messages">
  Kontrol pesan yang masuk
</h3>

Atur [`crossSessionInbound`](/docs/id/settings-reference#crosssessioninbound) untuk memilih apa yang dilakukan sesi dengan pesan yang tiba dari sesi lain Anda:

| Nilai    | Perilaku                                                                                                                                                                                                                      |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code mengirimkan setiap pesan ke Claude                                                                                                                                                                                |
| `hold`   | Claude Code menunjukkan pemberitahuan untuk setiap pesan dan tidak mengirimkannya. Jika `accept` kemudian berlaku, per [aturan prioritas](/docs/id/settings-reference#crosssessioninbound), Claude Code merilis pesan yang ditahan |
| `refuse` | Claude Code menjatuhkan setiap pesan tanpa mengirimkannya                                                                                                                                                                     |

Di luar mengedit file pengaturan, Anda dapat memilih nilai dalam baris `/config` **Messages from your other sessions**. Claude Code menulis nilai yang Anda pilih ke pengaturan pengguna Anda. Baris memerlukan Claude Code v2.1.232 atau lebih baru dan tidak muncul saat pengaturan terkelola atau bendera `--settings` menetapkan kunci, karena nilai pengaturan pengguna tidak akan berlaku kemudian. Claude Code menolak shorthand `/config crossSessionInbound=value` untuk kunci ini.

Untuk melihat nilai mana yang berlaku, ikuti aturan prioritas `crossSessionInbound` dalam [referensi pengaturan](/docs/id/settings-reference#crosssessioninbound).

Ketika tidak ada nilai yang berlaku, Claude Code memutuskan per pesan dari mode izin dua sesi. Ini mengelompokkan sesi yang [melewati prompt izin](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) ke dalam satu kelas, dan setiap sesi lain ke kelas lain. Plan mode dihitung sebagai melewati dalam sesi terminal interaktif dengan izin bypass tersedia, dan [auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits`, dan `dontAsk` dihitung sebagai prompting:

* **Sesi penerima meminta izin**: Claude Code mengirimkan setiap pesan. Ini menahan satu untuk persetujuan Anda hanya ketika sesi pengirim mengidentifikasi dirinya sebagai melewati prompt izin.
* **Sesi penerima melewati prompt izin**: Claude Code menahan setiap pesan untuk persetujuan Anda. Ini mengirimkan satu hanya ketika sesi pengirim juga mengidentifikasi dirinya sebagai melewati.

Ketika default menahan pesan, Claude Code membuka dialog persetujuan di sesi penerima. Dialog menunjukkan pengirim dan pratinjau:

* **Approve** mengirimkan pesan itu ke Claude.
* **Deny**, atau menutup dialog, menjatuhkannya.
* Ketika dialog tetap tanpa jawaban melewati batas waktu [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry), Claude Code menutupnya dan menjatuhkan pesan. Batas waktu default ke lima menit.
* Saat tidak ada terminal yang terpasang ke [background session](/docs/id/agent-view), Claude Code meninggalkan dialog terbuka melewati batas waktu. Setelah Anda melampirkan, jika dialog tetap tanpa jawaban untuk periode batas waktu penuh, Claude Code menutupnya dan menjatuhkan pesan.
* Jika kelas mode izin sesi ini berubah saat pesan ditahan, Claude Code menerapkan kembali aturan inbound, mengirimkan pesan yang sekarang diterima, dan menunjukkan pemberitahuan.
* Jika perubahan pengaturan membuat `refuse` berlaku saat pesan ditahan, Claude Code menjatuhkan setiap pesan yang ditahan dan melaporkan penolakan ke setiap pengirim yang dapat dijangkaunya.

Ketika pengirim adalah sesi di mesin yang sama, Claude Code mengirimkan pemberitahuan kembali ke sana ketika penerima menahan pesan, dan tindak lanjut ketika penerima kemudian mengirimkan, menolak, atau kedaluwarsa. Pemberitahuan mencapai Claude pengirim, jadi tahu tidak perlu terus menunggu pesan yang sesi lain belum baca.

Dalam sesi pengirim interaktif, pemberitahuan muncul dalam transkrip. Pengirim [`claude -p`](/docs/id/headless) menerima dalam [streamed output](/docs/id/headless#stream-responses) sebagai [informational `system` message](/docs/id/agent-sdk/typescript#sdkinformationalmessage). Pemberitahuan ke pengirim `claude -p` memerlukan Claude Code v2.1.271 atau lebih baru.

Jika penerima menolak pesan, pemberitahuan pengirim mengatakan penerima tidak menerima pesan cross-session dan memberi tahu Claude pengirim untuk tidak menunggu atau mengirim ulang.

Claude Code menahan paling banyak 100 pesan, terpisah dari antrian pengiriman, dan melampaui itu menjatuhkan yang tertua.

<h3 id="non-interactive-sessions">
  Sesi non-interaktif
</h3>

Claude Code mengikat soket inbox untuk sesi [`claude -p`](/docs/id/headless) seperti sesi interaktif, jadi pekerja `-p` yang berjalan lama dapat menerima pesan dan muncul dalam daftar. Ketika Anda memulai sesi dalam [bare mode](/docs/id/headless#start-faster-with-bare-mode), Claude Code tidak mengikat soket, jadi sesi itu tidak dapat menerima pesan dan tidak muncul dalam daftar agen.

Sesi `-p` tidak dapat menunjukkan dialog persetujuan. Ketika [default inbound](#control-inbound-messages) menahan pesan di sana, Claude Code menyimpannya untuk batas waktu [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry) yang sama yang digunakan dialog, lima menit secara default:

* **Sebelum batas waktu**: jika mode atau perubahan pengaturan memungkinkan pesan, Claude Code mengirimkannya.
* **Melewati batas waktu**: Claude Code menjatuhkan pesan dan melaporkannya sebagai kedaluwarsa ke pengirim yang dapat dijangkaunya.

Atur `dialogExpiry` ke `"never"` untuk menyimpan pesan default-held sampai sesi berakhir. Pesan yang ditahan oleh pengaturan `hold` eksplisit tidak kedaluwarsa; Claude Code mengirimkannya hanya ketika `accept` kemudian berlaku.

Ketika sesi berakhir dengan pesan masih ditahan, Claude Code melaporkannya sebagai kedaluwarsa ke setiap pengirim yang dapat dijangkaunya. Sebelum v2.1.225, tidak ada batas waktu yang berlaku dalam sesi `-p`: pesan yang ditahan tetap ditahan kecuali perubahan mode izin selama jalankan mengirimkannya, dan sesi yang berakhir dengan pesan yang ditahan melaporkan tidak ada kepada pengirim mereka.

Untuk membiarkan pekerja `-p` mengambil pesan tanpa pengawasan, mulai dengan `crossSessionInbound` diatur ke `accept` dalam nilai `--settings` miliknya. `accept` dalam pengaturan pengguna Anda juga bekerja tetapi berlaku untuk setiap sesi yang Anda jalankan.

<h3 id="the-sessions-inbox-socket">
  Soket inbox sesi
</h3>

Baca bagian ini ketika sesi yang Anda harapkan tidak ada dalam daftar agen, ketika Anda ingin script atau hook untuk memposting ke dalam sesi, atau ketika perintah sandboxed tidak dapat menjangkau soket.

Claude Code mengikat soket inbox untuk setiap sesi dengan cross-session messaging diaktifkan, di mana sesi lain di mesin mengirimkan pesan. Soket adalah soket domain Unix di macOS dan Linux, termasuk Linux di dalam WSL 2, dan pipa bernama di Windows native. Untuk jenis sesi mana yang mengikat satu, lihat [Non-interactive sessions](#non-interactive-sessions).

Anda dapat menemukan jalur soket di dua tempat:

* `/status` menunjukkannya dalam baris `Peer address`. Jalur diawali dengan `uds:`.
* Claude Code mengekspornya ke [hooks](/docs/id/hooks) dan perintah Bash sebagai variabel lingkungan [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/id/env-vars#variables):
  * Dalam sesi yang dimulai dengan messaging aktif, Claude Code mengekspor variabel sebelum hook apa pun berjalan, termasuk `SessionStart`.
  * Setiap sesi mengekspor soketnya sendiri, tidak pernah yang diwarisi dari sesi induk.

Di macOS dan Linux, Claude Code membatasi soket ke pengguna sistem operasi Anda. Di Windows native, itu malah memerlukan setiap koneksi untuk mengautentikasi terlebih dahulu dengan kunci yang hanya dapat dibaca pengguna sistem operasi Anda. Bagaimanapun, di mesin bersama sesi pengguna lain tidak dapat mengirimkan ke soket itu.

Di macOS dan Linux, Claude Code juga menolak untuk membuat soket di direktori yang tidak dapat diterima, misalnya yang dimiliki pengguna lain, dan menggunakan direktori pribadi per-pengguna, `/tmp/cc-socks-<uid>`, sebagai gantinya. Ketika tidak dapat menerima direktori apa pun, sesi berjalan tanpa inbox: Claude Code menunjukkan pemberitahuan, `/status` menunjukkan `unavailable` dan alasan dalam baris `Peer address` miliknya, dan log [`--debug`](/docs/id/cli-reference#cli-flags) mencatat penolakan penuh.

Bersama jalur soket, Claude Code mengekspor token per-sesi sebagai [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/id/env-vars#variables). Script yang memposting ke soket sesinya sendiri dapat mengirimkan `{"type":"auth","token":"<token>"}` sebagai baris pertama koneksinya, di mana `<token>` adalah nilai `CLAUDE_CODE_MESSAGING_TOKEN`. Apakah Claude Code memerlukan baris tergantung pada platform:

* **macOS dan Linux, termasuk WSL 2**: baris bersifat opsional. Claude Code menerima koneksi dengan atau tanpanya.
* **Windows native**: baris diperlukan. Claude Code menutup koneksi apa pun yang baris pertamanya bukan baris auth yang valid dan tidak mengirimkan apa pun dari koneksi itu.

Buka koneksi hanya ketika pesan yang Anda posting siap. Claude Code menutup koneksi yang belum mengirimkan baris lengkap dalam 30 detik, jadi tangkap output perintah lambat terlebih dahulu dan kemudian buka koneksi untuk mengirimkannya.

[Aturan own-child](#own-child-messages) di bawah mengatakan kapan Claude Code berkonsultasi dengan token dan bagaimana memperlakukan pesan yang tidak dapat diverifikasi.

<span id="own-child-messages" />Claude Code menjalankan pesan yang tiba di soket melalui [kontrol inbound](#control-inbound-messages) yang sama seperti pesan peer apa pun, dengan satu pengecualian dan satu prasyarat:

* **Pesan own-child**: ketika tidak ada nilai `crossSessionInbound` yang berlaku, Claude Code mengirimkan pesan yang diverifikasi berasal dari proses anak sesi itu sendiri, seperti hook atau perintah Bash memposting kembali ke soket sesinya sendiri.
  * Di Linux, termasuk di dalam WSL 2, Claude Code dapat memverifikasi dengan bukti proses bahkan untuk anak yang sudah keluar. Di macOS dapat memverifikasi dengan cara itu hanya saat proses posting masih berjalan, dan dalam kontainer di mana Claude Code berjalan sebagai ID proses 1 tidak memiliki bukti proses sama sekali. Di Windows native juga tidak memiliki.
  * Di macOS setelah proses posting keluar dan dalam kontainer di mana Claude Code berjalan sebagai ID proses 1, bukti proses itu hilang, dan Claude Code malah memverifikasi anak yang mengirimkan [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/id/env-vars#variables) yang diekspor sesi dalam baris auth yang membuka koneksinya. Di Windows native, token itu adalah satu-satunya cara Claude Code memverifikasi pesan own-child.
  * Ketika Claude Code tidak dapat memverifikasi dengan cara apa pun, itu memperlakukan pesan seperti yang lain yang tidak menyatakan kelas izin, jadi sesi yang melewati prompt izin menahan untuk persetujuan Anda.
* **Sesi sandboxed**: kontrol apakah perintah Bash dapat menjangkau soket dari dalam [sandbox](/docs/id/sandboxing) dengan pengaturan soket Unix sandbox, [`sandbox.network.allowAllUnixSockets` dan `sandbox.network.allowUnixSockets`](/docs/id/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Batasi cross-session messaging
</h2>

Di luar default per-pesan, Anda dapat mempersempit messaging dalam dua cara. Memerlukan persetujuan Anda sebelum pesan apa pun meninggalkan mesin, atau matikan messaging untuk sesi atau organisasi.

<h3 id="require-approval-for-cross-machine-messages">
  Memerlukan persetujuan untuk pesan cross-machine
</h3>

Atur [`isolatePeerMachines`](/docs/id/settings-reference#isolatepeermachines) ke `true` untuk memerlukan persetujuan eksplisit Anda sebelum `SendMessage` apa pun mencapai sesi di luar mesin ini:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Dengan ini diatur, Claude Code meminta persetujuan Anda sebelum pesan Claude ke sesi di luar mesin ini meninggalkan, bahkan dalam mode `bypassPermissions`, yang melewati prompt izin biasa. `true` dari cakupan pengaturan apa pun berlaku, jadi file proyek yang diperiksa dapat mengaktifkan persyaratan tetapi tidak mematikannya. Claude Code tidak meminta pesan antara sesi di mesin yang sama.

<h3 id="turn-off-cross-session-messaging">
  Matikan cross-session messaging
</h3>

Menerima dan mengirim adalah kontrol terpisah, jadi matikan arah mana pun yang Anda butuhkan, atau keduanya. Gunakan `crossSessionInbound` untuk pesan yang tiba, dan aturan izin untuk apa yang dapat dikirim atau didaftar Claude di sini:

* **Berhenti menerima**: atur `crossSessionInbound` ke `refuse`, dan Claude Code menjatuhkan pesan peer inbound tanpa mengirimkannya. Dari pengaturan proyek atau lokal, `refuse` berlaku atas setiap sumber lain, dan dari pengaturan pengguna Anda berlaku kecuali pengaturan terkelola atau bendera `--settings` menetapkan nilai.
* **Berhenti mengirim dan membuat daftar**: tambahkan [aturan deny izin](/docs/id/permissions#tool-specific-permission-rules) yang menamai `SendMessage` dan `ListAgents`. Keduanya mengambil nama tools telanjang tanpa spesifikasi.

Administrator dapat mematikan kedua sisi untuk organisasi dalam [managed settings](/docs/id/managed-settings), menggabungkan aturan deny dengan `refuse`:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Dengan ini berlaku, Claude Code masih mengikat soket inbox setiap sesi, tetapi menjatuhkan setiap pesan yang tiba padanya tanpa mengirimkan apa pun ke Claude. Menolak `SendMessage` juga menghapus messaging ke subagents dan rekan kerja agent-team, karena tools yang sama melayani keduanya. Sesi yang menolak menunjukkan tidak ada perubahan yang terlihat, dalam `/status` miliknya sendiri atau dalam daftar sesi lain di mesin yang sama, jadi untuk mengonfirmasinya, periksa file pengaturan yang berlaku untuk sesi itu daripada statusnya.

<h2 id="availability">
  Availability
</h2>

Cross-session messaging memerlukan Claude Code v2.1.224 atau lebih baru di macOS, Linux, dan WSL 2, dan v2.1.234 atau lebih baru di Windows native. Ketersediaan, dan sesi mana yang dapat dikirim pesan Claude, juga tergantung pada sistem operasi, penyedia, dan konfigurasi Anda:

* **Sistem operasi**: tersedia di macOS, Windows, dan Linux, termasuk Linux di dalam WSL 2.

* **Sesi di mesin ini**: tersedia di setiap penyedia, termasuk Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry, dan dalam sesi yang berjalan dengan [feature-flag fetching](/docs/id/env-vars#features-that-need-feature-flag-fetching) mati. Di penyedia itu, dan dengan flag fetching mati, messaging mesin yang sama memerlukan Claude Code v2.1.248 atau lebih baru. Claude Code mengirimkan pesan ini melalui [soket per-sesi di mesin Anda](#the-sessions-inbox-socket), tidak pernah melalui server Anthropic.

  Untuk menghentikan sesi dari menerimanya, atur [`crossSessionInbound`](#turn-off-cross-session-messaging) ke `refuse`.

* **Sesi di luar mesin ini**: Claude menemukan sesi [Claude Code on the web](/docs/id/claude-code-on-the-web) Anda dan sesi Anda di mesin lain dari sesi yang terhubung ke Remote Control, yang memerlukan sign-in claude.ai sebagai autentikasi aktif sesi ini dan [persyaratan Remote Control](/docs/id/remote-control#requirements) lainnya. Claude tidak dapat menemukan sesi itu dengan API key atau di Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, dan Microsoft Foundry.

Untuk memeriksa sesi, ketik `/list-agents`, juga tersedia sebagai `/peers`. Hasilnya memisahkan sesi yang tidak memiliki fitur dari sesi di mana sesuatu yang lebih sempit memblokir pesan, seperti tools `SendMessage` yang hilang atau pengiriman yang ditolak:

* **`/list-agents` tidak dikenali**: sesi tidak memiliki cross-session messaging. Bekerja melalui persyaratan di atas, dimulai dengan `claude --version` untuk persyaratan versi.
* **`/list-agents` bekerja tetapi pengiriman tidak tiba**: messaging aktif, dan sesuatu yang lebih sempit berlaku:
  * **Aturan deny**: [aturan deny izin](#turn-off-cross-session-messaging) menghapus tools `SendMessage` dan `ListAgents`.
  * **Kontrol inbound**: [kontrol inbound sesi penerima](#control-inbound-messages) dapat menahan atau menjatuhkan apa yang Anda kirimkan.
  * **Sesi cloud hilang**: sesi cloud muncul hanya saat sesi ini terhubung ke [Remote Control](/docs/id/remote-control).
  * **Sesi mesin lain hilang**: sesi di mesin lain Anda muncul hanya ketika berjalan dengan [Remote Control](/docs/id/remote-control) dan sesi ini juga terhubung.
  * **Sesi mesin lain `offline`**: pesan ke sesi yang terdaftar sebagai `offline` akan diteruskan, tetapi [tiba hanya setelah mesin sesi itu terhubung kembali](#message-sessions-on-other-machines).
  * **Sesi cloud atau mesin lain yang lebih lama hilang**: Claude Code [membaca daftar sesi itu terbaru terlebih dahulu dan berhenti setelah jumlah halaman terbatas](#see-which-sessions-claude-can-reach), jadi Claude tidak dapat mengirim pesan ke sesi yang jatuh melewati mereka berdasarkan nama.
  * **Memulai percakapan**: [Message sessions on other machines](#message-sessions-on-other-machines) mencakup memulai percakapan dengan sesi di luar mesin ini.

Dalam sesi dengan messaging, `/status` juga menunjukkan baris `Peer address` dengan alamat inbox sesi itu sendiri, atau `unavailable` dan alasan ketika Claude Code [tidak dapat mengatur inbox](#the-sessions-inbox-socket).

<h2 id="limitations">
  Limitations
</h2>

Batas di sini adalah properti dari saluran messaging itu sendiri dan berlaku di mana pun fitur berjalan. Untuk celah platform dan penyedia, lihat [Availability](#availability) sebagai gantinya.

* **Teks biasa saja**: Claude mengirimkan hanya teks biasa lintas sesi. Pesan protokol [agent team](/docs/id/agent-teams) terstruktur tetap dalam tim.
* **Ukuran pesan mesin yang sama dibatasi**: Claude Code menolak pesan ke sesi di mesin ini setelah bentuk serialnya melewati sekitar satu juta karakter. Penolakan [menamai ukuran yang tepat](/docs/id/errors#message-too-large-for-cross-session-delivery). Tidak ada yang mencapai sesi penerima.
* **Ledakan cepat ke satu sesi ditolak di pengirim**: setelah ledakan cepat pesan ke sesi di mesin ini mencapai apa yang diterima inbox sesi itu, Claude Code menolak pengiriman lebih lanjut dalam sesi pengirim. [Penolakan menamai ledakan](/docs/id/errors#too-many-messages-to-this-session-just-now) dan memberi tahu Claude untuk mengelompokkan sisanya menjadi satu pesan atau menunggu. Sebelum v2.1.236, Claude Code melaporkan pengiriman itu sebagai dikirim sementara sesi penerima menjatuhkannya.
* **Loop pesan dibatasi**: dalam sesi penerima, Claude Code membatasi laju pesan berulang per pengirim, menjatuhkan pengulangan identik yang tiba dalam jendela pendek, dan antrian paling banyak 50 pesan yang diterima untuk dibaca Claude. Loop pesan antara dua sesi karena itu berhenti sendiri. Ketika batas laju, pemeriksaan pengulangan, atau batas antrian menjatuhkan pesan dari sesi interaktif di mesin ini, Claude Code memberi tahu sesi itu mana yang menjatuhkannya dan memberi tahu Claude-nya untuk tidak mengirim ulang segera.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Subagents](/docs/id/sub-agents#resume-subagents) dan [agent teams](/docs/id/agent-teams#messages-between-agents): messaging dalam satu sesi atau tim
* [Background agents](/docs/id/agent-view): dispatch dan monitor sesi paralel yang mungkin Anda kirim pesan
* [Remote Control](/docs/id/remote-control): hubungkan sesi ini untuk menjangkau sesi Anda di mesin lain
* [Settings](/docs/id/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines`, dan `dialogExpiry`
* [Permission modes](/docs/id/permission-modes): mode di balik default inbound dua kelas
* [Tools reference](/docs/id/tools-reference): baris `ListAgents` dan `SendMessage` dalam tabel tools
* [Run agents in parallel](/docs/id/agents): bandingkan cara Claude Code menjalankan beberapa agen
