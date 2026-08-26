> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Output styles

> Sesuaikan Claude Code untuk penggunaan di luar rekayasa perangkat lunak

Output styles mengubah cara Claude merespons, bukan apa yang Claude ketahui. Mereka memodifikasi system prompt untuk menetapkan peran, nada, dan format output. Gunakan satu ketika Anda terus-menerus meminta kembali untuk suara atau format yang sama setiap giliran, atau ketika Anda ingin Claude bertindak sebagai sesuatu selain seorang insinyur perangkat lunak.

Custom output style menambahkan instruksi Anda ke system prompt dan memungkinkan Anda memilih apakah akan mempertahankan instruksi rekayasa perangkat lunak bawaan Claude Code. Pertahankan mereka ketika Anda mengubah cara Claude berkomunikasi tetapi masih coding, seperti selalu menjawab dengan diagram. Tinggalkan mereka ketika Claude tidak melakukan rekayasa perangkat lunak sama sekali, seperti asisten penulisan atau analis data.

Untuk instruksi tentang proyek, konvensi, atau codebase Anda, gunakan [CLAUDE.md](/docs/id/memory) sebagai gantinya.

<h2 id="built-in-output-styles">
  Gaya output bawaan
</h2>

Gaya output **Default** Claude Code adalah system prompt yang ada, dirancang untuk membantu Anda menyelesaikan tugas-tugas rekayasa perangkat lunak secara efisien.

Ada tiga gaya output bawaan tambahan:

* **Proactive**: Claude dieksekusi segera, membuat asumsi yang masuk akal alih-alih berhenti untuk keputusan rutin, dan lebih memilih tindakan daripada perencanaan. Ini adalah panduan eksekusi otonom yang lebih kuat daripada yang diterapkan [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), dan ini berfungsi tanpa mengubah mode izin Anda, jadi Anda masih melihat prompt izin sebelum alat dijalankan.

* **Explanatory**: Menyediakan "Insights" edukatif di antara membantu Anda menyelesaikan tugas-tugas rekayasa perangkat lunak. Membantu Anda memahami pilihan implementasi dan pola codebase.

* **Learning**: Mode kolaboratif belajar-dengan-melakukan di mana Claude tidak hanya akan berbagi "Insights" saat coding, tetapi juga meminta Anda untuk berkontribusi dengan potongan kode kecil dan strategis sendiri. Claude Code akan menambahkan penanda `TODO(human)` dalam kode Anda untuk Anda implementasikan.

<h2 id="change-your-output-style">
  Ubah gaya output Anda
</h2>

Jalankan `/config` dan pilih **Output style** untuk memilih gaya dari menu. Pilihan Anda disimpan ke `.claude/settings.local.json` di [tingkat proyek lokal](/docs/id/settings).

<Note>Perintah standalone `/output-style` sudah tidak digunakan lagi di v2.1.73 dan dihapus di v2.1.91. Gunakan `/config` atau edit pengaturan `outputStyle` secara langsung.</Note>

