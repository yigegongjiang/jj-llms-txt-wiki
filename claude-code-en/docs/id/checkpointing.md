> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Lacak, putar ulang, dan ringkas edit dan percakapan Claude untuk mengelola status sesi.

Claude Code secara otomatis melacak edit file Claude saat Anda bekerja, memungkinkan Anda dengan cepat membatalkan perubahan dan memutar ulang ke status sebelumnya jika ada yang tidak sesuai.

<h2 id="how-checkpoints-work">
  Cara kerja checkpoints
</h2>

Saat Anda bekerja dengan Claude, checkpointing secara otomatis menangkap status kode Anda sebelum setiap prompt pengguna. Jaring pengaman ini memungkinkan Anda mengejar tugas-tugas yang ambisius dan berskala besar dengan mengetahui Anda selalu dapat kembali ke status kode sebelumnya.

<h3 id="automatic-tracking">
  Pelacakan otomatis
</h3>

Claude Code melacak semua perubahan yang dibuat oleh alat pengeditan filenya:

* Setiap prompt pengguna membuat checkpoint baru
* Claude Code menyimpan snapshot file untuk 100 checkpoint paling terbaru dalam sesi. Membuang checkpoint yang lebih lama menghapus file snapshot yang tidak direferensikan oleh checkpoint yang tersisa, kecuali snapshot pertama setiap file, yang digunakan oleh ekstensi VS Code sebagai baseline untuk diffs sesi-nya. Sebelum v2.1.208, file snapshot yang digantikan tersebut tetap berada di disk hingga sesi dibersihkan.
* Checkpoints disimpan dengan percakapan, sehingga sesi yang dilanjutkan masih dapat `/rewind` ke dalamnya
* Dibersihkan secara otomatis bersama dengan sesi setelah 30 hari (dapat dikonfigurasi)

<h3 id="rewind-and-summarize">
  Putar ulang dan ringkas
</h3>

Jalankan `/rewind`, atau tekan `Esc` dua kali ketika bidang input prompt kosong, untuk membuka menu putar ulang.

<Note>
  Jika bidang input prompt berisi teks, tekan `Esc` dua kali akan menghapusnya alih-alih membuka menu. Teks yang dihapus disimpan ke riwayat input Anda, jadi tekan `Up` untuk memanggilnya kembali setelah Anda selesai di menu putar ulang.
</Note>

Menu putar ulang mencantumkan setiap prompt yang Anda kirim selama sesi. Pilih titik yang ingin Anda tindaklanjuti, kemudian pilih tindakan:

* **Pulihkan kode dan percakapan**: kembalikan kode dan percakapan ke titik tersebut
* **Pulihkan percakapan**: putar ulang ke pesan tersebut sambil mempertahankan kode saat ini
* **Pulihkan kode**: kembalikan perubahan file sambil mempertahankan percakapan
* **Ringkas dari sini**: kompres percakapan dari titik ini ke depan menjadi ringkasan, membebaskan ruang context window
* **Ringkas hingga di sini**: kompres percakapan sebelum titik ini menjadi ringkasan, menjaga pesan-pesan selanjutnya tetap utuh
* **Tidak jadi**: kembali ke daftar pesan tanpa membuat perubahan

Setelah memulihkan percakapan atau memilih Ringkas dari sini, prompt asli dari pesan yang dipilih dipulihkan ke dalam bidang input sehingga Anda dapat mengirimnya kembali atau mengeditnya.

Memilih Ringkas hingga di sini membuat Anda tetap berada di akhir percakapan dengan input kosong.

<h4 id="rewind-past-a-cleared-conversation">
  Putar ulang melewati percakapan yang dihapus
</h4>

Jika Anda menjalankan `/clear` sebelumnya dalam proses Claude Code yang sama, menu putar ulang menampilkan entri tambahan di bagian atas daftar berlabel `/resume <session-id> (previous session)`. Pilih untuk melanjutkan percakapan yang aktif sebelum `/clear` dijalankan. Entri tersedia hingga Anda keluar dari Claude Code atau melanjutkan sesi yang berbeda, dan memerlukan Claude Code v2.1.191 atau lebih baru. Pada versi sebelumnya, jalankan `/resume` dan pilih sesi sebelumnya dari daftar sebagai gantinya.

