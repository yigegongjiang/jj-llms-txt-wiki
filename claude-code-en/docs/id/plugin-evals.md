> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Uji plugin dengan evals

> Tulis kasus eval untuk plugin Claude Code Anda, jalankan dengan claude plugin eval, nilai hasilnya, bandingkan dengan baseline tanpa plugin, dan gating CI pada skor.

Perintah shell `claude plugin eval` menjalankan [plugin](/docs/id/plugins/overview) Anda terhadap serangkaian kasus uji dan menilai hasilnya. Setiap kasus adalah prompt realistis ditambah satu atau lebih grader. Grader adalah pemeriksaan lulus/gagal pada apa yang dihasilkan Claude, seperti regex atas balasan, apakah alat tertentu dipanggil, atau rubrik yang dinilai model kedua terhadap balasan.

Anda tidak harus menulis suite dengan tangan; `claude plugin eval init` menanyakan Anda tentang plugin Anda, mengusulkan kasus dan grader, mencobanya, dan menulis file. Anda juga dapat meminta Claude melakukan hal yang sama dari sesi yang sudah Anda buka.

Gunakan evals untuk:

* Mengukur seberapa andal plugin Anda mengarahkan Claude untuk menghasilkan hasil yang benar
* Menangkap regresi saat Anda mengubah plugin atau model baru dirilis
* Melihat apa yang dikontribusikan plugin dibandingkan tanpa plugin

