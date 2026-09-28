> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code di mobile

> Mulai, pantau, dan arahkan tugas Claude Code dari ponsel Anda dengan aplikasi Claude untuk iOS dan Android.

Aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) dan [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) adalah klien untuk sesi Claude Code daripada tempat di mana kode dijalankan. Dari ponsel Anda, Anda menjangkau [sesi cloud](#start-and-monitor-cloud-sessions) dan [proyek](/docs/id/claude-projects) di cloud, sesi yang berjalan di mesin Anda sendiri melalui [Remote Control](#continue-a-local-session-with-remote-control), atau aplikasi Desktop melalui [Dispatch](/docs/id/desktop#sessions-from-dispatch).

<Note>
  Claude Code tidak memiliki aplikasi mobile terpisah: sesi cloud dan Remote Control keduanya berada di tab **Code** di aplikasi Claude, dan Dispatch adalah tugas yang Anda kirimi pesan di aplikasi.
</Note>

<h2 id="get-the-app">
  Dapatkan aplikasi
</h2>

<Steps>
  <Step title="Unduh aplikasi Claude">
    Instal aplikasi Claude untuk [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) atau [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Di iPad, instal aplikasi iOS yang sama.

    <Tip>
      Jalankan `/mobile` dalam sesi Claude Code untuk menampilkan kode QR untuk [claude.ai/mobile](https://claude.ai/mobile), yang membuka app store yang tepat untuk ponsel Anda. `/ios` dan `/android` melakukan hal yang sama.
    </Tip>
  </Step>

  <Step title="Masuk">
    Masuk dengan akun claude.ai dan organisasi yang sama yang Anda gunakan untuk Claude Code. Sesi cloud dan Remote Control memerlukan akun claude.ai, jadi tidak dapat dijangkau dengan kunci API Anthropic Console atau dari penyedia pihak ketiga seperti Amazon Bedrock.
  </Step>

  <Step title="Buka tab Code">
    Ketuk **Code** di navigasi aplikasi untuk menjangkau sesi Anda, atau buka [claude.ai/code/new](https://claude.ai/code/new) di ponsel Anda untuk memulai sesi Code baru di aplikasi. Jika Anda tidak melihat tab Code, paket atau organisasi Anda mungkin tidak menyertakan fitur-fitur ini; lihat [ketersediaan menurut paket langganan](/docs/id/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Bekerja dari ponsel Anda
</h2>

Dari aplikasi Anda dapat memulai sesi cloud, membuka proyek, menjalankan sesi Claude Code yang berjalan di komputer Anda, atau mengirimkan pesan Dispatch dengan tugas. Aplikasi sama untuk masing-masing; mereka berbeda dalam tempat pekerjaan terjadi.

| Fitur                                          | Apa yang Anda hubungkan                                                      | Kapan digunakan                                                                                                                                                     |
| :--------------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Cloud sessions](/docs/id/claude-code-on-the-web)   | Sesi di infrastruktur cloud, dikelola Anthropic secara default               | Repositori Anda ada di GitHub dan tugas harus terus berjalan setelah Anda meletakkan ponsel Anda. Lihat [panduan cepat cloud](/docs/id/web-quickstart) untuk menyiapkan. |
| [Projects](/docs/id/claude-projects)                | Percakapan di mana Claude mengoordinasikan sesi cloud paralel sebagai thread | Anda memiliki aliran pekerjaan terkait daripada satu tugas dan ingin melihat thread mana yang selesai atau memerlukan Anda.                                         |
| [Remote Control](/docs/id/remote-control)           | Sesi Claude Code yang berjalan di komputer Anda                              | Pekerjaan memerlukan sistem file lokal, alat, atau server MCP Anda.                                                                                                 |
| [Dispatch](/docs/id/desktop#sessions-from-dispatch) | Aplikasi Desktop di komputer Anda                                            | Anda ingin mengirimkan pesan tugas dan membiarkan Dispatch memutuskan cara menjalankannya. Memerlukan paket Pro atau Max.                                           |

Jika komputer Anda akan mati, gunakan cloud sessions atau proyek, yang berjalan di cloud dan berlanjut dengan laptop Anda ditutup. Remote Control dan Dispatch menjalankan mesin Anda sendiri, jadi perlu tetap menyala dengan Claude Code atau aplikasi Desktop berjalan. Jika mesin Anda tidur selama sesi Remote Control, Claude Code terhubung kembali saat mesin kembali online.

Untuk perbandingan yang lebih lengkap, lihat [bekerja saat Anda jauh dari terminal Anda](/docs/id/platforms#work-when-you-are-away-from-your-terminal).

Cloud sessions dan Remote Control berjalan dari tab **Code**. Untuk Dispatch, yang Anda kirimkan pesan sebagai tugas di aplikasi, lihat [sesi dari Dispatch](/docs/id/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Mulai dan pantau cloud sessions
</h3>

Cloud sessions menjalankan tugas di infrastruktur cloud, dikelola Anthropic secara default, jadi sesi berlanjut setelah Anda meletakkan ponsel Anda. Dari tab Code, pilih repositori dan cabang, jelaskan tugas, dan kirimkan. Sesi bertahan di seluruh perangkat: tugas yang Anda mulai di laptop Anda siap untuk ditinjau dari ponsel Anda, dan yang Anda mulai dari ponsel Anda menunggu saat Anda kembali ke meja Anda.

Buka sesi di aplikasi untuk memeriksa kemajuan, menjawab pertanyaan Claude, atau mengarahkannya ke arah baru. Anda juga dapat memberi tahu Claude untuk [menonton permintaan tarik](/docs/id/claude-code-on-the-web#auto-fix-pull-requests) dan memperbaiki kegagalan CI atau meninjau komentar saat tiba. Untuk menghubungkan GitHub dan menyiapkan lingkungan Anda, ikuti [panduan cepat cloud](/docs/id/web-quickstart), dan lihat [Gunakan Claude Code di cloud](/docs/id/claude-code-on-the-web) untuk semua yang dapat dilakukan cloud sessions.

<h3 id="continue-a-local-session-with-remote-control">
  Lanjutkan sesi lokal dengan Remote Control
</h3>

Remote Control menghubungkan aplikasi Claude ke sesi Claude Code yang berjalan di mesin Anda, jadi eksekusi kode dan akses sistem file tetap lokal sementara Anda menjalankan sesi dari ponsel Anda. Mulai sesi di komputer Anda dengan `claude remote-control`, atau jalankan `/remote-control` dalam sesi yang sudah terbuka. Kemudian pindai kode QR yang dapat ditampilkan terminal, atau buka aplikasi Claude, ketuk **Code**, dan pilih sesi dari daftar. Lihat [hubungkan dari perangkat lain](/docs/id/remote-control#connect-from-another-device) untuk setiap opsi.

Ketika Anda menambahkan lampiran di aplikasi Claude, lampiran tersebut mencapai sesi lokal juga:

* **Foto**: Claude melihat foto yang dilampirkan secara langsung sebagai bagian dari pesan Anda. Claude Code juga menyimpan setiap foto di bawah `~/.claude/uploads/` dan memberi tahu Claude jalur file yang disimpan, jadi Claude dapat menyalin gambar ke dalam file yang dibuatnya.
* **File lainnya**: Claude Code mengunduhnya ke mesin Anda dan meneruskannya ke Claude sebagai referensi file `@`.

Untuk persyaratan, mode invokasi, dan pemecahan masalah, lihat [gambaran umum Remote Control](/docs/id/remote-control).

<h3 id="get-push-notifications">
  Dapatkan notifikasi push
</h3>

Ketika Remote Control aktif, Claude dapat mengirimkan notifikasi push ke ponsel Anda, biasanya ketika tugas yang berjalan lama selesai atau ketika memerlukan keputusan dari Anda. Anda juga dapat meminta satu dalam prompt Anda, seperti `notify me when the tests finish`. Lihat [notifikasi push mobile](/docs/id/remote-control#mobile-push-notifications) untuk dua toggle `/config` dan pemecahan masalah pengiriman.

Dispatch mengirimkan notifikasinya sendiri ketika sesi Code yang dihasilkannya selesai atau memerlukan persetujuan Anda, dijelaskan dalam [sesi dari Dispatch](/docs/id/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Keterbatasan
</h2>

Klien mobile mencakup sebagian besar dari apa yang dibutuhkan sesi, dengan beberapa keterbatasan:

* **Perintah lokal saja**: perintah yang hanya berjalan di antarmuka terminal, seperti `/plugin` dan `/resume`, tidak berfungsi dari aplikasi. [Keterbatasan Remote Control](/docs/id/remote-control#limitations) mencantumkan perintah yang berfungsi dari mobile dan bagaimana perilakunya berbeda.
* **Mode izin**: sesi cloud menawarkan Accept edits, Plan, dan Auto di dropdown mode, dan sesi Remote Control menawarkan Manual, Accept edits, dan Plan. Anda tidak dapat memilih Bypass permissions dari aplikasi dalam kedua kasus, dan Anda tidak dapat memilih Auto untuk sesi Remote Control. Lihat [beralih mode izin](/docs/id/permission-modes#switch-permission-modes).
* **Rencana Dispatch**: Dispatch memerlukan paket Pro atau Max dan tidak tersedia di Team atau Enterprise.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Platform dan integrasi](/docs/id/platforms): bandingkan setiap permukaan tempat Claude Code berjalan
* [Claude Code di web](/docs/id/claude-code-on-the-web): bagaimana sesi cloud berjalan dan cara memindahkan pekerjaan ke dan dari terminal Anda
* [Konfigurasi lingkungan cloud](/docs/id/cloud-environments): tingkat akses jaringan, variabel lingkungan, dan skrip penyiapan untuk sesi cloud
* [Remote Control](/docs/id/remote-control): lanjutkan sesi lokal dari perangkat apa pun
* [Sesi dari Dispatch](/docs/id/desktop#sessions-from-dispatch): bagaimana tugas Dispatch menjadi sesi Code di aplikasi Desktop
* [Channels](/docs/id/channels): tanyakan sesuatu kepada Claude dari ponsel Anda melalui Telegram, Discord, atau iMessage sementara pekerjaan berjalan di mesin Anda
* [Claude Code di Slack](/docs/id/slack): delegasikan tugas pengkodean dari ruang kerja Slack Anda dengan menyebutkan `@Claude`
