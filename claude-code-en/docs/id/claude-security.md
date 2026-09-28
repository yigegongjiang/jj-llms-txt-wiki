> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pindai basis kode Anda untuk menemukan kerentanan

> Instal plugin Claude Security untuk memindai basis kode Anda mencari kerentanan dalam sesi Claude Code dan ubah temuan menjadi patch yang Anda tinjau dan terapkan.

Plugin Claude Security menjalankan pemindaian kerentanan multi-agen dari basis kode Anda di dalam sesi Claude Code. Sebuah tim agen Claude memetakan arsitektur Anda, membangun model ancaman, berburu kerentanan, dan secara independen meninjau setiap temuan sebelum menulis laporan. Gunakan plugin untuk memindai seluruh repositori atau [hanya serangkaian perubahan](#scan-only-your-changes), seperti diff cabang, diff permintaan tarik, atau komit tunggal, kemudian ubah temuan yang Anda pilih menjadi patch yang Anda tinjau dan terapkan sendiri.

Plugin berjalan secara lokal dalam sesi Anda, menggunakan model apa pun yang Anda miliki akses di Claude Code, dan setiap pemindaian dihitung terhadap batas penggunaan rencana Anda. Jika Anda menginginkan layanan terkelola yang memantau repositori Anda, atau ingin menjalankan pemindaian pada [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5), lihat produk [Claude Security](https://claude.com/product/claude-security), tersedia di paket Enterprise. Plugin ini menjangkau kode yang tidak dapat dijangkau produk terkelola, seperti repositori yang dihosting di GitLab atau Bitbucket, atau di jaringan yang tidak memungkinkan koneksi masuk.

Plugin ini juga berbeda dari alat tinjauan yang sudah ada di Claude Code: plugin [security guidance](/docs/id/security-guidance) meninjau kode saat Claude menulisnya, [`/security-review`](/docs/id/commands#all-commands) menjalankan satu kali melewati cabang Anda, dan [Code Review](/docs/id/code-review) meninjau permintaan tarik. Untuk cara lapisan bertumpuk, lihat [Bagaimana plugin ini sesuai dengan alat keamanan lainnya](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Prasyarat
</h2>

Untuk menjalankan plugin, Anda memerlukan:

* Paket berbayar, untuk [dynamic workflows](/docs/id/workflows) yang digunakan pemindaian untuk mengorkestrasi agennya. Di Pro, aktifkan dari baris Dynamic workflows di `/config`.
* Python 3.9 atau lebih baru tersedia di `PATH` Anda sebagai `python3`. Periksa dengan `python3 --version`. Alat plugin hanya menggunakan pustaka standar Python, jadi tidak ada yang diinstal.
* Linux, macOS, atau Windows.
* Git, untuk pemindaian perubahan dan untuk mengubah temuan menjadi patch; pekerjaan tersebut tidak mendukung sistem kontrol versi lainnya. Pemindaian penuh berfungsi di direktori apa pun, dengan atau tanpa kontrol versi.

<h2 id="install-the-plugin">
  Instal plugin
</h2>

Dalam sesi Claude Code, instal dari [pasar resmi Anthropic](/docs/id/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

Perintah membuka detail plugin, di mana Anda memilih [cakupan instalasi](/docs/id/plugins/install#install-a-plugin) untuk memulai instalasi.

Jika instalasi gagal, perbaikannya tergantung pada pesan yang dilaporkan Claude Code:

* Jika melaporkan `Marketplace "claude-plugins-official" not found`, tambahkan pasar dengan `/plugin marketplace add anthropics/claude-plugins-official`, kemudian coba ulang instalasi.
* Jika melaporkan bahwa [tidak dapat menemukan plugin di pasar](/docs/id/plugins/install#install-a-plugin), periksa nama plugin untuk kesalahan ketik.

Periksa ringkasan instalasi. Jika melaporkan `Run /reload-plugins to activate.`, lihat [Terapkan perubahan plugin tanpa memulai ulang](/docs/id/plugins/cli-reference#reload-plugins) untuk mengaktifkan plugin dalam sesi Anda saat ini.

Setelah plugin aktif, Anda siap untuk [memindai dan memperbaiki basis kode Anda](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Copot plugin
</h3>

Untuk menghapus plugin, copot dari menu `/plugin`, atau jalankan `claude plugin uninstall claude-security` di terminal Anda.

<h2 id="scan-and-fix-your-codebase">
  Pindai dan perbaiki basis kode Anda
</h2>

Plugin menambahkan satu perintah, `/claude-security`, yang membuka menu dari tiga pekerjaannya: memindai basis kode, memindai serangkaian perubahan, dan menyarankan patch. Jalur bahagia menjalankan pemindaian penuh, kemudian mengubah temuannya menjadi patch:

<Steps>
  <Step title="Buka menu Claude Security">
    Jalankan `/claude-security` dan pilih **Scan codebase**.
  </Step>

  <Step title="Pilih apa yang akan dipindai">
    Plugin membaca repositori Anda terlebih dahulu, kemudian menawarkan seluruh repositori atau area terfokus, dengan jumlah file dan biaya relatif setiap opsi dinyatakan. Pilih seluruh repositori, atau jawab "I don't know" dan plugin memilih default yang masuk akal untuk ukuran repositori Anda.
  </Step>

  <Step title="Konfirmasi eksekusi">
    Pemindaian mungkin memakan waktu, mungkin menggunakan jumlah token yang signifikan, dan memerlukan Claude Code tetap terbuka saat selesai. Tidak ada yang berjalan sampai Anda mengonfirmasi.
  </Step>

  <Step title="Baca laporan">
    Saat pemindaian berjalan, laporan setiap tahap saat dimulai, dengan detail tersedia di bawah [`/workflows`](/docs/id/workflows). Hasil mendarat di direktori dengan stempel waktu di repositori Anda, dijelaskan dalam [Baca hasil pemindaian](#read-the-scan-results).
  </Step>

  <Step title="Ubah temuan menjadi patch">
    Jalankan `/claude-security` lagi dan pilih **Suggest patches**, kemudian pilih temuan mana yang akan ditangani. Patch yang ditinjau mendarat di folder `patches/` laporan; [Perbaiki temuan](#fix-findings) mencakup cara setiap patch dibangun dan ditinjau.
  </Step>

  <Step title="Terapkan patch yang Anda terima">
    Terapkan setiap patch dari shell Anda dengan `git apply`, dalam permintaan tarik sendirinya. Patch tidak pernah diterapkan secara otomatis.
  </Step>
</Steps>

Anda tidak harus memulai dari menu: minta pekerjaan secara langsung, sebagai argumen untuk perintah, seperti `/claude-security scan my branch`, atau dalam bahasa biasa, seperti "scan commit abc1234". Plugin bekerja terbaik dalam [mode otomatis](/docs/id/permission-modes), yang memungkinkan agen pemindaian untuk melanjutkan tanpa permintaan izin di setiap langkah.

<h3 id="scan-only-your-changes">
  Pindai hanya perubahan Anda
</h3>

Ketika cabang Anda memiliki komit yang tidak dimiliki basis, menu `/claude-security` menawarkan untuk memindai hanya diff itu, sehingga Anda dapat memeriksa cabang sebelum menggabungkan. Anda juga dapat memindai salah satu permintaan tarik terbuka Anda, atau komit tunggal dengan memintanya, seperti "scan commit abc1234". Hanya perubahan yang berkomit yang dipindai: komit atau stash edit yang sedang berlangsung terlebih dahulu, atau jalankan pemindaian penuh, yang membaca pohon kerja.

Pemindaian perubahan memerlukan repositori git; pemindaian penuh direktori tanpa versi masih berfungsi. Menemukan permintaan tarik terbuka Anda adalah satu-satunya langkah yang menjangkau jaringan, dan ditawarkan hanya ketika sesi Anda sudah memiliki izin untuk menjalankan CLI GitHub dan `gh` masuk.

<h3 id="scope-large-repositories">
  Cakupan repositori besar
</h3>

Di repositori besar, pindai satu area sekaligus alih-alih seluruh pohon. Pilih salah satu cakupan terfokus yang ditawarkan plugin, seperti lapisan API atau kode autentikasi Anda, dan pemindaian menyesuaikan cakupannya dengan apa yang Anda pilih. Bagian cakupan laporan menyatakan apa yang diperiksa dan tidak diperiksa. Jalankan pemindaian lain di area berbeda kapan saja.

<h3 id="read-the-scan-results">
  Baca hasil pemindaian
</h3>

Setiap pemindaian menulis hasilnya ke direktori `CLAUDE-SECURITY-<timestamp>/` dengan stempel waktu di repositori Anda:

* **`CLAUDE-SECURITY-RESULTS.md`**: laporan, dengan ID setiap temuan, seperti `F1`, ditambah dampak, skenario eksploitasi, keparahan, kepercayaan diri, dan rekomendasi
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: temuan yang sama dalam bentuk yang dapat dibaca mesin, satu objek JSON per baris
* **`CLAUDE-SECURITY-RESULTS.sarif`**: temuan yang sama sebagai log [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) untuk pemindaian kode GitHub dan alat lain yang membaca standar. Pemindaian mengklasifikasikan temuan di bawah kategori kelemahan [CWE](https://cwe.mitre.org/) mereka
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: stempel revisi, mencatat komit mana yang dipindai, dengan upaya apa, apakah perubahan yang tidak berkomit adalah bagian dari pohon yang dipindai, dan seberapa menyeluruh jalannya pemindaian diverifikasi, sehingga laporan selalu terikat pada kode yang dijelaskannya. Pemindaian di luar kontrol versi memberi stempel `UNVERSIONED` sebagai pengganti komit

Direktori itu adalah satu-satunya perubahan yang dibuat pemindaian pada checkout Anda, dan membawa `.gitignore` sendiri, sehingga `git add` yang tersesat tidak pernah menyapu laporan ke dalam komit. Untuk menyimpan laporan dalam riwayat untuk jejak audit, hapus satu file `.gitignore` itu dan komit direktori seperti yang lain.

Temuan hanya muncul dalam laporan setelah agen verifikasi independen menganalisisnya, yang membuat laporan singkat dan layak dibaca. Pemindaian bersifat nondeterministik: dua pemindaian kode yang sama dapat mengungkap temuan yang berbeda. Jalankan pemindaian secara teratur, dan gunakan stempel revisi untuk mengatribusikan setiap laporan ke kode dan pengaturan yang tepat yang dicakupnya.

<h2 id="fix-findings">
  Perbaiki temuan
</h2>

Mulai alur perbaikan dengan memilih **Suggest patches** dari menu `/claude-security`, atau tanyakan dalam bahasa biasa, seperti "fix finding F3", kemudian pilih temuan mana dari laporan yang akan ditangani. Patch dibangun terhadap kode yang berkomit, dan laporan harus masih menjelaskan kode yang Anda miliki: temuan yang kodenya telah berubah sejak saat itu dilewati dengan catatan, dan plugin menawarkan pemindaian segar alih-alih menambal dari laporan basi. Setiap patch dirancang dalam salinan kerja sementara repositori Anda, sehingga file sumber Anda tetap tidak tersentuh sampai Anda menerapkan patch sendiri.

Sebelum pengiriman, setiap patch ditinjau oleh agen yang independen dari yang menulisnya, yang menjalankan tes proyek Anda terhadap perubahan ketika kode memilikinya dan membaca diff menurut penilaiannya sendiri untuk apa pun yang baru yang mungkin diperkenalkannya. Patch ditulis hanya ketika tinjauan itu dapat membuktikan bahwa perubahan mengatasi satu temuan, tidak memperkenalkan kerentanan baru, dan membiarkan perilaku lainnya tidak berubah. Ketika tidak dapat membuktikan ketiga-tiganya, Anda mendapatkan catatan singkat yang menjelaskan alasannya alih-alih patch.

<h3 id="patches-are-never-applied-automatically">
  Patch tidak pernah diterapkan secara otomatis
</h3>

Menerapkan patch selalu keputusan Anda. Patch mendarat di folder `patches/` laporan, satu `F<n>.patch` per temuan dengan catatan di sebelahnya menjelaskan perubahan. Terapkan satu dari shell Anda, atau minta Claude untuk menerapkannya dan membuka permintaan tarik:

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Ketika kode yang dipatch tidak memiliki tes, catatan patch mengatakan demikian, sehingga Anda tahu tinjauannya berjalan tanpa pengujian. Terapkan setiap patch dalam permintaan tariknya sendiri sehingga dapat ditinjau dan diuji sendiri.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Bagaimana plugin cocok dengan alat keamanan lainnya
</h2>

Plugin Claude Security adalah lapisan pemindaian mendalam sesuai permintaan dalam tumpukan pertahanan berlapis, bersama dengan plugin [security guidance](/docs/id/security-guidance), [`/security-review`](/docs/id/commands#all-commands), [Code Review](/docs/id/code-review), produk [Claude Security](https://claude.com/product/claude-security) yang dikelola, dan pemindai yang ada:

| Tahap                                  | Alat                                                                            | Apa yang dicakup                                                                                    |
| :------------------------------------- | :------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------- |
| Dalam sesi                             | Plugin [Security guidance](/docs/id/security-guidance)                               | Kerentanan umum dalam kode yang ditulis Claude, diperbaiki dalam sesi yang sama                     |
| Sesuai permintaan, satu kali melewati  | [`/security-review`](/docs/id/commands#all-commands)                                 | Satu kali lintasan keamanan pada cabang saat ini                                                    |
| Sesuai permintaan, pemindaian mendalam | Plugin Claude Security                                                          | Pemindaian multi-agen repositori atau diff, dengan temuan dan patch yang ditinjau secara independen |
| Pada permintaan tarik                  | [Code Review](/docs/id/code-review), paket Team dan Enterprise                       | Tinjauan kebenaran dan keamanan multi-agen dengan konteks basis kode penuh                          |
| Dikelola                               | [Claude Security](https://claude.com/product/claude-security), paket Enterprise | Pemindaian yang dihosting yang memantau repositori yang terhubung                                   |
| Di CI                                  | Pemindai analisis statis dan ketergantungan yang ada                            | Aturan khusus bahasa, pemeriksaan rantai pasokan, dan penegakan kebijakan                           |

Plugin tidak menggantikan alat keamanan kode sumber yang ada. Jalankan bersama analisis statis, pemindaian ketergantungan, dan tinjauan kode: alat ini bernalar tentang kode Anda seperti peneliti keamanan manusia, yang melengkapi pemeriksaan deterministik yang disediakan alat tersebut.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

**Menu `/claude-security` terbuka dengan peringatan Python.** Plugin memerlukan `python3` 3.9 atau lebih baru di `PATH` Anda. Ketika tidak dapat menemukan `python3` sama sekali, menu memperingatkan bahwa Claude Security tidak akan berfungsi sampai satu diinstal; ketika `python3` pertama di `PATH` Anda lebih lama, peringatan menyebutkan versi yang ditemukannya. Instal Python 3, atau letakkan `python3` yang lebih baru terlebih dahulu di `PATH` Anda, kemudian mulai sesi baru.

**Anda mungkin melihat pemberitahuan "safeguards flagged this message" saat memindai pada model Fable.** Pesan menyebutkan model, misalnya "Fable 5.1's safeguards flagged this message". Pengklasifikasi keamanan siber Fable menandai permintaan tertentu, dan Claude Code menjalankan kembali permintaan yang ditandai pada model Opus melalui [automatic model fallback](/docs/id/model-config#automatic-model-fallback). Ini diharapkan, dan pemindaian harus tetap selesai dengan sukses.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Untuk mendalami bagian yang disentuh halaman ini:

* [Plugin Security guidance](/docs/id/security-guidance): tangkap masalah dalam kode saat Claude menulisnya, dalam sesi yang sama
* [Code Review](/docs/id/code-review): atur tinjauan multi-agen waktu PR
* [Claude Security](https://claude.com/product/claude-security): layanan terkelola yang memantau repositori yang terhubung
* [Keamanan Claude Code](/docs/id/security): bagaimana Claude Code mendekati kepercayaan, izin, dan penjaga
* [Instal dan kelola plugins](/docs/id/plugins/install): temukan dan instal plugin lainnya dari marketplace resmi
