> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pilih mode izin

> Kontrol apakah Claude meminta izin sebelum mengedit file atau menjalankan perintah. Siklus mode dengan Shift+Tab di CLI atau gunakan pemilih mode di VS Code, Desktop, dan claude.ai.

Ketika Claude ingin mengedit file, menjalankan perintah shell, atau membuat permintaan jaringan, ia berhenti dan meminta Anda untuk menyetujui tindakan tersebut. Mode izin mengontrol seberapa sering jeda itu terjadi. Mode yang Anda pilih membentuk alur sesi: mode manual membuat Anda meninjau setiap tindakan saat tiba, sementara mode yang lebih longgar memungkinkan Claude bekerja dalam peregangan yang lebih lama tanpa gangguan dan melaporkan kembali saat selesai. Pilih pengawasan lebih untuk pekerjaan sensitif, atau gangguan lebih sedikit ketika Anda mempercayai arahnya.

<h2 id="available-modes">
  Mode yang tersedia
</h2>

Setiap mode membuat tradeoff yang berbeda antara kenyamanan dan pengawasan. Tabel di bawah menunjukkan apa yang dapat dilakukan Claude tanpa prompt izin di setiap mode.

| Mode                                                                | Apa yang berjalan tanpa bertanya                                                              | Terbaik untuk                              |
| :------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------- | :----------------------------------------- |
| `default`                                                           | Pembacaan saja                                                                                | Memulai, pekerjaan sensitif                |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Pembacaan, pengeditan file, dan perintah filesystem umum (`mkdir`, `touch`, `mv`, `cp`, dll.) | Iterasi pada kode yang Anda tinjau         |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Pembacaan saja                                                                                | Menjelajahi basis kode sebelum mengubahnya |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Semuanya, dengan pemeriksaan keamanan latar belakang                                          | Tugas panjang, mengurangi kelelahan prompt |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Hanya alat yang telah disetujui sebelumnya                                                    | CI terkunci dan skrip                      |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Semuanya                                                                                      | Kontainer dan VM terisolasi saja           |

Mode yang meninjau setiap tindakan dinamai **Manual** di CLI, di `claude --help`, di ekstensi VS Code dan JetBrains, dan di aplikasi desktop. Nilai konfignya adalah `default`, yang digunakan oleh hooks dan integrasi SDK. CLI menerima `manual` sebagai alias di mana pun Anda mengetikkan nilainya, misalnya `claude --permission-mode manual` atau `"defaultMode": "manual"`. Label Manual dan alias `manual` memerlukan Claude Code v2.1.200 atau lebih baru. Label aplikasi desktop tidak bergantung pada versi CLI Anda.

