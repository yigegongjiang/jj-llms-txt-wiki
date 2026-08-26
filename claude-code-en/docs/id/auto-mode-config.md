> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi mode otomatis

> Beri tahu pengklasifikasi mode otomatis repositori, bucket, dan domain mana yang dipercaya organisasi Anda. Atur konteks lingkungan, ganti aturan blokir dan izin default, dan periksa konfigurasi efektif Anda dengan subperintah CLI mode otomatis.

[Mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) memungkinkan Claude Code berjalan tanpa permintaan izin rutin dengan merutekan panggilan alat melalui pengklasifikasi yang memblokir apa pun yang tidak dapat dibalikkan, merusak, atau ditujukan di luar lingkungan Anda. Aturan penolakan dan permintaan eksplisit dievaluasi sebelum pengklasifikasi dan masih memblokir atau meminta. Gunakan blok pengaturan `autoMode` untuk memberi tahu pengklasifikasi tersebut repositori, bucket, dan domain mana yang dipercaya organisasi Anda, sehingga berhenti memblokir operasi internal rutin.

<Note>
  Mode otomatis tersedia untuk semua pengguna di setiap penyedia, termasuk Anthropic API, Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk. Jika Claude Code melaporkan mode otomatis tidak tersedia untuk akun Anda, periksa [persyaratan lengkap](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), yang juga mencakup model yang didukung dan pengaktifan Pemilik pada paket Tim dan Enterprise. Dalam v2.1.158 hingga v2.1.206, mode otomatis di Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi gateway aplikasi Claude memerlukan pengaturan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 menghapus persyaratan.
</Note>

Secara default, pengklasifikasi hanya mempercayai direktori kerja dan remote yang dikonfigurasi dari repositori saat ini. Tindakan seperti mendorong ke organisasi kontrol sumber perusahaan Anda atau menulis ke bucket cloud tim diblokir sampai Anda menambahkannya ke `autoMode.environment`.

