> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orkestrasi subagen dalam skala besar dengan alur kerja dinamis

> Alur kerja dinamis mengorkestrasi banyak subagen dari skrip yang ditulis Claude dan dapat Anda jalankan kembali. Gunakan untuk audit basis kode, migrasi besar, dan penelitian lintas-periksa.

<Note>
  Alur kerja dinamis tersedia di semua paket berbayar, dengan akses API Anthropic, dan di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Di Pro, aktifkan dari baris Dynamic workflows di `/config`.
</Note>

Alur kerja dinamis adalah skrip JavaScript yang mengorkestrasi banyak [subagen](/docs/id/sub-agents) sekaligus. Claude menulis skrip untuk tugas yang Anda jelaskan, dan runtime menjalankannya di latar belakang sementara sesi Anda tetap responsif.

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

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Izinkan alur kerja">
    Claude Code menanyakan apakah akan mengizinkan alur kerja. Pilih **Yes** untuk melanjutkan. Prompt yang tepat tergantung pada mode izin Anda. Lihat [Setujui rencana sebelum berjalan](#approve-the-plan-before-it-runs) untuk opsi per-mode.
  </Step>

  <Step title="Tonton kemajuan">
    Run dimulai di latar belakang. Jalankan `/workflows`, gunakan tombol panah untuk memilih run, dan tekan Enter untuk membuka tampilan kemajuannya:

    ```text wrap theme={null}
    /workflows
    ```

    Tampilan menunjukkan setiap fase dengan jumlah agen, total token, dan waktu yang telah berlalu. Bor ke dalam fase apa pun untuk melihat agennya dan apa yang masing-masing temukan. Lihat [Tonton run](#watch-the-run) untuk set kontrol lengkap.

    Anda juga dapat menonton dari panel tugas di bawah kotak input: ringkasan kemajuan satu baris muncul di sana saat run sedang berjalan. Tekan panah bawah untuk fokus, lalu Enter untuk memperluas.
  </Step>

  <Step title="Baca laporan">
    Ketika run selesai, laporan mendarat di sesi Anda. Ini mengutip sumber setiap klaim berasal, dengan klaim yang tidak bertahan pemeriksaan silang sudah disaring.

    Ketika agen verifikasi tidak dapat memeriksa klaim, seperti setelah batas laju atau kesalahan API, laporan mencantumkan klaim tersebut sebagai tidak terverifikasi daripada menghitungnya sebagai dibantah.
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

`/deep-research` berjalan hanya ketika Anda menjalankannya.

[Alur kerja yang Anda simpan](#save-the-workflow-for-reuse) sendiri menjadi perintah dengan cara yang sama dan muncul dalam `/` autocomplete bersama yang bundel.

<h3 id="watch-the-run">
  Tonton run
</h3>

Alur kerja berjalan di latar belakang, jadi sesi tetap responsif sementara agen bekerja. Jalankan `/workflows` kapan saja untuk membuat daftar alur kerja yang sedang berjalan dan selesai, lalu pilih satu untuk membuka tampilan kemajuannya.

Tampilan kemajuan menunjukkan setiap fase dengan jumlah agen, total token, dan waktu yang telah berlalu. Footer mencantumkan kunci untuk setiap tindakan:

| Kunci            | Tindakan                                                                                                                              |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓`        | Pilih fase atau agen                                                                                                                  |
| `Enter` atau `→` | Bor ke dalam fase yang dipilih, lalu ke detail agen. Dalam detail, `Enter` memperluas atau meruntuhkannya                             |
| `Esc` atau `←`   | Kembali satu level. Dalam v2.1.203 hingga v2.1.205, `←` tidak melangkah keluar dari fase atau agen; gunakan `Esc` pada versi tersebut |
| `j` / `k`        | Gulir dalam detail agen ketika meluap                                                                                                 |
| `f`              | Filter daftar agen di fase yang dipilih berdasarkan status. Tekan lagi untuk siklus                                                   |
| `p`              | Jeda atau lanjutkan run                                                                                                               |
| `x`              | Hentikan agen yang dipilih, atau hentikan seluruh alur kerja ketika fokus ada di run                                                  |
| `r`              | Mulai ulang agen yang sedang berjalan yang dipilih                                                                                    |
| `s`              | [Simpan](#save-the-workflow-for-reuse) skrip run sebagai perintah                                                                     |

Detail agen mencantumkan prompt agen, panggilan alat terakhirnya, dan hasilnya. Setiap panggilan menunjukkan statusnya, seperti masih berjalan atau gagal. Ketika agen menyimpan daftar tugas miliknya sendiri, detail menunjukkannya juga, dengan status setiap tugas.

Tekan `Enter` untuk memperluas detail. Prompt dan hasil kemudian ditampilkan sepenuhnya, dan setiap panggilan yang tercantum menunjukkan input dan awal hasilnya.

<h2 id="have-claude-write-a-workflow">
  Biarkan Claude menulis alur kerja
</h2>

Anda dapat membiarkan Claude menulis alur kerja untuk tugas Anda dengan dua cara:

* [Minta alur kerja dalam prompt Anda](#ask-for-a-workflow-in-your-prompt), baik dengan kata-kata Anda sendiri atau dengan menyertakan kata kunci `ultracode`, dan Claude menulis satu untuk tugas tersebut.
* [Biarkan Claude memutuskan dengan ultracode](#let-claude-decide-with-ultracode): atur `/effort ultracode` dan Claude merencanakan alur kerja untuk setiap tugas substansial dalam sesi.

Anda juga dapat menjalankan perintah alur kerja yang sudah ada: alur kerja [bundel](#bundled-workflows) seperti `/deep-research`, atau satu yang telah Anda [simpan](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Minta alur kerja dalam prompt Anda
</h3>

Untuk menjalankan satu tugas sebagai alur kerja tanpa mengubah tingkat upaya sesi, sertakan kata kunci `ultracode` dalam prompt Anda. Meminta dengan kata-kata Anda sendiri, misalnya "gunakan alur kerja" atau "jalankan alur kerja", juga berfungsi: Claude memperlakukan permintaan langsung sebagai opt-in yang sama.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code menyoroti kata kunci dalam input Anda dan Claude menulis skrip alur kerja untuk tugas daripada mengerjakannya giliran demi giliran. Kata kunci hanya memilih cara Claude menyusun pekerjaan: panggilan alat agen menerima pemeriksaan izin yang sama dan [sandboxing](/docs/id/sandboxing) seperti panggilan alat lainnya dalam sesi.

Jika run melakukan apa yang Anda inginkan, Anda dapat [menyimpannya sebagai perintah](#save-the-workflow-for-reuse) setelahnya. Jika Anda sudah memiliki orchestrator yang dibangun dengan cara lain, seperti folder prompt subagen atau skill yang menyebarkan pekerjaan, Anda dapat menunjukkan Claude ke sana dan meminta alur kerja yang melakukan hal yang sama.

<h4 id="dismiss-or-turn-off-the-keyword">
  Hilangkan atau matikan kata kunci
</h4>

Jika Anda tidak bermaksud memulai alur kerja, tekan `Option+W` di macOS atau `Alt+W` di Windows dan Linux untuk menghilangkan sorotan untuk prompt ini, atau tekan backspace saat kursor berada tepat setelah kata kunci yang disorot. Untuk menghentikan kata kunci agar tidak memicu sama sekali, matikan pemicu kata kunci Ultracode di `/config`.

<h4 id="where-the-keyword-works">
  Tempat kata kunci bekerja
</h4>

Kata kunci adalah opt-in hanya dalam prompt yang Anda ketik sendiri: di prompt interaktif, di panel ekstensi IDE, di klien [Remote Control](/docs/id/remote-control), atau di aplikasi Agent SDK yang memberi stempel pada input keyboard Anda dengan [`origin`](/docs/id/agent-sdk/typescript#sdkmessageorigin) sebagai `{ kind: "human" }`. Ini tidak memulai alur kerja ketika mencapai sesi dengan cara lain:

* prompt yang dilewatkan dengan `-p`
* prompt yang dikirim aplikasi Agent SDK tanpa memberi stempel sebagai input manusia
* prompt tugas terjadwal
* payload webhook atau komentar pull request yang disalurkan ke percakapan

<Note>
  Sebelum v2.1.210, kata kunci memulai alur kerja dari salah satu rute ini juga, termasuk payload webhook atau komentar pull request yang disalurkan ke percakapan.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Biarkan Claude memutuskan dengan ultracode
</h3>

Ultracode adalah pengaturan Claude Code yang menggabungkan upaya [reasoning](/docs/id/model-config#adjust-effort-level) `xhigh` dengan orkestrasi alur kerja otomatis. Dengan itu aktif, Claude merencanakan alur kerja untuk setiap tugas substansial daripada menunggu Anda untuk meminta.

```text wrap theme={null}
/effort ultracode
```

Untuk memulai sesi dengan ultracode sudah aktif, luncurkan dengan `claude --effort ultracode`. Memerlukan Claude Code v2.1.203 atau lebih baru.

Untuk mengaktifkannya saat Anda memilih model, pindahkan penggeser upaya pemilih `/model` ke `ultracode` dengan tombol panah. [Sesuaikan tingkat upaya](/docs/id/model-config#adjust-effort-level) mencantumkan rute yang mengaktifkan ultracode.

Dengan ultracode aktif, Claude memutuskan kapan tugas memerlukan alur kerja. Satu permintaan dapat berubah menjadi beberapa alur kerja berturut-turut: satu untuk memahami kode, satu untuk membuat perubahan, dan satu untuk memverifikasinya. Ini berlaku untuk setiap tugas dalam sesi, jadi setiap permintaan menggunakan lebih banyak token dan memakan waktu lebih lama daripada pada tingkat upaya yang lebih rendah.

`/effort ultracode` berlangsung untuk sesi saat ini; untuk memiliki setiap sesi dimulai dengan itu, atur pengaturan [`ultracode`](/docs/id/settings-reference#ultracode). Turun kembali dengan `/effort high` ketika Anda kembali ke pekerjaan rutin. Menu `/effort` menawarkannya hanya [ketika ultracode tersedia](/docs/id/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Setujui rencana sebelum berjalan
</h3>

Di CLI, prompt per-run menunjukkan fase yang direncanakan dan opsi ini:

* **Yes, run it**: mulai run
* **Yes, and don't ask again for `<name>` in `<path>`**: mulai, dan lewati prompt ini untuk alur kerja ini di proyek ini dari sekarang. Claude Code menawarkan opsi ini ketika Anda menjalankan alur kerja bundel, disimpan, atau plugin berdasarkan nama, bukan untuk skrip yang Claude tulis untuk tugas saat ini.
* **View raw script**: baca skrip sebelum memutuskan
* **No**: batal

`Ctrl+G` membuka skrip di editor Anda. `Tab` memungkinkan Anda menyesuaikan prompt sebelum run dimulai.

Apakah Anda melihat prompt ini tergantung pada [mode izin](/docs/id/permission-modes) Anda:

| Mode izin              | Kapan Anda diminta                                                                                                                                                                  |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Auto                   | Peluncuran pertama saja. Setiap **Yes** mencatat persetujuan dalam pengaturan pengguna Anda, dan peluncuran nanti dimulai tanpa meminta. Dilewati sepenuhnya ketika ultracode aktif |
| Manual, accept edits   | Setiap run, kecuali Anda telah memilih **Yes, and don't ask again** untuk alur kerja itu di proyek ini                                                                              |
| Bypass permissions     | Claude Code tidak meminta Anda. Run dimulai segera                                                                                                                                  |
| `claude -p`, Agent SDK | Claude Code tidak meminta Anda                                                                                                                                                      |

Di `claude -p` dan Agent SDK, Claude Code tidak pernah menampilkan prompt ini. Ini menjalankan panggilan alat Workflow melalui [evaluasi izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) yang sama seperti sisa sesi, jadi aturan deny, aturan ask, dan mode `dontAsk` berlaku untuk peluncuran seperti yang berlaku untuk setiap panggilan alat. Untuk membiarkan alur kerja dimulai dalam run ini, gunakan salah satu dari ini:

* **Aturan izin**: `Workflow` dalam aturan allow Anda menyetujui setiap alur kerja, dan `Workflow(<name>)` menyetujui satu alur kerja yang disimpan berdasarkan nama.
* **Mode izin auto**: [classifier](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) meninjau panggilan dan dapat menyetujuinya.
* **Mode bypass permissions**: Claude Code menyetujui panggilan.
* **Hook `PreToolUse`**: [hook](/docs/id/hooks#pretooluse) yang mengembalikan `allow` untuk panggilan menyetujuinya.
* **Host Anda**: [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags) menyetujuinya, atau, dengan Agent SDK, callback [`canUseTool`](/docs/id/agent-sdk/permissions) atau hook [`PermissionRequest`](/docs/id/hooks#permissionrequest) menyetujuinya.

Di aplikasi Desktop, kartu persetujuan menunjukkan nama alur kerja, daftar fase, dan peringatan penggunaan token, dengan tindakan **Once**, **Always**, dan **Deny**. Tampilan kemajuan muncul di panel tugas Latar Belakang.

Subagen yang dihasilkan alur kerja menggunakan [aturan izin](/docs/id/settings-reference#permission-settings) Anda, dan Claude Code memilih mode izin mereka berdasarkan aturan di bawah [mode izin mana yang dijalankan subagen](/docs/id/sub-agents#permission-modes). Untuk menghindari prompt pada run yang panjang, tambahkan alat yang dibutuhkan agen ke aturan allow Anda sebelum memulai.

<h3 id="save-the-workflow-for-reuse">
  Simpan alur kerja untuk digunakan kembali
</h3>

Ketika Claude menulis alur kerja untuk tugas yang akan Anda ulangi, Anda dapat menyimpan skrip run itu sebagai perintah. Proses seperti tinjauan yang Anda jalankan di setiap cabang kemudian menjalankan orkestrasi yang sama setiap kali.

Jalankan `/workflows`, pilih run yang ingin Anda simpan, dan tekan `s`. Dalam dialog simpan, Tab beralih antara dua lokasi simpan:

* `.claude/workflows/` di proyek Anda: dibagikan dengan semua orang yang mengkloning repo
* `~/.claude/workflows/` di direktori home Anda: tersedia di setiap proyek, hanya terlihat oleh Anda. Jika Anda menetapkan [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars), lokasi ini adalah direktori `workflows/` di bawah jalur itu.

Dialog simpan menunjukkan jalur yang diselesaikan untuk lokasi pribadi.

Tekan Enter untuk menyimpan. Alur kerja berjalan sebagai `/<name>` di sesi mendatang dari lokasi mana pun.

Claude Code memeriksa lokasi simpan untuk symlink sebelum menulis, dan menampilkan kesalahan daripada menulis melalui satu. Apa yang diperiksa tergantung pada tempat Anda menyimpan:

* Lokasi proyek: Claude Code menolak jika `.claude`, `.claude/workflows`, atau file target adalah symlink.
* Lokasi pribadi: Claude Code hanya menolak jika file target itu sendiri adalah symlink, jadi direktori `~/.claude` yang dikelola oleh alat dotfiles masih berfungsi.

Sebelum v2.1.216, Claude Code mengikuti tautan, yang dapat menempatkan file di luar lokasi yang Anda pilih.

Dalam monorepo dengan beberapa direktori `.claude/`, Anda dapat menyimpan alur kerja di samping paket yang mereka terapkan. Menyimpan ke lokasi proyek menulis ke direktori `.claude/workflows/` terdekat yang sudah ada antara direktori kerja Anda dan akar repositori, atau ke akar repositori jika belum ada. Alur kerja proyek juga dimuat dari setiap `.claude/workflows/` di sepanjang jalur itu, dan ketika lebih dari satu mendefinisikan nama yang sama Claude Code menjalankan yang terdekat dengan direktori kerja.

Jika alur kerja proyek dan alur kerja pribadi berbagi nama, yang proyek berjalan.

<h3 id="distribute-a-workflow-in-a-plugin">
  Distribusikan alur kerja dalam plugin
</h3>

Untuk berbagi alur kerja di seluruh tim atau repositori, sertakan dalam [plugin](/docs/id/plugins/overview). Tempatkan skrip dalam direktori `workflows/` di akar plugin, atau arahkan ke lokasi berbeda dengan [field manifes `workflows`](/docs/id/plugins/manifest-reference#fields).

Alur kerja plugin diberi namespace oleh nama plugin. Plugin yang disebut `acme-tools` yang berisi skrip yang `meta.name` adalah `release-audit` berjalan sebagai `/acme-tools:release-audit`.

<h3 id="pass-input-to-a-saved-workflow">
  Teruskan input ke alur kerja yang disimpan
</h3>

Alur kerja yang disimpan dapat menerima input melalui parameter `args`. Skrip membacanya sebagai global bernama `args`. Gunakan ini untuk menyediakan pertanyaan penelitian, daftar jalur target, atau objek konfigurasi pada waktu pemanggilan daripada mengedit skrip untuk setiap run.

Prompt berikut menjalankan alur kerja yang disimpan dengan daftar nomor masalah:

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
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

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Terus memperbaiki sampai pemeriksaan lulus
</h3>

Jalankan pemeriksa, perbaiki apa yang gagal, dan ulangi sampai lulus atau berhenti membuat kemajuan.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrasi banyak file secara paralel
</h3>

Temukan file untuk migrasi, ubah masing-masing dalam salinan terisolasi sehingga edit tidak bertentangan, dan verifikasi setiap hasil.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Tinjau setiap file yang berubah dan tulis satu ringkasan
</h3>

Jalankan peninjau per file, lalu serahkan semua temuan ke satu agen yang mengurutkan dan menghilangkan duplikat.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Teliti topik di banyak sumber
</h3>

Sebarkan pembaca di seluruh changelog, masalah, dan dokumen, lalu sintesis. Alur kerja `/deep-research` bundel melakukan ini; Anda juga dapat menjelaskan versi yang lebih sempit.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Temukan masalah sampai daftar berhenti tumbuh
</h3>

Terus cari dalam putaran dan berhenti ketika putaran baru tidak menemukan apa pun yang baru.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
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

Badan adalah JavaScript biasa dengan `await` tingkat atas. `agent()` menghasilkan satu subagen, `pipeline()` menjalankan satu per item dalam daftar, dan `parallel()` menjalankan serangkaian tugas agen pada waktu yang sama dan menunggu semuanya selesai.

Panggilan `agent()` diselesaikan ke `null` jika Anda menghentikannya di tengah-jalan atau mengalami kesalahan API yang tidak dapat dipulihkan. `pipeline()` menyimpan setiap `null` dalam array hasil, itulah mengapa contoh berakhir dengan `.filter(Boolean)` untuk menghapus entri tersebut.

Dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), prompt yang diteruskan skrip Anda ke `agent()` tidak dihitung sebagai permintaan dari Anda ketika pengklasifikasi meninjau tindakan subagen itu, karena Claude Code menandainya sebagai teks yang dihitung skrip.

Jika Anda melewatkan `schema` pada panggilan `agent()`, subagen itu mengembalikan JSON yang cocok dengan bentuk alih-alih prosa. Claude Code memeriksa skema sebelum memulai subagen: ketika dapat membuktikan skema bertentangan dengan dirinya sendiri, panggilan gagal dengan kesalahan yang menamai kontradiksi, dan subagen tidak pernah dimulai. Satu kontradiksi yang dapat dibuktikan adalah kunci `required` yang `additionalProperties: false` aturan keluar.

Jika output subagen masih gagal validasi setelah lima upaya, panggilan gagal dengan kesalahan yang mencakup kegagalan validasi terakhir. Untuk mengubah jumlah upaya, atur [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/id/env-vars).

<h3 id="edit-a-saved-script">
  Edit skrip yang disimpan
</h3>

Untuk mengubah [alur kerja yang Anda simpan](#save-the-workflow-for-reuse), edit file `.js` atau minta Claude untuk membuat perubahan. Sebelum Anda mengedit atau meminta, jalankan [skill bundel](/docs/id/skills#bundled-skills) `/workflow-authoring` untuk memuat referensi penulisan skrip yang digunakan Claude. Skill memerlukan Claude Code v2.1.248 atau lebih baru.

Untuk menjalankan versi yang telah diedit dalam sesi saat ini, jalankan [`/reload-skills`](/docs/id/commands#all-commands) untuk membaca ulang direktori alur kerja, lalu jalankan `/<name>` lagi.

Claude Code menerapkan aturan ini untuk setiap bagian file ketika memuat dan menjalankan skrip:

* **Blok `meta`**: pertahankan `export const meta` sebagai pernyataan pertama, dan pertahankan sebagai literal objek biasa dengan `name` dan `description`. Jika berisi apa pun selain nilai literal, seperti variabel, panggilan fungsi, atau spread, Claude Code menghapus `/<name>` dari pelengkapan otomatis `/`.
* **Badan**: selain `agent()`, `pipeline()`, dan `parallel()`, Anda dapat memanggil `phase()` untuk mengelompokkan agen yang mengikuti di bawah judul dalam tampilan kemajuan, memanggil `log()` untuk menampilkan pesan di atas fase, dan membaca global [`args`](#pass-input-to-a-saved-workflow). Jika badan memiliki kesalahan sintaks, Claude Code melaporkannya ketika Anda menjalankan alur kerja.
* **`phases`**: jika Anda mencantumkannya di `meta`, berikan setiap entri judul yang tepat yang Anda berikan ke `phase()`. Judul `phase()` tanpa entri mendapatkan grup kemajuan sendiri.
* **Stempel waktu dan keacakan**: Claude Code membuat `Date.now()`, `Math.random()`, dan `new Date()` tanpa argumen melempar di dalam skrip, sehingga [peluncuran ulang](#resume-after-a-pause) mengulangi panggilan `agent()` yang sama. Berikan stempel waktu melalui `args` sebagai gantinya.

Anda juga dapat mengedit [skrip dari satu run tunggal](#how-a-workflow-runs) daripada salinan yang disimpan. [Lanjutkan setelah jeda](#resume-after-a-pause) mencakup agen mana yang berjalan lagi ketika Anda meluncurkan ulang skrip yang telah diedit. Untuk input alat Workflow, lihat entrinya dalam [referensi Agent SDK](/docs/id/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Bagaimana workflow berjalan
</h2>

Runtime workflow mengeksekusi script dalam lingkungan terisolasi, terpisah dari percakapan Anda. Hasil antara tetap dalam variabel script daripada masuk ke konteks Claude.

Setiap run menulis scriptnya ke file di bawah direktori sesi Anda di `~/.claude/projects/`. Claude menerima path ketika run dimulai, jadi Anda dapat memintanya. Anda dapat membuka file tersebut untuk membaca orkestrasi yang ditulis Claude, membandingkannya dengan script run sebelumnya, atau mengeditnya dan meminta Claude untuk meluncurkan kembali dari versi yang telah diedit.

Claude hanya dapat memulai workflow dari file script yang sudah diizinkan dibaca oleh sesi. Untuk menjalankan script yang disimpan di luar direktori kerja Anda, tambahkan direktorinya terlebih dahulu dengan [`/add-dir`](/docs/id/permissions#working-directories) atau [aturan Read allow](/docs/id/permissions#read-and-edit).

Runtime melacak hasil setiap agent saat run berlangsung, yang membuat run [dapat dilanjutkan](#resume-after-a-pause) dalam sesi yang sama.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt caching dalam fan-out
</h3>

Agent dalam run yang sama dapat membaca [prompt cache](/docs/id/prompt-caching#subagents-and-the-cache) satu sama lain. Dua agent yang berjalan dengan model yang sama, tingkat effort yang sama, tipe agent yang sama, tools yang sama, output schema yang sama, dan direktori kerja yang sama membangun prefix tools-and-system-prompt yang sama, jadi agent yang dimulai setelah respons sibling yang cocok telah dimulai membaca cache sibling tersebut pada permintaan pertamanya.

Permintaan workflow agent jatuh di luar [cache TTL bucket](/docs/id/prompt-caching#which-ttl-each-request-gets) percakapan utama, jadi cachenya bertahan selama lima menit secara default, termasuk pada langganan Claude. Untuk menyimpannya selama satu jam, atur [`subagentPromptCacheTtl`](/docs/id/settings-reference#subagentpromptcachettl) ke `1h`. API menagih penulisan cache 1 jam dengan tarif yang lebih tinggi.

Ketika fan-out memulai beberapa agent yang cocok sekaligus, Claude Code menahan semua kecuali yang pertama sampai respons agent pertama dimulai, kemudian melepaskan agent yang ditahan bersama-sama sehingga permintaan pertama mereka membaca prefix bersama daripada masing-masing memproses tanpa cache. Claude Code membatasi penahan pada [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/id/env-vars) milidetik, `5000` secara default. Atur ke `0` untuk menonaktifkan penahan.

<h3 id="behavior-and-limits">
  Perilaku dan batasan
</h3>

Runtime menerapkan batasan berikut:

| Batasan                                                                                                                                                                                                                                                                                                                                                | Alasan                                                                                                                                                                                                         |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tidak ada input pengguna di tengah run                                                                                                                                                                                                                                                                                                                 | Run dijeda dengan sendirinya hanya untuk prompt izin agent dan [tunggu batas penggunaan](#when-a-run-hits-your-usage-limit). Untuk persetujuan antar tahap, jalankan setiap tahap sebagai workflow terpisahnya |
| Tidak ada akses filesystem atau shell langsung dari workflow itu sendiri                                                                                                                                                                                                                                                                               | Agent membaca, menulis, dan menjalankan perintah. Script mengoordinasikan agent                                                                                                                                |
| Tidak ada pemuatan modul: script yang berisi `import()` gagal sebelum run dimulai                                                                                                                                                                                                                                                                      | Badan script adalah JavaScript biasa. Letakkan pekerjaan yang memerlukan library dalam task agent                                                                                                              |
| Hingga 16 agent bersamaan secara default, lebih sedikit ketika Claude Code memiliki lebih sedikit CPU yang tersedia, termasuk di dalam container yang dibatasi CPU. Untuk mengubah batas, atur [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/id/env-vars#variables) ke nilai dari 1 hingga 256, yang memerlukan Claude Code v2.1.269 atau lebih baru | Membatasi penggunaan sumber daya lokal                                                                                                                                                                         |
| Dalam fan-out, agent yang berbagi prefix prompt-cache agent pertama dimulai hingga 5 detik setelahnya secara default                                                                                                                                                                                                                                   | Semua kecuali yang pertama membaca [prefix yang di-cache agent pertama](#prompt-caching-in-a-fan-out) daripada masing-masing memproses tanpa cache                                                             |
| Hingga 4.096 item dalam satu panggilan `parallel()` atau `pipeline()`: runtime menolak daftar yang lebih panjang dengan error                                                                                                                                                                                                                          | Batas diam akan menghilangkan bagian dari workload tanpa memberitahu script                                                                                                                                    |
| 1.000 agent total per run                                                                                                                                                                                                                                                                                                                              | Mencegah loop yang tidak terkontrol                                                                                                                                                                            |

<h2 id="manage-runs">
  Kelola run
</h2>

Setelah run dimulai, Anda mengelolanya dari tampilan `/workflows`, atau dengan memperluas baris kemajuannya di panel tugas di bawah kotak input.

Ketika Anda menghentikan run, itu tetap berada di panel tugas sementara proses agen apa pun masih berjalan. Jika Anda menghentikannya lagi, Claude Code mengirim sinyal ulang ke proses tersebut.

<h3 id="resume-after-a-pause">
  Lanjutkan setelah jeda
</h3>

Lanjutkan run yang dijeda dari `/workflows` dengan memilihnya dan menekan `p`. Untuk run yang Anda hentikan, minta Claude untuk meluncurkan kembali alur kerja dengan skrip yang sama. Jika agen dari run yang dihentikan belum keluar, Claude Code menolak peluncuran ulang sampai mereka keluar, jadi salinan kedua dari agen tersebut tidak dapat berjalan bersama mereka.

Claude Code memutar ulang run dalam urutan agen dimulai, dan setiap agen baik mengembalikan hasil yang disimpannya atau berjalan lagi:

* **Selesai**: mengembalikan hasil yang disimpannya. Agen pertama yang promptnya berbeda dari run sebelumnya, karena Anda mengedit skrip atau agen sebelumnya mengembalikan sesuatu yang berbeda, berjalan lagi, begitu juga setiap agen setelahnya, bahkan yang sudah selesai.
* **Masih berjalan saat Anda menghentikan**: dimulai ulang. Menghentikan seluruh run tidak menghitung agen apa pun sebagai gagal.
* **Gagal**: berjalan lagi, begitu juga setiap agen yang dimulai setelahnya, bahkan yang sudah selesai. Menghentikan satu agen saja, dengan memilihnya di [`/workflows`](#watch-the-run) dan menekan `x`, dihitung sebagai gagal.

Kasus terakhir berarti kegagalan di tengah fan-out yang menjalankan ulang pekerjaan yang sudah selesai. Jika skrip memulai A, B, C, dan D dalam urutan itu dan B gagal, meluncurkan ulang mengembalikan A dari cache dan menjalankan B, C, dan D lagi.

Anda dapat melanjutkan run dalam sesi Claude Code yang sama. Apa yang terjadi pada alur kerja yang berjalan saat Anda meninggalkan sesi tergantung pada cara Anda pergi:

* Jika Anda [menempatkan sesi di latar belakang](/docs/id/agent-view#what-carries-over-when-you-background), Claude Code memutar ulang run dengan cara yang sama di sesi latar belakang dan melanjutkannya.
* Jika Anda keluar dari Claude Code saat alur kerja sedang berjalan dan [tampilan agen aktif](/docs/id/agent-view#from-inside-a-session), dialog keluar menawarkan `Move to background and exit`, yang membawa run dengan cara yang sama. Jika Anda memilih `Exit and stop tasks` sebagai gantinya, atau opsi tidak ditawarkan, run berhenti dengan sesi. Claude Code menyimpan hasil yang disimpan run di bawah direktori sesi itu di `~/.claude/projects/`, jadi sesi yang Anda lanjutkan dengan `claude --resume` dapat memutar ulangnya saat Anda meminta Claude untuk meluncurkan kembali alur kerja. Dalam sesi yang Anda mulai segar, Claude tidak memiliki run sebelumnya untuk diluncurkan ulang dan memulai alur kerja dari awal sebagai run baru.

Dalam [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code juga menyimpan hasil run dengan riwayat percakapan sesi, yang bertahan saat VM sesi diambil kembali. Ketika Anda [membuka kembali sesi seperti itu](/docs/id/claude-code-on-the-web#environment-expired) dan meminta Claude untuk meluncurkan kembali alur kerja, agen yang selesai masih mengembalikan hasil yang disimpannya.

Dalam sesi lokal dan cloud, ketika Claude meluncurkan ulang run sebelumnya dan Claude Code tidak dapat menemukan hasil run yang disimpan sama sekali, peluncuran ulang gagal dengan kesalahan `nothing to resume` alih-alih memulai run dari awal. Minta Claude untuk memulai alur kerja dari awal sebagai run baru.

<h3 id="when-a-run-hits-your-usage-limit">
  Ketika run mencapai batas penggunaan Anda
</h3>

Ketika agen mencapai [batas penggunaan](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset) claude.ai Anda, run dijeda daripada agen itu gagal: agen yang mencapai batas menunggu reset, dan tidak ada agen baru yang dimulai. Tidak lama setelah batas direset, agen yang menunggu berjalan lagi dan run berlanjut dengan sendirinya. Memerlukan Claude Code v2.1.271 atau lebih baru; pada versi sebelumnya, agen yang terpengaruh gagal.

Saat run menunggu, baris kemajuannya di panel tugas dan header [`/workflows`](#watch-the-run) menunjukkan kapan batas direset.

Run hanya dijeda ketika semua ini berlaku; ketika salah satu tidak, agen yang terpengaruh gagal sebagai gantinya:

* Sesi bersifat interaktif dan masuk dengan langganan claude.ai. Run tidak dijeda dalam [mode non-interaktif](/docs/id/headless) dengan `claude -p` atau [Agent SDK](/docs/id/agent-sdk/overview), dalam [sesi latar belakang](/docs/id/agent-view), atau dalam sesi rekan [Remote Control](/docs/id/remote-control) atau [agent team](/docs/id/agent-teams).
* [`autoContinueAtUsageLimit`](/docs/id/settings-reference#autocontinueatusagelimit) aktif, pengaturan yang sama yang memungkinkan sesi itu sendiri [menunggu batas penggunaan direset](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset). Jika Anda mematikannya selama menunggu, menunggu berakhir dan agen yang menunggu gagal.
* Batas direset dalam 24 jam. Batas mingguan dapat direset lebih jauh.
* Run belum menunggu dua kali. Ketika mencapai batas untuk ketiga kalinya, agen gagal.

<h3 id="cost">
  Biaya
</h3>

Alur kerja menghasilkan banyak agen, jadi satu run dapat menggunakan token yang jauh lebih bermakna daripada menyelesaikan tugas yang sama dalam percakapan. Run dihitung terhadap penggunaan paket Anda dan batas laju.

Untuk mengukur pengeluaran sebelum berkomitmen pada tugas besar, jalankan alur kerja pada irisan kecil terlebih dahulu: satu direktori alih-alih seluruh repo, atau pertanyaan sempit alih-alih yang luas. Tampilan `/workflows` menunjukkan penggunaan token setiap agen saat run berlangsung, dan Anda dapat menghentikan run di sana kapan saja, biasanya tanpa kehilangan pekerjaan yang selesai. [Lanjutkan setelah jeda](#resume-after-a-pause) mencakup apa yang disimpan run yang dihentikan. [Agent caps](#behavior-and-limits) runtime membatasi berapa banyak agen yang dapat dihasilkan satu run, yang membatasi biaya skrip yang lari. Untuk menjaga run tetap lebih sedikit agen, pilih panduan ukuran `small` [](#set-a-size-guideline).

Claude Code juga menandai run yang tumbuh secara tidak biasa besar. Ketika alur kerja menjadwalkan lebih dari 25 agen, atau total token proyeksiannya melampaui 1,5 juta, baris kemajuannya di panel tugas di bawah kotak input menampilkan peringatan `Large workflow`. Peringatan mengarahkan Anda ke [`/workflows`](#watch-the-run), di mana Anda dapat menghentikan run.

Peringatan bersifat informatif: tidak menghentikan atau membatasi run. Dua pengaturan berubah saat Anda melihatnya:

* Jika Anda memilih [panduan ukuran](#set-a-size-guideline) sendiri, jumlah agen panduan menggantikan ambang batas 25 agen. Panduan default bawaan meninggalkan ambang batas di 25.
* Sesi dengan [ultracode](#let-claude-decide-with-ultracode) aktif tidak menampilkan peringatan, karena mengaktifkan ultracode sudah memilih Anda untuk run besar.

Claude Code memilih model agen alur kerja setiap dalam [urutan yang sama yang digunakan untuk subagen](/docs/id/sub-agents#choose-a-model). Model yang dinamai skrip untuk tahap dihitung sebagai model per-invocation dalam urutan itu. Ketika tidak ada yang lain menetapkan satu, agen berjalan pada model sesi Anda.

Untuk mengontrol biaya model:

* Periksa `/model` sebelum run besar jika Anda biasanya beralih ke model yang lebih kecil untuk pekerjaan rutin
* Minta Claude untuk menggunakan model yang lebih kecil untuk tahap yang tidak memerlukan yang terkuat saat Anda menjelaskan tugas

Ketika [`availableModels` allowlist](/docs/id/model-config#restrict-model-selection) organisasi Anda memblokir model yang diminta skrip untuk agen, agen itu berjalan pada model substitusi sebagai gantinya, mengikuti [aturan substitusi yang sama seperti subagen](/docs/id/sub-agents#choose-a-model). Tampilan kemajuan run di [`/workflows`](#watch-the-run) menampilkan peringatan yang menamai model yang diminta dan disubstitusi.

<h3 id="set-a-size-guideline">
  Tetapkan panduan ukuran
</h3>

Panduan ukuran memberi tahu Claude berapa banyak agen yang harus ditargetkan saat menulis alur kerja dinamis. Claude Code mengirimkan panduan ke Claude sebagai saran, bukan batas, jadi prompt yang meminta skala berbeda masih menggantinya. Memerlukan Claude Code v2.1.202 atau lebih baru.

Setiap nilai memetakan ke jumlah agen:

| Nilai          | Jumlah agen yang ditargetkan Claude                    |
| :------------- | :----------------------------------------------------- |
| `unrestricted` | Tidak ada panduan: Claude mengukur alur kerja ke tugas |
| `small`        | Lebih sedikit dari 5 agen                              |
| `medium`       | Lebih sedikit dari 10 agen                             |
| `large`        | Lebih sedikit dari 50 agen                             |

Default adalah `medium`, atau `small` ketika Anda masuk pada paket Pro dengan Claude Code v2.1.271 atau lebih baru. Sampai Anda memilih nilai, baris `/config` menampilkan nilai sebagai default, dan baris `Running in background` alur kerja menampilkan ukuran yang berlaku. Memerlukan Claude Code v2.1.219 atau lebih baru; versi sebelumnya default ke `unrestricted`.

Untuk mengubah panduan, pilih nilai untuk pengaturan Dynamic workflow size di `/config`, atau jalankan `/config workflowSizeGuideline=small`. Pada v2.1.219 dan lebih baru, Anda juga dapat mengatur kunci [`workflowSizeGuideline`](/docs/id/settings-reference#workflowsizeguideline) di file pengaturan apa pun; nilai itu mengambil prioritas atas `/config`, dan Claude Code menyembunyikan baris `/config` saat file pengaturan menyediakan satu.

Perubahan berlaku pada prompt berikutnya. [Agent caps](#behavior-and-limits) runtime masih berlaku terlepas dari pengaturannya.

<h3 id="turn-workflows-off">
  Matikan alur kerja
</h3>

Alur kerja tersedia di CLI, aplikasi Desktop, ekstensi IDE, [mode non-interaktif](/docs/id/headless) dengan `claude -p`, dan [Agent SDK](/docs/id/agent-sdk/overview). Pengaturan disable yang sama berlaku di setiap permukaan.

Untuk mematikan alur kerja untuk diri sendiri:

* Matikan Dynamic workflows di `/config`. Bertahan di seluruh sesi.
* Atur `"disableWorkflows": true` di `~/.claude/settings.json`. Bertahan di seluruh sesi.
* Atur `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Dibaca saat startup, jadi berlaku di mana pun Anda mengaturnya.

Untuk mematikan alur kerja untuk seluruh organisasi Anda, atur `"disableWorkflows": true` di [pengaturan yang dikelola](/docs/id/server-managed-settings), atau gunakan toggle di halaman [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code).

Ketika alur kerja dinonaktifkan, perintah alur kerja bundel dan skill `/workflow-authoring` tidak tersedia, kata kunci `ultracode` tidak lagi memicu run, dan `ultracode` dihapus dari menu `/effort`.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Jalankan agen secara paralel](/docs/id/agents): bandingkan subagen, tampilan agen, tim agen, dan alur kerja
* [Buat subagen kustom](/docs/id/sub-agents): primitif pekerja yang diorkestrasikan alur kerja
* [Kelola biaya](/docs/id/costs): bagaimana run multi-agen dihitung terhadap batas penggunaan