Di setiap mode kecuali `bypassPermissions`, penulisan ke [jalur yang dilindungi](#protected-paths) tidak pernah disetujui otomatis, menjaga status repositori dan konfigurasi Claude sendiri dari kerusakan yang tidak disengaja.

Mode menetapkan baseline. Lapisi [aturan izin](/docs/id/permissions#manage-permissions) di atas untuk pre-approve atau memblokir alat tertentu. Aturan deny, aturan ask eksplisit, [pengaturan `ask` organisasi pada alat konektor](/docs/id/mcp#organization-controls-on-connector-tools), dan penanda [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) berlaku di setiap mode, termasuk `bypassPermissions`. Aturan allow tidak memiliki efek dalam mode itu karena semuanya sudah disetujui.

<h2 id="switch-permission-modes">
  Beralih mode izin
</h2>

Anda dapat beralih mode di tengah sesi, saat startup, atau sebagai default yang persisten. Mode diatur melalui kontrol ini, bukan dengan meminta Claude dalam obrolan. Pilih antarmuka Anda di bawah untuk melihat cara mengubahnya.

<Tabs>
  <Tab title="CLI">
    **Selama sesi**: tekan `Shift+Tab` untuk siklus `default` → `acceptEdits` → `plan`. Mode saat ini muncul di bilah status. Mode manual, `default` dalam siklus itu, menampilkan lencana abu-abu `⏸ manual mode on`. Sebelum v2.1.203, bilah status tidak menampilkan lencana dalam mode Manual.

    Tidak setiap mode ada dalam siklus default:

    * `auto`: muncul ketika akun Anda memenuhi [persyaratan mode auto](#eliminate-prompts-with-auto-mode); bersiklus ke dalamnya menampilkan mode tanpa prompt konfirmasi
    * `bypassPermissions`: muncul setelah Anda memulai dengan `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, atau `--allow-dangerously-skip-permissions`; varian `--allow-` menambahkan mode ke siklus tanpa mengaktifkannya
    * `dontAsk`: tidak pernah muncul dalam siklus; atur dengan `--permission-mode dontAsk`

    Mode opsional yang diaktifkan masuk setelah `plan`, dengan `bypassPermissions` terlebih dahulu dan `auto` terakhir. Jika Anda memiliki keduanya diaktifkan, Anda akan bersiklus melalui `bypassPermissions` dalam perjalanan ke `auto`.

    **Saat startup**: lewatkan mode sebagai flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Sebagai default**: atur `defaultMode` di [pengaturan](/docs/id/settings#settings-files).

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    Flag `--permission-mode` yang sama berfungsi dengan `-p` untuk [run non-interaktif](/docs/id/headless).
  </Tab>

  <Tab title="VS Code">
    **Selama sesi**: klik indikator mode di bagian bawah kotak prompt.

    **Sebagai default**: atur `claudeCode.initialPermissionMode` di pengaturan VS Code, atau gunakan panel pengaturan ekstensi Claude Code.

    Indikator mode menampilkan label ini, dipetakan ke mode yang masing-masing berlaku:

    | Label UI             | Mode                |
    | :------------------- | :------------------ |
    | Manual               | `default`           |
    | Edit secara otomatis | `acceptEdits`       |
    | Plan                 | `plan`              |
    | Auto                 | `auto`              |
    | Bypass permissions   | `bypassPermissions` |

    Sebelum v2.1.205, ekstensi melabeli `plan` sebagai Plan mode dan `auto` sebagai Auto mode.

    Mode auto muncul di indikator mode ketika akun Anda memenuhi setiap persyaratan yang tercantum di [bagian mode auto](#eliminate-prompts-with-auto-mode). Pengaturan `claudeCode.initialPermissionMode` tidak menerima `auto`. Untuk memulai dalam mode auto secara default, atur `defaultMode` di [pengaturan pengguna](/docs/id/settings#settings-files) Anda sebagai gantinya. Claude Code mengabaikan `defaultMode: "auto"` di pengaturan proyek dan lokal.

    Bypass permissions memerlukan toggle **Allow dangerously skip permissions** di pengaturan ekstensi sebelum muncul di indikator mode.

    Lihat [panduan VS Code](/docs/id/vs-code) untuk detail khusus ekstensi.
  </Tab>

  <Tab title="JetBrains">
    Plugin JetBrains menjalankan Claude Code di terminal IDE, jadi beralih mode berfungsi sama seperti di CLI: tekan `Shift+Tab` untuk bersiklus, atau lewatkan `--permission-mode` saat meluncurkan.
  </Tab>

  <Tab title="Desktop">
    **Selama sesi**: gunakan pemilih mode di sebelah tombol kirim. Tidak setiap mode muncul di pemilih:

    * **Auto**: muncul ketika akun Anda memenuhi [persyaratan mode auto](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: memerlukan toggle **Allow bypass permissions mode** di pengaturan Desktop pada paket Pro dan Max; pada paket Team dan Enterprise, kebijakan organisasi mengontrolnya sebagai gantinya

    Untuk detail khusus desktop, lihat [Pilih mode izin](/docs/id/desktop#choose-a-permission-mode) di panduan Desktop.

    **Sebagai default**: atur `defaultMode` di [pengaturan](/docs/id/settings#settings-files). Aplikasi desktop membaca file pengaturan yang sama seperti CLI dan menerapkan mode ke sesi lokal baru.

    Mode yang Anda pilih di pemilih mode diingat per folder dan mengambil alih `defaultMode` untuk folder itu. Plan adalah pengecualian: memilihnya berlaku hanya untuk sesi saat ini.

    Contoh ini menetapkan Plan mode sebagai default untuk sesi lokal baru:

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web dan mobile">
    Gunakan dropdown mode di sebelah kotak prompt di [claude.ai/code](https://claude.ai/code) atau di aplikasi mobile. Prompt izin muncul di claude.ai untuk persetujuan. Mode mana yang muncul tergantung di mana sesi berjalan:

    * **Sesi cloud** di [Claude Code on the web](/docs/id/claude-code-on-the-web): Accept edits, Plan, dan Auto. Accept edits sesuai dengan mode `default`: lingkungan cloud pra-menyetujui pengeditan file terlepas dari mode, jadi dropdown menampilkan Accept edits alih-alih Manual. Sesi cloud masih menghormati `defaultMode: "acceptEdits"` dari pengaturan. Mode Auto muncul hanya ketika organisasi Anda mengizinkannya dan model yang dipilih mendukungnya. Bypass permissions tidak tersedia.
    * **Sesi [Remote Control](/docs/id/remote-control)** di mesin lokal Anda: Manual, Accept edits, dan Plan. Anda tidak dapat memilih Auto atau Bypass permissions dari aplikasi. Dropdown menampilkan mode yang sesi lokal gunakan, termasuk mode yang diatur dari terminal, dan diperbarui ketika mode berubah di aplikasi atau di terminal. Satu-satunya pengecualian adalah Bypass permissions: sesi tidak pernah melaporkan mode itu ke claude.ai, jadi beralih ke dalamnya dari terminal tidak mengubah apa yang ditampilkan dropdown. Sebelum v2.1.202, sesi yang terhubung dengan `/remote-control` atau `claude --remote-control` tidak melaporkan mode mereka sama sekali, jadi claude.ai dan aplikasi mobile dapat menampilkan mode yang sesi tidak gunakan. Ketidaksesuaian hanya mempengaruhi label: Claude Code menghasilkan prompt izin dari mode aktual sesi, dan mereka masih muncul di aplikasi untuk persetujuan.

    Untuk Remote Control, Anda juga dapat mengatur mode awal saat meluncurkan host:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approve pengeditan file dengan mode acceptEdits
</h2>

Mode `acceptEdits` memungkinkan Claude membuat dan mengedit file di direktori kerja Anda tanpa meminta. Bilah status menunjukkan `⏵⏵ accept edits on` saat mode ini aktif.

Selain pengeditan file, mode `acceptEdits` auto-approve perintah Bash filesystem umum: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, dan `sed`. Perintah ini juga auto-approved ketika diawali dengan variabel lingkungan aman seperti `LANG=C` atau `NO_COLOR=1`, atau pembungkus proses seperti `timeout`, `nice`, atau `nohup`. Seperti pengeditan file, auto-approval hanya berlaku untuk jalur di dalam direktori kerja Anda atau `additionalDirectories`. Jalur di luar cakupan itu, penulisan ke [jalur yang dilindungi](#protected-paths), dan semua perintah Bash lainnya kecuali [set bawaan read-only](/docs/id/permissions#read-only-commands) masih meminta.

Ketika [alat PowerShell](/docs/id/tools-reference#powershell-tool) diaktifkan, mode `acceptEdits` juga auto-approve `Set-Content`, `Add-Content`, `Clear-Content`, dan `Remove-Item` pada jalur dalam cakupan, bersama dengan alias umum mereka. Aturan cakupan dan jalur yang dilindungi yang sama berlaku.

Gunakan `acceptEdits` ketika Anda ingin meninjau perubahan di editor Anda atau melalui `git diff` setelahnya daripada menyetujui setiap pengeditan inline.

Tekan `Shift+Tab` sekali dari mode Manual untuk memasukkannya, atau mulai dengannya langsung:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analisis sebelum Anda mengedit dengan mode rencana
</h2>

Mode rencana memberi tahu Claude untuk meneliti dan mengusulkan perubahan tanpa membuatnya. Claude membaca file, menjalankan perintah shell untuk menjelajahi, dan menulis rencana, tetapi tidak mengedit sumber Anda. Prompt izin berlaku seperti dalam mode Manual kecuali [mode otomatis](/docs/id/auto-mode-config) tersedia dan `useAutoModeDuringPlan` aktif, yang merupakan default. Dengan mode otomatis aktif, pengklasifikasi menyetujui perintah hanya-baca seperti pencarian dan pembacaan file tanpa meminta. Pengeditan tetap diblokir baik cara manapun sampai Anda menyetujui rencana.

Masukkan mode rencana dengan menekan `Shift+Tab` atau mengawali prompt tunggal dengan `/plan`. Anda juga dapat memulai dalam mode rencana dari CLI:

```bash theme={null}
claude --permission-mode plan
```

Tekan `Shift+Tab` lagi untuk meninggalkan mode rencana tanpa menyetujui rencana.

<h3 id="review-and-approve-a-plan">
  Tinjau dan setujui rencana
</h3>

Ketika rencana siap, Claude menyajikannya dan menanyakan cara melanjutkan. Dari prompt itu Anda dapat:

* Setujui dan mulai dalam mode otomatis
* Setujui dan terima pengeditan
* Setujui dan tinjau setiap pengeditan secara manual
* Terus merencanakan dengan umpan balik
* Perbaiki dengan [Ultraplan](/docs/id/ultraplan) untuk tinjauan berbasis browser

Menyetujui rencana keluar dari mode rencana dan mengalihkan sesi ke mode izin yang dijelaskan oleh setiap opsi persetujuan, sehingga Claude mulai mengedit. Untuk merencanakan lagi, kembali ke mode rencana dengan `Shift+Tab`, atau awali prompt berikutnya dengan `/plan`.

Tekan `Ctrl+G` untuk membuka rencana yang diusulkan di editor teks default Anda dan mengeditnya secara langsung sebelum Claude melanjutkan. Ketika [`showClearContextOnPlanAccept`](/docs/id/settings#available-settings) diaktifkan, setiap opsi persetujuan juga menawarkan untuk menghapus konteks perencanaan terlebih dahulu.

Menerima rencana juga memberi nama sesi dari konten rencana secara otomatis, kecuali Anda telah menetapkan nama dengan `--name` atau `/rename`.

<h3 id="set-plan-mode-as-the-default">
  Atur mode rencana sebagai default
</h3>

Untuk membuat mode rencana sebagai default untuk proyek, atur `defaultMode` dalam `.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  Hilangkan prompt izin dengan mode otomatis
</h2>

Mode otomatis memungkinkan Claude menjalankan tanpa prompt izin rutin. Model pengklasifikasi terpisah meninjau tindakan sebelum berjalan, memblokir apa pun yang melampaui permintaan Anda, menargetkan infrastruktur yang tidak dikenali, atau tampak didorong oleh konten bermusuhan yang dibaca Claude. [Aturan ask](/docs/id/permissions#manage-permissions) eksplisit masih memaksa prompt.

Penghapusan yang menargetkan akar sistem file atau direktori home, seperti `rm -rf /` dan `rm -rf ~`, meminta persetujuan daripada pergi ke pengklasifikasi. Prompt ini juga menyala ketika perintah berisi substitusi perintah dengan `$(...)` atau backtick, atau substitusi proses dengan `<(...)`, apakah penghapusan berada di dalam substitusi, seperti dalam `echo "$(rm -rf ~)"`, atau di tempat lain dalam perintah yang sama. Sebelum v2.1.208, perintah yang berisi bentuk-bentuk tersebut pergi ke pengklasifikasi daripada meminta.

Mode otomatis juga mendorong Claude untuk terus bekerja tanpa berhenti untuk pertanyaan klarifikasi, meskipun Claude masih bertanya ketika prompt Anda atau skill secara eksplisit bergantung padanya. Untuk perilaku otonom yang lebih kuat sambil tetap mempertahankan prompt izin, atur [gaya output Proaktif](/docs/id/output-styles) sebagai gantinya.

<Warning>
  Mode otomatis mengurangi prompt izin tetapi tidak menjamin keamanan. Gunakan untuk tugas di mana Anda mempercayai arah umum, bukan sebagai pengganti tinjauan pada operasi sensitif.
</Warning>

Mode otomatis hanya tersedia ketika akun Anda memenuhi semua persyaratan ini:

* **Rencana**: Semua rencana.
* **Pemilik**: di Team dan Enterprise, Pemilik harus mengaktifkannya di [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code) sebelum pengguna dapat mengaktifkannya. Administrator juga dapat mematikan mode otomatis dengan mengatur `permissions.disableAutoMode` ke `"disable"` di [pengaturan terkelola](/docs/id/permissions#managed-settings). Untuk tab Code aplikasi desktop, `disableAutoMode` adalah kontrol tingkat organisasi, dan toggle pengaturan admin tidak berlaku.
* **Model**: di API Anthropic, Claude Opus 4.6 atau lebih baru, atau Sonnet 4.6 atau lebih baru. Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan sesi [Claude apps gateway](/docs/id/claude-apps-gateway) yang masuk, hanya Claude Sonnet 5, Opus 4.7, dan Opus 4.8. Model yang lebih lama, termasuk Sonnet 4.5, Opus 4.5, Haiku, dan model claude-3, tidak didukung di penyedia mana pun.
* **Penyedia**: tersedia secara default di API Anthropic, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan sesi Claude apps gateway yang masuk. Di v2.1.158 hingga v2.1.206, mode otomatis mati di semua penyedia ini kecuali API Anthropic sampai Anda mengatur `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 menghapus persyaratan.

Jika Claude Code melaporkan mode otomatis sebagai tidak tersedia, salah satu persyaratan ini tidak terpenuhi; ini bukan pemadaman sementara. Pesan terpisah yang menyebutkan model dan mengatakan mode otomatis "tidak dapat menentukan keamanan" tindakan adalah pemadaman pengklasifikasi sementara; lihat [referensi kesalahan](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action).

Jika Anda mengatur `defaultMode: "auto"` di [pengaturan](/docs/id/settings#available-settings) dan sesi dimulai dalam mode `default` tanpa kesalahan, pengaturan kemungkinan berada di `.claude/settings.json` atau `.claude/settings.local.json`. Claude Code v2.1.142 dan lebih baru mengabaikan `auto` dari file-file tersebut sehingga repositori tidak dapat memberikan dirinya sendiri mode otomatis. Pindahkan ke `~/.claude/settings.json`.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Mode otomatis di Bedrock, Agent Platform, atau Foundry
</h3>

Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), dan sesi [Claude apps gateway](/docs/id/claude-apps-gateway) yang masuk, mode otomatis muncul dalam siklus `Shift+Tab` secara default. Muncul dalam siklus tidak mengubah mode yang dimulai sesi: sesi masih dimulai dalam [`defaultMode`](/docs/id/settings#available-settings) Anda, yang Manual kecuali Anda mengubahnya. Hanya Claude Sonnet 5, Opus 4.7, dan Opus 4.8 yang didukung di penyedia ini.

Untuk menjadikan mode otomatis mode awal default, atur `"permissions": {"defaultMode": "auto"}` di pengaturan pengguna atau terkelola.

Untuk mencegah pengembang menggunakan mode otomatis, atur `disableAutoMode` ke `"disable"` di [pengaturan terkelola](/docs/id/permissions#managed-settings). Ini menghapus `auto` dari siklus `Shift+Tab` dan menolak `--permission-mode auto` saat startup.

Di v2.1.158 hingga v2.1.206, mode otomatis mati di penyedia ini sampai Anda mengatur `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, dan Claude Code mengabaikan `defaultMode: "auto"` di penyedia ini kecuali variabel juga diatur. Variabel masih diterima untuk kompatibilitas dan tidak berpengaruh dari v2.1.207 ke depan.

<h3 id="what-the-classifier-blocks-by-default">
  Apa yang diblokir pengklasifikasi secara default
</h3>

Pengklasifikasi mempercayai direktori kerja Anda dan remote yang dikonfigurasi untuk itu ketika sesi dimulai. Remote yang ditambahkan atau ditunjuk ulang selama sesi dengan `git remote add` atau `git remote set-url` tidak dipercaya, dan semuanya yang lain diperlakukan sebagai eksternal sampai Anda [mengkonfigurasi infrastruktur terpercaya](/docs/id/auto-mode-config). Sebelum v2.1.200, remote yang ditambahkan di tengah sesi juga dipercaya.

**Diblokir secara default**:

* Mengunduh dan menjalankan kode, seperti `curl | bash`
* Mengirim data sensitif ke endpoint eksternal
* Deploy dan migrasi produksi
* Penghapusan massal pada penyimpanan cloud
* Memberikan izin IAM atau repo
* Memodifikasi infrastruktur bersama
* Menghancurkan file secara tidak dapat dipulihkan yang ada sebelum sesi
* Force push
* Mendorong ke cabang default repositori ketika push membawa konten sensitif seperti rahasia atau data pribadi atau terpercaya, membawa perubahan yang disembunyikan atau salah dideskripsikan relatif terhadap apa yang Anda minta, membawa konten yang diimpor atau pertama kali dibaca dari luar repositori, atau merutekan di sekitar pull request, review, atau check yang Anda minta. Push biasa ke cabang default tidak diblokir dengan sendirinya, dan menghapus push yang ditandai memerlukan penamaan konten yang ditandai atau review yang dilewati, bukan hanya push. Pengklasifikasi adalah satu lapisan: [aturan `permissions.deny`](/docs/id/permissions#manage-permissions) berlaku di setiap mode dan dapat memblokir push ke cabang default sepenuhnya, dan perlindungan cabang remote itu sendiri masih berlaku. Sebelum v2.1.203, push langsung apa pun ke cabang default diblokir
* }`git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop`, atau `git stash clear`, yang pengklasifikasi asumsikan akan membuang perubahan yang belum dikomit
* `git commit --amend` ketika commit di HEAD tidak dibuat dalam sesi ini
* Dari v2.1.198, `git commit --amend` ketika commit di HEAD sudah didorong. Reword hanya pesan tidak diblokir: `--amend -m` tanpa apa pun yang baru dipentaskan, pada commit yang dibuat Claude selama sesi ini
* `terraform destroy`, `pulumi destroy`, `cdk destroy`, atau `terragrunt destroy`, dan menerapkan rencana yang menghancurkan sumber daya

Claude Code v2.1.195 dan lebih baru memblokir lebih banyak kategori secara default. Beberapa bergantung pada entri [lingkungan](/docs/id/auto-mode-config#define-trusted-infrastructure), seperti target jarak jauh sensitif dan cakupan IaC yang dilindungi, yang dapat Anda persempit ke nama konkret.

* Menulis ke manajer rahasia, atau mengubah catatan DNS atau sertifikat TLS
* Menggabungkan permintaan tarik yang belum disetujui manusia, menyetujui permintaan tarik Claude sendiri, atau menonaktifkan pemeriksaan CI
* Memposting komentar yang merupakan perintah untuk otomasi, seperti `atlantis apply` atau `/deploy` atau `/merge` bot
* Mengalihkan, meningkatkan, atau menghapus bendera fitur produksi
* Menerapkan perubahan infrastruktur ke cakupan IaC yang dilindungi, atau mengalirkan dan menghapus node cluster
* Penulisan ke cluster komputasi bersama yang melampaui sumber daya yang Anda beri nama, seperti pemilih label atau `--all` yang menangkap pekerjaan pengguna lain
* Membuat sumber daya Kubernetes yang berjalan di setiap node atau mencegat lalu lintas cluster, seperti DaemonSets dan webhook penerimaan
* Shell interaktif atau port-forward ke target jarak jauh sensitif
* Membuka terowongan atau shell terbalik yang membuat layanan lokal dapat dijangkau dari internet publik
* Mencetak kredensial atau token langsung ke transkrip atau file
* Mengakses lokasi yang tercantum sebagai lokasi data sensitif di [lingkungan](/docs/id/auto-mode-config#define-trusted-infrastructure) Anda, atau menyalin data keluar dari satu. Mulai dari v2.1.198 ini juga memblokir pengiriman data dari satu ke audiens yang entri kecualikan
* Merutekan instalasi paket di sekitar registri paket internal Anda ke registri publik. Mulai dari v2.1.198, ini juga berlaku ketika Anda telah memberi tahu Claude bahwa registri internal atau mirror ada dalam percakapan, bukan hanya ketika satu tercantum di lingkungan Anda
* Menjalankan perintah dengan bendera yang melucuti penjaga keselamatan, seperti `--insecure`
* Meluncurkan loop agen otonom yang berjalan tanpa persetujuan manusia atau sandbox, seperti yang dimulai dengan `--dangerously-skip-permissions` atau `--no-sandbox`. Mulai dari v2.1.198 ini juga mencakup menjalankan agen pihak ketiga atau harness eval dengan isolasi dan persetujuan per-tindakan dinonaktifkan, seperti runner yang dimulai dengan `--yes-always`
* Tindakan browser [Claude di Chrome](/docs/id/chrome) yang dapat mengirim konten halaman, cookie, atau kredensial off-origin

Claude Code v2.1.198 dan lebih baru juga memblokir ini secara default:

* Menghapus file di `/tmp`, `$TMPDIR`, atau direktori scratch atau cache bersama lainnya dengan wildcard, glob, atau filter usia daripada dengan jalur bernama spesifik
* Termasuk detail sensitif dalam konten yang dikirim, diunggah, dipublikasikan, atau ditulis ke orang lain atau sistem bersama, ketika pesan Anda sendiri tidak mengotorisasi detail tersebut untuk penerima itu. Badan PR dan issue, pesan komit, dan komentar dihitung sebagai jenis konten keluar ini ketika repositori berada di luar batas kepercayaan atau publik, termasuk repositori publik organisasi Anda sendiri; jalur file internal, nama kode, data respons API langsung seperti email atau pengenal akun, dan pengenal infrastruktur dihitung sebagai detail sensitif. Scoping PR, issue, dan pesan komit memerlukan Claude Code v2.1.200 atau lebih baru. Data pribadi langsung dari respons API dalam badan PR atau issue, seperti alamat email, pengenal akun atau organisasi, atau metrik penggunaan, memerlukan Anda untuk menyebutkan detail tersebut dan penerima terlepas dari visibilitas atau batas kepercayaan repositori. Pemeriksaan itu memerlukan Claude Code v2.1.203 atau lebih baru
* Mengirim keystroke ke pane tmux Claude Code sendiri untuk menjalankan antarmukanya sendiri, yang pengklasifikasi perlakukan sebagai Claude mengubah izin atau pengawasannya sendiri

Claude Code v2.1.200 dan lebih baru juga memblokir ini secara default:

* Mengomentari, menghapus, atau force-passing test atau assertion yang menjaga perilaku keamanan, seperti auth, kontrol akses, validasi input, atau sandboxing
* Menghapus atau merobohkan sumber daya stateful yang tidak dibuat Claude dalam sesi, ketika tidak ada aturan penghapusan yang lebih spesifik berlaku dan Anda tidak menyebutkan sumber daya itu
* Menunjuk ulang URL dasar API, endpoint proxy, penerima webhook, atau mirror registri ke host pihak ketiga yang tidak sesuai dengan tugas, termasuk dalam file contoh seperti `.env.example`
* Mengubah ke mana push pergi dengan `git remote set-url` atau `git remote add`, kecuali Anda menyebutkan remote baru
* Mendorong rahasia atau data pribadi atau terpercaya ke repositori yang diketahui publik, atau mendorong materi rahasia lainnya ke sana yang bukan bagian dari pekerjaan repositori itu sendiri. Subjek repositori dotfiles itu sendiri adalah satu-satunya pengecualian untuk data pribadi atau terpercaya, dan konten dari repositori pribadi yang mencapai permukaan publik apa pun diblokir dengan cara yang sama; kedua penyempurnaan memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.203, data pribadi dikelompokkan dengan materi rahasia dan diblokir hanya ketika itu bukan bagian dari pekerjaan repositori itu sendiri. Ketika visibilitas repositori tidak ditetapkan, pengklasifikasi tidak memblokir itu saja; itu menilai konten terhadap aturan lain sebagai gantinya
* Membuka permintaan tarik terhadap repositori atau organisasi yang berbeda, forking dengan `gh repo fork`, atau mendorong ke repositori pihak ketiga, kecuali Anda menyebutkan target eksternal itu

Claude Code v2.1.203 dan lebih baru juga memblokir ini secara default:

* Konten dari toko lokal sensitif, atau dari file yang nama, jalur, atau jenisnya menandainya sebagai sensitif, memasuki komit, push, teks PR atau issue, gist atau paste, atau package publish, kecuali Anda menyebutkan sumber dan tujuan. Transkrip sesi dan log percakapan, folder dot kredensial dan konfigurasi seperti kunci SSH, kredensial cloud, profil browser, dan riwayat shell, dan ekspor data pengguna semuanya dihitung, dan repositori yang pribadi tidak menghapusnya

Claude Code v2.1.205 dan lebih baru juga memblokir ini secara default:

* Menulis ke transkrip sesi Claude Code, file riwayat `.jsonl` di bawah `~/.claude/projects/` atau direktori konfigurasi Anda yang dikonfigurasi, baik secara langsung atau melalui perintah shell. Aturan ini juga mencakup baris metadata yang Claude Code tambahkan ke setiap entri transkrip untuk pemeriksaannya sendiri. Transkrip adalah status sesi yang ditulis Claude Code, bukan file kerja, dan entri yang dirusak mencapai setiap pemeriksaan nanti setelah Anda melanjutkan sesi, jadi mode otomatis memblokir penulisan ini sebagai pertahanan berlapis. Membaca transkrip tidak diblokir
* Penghapusan paksa rekursif seperti `rm -rf "$VAR"` atau `Remove-Item -Recurse -Force $dir` yang targetnya adalah variabel shell, atau glob yang berakar pada satu, yang tidak ditugaskan di mana pun dalam percakapan yang dilihat pengklasifikasi. Nilai berasal hanya dari output perintah sebelumnya, yang tidak pernah diterima pengklasifikasi, jadi pengklasifikasi tidak dapat memverifikasi target penghapusan terhadap aturan penghapusan lainnya. Pengklasifikasi membaca percakapan daripada output perintah dengan desain, jadi itu memblokir panggilan daripada menebak target. Blokir jelas ketika Anda menyebutkan jalur yang tepat dihapus, atau ketika Claude menjalankan ulang penghapusan dengan jalur literal yang diselesaikan ditulis ke dalam perintah. Penghapusan yang targetnya dapat diselesaikan pengklasifikasi tidak terpengaruh

**Diizinkan secara default**:

* Operasi file lokal di direktori kerja Anda
* Menginstal dependensi yang dideklarasikan dalam file kunci atau manifes Anda
* Membaca `.env` dan mengirim kredensial ke API yang cocok
* Permintaan HTTP read-only
* Push ke cabang yang Anda mulai atau yang dibuat Claude
* Push rutin ke cabang default repositori. Sebelum v2.1.203, push langsung apa pun ke cabang default diblokir

Claude Code v2.1.195 dan lebih baru juga memungkinkan ini secara default:

* Menghapus pekerjaan yang tepat yang dibuat Claude sebelumnya dalam sesi yang sama
* Membaca, meninjau, atau menulis kode, konfigurasi, dan model ancaman terkait keamanan sebagai bagian dari tugas Anda
* Pesan antara agen yang bekerja bersama dalam sesi multi-agen yang sama
* Mengirim data ke domain terpercaya, bucket, dan layanan yang Anda daftarkan di [`environment`](/docs/id/auto-mode-config#define-trusted-infrastructure). Ini mencakup aliran data saja, bukan operasi destruktif atau kredensial pada infrastruktur yang sama
* Navigasi [Claude di Chrome](/docs/id/chrome) ke domain internal terpercaya, localhost, atau URL yang Anda beri nama

Permintaan akses jaringan sandbox dirutekan melalui pengklasifikasi daripada diizinkan secara default. Mulai dari v2.1.198, pengklasifikasi menggunakan kembali vonis untuk host dan port jaringan daripada menjalankan ulang pada setiap koneksi:

* Izin digunakan kembali sampai konten baru memasuki percakapan, di mana titik host itu diperiksa lagi
* Di CLI interaktif, penolakan dijatuhkan ketika giliran berakhir
* Di [mode non-interaktif](/docs/id/headless) dan sesi Agent SDK tidak ada batas giliran, jadi penolakan digunakan kembali untuk sisa jalannya
* Mengubah mode izin atau aturan Anda menjatuhkan semua vonis yang di-cache

Jalankan `claude auto-mode defaults` untuk melihat daftar aturan lengkap. Jika tindakan rutin diblokir, administrator dapat menambahkan repo terpercaya, bucket, dan layanan melalui pengaturan `autoMode.environment`: lihat [Konfigurasi mode otomatis](/docs/id/auto-mode-config).

Push ke cabang kerja Anda, membuat push rutin ke cabang default repositori, dan membuat permintaan tarik yang cocok dengan permintaan Anda semua berjalan tanpa prompt. Pengklasifikasi memblokir push hanya ketika membawa risiko, seperti force push atau konten yang merutekan di sekitar review yang Anda atur. Untuk memerlukan checkpoint manusia sebelum tindakan ini sambil tetap dalam mode otomatis, tambahkan aturan `permissions.ask`: lihat [Batas umum](/docs/id/auto-mode-config#common-boundaries).

<h3 id="boundaries-you-state-in-conversation">
  Batas yang Anda nyatakan dalam percakapan
</h3>

Pengklasifikasi memperlakukan batas yang Anda nyatakan dalam percakapan sebagai sinyal blokir. Jika Anda memberi tahu Claude "jangan push" atau "tunggu sampai saya tinjau sebelum deploy", pengklasifikasi memblokir tindakan yang cocok bahkan ketika aturan default akan mengizinkannya. Batas tetap berlaku sampai Anda mengangkatnya dalam pesan yang lebih baru. Penilaian Claude sendiri bahwa kondisi terpenuhi tidak mengangkatnya.

Batas tidak disimpan sebagai aturan. Pengklasifikasi membaca ulang dari transkrip pada setiap pemeriksaan, jadi batas dapat hilang jika [pemadatan konteks](/docs/id/costs#reduce-token-usage) menghapus pesan yang menyatakannya. Untuk jaminan keras, tambahkan [aturan deny](/docs/id/permissions#permission-rule-syntax) sebagai gantinya.

<h3 id="when-auto-mode-falls-back">
  Ketika mode otomatis jatuh kembali
</h3>

Setiap tindakan yang ditolak menunjukkan notifikasi dan muncul di `/permissions` di bawah tab Recently denied, di mana Anda dapat menekan `r` untuk mencoba ulang dengan persetujuan manual.

Jika pengklasifikasi memblokir tindakan 3 kali berturut-turut atau 20 kali total, mode otomatis dijeda dan Claude Code melanjutkan prompting. Menyetujui tindakan yang diminta melanjutkan mode otomatis. Ambang batas ini tidak dapat dikonfigurasi. Tindakan yang diizinkan apa pun mengatur ulang penghitung berturut-turut, sementara penghitung total bertahan untuk sesi dan hanya direset ketika batasnya sendiri memicu fallback.

Dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p`, blokir berulang membatalkan sesi karena tidak ada pengguna untuk diminta.

Blokir berulang biasanya berarti pengklasifikasi kehilangan konteks tentang infrastruktur Anda. Gunakan `/feedback` untuk melaporkan positif palsu, atau minta administrator untuk [mengkonfigurasi infrastruktur terpercaya](/docs/id/auto-mode-config).

<AccordionGroup>
  <Accordion title="Bagaimana pengklasifikasi mengevaluasi tindakan">
    Setiap tindakan melalui urutan keputusan yang tetap. Langkah pertama yang cocok menang:

    1. Tindakan yang cocok dengan [aturan allow, ask, atau deny Anda](/docs/id/permissions#manage-permissions) diselesaikan segera. Penulisan ke [jalur yang dilindungi](#protected-paths) dirutekan ke pengklasifikasi bahkan ketika aturan allow cocok. Alat connector [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) meminta Anda secara langsung bahkan ketika aturan allow cocok. Aturan ask yang dibatasi konten jatuh kembali ke prompt izin
    2. Tindakan read-only dan pengeditan file di direktori kerja Anda disetujui otomatis, kecuali penulisan ke [jalur yang dilindungi](#protected-paths)
    3. Semuanya yang lain pergi ke pengklasifikasi. Alat connector [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) melewati pengklasifikasi dan meminta Anda secara langsung, jadi persetujuan yang diperlukan organisasi tidak pernah disetujui otomatis. Mulai dari v2.1.199, alat MCP yang ditandai dengan [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) juga melewati pengklasifikasi dan meminta Anda secara langsung, jadi langkah persetujuan tidak pernah disetujui otomatis atas nama penulis alat
    4. Jika pengklasifikasi memblokir, Claude menerima alasan dan mencoba alternatif

    Saat memasuki mode otomatis, aturan allow luas yang memberikan eksekusi kode arbitrer dijatuhkan:

    * Blanket `Bash(*)` atau `PowerShell(*)`
    * Penafsir yang diberi wildcard seperti `Bash(python*)`
    * Perintah run manajer paket
    * Aturan `Agent` allow

    Aturan sempit seperti `Bash(npm test)` dibawa. Aturan yang dijatuhkan dipulihkan ketika Anda meninggalkan mode otomatis.

    Pengklasifikasi melihat pesan pengguna, panggilan alat, dan konten CLAUDE.md Anda. Hasil alat dilepas, jadi konten bermusuhan dalam file atau halaman web tidak dapat memanipulasinya secara langsung. Probe sisi server terpisah memindai hasil alat masuk dan menandai konten mencurigakan sebelum Claude membacanya. Untuk lebih lanjut tentang cara lapisan ini bekerja bersama, lihat [pengumuman mode otomatis](https://claude.com/blog/auto-mode) dan [penggalian teknis](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Bagaimana mode otomatis menangani subagen">
    Pengklasifikasi memeriksa pekerjaan [subagen](/docs/id/sub-agents) di tiga titik:

    1. Sebelum subagen dimulai, deskripsi tugas yang didelegasikan dievaluasi, jadi tugas yang terlihat berbahaya diblokir pada waktu spawn.
    2. Saat subagen berjalan, setiap tindakannya melalui pengklasifikasi dengan aturan yang sama seperti sesi induk, dan `permissionMode` apa pun di frontmatter subagen diabaikan.
    3. Ketika subagen selesai, pengklasifikasi meninjau riwayat tindakan lengkapnya; jika pemeriksaan pengembalian menandai kekhawatiran, peringatan keamanan ditambahkan ke hasil subagen.

    Langkah 1 memerlukan Claude Code v2.1.178 atau lebih baru. Versi sebelumnya menerapkan pengklasifikasi pada langkah 2 dan 3, tetapi tidak mengevaluasi deskripsi tugas sebelum subagen dimulai.
  </Accordion>

  <Accordion title="Biaya dan latensi">
    Pengklasifikasi berjalan pada model yang dikonfigurasi server yang independen dari pilihan `/model` Anda, jadi beralih model tidak mengubah ketersediaan pengklasifikasi. Panggilan pengklasifikasi dihitung terhadap penggunaan token Anda. Setiap pemeriksaan mengirim sebagian dari transkrip ditambah tindakan yang tertunda, menambahkan perjalanan bolak-balik sebelum eksekusi. Pembacaan dan pengeditan direktori kerja di luar jalur yang dilindungi melewati pengklasifikasi, jadi overhead terutama berasal dari perintah shell dan operasi jaringan. Mulai dari v2.1.198, vonis jaringan sandbox untuk host dan port digunakan kembali daripada diklasifikasi ulang pada setiap koneksi, jadi koneksi berulang ke host yang sama tidak masing-masing menambahkan pemeriksaan. [Apa yang diblokir pengklasifikasi secara default](#what-the-classifier-blocks-by-default) menjelaskan berapa lama izin dan penolakan berlangsung.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Izinkan hanya alat yang telah disetujui sebelumnya dengan mode dontAsk
</h2>

Jika Anda menetapkan mode `dontAsk`, Claude Code secara otomatis menolak setiap panggilan alat yang akan meminta sebaliknya. Claude hanya menjalankan tindakan yang cocok dengan aturan `permissions.allow` Anda, [perintah Bash read-only](/docs/id/permissions#read-only-commands), dan panggilan yang disetujui oleh [hook PreToolUse](/docs/id/permissions#extend-permissions-with-hooks). Gunakan mode ini untuk pipeline CI atau lingkungan terbatas di mana Anda pre-define dengan tepat apa yang Claude boleh lakukan; sesi tidak pernah menunggu input. Bilah status menunjukkan `⏵⏵ don't ask on` saat mode ini aktif.

Claude Code menolak panggilan yang cocok dengan aturan [`ask`](/docs/id/permissions#manage-permissions) eksplisit Anda daripada meminta. Ini juga menolak alat `AskUserQuestion` bawaan dan alat konektor [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), bahkan jika aturan allow Anda cocok dengannya. Ini menolak alat MCP yang ditandai [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) dengan cara yang sama, karena kartu persetujuan mereka memerlukan jawaban yang mode ini tidak pernah kumpulkan; ini memerlukan Claude Code v2.1.199 atau lebih baru.

Sesi cloud di [Claude Code di web](/docs/id/claude-code-on-the-web) mengabaikan `defaultMode: "dontAsk"`; lihat [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) untuk detail.

Atur saat startup dengan flag:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Lewati semua pemeriksaan dengan mode bypassPermissions
</h2>

Mode `bypassPermissions` menonaktifkan prompt izin dan pemeriksaan keamanan sehingga panggilan alat dijalankan segera, termasuk penulisan ke [jalur yang dilindungi](#protected-paths). Sebelum v2.1.126, penulisan jalur yang dilindungi masih meminta dalam mode ini.

Aturan [ask](/docs/id/permissions#manage-permissions) yang eksplisit dan alat konektor [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) masih memaksa prompt dalam mode ini. Alat MCP yang ditandai dengan [`_meta["anthropic/requiresUserInteraction"]`](/docs/id/mcp#require-approval-for-a-specific-tool) juga masih meminta; ini memerlukan Claude Code v2.1.199 atau lebih baru.

Penghapusan yang menargetkan akar sistem file atau direktori home, seperti `rm -rf /` dan `rm -rf ~`, masih meminta sebagai pemutus sirkuit terhadap kesalahan model. Pemutus sirkuit juga aktif ketika perintah berisi substitusi perintah dengan `$(...)` atau backtick, atau substitusi proses dengan `<(...)`, baik penghapusan berada di dalam substitusi, seperti dalam `echo "$(rm -rf ~)"`, atau di tempat lain dalam perintah yang sama. Bentuk biasa, diketik sebagai perintahnya sendiri, telah meminta dalam mode ini sejak pemutus sirkuit diperkenalkan; sebelum v2.1.208, perintah yang berisi bentuk-bentuk tersebut tidak meminta.

<Warning>
  Hanya gunakan mode ini di lingkungan terisolasi seperti kontainer, VM, atau dev container tanpa akses internet, di mana Claude Code tidak dapat merusak sistem host Anda.
</Warning>

Anda tidak dapat memasukkan `bypassPermissions` dari sesi yang dimulai tanpa salah satu flag yang mengaktifkan; restart dengan salah satu untuk mengaktifkannya:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Flag `--dangerously-skip-permissions` setara.

Di Linux dan macOS, Claude Code menolak untuk memulai dalam mode ini saat berjalan sebagai root atau di bawah `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Pemeriksaan dilewati secara otomatis di dalam sandbox yang dikenali. Untuk menjalankan secara otonom dalam kontainer, gunakan konfigurasi [dev container](/docs/id/devcontainer), yang menjalankan Claude Code sebagai pengguna non-root.

[Claude Code di web](/docs/id/claude-code-on-the-web) tidak menghormati `defaultMode: "bypassPermissions"` atau `"dontAsk"` dari file pengaturan Anda, jadi pengaturan yang diperiksa dalam repositori tidak dapat memulai sesi cloud dalam mode bypass-permissions. Pengaturan diabaikan secara diam-diam dan sesi dimulai dalam mode yang ditampilkan di dropdown mode sebagai gantinya. Lihat [Beralih mode izin](#switch-permission-modes) untuk mode mana yang ditawarkan sesi cloud.

<Warning>
  `bypassPermissions` tidak menawarkan perlindungan terhadap injeksi prompt atau tindakan yang tidak diinginkan. Untuk pemeriksaan keamanan latar belakang dengan jauh lebih sedikit prompt, gunakan [mode otomatis](#eliminate-prompts-with-auto-mode) sebagai gantinya. Administrator dapat memblokir mode ini dengan mengatur `permissions.disableBypassPermissionsMode` ke `"disable"` di [pengaturan terkelola](/docs/id/permissions#managed-settings).
</Warning>

<h2 id="protected-paths">
  Jalur yang dilindungi
</h2>

Penulisan ke serangkaian jalur kecil tidak pernah disetujui otomatis, di setiap mode kecuali `bypassPermissions`. Ini mencegah kerusakan yang tidak disengaja dari status repositori dan konfigurasi Claude sendiri.

| Mode                             | Penulisan jalur yang dilindungi |
| :------------------------------- | :------------------------------ |
| `default`, `acceptEdits`, `plan` | Diminta                         |
| `auto`                           | Dirutekan ke pengklasifikasi    |
| `dontAsk`                        | Ditolak                         |
| `bypassPermissions`              | Diizinkan                       |

Aturan [`permissions.allow`](/docs/id/permissions#manage-permissions) dalam file pengaturan tidak pra-menyetujui penulisan jalur yang dilindungi. Pemeriksaan keamanan berjalan sebelum Claude Code mengevaluasi aturan allow dari pengaturan, jadi entri seperti `Edit(.claude/**)` dalam `~/.claude/settings.json` atau `.claude/settings.json` tidak mengubah hasil per-mode dalam tabel di atas. Dalam mode yang meminta, prompt untuk penulisan `.claude/` menawarkan **Ya, dan izinkan Claude untuk mengedit pengaturannya sendiri untuk sesi ini**, yang menyetujui penulisan `.claude/` berikutnya dalam sesi itu tanpa meminta lagi.

Direktori yang dilindungi:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, kecuali untuk `.claude/worktrees` di mana Claude menyimpan git worktrees-nya sendiri

File yang dilindungi:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="see-also">
  Lihat juga
</h2>

* [Permissions](/docs/id/permissions): aturan allow, ask, dan deny; kebijakan terkelola
* [Konfigurasi mode otomatis](/docs/id/auto-mode-config): beri tahu pengklasifikasi infrastruktur mana yang dipercaya organisasi Anda
* [Hooks](/docs/id/hooks): logika izin kustom melalui hook `PreToolUse` dan `PermissionRequest`
* [Ultraplan](/docs/id/ultraplan): jalankan mode rencana dalam sesi Claude Code di web dengan tinjauan berbasis browser
* [Security](/docs/id/security): perlindungan dan praktik terbaik
* [Sandboxing](/docs/id/sandboxing): isolasi filesystem dan jaringan untuk perintah Bash
* [Mode non-interaktif](/docs/id/headless): jalankan Claude Code dengan flag `-p`