Untuk menetapkan gaya tanpa menu, edit field `outputStyle` secara langsung dalam file settings:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Output style adalah bagian dari system prompt, yang dibaca Claude Code sekali saat awal sesi. Perubahan berlaku setelah `/clear` atau sesi baru. Lihat [Bagaimana Claude Code menggunakan prompt caching](/docs/id/prompt-caching#changing-output-style) untuk mengetahui apa yang dilakukan perubahan output style terhadap cache.

<h2 id="create-a-custom-output-style">
  Buat custom output style
</h2>

Custom output style adalah file Markdown: frontmatter untuk metadata, kemudian instruksi untuk ditambahkan ke system prompt.

<Steps>
  <Step title="Buat file Markdown">
    Simpan di salah satu dari tiga tingkat. Nama file menjadi nama style kecuali Anda menetapkan `name` dalam frontmatter.

    * User: `~/.claude/output-styles`
    * Project: `.claude/output-styles`
    * Managed policy: `.claude/output-styles` di dalam [direktori pengaturan terkelola](/docs/id/settings#settings-files)

    Project output styles dimuat dari setiap `.claude/output-styles/` antara direktori kerja dan akar repositori. Mulai dari v2.1.178, ketika lebih dari satu direktori bersarang ini mendefinisikan style dengan nama yang sama, Claude Code menggunakan yang paling dekat dengan direktori kerja.
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
    Jalankan `/config` dan pilih style Anda di bawah **Output style**. Ini berlaku setelah `/clear` atau saat Anda memulai sesi berikutnya.
  </Step>
</Steps>

[Plugins](/docs/id/plugins-reference) juga dapat mengirimkan output styles dalam direktori `output-styles/`.

<h3 id="frontmatter">
  Frontmatter
</h3>

File output style mendukung field frontmatter ini:

| Frontmatter                | Tujuan                                                                                                                                                                                                                                                                                       | Default                 |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `name`                     | Nama output style, jika bukan nama file                                                                                                                                                                                                                                                      | Mewarisi dari nama file |
| `description`              | Deskripsi output style, ditampilkan dalam picker `/config`                                                                                                                                                                                                                                   | Tidak ada               |
| `keep-coding-instructions` | Pertahankan instruksi rekayasa perangkat lunak bawaan Claude Code                                                                                                                                                                                                                            | `false`                 |
| `force-for-plugin`         | Plugin output styles hanya: terapkan style ini secara otomatis kapan pun plugin diaktifkan, tanpa memerlukan pengguna untuk memilihnya. Mengesampingkan pengaturan `outputStyle` pengguna. Jika beberapa plugin yang diaktifkan menetapkan ini, Claude Code menggunakan yang pertama dimuat. | `false`                 |

<h2 id="how-output-styles-work">
  Cara kerja output styles
</h2>

Output styles secara langsung memodifikasi system prompt Claude Code.

* Semua output styles memiliki instruksi kustom mereka sendiri yang ditambahkan ke akhir system prompt.
* Semua output styles memicu pengingat bagi Claude untuk mematuhi instruksi output style selama percakapan.
* Custom output styles menghilangkan instruksi rekayasa perangkat lunak bawaan Claude Code, seperti cara membatasi perubahan, menulis komentar, dan memverifikasi pekerjaan, kecuali `keep-coding-instructions` diatur ke `true`.

Penggunaan token tergantung pada style. Menambahkan instruksi ke system prompt meningkatkan input tokens, meskipun prompt caching mengurangi biaya ini setelah permintaan pertama dalam sesi. Built-in Explanatory dan Learning styles menghasilkan respons yang lebih panjang daripada Default secara desain, yang meningkatkan output tokens. Untuk custom styles, penggunaan output tokens tergantung pada apa yang instruksi Anda katakan kepada Claude untuk diproduksi.

<h2 id="comparisons-to-related-features">
  Perbandingan dengan fitur terkait
</h2>

Beberapa fitur menyesuaikan perilaku Claude Code. Output styles memodifikasi system prompt secara langsung dan berlaku untuk setiap respons. Yang lain menambahkan instruksi tanpa mengubah system prompt default, atau membatasi mereka ke tugas tertentu.

| Fitur                    | Cara kerjanya                                                           | Gunakan ketika                                                                         |
| :----------------------- | :---------------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| Output styles            | Memodifikasi system prompt                                              | Anda menginginkan peran, nada, atau format respons default yang berbeda setiap giliran |
| [CLAUDE.md](/docs/id/memory)  | Menambahkan pesan pengguna setelah system prompt                        | Claude harus selalu mengetahui konvensi proyek dan konteks codebase Anda               |
| `--append-system-prompt` | Menambahkan ke system prompt tanpa menghapus apa pun                    | Anda menginginkan penambahan satu kali untuk satu invokasi                             |
| [Agents](/docs/id/sub-agents) | Menjalankan subagent dengan system prompt, model, dan tools-nya sendiri | Anda menginginkan helper dengan cakupan terpisah untuk tugas yang terfokus             |
| [Skills](/docs/id/skills)     | Memuat instruksi khusus tugas saat dipanggil atau relevan               | Anda memiliki alur kerja yang dapat digunakan kembali                                  |

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Settings](/docs/id/settings): di mana field `outputStyle` berada dan cara kerja precedence settings
* [Permission modes](/docs/id/permission-modes): bagaimana style Proactive dibandingkan dengan mode otomatis
* [Plugins](/docs/id/plugins): paket dan distribusikan output styles bersama skills, hooks, dan agents
* [Debug your configuration](/docs/id/debug-your-config): diagnosa mengapa output style tidak berlaku
