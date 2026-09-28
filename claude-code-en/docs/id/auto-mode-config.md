> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi mode otomatis

> Beri tahu pengklasifikasi mode otomatis repositori, bucket, dan domain mana yang dipercaya organisasi Anda. Atur konteks lingkungan, ganti aturan blokir dan izin default, dan periksa konfigurasi efektif Anda dengan subperintah CLI mode otomatis.

[Mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) memungkinkan Claude Code berjalan tanpa permintaan izin rutin dengan merutekan panggilan alat melalui pengklasifikasi yang memblokir apa pun yang tidak dapat dibalikkan, merusak, atau ditujukan di luar lingkungan Anda. Aturan penolakan dan permintaan eksplisit dievaluasi sebelum pengklasifikasi dan masih memblokir atau meminta. Gunakan blok pengaturan `autoMode` untuk memberi tahu pengklasifikasi tersebut repositori, bucket, dan domain mana yang dipercaya organisasi Anda, sehingga berhenti memblokir operasi internal rutin.

<Note>
  Mode otomatis tersedia untuk semua pengguna di setiap penyedia, termasuk Anthropic API, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang masuk. Jika Claude Code melaporkan mode otomatis tidak tersedia untuk akun Anda, periksa [persyaratan lengkap](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), yang juga mencakup model yang didukung dan kontrol tingkat organisasi pada paket Tim dan Enterprise. Dalam v2.1.158 hingga v2.1.206, mode otomatis di Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan sesi gateway aplikasi Claude memerlukan pengaturan `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 menghapus persyaratan.
</Note>

Secara default, pengklasifikasi hanya mempercayai direktori kerja dan remote yang dikonfigurasi dari repositori saat ini. Tindakan seperti mendorong ke organisasi kontrol sumber perusahaan Anda atau menulis ke bucket cloud tim diblokir sampai Anda menambahkannya ke `autoMode.environment`.

Untuk cara sesi berakhir dalam mode otomatis dan apa yang diblokir pengklasifikasi secara default, lihat [mode otomatis di halaman Permission modes](/docs/id/permission-modes#eliminate-prompts-with-auto-mode). Halaman ini adalah referensi konfigurasi.

Halaman ini mencakup cara:

* [Tambahkan checkpoint manusia](#add-a-human-checkpoint) untuk push dan pull request dengan `permissions.ask`
* [Pilih tempat untuk menetapkan aturan](#where-the-classifier-reads-configuration) di seluruh CLAUDE.md, pengaturan pengguna, dan pengaturan terkelola
* [Tentukan infrastruktur terpercaya](#define-trusted-infrastructure) dengan `autoMode.environment`
* [Hasilkan entri lingkungan](#generate-environment-entries) dengan `/auto-mode-setup`
* [Ganti aturan blokir dan izin](#override-the-block-and-allow-rules) ketika default tidak sesuai dengan pipeline Anda
* [Edit aturan dari `/permissions`](#edit-rules-from-permissions) tanpa membuka file pengaturan
* [Rutekan semua perintah shell melalui pengklasifikasi](#route-all-shell-commands-through-the-classifier) dengan `autoMode.classifyAllShell`
* [Periksa konfigurasi efektif Anda](#inspect-the-defaults-and-your-effective-config) dengan subperintah `claude auto-mode`
* [Tinjau penolakan](#review-denials) sehingga Anda tahu apa yang harus ditambahkan selanjutnya

<h2 id="common-boundaries">
  Batas-batas umum
</h2>

Mode otomatis memungkinkan push ke cabang mana pun dari repositori yang Anda kerjakan, termasuk cabang default, dan pembuatan pull request secara default. Cabang non-default yang namanya menandainya sebagai target deploy atau publikasi, seperti `production`, `release`, atau `gh-pages`, tidak tercakup oleh default tersebut: pengklasifikasi menilai push di sana berdasarkan syarat-syaratnya sendiri, termasuk sebagai deploy produksi. Konten push juga masih diperiksa, jadi force push, rahasia yang memasuki commit, atau perubahan yang akan mengirim rahasia di luar repositori ketika CI atau pipeline deploy menjalankannya tetap diblokir.

<Info>Sebelum v2.1.211, pengklasifikasi hanya memungkinkan push ke cabang kerja Anda, cabang yang dibuat Claude, dan push rutin ke cabang default.</Info>

Jika Anda ingin checkpoint manusia sebelum perintah push dan pull request Claude, tambahkan aturan izin: [resep di bawah](#add-a-human-checkpoint) menjaga mode otomatis tetap aktif untuk segalanya.

<h3 id="add-a-human-checkpoint">
  Tambahkan checkpoint manusia
</h3>

Mekanisme paling langsung adalah [`permissions.ask`](/docs/id/permissions#permission-rule-syntax). Aturan ask yang dibatasi konten seperti yang di bawah ini dievaluasi sebelum pengklasifikasi dan selalu memaksa prompt izin, bahkan dalam mode otomatis, karena aturan ask eksplisit adalah niat yang dinyatakan untuk diminta untuk tindakan tersebut. Tambahkan aturan di [settings](/docs/id/settings#where-settings-live) Anda:

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

Aturan-aturan ini cocok dengan perintah yang dimulai dengan `git push` atau `gh pr create`. Push yang Claude tulis dengan cara lain, seperti `git -C <dir> push` atau `git -c <key>=<value> push`, [tidak cocok dengan aturan](/docs/id/permissions#bash-rule-limits), jadi tidak diperiksa. Untuk checkpoint yang memeriksa teks perintah lengkap, tambahkan hook [PreToolUse](/docs/id/hooks#pretooluse).

Pilih mekanisme yang sesuai dengan seberapa tegas batas yang diperlukan:

| Batas                           | Mekanisme                                                           | Perilaku dalam mode otomatis                                                                                                                                                                                                     |
| :------------------------------ | :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Prompt sebelum tindakan         | `permissions.ask`                                                   | Selalu meminta untuk perintah yang cocok dengan aturan yang dibatasi konten seperti resep di atas. Pengklasifikasi tidak dapat auto-approve tindakan yang cocok.                                                                 |
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

Untuk sebagian besar organisasi, `autoMode.environment` adalah satu-satunya bidang yang perlu Anda atur. Ini memberitahu pengklasifikasi repositori, bucket, dan domain mana yang terpercaya: pengklasifikasi menggunakannya untuk memutuskan apa arti "eksternal", jadi tujuan apa pun yang tidak tercantum adalah target potensi exfiltration.

Mulai dari Claude Code v2.1.198, `claude auto-mode defaults` mencetak tiga jenis entri lingkungan. Versi sebelum v2.1.195 hanya mencetak lima slot kepercayaan pertama.

* **Context slots**: mendeskripsikan organisasi, stack, dan postur keamanan Anda sehingga pengklasifikasi membaca aturan lain dalam konteks Anda. Masing-masing default ke `None configured` atau ke asumsi konservatif yang dinamai di sebelahnya:
  * **Organization**
  * **Primary use of Claude Code**: default ke software development
  * **Cloud provider(s)**
  * **Repository visibility**: sebuah repositori diasumsikan private kecuali host remote dan namanya menunjukkan sebaliknya, atau pengklasifikasi membaca pemeriksaan visibilitas lebih awal dalam percakapan yang menunjukkan bahwa itu public.

    Dalam permintaan pengklasifikasi yang dikirim oleh Claude Code itu sendiri, pengklasifikasi membaca pesan Anda dan perintah yang dijalankan Claude, bukan output mereka. Bukti harus berupa sesuatu yang dapat dibaca pengklasifikasi, seperti pesan Anda sendiri yang menyebutkan repositori sebagai public; output dari `gh repo view` saja tidak mencapainya. Pemeriksaan bukti transkrip memerlukan Claude Code v2.1.200 atau lebih baru
  * **Internal sharing / snippet hosting**: layanan paste dan gist publik diperlakukan sebagai di luar batas kepercayaan sampai Anda menyebutkan satu
  * **Org-specific CLIs**
  * **Secrets management**
  * **CI/CD deploy targets**
  * **Network posture**
  * **Host containment**: default ke mesin developer biasa atau CI runner dengan internet terbuka. Jika Claude Code berjalan dalam container, VM, atau pod dengan daftar allow-list egress atau tetangga yang tidak boleh disentuh, sebutkan host yang diizinkan, apakah endpoint metadata cloud harus dapat dijangkau, dan proyek cloud, cluster, atau registry mana yang digunakan tugas dan di bawah identitas apa. Sampai entri ini menyebutkan identitas itu, pengklasifikasi [memblokir](/docs/id/permission-modes#what-the-classifier-blocks-by-default) permintaan untuk kredensial host itu sendiri. Memerlukan Claude Code v2.1.257 atau lebih baru
  * **Protected deployment namespaces / environments**: kembali ke heuristik Sensitive remote targets sampai Anda menyebutkannya
  * **Data retention / declassification**
* **Trust slots**: sebutkan apa yang diperlakukan pengklasifikasi sebagai dalam batas Anda. Slot adalah Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services, dan Internal package registry. Entri repo dan source-control default ke repositori kerja dan remote yang dikonfigurasinya. Setiap slot kepercayaan lainnya default ke `None configured`, jadi tidak ada yang lain terpercaya sampai Anda menambahkannya. Visibilitas repositori hanya mencakup materi rahasia: repositori private adalah tujuan yang dapat diterima untuk materi rahasia, tetapi membuat repositori private tidak pernah menghapus rahasia atau data pribadi atau terpercaya ke dalamnya, dan pengklasifikasi memperlakukan konten yang diport, diarahkan ulang, atau pertama kali dibaca dari luar repositori kerja sebagai bukan pekerjaan repositori itu sendiri. Scoping ini memerlukan Claude Code v2.1.203 atau lebih baru.
* **Sensitivity slots**: sebutkan apa yang diperlakukan aturan perlindungan sebagai berisiko tinggi. Slot adalah Sensitive data locations & audiences, Sensitive remote targets, dan Protected IaC scopes. Masing-masing default ke heuristik yang luas, seperti memperlakukan host atau namespace apa pun yang namanya membawa `prod` atau `production` sebagai target remote sensitif, jadi aturan perlindungan aktif sebelum Anda mengonfigurasi apa pun. Menyebutkan target konkret dalam slot sensitivitas membuat aturan tersebut berlaku untuk target yang disebutkan alih-alih heuristik.

<Info>Sebelum v2.1.211, context slots juga menyertakan entri Default / protected branches yang memperlakukan `main` dan `master` sebagai protected sampai Anda menyebutkan yang lain. v2.1.211 menghapusnya: [push ke cabang apa pun dari repositori yang Anda kerjakan](#common-boundaries) diizinkan secara default, jadi tidak ada default protected-branch untuk dikonfigurasi.</Info>

Untuk menambahkan entri Anda sendiri bersama default, sertakan string literal `"$defaults"` dalam array. Entri default disambung pada posisi itu, jadi entri kustom Anda dapat berada sebelum atau sesudahnya.

Contoh berikut mempertahankan entri default dan menambahkan repo, bucket, domain, dan layanan organisasi.

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

Setelah Anda menyimpan pengaturan Anda, jalankan `claude auto-mode config` untuk [mengkonfirmasi aturan efektif](#inspect-the-defaults-and-your-effective-config) mencakup entri Anda.

Entri adalah prosa, bukan regex atau pola alat. Pengklasifikasi membacanya sebagai aturan bahasa alami. Tulislah dengan cara Anda akan mendeskripsikan infrastruktur Anda kepada engineer baru. Bagian lingkungan yang menyeluruh mencakup:

* **Organization**: nama perusahaan Anda dan apa Claude Code terutama digunakan untuk, seperti software development, infrastructure automation, atau data engineering
* **Source control**: setiap GitHub, GitLab, atau Bitbucket org yang didorong developer Anda
* **Cloud providers and trusted buckets**: nama bucket atau awalan yang Claude harus dapat membaca dan menulis
* **Trusted internal domains**: hostname untuk API, dashboard, dan layanan di dalam jaringan Anda, seperti `*.internal.example.com`
* **Key internal services**: CI, artifact registries, internal package indexes, incident tooling
* **Internal package registry**: npm pribadi, PyPI, atau registry lain yang harus dirutekan install, jadi install yang melewatinya untuk registry publik mendapat blokir
* **Sensitive data locations & audiences**: bucket, database, atau path yang menyimpan data pribadi, data bisnis rahasia, kredensial, data yang diatur, atau materi sensitif serupa, dan audiens yang data di setiap lokasi dapat dibagikan, jadi pengklasifikasi melindungi lokasi tersebut alih-alih menebak dari konten. Claude Code v2.1.195 melalui v2.1.197 menyebutkan entri ini PII / regulated-data locations dan hanya mencakup lokasi yang menyimpan data pribadi atau yang diatur, tanpa dimensi audiens
* **Sensitive remote targets**: namespace, host, atau container yang dihitung sebagai production, jadi remote shell dan port-forward ke dalamnya memerlukan persetujuan eksplisit Anda
* **Protected IaC scopes**: resource infrastruktur yang apply atau destroy harus selalu memerlukan Anda untuk menyebutkan perubahan
* **Additional context**: batasan industri yang diatur, infrastruktur multi-tenant, atau persyaratan kepatuhan yang mempengaruhi apa yang harus diperlakukan pengklasifikasi sebagai berisiko

Entri Internal package registry, Sensitive data locations & audiences, Sensitive remote targets, dan Protected IaC scopes memerlukan Claude Code v2.1.195 atau lebih baru. Versi sebelumnya masih membacanya sebagai konteks biasa tetapi tidak memiliki aturan built-in yang menargetkannya.

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

Anda tidak perlu mengisi semuanya sekaligus. Rollout yang masuk akal: mulai dengan default dan tambahkan org source control dan layanan internal kunci Anda, yang menyelesaikan false positive paling umum seperti push ke repo Anda sendiri. Tambahkan domain terpercaya dan cloud bucket berikutnya. Isi sisanya saat blok muncul.

<h2 id="generate-environment-entries">
  Buat entri lingkungan dengan `/auto-mode-setup`
</h2>

Jalankan `/auto-mode-setup` untuk membuat Claude Code membuat draf entri `autoMode.environment`, dan kadang-kadang juga [entri aturan](#override-the-block-and-allow-rules), dari proyek Anda dan sesi terbaru Anda di dalamnya. Jika Anda menerima draf, Claude Code menulisnya ke `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` memerlukan paket Pro, Max, atau Team dan Claude Code v2.1.228 atau lebih baru. Di Windows native memerlukan v2.1.233 atau lebih baru. Anda tidak dapat menjalankannya di [Claude Code di web](/docs/id/claude-code-on-the-web). Ini juga memerlukan [pengambilan bendera fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching), jadi Anda tidak dapat menjalankannya dalam sesi di mana Anda telah mematikan pengambilan bendera.
</Note>

<h3 id="what-auto-mode-setup-reads">
  Apa yang dibaca `/auto-mode-setup`
</h3>

Jika `~/.claude/settings.json` sudah menyimpan entri `autoMode`, Claude Code dimulai dengan menanyakan apakah akan menambah daftar lingkungan Anda atau menggantinya, dan tetap menyimpan aturan yang Anda tulis bagaimanapun. Claude Code kemudian menanyakan bagaimana Anda menggunakan proyek ini dan menawarkan dua pemindaian opsional sebelum memindai apa pun. Dalam pemindaian, Claude Code selalu membaca sumber-sumber ini:

* `CLAUDE.md`, `README.md`, file konfigurasi, dan git remotes proyek ini
* Pengaturan `autoMode` dan `permissions.allow` Anda
* Host, bucket, dan nama perintah dari perintah yang Claude jalankan dalam sesi terbaru Anda di proyek ini, tidak pernah pesan Anda

Dua pemindaian opsional menambahkan satu sumber masing-masing:

* Kata pertama dari setiap perintah dalam riwayat shell Anda
* Host jarak jauh dan nama repositori di bawah direktori home Anda

<h3 id="review-and-save-the-draft">
  Tinjau dan simpan draf
</h3>

Claude Code memindai di latar belakang, kemudian menampilkan draf kepada Anda. Anda menerima atau membuang semuanya sebagai satu kesatuan, jadi edit `~/.claude/settings.json` setelahnya untuk menyesuaikan entri tunggal. Ketika Anda menerima, Claude Code menulis draf dan merekonsiliasi dengan pengaturan yang sudah Anda miliki:

* Claude Code menulis daftar `environment` tanpa `"$defaults"`, karena draf menjabarkan entri bawaan yang tidak diubahnya
* Claude Code menyertakan `"$defaults"` dalam setiap daftar `allow`, `soft_deny`, dan `hard_deny` yang ditambahkan draf ke dalamnya, kecuali Anda sudah menulis daftar `allow` tanpanya, jadi [aturan bawaan](#override-the-block-and-allow-rules) yang belum Anda ganti tetap berlaku
* Setelah menyimpan, Claude Code menawarkan untuk menghapus aturan `permissions.allow` di `~/.claude/settings.json` yang diabaikan mode otomatis, seperti `Bash(*)`, atau yang secara otomatis menyetujui perintah destruktif

Kemudian jalankan `claude auto-mode config` untuk [melihat hasil yang efektif](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  Matikan `/auto-mode-setup`
</h3>

Setelah mode otomatis telah memblokir beberapa tindakan dan Anda masih tidak memiliki entri `autoMode.environment`, Claude Code menampilkan dialog berjudul "Ajarkan mode otomatis tentang lingkungan Anda?" di akhir giliran dan menawarkan untuk menjalankan `/auto-mode-setup` untuk Anda. Untuk menghentikan penawaran tetapi menyimpan perintah, pilih **Jangan tampilkan lagi** dalam dialog itu.

Untuk mematikan perintah dan penawaran, tambahkan entri [`skillOverrides`](/docs/id/skills#override-skill-visibility-from-settings) ini ke `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` adalah perintah bawaan daripada [skill bundel](/docs/id/skills#bundled-skills), jadi entri `skillOverrides` ini masih berlaku untuk itu, tetapi [`disableBundledSkills`](/docs/id/settings-reference#disablebundledskills) tidak mematikannya.

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

<h2 id="edit-rules-from-permissions">
  Edit rules from `/permissions`
</h2>

Untuk melihat dan mengedit aturan classifier tanpa membuka file pengaturan, jalankan [`/permissions`](/docs/id/permissions#manage-permissions) dan pilih tab **Auto mode**. Tab tersebut memerlukan Claude Code v2.1.246 atau lebih baru, dan hanya muncul ketika [auto mode tersedia](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) untuk sesi Anda.

Tab ini mencantumkan entri `allow`, `soft_deny`, `hard_deny`, dan `environment` dari masing-masing [scope yang dibaca classifier](#where-the-classifier-reads-configuration), dan menunjukkan apakah aturan bawaan berlaku untuk setiap bagian. Claude Code menampilkan entri dari [managed settings](/docs/id/server-managed-settings) atau flag `--settings` sebagai read-only, dan menyimpan setiap perubahan yang Anda buat di tab ke `~/.claude/settings.json`. Dari tab ini Anda dapat:

* Menambah, mengedit, atau menghapus aturan di bagian `allow`, `soft_deny`, dan `hard_deny`. Ketika Anda menambahkan aturan pertama ke bagian, Claude Code juga menyisipkan `"$defaults"` sehingga [aturan bawaan](#override-the-block-and-allow-rules) tetap berlaku.
* Mematikan atau menghidupkan kembali aturan bawaan untuk `allow`, `soft_deny`, atau `hard_deny`. Claude Code mencatat pilihan dengan menambahkan atau menghapus `"$defaults"` dalam daftar Anda untuk bagian tersebut, jadi bagian memerlukan setidaknya satu aturan Anda sendiri sebelum Anda dapat mematikan aturan bawaannya.
* Mengedit entri `environment` sebagai satu dokumen di editor Anda. Jika Anda belum mengonfigurasi entri `environment` apa pun, Claude Code terlebih dahulu menanyakan apakah akan mengganti environment bawaan, kemudian membuka editor pada teks bawaan lengkap. Ketika Anda menyimpan, Claude Code mengganti array `autoMode.environment` Anda dengan dokumen. Sertakan baris `"$defaults"` untuk [menjaga entri bawaan](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Arahkan semua perintah shell melalui pengklasifikasi
</h2>

Secara default, aturan Bash dan PowerShell yang sempit seperti `Bash(npm test)` tetap berlaku dalam mode otomatis. Claude Code menyelesaikannya sebelum pengklasifikasi berjalan, kecuali perintah membawa [domain yang diizinkan per-perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code menangguhkan hanya aturan luas yang memberikan eksekusi kode arbitrer, seperti `Bash(*)` atau interpreter dengan wildcard, bersama dengan setiap aturan yang menyebutkan [`Monitor`](/docs/id/tools-reference#monitor-tool), karena perintah Monitor berjalan melalui shell. Ini berarti aturan yang sempit masih dapat membiarkan argumen yang merusak melewati tanpa pengklasifikasi melihatnya, misalnya jalur skrip atau flag yang tidak diantisipasi oleh awalan aturan.

Atur `autoMode.classifyAllShell` ke `true` untuk menangguhkan setiap aturan izin Bash dan PowerShell saat mode otomatis aktif, sehingga pengklasifikasi mengevaluasi setiap perintah shell terlepas dari daftar izin Anda.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Ini menukar latensi untuk cakupan: perintah yang akan disetujui oleh aturan izin secara instan sekarang menunggu keputusan pengklasifikasi, dan setiap perintah shell dihitung sebagai panggilan pengklasifikasi.

Pengaturan hanya berlaku saat mode otomatis aktif, dan aturan izin Anda berperilaku normal dalam mode izin lainnya.

<Note>
  `autoMode.classifyAllShell` memerlukan Claude Code v2.1.193 atau lebih baru. Versi sebelumnya mengabaikan kunci dan terus membawa aturan izin shell yang sempit ke dalam mode otomatis.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Periksa default dan konfigurasi efektif Anda
</h2>

Subperintah `claude auto-mode` membantu Anda memeriksa, memvalidasi, dan mengatur ulang konfigurasi Anda.

Cetak aturan bawaan `environment`, `allow`, `soft_deny`, dan `hard_deny` sebagai JSON:

```bash theme={null}
claude auto-mode defaults
```

Untuk membaca kata-kata lengkap satu aturan tanpa melalui pipa `jq`, berikan `--label` dengan awal label aturan, seperti `claude auto-mode defaults --label 'Git Destructive'`. Pencocokan adalah awalan yang tidak peka huruf besar-kecil pada label setiap aturan, dan bagian tanpa kecocokan dicetak sebagai daftar kosong. Memerlukan Claude Code v2.1.208 atau lebih baru.

Cetak apa yang sebenarnya digunakan pengklasifikasi sebagai JSON, dengan pengaturan Anda diterapkan di mana diatur dan default sebaliknya:

```bash theme={null}
claude auto-mode config
```

Baik `defaults` maupun `config` mencetak empat daftar aturan sebagai satu objek JSON, dengan setiap aturan sebagai string prosa. Ini adalah contoh yang dipotong:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, atau hardcoding test cases. Placeholder berarti ditulis sebagai placeholder — file atau nilai yang disalin dari rahasia nyata atau jalur sensitif tidak pernah merupakan artefak pengujian (lihat Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), menghapus cabang jarak jauh, tag, atau rilis, atau menulis ulang riwayat jarak jauh. Juga `git commit --amend` ketika komit yang ditulis ulang bukan pekerjaan yang belum didorong agen: baik tidak ada `git commit` sebelumnya yang terlihat (HEAD mendahului sesi), atau `git push` dari cabang saat ini terlihat setelah komit terbaru (telah didorong). Dihapus ketika pengguna meminta untuk mengubah/mengubah kata-kata/memperbaiki, atau ketika itu adalah pengubahan kata-kata pesan saja (`--amend -m …`, tidak ada yang baru dipentaskan) dari komit yang agen buat secara terlihat sesi ini.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: Repositori git tempat agen dimulai (direktori kerjanya) dan remote(s) yang dikonfigurasinya. Ketika visibilitas publik/pribadi repo diberikan — oleh entri Repository visibility atau pesan pengguna sendiri — gunakan untuk menentukan ruang lingkup apa yang OK untuk dilakukan atau didorong di sana: materi rahasia baik-baik saja di repo pribadi; di repo publik, hanya pekerjaan repo itu sendiri — dan konten yang diportasi, ditunjuk kembali, atau pertama kali dibaca dari luar repo sesi ini bukan pekerjaan miliknya, siapa pun yang mengarahkan port. Visibilitas menentukan ruang lingkup materi rahasia saja: rahasia dan data sensitif (pribadi & dipercaya) tidak pernah dihapus ke repo apa pun oleh visibilitasnya (lihat Definitions).",
    ...
  ]
}
```

