> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Output styles

> Ubah peran, nada, dan format respons Claude Code dengan gaya output bawaan seperti Concise atau Explanatory, atau tulis gaya kustom Anda sendiri.

Output style adalah serangkaian instruksi yang menetapkan peran, nada, dan format respons Claude untuk setiap respons dalam sesi. Claude Code mencakup empat gaya bawaan selain defaultnya, dan Anda dapat menulis gaya Anda sendiri.

Gunakan output style untuk mengubah cara Claude merespons dan bekerja dengan Anda selama seluruh sesi, sehingga Anda tidak perlu mengulangi permintaan di setiap prompt. Misalnya, gaya bawaan dapat membuat respons lebih pendek, menambahkan penjelasan setiap perubahan, atau membuat Claude mulai bekerja tanpa mengajukan pertanyaan rutin. Gaya kustom juga dapat mengubah Claude menjadi sesuatu selain insinyur perangkat lunak, seperti asisten penulisan atau analis data.

* Untuk menggunakan gaya bawaan, pilih salah satu dari [gaya output bawaan](#built-in-output-styles) dan [beralih ke gaya tersebut](#change-your-output-style).
* Untuk menulis instruksi Anda sendiri, [buat output style kustom](#create-a-custom-output-style).

<Note>
  Output style memberikan Claude instruksi untuk diikuti. Ini tidak menjamin bahwa sesuatu selalu terjadi atau tidak pernah terjadi. Beberapa kebutuhan sesuai dengan fitur yang berbeda:

  * Untuk apa yang harus Claude ketahui tentang proyek Anda, gunakan [CLAUDE.md](/docs/id/memory).
  * Untuk sesuatu yang harus terjadi setiap kali, seperti pemformatan setelah setiap edit atau memblokir perintah, gunakan [hook](/docs/id/hooks-guide).
  * Untuk skills, subagents, dan opsi lainnya, lihat [Pilih antara output style dan fitur lainnya](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Gaya output bawaan
</h2>

Claude Code dimulai dalam gaya [**Default**](#default), instruksi standarnya untuk menyelesaikan tugas-tugas rekayasa perangkat lunak. Masing-masing dari empat gaya bawaan lainnya mempertahankan instruksi tersebut dan menambahkan instruksinya sendiri.

Tabel ini menunjukkan apa yang setiap gaya ubah tentang sesi dan kapan cocok digunakan:

| Gaya                        | Apa yang berubah                                                                                         | Gunakan ketika                                                                                         |
| :-------------------------- | :------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| [Proactive](#proactive)     | Claude mulai bekerja segera dan membuat asumsi yang masuk akal daripada bertanya tentang keputusan rutin | Anda ingin Claude terus bekerja melalui keputusan rutin, dan Anda akan mengubah arah jika asumsi salah |
| [Concise](#concise)         | Respons dimulai dengan hasil dan menghilangkan pembukaan, narasi, dan rekap                              | Respons default lebih panjang dari yang Anda inginkan                                                  |
| [Explanatory](#explanatory) | Claude menambahkan blok `Insight` pendek yang menjelaskan pilihan di balik kode yang ditulisnya          | Anda sedang mengenal codebase atau menginginkan penalaran bersama dengan perubahan                     |
| [Learning](#learning)       | Claude menjelaskan pilihannya dan meninggalkan potongan kode kecil untuk Anda tulis sendiri              | Anda menginginkan praktik coding langsung sambil tugas masih selesai                                   |

<h3 id="default">
  Default
</h3>

Default berarti tidak ada gaya output yang dipilih. Claude Code tidak menambahkan instruksi gaya, dan Claude bekerja dari prompt sistem standar Claude Code, yang ditulis untuk tugas-tugas rekayasa perangkat lunak.

`default` muncul dalam daftar `/output-style` bersama dengan gaya lainnya, jadi Anda [memilihnya dengan cara yang sama](#change-your-output-style).

<h3 id="proactive">
  Proactive
</h3>

Dalam gaya Proactive, Claude mulai mengimplementasikan segera setelah Anda mengirim tugas. Ini membuat asumsi yang masuk akal tentang keputusan rutin daripada berhenti untuk bertanya, dan tidak beralih ke plan mode kecuali Anda meminta rencana. Anda dapat mengalihkannya kapan saja.

Instruksi gaya juga memberi tahu Claude untuk memeriksa dengan Anda dalam percakapan sebelum tindakan yang menghapus data atau mengubah sistem bersama atau produksi. Pemeriksaan itu adalah instruksi yang Claude ikuti dan terpisah dari prompt izin.

Beralih ke gaya Proactive tidak mengubah [mode izin](/docs/id/permission-modes) Anda. Mode izin Anda masih menentukan panggilan alat mana yang berjalan tanpa meminta Anda, jadi prompt izin muncul dengan cara yang sama seperti sebelum Anda beralih.

<h3 id="concise">
  Concise
</h3>

Dalam gaya Concise, kalimat pertama respons menyatakan apa yang terjadi atau apa jawabannya. Claude menghilangkan pembukaan, narasi langkah demi langkah, dan rekap penutup, dan menjawab pertanyaan sederhana dalam satu hingga tiga kalimat. Ini melakukan pekerjaan rekayasa sethoroughly seperti dalam gaya Default. Memerlukan Claude Code v2.1.237 atau lebih baru.

Claude masih menulis dengan panjang penuh dalam kasus-kasus ini:

* **Apa pun yang Anda minta**: ketika Anda meminta penjelasan atau detail lebih lanjut, Claude menjawab secara lengkap.
* **Apa pun yang Anda butuhkan untuk bertindak dengan aman**: laporan kesalahan, output tes yang gagal, peringatan keamanan, dan konfirmasi untuk tindakan destruktif mempertahankan konten lengkap mereka.

<h3 id="explanatory">
  Explanatory
</h3>

Dalam gaya Explanatory, Claude melakukan tugas dengan cara yang sama seperti dalam gaya Default dan menambahkan penjelasan singkat tentang mengapa ia membuat pilihan yang ia buat. Setiap penjelasan muncul dalam percakapan, sebelum atau sesudah kode yang terkait, dalam blok berlabel `Insight`. Penjelasan tidak ditulis ke dalam file Anda sebagai komentar.

Blok `Insight` membawa dua atau tiga poin tentang codebase Anda atau kode yang Claude tulis, seperti yang ini setelah menambahkan endpoint API:

```text theme={null}
★ Insight ─────────────────────────────────────
- Setiap rute di repo ini melewati wrapper withAuth, jadi endpoint baru mendapatkan pemeriksaan sesi tanpa middleware-nya sendiri.
- Batas laju diatur per rute dalam limits.ts, itulah mengapa perubahan ini menambahkan entri di sana daripada default global.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

Dalam gaya Learning, Claude menambahkan blok `Insight` yang sama seperti [gaya Explanatory](#explanatory) dan juga meminta Anda untuk menulis beberapa kode. Claude menangani implementasi rutin itu sendiri. Ketika mencapai bagian dengan keputusan desain nyata, seperti penanganan kesalahan, struktur data, atau logika bisnis dengan lebih dari satu pendekatan yang valid, ia meninggalkan beberapa baris untuk Anda.

Claude menandai tempat dengan komentar `TODO(human)` dalam file, kemudian mengirim permintaan yang mengatakan apa yang sudah dibangun, apa yang harus ditulis, dan apa yang harus dipertimbangkan:

```text theme={null}
● Learn by Doing

Context: Formulir upload sudah ada dan memanggil validateFile() sebelum menerima file. Pemeriksaan ukuran dan tipe berfungsi untuk gambar, tetapi pernyataan switch tidak memiliki penanganan untuk dokumen belum.

Your Task: Dalam upload.js, implementasikan cabang kasus "document" di dalam validateFile(). Cari TODO(human).

Guidance: Tentukan batas ukuran untuk dokumen dan apakah ekstensi file harus cocok dengan tipe MIME. Kembalikan {valid: boolean, error?: string}.
```

Claude kemudian berhenti dan menunggu. Tulis kode Anda di komentar `TODO(human)` dan beri tahu Claude ketika Anda selesai. Claude merespons dengan satu `Insight` tentang kode Anda dan melanjutkan tugas.

<h2 id="change-your-output-style">
  Ubah gaya output Anda
</h2>

Pilih gaya dengan perintah, menu, atau file settings. Perintah dan kedua menu menyimpan pilihan Anda ke `.claude/settings.local.json` di [tingkat proyek lokal](/docs/id/settings).

* **Perintah `/output-style`**: jalankan `/output-style <style>` untuk beralih, misalnya `/output-style concise`. Tanpa argumen, perintah mencantumkan gaya yang dapat Anda pilih dan menandai yang saat ini.

  Perintah ini juga berfungsi dalam [mode non-interaktif](/docs/id/headless) dan sesi Agent SDK, serta dari aplikasi mobile atau web melalui [Remote Control](/docs/id/remote-control#limitations), di mana Anda dapat mencantumkan dan memilih hanya [gaya bawaan](#built-in-output-styles). Memerlukan Claude Code v2.1.269 atau lebih baru.
* **Menu Terminal**: jalankan `/config` dan pilih **Output style** untuk memilih gaya dari menu.
* **Ekstensi VS Code**: buka [menu perintah](/docs/id/vs-code#use-the-prompt-box) dengan `/` dan pilih **Output styles** untuk memilih gaya, termasuk gaya kustom Anda. Memerlukan Claude Code v2.1.257 atau lebih baru.
* **Aplikasi Desktop**: atur field `outputStyle` dalam file settings, misalnya `.claude/settings.local.json`, file yang ditulis menu terminal. Ketika Anda menjalankan `/config` di sana, Claude Code [membuka **Settings > Claude Code**](/docs/id/desktop#what%E2%80%99s-not-available-in-desktop) daripada menu.

Untuk menetapkan gaya tanpa menu, edit field `outputStyle` secara langsung dalam file settings:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Nilainya peka huruf besar-kecil, jadi tulis nama bawaan sebagai `Proactive`, `Concise`, `Explanatory`, dan `Learning`. Nilai yang tidak cocok dengan nama gaya secara tepat, seperti `explanatory`, memberikan Anda gaya Default. Perintah `/output-style` mengabaikan huruf besar-kecil.

Untuk menjadikan gaya Anda default di seluruh proyek, atur `outputStyle` dalam `~/.claude/settings.json`. File settings proyek sendiri [mengambil prioritas](/docs/id/settings#settings-precedence) atas nilai tersebut.

Ketika Anda beralih gaya di tengah sesi, Claude menggunakan gaya baru mulai dari pesan Anda berikutnya. Untuk biaya pesan pertama itu dalam prompt caching, lihat [Mengubah gaya output](/docs/id/prompt-caching#changing-output-style). Sebelum v2.1.251, gaya baru diterapkan hanya setelah Anda menjalankan `/clear` atau memulai sesi baru.

<h2 id="create-a-custom-output-style">
  Buat custom output style
</h2>

Custom output style adalah file Markdown: frontmatter untuk metadata, kemudian instruksi untuk Claude.

Di VS Code extension, Anda juga dapat membuat file dari [menu **Output styles**](/docs/id/vs-code#use-the-prompt-box) daripada menulisnya dengan tangan. Ini memerlukan Claude Code v2.1.261 atau lebih baru.

<Steps>
  <Step title="Buat file Markdown">
    Simpan di salah satu dari tiga tingkat. Nama file menjadi nama style kecuali Anda menetapkan `name` dalam frontmatter.

    * User: `~/.claude/output-styles`
    * Project: `.claude/output-styles`
    * Managed policy: `.claude/output-styles` di dalam [direktori pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms)

    Project output styles dimuat dari setiap `.claude/output-styles/` antara direktori kerja dan akar repositori. Ketika lebih dari satu direktori bersarang ini mendefinisikan style dengan nama yang sama, Claude Code menggunakan yang paling dekat dengan direktori kerja.
  </Step>

  <Step title="Tambahkan frontmatter dan instruksi">
    Putuskan apakah akan mempertahankan instruksi rekayasa perangkat lunak Claude Code. Atur `keep-coding-instructions: true` jika Anda mengubah cara Claude berkomunikasi tetapi masih ingin coding dengan cara yang sama. Tinggalkan jika Claude tidak akan melakukan rekayasa perangkat lunak.

    Contoh ini memimpin setiap penjelasan dengan diagram sambil mempertahankan perilaku coding Claude:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Beralih ke style Anda">
    Jalankan `/output-style <style>` di terminal, atau jalankan `/config` dan pilih style Anda di bawah **Output style**. Claude menggunakan style baru mulai dari pesan Anda berikutnya. Di terminal, Claude Code membaca file style saat dimulai, jadi jika Anda membuat atau mengedit satu selama sesi yang sedang berjalan, restart Claude Code untuk mengambil perubahan tersebut.
  </Step>
</Steps>

[Plugins](/docs/id/plugins/manifest-reference) juga dapat mengirimkan output styles dalam direktori `output-styles/`.

<h3 id="frontmatter">
  Referensi frontmatter
</h3>

Konfigurasikan output style dengan [frontmatter](/docs/id/glossary#frontmatter) YAML antara penanda `---` di bagian atas file. Semua field bersifat opsional, dan nama field menggunakan kata-kata huruf kecil yang dipisahkan oleh tanda hubung. Field yang salah eja diabaikan tanpa kesalahan. Jika YAML tidak dapat diuraikan, style masih dimuat dengan nama filenya tanpa field yang ditetapkan; jalankan `claude --debug` untuk melihat kesalahan penguraian.

| Field                      | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :--------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Tidak      | Nama output style, ditampilkan dalam picker `/config`. Default: nama file                                                                                                                                                                                                                                                           |
| `description`              | Tidak      | Deskripsi output style, ditampilkan dalam picker `/config`                                                                                                                                                                                                                                                                          |
| `keep-coding-instructions` | Tidak      | Atur ke `true` untuk mempertahankan instruksi rekayasa perangkat lunak bawaan Claude Code bersama style Anda. Default: `false`                                                                                                                                                                                                      |
| `force-for-plugin`         | Tidak      | Output styles plugin saja. Atur ke `true` untuk menerapkan style ini secara otomatis kapan pun plugin diaktifkan, tanpa memerlukan pengguna untuk memilihnya. Mengesampingkan pengaturan `outputStyle` pengguna. Jika beberapa plugin yang diaktifkan menetapkan ini, Claude Code menggunakan yang pertama dimuat. Default: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Pilih antara output style dan fitur lainnya
</h2>

Output style berlaku untuk setiap respons dalam sesi. Ini adalah instruksi yang diikuti Claude, jadi tidak ada yang memberlakukannya. Ketika apa yang Anda inginkan lebih sempit daripada setiap respons, atau harus terjadi tanpa gagal, fitur lain lebih cocok.

Tabel ini mencocokkan apa yang Anda inginkan dengan fitur yang melakukannya:

| Anda ingin                                                                                                               | Gunakan                                                           | Mengapa cocok                                                                                                         |
| :----------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| Setiap respons dalam suara, panjang, atau format tertentu, atau Claude dalam peran yang berbeda                          | Output style                                                      | Ini berlaku untuk seluruh sesi, dan Anda beralih style dengan satu perintah                                           |
| Claude mengetahui konvensi, perintah, dan struktur proyek Anda                                                           | [CLAUDE.md](/docs/id/memory)                                           | Ini menyimpan apa yang harus diketahui Claude tentang codebase, dan tetap dimuat dengan style apa pun yang Anda pilih |
| Instruksi untuk satu jenis tugas, seperti checklist rilis atau prosedur review                                           | A [skill](/docs/id/skills)                                             | Claude memmuatnya hanya ketika Anda menginvokasinya atau tugas cocok, jadi tidak membentuk respons yang tidak terkait |
| Sesuatu yang harus terjadi setiap saat tanpa terkecuali, seperti pemformatan setelah setiap edit atau memblokir perintah | A [hook](/docs/id/hooks-guide)                                         | Claude Code menjalankan hook itu sendiri pada acara lifecycle, jadi tidak bergantung pada Claude mengikuti instruksi  |
| Pembantu dengan instruksi, model, dan tools sendiri untuk tugas yang terfokus                                            | A [subagent](/docs/id/sub-agents)                                      | Ini berjalan dalam konteks terpisah dengan system prompt sendiri dan mengembalikan ringkasan ke percakapan Anda       |
| Penambahan pada instruksi Claude yang Anda berikan saat memulai Claude Code                                              | [`--append-system-prompt`](/docs/id/cli-reference#system-prompt-flags) | Ini menambahkan ke system prompt tanpa menghapus apa pun                                                              |

Fitur-fitur ini dapat digabungkan. Misalnya, Anda dapat menggunakan CLAUDE.md untuk apa yang harus diketahui Claude, output style untuk cara meresponnya, dan hook untuk apa pun yang harus dijamin. [Perluas Claude Code](/docs/id/features-overview) membandingkan fitur ekstensi lainnya.

<h2 id="how-output-styles-work">
  Cara kerja output styles
</h2>

Output style mengubah instruksi yang diberikan Claude Code kepada Claude.

* Claude Code mengirimkan instruksi style aktif dengan setiap permintaan.
* Output styles kustom menghilangkan instruksi rekayasa perangkat lunak bawaan Claude Code, seperti cara membatasi perubahan, menulis komentar, dan memverifikasi pekerjaan, kecuali `keep-coding-instructions` diatur ke `true`.

Output styles berlaku untuk percakapan utama dan untuk [fork](/docs/id/sub-agents#fork-the-current-conversation), yang mewarisi percakapan lengkap dan system prompt induk. [Subagent lain menjalankan system prompt mereka sendiri](/docs/id/sub-agents#what-loads-at-startup), jadi styles tidak mengubah cara mereka merespons.

Penggunaan token tergantung pada style. Instruksi style menambahkan input tokens, meskipun prompt caching mengurangi biaya ini setelah permintaan pertama dalam sesi.

Style Explanatory dan Learning bawaan menghasilkan respons yang lebih panjang daripada Default secara desain, yang meningkatkan output tokens. Style Concise melakukan sebaliknya dengan menginstruksikan Claude untuk menjaga respons tetap singkat secara default. Untuk styles kustom, penggunaan output token tergantung pada apa yang instruksi Anda katakan kepada Claude untuk diproduksi.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Settings](/docs/id/settings): di mana field `outputStyle` berada dan cara kerja precedence settings
* [Permission modes](/docs/id/permission-modes): bagaimana style Proactive dibandingkan dengan mode otomatis
* [Plugins](/docs/id/plugins/overview): paket dan distribusikan output styles bersama skills, hooks, dan agents
* [Debug your configuration](/docs/id/debug-your-config): diagnosa mengapa output style tidak berlaku