Untuk cara mengaktifkan mode otomatis dan apa yang diblokir secara default, lihat [Mode izin](/docs/id/permission-modes#eliminate-prompts-with-auto-mode). Halaman ini adalah referensi konfigurasi.

Halaman ini mencakup cara:

* [Tambahkan checkpoint manusia](#common-boundaries) untuk push dan pull request dengan `permissions.ask`
* [Pilih tempat untuk menetapkan aturan](#where-the-classifier-reads-configuration) di seluruh CLAUDE.md, pengaturan pengguna, dan pengaturan terkelola
* [Tentukan infrastruktur terpercaya](#define-trusted-infrastructure) dengan `autoMode.environment`
* [Ganti aturan blokir dan izin](#override-the-block-and-allow-rules) ketika default tidak sesuai dengan pipeline Anda
* [Rutekan semua perintah shell melalui pengklasifikasi](#route-all-shell-commands-through-the-classifier) dengan `autoMode.classifyAllShell`
* [Periksa konfigurasi efektif Anda](#inspect-the-defaults-and-your-effective-config) dengan subperintah `claude auto-mode`
* [Tinjau penolakan](#review-denials) sehingga Anda tahu apa yang harus ditambahkan selanjutnya

<h2 id="common-boundaries">
  Batas-batas umum
</h2>

Mode otomatis memungkinkan push ke cabang kerja Anda, push rutin ke cabang default repositori, dan pembuatan pull request secara default. Pengklasifikasi memblokir push hanya ketika membawa risiko, seperti force push atau konten yang menghindari review yang Anda atur. Jika Anda ingin checkpoint manusia sebelum setiap push atau pull request, tambahkan aturan izin: resep di bawah ini menjaga mode otomatis tetap aktif untuk segalanya.

Mekanisme paling langsung adalah [`permissions.ask`](/docs/id/permissions#permission-rule-syntax). Aturan ask yang dibatasi konten seperti yang di bawah ini dievaluasi sebelum pengklasifikasi dan selalu memaksa prompt izin, bahkan dalam mode otomatis, karena aturan ask eksplisit adalah niat yang dinyatakan untuk diminta untuk tindakan tersebut. Tambahkan aturan di [settings](/docs/id/settings#settings-files) Anda:

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Pilih mekanisme yang sesuai dengan seberapa tegas batas yang diperlukan:

| Batas                           | Mekanisme                                                           | Perilaku dalam mode otomatis                                                                                                                                                                                                     |
| :------------------------------ | :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Prompt sebelum tindakan         | `permissions.ask`                                                   | Selalu meminta untuk aturan yang dibatasi konten seperti resep di atas. Pengklasifikasi tidak dapat auto-approve tindakan yang cocok.                                                                                            |
| Jangan pernah jalankan tindakan | `permissions.deny`                                                  | Memblokir sebelum pengklasifikasi dikonsultasikan. Baik pengklasifikasi maupun niat pengguna tidak dapat menggantinya.                                                                                                           |
| Batas satu kali untuk sesi ini  | Nyatakan dalam percakapan, seperti "jangan push sampai saya review" | Pengklasifikasi memblokir tindakan yang cocok, tetapi batas dapat hilang jika [context compaction](/docs/id/costs#reduce-token-usage) menghapus pesan yang menyatakannya. Gunakan aturan ask atau deny untuk jaminan yang tahan lama. |

<h2 id="where-the-classifier-reads-configuration">
  Tempat pengklasifikasi membaca konfigurasi
</h2>

Pengklasifikasi membaca konten [CLAUDE.md](/docs/id/memory) yang sama yang dimuat Claude sendiri, jadi instruksi seperti "jangan pernah force push" di CLAUDE.md proyek Anda mengarahkan Claude dan pengklasifikasi secara bersamaan. Mulai dari sana untuk konvensi proyek dan aturan perilaku.

Untuk aturan yang berlaku di seluruh proyek, seperti infrastruktur terpercaya atau aturan penolakan di tingkat organisasi, gunakan blok pengaturan `autoMode`. Pengklasifikasi membaca `autoMode` dari cakupan berikut:

| Cakupan                             | File                                                | Gunakan untuk                                                     |
| :---------------------------------- | :-------------------------------------------------- | :---------------------------------------------------------------- |
| Satu pengembang                     | `~/.claude/settings.json`                           | Infrastruktur terpercaya pribadi                                  |
| Di seluruh organisasi               | [Pengaturan terkelola](/docs/id/server-managed-settings) | Infrastruktur terpercaya yang didistribusikan ke semua pengembang |
| Bendera `--settings` atau Agent SDK | JSON inline                                         | Penggantian per-invokasi untuk otomasi                            |

Pengklasifikasi tidak membaca `autoMode` dari pengaturan proyek di `.claude/settings.json` atau `.claude/settings.local.json`. Kedua file berada di direktori repo, jadi repo yang diperiksa atau langkah build dapat sebaliknya menyuntikkan aturan izinnya sendiri. Sebelum v2.1.207, pengklasifikasi juga membaca `.claude/settings.local.json`; pindahkan blok `autoMode` apa pun di file tersebut ke `~/.claude/settings.json`. Mengecualikan `.claude/settings.local.json` juga menutup kasus di mana repositori melakukan commit file atau alat lokal atau langkah build menulisnya.

Entri dari setiap cakupan digabungkan. Pengembang dapat memperluas `environment`, `allow`, `soft_deny`, dan `hard_deny` dengan entri pribadi tetapi tidak dapat menghapus entri yang disediakan pengaturan terkelola. Karena aturan izin bertindak sebagai pengecualian untuk aturan blok lunak di dalam pengklasifikasi, entri `allow` yang ditambahkan pengembang dapat mengganti entri `soft_deny` organisasi: kombinasinya bersifat aditif, bukan batas kebijakan keras.

<Note>
  Pengklasifikasi adalah gerbang kedua yang berjalan setelah [sistem izin](/docs/id/permissions). Untuk tindakan yang tidak boleh pernah berjalan terlepas dari niat pengguna atau konfigurasi pengklasifikasi, gunakan `permissions.deny` dalam pengaturan terkelola, yang memblokir tindakan sebelum pengklasifikasi dikonsultasikan dan tidak dapat ditimpa.
</Note>

<h2 id="define-trusted-infrastructure">
  Tentukan infrastruktur terpercaya
</h2>

Untuk sebagian besar organisasi, `autoMode.environment` adalah satu-satunya bidang yang perlu Anda atur. Ini memberi tahu pengklasifikasi repo, bucket, dan domain mana yang dipercaya: pengklasifikasi menggunakannya untuk memutuskan apa arti "eksternal", jadi tujuan apa pun yang tidak terdaftar adalah target exfiltration potensial.

Mulai dari Claude Code v2.1.198, `claude auto-mode defaults` mencetak tiga jenis entri lingkungan. Versi sebelum v2.1.195 hanya mencetak lima trust slot pertama.

* **Context slots**: menggambarkan organisasi, stack, dan postur keamanan Anda sehingga pengklasifikasi membaca aturan lain dalam konteks Anda. Tidak seperti dua jenis lainnya, context slots tidak memiliki aturan mereka sendiri yang menargetkan mereka. Masing-masing default ke `None configured` atau ke asumsi konservatif yang dinamai di sebelahnya:
  * **Organization**
  * **Primary use of Claude Code**: default ke pengembangan perangkat lunak
  * **Cloud provider(s)**
  * **Repository visibility**: repositori diasumsikan pribadi kecuali host remote dan namanya menunjukkan sebaliknya, atau pemeriksaan visibilitas sebelumnya dalam percakapan yang dibaca pengklasifikasi menunjukkan bahwa itu publik. Pengklasifikasi membaca pesan Anda dan perintah yang dijalankan Claude, bukan output mereka, jadi bukti harus berupa sesuatu yang dapat dibacanya, seperti pesan Anda sendiri yang menyebutkan repositori sebagai publik; output dari `gh repo view` sendirian tidak mencapainya. Pemeriksaan bukti transkrip memerlukan Claude Code v2.1.200 atau lebih baru
  * **Internal sharing / snippet hosting**: layanan paste dan gist publik diperlakukan sebagai di luar batas kepercayaan sampai Anda menyebutkan satu
  * **Org-specific CLIs**
  * **Secrets management**
  * **Default / protected branches**: `main` dan `master` diperlakukan sebagai terlindungi sampai Anda menyebutkan yang lain
  * **CI/CD deploy targets**
  * **Network posture**
  * **Protected deployment namespaces / environments**: kembali ke heuristik Sensitive remote targets sampai Anda menyebutkan mereka
  * **Data retention / declassification**
* **Trust slots**: nama apa yang diperlakukan pengklasifikasi sebagai di dalam batas Anda. Slot tersebut adalah Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services, dan Internal package registry. Entri repo dan source-control default ke repositori kerja dan remote yang dikonfigurasinya. Setiap trust slot lainnya default ke `None configured`, jadi tidak ada yang lain dipercaya sampai Anda menambahkannya. Visibilitas repositori hanya mencakup materi rahasia: repositori pribadi adalah tujuan yang dapat diterima untuk materi rahasia, tetapi membuat repositori pribadi tidak pernah menghapus rahasia atau data pribadi atau data terpercaya ke dalamnya, dan pengklasifikasi memperlakukan konten yang diport, diubah arah, atau pertama kali dibaca dari luar repositori kerja sebagai bukan pekerjaan repositori itu sendiri. Scoping ini memerlukan Claude Code v2.1.203 atau lebih baru.
* **Sensitivity slots**: nama apa yang diperlakukan aturan perlindungan sebagai berisiko tinggi. Slot tersebut adalah Sensitive data locations & audiences, Sensitive remote targets, dan Protected IaC scopes. Masing-masing default ke heuristik luas, seperti memperlakukan host atau namespace apa pun yang namanya membawa `prod` atau `production` sebagai target remote sensitif, jadi aturan perlindungan aktif sebelum Anda mengonfigurasi apa pun. Penamaan target konkret dalam sensitivity slot membuat aturan tersebut berlaku untuk target bernama daripada heuristik.

Untuk menambahkan entri Anda sendiri bersama default tersebut, sertakan string literal `"$defaults"` dalam array. Entri default disisipi pada posisi tersebut, jadi entri kustom Anda dapat berada sebelum atau sesudahnya.

Contoh berikut menjaga entri default dan menambahkan repo, bucket, domain, dan layanan organisasi.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Entri adalah prosa, bukan regex atau pola alat. Pengklasifikasi membacanya sebagai aturan bahasa alami. Tulislah dengan cara Anda akan menggambarkan infrastruktur Anda kepada insinyur baru. Bagian lingkungan yang menyeluruh mencakup:

* **Organisasi**: nama perusahaan Anda dan apa yang Claude Code digunakan terutama untuk, seperti pengembangan perangkat lunak, otomasi infrastruktur, atau rekayasa data
* **Kontrol sumber**: setiap GitHub, GitLab, atau org Bitbucket yang didorong pengembang Anda
* **Penyedia cloud dan bucket terpercaya**: nama bucket atau awalan yang Claude harus dapat membaca dari dan menulis ke
* **Domain internal terpercaya**: nama host untuk API, dasbor, dan layanan di dalam jaringan Anda, seperti `*.internal.example.com`
* **Layanan internal utama**: CI, registri artefak, indeks paket internal, tooling insiden
* **Registri paket internal**: registri npm, PyPI, atau lainnya yang pribadi yang instalasi harus dirutekan melalui, sehingga instalasi yang melewatinya untuk registri publik diblokir
* **Sensitive data locations & audiences**: bucket, database, atau path yang menyimpan data pribadi, data bisnis rahasia, kredensial, data yang diatur, atau materi sensitif serupa, dan audiens yang data di setiap lokasi dapat dibagikan dengannya, sehingga pengklasifikasi melindungi lokasi tersebut daripada menebak dari konten. Claude Code v2.1.195 hingga v2.1.197 menyebutkan entri ini sebagai lokasi PII / regulated-data dan hanya mencakup lokasi yang menyimpan data pribadi atau yang diatur, tanpa dimensi audiens
* **Target remote sensitif**: namespace, host, atau container yang dihitung sebagai production, sehingga remote shell dan port-forward ke dalamnya memerlukan persetujuan eksplisit Anda
* **Protected IaC scopes**: sumber daya infrastruktur yang apply atau destroy harus selalu memerlukan Anda untuk menyebutkan perubahan
* **Konteks tambahan**: batasan industri yang diatur, infrastruktur multi-tenant, atau persyaratan kepatuhan yang mempengaruhi apa yang harus diperlakukan pengklasifikasi sebagai berisiko

Entri Internal package registry, Sensitive data locations & audiences, Sensitive remote targets, dan Protected IaC scopes memerlukan Claude Code v2.1.195 atau lebih baru. Versi sebelumnya masih membacanya sebagai konteks biasa tetapi tidak memiliki aturan bawaan yang menargetkannya.

Template awal yang berguna: isi bidang dalam kurung dan hapus baris apa pun yang tidak berlaku.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Semakin spesifik konteks yang Anda berikan, semakin baik pengklasifikasi dapat membedakan operasi internal rutin dari upaya exfiltration.

Anda tidak perlu mengisi semuanya sekaligus. Peluncuran yang masuk akal: mulai dengan default dan tambahkan org kontrol sumber Anda dan layanan internal utama, yang menyelesaikan false positive paling umum seperti mendorong ke repo Anda sendiri. Tambahkan domain terpercaya dan bucket cloud selanjutnya. Isi sisanya saat blokir muncul.

<h2 id="override-the-block-and-allow-rules">
  Ganti aturan blokir dan izin
</h2>

Tiga bidang tambahan memungkinkan Anda mengganti daftar aturan bawaan pengklasifikasi:

* `autoMode.hard_deny`: batas keamanan tanpa syarat
* `autoMode.soft_deny`: tindakan destruktif yang niat pengguna dapat menghapus
* `autoMode.allow`: pengecualian untuk aturan blokir soft

Masing-masing adalah array deskripsi prosa, dibaca sebagai aturan bahasa alami. Untuk hard block berbasis pola alat yang berjalan sebelum pengklasifikasi, gunakan [`permissions.deny`](/docs/id/permissions).

Di dalam pengklasifikasi, prioritas bekerja dalam empat tingkat:

* Aturan `hard_deny` memblokir tanpa syarat. Niat pengguna dan pengecualian `allow` tidak berlaku.
* Aturan `soft_deny` memblokir selanjutnya. Niat pengguna dan pengecualian `allow` dapat mengganti ini.
* Aturan `allow` kemudian mengganti aturan `soft_deny` yang cocok sebagai pengecualian.
* Niat pengguna eksplisit mengganti blokir soft yang tersisa: jika pesan pengguna secara langsung dan spesifik menggambarkan tindakan yang tepat Claude akan ambil, pengklasifikasi mengizinkannya bahkan ketika aturan `soft_deny` cocok.

Permintaan umum tidak dihitung sebagai niat eksplisit. Meminta Claude untuk "membersihkan repo" tidak mengotorisasi force-push, tetapi meminta Claude untuk "force-push cabang ini" melakukannya.

Untuk melonggarkan, tambahkan ke `allow` ketika pengklasifikasi berulang kali menandai pola rutin yang pengecualian default tidak cover. Untuk mengencangkan, tambahkan ke `soft_deny` untuk risiko destruktif spesifik lingkungan Anda yang default lewatkan, atau ke `hard_deny` untuk batas keamanan yang tidak boleh pernah dilintasi.

Untuk menjaga aturan bawaan sambil menambahkan aturan Anda sendiri, sertakan string literal `"$defaults"` dalam array. Aturan default disisipi pada posisi itu, jadi aturan kustom Anda dapat berada sebelum atau sesudahnya, dan Anda terus mewarisi pembaruan saat daftar bawaan berubah di seluruh rilis.

Contoh berikut menjaga default di semua empat daftar dan menambahkan aturan spesifik organisasi ke masing-masing.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  Menetapkan salah satu dari `environment`, `allow`, `soft_deny`, atau `hard_deny` tanpa `"$defaults"` menggantikan seluruh daftar default untuk bagian itu. Jika Anda menetapkan array tanpa `"$defaults"`, Anda membuang aturan bawaan untuk bagian itu:

  * `soft_deny`: setiap aturan blokir soft bawaan, termasuk force push, `curl | bash`, production deploys, dan bypass auto-mode
  * `hard_deny`: aturan data exfiltration bawaan
</Danger>

Setiap bagian dievaluasi secara independen, jadi menetapkan `environment` saja membiarkan daftar `allow`, `soft_deny`, dan `hard_deny` default tetap utuh.

Hanya hilangkan `"$defaults"` ketika Anda bermaksud mengambil kepemilikan penuh atas daftar. Untuk melakukan itu dengan aman, jalankan `claude auto-mode defaults` untuk mencetak aturan bawaan, salin ke file pengaturan Anda, kemudian tinjau setiap aturan terhadap pipeline Anda sendiri dan toleransi risiko.

<h2 id="route-all-shell-commands-through-the-classifier">
  Perutean semua perintah shell melalui pengklasifikasi
</h2>

Secara default, aturan izin Bash dan PowerShell yang sempit seperti `Bash(npm test)` terbawa ke auto mode dan diselesaikan sebelum pengklasifikasi berjalan. Auto mode hanya menangguhkan aturan luas yang memberikan eksekusi kode arbitrer, seperti `Bash(*)` atau interpreter dengan wildcard. Ini berarti aturan sempit masih dapat membiarkan argumen destruktif melewati tanpa pengklasifikasi melihatnya, misalnya path skrip atau flag yang awalan aturan tidak antisipasi.

Atur `autoMode.classifyAllShell` ke `true` untuk menangguhkan setiap aturan izin Bash dan PowerShell saat auto mode aktif, sehingga pengklasifikasi mengevaluasi setiap perintah shell terlepas dari daftar izin Anda.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Ini menukar latensi untuk cakupan: perintah yang aturan izin akan setujui secara instan sekarang menunggu keputusan pengklasifikasi, dan setiap perintah shell dihitung sebagai panggilan pengklasifikasi.

Pengaturan hanya berlaku saat auto mode aktif, dan aturan izin Anda berperilaku normal dalam mode izin lainnya.

<Note>
  `autoMode.classifyAllShell` memerlukan Claude Code v2.1.193 atau lebih baru. Versi sebelumnya mengabaikan kunci dan terus membawa aturan izin shell sempit ke auto mode.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Periksa default dan konfigurasi efektif Anda
</h2>

Tiga subperintah CLI membantu Anda memeriksa dan memvalidasi konfigurasi Anda.

Cetak aturan `environment`, `allow`, `soft_deny`, dan `hard_deny` bawaan sebagai JSON:

```bash theme={null}
claude auto-mode defaults
```

Untuk membaca kata-kata lengkap satu aturan tanpa melalui pipa `jq`, berikan `--label` dengan awal label aturan, seperti `claude auto-mode defaults --label 'Git Destructive'`. Pencocokan adalah awalan yang tidak peka huruf besar-kecil pada label setiap aturan, dan bagian tanpa kecocokan dicetak sebagai daftar kosong. Memerlukan Claude Code v2.1.208 atau lebih baru.

Cetak apa yang sebenarnya digunakan pengklasifikasi sebagai JSON, dengan pengaturan Anda diterapkan di mana diatur dan default sebaliknya:

```bash theme={null}
claude auto-mode config
```

Dapatkan umpan balik AI tentang aturan `allow`, `soft_deny`, dan `hard_deny` kustom Anda:

```bash theme={null}
claude auto-mode critique
```

Jalankan `claude auto-mode config` setelah menyimpan pengaturan Anda untuk mengonfirmasi bahwa aturan efektif adalah apa yang Anda harapkan, dengan `"$defaults"` diperluas di tempat. Jika Anda telah menulis aturan kustom, `claude auto-mode critique` meninjau mereka dan menandai entri yang ambigu, berlebihan, atau mungkin menyebabkan false positif.

Jika Anda perlu menghapus atau menulis ulang aturan bawaan daripada menambahkan di sampingnya, simpan output `claude auto-mode defaults` ke file, edit daftarnya, dan tempel hasilnya ke file pengaturan Anda sebagai pengganti `"$defaults"`.

<h2 id="review-denials">
  Tinjau penolakan
</h2>

Ketika auto mode menolak panggilan alat, penolakan dicatat di `/permissions` di bawah tab Recently denied. Tekan `r` pada tindakan yang ditolak untuk menandainya untuk retry: ketika Anda keluar dari dialog, Claude Code mengirim pesan memberi tahu model itu dapat retry panggilan alat itu dan melanjutkan percakapan.

Dalam Claude Code v2.1.193 dan lebih baru, alasan pengklasifikasi untuk setiap penolakan muncul bersama panggilan alat yang diblokir dalam transkrip, dalam notifikasi penolakan, dan di bawah setiap entri pada tab Recently denied. Gunakan alasan untuk memutuskan apakah perbaikannya adalah entri `environment`, pengecualian `allow`, atau retry dengan niat eksplisit dalam pesan Anda berikutnya.

Penolakan berulang untuk tujuan yang sama biasanya berarti pengklasifikasi kehilangan konteks. Tambahkan tujuan itu ke `autoMode.environment`, kemudian jalankan `claude auto-mode config` untuk mengonfirmasi itu berlaku.

Untuk bereaksi terhadap penolakan secara terprogram, gunakan hook [`PermissionDenied`](/docs/id/hooks#permissiondenied).

<h2 id="see-also">
  Lihat juga
</h2>

* [Permission modes](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): apa itu auto mode, apa yang diblokir secara default, dan cara mengaktifkannya
* [Pengaturan terkelola](/docs/id/server-managed-settings): sebarkan konfigurasi `autoMode` di seluruh organisasi Anda
* [Permissions](/docs/id/permissions): aturan izin, tanya, dan tolak yang berlaku sebelum pengklasifikasi berjalan
* [Settings](/docs/id/settings): referensi pengaturan lengkap, termasuk kunci `autoMode`