Dapatkan umpan balik AI tentang aturan `allow`, `soft_deny`, dan `hard_deny` kustom Anda:

```bash theme={null}
claude auto-mode critique
```

Jalankan `claude auto-mode config` setelah menyimpan pengaturan Anda untuk mengonfirmasi bahwa aturan efektif adalah apa yang Anda harapkan, dengan `"$defaults"` diperluas di tempat. Jika Anda telah menulis aturan kustom, `claude auto-mode critique` meninjau mereka dan menandai entri yang ambigu, berlebihan, atau kemungkinan menyebabkan positif palsu.

Untuk membuang kustomisasi Anda dan kembali ke default bawaan, jalankan subperintah reset. Ini memerlukan Claude Code v2.1.212 atau lebih baru dan menghapus bagian `autoMode` dari file pengaturan pengguna Anda:

```bash theme={null}
claude auto-mode reset
```

Perintah merangkum apa yang akan dihapus dan menanyakan `Reset auto mode configuration to defaults?` sebelum menulis; berikan `--yes` untuk melewati konfirmasi. Reset hanya mengubah `~/.claude/settings.json`: aturan `autoMode` dari [managed settings](/docs/id/server-managed-settings) atau bendera `--settings` masih berlaku.

<h2 id="review-denials">
  Tinjau penolakan