Halaman ini untuk penulis plugin dan skill yang memiliki plugin yang berfungsi dan ingin menguji perilakunya, dan untuk tim yang gating perubahan plugin di CI. Format kasusnya terpisah dari file `evals/evals.json` yang digunakan [plugin skill-creator](/docs/id/skills#run-evals-with-skill-creator). Untuk membuat plugin, lihat [Buat plugin](/docs/id/plugins/create); untuk memeriksa file plugin untuk kesalahan sintaks dan skema daripada perilakunya, gunakan [`claude plugin validate`](/docs/id/plugins/cli-reference#plugin-validate).

<Note>
  Setiap eval run dan setiap judge grader adalah panggilan model nyata pada akun Anda, dihitung terhadap penggunaan rencana Anda atau tagihan API Anda, jadi periksa [persyaratan](#requirements) terlebih dahulu. Kemudian [buat suite eval pertama Anda](#create-your-first-eval-suite), atau buka [Jalankan evals di CI](#run-evals-in-ci) jika Anda sudah memilikinya.
</Note>

<h2 id="requirements">
  Persyaratan
</h2>

Untuk menjalankan plugin evals Anda memerlukan:

* Claude Code v2.1.269 atau lebih baru. Jalankan `claude --version` untuk memeriksa dan `claude update` untuk upgrade.
* Direktori plugin dengan manifest `plugin.json` atau `.claude-plugin/plugin.json`, atau [plugin direktori skills](/docs/id/plugins/loading#plugins-shared-through-a-repository).
* Autentikasi dan penyedia model yang sama dengan sesi Claude Code normal Anda. Eval runs, grader yang dinilai judge, dan `claude plugin eval init` memanggil model dengan kredensial Anda, jadi mereka dihitung terhadap batas penggunaan rencana Anda atau tagihan API Anda. Ketika perintah melaporkan biaya, angka tersebut adalah [perkiraan harga daftar](/docs/id/costs) dari panggilan tersebut.

<h2 id="how-an-eval-run-works">
  Cara kerja eval run
</h2>

Suite eval hidup di direktori bernama `evals/` di dalam plugin Anda, diatur seperti yang ditunjukkan [Tulis dan perbaiki kasus](#write-and-refine-cases). Setiap kasus adalah subdirektorinya sendiri dengan [prompt](#set-run-limits-and-tools-in-prompt-md) dan satu atau lebih [grader](#grade-the-result). Prompt adalah sesuatu yang mungkin diketik oleh orang yang menggunakan plugin Anda, seperti permintaan yang seharusnya ditangani salah satu skillnya.

<h3 id="what-happens-in-a-run">
  Apa yang terjadi dalam run
</h3>

Untuk setiap run kasus, Claude Code memulai sesi [terisolasi](#how-runs-are-isolated) [non-interaktif](/docs/id/headless) yang segar dengan hanya plugin Anda yang dimuat, mengirim prompt, dan membiarkan Claude bekerja sampai selesai atau mencapai batas turn atau waktu kasus. Setiap grader kemudian memeriksa balasan akhir, transkrip, atau file yang dibuat Claude, dan lulus atau gagal.

<h3 id="how-a-case-is-scored">
  Cara kasus dinilai
</h3>

Satu run dari agen non-deterministik memberi tahu Anda sedikit, jadi setiap kasus berjalan tiga kali secara default. Skor run adalah fraksi gradernya yang lulus, tertimbang jika Anda menetapkan bobot, dan skor kasus adalah rata-rata di seluruh runnya. Kasus lulus ketika skornya memenuhi [`--threshold`](#command-options), `1.0` secara default.

Dalam panggilan model, suite membuat kira-kira kasus × runs agent runs dengan plugin dan sebanyak lagi untuk [baseline tanpa plugin](#the-no-plugin-baseline), ditambah tiga panggilan judge pendek per grader `llm` atau `baseline` per run.

<h3 id="the-no-plugin-baseline">
  Baseline tanpa plugin
</h3>

Skor tinggi sendiri tidak memberi tahu Anda plugin membantu, karena Claude mungkin melakukan hal yang sama tanpanya. Untuk memisahkan keduanya, setiap run kasus diulang tanpa plugin yang dimuat secara default, dan Anda mendapatkan dua skor, `WITH` dan `W/OUT`. Perbedaan mereka, `Δ`, adalah apa yang dikontribusikan plugin. Jika kasus mencetak 1.0 baik dengan maupun tanpa plugin, plugin bukan yang membuatnya lulus.

Dua set run disebut with-arm dan without-arm; [Bandingkan dengan baseline tanpa plugin](#compare-against-a-no-plugin-baseline) mencakup cara grader dinilai di seluruh mereka dan cara mematikan baseline.

<h2 id="create-your-first-eval-suite">
  Buat suite eval pertama Anda
</h2>

Panduan ini menulis satu kasus untuk plugin Anda sendiri, menjalankannya, dan membaca hasilnya. Sebelum Anda mulai, pastikan Anda memiliki:

* Claude Code v2.1.269 atau lebih baru dan [persyaratan](#requirements) lainnya
* Terminal terbuka di direktori root plugin Anda, yang berisi `plugin.json` atau `.claude-plugin/plugin.json`
* Satu skill di plugin yang ingin Anda uji, dan permintaan yang akan diketik pengguna yang seharusnya memicunya

<Steps>
  <Step title="Buat kasusnya">
    Dari root plugin, jalankan:

    ```bash theme={null}
    claude plugin eval init
    ```

    Jika Claude Code belum mempercayai direktori ini, pertama-tama menanyakan `Trust this plugin directory?`; jawab `y`. Sesi Claude Code interaktif kemudian terbuka. Claude membaca plugin Anda dan menanyakan apa hasil yang baik, mengusulkan prompt yang seharusnya dan tidak seharusnya memicu plugin, merancang grader untuk masing-masing, menjalankan pilot mereka sekali untuk memeriksa perilakunya, dan menulis satu direktori kasus per prompt di bawah `evals/`, masing-masing dinamai setelah promptnya. Ketika Claude memberi tahu Anda suite siap, keluar dari sesi itu dengan `/exit` atau Ctrl+D untuk kembali ke shell Anda.

    Jika Anda sudah memiliki sesi Claude Code terbuka di root plugin, Anda dapat meminta Claude di sana untuk menjalankan `claude plugin eval init`. Claude menjalankan perintah dan kemudian menanyakan Anda pertanyaan yang sama dalam percakapan itu.

    Jika Anda lebih suka menulis kasus sendiri untuk melihat dengan tepat apa yang berisi file, ikuti [Tulis kasus dengan tangan](#write-a-case-manually) dan kembali ke sini untuk menjalankannya.
  </Step>

  <Step title="Jalankan suite">
    Kembali di shell Anda di root plugin, jalankan setiap kasus di bawah `evals/`:

    ```bash theme={null}
    claude plugin eval .
    ```

    Anda sudah mempercayai direktori ini selama langkah 1, jadi run dimulai segera. Jika Anda menulis kasus dengan tangan sebagai gantinya, run pertama menanyakan `Trust this plugin directory? [y/N]`; jawab `y`. [Apa yang dapat diakses run](#security) menjelaskan apa yang Anda setujui.

    Setiap kasus berjalan tiga kali dengan plugin Anda dan tiga kali tanpanya, jadi satu kasus adalah enam run. Baris kemajuan dicetak saat setiap run selesai, dengan skor run itu dan putusan setiap grader.
  </Step>

  <Step title="Baca ringkasannya">
    Ketika suite selesai Anda melihat tabel ringkasan, diikuti oleh tempat laporan pergi:

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` adalah skor kasus dengan plugin Anda dimuat, `W/OUT` adalah skor tanpanya, dan `Δ` positif berarti plugin menaikkan skor. `COST` adalah perkiraan harga daftar dari panggilan model, dan `NOTES` menunjukkan penjelasan grader yang gagal dengan bobot tertinggi, atau kesalahan run, dari with-arm.
  </Step>

  <Step title="Buka laporan dan ulangi">
    Buka URL `Published:`, atau jalur `Report:` ketika tidak ada baris `Published:` yang muncul, untuk melihat putusan setiap grader dan penjelasan untuk setiap run, dan untuk grader `llm` suara judge dan kutipan yang dinilainya. Baris `Published:` muncul hanya ketika akun Anda dapat [menerbitkan laporan](#html-report).

    Temuan pertama yang paling umum adalah `Δ` mendekati nol dengan grader `tool_used: Skill` kasus gagal, yang berarti Claude tidak memilih skill Anda pada frasa alami. Sesuaikan [`description`](/docs/id/skills#frontmatter-reference) skill, jalankan `claude plugin eval .` lagi, dan bandingkan.

    Untuk mengulangi satu kasus dengan murah, jalankan satu arm sekali. Satu run bising, jadi konfirmasi perubahan apa pun pada tiga run default sebelum Anda mempercayainya. Dengan satu arm tabel menunjukkan kolom `SCORE` dan `PASS%` alih-alih `WITH`, `W/OUT`, dan `Δ`:

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Ganti `<case-name>` dengan salah satu nama direktori di bawah `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Tulis dan perbaiki kasus
</h2>

Kasus yang ditulis `claude plugin eval init` adalah file biasa yang dapat Anda buka, ubah, dan tambahkan. Kasus adalah direktori di bawah direktori eval plugin yang berisi `prompt.md`, `case.yaml`, atau keduanya. Untuk mengelompokkan kasus, bersarangkan mereka di bawah direktori yang bukan kasus itu sendiri; apa pun di dalam direktori kasus, seperti `graders/` dan file fixture, milik kasus itu.

Ini adalah tata letak yang ditulis `claude plugin eval init` dan yang digunakan untuk suite baru. [Referensi suite eval](#eval-suite-reference) memiliki pohon lengkap, termasuk mock dan hasil:

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Tulis kasus dengan tangan
</h3>

Memiliki Claude menulis kasus dengan `claude plugin eval init` adalah jalur yang direkomendasikan. Untuk menulis satu sendiri, mulai dari template kosong. Perintah berikut menulis kasus bernama `first-case` dengan `prompt.md` placeholder dan satu grader placeholder, dan tidak menjalankan apa pun:

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

Di `prompt.md` Anda menulis pesan yang diterima Claude di setiap run, dan menetapkan batas run dan alat yang dapat digunakan kasus di frontmatter. Buka `evals/first-case/prompt.md` dan ganti body placeholder dengan permintaan yang salah satu skill Anda harus tangani, diucapkan dengan cara pengguna akan mengetiknya daripada menamakan skill. Contoh ini untuk skill yang menyusun pesan commit; gunakan permintaan Anda sendiri:

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Setiap run dimulai di direktori kerja kosong, jadi letakkan apa pun yang dibutuhkan tugas di prompt itu sendiri, atau [atur workspace](#add-setup-or-history-with-case-yaml) terlebih dahulu.

[Daftar lengkap field frontmatter](#prompt-md-fields) mencakup model, timeout, tag, dan variabel lingkungan.

Setiap file di bawah `graders/` adalah satu pemeriksaan yang diterapkan setelah run. Buka `evals/first-case/graders/criteria.md` dan ganti placeholder dengan rubrik untuk model judge, ditulis sebagai kondisi PASS dan FAIL konkret:

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Kemudian tambahkan grader kedua yang memeriksa apakah skill Anda yang menghasilkan jawaban. Buat `evals/first-case/graders/skill-fired.md`, ganti `your-skill-name` dengan nama direktori skill di bawah `skills/`, yang merupakan nama yang Claude panggil:

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Ini lulus ketika Claude menginvokasi skill itu setidaknya sekali selama run, termasuk dengan bentuk `plugin-name:skill-name` yang diberi namespace.

[Tipe grader](#grader-types) mencantumkan pemeriksaan lain yang tersedia, seperti mencocokkan regex atau mengkonfirmasi file dibuat.

Dengan kedua file disimpan, jalankan kasus dengan cara [quickstart](#create-your-first-eval-suite) lakukan, dengan `claude plugin eval .` dari root plugin.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Atur batas run dan alat di prompt.md
</h3>

Atur `max_turns`, `timeout_seconds`, `model`, `tags`, dan `allowed_tools` kasus di frontmatter `prompt.md`; referensi [prompt.md frontmatter](#prompt-md-fields) mencantumkan setiap field dan defaultnya.

Claude menerima body persis seperti yang Anda tulis. Penyebutan `@path` di dalamnya tidak diperluas menjadi lampiran file, jadi jika Claude perlu membaca file, berikan alat untuk itu di `allowed_tools`.

<h3 id="grade-the-result">
  Pilih dan timbang grader
</h3>

Frontmatter grader menetapkan `type`-nya, dan secara opsional `weight` yang membuatnya dihitung untuk lebih banyak skor run dan [`arm`](#compare-against-a-no-plugin-baseline) yang mengontrol cara dinilai terhadap baseline. Dari enam tipe, `regex`, `tool_used`, `tool_order`, dan `file_exists` dihitung dari transkrip dan file dan tidak ada biaya, sementara `llm` dan `baseline` memanggil model judge dan menambah biaya run.

Tidak ada grader kode kustom.

[Tipe grader](#grader-types) mencantumkan opsi setiap tipe dan kondisi lulus, dan [apa yang dapat dilihat grader](#what-a-grader-can-look-at) mencantumkan nilai yang diterima `target` dan `focus`.

Judge untuk grader `llm` dan `baseline` adalah model cepat kecil secara default. Lewati `--judge-model sonnet` atau ID model lengkap untuk menggunakan yang lebih kuat untuk rubrik bernuansa.

<h4 id="choose-graders-that-give-a-stable-signal">
  Pilih grader yang memberikan sinyal stabil
</h4>

Grader `llm` meminta model untuk putusan, jadi jawabannya dapat berbeda antar run, dan berbeda lebih banyak teks yang harus dibacanya. Kebiasaan ini menjaga skor suite cukup stabil untuk dipercaya:

* Untuk output panjang seperti file yang dihasilkan, nilainya dengan grader `regex` atas konten file, yang memeriksa seluruh file dengan cara yang sama setiap kali. Simpan grader `llm` untuk output pendek, dengan rubrik ditulis sebagai kondisi PASS dan FAIL konkret.
* Berikan setiap kasus satu grader pada hasil, seperti pesan akhir atau file yang dihasilkan, dan satu tentang cara Claude sampai di sana, seperti `tool_used` atau `tool_order`. Bersama-sama mereka memberi tahu Anda baik jawaban benar dan apakah plugin Anda menghasilkannya.
* Jika grader `tool_used: Skill` kasus lulus tetapi `Δ` negatif, curigai judge sebelum plugin. Model judge kecil dapat menandai jawaban yang benar salah karena diformat berbeda dari apa yang dijelaskan rubrik. Jalankan ulang dengan `--judge-model sonnet`, dan ketatkan rubrik sehingga pemformatan tidak memutuskan putusan.
* Untuk memeriksa bahwa build atau test lulus di dalam run, minta prompt Claude untuk menjalankannya dan menulis hasil ke file, nilai file itu, dan tegaskan perintah berjalan dengan grader `tool_used` yang `input_match` menamakan perintah.

<h3 id="compare-against-a-no-plugin-baseline">
  Nilai terhadap baseline tanpa plugin
</h3>

Ketika plugin sedang diuji, setiap kasus berjalan di dua arm secara default. With-arm adalah runnya dengan plugin dimuat, dan without-arm adalah jumlah run yang sama tanpa plugin sama sekali. Ringkasan dan laporan menunjukkan kedua skor dan `Δ`, skor with-arm minus skor without-arm.

Lewati `--ablation none` untuk menjalankan hanya with-arm, yang mengurangi biaya setengahnya ketika Anda tidak memerlukan perbandingan, seperti saat mengulangi grader.

Dalam run dua-arm, beberapa grader dilaporkan dengan `scored: false`. Pemeriksaan seperti "skill dipanggil" tidak pernah dapat lulus tanpa plugin, jadi menghitungnya akan mendorong without-arm menuju nol dan menginflasi `Δ`. Untuk menjaga kedua arm dapat dibandingkan, Claude Code mengecualikan grader tersebut dari skor di kedua arm dan melaporkannya di with-arm sebagai indikator lulus/gagal saja. Itu termasuk:

* Setiap grader `tool_used` yang `tool`-nya adalah `Skill`
* Setiap grader `regex` dengan `target: mock_calls` dan setiap grader `llm` dengan `focus: mock_calls`, ketika setiap [server mock](#mock-mcp-servers) dalam kasus adalah salah satu yang dideklarasikan plugin Anda
* Grader apa pun yang Anda tandai `arm: with-only`

Tiga pengaturan mengubah pengecualian itu:

* **Setiap grader dikecualikan**: jika setiap grader dalam kasus adalah salah satu dari set yang dikecualikan, mereka dinilai secara normal sebagai gantinya, karena tidak akan ada yang tersisa untuk dinilai.
* **`arm: both`**: atur `arm: both` pada grader untuk menilainya di kedua arm terlepas, yang ingin Anda lakukan untuk pemeriksaan "harus tidak menginvokasi skill" dengan `min: 0` dan `max: 0`.
* **`--ablation none`**: di bawah `--ablation none` tidak ada yang dikecualikan, jadi suite yang sama dapat menghasilkan skor absolut yang berbeda di dua mode.

<h3 id="use-a-different-eval-directory">
  Gunakan direktori eval yang berbeda
</h3>

Jika `evals/` sudah diambil oleh alat lain, simpan suite di direktori yang berbeda. Anda dapat mencatat direktori itu di `plugin.json` plugin sehingga setiap run dan setiap kolaborator menggunakannya, atau lewati di baris perintah untuk satu run:

* **Di `plugin.json`**: tambahkan `"experimental": { "evals": "quality/evals" }`.
* **Di baris perintah**: lewati `--eval-dir quality/evals` ke `claude plugin eval` dan `claude plugin eval init`.

Jika Anda menetapkan keduanya, direktori flag digunakan. Berikan jalur relatif dari nama direktori biasa seperti `qa` atau `quality/evals`. Jalur absolut atau yang berisi `..` tidak diterima: sebagai nilai flag itu adalah kesalahan, sementara nilai manifest yang tidak dapat digunakan mencetak baris `Warning:` dan run menggunakan `evals/` sebagai gantinya. Kasus, hasil, dan output `init` semuanya pindah ke direktori itu.

<h2 id="set-up-fixtures-and-mocks">
  Atur fixture dan mock
</h2>

Kasus dapat memerlukan lebih dari sekadar prompt: file atau repositori git di workspace, percakapan sebelumnya untuk dilanjutkan, atau jawaban dari server MCP yang dibicarakan plugin Anda. Masing-masing diatur di samping kasus sehingga run tetap dapat diulang.

<h3 id="add-setup-or-history-with-case-yaml">
  Seed workspace atau percakapan
</h3>

Setiap run dimulai di workspace kosong. Ketika kasus memerlukan lebih dari prompt, tambahkan `case.yaml` di samping `prompt.md` dengan blok `context`:

* **File fixture atau repositori git**: tulis skrip Bash di direktori kasus dan namai di `context.scaffold_script`. Skrip berjalan sebagai Anda, di luar sandbox agen, dan hanya ketika Anda lewati `--scaffold`, jadi lewati flag itu hanya untuk suite yang Anda atau organisasi Anda tulis.
* **Percakapan sebelumnya untuk dilanjutkan**: simpan transkrip sebagai file `.jsonl` dan namai di `context.history_file`, dan prompt kasus menjadi turn pengguna berikutnya.
* **Direktori fixture yang dapat dibaca Claude selama run**: cantumkan di `context.add_dirs`.

`case.yaml` juga memerlukan `schema_version: "1.1"` dan `name`; referensi [case.yaml fields](#case-yaml-fields) memiliki daftar lengkap.

`case.yaml` ini seed workspace dari skrip dan membiarkan Claude membaca fixture dari direktori `resources/`:

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Mock server MCP
</h3>

Anda dapat mengevaluasi plugin yang skills-nya memanggil alat MCP tanpa layanan nyata di belakangnya. Letakkan satu file Markdown per alat di bawah `evals/mocks/<server>/<tool>.md` untuk seluruh suite, atau di bawah direktori `mocks/` kasus sendiri untuk satu kasus, di mana `<server>` adalah nama server di [konfigurasi MCP](/docs/id/plugins/components#mcp-servers) plugin Anda.

Run tidak pernah memulai server MCP plugin Anda yang sebenarnya kecuali Anda meminta. Claude Code mendaftarkan pengganti di bawah nama server itu sendiri. Alat dengan file mock menjawab darinya dan diizinkan tanpa grant `--allow-tools`, dan alat tanpa file mock tidak tersedia untuk Claude. Server tanpa mock sama sekali muncul di baris kemajuan kasus sebagai `plugin_<plugin>_<server>[not started: no mock]`.

Body file adalah apa yang dikembalikan alat ke Claude. Mock ini berdiri untuk alat `create_issue` pada server bernama `tracker`, memeriksa input yang dikirim Claude, dan mengembalikan judul. Simpan sebagai `evals/mocks/tracker/create_issue.md`:

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

Body dan frontmatter file mock menerima opsi-opsi ini:

* **Substitusi**: sisipkan field dari input panggilan dengan `{{input.<field>}}`, dan konten file fixture di samping mock dengan `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`**: blok `expect:` menjaga input. Jika panggilan melanggarnya, run membatalkan dengan skor 0 dan mencatat mengapa, jadi kasus dapat menegaskan apa yang diminta plugin ke server.
* **`error: true`**: atur `error: true` untuk mengembalikan body sebagai kesalahan alat sebagai gantinya.
* **`type: agent`**: atur `type: agent` untuk memiliki model kecil menjawab sebagai server dari instruksi di body.

Referensi [mock file](#mock-files) mencantumkan setiap kunci dan file `_server.md` dan `_tools.json`.

Untuk menilai panggilan itu sendiri, arahkan grader ke `target: mock_calls`.

Untuk menjalankan terhadap server MCP plugin Anda yang sebenarnya, lewati salah satu flag ini. Baik cara proses itu berjalan sebagai Anda, di luar sandbox run, dan alatnya memerlukan grant [`--allow-tools`](#grant-tools):

* **`--allow-real-servers`**: mulai proses nyata untuk setiap server yang belum Anda mock, dan terus menjawab alat yang dimock dari file mereka
* **`--mocks off`**: abaikan `mocks/` sepenuhnya dan mulai setiap server yang dideklarasikan plugin

<h4 id="replay-agent-mock-answers">
  Putar ulang jawaban mock agen
</h4>

Mock `type: agent` menjawab dengan panggilan ke [`--judge-model`](#command-options), jadi outputnya bervariasi antar run dan berubah jika Anda mengubah judge. Ketika run selesai tanpa kesalahan atau pembatalan, Claude Code menyimpan setiap jawaban yang diberikan mock agen di bawah direktori hasil di `mock-recordings/`.

Buka `ADOPT.txt` di sana untuk melihat setiap rekaman dan direktori `.replay/<server>/` untuk menyalinnya, di samping mock yang menghasilkannya. Setelah Anda menyalin rekaman di sana, run kemudian menjawab panggilan identik darinya tanpa panggilan model. Commit `mocks/.replay/` bersama `mocks/` sehingga run CI dapat diulang.

<h2 id="run-evals">
  Jalankan evals
</h2>

Setelah suite ada, `claude plugin eval` menjalankannya. Anda memilih plugin dan kasus mana yang berjalan dengan argumen target, memberikan izin kepada alat apa pun yang diperlukan kasus di luar set read-only dengan `--allow-tools`, dan mengontrol jumlah run, model, biaya, dan output dengan opsi lainnya.

<h3 id="choose-what-to-evaluate">
  Pilih apa yang akan dievaluasi
</h3>

Sebagian besar waktu Anda menjalankan `claude plugin eval .` dari root plugin, yang menjalankan setiap kasus dalam suite dengan plugin yang Anda gunakan dimuat. Untuk menjalankan file kasus tunggal, atau untuk mengevaluasi plugin yang Anda instal daripada yang sedang Anda kembangkan, berikan target yang berbeda:

| Target                                                                | Apa yang berjalan                                                                                                                                                                                   |
| :-------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Direktori root plugin, seperti `.`                                    | Setiap kasus di bawah direktori eval-nya, dengan plugin itu dimuat                                                                                                                                  |
| File `prompt.md` atau `case.yaml` tunggal                             | Kasus itu, dengan plugin yang memuatnya dimuat                                                                                                                                                      |
| Plugin yang diinstal berdasarkan nama, `name` atau `name@marketplace` | Kasus dalam direktori eval salinan yang diinstal, dengan salinan yang diinstal dimuat. Hasil ditulis di bawah `./evals/results/` di direktori saat ini, atau `./<dir>/results/` dengan `--eval-dir` |
| `name@skills-dir`                                                     | Sama, untuk [plugin skills-directory](/docs/id/plugins/loading#plugins-shared-through-a-repository)                                                                                                      |
| Dihilangkan                                                           | Direktori saat ini sebagai path                                                                                                                                                                     |

Tambahkan `--case <glob>` untuk memfilter berdasarkan nama kasus dan `--tag <tag>` untuk menyimpan kasus dengan salah satu tag yang diberikan.

Letakkan target sebelum `--tag`, `--allow-tools`, dan `--json`. Dua yang pertama mengambil daftar dan `--json` mengambil path opsional, jadi masing-masing membaca target yang mengikuti sebagai nilainya sendiri.

<h3 id="grant-tools">
  Berikan alat
</h3>

Run tidak pernah berhenti untuk meminta izin. Alat bawaan yang memerlukan izin yang tidak Anda berikan, seperti `Bash`, `Write`, `Edit`, `WebFetch`, dan `WebSearch`, dihapus dari sesi, jadi Claude tidak dapat memanggilnya sama sekali.

Daftar izin adalah alat read-only yang daftar kasus dalam `allowed_tools`, dari `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite`, dan alat task `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, dan `TaskStop`, ditambah apa pun yang Anda berikan dengan `--allow-tools`. Pemberian itu berlaku untuk setiap kasus dalam run. Untuk membiarkan kasus menggunakan `Bash`, `Write`, `Edit`, `WebFetch`, atau `WebSearch`, berikan mereka sendiri:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Ketika kasus meminta alat yang tidak Anda berikan, output kemajuan mencantumkannya sebagai `not granted`. Alat pada [server MCP yang dimock](#mock-mcp-servers) tidak memerlukan izin. Alat pada server MCP plugin nyata memerlukan server yang dimulai, dengan `--allow-real-servers` atau `--mocks off`, dan izin berdasarkan nama, seperti `--allow-tools "mcp__plugin_my-plugin_github__*"`; alat MCP plugin dinamai `mcp__plugin_<plugin>_<server>__<tool>`.

Ketika Anda memberikan `Bash` dalam bentuk apa pun, setiap perintah berjalan di bawah [sandbox tingkat OS](/docs/id/sandboxing) Claude Code. Penulisan dibatasi pada workspace run, direktori home dan konfigurasi Claude Code tidak dapat dibaca, dan akses jaringan dibatasi pada domain yang Anda berikan dengan `--allow-tools "WebFetch(domain:example.com)"`. Jika Anda memberikan Bash atau PowerShell pada mesin tanpa backend sandbox, Claude Code menolak setiap run daripada menjalankannya tanpa batasan, dan kasus menunjukkan error run dan biasanya mencetak skor 0. Windows native tidak memiliki backend, jadi jalankan suite yang memberikan izin shell di bawah WSL2; di Linux, instal `bubblewrap` dan `socat` terlebih dahulu. Lihat [prasyarat sandboxing](/docs/id/sandboxing).

<h3 id="command-options">
  Opsi perintah
</h3>

Tabel ini mencakup opsi untuk jumlah run, model, penilaian, biaya, pemberian izin alat, mock, dan output. Jalankan `claude plugin eval --help` untuk daftar lengkap, yang juga mencakup `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report`, dan `--verbose`.

| Opsi                       | Default                                                                            | Efek                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :--------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | `runs` setiap kasus, atau 3                                                        | Run per kasus per arm                                                                                                                                                                                                                                                                                                                                 |
| `-j`, `--concurrency <n>`  | `1`                                                                                | Jalankan hingga banyak agent run sekaligus, dari 1 hingga 8. Mereka berbagi batas laju akun Anda, jadi ini mempersingkat waktu dinding daripada meningkatkan throughput melampaui batas itu. Hasil menjaga urutan kasus                                                                                                                               |
| `--model <model>`          | `model` setiap kasus, atau `ANTHROPIC_MODEL` jika diatur, atau default Claude Code | Model untuk agent yang diuji. Pasangnya di CI sehingga rollout model tidak disalahartikan sebagai regresi plugin                                                                                                                                                                                                                                      |
| `--judge-model <model>`    | Model kecil cepat                                                                  | Model untuk grader `llm` dan `baseline`                                                                                                                                                                                                                                                                                                               |
| `--ablation <mode>`        | `with-without` ketika plugin diselesaikan, atau `none`                             | Apakah juga menjalankan setiap kasus tanpa plugin untuk mengukur apa yang ditambahkannya. `none` menjalankan satu arm; `with-without` menambahkan baseline tanpa plugin                                                                                                                                                                               |
| `--threshold <0..1>`       | `1.0`                                                                              | Kasus lulus ketika skor with-arm-nya setidaknya ini. Kasus apa pun di bawahnya membuat perintah keluar 1                                                                                                                                                                                                                                              |
| `--max-cost-usd <usd>`     | Tidak ada batas                                                                    | Batas pada estimasi biaya harga daftar run, bukan pada penggunaan rencana. Diperiksa sebelum setiap run dimulai. Setelah dihabiskan, tidak ada yang dimulai lebih lanjut; run yang sudah dalam penerbangan selesai, jadi pengeluaran dapat melampaui batas oleh run tersebut. Jika ada run yang tidak dimulai, perintah keluar 2 dengan hasil parsial |
| `--allow-tools <tools...>` | Tidak ada                                                                          | Berikan alat di luar set read-only. Lihat [Berikan alat](#grant-tools)                                                                                                                                                                                                                                                                                |
| `--scaffold`               | Mati                                                                               | Jalankan [`scaffold_script`](#add-setup-or-history-with-case-yaml) setiap kasus                                                                                                                                                                                                                                                                       |
| `--trust-plugin`           | Mati                                                                               | Lewati prompt kepercayaan first-run untuk plugin yang kode dan suite-nya akan Anda jalankan sendiri. Berikan di CI sehingga pekerjaan tidak pernah ditolak oleh atau menunggu di prompt. Lihat [Apa yang dapat diakses run](#security)                                                                                                                |
| `--mocks <mode>`           | `record`                                                                           | `record` menjawab panggilan alat MCP dari [mock](#mock-mcp-servers), tidak memulai server nyata plugin, dan menyimpan jawaban agent-mock untuk replay. `off` mengabaikan mock dan memulai server MCP nyata plugin                                                                                                                                     |
| `--allow-real-servers`     | Mati                                                                               | Dengan `--mocks record`, juga mulai server MCP nyata plugin untuk server yang tidak memiliki mock                                                                                                                                                                                                                                                     |
| `--json [path]`            | Mati                                                                               | Cetak [dokumen hasil](#json-result) ke stdout, atau tulis ke path yang berakhir dengan `.json`. Run sunyi: tidak ada baris kemajuan atau tabel ringkasan                                                                                                                                                                                              |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                  | Tempat `aggregate-result.json` dan `report.html` pergi                                                                                                                                                                                                                                                                                                |
| `--no-publish`             |                                                                                    | Simpan laporan HTML secara lokal. Lihat [laporan HTML](#html-report)                                                                                                                                                                                                                                                                                  |
| `--publish-report`         |                                                                                    | Publikasikan laporan bahkan di mana itu akan tetap lokal secara default, seperti run yang dimulai sesi Claude Code                                                                                                                                                                                                                                    |
| `--keep-temp`              | Mati                                                                               | Simpan direktori sandbox setiap run dan cetak pathnya, untuk debugging apa yang dihasilkan Claude                                                                                                                                                                                                                                                     |

<h3 id="run-evals-in-ci">
  Jalankan evals di CI
</h3>

Dalam pekerjaan CI Anda, jalankan suite dengan `--json` untuk menulis hasil untuk pengarsipan, dan gagalkan build pada kode keluar. Berikan `--trust-plugin` sehingga pekerjaan tidak pernah menunggu di [prompt kepercayaan first-run](#security), pasang kedua model sehingga skor dapat dibandingkan dari waktu ke waktu, simpan laporan secara lokal, dan atur batas biaya sebagai batas atas:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

Kode keluar pekerjaan memberi tahu Anda apa yang terjadi:

| Kode keluar | Arti                                                                                                                                                                                                                  |
| :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0           | Setiap kasus mencetak skor pada atau di atas `--threshold` dan setiap file kasus dimuat                                                                                                                               |
| 1           | Kasus mencetak skor di bawah threshold, file kasus gagal dimuat, tidak ada kasus yang ditemukan, run tidak dapat dimulai, direktori plugin tidak dipercaya dan `--trust-plugin` tidak dilewati, atau opsi tidak valid |
| 2           | Run parsial: batas `--max-cost-usd` tercapai, atau kredensial Anda ditolak sebelum atau pada run pertama. `results.json` masih ditulis dengan `partial: true` dan alasannya                                           |
| 130         | Terputus. Hasil parsial ditulis                                                                                                                                                                                       |
| 143         | Dihentikan, seperti oleh timeout CI                                                                                                                                                                                   |

Masalah menulis atau menerbitkan laporan HTML tidak pernah mengubah kode keluar.

Untuk melihat mengapa kasus mencetak skor rendah, jalankan secara lokal tanpa `--json` sehingga kemajuan per-run dan baris grader mencetak.

Runner CI juga memerlukan hal-hal ini:

* **Instal dan kredensial**: runner CI memerlukan instalasi Claude Code dan [kredensial di lingkungan](/docs/id/authentication) seperti `ANTHROPIC_API_KEY`.
* **Kepercayaan**: tanpa `--trust-plugin`, pekerjaan yang direktori checkoutnya Claude Code belum percayai memerlukan [prompt kepercayaan first-run](#trust-the-plugin-directory), dan run yang tidak dapat bertanya ditolak dengan keluar 1.
* **`init` di CI**: `claude plugin eval init` memerlukan terminal untuk mengajukan pertanyaan Anda; di CI, jalankan `claude plugin eval init --bare <name>` untuk mendapatkan template kosong.

Untuk menjaga biaya dapat diprediksi, berikan suite setiap perubahan cepat hanya grader yang tidak memanggil hakim, gunakan `--ablation none` di mana Anda tidak memerlukan `Δ`, dan tinggalkan dokumen `partial: true` dan run dengan `skippedPaidGraders` keluar dari tren apa pun yang Anda buat.

<h2 id="read-the-results">
  Baca hasilnya
</h2>

Setiap run dengan setidaknya satu kasus menulis direktori `results/<timestamp>/` di dalam direktori eval, berisi `aggregate-result.json` dan `report.html`. Untuk target jalur yang berada di bawah plugin; untuk plugin yang Anda namai, itu di bawah direktori saat ini, seperti yang ditunjukkan [tabel target](#choose-what-to-evaluate). Tabel ringkasan, JSON, dan laporan semuanya merender data hasil yang sama.

<h3 id="html-report">
  Laporan HTML
</h3>

`report.html` adalah file mandiri tunggal yang tidak membuat permintaan eksternal, jadi Anda dapat melampirkannya ke pekerjaan CI atau membukanya dari disk. Contoh ini adalah bagian atas laporan untuk run suite tiga kasus dengan `--threshold 0.8`; biaya yang ditampilkan adalah perkiraan harga daftar dan bervariasi dengan model dan jumlah kasus:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Bagian atas laporan eval: baris putusan yang berbunyi &#x22;Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases&#x22;, lima ubin ringkasan untuk skor suite, delta ablasi, skor baseline, kasus yang melewati threshold, dan run sempurna, kemudian kasus pertama dengan delta, batang skor, dan satu run yang kedua grader-nya menunjukkan lulus" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Bacanya dari atas ke bawah:

* **Baris putusan dan ubin** menjawab apakah plugin membantu di seluruh suite. Skor suite adalah rata-rata skor with-plugin per-kasus, Ablation Δ adalah seberapa jauh itu berada di atas atau di bawah skor baseline, dan Cases menghitung berapa banyak yang memenuhi threshold. Perfect runs adalah bagian dari run with-plugin di mana setiap grader lulus.
* **Setiap kartu kasus** menunjukkan `Δ` kasus sendiri dan skor with-plugin, dengan tanda centang pada batang di threshold. Kasus yang `Δ`-nya negatif mendapat tepi kiri merah, jadi regresi menonjol saat Anda menggulir.
* **Di dalam kasus**, run with-plugin datang terlebih dahulu dan run baseline setelahnya. Setiap run mencantumkan grader-nya dengan chip lulus atau gagal. Grader yang gagal sudah diperluas dengan penjelasannya, dan grader `llm` juga menunjukkan suara hakim dan bukti yang ditampilkannya, di mana Anda menemukan alasan mengapa run mendapat skor rendah. Grader yang tidak diperhitungkan terhadap skor, seperti `tool_used: Skill`, membawa lencana `plugin-fired indicator`.
* **Prompt dan Graders**, di bawah run, menunjukkan prompt kasus dan rubrik atau pola setiap grader, sehingga seseorang yang membaca laporan tanpa suite dapat melihat apa yang ditanyakan dan apa yang dihitung sebagai baik.

Jika Anda masuk dengan langganan claude.ai dan [artifacts](/docs/id/artifacts) tersedia untuk akun Anda, Claude Code juga menerbitkan laporan sebagai artifact pribadi dan mencetak `Published: <url>`. Lewati `--no-publish` untuk menyimpannya lokal. Jika tidak ada baris `Published:` yang muncul, seperti dengan autentikasi kunci API, file lokal adalah laporan.

Run yang dimulai sesi Claude Code, seperti ketika Anda meminta Claude menjalankan suite untuk Anda, juga tetap lokal, dan baris `Report:`-nya mengatakan `kept local`. Tambahkan `--publish-report` ke perintah itu untuk menerbitkannya.

<h3 id="json-result">
  Hasil JSON
</h3>

`aggregate-result.json`, dan output `--json`, adalah dokumen versi dengan `schemaVersion: 1` untuk skrip CI untuk diurai. Nama field adalah camelCase dan field baru ditambahkan tanpa mengganti nama yang ada, jadi tulis skrip Anda untuk mengabaikan field yang tidak dikenalinya.

Ini adalah field yang biasanya dibaca skrip gating. Dokumen juga membawa konfigurasi suite, setiap definisi grader, dan hasil grader per-run dengan penjelasan dan bukti:

| Field                                             | Arti                                                                                                                                                                                                           |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `true` dengan `cost_ceiling`, `interrupted`, atau `auth_failed` ketika suite tidak selesai. Tinggalkan hasil parsial dari tren bagan                                                                           |
| `aggregates.overallScore`                         | Skor kasus rata-rata di seluruh suite                                                                                                                                                                          |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Kasus pada atau di atas `--threshold`, dan totalnya                                                                                                                                                            |
| `aggregates.meanDelta`                            | Rata-rata `Δ` di seluruh kasus, di bawah mode dua-arm                                                                                                                                                          |
| `cases[].name`                                    | Nama kasus                                                                                                                                                                                                     |
| `cases[].aggregates.score`                        | Skor run with-arm rata-rata untuk kasus                                                                                                                                                                        |
| `cases[].aggregates.delta`                        | Skor with-arm minus skor without-arm. Dihilangkan ketika arm tidak dapat dibandingkan                                                                                                                          |
| `cases[].arms.with[].error`                       | `null`, atau mengapa run berakhir abnormal, seperti `timed out after 300s`. Run yang dimulai tetapi berakhir buruk masih dinilai pada apa yang dihasilkannya, jadi kesalahan non-null tidak menyiratkan skor 0 |
| `cases[].arms.with[].aborted`                     | Hadir ketika [mock](#mock-mcp-servers) `expect:` atau `abort_when` menghentikan run, dengan `server`, `tool`, dan `reason`. Run mencetak 0 dan `error` tetap `null`                                            |
| `cases[].arms.with[].skippedPaidGraders`          | `true` ketika batas biaya melewati grader judge run ini, jadi skornya tidak dapat dibandingkan                                                                                                                 |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Biaya perkiraan pada harga daftar termasuk panggilan judge, detik dinding-jam, dan versi Claude Code yang menjalankan suite                                                                                    |

<h2 id="security">
  Apa yang dapat diakses run
</h2>

`claude plugin eval` memuat skill, hook, dan agent plugin target dan menjalankan suite evalnya di mesin Anda, sebagai Anda. Menunjuknya ke plugin adalah keputusan kepercayaan yang sama dengan `claude --plugin-dir`, jadi hanya evaluasi plugin yang Anda percayai.

Isolasi yang dijelaskan di bagian ini membatasi apa yang dapat dijangkau agen yang diuji; itu bukan batas terhadap kode plugin itu sendiri, dan suite yang lulus tidak mengatakan apa pun tentang apakah plugin aman.

<h3 id="trust-the-plugin-directory">
  Percayai direktori plugin
</h3>

Pertama kali Anda menjalankan `claude plugin eval` terhadap direktori, Claude Code menanyakan `Trust this plugin directory?` sebelum memuat apa pun darinya, kecuali Anda sudah menerima prompt kepercayaan di sana dalam sesi `claude` interaktif. Di dalam repositori git, menjawab ya mempercayai seluruh repositori, untuk sesi interaktif juga. Ketika stdin atau stdout bukan terminal, di bawah `--json`, atau ketika variabel lingkungan `CI` diatur ke nilai true seperti `true`, run tidak dapat bertanya dan ditolak dengan keluar 1; lewati `--trust-plugin` untuk menegaskan kepercayaan sendiri, hanya untuk plugin yang akan Anda jalankan di mesin Anda sendiri. Target yang Anda namai daripada berikan sebagai jalur, berarti plugin yang diinstal atau plugin direktori skills, melewati prompt.

Beberapa bagian dari plugin dan suite berjalan hanya ketika Anda melewati flag mereka untuk run itu:

* [`scaffold_script`](#add-setup-or-history-with-case-yaml) kasus dengan `--scaffold`
* [Alat di luar set read-only](#grant-tools) dengan `--allow-tools`
* [Server MCP nyata](#mock-mcp-servers) plugin dengan `--allow-real-servers` atau `--mocks off`

`allowed_tools` kasus dan frontmatter `allowed-tools` skill sendiri tidak dapat memperluas salah satu dari mereka.

Ketika plugin mengirim hook yang tidak Anda tulis, atau Anda memulai server MCP nyatanya, perlakukan skornya sebagai penasihat kecuali Anda menjalankannya di lingkungan terisolasi seperti kontainer atau runner CI, karena hook dan server berjalan di luar sandbox agen dan dapat mengubah file yang dibaca grader.

<h3 id="how-runs-are-isolated">
  Cara run diisolasi
</h3>

Setiap run mendapat direktori home, direktori kerja, dan konfigurasi Claude Code yang dapat dibuang, dan agen yang diuji berjalan di sana sebagai proses anak `claude -p` dengan hanya plugin Anda dimuat. Ingat konsekuensi ini saat menulis kasus:

* **Tidak ada yang pribadi atau tingkat proyek yang dimuat.** Pengaturan pengguna, hook, file `CLAUDE.md`, server MCP, plugin yang diinstal lainnya, memori, dan skill Anda tidak ada, dan tidak ada proyek-scoped `.claude/` atau `.mcp.json` di atas sandbox yang dibaca. Sebagian besar lingkungan shell Anda juga ditahan; hanya [daftar izin](#prompt-md-fields) dan variabel `EVAL_*` mencapai run. Jika plugin memerlukan setup, kirimkan di plugin, buat di `scaffold_script`, atau lewati variabel `EVAL_*`.
* **Kebijakan terkelola masih dapat membatasi run.** Pembatasan di [pengaturan terkelola](/docs/id/managed-settings) yang diterapkan administrator ke mesin berlaku di dalam run, jadi hasil pada mesin terkelola dapat berbeda dari yang tidak terkelola oleh kebijakan itu.
* **Alat Artifact mati.** Skill yang menerbitkan [artifact](/docs/id/artifacts) dapat dinilai hanya pada apa yang dihasilkannya sebelum langkah itu.
* **Definisi kasus disembunyikan dari agen.** Run tidak dapat membaca direktori eval, jadi Claude tidak dapat melihat prompt kasus, grader, atau kasus saudara.
* **Tidak ada sandbox jaringan di luar perintah shell.** Perintah shell yang Anda berikan berjalan di bawah aturan sandbox jaringan. Grant `WebFetch(domain:…)` mencapai domain itu secara langsung, dan hook plugin sendiri dan server MCP nyata apa pun yang Anda mulai dapat mencapai host apa pun.

<h2 id="eval-suite-reference">
  Referensi suite eval
</h2>

Semuanya yang dapat berisi suite eval hidup di bawah direktori eval plugin, `evals/` kecuali Anda [mengonfigurasi yang lain](#use-a-different-eval-directory). Pohon ini menunjukkan setiap file yang dibaca atau ditulis `claude plugin eval` di sana; hanya `prompt.md` atau `case.yaml` yang diperlukan untuk kasus ada:

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  Frontmatter prompt.md
</h3>

Frontmatter `prompt.md` menerima field ini. Kunci yang tidak dikenal adalah kesalahan:

| Field                  | Default                    | Tujuan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :--------------------- | :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, diatur untuk Anda | Versi format kasus. Kasus yang ditulis sebagai `prompt.md` mendapatkannya secara otomatis, jadi Anda jarang mengaturnya                                                                                                                                                                                                                                                                                                                                                                    |
| `name`                 | Nama direktori             | Nama kasus. Glob `--case` cocok dengannya dan laporan kunci padanya                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `description`          |                            | Untuk manusia. Tidak digunakan saat runtime                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `tags`                 | `[]`                       | Label untuk penyaringan `--tag`. Kasus berjalan jika salah satu tagnya cocok                                                                                                                                                                                                                                                                                                                                                                                                               |
| `plugins`              | Plugin penutup terdekat    | Direktori plugin di bawah pengujian, relatif terhadap direktori kasus. Atur `plugins: ["../.."]` ketika deteksi otomatis tidak menemukan plugin Anda; lihat [plugin tidak dimuat](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                      |
| `runs`                 | `3`                        | Run per arm, 1 hingga 50. `--runs` menimpanya                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `expected_outcome`     |                            | Untuk manusia. Tidak digunakan saat runtime                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `model`                | Default sesi anak          | Model untuk agen yang diuji. `--model` menimpanya                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `max_turns`            | `10`                       | Batas turn, hingga 200. Mencapainya dicatat sebagai kesalahan run dan biasanya menurunkan skor, jadi aturnya dengan murah hati                                                                                                                                                                                                                                                                                                                                                             |
| `timeout_seconds`      | `300`                      | Batas dinding-jam per run, hingga 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed_tools`        | `[]`                       | Alat yang diinginkan kasus, seperti `[Read, Glob, Grep, Skill]`. Alat read-only diberikan ketika dicantumkan di sini; untuk apa pun yang lain, lihat [Berikan alat](#grant-tools)                                                                                                                                                                                                                                                                                                          |
| `append_system_prompt` |                            | Teks ditambahkan ke prompt sistem sesi anak                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `env`                  | `{}`                       | Variabel lingkungan ekstra untuk sesi anak. Kunci harus cocok `EVAL_[A-Z0-9_]*`; kunci apa pun yang lain gagal run. Run mewarisi hanya daftar izin dari shell Anda: dasar seperti `PATH` dan lokal, pengaturan proxy dan sertifikat, variabel yang memilih dan mengautentikasi penyedia model Anda, sebagian besar `ANTHROPIC_*` dan `CLAUDE_CODE_*` konfigurasi, dan `EVAL_*`. Untuk menyerahkan plugin apa pun yang lain, seperti pengaturan toolchain, ekspor sebagai variabel `EVAL_*` |

<h3 id="case-yaml-fields">
  Field case.yaml
</h3>

`case.yaml` adalah alternatif atau pendamping untuk `prompt.md`: ini menjelaskan kasus dalam YAML dan menambahkan field yang menunjuk ke file lain. Ini memerlukan `schema_version: "1.1"` dan `name`. Field `prompt.md` `description`, `tags`, `plugins`, `runs`, dan `expected_outcome` berada di tingkat atas; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt`, dan `env` berada di bawah `execution:`. Ketika kedua file ada, frontmatter `prompt.md` menimpa field `case.yaml` yang cocok, body `prompt.md` adalah prompt, dan `graders/*.md` ditambahkan setelah grader apa pun yang dicantumkan di `case.yaml`.

Field ini hanya ada di `case.yaml`:

| Field                     | Tujuan                                                                                                                                                                                                                             |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Skrip Bash di direktori kasus yang berjalan di workspace kosong sebelum Claude dimulai, untuk membuat file fixture atau repositori git. Ini berjalan hanya ketika Anda lewati [`--scaffold`](#add-setup-or-history-with-case-yaml) |
| `context.history_file`    | Transkrip `.jsonl` di direktori kasus untuk dilanjutkan. Prompt kasus menjadi turn pengguna berikutnya                                                                                                                             |
| `context.add_dirs`        | Direktori di dalam direktori kasus yang dapat dibaca Claude selama run, diberikan read-only                                                                                                                                        |
| `execution.prompt`        | Prompt, ketika Anda menyimpan seluruh kasus di `case.yaml` dan menghilangkan `prompt.md`                                                                                                                                           |
| `graders`                 | Daftar grader, masing-masing dengan `name` ditambah kunci yang sama yang diambil file `graders/*.md` di frontmatter. Untuk grader `llm`, letakkan rubrik di `criteria`                                                             |

<h3 id="grader-frontmatter">
  Frontmatter grader
</h3>

Setiap file grader di bawah `graders/` mengambil kunci ini di frontmatter, ditambah opsi untuk tipenya. Nama grader adalah nama file tanpa `.md`:

| Kunci    | Default      | Tujuan                                                                                                                                                                                           |
| :------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`   | diperlukan   | Salah satu [tipe grader](#grader-types)                                                                                                                                                          |
| `weight` | `1`          | Bobot relatif dalam skor run. Angka positif apa pun                                                                                                                                              |
| `arm`    | tidak diatur | `with-only` mengecualikan grader dari penilaian dalam [run dua-arm](#compare-against-a-no-plugin-baseline); `both` memaksa grader yang Claude Code akan mengecualikan untuk dinilai di kedua arm |

<h4 id="what-a-grader-can-look-at">
  Apa yang dapat dilihat grader
</h4>

Grader `regex` mengambil `target` dan grader `llm` mengambil `focus`. Keduanya menerima nilai yang sama:

| Nilai                            | Apa yang dilihat grader                                                                                                                                                                                                                                                                            |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Teks respons akhir Claude. Ini adalah default                                                                                                                                                                                                                                                      |
| `trace`                          | Seluruh sesi sebagai JSON, satu pesan per baris. Grader `regex` melihat setiap pesan; judge `llm` melihat 12 pertama dan 12 terakhir. Kutipan dan baris baru di dalamnya adalah JSON-escaped, jadi regex cocok `\"` daripada `"`                                                                   |
| `files`                          | Daftar jalur yang dibuat Claude selama run, satu per baris. Bukan kontennya, dan bukan file yang dibuat scaffold atau yang hanya dimodifikasi Claude                                                                                                                                               |
| `{ source: file, path: <path> }` | Konten satu file di workspace setelah run. Gunakan ini untuk menilai apa yang dihasilkan plugin. File PNG, JPEG, GIF, atau WebP ditunjukkan ke judge `llm` sebagai gambar. Judge `llm` menolak file biner lainnya seperti `.pptx` atau PDF; render ke gambar atau tulis sebagai teks dan nilai itu |
| `mock_calls`                     | Setiap panggilan yang dibuat Claude ke [alat MCP yang dimock](#mock-mcp-servers), dengan input dan jawaban mock                                                                                                                                                                                    |

<h4 id="grader-types">
  Tipe grader
</h4>

Setiap tipe grader di bawah mencantumkan opsi dan kapan lulus:

| Tipe          | Opsi                                  | Lulus ketika                                                                                                                                                                                                                                           |
| :------------ | :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`, `flags`, `match`, `target` | Regex JavaScript `pattern` ditemukan di target. Atur `match: not_contains` untuk memerlukan ketiadaan atau `match: "count:N"` untuk memerlukan tepat N kecocokan. Letakkan ketidakpekaan huruf besar-kecil di `flags: i`; inline `(?i)` tidak didukung |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | Jumlah panggilan ke `tool` yang input JSON-encoded cocok dengan regex `input_match` opsional berada antara `min`, default 1, dan `max`, default unlimited. Untuk menegaskan alat tidak pernah dipanggil, atur keduanya `min: 0` dan `max: 0`           |
| `tool_order`  | `before`, `after`                     | Kedua alat dipanggil dan panggilan `before` pertama yang cocok mendahului panggilan `after` pertama yang cocok. Masing-masing adalah nama alat atau `{ tool, input_match }`                                                                            |
| `file_exists` | `path`, `exists`                      | File yang dibuat Claude cocok dengan glob `path`, atau tidak ada dengan `exists: false`. Hanya file yang dibuat selama run yang dihitung                                                                                                               |
| `llm`         | `criteria`, `focus`                   | Model judge memilih PASS pada rubrik dalam setidaknya dua dari tiga suara. Dalam tata letak `.md` body file adalah kriteria                                                                                                                            |
| `baseline`    | `baseline_file`, `criteria`           | Judge menemukan run memenuhi kriteria setidaknya sebaik transkrip referensi di `baseline_file`, `.jsonl` di direktori kasus                                                                                                                            |

<h3 id="mock-files">
  File mock
</h3>

File `<tool>.md` di bawah `mocks/<server>/` menjawab satu alat. Bodynya adalah hasil alat, dengan substitusi `{{input.<field>}}` dan `{{file:fixtures/<name>}}`. Frontmatternya menerima kunci ini:

| Kunci        | Default      | Tujuan                                                                                                                                                                                                                                                                                        |
| :----------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `fixed`      | `fixed` mengembalikan body seperti yang ditulis. `agent` memperlakukan body sebagai instruksi untuk model kecil yang memainkan server untuk run dan melihat panggilan sebelumnya sebagai riwayat                                                                                              |
| `expect`     | tidak diatur | Peta dari jalur input bertitik ke nama tipe seperti `string`, `number`, `boolean`, `array`, atau `object`, `/regex/`, literal, atau daftar literal yang diizinkan. Panggilan yang melanggarnya membatalkan run dengan skor 0 dan dilaporkan sebagai `aborted` dengan server, alat, dan alasan |
| `error`      | `false`      | `fixed` hanya. Kembalikan body sebagai kesalahan alat                                                                                                                                                                                                                                         |
| `abort_when` | tidak diatur | `agent` hanya. Prosa yang mencantumkan satu-satunya kondisi di mana agen dapat membatalkan run                                                                                                                                                                                                |

Dua file opsional duduk di samping file alat di direktori server:

* **`_server.md`**: mock `type: agent` tunggal yang menjawab beberapa alat, dicantumkan di kunci frontmatter `tools:`. `<tool>.md` untuk alat yang sama mengambil prioritas. Letakkan guard `expect:` pada `<tool>.md` individual, bukan di sini
* **`_tools.json`**: respons `tools/list` yang disimpan dari server nyata, jadi alat yang dimock membawa deskripsi dan skema input nyata mereka daripada placeholder yang permisif

Direktori `mocks/` kasus sendiri menggunakan tata letak yang sama dan menimpa file suite file mock.

<h2 id="troubleshooting">
  Pemecahan masalah
</h2>

Ini adalah masalah yang paling sering dihadapi penulis, dikunci pada apa yang Anda lihat.

<h3 id="plugin-eval-is-currently-in-early-access">
  "plugin eval is currently in early access"
</h3>

Build Anda mendahului ketersediaan umum perintah. Jalankan `claude update`, kemudian jalankan perintah lagi dalam sesi segar.

<h3 id="plugin-eval-is-currently-unavailable">
  "plugin eval is currently unavailable"
</h3>

Anthropic telah mematikan perintah server-side. Tidak ada yang di mesin Anda menghidupkannya kembali; jalankan `claude update` dan coba lagi dalam sesi segar nanti.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  "is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

Ini adalah run pertama terhadap direktori yang Claude Code belum percayai, dan tidak dapat bertanya karena stdin atau stdout bukan terminal, Anda melewati `--json`, atau variabel lingkungan `CI` diatur ke nilai true seperti `true`. Jalankan `claude plugin eval <dir>` sekali dalam terminal dan jawab prompt, atau lewati `--trust-plugin` jika Anda mempercayai kode dan suite plugin. Lihat [Apa yang dapat diakses run](#security).

<h3 id="no-eval-cases-found">
  "No eval cases found"
</h3>

Tidak ada `<case>/prompt.md` atau `<case>/case.yaml` yang ada di bawah direktori eval yang berlaku, atau filter `--case` dan `--tag` Anda tidak cocok dengan kasus apa pun. Jalankan dari root plugin, atau jalankan `claude plugin eval init` untuk membuat suite.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  Arm baseline menunjukkan tidak ada plugin, atau delta adalah nol
</h3>

Jika ringkasan tidak memiliki kolom `W/OUT`, atau kasus gagal dengan "ablation requested but no plugin resolved", tidak ada plugin yang ditemukan untuk kasus. Tambahkan `plugins: ["../.."]` ke kasus, memberikan jalur dari direktori kasus ke direktori plugin.

Jika plugin memang dimuat dan `Δ` masih mendekati nol dengan grader `tool_used: Skill` Anda gagal, itu biasanya temuan nyata, berarti `description` skill tidak memicu pada frasa prompt. Sesuaikan deskripsi dan jalankan ulang suite yang sama.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  "Agent type '...' not found" untuk salah satu agen plugin Anda
</h3>

Secara default setiap kasus berjalan baik dengan plugin Anda maupun tanpanya, dan run tanpanya adalah [baseline tanpa plugin](#the-no-plugin-baseline). Ketika Claude mengirimkan salah satu agen plugin Anda dalam run baseline, panggilan alat Agent gagal dengan `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`. Daftar hanya menamai agen yang ada tanpa plugin, seperti [subagen bawaan](/docs/id/sub-agents#built-in-subagents).

Kesalahan diharapkan, karena `Δ` membandingkan run plugin Anda terhadap baseline. Dalam hasil JSON, run baseline berada di bawah `cases[].arms.without`.

Dalam run dengan plugin Anda dimuat, kasus yang mencantumkan `Agent` dalam `allowed_tools` dapat mengirimkan salah satu agen plugin Anda dengan nama yang diberi namespace, seperti `my-plugin:code-reviewer` untuk agen `code-reviewer` dalam plugin bernama `my-plugin`. Untuk melewati run baseline, lewati `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Semuanya mencetak nol meskipun file yang benar diproduksi
</h3>

Grader Anda menargetkan `files`, daftar jalur yang dibuat, ketika Anda bermaksud konten file. Gunakan `{ source: file, path: <path> }` sebagai `target` atau `focus`. Terpisah, `file_exists` menghitung hanya file yang dibuat selama run, jadi file yang dibuat scaffold atau yang hanya diedit Claude tidak terlihat; nilai kontennya, atau gunakan `tool_used` pada `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Regex atas trace tidak cocok dengan teks yang dapat saya lihat
</h3>

* **Target yang salah**: `target` default adalah `last_message`, bukan trace.
* **Escaping JSON**: ketika Anda menargetkan `trace`, itu JSON per baris, jadi kutipan muncul sebagai `\"`.
* **Sintaks regex**: regex menggunakan sintaks JavaScript, jadi letakkan `i` di `flags` daripada menulis `(?i)`.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Alat ditolak, alat MCP hilang, atau Bash tidak akan berjalan
</h3>

Apa pun di luar set read-only memerlukan grant Anda, seperti `--allow-tools Bash Write`. Server MCP pribadi Anda tidak pernah dimuat dalam run. Server plugin sendiri tidak dimulai kecuali Anda [opt in](#mock-mcp-servers), dan alatnya kemudian juga memerlukan grant `--allow-tools "mcp__plugin_<plugin>_<server>__*"`; alat yang dimock tidak memerlukan keduanya.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  Run keluar 1 tetapi hasilnya terlihat baik
</h3>

Default `--threshold` adalah 1.0, jadi perintah keluar 1 ketika kasus apa pun mencetak di bawah sempurna. Atur ambang yang cocok dengan skor yang Anda perlukan. Keluar 1 juga mencakup file kasus yang gagal dimuat, yang dilaporkan di stderr di atas tabel.

<h3 id="json-output-path-must-end-in-json">
  "--json output path must end in .json"
</h3>

Anda menempatkan target setelah `--json`, jadi itu dibaca sebagai jalur output. Letakkan target terlebih dahulu, seperti dalam `claude plugin eval . --json`, atau berikan `--json` jalur `.json` eksplisit.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Grader menunjukkan passed: false di bawah run yang mencetak 1.0
</h3>

Grader itu dikecualikan dari skor dengan desain dalam run dua-arm, dan field `scored`-nya adalah `false`. Lihat [Bandingkan dengan baseline tanpa plugin](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Run gagal dengan kesalahan batas penggunaan atau batas laju setengah jalan
</h3>

Jika akun Anda mencapai batas penggunaan rencana atau batas laju API saat suite berjalan, setiap run kemudian berakhir dengan kesalahan itu, dinilai pada apa yang dihasilkannya, dan biasanya mencetak 0. Suite masih selesai dan tidak ditandai `partial`, jadi hasilnya dapat terlihat seperti regresi. Periksa kolom `NOTES` atau `cases[].arms.with[].error` dalam JSON untuk pesan batas sebelum mempercayai skor, kemudian jalankan ulang setelah batas disetel ulang, dengan `--runs 1` atau filter `--case` jika Anda perlu tetap di bawahnya.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Run timeout atau mencapai batas turn
</h3>

Default adalah 10 turn dan 300 detik. Naikkan `max_turns` dan `timeout_seconds` dalam kasus untuk tugas yang memerlukan lebih banyak, dan gunakan `--max-cost-usd` sebagai batas biaya daripada batas per-run yang ketat.

<h2 id="see-also">
  Lihat juga
</h2>

* [Buat plugin](/docs/id/plugins/create): bangun plugin yang Anda uji, dan muat dengan `--plugin-dir` selama pengembangan
* [Referensi perintah plugin](/docs/id/plugins/cli-reference#plugin-eval): entri perintah `plugin eval` dan `plugin eval init`. Kunci manifest [`experimental.evals`](/docs/id/plugins/manifest-reference#fields) ada di referensi manifest
* [Skills](/docs/id/skills): bagaimana deskripsi skill memutuskan kapan Claude menginvokasinya, yang merupakan apa yang diukur kasus yang memeriksa apakah skill memicu
* [Sandboxing](/docs/id/sandboxing): sandbox tingkat OS yang berlaku ketika Anda memberikan Bash ke run
* [Terbitkan plugin](/docs/id/plugins/publish): terbitkan plugin setelah suitenya lulus
* [Ukur biaya dan penggunaan plugin](/docs/id/plugins/measure): apa yang ditambahkan plugin ke konteks setiap sesi dan apakah orang masih menggunakannya
