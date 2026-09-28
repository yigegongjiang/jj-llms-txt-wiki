> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Lacak, putar ulang, dan ringkas edit dan percakapan Claude untuk mengelola status sesi.

Claude Code secara otomatis melacak edit file Claude saat Anda bekerja, memungkinkan Anda dengan cepat membatalkan perubahan dan memutar ulang ke status sebelumnya jika ada yang tidak sesuai.

<h2 id="how-checkpoints-work">
  Cara kerja checkpoints
</h2>

Saat Anda bekerja dengan Claude, checkpointing secara otomatis menangkap status kode Anda sebelum setiap prompt yang Anda kirim yang memulai giliran.

<h3 id="automatic-tracking">
  Pelacakan otomatis
</h3>

Claude Code melacak semua perubahan yang dibuat oleh alat pengeditan filenya:

* Setiap prompt yang Anda kirim yang memulai giliran membuat checkpoint baru
* Claude Code menyimpan snapshot file untuk 100 checkpoint paling terbaru dalam sesi. Membuang checkpoint yang lebih lama menghapus file snapshot yang tidak direferensikan oleh checkpoint yang tersisa, kecuali snapshot pertama setiap file, yang digunakan oleh ekstensi VS Code sebagai baseline untuk diffs sesi-nya.
* Claude Code menyimpan checkpoints dengan percakapan, sehingga Anda masih dapat menjalankan `/rewind` setelah Anda melanjutkan sesi
* Claude Code menghapus snapshot file sesi dalam [retention sweep](/docs/id/claude-directory#cleaned-up-automatically), secara default sekitar 30 hari setelah sesi terakhir menyimpannya. Memutar ulang ke checkpoint yang snapshotnya hilang dapat gagal dengan [`No files were restored`](/docs/id/errors#no-files-were-restored). Untuk menyimpan snapshot lebih lama, atur [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Putar ulang dan ringkas
</h3>

Jalankan `/rewind`, atau tekan `Esc` dua kali ketika bidang input prompt kosong, untuk membuka menu putar ulang.

<Note>
  Jika bidang input prompt berisi teks, tekan `Esc` dua kali akan menghapusnya alih-alih membuka menu. Teks yang dihapus disimpan ke riwayat input Anda, jadi tekan `Up` untuk memanggilnya kembali setelah Anda selesai di menu putar ulang.
</Note>

Menu putar ulang mencantumkan setiap prompt yang Anda kirim selama sesi, kecuali [pesan yang bergabung dengan giliran yang sedang berjalan](#messages-sent-mid-turn-not-checkpointed). Pilih titik yang ingin Anda tindaklanjuti, kemudian pilih tindakan:

* **Pulihkan kode dan percakapan**: kembalikan kode dan percakapan ke titik tersebut
* **Pulihkan percakapan**: putar ulang ke pesan tersebut sambil mempertahankan kode saat ini
* **Pulihkan kode**: kembalikan perubahan file sambil mempertahankan percakapan
* **Ringkas dari sini**: kompres percakapan dari titik ini ke depan menjadi ringkasan, membebaskan ruang context window
* **Ringkas hingga di sini**: kompres percakapan sebelum titik ini menjadi ringkasan, menjaga pesan-pesan selanjutnya tetap utuh
* **Tidak jadi**: kembali ke daftar pesan tanpa membuat perubahan

Dua opsi restore kode muncul hanya ketika checkpoint yang dipilih memiliki perubahan file yang dilacak untuk dikembalikan. Jika tidak ada pengeditan file yang ditangkap setelah titik tersebut, menu hanya menawarkan **Restore conversation**, opsi summarize, dan **Never mind**.

Setelah memulihkan percakapan atau memilih Ringkas dari sini, prompt asli dari pesan yang dipilih dipulihkan ke dalam bidang input sehingga Anda dapat mengirimnya kembali atau mengeditnya.

Memilih Summarize up to here membuat Anda tetap berada di akhir percakapan dengan input kosong. Dengan opsi summarize apa pun, penanda **Summarized conversation** muncul dalam percakapan di mana pesan yang dikompres berada.

<h4 id="rewind-past-a-cleared-conversation">
  Putar ulang melewati percakapan yang dihapus
</h4>

Jika Anda menjalankan `/clear` sebelumnya dalam proses Claude Code yang sama, menu putar ulang menampilkan entri tambahan di bagian atas daftar berlabel `/resume <session-id> (previous session)`. Pilih untuk melanjutkan percakapan yang aktif sebelum `/clear` dijalankan. Entri tersedia hingga Anda keluar dari Claude Code atau melanjutkan sesi yang berbeda.

<h4 id="guide-a-summary">
  Panduan ringkasan
</h4>

Meringkas tidak mengubah file di disk, dan pesan asli tetap berada dalam transkrip sesi, sehingga Claude masih dapat mereferensikan detail. Untuk memandu apa yang difokuskan ringkasan, sorot opsi **Summarize** dengan tombol panah dan ketik instruksi di mana baris membaca **add context (optional)**, kemudian tekan `Enter`. Memilih opsi dengan tombol nomornya meringkas segera tanpa instruksi.

<Note>
  Summarize membuat Anda tetap berada di sesi yang sama dan mengompres konteks, seperti `/compact` yang ditargetkan. Untuk bercabang dan mencoba pendekatan berbeda sambil mempertahankan sesi asli tetap utuh, gunakan [`/branch`](/docs/id/sessions#branch-a-session) atau `claude --continue --fork-session` sebagai gantinya.
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

Checkpointing tidak melacak file yang dimodifikasi oleh perintah Bash. Misalnya, jika Claude Code menjalankan:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Modifikasi file ini tidak dapat dibatalkan melalui rewind. Hanya edit file langsung yang dibuat melalui alat pengeditan file Claude yang dilacak.

<h3 id="subagent-edits-not-restored">
  Edit subagent tidak dipulihkan
</h3>

Sebuah [subagent](/docs/id/sub-agents) membuat edit dengan alat pengeditan file Claude, tetapi Claude Code biasanya tidak menangkap edit tersebut dalam checkpoint sesi Anda. Apakah rewind memulihkannya tergantung pada cara subagent berjalan:

* **Foreground forked skill**: sebuah [skill dengan `context: fork`](/docs/id/skills#run-skills-in-a-subagent) yang berjalan di foreground mengedit working tree Anda selama giliran Anda sendiri, sehingga rewind memulihkan editnya seperti biasa. Atur `background: false` untuk menjalankan fork di foreground; beberapa situasi, [tercantum di halaman skills](/docs/id/skills#run-skills-in-a-subagent), menjalankannya di sana terlepas dari pengaturannya.
* **Subagent lainnya**: rewind tidak memulihkan edit. Gunakan git untuk mengembalikannya. Ini termasuk forked skill yang berjalan di background, default, dan background [`/code-review --fix`](/docs/id/code-review) run.

<h3 id="external-changes-not-tracked">
  Perubahan eksternal tidak dilacak
</h3>

Checkpointing hanya melacak file yang telah diedit dalam sesi saat ini. Perubahan manual yang Anda buat pada file di luar Claude Code dan edit dari sesi bersamaan lainnya biasanya tidak ditangkap, kecuali jika kebetulan memodifikasi file yang sama dengan sesi saat ini.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  Pesan yang dikirim di tengah giliran tidak di-checkpoint
</h3>

Ketika pesan yang Anda [antri saat Claude bekerja](/docs/id/interactive-mode#queue-messages-while-claude-works) mencapai Claude dalam giliran yang sedang berjalan, pesan tersebut bergabung dengan giliran itu alih-alih memulai yang baru. Pesan muncul dalam percakapan, tetapi Claude Code tidak membuat checkpoint untuknya, dan menu rewind tidak mencantumnya. Pesan antri yang Claude Code kirim sebagai gilirannya sendiri mendapatkan checkpoint seperti biasa.

Untuk menghapus pesan seperti itu, atau membatalkan edit yang Claude buat setelahnya, rewind ke prompt yang memulai giliran. Itu melakukan rewind seluruh giliran, termasuk pekerjaan yang Claude lakukan sebelum pesan Anda tiba.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  Jalur symlink dan hard-link tidak dipulihkan
</h3>

Checkpointing tidak melakukan rewind file symlink atau hard-link. Ketika Anda memilih **Restore code** atau **Restore code and conversation** dari menu `/rewind`, Claude Code melewati jalur terlacak apa pun yang merupakan symlink atau hard link dan menampilkan peringatan `Restored the code, but skipped N files`. File yang dilewati mempertahankan konten saat ini mereka. Untuk membatalkan perubahan sesi pada salah satunya, minta Claude untuk membalikkan edit atau edit file sendiri. File konfigurasi yang disymlink oleh dotfile manager ke proyek Anda dan file yang dipnpm hard-link ke tempat keduanya termasuk dalam kategori ini.

Untuk melihat jalur mana yang dilewati restore, aktifkan debug logging dengan `/debug` sebelum Anda memulihkan: debug log di `~/.claude/debug/<session-id>.txt` menamai setiap jalur yang dilewati. Untuk setiap alasan skip dan langkah pemulihan, lihat [entri skipped-files dalam referensi kesalahan](/docs/id/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  Bukan pengganti kontrol versi
</h3>

Checkpoints dirancang untuk pemulihan cepat tingkat sesi. Untuk riwayat versi permanen dan kolaborasi, terus gunakan kontrol versi, seperti Git, untuk commit, branch, dan riwayat jangka panjang.

<h2 id="see-also">
  Lihat juga
</h2>

* [Mode interaktif](/docs/id/interactive-mode) - Pintasan keyboard dan kontrol sesi
* [Commands](/docs/id/commands) - Mengakses checkpoints menggunakan `/rewind`
* [CLI reference](/docs/id/cli-reference) - Opsi baris perintah