</h2>

Untuk meninjau dan mencoba kembali tindakan yang ditolak oleh pengklasifikasi mode otomatis, buka `/permissions` dan pilih tab **Recently denied**, di mana Claude Code mencatat setiap penolakan. Tekan `r` pada tindakan yang ditolak untuk menandainya untuk dicoba kembali: ketika Anda keluar dari dialog, Claude Code mengirimkan pesan yang memberitahu model bahwa mungkin dapat mencoba kembali panggilan alat tersebut dan melanjutkan percakapan.

Ketika pengklasifikasi menghasilkan [tidak ada putusan tentang tindakan](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action), karena pemeriksaan keamanan terpisah dari mode otomatis menolak permintaan pengklasifikasi sendiri atau responsnya tidak dapat diuraikan, Claude Code menolak tindakan tanpa mencatatnya di bawah **Recently denied**. Entri kesalahan yang ditautkan mencakup apa yang diberitahu kepada Claude dan cara menjalankan tindakan jika Anda membutuhkannya.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Perbaiki penolakan dengan aturan izin, entri lingkungan, atau percobaan ulang
</h3>

Untuk melihat apa yang diblokir oleh pengklasifikasi, temukan panggilan alat dalam percakapan. Jika panggilan muncul diperpendek atau dilipat ke dalam baris ringkasan seperti `Ran 3 shell commands`, tekan `Ctrl+O` untuk membuka [penampil transkrip](/docs/id/interactive-mode#transcript-viewer), yang memperluas panggilan tersebut.

Dua tempat lain di layar yang melaporkan penolakan menghilangkan perintah atau URL: pemberitahuan di dekat kotak input, seperti `bash denied by auto mode · [Data Exfiltration] · /permissions`, memberikan alat dan alasannya, dan tab **Recently denied** mencantumkan perintah shell berdasarkan deskripsi yang ditulis Claude untuknya. Untuk menangkap input yang tepat dari penolakan ini secara terprogram, tambahkan [`PermissionDenied` hook](/docs/id/hooks#permissiondenied), yang menerimanya sebagai `tool_input`.

Teks di bawah panggilan memberi tahu Anda apakah ada yang perlu diperbaiki. Teks yang melaporkan masalah dengan pengklasifikasi itu sendiri, seperti model yang `is temporarily unavailable` atau kesalahan pengklasifikasi, berarti Claude Code memblokir panggilan tanpa putusan akhir dari pengklasifikasi; lihat [Auto mode cannot determine the safety of an action](/docs/id/errors#auto-mode-cannot-determine-the-safety-of-an-action) untuk mengetahui apa yang harus dilakukan. Jika tidak, baris yang berbunyi `Denied by auto mode classifier` dengan alasan seperti `[Production Deploy]` atau `Blocked by classifier` berarti pengklasifikasi menilai panggilan tidak aman, jadi pilih perbaikan dari apa yang dicoba oleh panggilan untuk dicapai atau lakukan:

* Tujuan yang Claude butuhkan sepanjang tugas, seperti registri paket, domain internal, atau host repositori: tambahkan ke `autoMode.environment`.
* Perintah yang ingin Anda jalankan tanpa tinjauan dari sekarang: tambahkan aturan `allow`.
* Tindakan sekali pakai yang memang Anda maksudkan: nyatakan niat itu dalam pesan berikutnya dan biarkan Claude mencoba kembali.

Anda dapat menambahkan entri lingkungan atau aturan `allow` dari tab [**Auto mode**](#edit-rules-from-permissions) dialog `/permissions`.

Dalam sebagian besar sesi nama alasan menyebutkan aturan yang cocok dengan pengklasifikasi, dalam tanda kurung siku, seperti `[Data Exfiltration]` atau `[Production Deploy]`, dan beberapa sesi menjalankan model pengklasifikasi yang menambahkan penjelasan singkat. Claude Code memilih model pengklasifikasi, jadi bentuk mana yang Anda lihat bukan sesuatu yang Anda konfigurasi.

<h3 id="fix-repeated-denials">
  Perbaiki penolakan berulang
</h3>

Penolakan berulang untuk tujuan yang sama biasanya berarti pengklasifikasi kehilangan konteks. Tambahkan tujuan itu ke `autoMode.environment`, atau [jalankan `/auto-mode-setup`](#generate-environment-entries) untuk membuat Claude Code membuat draf entri, kemudian jalankan `claude auto-mode config` untuk mengonfirmasi perubahan telah diterapkan.

Untuk bereaksi terhadap penolakan secara terprogram, gunakan [`PermissionDenied` hook](/docs/id/hooks#permissiondenied).

<h2 id="see-also">
  Lihat juga
</h2>

* [Mode izin](/docs/id/permission-modes#eliminate-prompts-with-auto-mode): apa itu mode otomatis, apa yang diblokir secara default, dan sesi mana yang dimulai di dalamnya
* [Pengaturan terkelola](/docs/id/server-managed-settings): terapkan konfigurasi `autoMode` di seluruh organisasi Anda
* [Izin](/docs/id/permissions): aturan izinkan, tanya, dan tolak yang berlaku sebelum pengklasifikasi berjalan
* [Semua pengaturan](/docs/id/settings-reference#automode): setiap kunci pengaturan, termasuk `autoMode`
