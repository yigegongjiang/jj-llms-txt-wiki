> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orkestrasi subagen dalam skala besar dengan alur kerja dinamis

> Alur kerja dinamis mengorkestrasi banyak subagen dari skrip yang ditulis Claude dan dapat Anda jalankan kembali. Gunakan untuk audit basis kode, migrasi besar, dan penelitian lintas-periksa.

<Note>
  Alur kerja dinamis memerlukan Claude Code v2.1.154 atau lebih baru dan tersedia di semua paket berbayar, dengan akses API Anthropic, dan di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Di Pro, aktifkan dari baris Dynamic workflows di `/config`.
</Note>

Alur kerja dinamis adalah skrip JavaScript yang mengorkestrasi [subagen](/docs/id/sub-agents) dalam skala besar. Claude menulis skrip untuk tugas yang Anda jelaskan, dan runtime menjalankannya di latar belakang sementara sesi Anda tetap responsif.

Gunakan alur kerja ketika tugas memerlukan lebih banyak agen daripada yang dapat dikoordinasikan satu percakapan, atau ketika Anda ingin orkestrasi dikodifikasi sebagai skrip yang dapat Anda baca dan jalankan kembali. Contohnya termasuk penyapuan bug di seluruh basis kode, migrasi 500 file, pertanyaan penelitian yang memerlukan sumber untuk diperiksa silang satu sama lain, dan rencana sulit yang layak dirancang dari beberapa sudut pandang independen sebelum Anda berkomitmen pada satu.

<h2 id="when-to-use-a-workflow">
  Kapan menggunakan alur kerja
</h2>

[Subagen](/docs/id/sub-agents), [skills](/docs/id/skills), [tim agen](/docs/id/agent-teams), dan alur kerja semuanya dapat menjalankan tugas multi-langkah. Perbedaannya adalah siapa yang memegang rencana:

|                                                     | Subagen                                       | Skills                        | Tim agen                             | Alur kerja                             |
| :-------------------------------------------------- | :-------------------------------------------- | :---------------------------- | :----------------------------------- | :------------------------------------- |
| Apa itu                                             | Pekerja Claude yang dihasilkan                | Instruksi yang diikuti Claude | Agen utama yang mengawasi sesi rekan | Skrip yang dijalankan runtime          |
| Siapa yang memutuskan apa yang berjalan selanjutnya | Claude, giliran demi giliran                  | Claude, mengikuti prompt      | Agen utama, giliran demi giliran     | Skrip                                  |
| Di mana hasil antara tinggal                        | Jendela konteks Claude                        | Jendela konteks Claude        | Daftar tugas bersama                 | Variabel skrip                         |
| Apa yang dapat diulang                              | Definisi pekerja                              | Instruksi                     | Definisi tim                         | Orkestrasi itu sendiri                 |
| Skala                                               | Beberapa tugas yang didelegasikan per giliran | Sama dengan subagen           | Segelintir rekan yang berjalan lama  | Puluhan hingga ratusan agen per run    |
| Gangguan                                            | Memulai ulang giliran                         | Memulai ulang giliran         | Rekan kerja terus berjalan           | Dapat dilanjutkan dalam sesi yang sama |

Alur kerja memindahkan rencana ke dalam kode. Dengan subagen, skills, dan tim agen, Claude adalah orkestrator: ia memutuskan giliran demi giliran apa yang akan dihasilkan atau ditugaskan selanjutnya, dan setiap hasil mendarat di jendela konteks. Skrip alur kerja memegang loop, percabangan, dan hasil antara itu sendiri, jadi konteks Claude hanya memegang jawaban akhir.

Memindahkan rencana ke dalam kode juga memungkinkan alur kerja menerapkan pola kualitas yang dapat diulang, bukan hanya menjalankan lebih banyak agen: ia dapat memiliki agen independen yang secara adversarial meninjau temuan satu sama lain sebelum dilaporkan, atau merancang rencana dari beberapa sudut dan menimbangnya satu sama lain, sehingga Anda mendapatkan hasil yang lebih dapat dipercaya daripada satu kali jalan.

<h2 id="run-a-bundled-workflow">
  Jalankan alur kerja bundel
</h2>