<h4 id="restore-vs-summarize">
  Pulihkan vs. ringkas
</h4>

Opsi pemulihan mengembalikan status: mereka membatalkan perubahan kode, riwayat percakapan, atau keduanya. Opsi ringkas mengompres bagian dari percakapan menjadi ringkasan yang dihasilkan AI tanpa mengubah file di disk:

* **Ringkas dari sini**: pesan sebelum pesan yang dipilih tetap utuh. Pesan yang dipilih dan semua pesan berikutnya diganti dengan ringkasan. Gunakan ini untuk membuang diskusi sampingan sambil menjaga konteks awal dalam detail lengkap.
* **Ringkas hingga di sini**: pesan sebelum pesan yang dipilih diganti dengan ringkasan. Pesan yang dipilih dan semua pesan berikutnya tetap utuh, dan Anda tetap berada di akhir percakapan. Gunakan ini untuk mengompres diskusi setup awal sambil menjaga pekerjaan terbaru dalam detail lengkap.

Dalam kedua kasus, pesan asli disimpan dalam transkrip sesi, sehingga Claude dapat mereferensikan detail jika diperlukan. Anda dapat mengetik instruksi opsional untuk memandu fokus ringkasan. Ini mirip dengan `/compact`, tetapi ditargetkan: alih-alih meringkas seluruh percakapan, Anda memilih sisi mana dari pesan yang dipilih untuk dikompres.

<Note>
  Ringkas membuat Anda tetap berada di sesi yang sama dan mengompres konteks. Jika Anda ingin bercabang dan mencoba pendekatan berbeda sambil mempertahankan sesi asli tetap utuh, gunakan [fork](/docs/id/sessions#branch-a-session) sebagai gantinya (`claude --continue --fork-session`).
</Note>

<h2 id="common-use-cases">
  Kasus penggunaan umum
</h2>

Checkpoints sangat berguna ketika:

* **Menjelajahi alternatif**: coba pendekatan implementasi berbeda tanpa kehilangan titik awal Anda
* **Memulihkan dari kesalahan**: dengan cepat batalkan perubahan yang memperkenalkan bug atau merusak fungsionalitas
* **Iterasi pada fitur**: bereksperimen dengan variasi mengetahui Anda dapat kembali ke status yang berfungsi
* **Membebaskan ruang konteks**: ringkas sesi debugging yang bertele-tele dari titik tengah ke depan, menjaga instruksi awal Anda tetap utuh

<h2 id="limitations">
  Keterbatasan
</h2>

<h3 id="bash-command-changes-not-tracked">
  Perubahan perintah Bash tidak dilacak
</h3>

Checkpointing tidak melacak file yang dimodifikasi oleh perintah bash. Misalnya, jika Claude Code menjalankan:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Modifikasi file ini tidak dapat dibatalkan melalui rewind. Hanya edit file langsung yang dibuat melalui alat pengeditan file Claude yang dilacak.

<h3 id="external-changes-not-tracked">
  Perubahan eksternal tidak dilacak
</h3>

Checkpointing hanya melacak file yang telah diedit dalam sesi saat ini. Perubahan manual yang Anda buat pada file di luar Claude Code dan edit dari sesi bersamaan lainnya biasanya tidak ditangkap, kecuali jika kebetulan memodifikasi file yang sama dengan sesi saat ini.

<h3 id="not-a-replacement-for-version-control">
  Bukan pengganti kontrol versi
</h3>

Checkpoints dirancang untuk pemulihan cepat tingkat sesi. Untuk riwayat versi permanen dan kolaborasi:

* Terus gunakan kontrol versi (mis. Git) untuk commit, branch, dan riwayat jangka panjang
* Checkpoints melengkapi tetapi tidak menggantikan kontrol versi yang tepat
* Pikirkan checkpoints sebagai "undo lokal" dan Git sebagai "riwayat permanen"

<h2 id="see-also">
  Lihat juga
</h2>

* [Mode interaktif](/docs/id/interactive-mode) - Pintasan keyboard dan kontrol sesi
* [Commands](/docs/id/commands) - Mengakses checkpoints menggunakan `/rewind`
* [CLI reference](/docs/id/cli-reference) - Opsi baris perintah