Cara tercepat untuk melihat alur kerja dalam tindakan adalah menjalankan `/deep-research`, [alur kerja bawaan](#bundled-workflows) yang disertakan Claude Code untuk menyelidiki pertanyaan di banyak sumber. Anda akan melihat agen bekerja melalui serangkaian fase di latar belakang sementara sesi Anda tetap bebas, dan dapatkan satu laporan di akhir daripada transkrip giliran demi giliran.

<Steps>
  <Step title="Jalankan alur kerja">
    Jalankan `/deep-research` dengan pertanyaan yang ingin Anda selidiki. Ini menyebarkan pencarian web di beberapa sudut, mengambil dan memeriksa silang sumber yang ditemukannya, dan mensintesis laporan yang dikutip.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Izinkan alur kerja">
    Claude Code menanyakan apakah akan mengizinkan alur kerja. Pilih **Yes** untuk melanjutkan. Prompt yang tepat tergantung pada mode izin Anda. Lihat [Setujui rencana sebelum berjalan](#approve-the-plan-before-it-runs) untuk opsi per-mode.
  </Step>

  <Step title="Tonton kemajuan">
    Run dimulai di latar belakang. Jalankan `/workflows`, gunakan tombol panah untuk memilih run, dan tekan Enter untuk membuka tampilan kemajuannya:

    ```text theme={null}
    /workflows
    ```

    Tampilan menunjukkan setiap fase dengan jumlah agen, total token, dan waktu yang telah berlalu. Bor ke dalam fase apa pun untuk melihat agennya dan apa yang masing-masing temukan. Lihat [Tonton run](#watch-the-run) untuk set kontrol lengkap.

    Anda juga dapat menonton dari panel tugas di bawah kotak input: ringkasan kemajuan satu baris muncul di sana saat run sedang berjalan. Tekan panah bawah untuk fokus, lalu Enter untuk memperluas.
  </Step>

  <Step title="Baca laporan">
    Ketika run selesai, laporan mendarat di sesi Anda. Ini mengutip sumber setiap klaim berasal, dengan klaim yang tidak bertahan pemeriksaan silang sudah disaring.

    Mulai dari v2.1.196, ketika agen verifikasi tidak dapat memeriksa klaim, seperti setelah batas laju atau kesalahan API, laporan mencantumkan klaim tersebut sebagai tidak terverifikasi daripada menghitungnya sebagai dibantah.
  </Step>
</Steps>

Untuk menjalankan alur kerja untuk tugas Anda sendiri, [biarkan Claude menulis satu](#have-claude-write-a-workflow), dan setelah run melakukan apa yang Anda inginkan, Anda dapat [menyimpannya](#save-the-workflow-for-reuse) sebagai perintah Anda sendiri.

<h3 id="bundled-workflows">
  Alur kerja bundel
</h3>

Claude Code menyertakan `/deep-research` sebagai alur kerja bawaan:

| Perintah                    | Apa yang dilakukannya                                                                                                                                                                                                                                                                                                              |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Menyebarkan pencarian web pada pertanyaan di beberapa sudut, mengambil dan memeriksa silang sumber yang ditemukannya, memilih setiap klaim, dan mengembalikan laporan yang dikutip dengan klaim yang tidak bertahan pemeriksaan silang disaring. Memerlukan [alat WebSearch](/docs/id/tools-reference#websearch-tool-behavior) tersedia |

[Alur kerja yang Anda simpan](#save-the-workflow-for-reuse) sendiri menjadi perintah dengan cara yang sama dan muncul dalam `/` autocomplete bersama yang bundel.

<h3 id="watch-the-run">
  Tonton run
</h3>

Alur kerja berjalan di latar belakang, jadi sesi tetap responsif sementara agen bekerja. Jalankan `/workflows` kapan saja untuk membuat daftar alur kerja yang sedang berjalan dan selesai, lalu pilih satu untuk membuka tampilan kemajuannya.

```text theme={null}
/workflows
```

Tampilan kemajuan menunjukkan setiap fase dengan jumlah agen, total token, dan waktu yang telah berlalu. Footer mencantumkan kunci untuk setiap tindakan:

| Kunci            | Tindakan                                                                                                                              |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓`        | Pilih fase atau agen                                                                                                                  |
| `Enter` atau `→` | Bor ke dalam fase yang dipilih, lalu ke agen untuk membaca prompt, panggilan alat terbaru, dan hasilnya                               |
| `Esc` atau `←`   | Kembali satu level. Dalam v2.1.203 hingga v2.1.205, `←` tidak melangkah keluar dari fase atau agen; gunakan `Esc` pada versi tersebut |
| `j` / `k`        | Gulir dalam detail agen ketika meluap                                                                                                 |
| `f`              | Filter daftar agen di fase yang dipilih berdasarkan status. Tekan lagi untuk siklus                                                   |
| `p`              | Jeda atau lanjutkan run                                                                                                               |
| `x`              | Hentikan agen yang dipilih, atau hentikan seluruh alur kerja ketika fokus ada di run                                                  |
| `r`              | Mulai ulang agen yang sedang berjalan yang dipilih                                                                                    |
| `s`              | [Simpan](#save-the-workflow-for-reuse) skrip run sebagai perintah                                                                     |

<h2 id="have-claude-write-a-workflow">
  Biarkan Claude menulis alur kerja
</h2>

Anda dapat membiarkan Claude menulis alur kerja untuk tugas Anda dengan dua cara:

* [Minta alur kerja dalam prompt Anda](#ask-for-a-workflow-in-your-prompt) dengan kata kunci `ultracode`, dan Claude menulis satu untuk tugas tersebut.
* [Biarkan Claude memutuskan dengan ultracode](#let-claude-decide-with-ultracode): atur `/effort ultracode` dan Claude merencanakan alur kerja untuk setiap tugas substansial dalam sesi.

Anda juga dapat menjalankan perintah alur kerja yang sudah ada: alur kerja [bundel](#bundled-workflows) seperti `/deep-research`, atau satu yang telah Anda [simpan](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Minta alur kerja dalam prompt Anda
</h3>

Untuk menjalankan satu tugas sebagai alur kerja tanpa mengubah tingkat upaya sesi, sertakan kata kunci `ultracode` dalam prompt Anda. Meminta dengan kata-kata Anda sendiri, misalnya "gunakan alur kerja" atau "jalankan alur kerja", juga berfungsi: Claude memperlakukan permintaan langsung sebagai opt-in yang sama. Sebelum v2.1.160 kata kunci pemicu literal adalah `workflow`; permintaan bahasa alami berfungsi di kedua versi.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code menyoroti kata kunci dalam input Anda dan Claude menulis skrip alur kerja untuk tugas daripada mengerjakannya giliran demi giliran. Jika Anda tidak bermaksud memulai alur kerja, tekan `Option+W` di macOS atau `Alt+W` di Windows dan Linux untuk menghilangkan sorotan untuk prompt ini, atau tekan backspace saat kursor berada tepat setelah kata kunci yang disorot. Untuk menghentikan kata kunci agar tidak memicu sama sekali, matikan pemicu kata kunci Ultracode di `/config`.

Jika run melakukan apa yang Anda inginkan, Anda dapat [menyimpannya sebagai perintah](#save-the-workflow-for-reuse) setelahnya.

Jika Anda sudah memiliki orchestrator yang dibangun dengan cara lain, seperti folder prompt subagen atau skill yang menyebarkan pekerjaan, Anda dapat menunjukkan Claude ke sana dan meminta alur kerja yang melakukan hal yang sama.

<h3 id="let-claude-decide-with-ultracode">
  Biarkan Claude memutuskan dengan ultracode
</h3>

Ultracode adalah pengaturan Claude Code yang menggabungkan upaya [reasoning](/docs/id/model-config#adjust-effort-level) `xhigh` dengan orkestrasi alur kerja otomatis. Dengan itu aktif, Claude merencanakan alur kerja untuk setiap tugas substansial daripada menunggu Anda untuk meminta.

```text theme={null}
/effort ultracode
```

Untuk memulai sesi dengan ultracode sudah aktif, luncurkan dengan `claude --effort ultracode`. Memerlukan Claude Code v2.1.203 atau lebih baru.

Dengan ultracode aktif, Claude memutuskan kapan tugas memerlukan alur kerja. Satu permintaan dapat berubah menjadi beberapa alur kerja berturut-turut: satu untuk memahami kode, satu untuk membuat perubahan, dan satu untuk memverifikasinya. Ini berlaku untuk setiap tugas dalam sesi, jadi setiap permintaan menggunakan lebih banyak token dan memakan waktu lebih lama daripada pada tingkat upaya yang lebih rendah.

Ultracode berlangsung untuk sesi saat ini dan disetel ulang ketika Anda memulai yang baru. Turun kembali dengan `/effort high` ketika Anda kembali ke pekerjaan rutin. Ini tersedia di model yang mendukung upaya `xhigh` [effort](/docs/id/model-config#adjust-effort-level); di model lain menu `/effort` tidak menawarkannya.

<h3 id="approve-the-plan-before-it-runs">
  Setujui rencana sebelum berjalan
</h3>

Di CLI, prompt per-run menunjukkan fase yang direncanakan dan opsi ini:

* **Yes, run it**: mulai run
* **Yes, and don't ask again for `<name>` in `<path>`**: mulai, dan lewati prompt ini untuk alur kerja ini di proyek ini dari sekarang
* **View raw script**: baca skrip sebelum memutuskan
* **No**: batal

`Ctrl+G` membuka skrip di editor Anda. `Tab` memungkinkan Anda menyesuaikan prompt sebelum run dimulai.

Apakah Anda melihat prompt ini tergantung pada [mode izin](/docs/id/permission-modes) Anda:

| Mode izin                                  | Kapan Anda diminta                                                                                                                                                                  |
| :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Default, accept edits                      | Setiap run, kecuali Anda telah memilih **Yes, and don't ask again** untuk alur kerja itu di proyek ini                                                                              |
| Auto                                       | Peluncuran pertama saja. Setiap **Yes** mencatat persetujuan dalam pengaturan pengguna Anda, dan peluncuran nanti dimulai tanpa meminta. Dilewati sepenuhnya ketika ultracode aktif |
| Bypass permissions, `claude -p`, Agent SDK | Tidak pernah. Run dimulai segera                                                                                                                                                    |

Di aplikasi Desktop, kartu persetujuan menunjukkan nama alur kerja, daftar fase, dan peringatan penggunaan token, dengan tindakan **Once**, **Always**, dan **Deny**. Tampilan kemajuan muncul di panel tugas Latar Belakang.

Mode izin Anda hanya mengontrol prompt peluncuran di atas. Subagen yang dihasilkan alur kerja selalu berjalan dalam mode `acceptEdits` dan mewarisi [allowlist alat](/docs/id/settings#permission-settings) Anda, terlepas dari mode sesi Anda. Edit file disetujui secara otomatis.

Perintah shell, pengambilan web, dan alat MCP yang tidak ada dalam allowlist Anda masih dapat meminta Anda di tengah run. Untuk menghindari ini pada run yang panjang, tambahkan perintah yang dibutuhkan agen ke allowlist Anda sebelum memulai.

Di `claude -p` dan Agent SDK tidak ada orang untuk diminta, jadi panggilan alat mengikuti aturan izin yang dikonfigurasi tanpa konfirmasi interaktif.

<h3 id="save-the-workflow-for-reuse">
  Simpan alur kerja untuk digunakan kembali
</h3>

Ketika Claude menulis alur kerja untuk tugas yang akan Anda ulangi, Anda dapat menyimpan skrip run itu sebagai perintah. Proses seperti tinjauan yang Anda jalankan di setiap cabang kemudian menjalankan orkestrasi yang sama setiap kali.

Jalankan `/workflows`, pilih run yang ingin Anda simpan, dan tekan `s`. Dalam dialog simpan, Tab beralih antara dua lokasi simpan:

* `.claude/workflows/` di proyek Anda: dibagikan dengan semua orang yang mengkloning repo
* `~/.claude/workflows/` di direktori home Anda: tersedia di setiap proyek, hanya terlihat oleh Anda. Jika Anda menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars), lokasi ini adalah direktori `workflows/` di bawah jalur itu.

Dialog simpan menunjukkan jalur yang diselesaikan untuk lokasi pribadi. Sebelum v2.1.208, itu menunjukkan `~/.claude/workflows/` bahkan ketika `CLAUDE_CONFIG_DIR` ditetapkan; file masih disimpan di bawah direktori yang dikonfigurasi.

Tekan Enter untuk menyimpan. Alur kerja berjalan sebagai `/<name>` di sesi mendatang dari lokasi mana pun.

Dalam monorepo dengan beberapa direktori `.claude/`, Anda dapat menyimpan alur kerja di samping paket yang mereka terapkan. Mulai dari v2.1.178, menyimpan ke lokasi proyek menulis ke direktori `.claude/workflows/` terdekat yang sudah ada antara direktori kerja Anda dan akar repositori, atau ke akar repositori jika belum ada. Alur kerja proyek juga dimuat dari setiap `.claude/workflows/` di sepanjang jalur itu, dan ketika lebih dari satu mendefinisikan nama yang sama Claude Code menjalankan yang terdekat dengan direktori kerja.

Jika alur kerja proyek dan alur kerja pribadi berbagi nama, yang proyek berjalan.

<h3 id="pass-input-to-a-saved-workflow">
  Teruskan input ke alur kerja yang disimpan
</h3>

Alur kerja yang disimpan dapat menerima input melalui parameter `args`. Skrip membacanya sebagai global bernama `args`. Gunakan ini untuk menyediakan pertanyaan penelitian, daftar jalur target, atau objek konfigurasi pada waktu pemanggilan daripada mengedit skrip untuk setiap run.

Prompt berikut menjalankan alur kerja yang disimpan dengan daftar nomor masalah:

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
```

Claude meneruskan daftar sebagai data terstruktur, sehingga skrip dapat memanggil metode array dan objek pada `args` secara langsung tanpa menguraikannya terlebih dahulu. Jika `args` dihilangkan, global adalah `undefined` di dalam skrip.

<h2 id="example-workflow-prompts">
  Contoh prompt alur kerja
</h2>

Alur kerja paling cocok ketika tugas lebih besar daripada yang dapat dipegang satu agen dalam konteks, atau ketika langkah yang sama perlu berjalan di banyak item. Prompt di bawah menunjukkan bentuk umum. Masing-masing meminta Claude untuk menulis dan menjalankan alur kerja untuk tugas itu; Anda tidak menulis skrip sendiri.

<h3 id="audit-many-files-for-the-same-issue">
  Audit banyak file untuk masalah yang sama
</h3>

Sebarkan satu agen per file, lalu kumpulkan dan verifikasi temuan.

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Terus memperbaiki sampai pemeriksaan lulus
</h3>

Jalankan pemeriksa, perbaiki apa yang gagal, dan ulangi sampai lulus atau berhenti membuat kemajuan.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrasi banyak file secara paralel
</h3>

Temukan file untuk migrasi, ubah masing-masing dalam salinan terisolasi sehingga edit tidak bertentangan, dan verifikasi setiap hasil.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Tinjau setiap file yang berubah dan tulis satu ringkasan
</h3>

Jalankan peninjau per file, lalu serahkan semua temuan ke satu agen yang mengurutkan dan menghilangkan duplikat.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Teliti topik di banyak sumber
</h3>

Sebarkan pembaca di seluruh changelog, masalah, dan dokumen, lalu sintesis. Alur kerja `/deep-research` bundel melakukan ini; Anda juga dapat menjelaskan versi yang lebih sempit.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Temukan masalah sampai daftar berhenti tumbuh
</h3>

Terus cari dalam putaran dan berhenti ketika putaran baru tidak menemukan apa pun yang baru.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  Apa yang terlihat seperti skrip yang disimpan
</h3>

Ketika Anda [menyimpan alur kerja](#save-the-workflow-for-reuse), file di `.claude/workflows/` memegang blok `meta` diikuti oleh badan skrip yang mengorkestrasi subagen. Anda biasanya tidak perlu mengeditnya, tetapi di sini adalah bentuk yang kecil sehingga Anda dapat mengenali apa yang dihasilkan Claude:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

Badan adalah JavaScript biasa dengan `await` tingkat atas. `agent()` menghasilkan satu subagen dan `pipeline()` menjalankan satu per item dalam daftar. Jika Anda ingin mengedit skrip dengan tangan, minta Claude untuk memandu Anda melalui perubahan, atau lihat entri alat Workflow dalam [referensi Agent SDK](/docs/id/agent-sdk/typescript) untuk set opsi lengkap.

<h2 id="how-a-workflow-runs">
  Bagaimana alur kerja berjalan
</h2>

Runtime alur kerja menjalankan skrip di lingkungan terisolasi, terpisah dari percakapan Anda. Hasil antara tetap dalam variabel skrip daripada mendarat di konteks Claude.

Setiap run menulis skripnya ke file di bawah direktori sesi Anda di `~/.claude/projects/`. Claude menerima jalur saat run dimulai, jadi Anda dapat memintanya. Anda dapat membuka file tersebut untuk membaca orkestrasi yang ditulis Claude, membandingkannya dengan skrip run sebelumnya, atau mengeditnya dan meminta Claude untuk meluncurkan kembali dari versi yang telah diedit.

Runtime melacak hasil setiap agen saat run berlangsung, yang membuat run [dapat dilanjutkan](#resume-after-a-pause) dalam sesi yang sama.

<h3 id="behavior-and-limits">
  Perilaku dan batas
</h3>

Runtime menerapkan batasan berikut:

| Batasan                                                                    | Mengapa                                                                                                                         |
| :------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| Tidak ada input pengguna di tengah run                                     | Hanya prompt izin agen yang dapat menjeda run. Untuk persetujuan antara tahap, jalankan setiap tahap sebagai alur kerja sendiri |
| Tidak ada akses filesystem atau shell langsung dari alur kerja itu sendiri | Agen membaca, menulis, dan menjalankan perintah. Skrip mengoordinasikan agen                                                    |
| Hingga 16 agen bersamaan, lebih sedikit di mesin dengan inti CPU terbatas  | Membatasi penggunaan sumber daya lokal                                                                                          |
| 1.000 agen total per run                                                   | Mencegah loop yang melarikan diri                                                                                               |

<h2 id="manage-runs">
  Kelola run
</h2>

Setelah run dimulai, Anda mengelolanya dari tampilan `/workflows`, atau dengan memperluas baris kemajuannya di panel tugas di bawah kotak input.

<h3 id="resume-after-a-pause">
  Lanjutkan setelah jeda
</h3>

Jika Anda menghentikan run, Anda dapat melanjutkannya: agen yang sudah selesai mengembalikan hasil cache mereka, dan sisanya berjalan langsung. Agen yang masih berjalan saat Anda menghentikan tidak disimpan dan dimulai ulang saat dilanjutkan, jadi alur kerja yang menyebarkan pekerjaan di banyak agen kecil mempertahankan lebih banyak kemajuan daripada satu agen panjang. Lanjutkan run yang dijeda dari `/workflows` dengan memilihnya dan menekan `p`, atau minta Claude untuk meluncurkan kembali alur kerja dengan skrip yang sama.

Lanjutkan bekerja dalam sesi Claude Code yang sama. Jika Anda keluar dari Claude Code saat alur kerja sedang berjalan, sesi berikutnya memulai alur kerja segar.

<h3 id="cost">
  Biaya
</h3>

Alur kerja menghasilkan banyak agen, jadi satu run dapat menggunakan token yang jauh lebih bermakna daripada menyelesaikan tugas yang sama dalam percakapan. Run dihitung terhadap penggunaan paket Anda dan batas laju seperti sesi lainnya.

Untuk mengukur pengeluaran sebelum berkomitmen pada tugas besar, jalankan alur kerja pada irisan kecil terlebih dahulu: satu direktori alih-alih seluruh repo, atau pertanyaan sempit alih-alih yang luas. Tampilan `/workflows` menunjukkan penggunaan token setiap agen saat run berlangsung, dan Anda dapat menghentikan run di sana kapan saja tanpa kehilangan pekerjaan yang selesai. [agent caps](#behavior-and-limits) runtime membatasi berapa banyak agen yang dapat dihasilkan satu run, yang membatasi biaya skrip yang lari. Untuk menjaga setiap run tetap lebih kecil secara default, [tetapkan panduan ukuran](#set-a-size-guideline) di `/config`.

Claude Code juga menandai run yang tumbuh secara tidak biasa besar. Ketika alur kerja menjadwalkan lebih dari 25 agen, atau total token proyeksiannya melampaui 1,5 juta, baris kemajuannya di panel tugas di bawah kotak input menampilkan peringatan `Large workflow`. Peringatan mengarahkan Anda ke [`/workflows`](#watch-the-run), di mana Anda dapat menghentikan run. Memerlukan Claude Code v2.1.203 atau lebih baru.

Peringatan bersifat informatif: tidak menghentikan atau membatasi run. Dua pengaturan berubah ketika Anda melihatnya:

* Jika Anda [menetapkan panduan ukuran](#set-a-size-guideline), jumlah agen panduan menggantikan ambang batas 25 agen.
* Sesi dengan [ultracode](#let-claude-decide-with-ultracode) aktif tidak menampilkan peringatan, karena mengaktifkan ultracode sudah memilih Anda untuk run besar.

Setiap agen dalam alur kerja menggunakan model sesi Anda kecuali skrip merutekan tahap ke yang berbeda atau variabel lingkungan [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/id/model-config#environment-variables) diatur, yang mengesampingkan keduanya. Untuk mengontrol biaya model:

* Periksa `/model` sebelum run besar jika Anda biasanya beralih ke model yang lebih kecil untuk pekerjaan rutin
* Minta Claude untuk menggunakan model yang lebih kecil untuk tahap yang tidak memerlukan yang terkuat ketika Anda menjelaskan tugas

<h3 id="set-a-size-guideline">
  Tetapkan panduan ukuran
</h3>

Pengaturan Dynamic workflow size di `/config` menjaga alur kerja yang ditulis Claude tetap dalam skala yang lebih kecil secara default. Claude Code mengirimkan pengaturan ke Claude sebagai saran, jadi prompt yang meminta skala berbeda masih akan menggantinya. Memerlukan Claude Code v2.1.202 atau lebih baru.

Setiap nilai menetapkan jumlah agen yang ditargetkan Claude dalam skrip yang ditulisnya.

| Nilai          | Panduan yang dikirim ke Claude         |
| :------------- | :------------------------------------- |
| `unrestricted` | Tidak ada panduan. Ini adalah default. |
| `small`        | Targetkan lebih dari 5 agen.           |
| `medium`       | Targetkan lebih dari 15 agen.          |
| `large`        | Targetkan lebih dari 50 agen.          |

Perubahan berlaku pada prompt berikutnya. [agent caps](#behavior-and-limits) runtime masih berlaku terlepas dari pengaturannya.

<h3 id="turn-workflows-off">
  Matikan alur kerja
</h3>

Alur kerja tersedia di CLI, aplikasi Desktop, ekstensi IDE, [mode non-interaktif](/docs/id/headless) dengan `claude -p`, dan [Agent SDK](/docs/id/agent-sdk/overview). Pengaturan disable yang sama berlaku di setiap permukaan.

Untuk mematikan alur kerja untuk diri sendiri:

* Matikan Dynamic workflows di `/config`. Bertahan di seluruh sesi.
* Atur `"disableWorkflows": true` di `~/.claude/settings.json`. Bertahan di seluruh sesi.
* Atur `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Dibaca saat startup, jadi berlaku di mana pun Anda mengaturnya.

Untuk mematikan alur kerja untuk seluruh organisasi Anda, atur `"disableWorkflows": true` di [pengaturan yang dikelola server](/docs/id/server-managed-settings), atau gunakan toggle di halaman [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).

Ketika alur kerja dinonaktifkan, perintah alur kerja bundel tidak tersedia, kata kunci `ultracode` tidak lagi memicu run, dan `ultracode` dihapus dari menu `/effort`.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Jalankan agen secara paralel](/docs/id/agents): bandingkan subagen, tampilan agen, tim agen, dan alur kerja
* [Buat subagen kustom](/docs/id/sub-agents): primitif pekerja yang diorkestrasikan alur kerja
* [Kelola biaya](/docs/id/costs): bagaimana run multi-agen dihitung terhadap batas penggunaan
