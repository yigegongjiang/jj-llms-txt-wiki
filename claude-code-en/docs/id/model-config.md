> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi model

> Konfigurasikan model mana yang digunakan Claude Code, tingkat upaya, konteks yang diperluas, dan jendela auto-compact

<h2 id="available-models">
  Model yang tersedia
</h2>

Untuk pengaturan `model` di Claude Code, Anda dapat mengonfigurasi salah satu dari:

* **alias model**
* **nama model**
  * Anthropic API: **[nama model](https://platform.claude.com/docs/en/about-claude/models/overview)** lengkap
  * Amazon Bedrock: ARN profil inferensi
  * Microsoft Foundry: nama deployment
  * Google Cloud's Agent Platform: nama versi

Untuk panduan tentang model dan tingkat upaya mana yang sesuai untuk berbagai jenis pekerjaan, lihat [Memilih model Claude dan tingkat upaya di Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) di blog.

<Note>
  `ANTHROPIC_BASE_URL` mengubah tempat permintaan dikirim, bukan model mana yang menjawabnya. Untuk merutekan Claude melalui gateway LLM, lihat [LLM gateways](/docs/id/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias model
</h3>

Gunakan alias model untuk memilih pengaturan model tanpa perlu mengingat nomor versi yang tepat:

| Alias model      | Perilaku                                                                                                                                                                                                                                                                                                                                                  |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Nilai khusus yang menghapus penggantian model apa pun dan kembali ke [default runtime untuk akun Anda](#default-model-setting). Bukan sendiri alias model                                                                                                                                                                                                 |
| **`best`**       | Menggunakan model yang diselesaikan oleh alias [`fable`](#fable-alias-resolution) di mana Fable tersedia untuk Anda, jika tidak maka model yang sama dengan `opus`                                                                                                                                                                                        |
| **`fable`**      | Menggunakan [model Fable untuk penyedia Anda](#fable-alias-resolution) untuk tugas-tugas tersulit dan paling lama                                                                                                                                                                                                                                         |
| **`sonnet`**     | Menggunakan model Sonnet terbaru untuk tugas-tugas pengkodean sehari-hari                                                                                                                                                                                                                                                                                 |
| **`opus`**       | Menggunakan model Opus terbaru untuk tugas-tugas penalaran kompleks                                                                                                                                                                                                                                                                                       |
| **`haiku`**      | Menggunakan model Haiku yang cepat dan efisien untuk tugas-tugas sederhana                                                                                                                                                                                                                                                                                |
| **`sonnet[1m]`** | Menggunakan Sonnet dengan [jendela konteks 1 juta token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang. Tidak berpengaruh ketika `sonnet` sudah menyelesaikan ke Sonnet 5 dengan jendela 1M aslinya; di balik [gateway LLM](/docs/id/llm-gateway), memilih jendela 1M untuk Sonnet 5 |
| **`opus[1m]`**   | Menggunakan Opus dengan [jendela konteks 1 juta token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang                                                                                                                                                                            |
| **`opusplan`**   | Mode khusus yang menggunakan `opus` selama plan mode, kemudian beralih ke `sonnet` untuk eksekusi                                                                                                                                                                                                                                                         |

Versi yang diselesaikan oleh alias `opus` dan `sonnet` tergantung pada penyedia:

| Penyedia                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| Anthropic API                                        | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/id/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

Kecuali Anda menetapkan `ANTHROPIC_DEFAULT_FABLE_MODEL`, alias `fable` diselesaikan ke Fable 5.1, kecuali dalam sesi [Claude apps gateway](/docs/id/claude-apps-gateway), di mana `fable` dan `best` diselesaikan ke Fable 5. Sebelum v2.1.257, `fable` diselesaikan ke Fable 5 di setiap penyedia.

Gateway yang tidak dikonfigurasi untuk melayani `claude-fable-5-1` menolak permintaan untuk model itu. Untuk menggunakan Fable 5.1 melalui gateway yang melayaninya, pilih dengan `/model claude-fable-5-1`.

Di mana alias diselesaikan ke model yang lebih lama, model yang lebih baru tersedia dengan memilih nama model lengkap secara eksplisit atau menetapkan `ANTHROPIC_DEFAULT_OPUS_MODEL` atau `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Sebelum v2.1.280, `opus` diselesaikan ke Opus 5 di Anthropic API, Claude Platform on AWS, Amazon Bedrock, dan Google Cloud's Agent Platform dari v2.1.219. Sebelum v2.1.219, `opus` diselesaikan ke Opus 4.8 di Anthropic API dari v2.1.154, dan di Claude Platform on AWS, Amazon Bedrock, dan Google Cloud's Agent Platform dari v2.1.207. Sebelum v2.1.207, `opus` diselesaikan ke Opus 4.7 di Claude Platform on AWS dan ke Opus 4.6 di Amazon Bedrock dan Google Cloud's Agent Platform.

Alias menunjuk ke versi yang direkomendasikan untuk penyedia Anda dan diperbarui seiring waktu. Untuk menyematkan ke versi tertentu, gunakan nama model lengkap, misalnya `claude-opus-5-5`, atau atur variabel lingkungan yang sesuai seperti `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 memerlukan Claude Code v2.1.280 atau lebih baru. Opus 5 memerlukan v2.1.219 atau lebih baru. Sonnet 5 memerlukan v2.1.197 atau lebih baru. Jalankan `claude update` untuk upgrade.
</Note>

<h3 id="work-with-fable">
  Bekerja dengan Fable
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) dan Claude Fable 5 adalah model paling mampu di Claude Code, cocok untuk tugas-tugas yang lebih besar dari satu sesi. Mereka mempertahankan sesi otonomi yang panjang, menyelidiki sebelum bertindak, dan memverifikasi pekerjaan mereka lebih sering daripada model yang lebih kecil. Fable 5.1 adalah rilis yang lebih baru.

Tidak ada model Fable yang merupakan default tipe akun di paket atau penyedia apa pun. Pilih satu secara eksplisit:

* **Fable 5.1**: jalankan `/model fable`, atau luncurkan dengan `claude --model fable`. Dalam sesi [Claude apps gateway](/docs/id/claude-apps-gateway), di mana alias diselesaikan ke Fable 5, jalankan `/model claude-fable-5-1` sebagai gantinya.
* **Fable 5**: pilih berdasarkan ID model. Di Anthropic API, jalankan `/model claude-fable-5` atau luncurkan dengan `claude --model claude-fable-5`. Di penyedia lain, gunakan ID model Fable 5 penyedia Anda atau [sematkan](#pin-models-for-third-party-deployments) dengan `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Jika Anda terhubung ke Anthropic API secara langsung dan pengaturan pengguna Anda menyimpan `claude-fable-5` atau `claude-fable-5[1m]` sebagai model, misalnya karena Anda memilih Fable di pemilih `/model` sebelum v2.1.257, Claude Code mengubah nilai yang disimpan itu ke alias `fable` atau `fable[1m]` pertama kali Anda menjalankan v2.1.257 atau lebih baru. Baris model startup menunjukkan `(auto-updated)` sekali. Nilai `claude-fable-5` dalam pengaturan proyek, lokal, atau terkelola tetap apa adanya.

Permintaan yang pengklasifikasi keamanan model Fable tandai, paling sering di domain keamanan siber dan biologi, memicu [fallback model otomatis](#automatic-model-fallback).

Untuk mendapatkan hasil maksimal dari Fable:

* **Jelaskan hasilnya, bukan langkah-langkahnya**: berikan hasil yang Anda inginkan dan biarkan merencanakan jalurnya. Untuk membuatnya terus bekerja menuju hasil itu, [tetapkan tujuan](/docs/id/goal).
* **Berikan masalah yang ambigu**: investigasi akar penyebab, debugging pemadaman, dan keputusan arsitektur adalah tempat investigasi dan verifikasi tambahan membayar.
* **Lewati pengingat verifikasi**: ia memverifikasi pekerjaan sendiri dengan prompting yang lebih sedikit, jadi pengingat untuk menguji atau memeriksa biasanya tidak perlu.
* **Ukur tugas yang lebih besar**: berikan pekerjaan yang biasanya akan Anda pecah menjadi beberapa bagian. Ia mempertahankan sesi panjang tanpa kehilangan benang merah.

<Note>
  Fable 5.1 memerlukan Claude Code v2.1.257 atau lebih baru. Jika permintaan untuk itu dari versi yang lebih lama gagal, lihat [Claude Code tidak mendukung model ini](/docs/id/errors#claude-code-does-not-support-this-model). Jalankan `claude update` untuk upgrade. Untuk ketersediaan di bawah zero data retention, lihat [Ketersediaan model di bawah ZDR](/docs/id/zero-data-retention#model-availability-under-zdr).
</Note>

Di Anthropic API, model Fable muncul di pemilih `/model` kecuali [`availableModels`](#restrict-model-selection) atau [pembatasan model organisasi](#organization-model-restrictions) mengecualikannya. Ketika organisasi Anda tidak dapat menggunakan Fable sama sekali, misalnya di bawah [zero data retention](/docs/id/zero-data-retention#model-availability-under-zdr), baris tetap di pemilih berwarna abu-abu, dengan catatan tentang alasannya.

<h4 id="fable-and-usage-credits">
  Fable dan kredit penggunaan
</h4>

Tergantung pada paket dan tingkat kursi Anda, penggunaan Fable dapat ditagih ke [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) alih-alih mengambil dari batas yang disertakan paket Anda. Ketika itu terjadi, pemilih `/model` menunjukkan "Memerlukan kredit penggunaan" di baris Fable. Untuk mengelola kredit penggunaan, lihat [Tambahkan kredit penggunaan ke langganan Anda](/docs/id/costs#add-usage-credits-to-your-subscription).

Dalam sesi interaktif, Claude Code menampilkan prompt persetujuan sebelum permintaan Fable menagih kredit penggunaan. Anggota paket Enterprise dengan penagihan organisasi tidak melihat prompt. Anda dapat melanjutkan di Fable menggunakan kredit penggunaan atau beralih ke model default Anda. Anda juga dapat menutup prompt:

* Di pemilih `/model`, Anda menyimpan model saat ini.
* Pertengahan sesi, Claude Code melanjutkan giliran di model default Anda.

Setelah Anda memilih untuk melanjutkan di Fable menggunakan kredit penggunaan, Claude Code tidak menampilkan prompt lagi.

Dalam sesi dengan [Remote Control](/docs/id/remote-control) terhubung, [sesi latar belakang](/docs/id/agent-view), atau sesi rekan tim [agent team](/docs/id/agent-teams), mungkin tidak ada yang berada di terminal, jadi Claude Code menahan prompt persetujuan pertengahan sesi untuk batas waktu [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry), lima menit secara default. Jika tidak ada yang menjawab pada batas waktu, Claude Code mengakhiri giliran tanpa mengirim permintaan dan menambahkan pemberitahuan ke transkrip, yang juga ditampilkan klien Remote Control. Pilihan model Anda tidak berubah, dan Claude Code meminta persetujuan lagi pada pesan berikutnya Anda.

Apa yang dapat Anda lakukan saat prompt menunggu tergantung pada sesi:

* Dengan Remote Control terhubung atau dalam sesi rekan tim, tekan tombol apa pun di terminal untuk membatalkan batas waktu, dan Claude Code menunggu jawaban Anda.
* Dalam sesi latar belakang, jawab sebelum batas waktu.
* Jika Anda mengirim pesan baru dari klien jarak jauh sebelum ada yang mengetik di terminal, Claude Code mengakhiri giliran dengan cara yang sama, dan pesan baru Anda memulai giliran berikutnya. Setelah seseorang mengetik di terminal, Claude Code terus menunggu jawaban dan mengantrekan pesan baru Anda di belakangnya.

Dalam [mode non-interaktif](/docs/id/headless) dengan flag `-p` dan melalui Agent SDK, Claude Code tidak pernah menampilkan prompt persetujuan. Ketika permintaan Fable di sana akan ditagih ke kredit penggunaan, Claude Code menagihnya tanpa bertanya.

<h3 id="setting-your-model">
  Mengatur model Anda
</h3>

Anda dapat mengonfigurasi model Anda dengan beberapa cara, tercantum dalam urutan prioritas:

1. **Selama sesi**: gunakan `/model <alias|name>` untuk beralih segera, atau jalankan `/model` tanpa argumen untuk membuka pemilih. Lihat [ketika Claude Code meminta Anda untuk mengonfirmasi switch](/docs/id/prompt-caching#switching-models)
2. **Saat startup**: luncurkan dengan `claude --model <alias|name>`
3. **Variabel lingkungan**: atur `ANTHROPIC_MODEL=<alias|name>`
4. **Pengaturan**: konfigurasi secara permanen di file pengaturan Anda menggunakan bidang `model`
5. **[Default untuk sesi baru](#set-a-default-model-for-new-sessions)**: atur `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` menyimpan pilihan Anda sebagai default untuk sesi baru dengan menulis bidang `model` di pengaturan pengguna Anda. Di pemilih:

* `Enter`: alihkan model dan simpan sebagai default Anda
* `s`: alihkan model hanya untuk sesi ini dan biarkan default Anda tidak berubah. Untuk menggunakan kunci yang berbeda, ikat ulang [`modelPicker:thisSessionOnly`](/docs/id/keybindings#model-picker-actions)

Mengetik `/model <name>` langsung berperilaku seperti `Enter`. Untuk beralih hanya untuk sesi ini, buka pemilih dengan `/model` dan tekan `s` di baris model.

Jika Anda beralih model dengan `/model`, switch juga mencapai [subagent yang mewarisi model percakapan utama](/docs/id/sub-agents#choose-a-model), karena Claude Code menyelesaikan model mereka dari yang digunakan sesi Anda ketika Claude memulainya. Beralih ke Opus sebelum Claude mendelegasikan penelitian atau test run ke salah satunya, dan pekerjaan itu berjalan di Opus juga. Untuk menjaga subagent kustom di model yang lebih kecil, atur `model` dalam definisinya.

Jika Anda menetapkan model dengan `/model` dalam [mode non-interaktif](/docs/id/headless), dengan flag `-p`, pilihan Anda berlaku hanya untuk sesi saat ini dan tidak disimpan sebagai default Anda; `/model` dalam mode itu memerlukan Claude Code v2.1.205 atau lebih baru. Pengaturan proyek dan terkelola masih memiliki prioritas dan diterapkan kembali pada peluncuran berikutnya. [Model default organisasi](#organization-default-model) yang admin Anda telah dikonfigurasi untuk mengganti pilihan pengguna juga diterapkan kembali pada peluncuran berikutnya.

Dalam v2.1.144 hingga v2.1.152, `/model` berlaku hanya untuk sesi saat ini dan `d` di pemilih menyimpan default.

Flag `--model` dan variabel lingkungan `ANTHROPIC_MODEL` hanya berlaku untuk sesi yang Anda luncurkan dengannya. Untuk menjalankan model berbeda di terminal berbeda pada waktu yang sama, luncurkan masing-masing dengan flag `--model` sendiri daripada beralih dengan `/model`.

Harga di pemilih `/model` muncul ketika Claude Code berbicara dengan Anthropic API, secara langsung atau melalui [gateway LLM](/docs/id/llm-gateway) yang memproksikannya, dan harga di baris adalah harga model yang dipilih baris itu. Di [penyedia pihak ketiga](/docs/id/third-party-integrations) seperti Amazon Bedrock dan di [gateway aplikasi Claude](/docs/id/claude-apps-gateway), penyedia atau gateway Anda menentukan apa yang Anda bayar, jadi baris pemilih tidak menampilkan harga. Harga adalah label tampilan saja; itu tidak mempengaruhi model mana yang dipilih baris atau apa yang ditagih penyedia Anda. Sebelum v2.1.206, [Claude Platform on AWS](/docs/id/claude-platform-on-aws) dan sesi gateway menampilkan harga daftar Anthropic, dan baris dapat menampilkan harga model yang berbeda dari yang dipilihnya.

Sesi yang dilanjutkan dimulai dengan `claude --resume`, `--continue`, atau pemilih `/resume` menyimpan model yang mereka gunakan ketika transkrip disimpan, terlepas dari pengaturan `model` saat ini. Jika model yang dipulihkan telah pensiun atau dikecualikan oleh [`availableModels`](#restrict-model-selection), sesi jatuh melalui urutan prioritas normal. Ini mencegah pilihan `/model` sesi lain mengubah model pada resume. Di penyedia yang menggunakan ID deployment khusus penyedia daripada ID model Anthropic, seperti Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, model transkrip tidak dipulihkan sama sekali dan sesi menyelesaikan modelnya melalui urutan prioritas normal.

Model yang Anda pilih untuk peluncuran baru dengan `--model` atau `ANTHROPIC_MODEL` masih memiliki prioritas atas model yang dipulihkan. Sejak v2.1.195, demikian juga variabel keluarga [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables). [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) juga bisa, di bawah kondisi yang tercantum di bagiannya.

Ketika model aktif saat startup berasal dari pengaturan proyek atau terkelola daripada pilihan Anda sendiri, header startup menunjukkan file pengaturan mana yang menetapkannya. Jalankan `/model` untuk mengganti; pengaturan proyek atau terkelola diterapkan kembali pada peluncuran berikutnya. Di platform yang menyematkan Claude Code dan menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars), konfigurasi model host memiliki prioritas atas pengaturan model terkelola, sementara daftar allowlist `availableModels` terkelola tetap berlaku kecuali host menyediakan miliknya sendiri; [Pengecualian untuk prioritas pengaturan terkelola](/docs/id/settings#exceptions-to-managed-settings-precedence) mengatakan kunci dan variabel mana yang diganti host.

Jika Anda atau organisasi Anda mengonfigurasi [PreModelSwitch hooks](/docs/id/hooks#premodelswitch), mereka berjalan sebelum switch yang diminta diterapkan dan dapat memblokir atau meminta Anda untuk mengonfirmasi.

Ketika Claude Code tidak dapat mengetahui hook PreModelSwitch mana yang dikirimkan [plugin terkelola](/docs/id/settings-reference#enabledplugins) organisasi Anda, misalnya karena plugin terkelola gagal dimuat, itu menolak switch daripada menerapkannya tanpa diperiksa, dan itu memeriksa lagi pada setiap upaya baru. Lihat [Model switch diblokir oleh hook PreModelSwitch](/docs/id/errors#model-switch-was-blocked-by-a-premodelswitch-hook) untuk pesan dan pemulihan.

Ketika Anda beralih model melalui metode [`setModel()`](/docs/id/agent-sdk/overview) Agent SDK atau dari perangkat yang terhubung melalui [Remote Control](/docs/id/remote-control), atau aplikasi seperti [Desktop app](/docs/id/desktop) yang menjalankan Claude Code CLI beralih untuk Anda, Claude Code memeriksa bahwa string adalah yang dikenalinya sebelum menyimpannya. Pemeriksaan ini memerlukan Claude Code v2.1.200 atau lebih baru. Memeriksa pilihan Remote Control memerlukan Claude Code v2.1.260 atau lebih baru di mesin Anda. Di Anthropic API, Claude Code mengenali:

* alias model
* entri dari pemilih `/model`
* nama apa pun yang dimulai dengan `claude-`
* nilai yang Anda konfigurasi sendiri sebagai [opsi model kustom](#add-a-custom-model-option) atau dalam [`modelOverrides`](#override-model-ids-per-version)

Claude Code menolak string yang tidak dikenali dengan `Model "<name>" is not a recognized model id.` dan sesi menyimpan model saat ini, alih-alih menyimpan string dan gagal pada permintaan berikutnya. Lihat [referensi kesalahan](/docs/id/errors#model-is-not-a-recognized-model-id) untuk langkah pemulihan.

Pemeriksaan hanya berjalan di Anthropic API. Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), dan di balik [gateway LLM](/docs/id/llm-gateway) atau `ANTHROPIC_BASE_URL` kustom, penyedia atau gateway Anda mendefinisikan nama model, jadi Claude Code melewatkan string apa pun tanpa memeriksanya. Pemeriksaan juga tidak mencakup flag `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau pengaturan `model`; nilai yang salah ketik di sana menghasilkan [Ada masalah dengan model yang dipilih](/docs/id/errors#theres-an-issue-with-the-selected-model) pada permintaan pertama. Claude Code masih dapat menulis [baris diagnostik model yang tidak dikenali](/docs/id/errors#unrecognized-model-id-on-a-request) pada waktu permintaan, di setiap penyedia.

Ketika model yang diminta memiliki tanggal pensiun terjadwal atau secara otomatis dipetakan ulang ke versi yang lebih baru, Claude Code menampilkan peringatan yang menyebutkan model yang diminta. Sesi interaktif menampilkannya sebagai pemberitahuan startup. Dari v2.1.182, peringatan yang sama ditulis ke stderr dalam [mode non-interaktif](/docs/id/headless) ketika menggunakan format output teks default. Pemeriksaan juga mencakup `model` yang ditetapkan dalam [frontmatter subagent](/docs/id/sub-agents). Peringatan stderr ditekan untuk `--output-format json` dan `stream-json`; baca model aktual dari bidang `modelUsage` dari [pesan hasil](/docs/id/headless#get-structured-output) sebagai gantinya.

Misalnya, mulai sesi di Opus:

```bash theme={null}
claude --model opus
```

Kemudian alihkan model dari dalam sesi:

```text theme={null}
/model sonnet
```

File pengaturan contoh:

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Tetapkan model default untuk sesi baru
</h4>

Atur `ANTHROPIC_DEFAULT_MODEL=<alias|name>` untuk memilih model yang dimulai sesi Anda secara default. Memerlukan Claude Code v2.1.236 atau lebih baru.

Claude Code memulai sesi baru di model variabel hanya ketika tidak ada yang memilih model:

* Flag `--model`
* `ANTHROPIC_MODEL`
* Nilai `model` dalam file pengaturan apa pun, termasuk pilihan yang Anda simpan dengan `/model`
* [Model default organisasi](#organization-default-model)

Pilihan yang Anda simpan dengan `/model` memiliki prioritas atas variabel pada peluncuran berikutnya juga. Dengan `ANTHROPIC_MODEL` yang ditetapkan sebagai gantinya, Claude Code kembali ke model variabel itu pada peluncuran berikutnya, apa pun yang Anda simpan dengan `/model`.

Claude Code juga menyelesaikan opsi Default ke model variabel, kecuali model default organisasi berlaku. Ketika opsi Default diselesaikan ke model variabel, baris Default di pemilih `/model` menampilkan label Set by ANTHROPIC\_DEFAULT\_MODEL.

Claude Code mengabaikan variabel dalam kasus-kasus ini, dan opsi Default diselesaikan seolah-olah Anda belum menetapkannya:

* Anda menetapkannya ke `default`, `inherit`, `opusplan`, atau `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) aktif
* [`availableModels`](#restrict-model-selection) atau [pembatasan model organisasi](#organization-model-restrictions) mengecualikan model
* Model tidak tersedia untuk akun Anda

Ketika sesi baru akan dimulai di model variabel, sesi yang Anda lanjutkan dengan `claude --resume`, `--continue`, atau pemilih `/resume` dimulai di sana juga. Claude Code tidak memulihkan model yang disimpan dalam transkrip sesi itu. Jika tidak, Claude Code tidak menggunakan variabel ketika Anda [melanjutkan sesi](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Sesi baru dimulai di model yang berbeda dari yang Anda pilih
</h4>

Ketika Anda memilih model dengan `/model` dan sesi berikutnya Anda dimulai di sesuatu yang lain, ini adalah penyebab biasanya:

* **Anda memilihnya untuk satu sesi.** Menekan `s` di pemilih, meluncurkan dengan `--model`, dan menjalankan `/model` dalam mode non-interaktif semuanya berlaku untuk sesi saat ini dan meninggalkan default yang disimpan sendirian.
* **Sesuatu dengan prioritas lebih tinggi menetapkan model.** Nilai `model` dalam pengaturan proyek atau terkelola, `ANTHROPIC_MODEL` di shell Anda, atau [default organisasi](#organization-default-model) yang admin Anda tetapkan untuk mengganti pilihan pengguna berlaku lagi di setiap peluncuran. Pilihan `/model` Anda masih disimpan; itu dilampaui. Ketika pengaturan proyek atau terkelola menetapkan model, header startup menyebutkan file.
* **Claude Code tidak dapat menyimpan pilihan Anda.** `/model` menulis `model` ke `~/.claude/settings.json`. Jika Anda tidak dapat menulis ke file itu, misalnya karena alat lain menghasilkannya atau menautkannya ke salinan baca-saja, model yang Anda pilih berlangsung untuk sesi dan peluncuran berikutnya membaca nilai lama. Atur `model` dalam alat yang menghasilkan file, atau buat file dapat ditulis. Lihat [Perubahan yang Anda buat di Claude Code hilang di sesi baru](/docs/id/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Anda melanjutkan sesi.** Sesi yang Anda lanjutkan dengan `claude --resume` atau `--continue` biasanya [menyimpan model yang digunakan](#setting-your-model) daripada default saat ini Anda.

<h2 id="restrict-model-selection">
  Batasi pemilihan model
</h2>

Administrator enterprise dapat menggunakan `availableModels` dalam [pengaturan terkelola atau kebijakan](/docs/id/managed-settings) untuk membatasi model mana yang dapat dipilih pengguna. Entri cocok dengan keluarga model seperti `sonnet`, awalan versi seperti `claude-sonnet-4-5`, atau ID model lengkap seperti `claude-sonnet-4-5-20250929`. Awalan versi juga cocok dengan ID model yang lebih baru yang memperpanjangnya dengan segmen lain, jadi `claude-fable-5` memungkinkan Fable 5 dan Fable 5.1, sementara `claude-fable-5-1` memungkinkan Fable 5.1 saja.

Pada platform yang menyematkan Claude Code dan menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars), konfigurasi model host mengambil alih pengaturan model terkelola, sementara daftar izin `availableModels` terkelola tetap berlaku kecuali host menyediakan miliknya sendiri; [Pengecualian untuk prioritas pengaturan terkelola](/docs/id/settings#exceptions-to-managed-settings-precedence) mengatakan kunci dan variabel mana yang ditimpa host.

Ketika `availableModels` diatur, daftar izin berlaku di mana pun pengguna dapat menentukan model:

* **Model sesi utama**: `/model`, bendera `--model`, variabel lingkungan `ANTHROPIC_MODEL`, pengaturan `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), dan model yang dipulihkan saat [melanjutkan sesi](#setting-your-model)
* **Resolusi alias**: variabel lingkungan `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, dan `ANTHROPIC_DEFAULT_FABLE_MODEL` tidak dapat mengarahkan ulang alias yang diizinkan ke model di luar daftar
* **Mode cepat**: `/fast` menolak untuk beralih ketika itu akan secara implisit beralih ke model Opus di luar daftar, dengan pesan "is not in your organization's allowed models"
* **Model subagent dan rekan kerja**: bidang `model` dalam frontmatter [subagent](/docs/id/sub-agents#choose-a-model), parameter `model` alat Agent, model rekan kerja [tim agent](/docs/id/agent-teams#specify-teammates-and-models), `CLAUDE_CODE_SUBAGENT_MODEL`, dan, pada v2.1.197 dan lebih awal, pemilih model dalam wizard `/agents`&#x20;
* **Model skill dan perintah**: frontmatter `model` dalam [skills dan commands](/docs/id/skills)
* **Model advisor**: pengaturan [`advisorModel`](/docs/id/advisor) yang dikonfigurasi dan bendera `--advisor`
* **Model agent latar belakang**: model yang dipilih dalam [pemilih dispatch](/docs/id/agent-view)

Pada Anthropic API dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws), alias keluarga model, `opus`, `sonnet`, `haiku`, atau `fable`, diselesaikan ke model biasanya ketika daftar izin memungkinkan model itu. Ketika daftar izin memblokir model itu, Claude Code mengganti versi terbaru keluarga yang daftar izin izinkan dan menampilkan pemberitahuan yang menyebutkan model yang diminta dan diganti. Dengan `["sonnet", "claude-opus-4-6"]`, misalnya, baik `/model opus` maupun `--model opus` memilih Claude Opus 4.6, Opus terbaru yang diizinkan. Sebelum v2.1.205, alias yang versi rilis terbarunya berada di luar daftar ditolak atau diganti seperti pemilihan terblokir lainnya, bahkan ketika daftar memungkinkan versi yang lebih lama.

Substitusi membutuhkan versi yang diizinkan untuk mendarat: ketika daftar izin tidak memungkinkan versi apa pun dari keluarga alias, alias mengikuti perilaku penolakan dan penggantian di bawah seperti nilai terblokir lainnya.

Claude Code menangani pemilihan terblokir lainnya sesuai dengan tempat model diatur:

* **`/model`**: Claude Code menolak switch dengan kesalahan
* **Bendera `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`**: Claude Code mengganti nilai saat startup dengan peringatan yang menyebutkan model yang diminta dan diganti, dan sesi dimulai pada model default
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**: Claude Code mengabaikan variabel
* **Penggantian subagent atau rekan kerja**: Claude Code menjalankan subagent atau rekan kerja pada model fallback daripada gagal permintaan. Lihat [Pilih model](/docs/id/sub-agents#choose-a-model) untuk fallback subagent dan [Tentukan rekan kerja dan model](/docs/id/agent-teams#specify-teammates-and-models) untuk fallback rekan kerja.

  Dalam sesi interaktif, Claude Code memperingatkan Anda ketika mengganti model subagent, dengan fallback ini atau dengan substitusi versi-terbaru-yang-diizinkan di atas, menyebutkan model yang diminta dan diganti; itu tidak melaporkan fallback rekan kerja.

  Di mana substitusi versi-terbaru-yang-diizinkan di atas beroperasi, alias keluarga terblokir mengikutinya. Sebelum v2.1.222, alias jatuh kembali seperti nilai terblokir lainnya di setiap penyedia
* **Penggantian skill atau perintah**: Claude Code mengabaikan penggantian, termasuk alias keluarga terblokir, dan skill atau perintah berjalan pada model sesi. Skill atau perintah yang [berjalan dalam subagent](/docs/id/skills#run-skills-in-a-subagent) mengikuti perilaku subagent di atas
* **Pengaturan `advisorModel`**: advisor dinonaktifkan untuk sesi
* **Bendera `--advisor`**: Claude Code keluar dengan kesalahan saat peluncuran. Dalam [sesi latar belakang](/docs/id/agent-view), itu memulai sesi tanpa advisor daripada keluar

Claude Code menyembunyikan model yang dikecualikan dari pemilih `/model`. ID model lengkap dalam daftar yang tidak memiliki baris pemilih bawaan, seperti versi yang lebih lama yang daftar pin, muncul dalam pemilih `/model` sebagai baris berlabel miliknya sendiri, kecuali Claude Code mengganti opsi bawaan dengan lineup [`modelPicker`](/docs/id/settings-reference#modelpicker). Sebelum v2.1.199, ID tersebut hanya dapat dipilih dengan mengetik `/model <id>`.

Perubahan model yang Claude Code buat atas nama Anda diperiksa dengan cara yang sama:

* **[Rantai model fallback](#fallback-model-chains)**: entri di luar daftar izin dijatuhkan
* **Upgrade mode plan**: pada Anthropic API dan Claude Platform on AWS, upgrade seperti [`opusplan`](#opusplan-model-setting) ke model yang dikecualikan menggunakan versi terbaru keluarga upgrade yang diizinkan. Pada penyedia dengan ID model khusus penyedia, dan ketika tidak ada versi yang diizinkan, upgrade dilewati dan perencanaan berlanjut pada model sesi
* **[Fallback model otomatis](#automatic-model-fallback)**: fallback yang targetnya dikecualikan tidak berjalan, jadi permintaan yang ditandai berakhir dengan penolakan
* **[Pengklasifikasi mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode)**: default Claude Sonnet 5 pengklasifikasi hanya berlaku ketika daftar izin memungkinkan Sonnet 5. Ketika dikecualikan, pengklasifikasi berjalan pada model sesi, yang daftar izin sudah mengatur, atau pada model Opus ketika sesi berjalan pada [model Fable](#work-with-fable). Pada penyedia selain Anthropic API, fallback Opus itu berjalan pada model Opus default penyedia tanpa berkonsultasi dengan daftar izin. Memerlukan Claude Code v2.1.210 atau lebih baru
* **[Mode cepat](/docs/id/fast-mode)**: mengaktifkan mode cepat ditolak ketika model yang sesi jalankan setelahnya berada di luar daftar izin

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Cakupan permukaan
</h3>

Setiap permukaan memberlakukan daftar izin yang diterimanya. Mekanisme pengiriman mana yang mencapai setiap permukaan berbeda:

| Mekanisme pengiriman                                                           | CLI dan IDE  | Sesi lokal desktop | Sesi web, mobile, dan cloud                                                                                                                                                                                                                                                           | Agent SDK dan non-interaktif | Cowork                             |
| :----------------------------------------------------------------------------- | :----------- | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------- | :--------------------------------- |
| [Pengaturan terkelola server](/docs/id/server-managed-settings) dari konsol admin   | Diberlakukan | Diberlakukan       | Diberlakukan                                                                                                                                                                                                                                                                          | Diberlakukan                 | Tidak dikirim                      |
| [File MDM atau pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms) | Diberlakukan | Diberlakukan       | Tidak dikirim di lingkungan yang dihosting Anthropic; di [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments), diberlakukan dari gambar runner per [bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) | Diberlakukan                 | Diberlakukan di mana pun digunakan |

* Sesi cloud, di [Claude Code di web](/docs/id/claude-code-on-the-web) atau di aplikasi Desktop, berjalan pada VM yang dikelola Anthropic secara default: pengaturan yang digunakan pada perangkat Anda tidak mencapainya, jadi kirimkan daftar izin melalui pengaturan terkelola server. Sesi yang organisasi Anda arahkan ke [lingkungan yang dihosting sendiri](/docs/id/self-hosted-environments) berjalan pada komputasi Anda sendiri dan juga membaca file pengaturan terkelola dalam gambar runner. [Bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) mengatakan kapan file itu berlaku. Perubahan model pertengahan sesi dalam sesi cloud ditolak ketika model yang diminta dikecualikan oleh daftar izin. Ketika daftar `availableModels` dalam pengaturan terkelola server Anda tidak kosong, server menolak permintaan pengguna untuk memulai sesi cloud pada model yang daftar kecualikan.
* Cowork, tab pekerjaan agentic di aplikasi Claude Desktop, menjalankan sesinya di Claude Code tetapi, dengan desain, tidak menerima pengaturan terkelola server dari konsol admin claude.ai. File pengaturan terkelola berlaku untuk sesi Cowork ketika ada di mana sesi berjalan; sesi Cowork jarak jauh berjalan pada VM yang dikelola Anthropic, di mana file yang digunakan perangkat tidak ada.
* Sesi pada [penyedia pihak ketiga](/docs/id/server-managed-settings#platform-availability) seperti Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws) tidak menerima pengaturan terkelola server, jadi kirimkan daftar izin melalui file MDM atau pengaturan terkelola di sana.
* Pengiriman terkelola server juga memerlukan sesi untuk mengautentikasi dengan [login atau kunci yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability). Armada yang menghasilkan kunci hanya melalui skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) harus mengirimkan daftar izin melalui file MDM atau pengaturan terkelola.
* Tab Desktop Code juga menghosting [sesi SSH](/docs/id/desktop#ssh-sessions), yang membaca file pengaturan terkelola dari host jarak jauh tempat mereka berjalan. Lihat [Pengaturan terkelola Desktop](/docs/id/desktop#managed-settings).
* Pemilih model di claude.ai dan di aplikasi Desktop menyembunyikan atau memudarkan model yang dikecualikan oleh daftar izin organisasi Anda. Status pemilih adalah kenyamanan bagi pengguna; penegakan terjadi dalam sesi.

<h3 id="default-model-behavior">
  Perilaku model default
</h3>

Dengan sendirinya, `availableModels` meninggalkan opsi Default pada [default runtime](#default-model-setting) sistem untuk akun sampai Anda juga menetapkan [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model). Jika default itu adalah model yang ingin Anda batasi, atur `enforceAvailableModels` juga.

Array `availableModels` kosong tidak pernah melibatkan penegakan model Default: dengan `availableModels: []`, pemilihan model bernama diblokir tetapi model Default untuk tipe akun tetap dapat digunakan terlepas dari `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Berlakukan daftar izin untuk model Default
</h3>

Atur `enforceAvailableModels: true` bersama dengan `availableModels` yang tidak kosong dalam pengaturan terkelola untuk memperluas daftar izin ke opsi Default. Ini memerlukan Claude Code v2.1.175 atau lebih baru.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Opsi Default diselesaikan ke default tipe akun, atau ke [model default organisasi](#organization-default-model) ketika admin telah menetapkan satu. Ketika model itu tidak dalam daftar izin, opsi Default malah diselesaikan ke entri `availableModels` pertama yang menyebutkan model yang diizinkan dan tersedia, dan baris Default pemilih `/model` menunjukkan model itu. Ini berlaku di mana pun default dicapai: startup sesi, memilih Default di `/model`, kata kunci `"default"` dalam [rantai model fallback](#fallback-model-chains), dan fallback yang digunakan ketika pemilihan yang dikecualikan dijatuhkan.

`enforceAvailableModels` memetakan ulang opsi Default hanya ketika `availableModels` tidak kosong. Dengan `availableModels: []`, model Default untuk tipe akun tetap dapat digunakan, jadi pengaturan tidak dapat mengunci pengguna dari setiap model. Ketika `availableModels` tidak kosong tetapi tidak ada entri yang diselesaikan ke model yang diizinkan dan tersedia, penegakan dilewati dan Default diselesaikan ke default tipe akun, dengan peringatan yang terlihat hanya di bawah `--debug`. Pertahankan setidaknya satu entri yang dijamin tersedia dalam daftar untuk menghindari ini.

Gunakan kedua kunci bersama dalam sumber terkelola dengan peringkat tertinggi yang Anda kirimkan. Secara default Claude Code hanya membaca sumber itu, jadi pasangan yang ditempatkan dalam file pengaturan terkelola diabaikan ketika konsol admin mengirimkan pengaturan apa pun; di bawah penggabungan opt-in dalam [bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources), Claude Code masih mengabaikan peta `modelOverrides` dari sumber yang diperingkat di bawah yang menetapkan `availableModels`.

<h3 id="control-the-model-users-run-on">
  Kontrol model yang dijalankan pengguna
</h3>

Pengaturan `model` adalah pemilihan awal, bukan penegakan. Ini menetapkan model mana yang aktif ketika sesi dimulai, tetapi pengguna masih dapat membuka `/model` dan memilih Default, yang diselesaikan ke [default runtime](#default-model-setting) sistem terlepas dari apa yang `model` diatur, kecuali [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) mengarahkan ulangnya.

Untuk sepenuhnya mengontrol pengalaman model, gabungkan pengaturan ini:

* **`availableModels`**: membatasi model bernama mana yang dapat dialihkan pengguna
* **`enforceAvailableModels`**: memperluas daftar izin `availableModels` ke opsi Default, jadi Default tidak dapat diselesaikan ke model di luar daftar
* **`model`**: menetapkan pemilihan model awal ketika sesi dimulai
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: mengontrol apa yang alias `sonnet`, `opus`, `haiku`, dan `fable` diselesaikan, dan versi mana yang [default tipe akun](#default-model-setting) gunakan

Contoh ini memulai pengguna di Sonnet 4.5, membatasi pemilih ke Sonnet dan Haiku, dan memastikan Default diselesaikan ke model dalam daftar izin daripada default tier:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Tanpa `enforceAvailableModels` atau blok `env`, pengguna yang memilih Default dalam pemilih mendapatkan [default runtime](#default-model-setting) daripada versi yang disematkan dalam `model`. Dua pengaturan mencakup cakupan berbeda: `enforceAvailableModels` membuat Default mematuhi daftar izin, sementara blok `env` menyematkan versi mana alias yang diizinkan seperti `sonnet` diselesaikan. Gunakan `enforceAvailableModels` saja ketika membatasi keluarga model cukup; tambahkan blok `env` ketika Anda juga perlu menyematkan versi tertentu.

<h3 id="merge-behavior">
  Perilaku penggabungan
</h3>

Ketika pengaturan terkelola yang Claude Code terapkan mendefinisikan `availableModels`, daftar itu saja berlaku, terlepas dari [platform host yang menyediakan miliknya sendiri](/docs/id/settings#exceptions-to-managed-settings-precedence): entri dalam pengaturan pengguna, proyek, atau lokal tidak dapat memperpanjangnya, dan Claude Code tidak pernah menggabungkan `availableModels` di seluruh sumber terkelola; [bagaimana Claude Code menggabungkan sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) mengatakan daftar sumber mana yang berlaku. Jika tidak, daftar dari pengaturan pengguna, proyek, dan lokal [digabungkan dan dideduplikasi](/docs/id/settings#settings-precedence) seperti pengaturan array lainnya. Sebelum Claude Code v2.1.175, entri dari cakupan prioritas lebih rendah digabungkan ke dalam daftar terkelola daripada diganti olehnya.

Dalam daftar efektif, entri yang menyebutkan model tertentu dalam keluarga, baik awalan versi atau ID model lengkap, menonaktifkan entri wildcard keluarga itu: `["sonnet", "claude-sonnet-4-5"]` memungkinkan hanya versi Sonnet 4.5, bukan setiap model Sonnet.

<h3 id="mantle-model-ids">
  ID model Mantle
</h3>

Ketika [titik akhir Amazon Bedrock Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) diaktifkan, entri dalam `availableModels` yang dimulai dengan `anthropic.` ditambahkan ke pemilih `/model` sebagai opsi khusus dan dialihkan ke titik akhir Mantle. Ini adalah pengecualian untuk pencocokan alias yang dijelaskan dalam [Sematkan model untuk penyebaran pihak ketiga](#pin-models-for-third-party-deployments). Pengaturan masih membatasi pemilih ke entri yang terdaftar, dan ID Mantle menyematkan nama keluarga, jadi itu dihitung sebagai entri tertentu dan menonaktifkan wildcard keluarga itu: bersama ID Mantle apa pun, daftarkan awalan versi atau ID lengkap yang ingin Anda pertahankan dapat dipilih. Lihat [Perilaku penggabungan](#merge-behavior).

<h3 id="organization-model-restrictions">
  Pembatasan model organisasi
</h3>

Admin organisasi pada rencana Claude Enterprise membatasi model mana yang dapat dijalankan anggota dengan menonaktifkan model individual dalam konsol admin claude.ai. Pembatasan ini dikirimkan dengan hak akses akun ketika Claude Code mengautentikasi, terpisah dari daftar `availableModels` apa pun dalam pengaturan, dan server memberlakukan pembatasan yang sama secara independen ketika sesi dibuat. Memerlukan Claude Code v2.1.187 atau lebih baru.

Pembatasan berlaku ketika anggota masuk atau menggunakan kunci API mereka sendiri. Kredensial yang dicakup organisasi, seperti kunci layanan organisasi, tidak terikat pada pengguna, jadi pembatasan tidak berlaku untuk mereka.

Claude Console tidak memiliki kontrol pembatasan model. Organisasi tanpa rencana Claude Enterprise, termasuk yang anggotanya mengautentikasi melalui Anthropic API, membatasi model dengan [`availableModels`](#restrict-model-selection) dalam [pengaturan terkelola](/docs/id/managed-settings), menambahkan [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) untuk mencakup opsi Default. [Cakupan permukaan](#surface-coverage) mengatakan bagaimana setiap permukaan menerima dan memberlakukan pengaturan ini.

Model yang dibatasi disembunyikan dari pemilih `/model`. Memilihnya berdasarkan nama dengan `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau pengaturan `model` menunjukkan pemberitahuan `Model "<name>" is restricted by your organization's settings. Using <model> instead.` dan sesi dimulai pada model yang diizinkan. Mengetik `/model <name>` untuk model yang dibatasi ditolak dengan `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` dan sesi mempertahankan model saat ini.

[Alias keluarga model](#restrict-model-selection) seperti `opus` diselesaikan ke model biasanya ketika organisasi memungkinkannya. Ketika organisasi membatasi model itu, Claude Code mengganti versi terbaru keluarga yang organisasi izinkan, dengan pemberitahuan substitusi yang sama. `/model <alias>` ditolak hanya ketika setiap versi keluarganya dibatasi; alias yang diatur dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model` masih diganti saat startup dalam hal itu. Sebelum v2.1.205, alias keluarga diganti atau ditolak berdasarkan versi rilis terbarunya saja, bahkan ketika versi yang lebih lama diizinkan.

Pembatasan berlaku di seluruh organisasi atau per peran:

* Menonaktifkan model di tingkat organisasi menghapusnya untuk setiap anggota.
* Akses tingkat peran memberikan model berbeda ke peran khusus yang berbeda, dan anggota yang memiliki beberapa peran dapat menggunakan model apa pun yang salah satu peran mereka berikan.
* Model Haiku selalu tersedia dan tidak dapat dinonaktifkan, jadi setiap anggota mempertahankan setidaknya satu model yang dapat digunakan.
* Perubahan akses berlaku di seluruh organisasi dalam waktu sekitar satu menit; pemilih `/model` mencerminkannya saat sesi berikutnya dimulai.

Kedua pembatasan berlaku bersama: model dapat dipilih hanya ketika diizinkan oleh `availableModels` dan tidak dibatasi oleh organisasi. Pembatasan organisasi mencapai sesi pada Anthropic API dan penyebaran [gateway LLM](/docs/id/llm-gateway) saja; pada penyedia lain apa pun, gunakan `availableModels`.

<h2 id="organization-default-model">
  Model default organisasi
</h2>

Admin organisasi pada paket Claude Enterprise dapat menetapkan model default untuk anggota Claude Code dari konsol admin claude.ai, untuk seluruh organisasi atau per peran kustom. Ketika satu ditetapkan, opsi Default akan diselesaikan ke model tersebut. Memerlukan Claude Code v2.1.196 atau lebih baru.

Baris Default dalam pemilih `/model` menampilkan nama default organisasi dengan label Org default. Label membaca Org default apakah admin menetapkan default untuk seluruh organisasi atau untuk peran Anda. Default peran mencakup anggota peran kustom tersebut dan mengambil alih dari default organisasi-lebar; ketika beberapa peran Anda menetapkan default yang berbeda, model yang paling mampu diterapkan.

Default organisasi adalah titik awal, bukan pembatasan. Pilihan-pilihan ini mengambil alih darinya:

* bendera `--model` dan variabel lingkungan `ANTHROPIC_MODEL`
* nilai `model` dalam [pengaturan terkelola](/docs/id/managed-settings) atau disediakan melalui `--settings`
* nilai `model` dalam pengaturan pengguna, proyek, atau lokal Anda, termasuk model yang Anda simpan dengan `/model`

Admin juga dapat mengonfigurasi default organisasi untuk mengganti pilihan pengguna. Dengan override aktif, ini mengambil alih dari nilai `model` dalam pengaturan pengguna, proyek, dan lokal, sehingga model yang Anda simpan dengan `/model` berlaku untuk sesi saat ini dan default organisasi kembali pada peluncuran berikutnya. Ketika pilihan Anda berbeda, `/model` menampilkan `Your organization's default (<model>) applies on restart`. Bendera `--model`, `ANTHROPIC_MODEL`, pengaturan terkelola, dan `--settings` masih mengambil alih bahkan dengan override aktif. Override tersedia untuk serangkaian organisasi terbatas; tanyakan kepada tim akun Anthropic Anda tentang ketersediaan.

Untuk membatasi model mana yang dapat dipilih anggota, gunakan [pembatasan model organisasi](#organization-model-restrictions) atau [`availableModels`](#restrict-model-selection) sebagai gantinya.

Claude Code membaca default organisasi sekali saat startup, jadi default yang diubah admin di tengah-sesi berlaku pada peluncuran berikutnya.

Ketika default organisasi tidak mengganti pilihan pengguna, peluncuran interaktif pertama setelah admin mengubahnya menghapus kunci `model` dari pengaturan pengguna Anda sekali, sehingga default baru berlaku. Ini tidak mengubah apa pun di file, dan model yang Anda simpan dengan `/model` setelah peluncuran itu disimpan.

Default organisasi melewati pemeriksaan pembatasan ini sebelum diadopsi:

* [`availableModels`](#restrict-model-selection) sendiri tidak berlaku untuk default organisasi, jadi default organisasi di luar daftar izin masih berlaku. Ketika [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) juga diatur, default organisasi di luar daftar izin dipetakan ulang ke entri daftar izin pertama, seperti Default lainnya
* default organisasi yang [pembatasan model organisasi](#organization-model-restrictions) tolak untuk akun Anda diganti dengan model terbaru yang diizinkan dalam keluarganya, atau keluarga biaya lebih rendah ketika setiap versinya dibatasi
* default organisasi yang tidak tersedia untuk akun Anda sama sekali dilewati, dan opsi Default diselesaikan seperti yang akan terjadi [tanpa default organisasi](#default-model-setting)

Mulai dari v2.1.199, ketika default organisasi adalah keluarga model yang berbeda dari default biasa tipe akun Anda, pemilih `/model` menyimpan baris terpisah untuk keluarga biasa itu, sehingga Anda masih dapat beralih ke itu untuk sesi. Dalam v2.1.196 hingga v2.1.198 baris itu hilang dari pemilih.

Default organisasi hanya mencapai sesi yang diautentikasi dengan API Anthropic. Untuk menetapkan default di tempat lain, termasuk penyebaran [LLM gateway](/docs/id/llm-gateway), gunakan kunci `model` dalam [pengaturan terkelola](/docs/id/managed-settings) sebagai gantinya.

<h2 id="organization-effort-limits">
  Batas upaya organisasi
</h2>

Organisasi Anda dapat membatasi [tingkat upaya](#adjust-effort-level) dengan dua cara. Pada paket Claude Enterprise, admin organisasi menetapkan batas upaya per peran, dijelaskan di bawah. Pada paket apa pun dan penyedia apa pun, termasuk Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, pengaturan terkelola [`maxEffortLevel`](/docs/id/settings-reference#maxeffortlevel) membatasi upaya pada klien sebagai gantinya. Ketika keduanya berlaku untuk model, batas yang lebih rendah diterapkan.

Admin organisasi pada paket Claude Enterprise dapat menetapkan tingkat [upaya](#adjust-effort-level) maksimum per model untuk setiap peran kustom, bersama dengan [pembatasan model organisasi](#organization-model-restrictions) tingkat peran. Level di atas batas tidak ditawarkan dalam pemilih `/effort`, dan menamai level yang lebih tinggi dengan `--effort` atau `/effort` berjalan pada batas sebagai gantinya. Dalam sesi interaktif dan `--print` teks biasa, peringatan menyebutkan level yang diminta dan diterapkan; dengan output `json` atau `stream-json` atau dalam agen latar belakang, penjepit diterapkan secara diam-diam. Batas adalah per model, jadi beralih model dapat mengubah level mana yang tersedia. Ketika beberapa peran Anda memberikan model yang sama, batas paling tidak ketat berlaku. Memerlukan Claude Code v2.1.195 atau lebih baru.

Batas upaya disampaikan bersama dengan [pembatasan model organisasi](#organization-model-restrictions) dan mencapai sesi yang sama.

<h2 id="special-model-behavior">
  Perilaku model khusus
</h2>

<h3 id="default-model-setting">
  Pengaturan model `default`
</h3>

Perilaku `default` tergantung pada jenis akun Anda:

* **Pro, Max, Team, Enterprise, dan Anthropic API**: default ke Opus 5.5
* **Claude Platform on AWS, Amazon Bedrock, dan Google Cloud's Agent Platform**: default ke Opus 5.5
* **Microsoft Foundry**: default ke Sonnet 4.5

Sebelum v2.1.280, `default` diselesaikan ke Sonnet 5 pada Pro dan Team Standard, dan ke Opus 5 pada Max, Team Premium, Enterprise, Anthropic API, Claude Platform on AWS, Amazon Bedrock, dan Google Cloud's Agent Platform dari v2.1.219. Sebelum v2.1.219, `default` diselesaikan ke Opus 4.8 pada Anthropic API, Max, Team Premium, dan Enterprise pay-as-you-go dari v2.1.154, dan pada Claude Platform on AWS, Amazon Bedrock, dan Google Cloud's Agent Platform dari v2.1.207. Sebelum v2.1.207, `default` diselesaikan ke Opus 4.7 pada Claude Platform on AWS dan ke Sonnet 4.5 pada Amazon Bedrock dan Google Cloud's Agent Platform.

Ketika admin telah menetapkan [model default organisasi](#organization-default-model), `default` diselesaikan ke model tersebut alih-alih default jenis akun di atas. Memerlukan Claude Code v2.1.196 atau lebih baru. `default` juga dapat diselesaikan ke model yang Anda atur dengan [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), di bawah kondisi yang tercantum di bagiannya.

Ketika pengaturan terkelola [memberlakukan daftar yang diizinkan untuk model Default](#enforce-the-allowlist-for-the-default-model) dan default jenis akun tidak ada dalam `availableModels`, `default` diselesaikan ke Default yang diberlakukan alih-alih default jenis akun di atas. Ketika keduanya berlaku, default organisasi menggantikan default jenis akun terlebih dahulu dan penegakan kemudian diterapkan padanya: default organisasi yang terdaftar dipertahankan, sementara yang di luar daftar diselesaikan ke Default yang diberlakukan.

Model Fable bukan default jenis akun pada paket atau penyedia apa pun. Memilih salah satu dengan `/model` menyimpannya sebagai model yang dipilih dalam pengaturan pengguna Anda, sehingga sesi berikutnya dimulai padanya. Untuk perubahan satu kali yang Claude Code buat pada pilihan Fable 5 yang disimpan di v2.1.257, lihat [Bekerja dengan Fable](#work-with-fable).

<h3 id="opusplan-model-setting">
  Pengaturan model `opusplan`
</h3>

Alias model `opusplan` menyediakan pendekatan hibrida otomatis:

* **Dalam plan mode**: menggunakan `opus` untuk penalaran kompleks dan keputusan arsitektur
* **Dalam execution mode**: secara otomatis beralih ke `sonnet` untuk pembuatan kode dan implementasi

Ini menggabungkan penalaran Opus untuk perencanaan dengan efisiensi Sonnet untuk eksekusi.

Fase Opus plan-mode menggunakan jendela konteks yang sama dengan pengaturan model `opus`, dan fase eksekusi menggunakan jendela yang sama dengan `sonnet`. Ketika `opus` dan `sonnet` diselesaikan ke model yang berjalan dengan [jendela konteks 1M](#extended-context) secara default, seperti model saat ini pada Anthropic API, kedua fase berjalan dengannya. Untuk meminta konteks 1M untuk kedua fase di mana mereka tidak, [atur model](#setting-your-model) ke `opusplan[1m]`, misalnya dengan `/model opusplan[1m]`. Menetapkannya dengan `/model` memerlukan Claude Code v2.1.265 atau lebih baru; pada versi sebelumnya, gunakan flag `--model` atau pengaturan `model` sebagai gantinya.

Ketika [`availableModels`](#restrict-model-selection) mengecualikan Opus terbaru tetapi mengizinkan versi yang lebih lama, misalnya `["sonnet", "claude-opus-4-6"]`, `opusplan` menggunakan Opus terbaru yang diizinkan untuk perencanaan dan tetap pada Sonnet hanya ketika setiap Opus dikecualikan. Sesi Haiku yang biasanya akan ditingkatkan ke Sonnet dalam plan mode juga menggunakan Sonnet terbaru yang diizinkan, dan tetap pada Haiku hanya ketika setiap Sonnet dikecualikan. Sebelum v2.1.205, plan mode tetap pada model sesi kapan pun versi terbaru dari keluarga upgrade dikecualikan, bahkan ketika daftar yang diizinkan mengizinkan yang lebih lama.

Substitusi versi yang lebih lama yang diizinkan berlaku pada Anthropic API dan [Claude Platform on AWS](/docs/id/claude-platform-on-aws). Pada Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan Mantle, yang penerapannya menggunakan ID model khusus penyedia, plan mode tetap pada model sesi kapan pun model upgrade dikecualikan.

Untuk pendekatan hibrida di mana Claude memutuskan di tengah-tugas kapan harus berkonsultasi dengan model kedua daripada beralih di batas rencana, lihat [alat advisor](/docs/id/advisor).

<h3 id="fallback-model-chains">
  Rantai model fallback
</h3>

Ketika model utama kelebihan beban, tidak tersedia, atau mengembalikan kesalahan server non-retryable lainnya, Claude Code dapat beralih ke model fallback alih-alih gagal permintaan. Kesalahan autentikasi, penagihan, rate-limit, ukuran permintaan, dan transportasi, serta [penolakan oleh pemeriksaan kebijakan organisasi Anda](/docs/id/errors#automatic-retries), tidak pernah memicu switch; mereka mengikuti penanganan retry dan error normal mereka.

Konfigurasikan satu atau lebih model fallback dan Claude Code mencobanya secara berurutan, menampilkan pemberitahuan saat beralih. Switch berlangsung untuk giliran saat ini saja, jadi pesan berikutnya Anda mencoba model utama terlebih dahulu lagi. Claude Code membatasi rantai pada tiga model setelah penghapusan duplikat dan mengabaikan entri tambahan.

Atur rantai untuk satu sesi dengan flag `--fallback-model`, yang menerima daftar yang dipisahkan koma:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Untuk mempertahankan rantai di seluruh sesi, atur `fallbackModel` dalam [settings](/docs/id/settings) sebagai array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Flag `--fallback-model` mengambil alih pengaturan `fallbackModel`. Setiap entri menerima nama model atau alias, dan `"default"` berkembang ke model default.

Claude Code tidak mengkonfirmasi rantai saat startup dan `/status` tidak menampilkannya. Pemberitahuan yang ditampilkan saat switch terjadi adalah tanda pertama yang terlihat bahwa fallback dikonfigurasi.

Ketika permintaan gagal, Claude Code mencoba setiap entri secara berurutan sampai salah satu menerimanya. Entri yang juga tidak dapat dijangkau, seperti model yang pensiun yang disematkan dalam pengaturan, gagal ke entri berikutnya dengan cara yang sama. Claude Code menghapus dua jenis entri sebelum walk itu dimulai:

* **Di luar daftar yang diizinkan**: Claude Code menghapus entri apa pun yang tidak diizinkan oleh [`availableModels`](#restrict-model-selection) saat membaca rantai.
* **Jendela konteks lebih kecil selama compaction**: rantai juga mencakup [compaction](/docs/id/context-window#what-survives-compaction), tetapi Claude Code tidak akan fallback ke model dengan jendela konteks lebih kecil dari model utama, karena merangkum di sana akan memotong bagian percakapan terlebih dahulu. Jika setiap fallback lebih kecil, compaction menampilkan error asli dan Anda dapat mencoba lagi.

Claude Code juga menerapkan rantai ke [subagents](/docs/id/sub-agents). Ketika permintaan subagent gagal, Claude Code mencoba model fallback yang dikonfigurasi secara berurutan, dan subagent melanjutkan pada model yang menerima permintaan. Model sesi Anda tidak berubah. Sebelum v2.1.247, kegagalan yang rantai tutupi mengakhiri subagent.

<h3 id="automatic-model-fallback">
  Fallback model otomatis
</h3>

Bagian ini mencakup fallback berbasis konten dari model Fable, Opus 5.5, dan Opus 5. Untuk fallback berbasis ketersediaan ketika model kelebihan beban atau tidak tersedia, lihat [Rantai model fallback](#fallback-model-chains).

Model Fable, Opus 5.5, dan Opus 5 berjalan dengan pengklasifikasi keamanan, yang paling sering menandai konten keamanan siber dan biologi. Ketika pengklasifikasi menandai permintaan dan kategori yang ditandai memiliki model fallback, Claude Code menjalankan kembali permintaan pada model tersebut dan menampilkan pemberitahuan dalam transkrip. Untuk dua kategori tersebut, model fallback tergantung pada model mana yang menolak:

* **Fable 5.1, Fable 5, dan Opus 5.5**: permintaan yang ditandai biologi dijalankan kembali pada Opus 5, dan permintaan yang ditandai keamanan siber dijalankan kembali pada Opus 4.8.
* **Opus 5**: permintaan yang ditandai keamanan siber dijalankan kembali pada Opus 4.8. Permintaan yang ditandai biologi berakhir dengan penolakan, karena Opus 5 menjalankan pengklasifikasi biologi sendiri tanpa model fallback.

Pada Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, Claude Code menyelesaikan target ini melalui penerapan Anda, dan jika Anda menetapkan `ANTHROPIC_DEFAULT_OPUS_MODEL`, kategori yang memiliki fallback dijalankan kembali pada model yang disematkan; lihat [Aktifkan fallback pada Bedrock, Agent Platform, dan Foundry](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Setelah fallback, sesi berlanjut pada model fallback. Untuk kembali ke model asli Anda, jalankan [`/model`](#setting-your-model).

Fallback berbasis kategori memerlukan Claude Code v2.1.219 atau lebih baru. Sebelum v2.1.219, setiap permintaan Fable 5 yang ditandai dijalankan kembali pada model Opus default penyedia Anda, dan Opus 5 bukan sumber fallback.

Model fallback diperiksa terhadap [`availableModels`](#restrict-model-selection). Ketika diblokir, tidak ada fallback yang terjadi. Penolakan ditampilkan sebagai error normal dan model sesi tidak berubah.

<h4 id="check-what-triggered-fallback">
  Periksa apa yang memicu fallback
</h4>

Fallback dapat memicu pada permintaan pertama sesi, sebelum Anda mengirim apa pun yang tidak biasa, karena permintaan pertama membawa konteks ruang kerja seperti konten CLAUDE.md dan status git Anda. Repositori yang berisi materi keamanan atau biologi dapat memicu pengklasifikasi pada konteks itu saja.

Untuk memeriksa apakah kustomisasi adalah pemicunya, mulai sesi dengan `claude --safe-mode`, yang menonaktifkan kustomisasi seperti CLAUDE.md, skills, server MCP, dan hooks. Status git dan nama direktori bukan kustomisasi dan masih disertakan.

<h4 id="ask-before-switching">
  Tanyakan sebelum beralih
</h4>

Untuk memutuskan apa yang terjadi setiap kali permintaan ditandai, daripada beralih secara otomatis, jalankan `/config` dan matikan **Switch models when a message is flagged**, atau atur [`switchModelsOnFlag`](/docs/id/settings-reference#switchmodelsonflag) ke `false` dalam file pengaturan Anda. Permintaan yang ditandai kemudian menjeda sesi dengan dua opsi: beralih ke model fallback, atau edit prompt dan coba lagi pada model saat ini.

Beberapa kasus berperilaku berbeda:

* Ketika kategori yang ditandai tidak memiliki model fallback, seperti flag biologi pada Opus 5, Claude Code tidak menampilkan prompt dan permintaan berakhir dengan penolakan.
* Jika kedua model menandai permintaan yang sama, Anda dapat mengedit prompt dan mencoba lagi, atau memulai sesi baru.
* Pada sesi [cloud sessions](/docs/id/claude-code-on-the-web) di aplikasi mobile, pengeditan dan pengulangan tidak didukung. Beralih model, atau lanjutkan sesi dari browser desktop atau aplikasi desktop.
* Dalam [mode non-interaktif](/docs/id/cli-reference#cli-flags) dan integrasi SDK yang tidak dapat menampilkan prompt, permintaan yang ditandai mengakhiri giliran dengan penolakan.
* Ketika target fallback diblokir oleh [`availableModels`](#restrict-model-selection), Claude Code tidak menampilkan prompt. Permintaan yang ditandai berakhir dengan penolakan, sama seperti fallback otomatis ketika target diblokir.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Aktifkan fallback pada Bedrock, Agent Platform, dan Foundry
</h4>

Pada [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), ID model khusus penyedia, jadi fallback otomatis hanya beroperasi ketika Claude Code dapat mengidentifikasi kedua model yang terlibat:

* Claude Code harus mengenali model saat ini sebagai sumber fallback. Fable 5.1 dan Fable 5 dikenali ketika ID model berisi `claude-fable-5`, cocok dengan nilai `ANTHROPIC_DEFAULT_FABLE_MODEL`, atau dipetakan dengan [`modelOverrides`](#override-model-ids-per-version). Opus 5.5 dan Opus 5 dikenali oleh ID model penyedianya atau pemetaan [`modelOverrides`](#override-model-ids-per-version).
* Model fallback harus diselesaikan dalam penerapan Anda. Jika Anda menetapkan `ANTHROPIC_DEFAULT_OPUS_MODEL`, permintaan yang ditandai dijalankan kembali pada model tersebut untuk setiap kategori yang memiliki fallback; flag biologi pada Opus 5 masih berakhir dengan penolakan. Jika Anda tidak menetapkannya, permintaan yang ditandai keamanan siber dijalankan kembali pada entri Opus 4.8 dalam daftar model penyedia, dan permintaan yang ditandai biologi dari model Fable atau Opus 5.5 pada entri Opus 5.

Jika salah satu model tidak dapat diidentifikasi, Claude Code tidak beralih secara otomatis. Permintaan yang ditandai berakhir dengan pesan penolakan, dan Anda dapat beralih model dengan [`/model`](#setting-your-model) dan mencoba lagi. Menetapkan `ANTHROPIC_DEFAULT_FABLE_MODEL` ke ID model Fable Anda memungkinkan pengenalan Fable. Menetapkan `ANTHROPIC_DEFAULT_OPUS_MODEL` ke ID model Opus memberikan kategori yang ditandai target fallback, kecuali pin menamai model di luar keluarga Opus atau model yang menolak; kemudian Claude Code tidak beralih dan penolakan berdiri.

<h4 id="security-research-and-biology-workloads">
  Penelitian keamanan dan beban kerja biologi
</h4>

Beban kerja dalam keamanan ofensif atau biologi, termasuk penetration testing, latihan Capture the Flag (CTF), dan codebase yang berdekatan dengan biologi, memicu fallback sering, sering pada permintaan pertama. Untuk pekerjaan biologi substantif pada Fable 5.1, Fable 5, atau Opus 5.5, Claude Code memindahkan sesi ke Opus 5 pada permintaan pertama yang ditandai, dan permintaan yang ditandai biologi kemudian berakhir dalam penolakan di sana, karena Opus 5 tidak memiliki fallback biologi. Pada Opus 5, Anda mendapatkan penolakan tersebut dari permintaan pertama yang ditandai.

Ini adalah routing yang diharapkan untuk domain ini, bukan flag akun. Jika organisasi Anda memerlukan kemampuan kelas Fable untuk pekerjaan ini, tanyakan kepada tim akun Anthropic Anda tentang program akses terpercaya.

<h3 id="adjust-effort-level">
  Sesuaikan tingkat usaha
</h3>

[Tingkat usaha](https://platform.claude.com/docs/en/build-with-claude/effort) mengontrol penalaran adaptif, yang memungkinkan model memutuskan apakah dan berapa banyak untuk berpikir pada setiap langkah berdasarkan kompleksitas tugas. Usaha lebih rendah lebih cepat dan lebih murah untuk tugas-tugas langsung, sementara usaha lebih tinggi memberikan penalaran lebih dalam untuk masalah kompleks.

Tingkat usaha yang tersedia tergantung pada model. Model yang tidak tercantum di sini tidak mendukung usaha:

| Model                                              | Tingkat                                 |
| :------------------------------------------------- | :-------------------------------------- |
| Fable 5.1 dan Fable 5                              | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8, dan Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 dan Sonnet 4.6                            | `low`, `medium`, `high`, `max`          |

Jika Anda menetapkan tingkat yang model aktif tidak mendukung, Claude Code kembali ke tingkat tertinggi yang didukung pada atau di bawah yang Anda tetapkan. Misalnya, `xhigh` berjalan sebagai `high` pada Opus 4.6. Organisasi Anda juga dapat membatasi tingkat mana yang tersedia untuk model; lihat [Batas usaha organisasi](#organization-effort-limits).

Dengan pengaturan [`ultracode`](/docs/id/settings-reference#ultracode) mati, Claude Code menyelesaikan tingkat usaha sesi dalam urutan ini, mengambil yang pertama yang berlaku:

1. Pilihan eksplisit: variabel lingkungan [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/id/env-vars#variables), peluncuran dengan `--effort`, atau `/effort` dalam sesi ([`/effort` non-interaktif memiliki efek lebih sempit](#non-interactive-effort))
2. Pengaturan Anda: tingkat yang Anda simpan untuk model atau kunci [`effortLevel`](/docs/id/settings-reference#effortlevel), dengan prioritas di antara mereka dan di seluruh file pengaturan yang dinyatakan pada [`modelSettings`](/docs/id/settings-reference#modelsettings)
3. Tingkat usaha default model: `high` pada setiap model yang mendukung usaha, kecuali Opus 5.5 default ke `medium`, Opus 4.7 default ke `xhigh`, dan, ketika organisasi Anda menetapkan tingkat usaha default untuk [model default organisasinya](#organization-default-model), tingkat itu adalah default ketika Anda menjalankan model tersebut

Opus 5.5 dimulai pada `medium` kecuali salah satu sumber di atas menetapkan tingkat untuk itu, dan `effortLevel` tingkat atas dalam file pengaturan pengguna Anda tidak dihitung untuk Opus 5.5. Kunci itu adalah bentuk yang lebih lama `/effort` ditulis sebelum Claude Code menyimpan tingkat per model: itu terus berlaku di mana itu berlaku sebelumnya, pada Opus 5, Fable 5.1, dan model sebelumnya, sementara Opus 5.5 dan model yang dirilis setelahnya dimulai pada default mereka sendiri sampai Anda memilih tingkat untuk mereka dengan `/effort` atau pengambil `/model`. `effortLevel` tingkat atas dalam pengaturan proyek, lokal, atau terkelola, atau yang dilewatkan dengan `--settings`, berlaku untuk setiap model.

Ketika Anda menetapkan `low`, `medium`, `high`, atau `xhigh` dalam sesi interaktif pada mesin Anda, Anda memilih berapa lama itu berlangsung dengan cara Anda mengkonfirmasinya:

* `Enter` dalam slider `/effort` atau pengambil `/model`, atau tingkat yang diketik setelah `/effort`: simpan tingkat sebagai default Anda dan terapkan dalam sesi kemudian
* `s` dalam slider `/effort` atau pengambil `/model`: terapkan tingkat ke sesi ini saja. Memerlukan Claude Code v2.1.257 atau lebih baru

Claude Code menyimpan tingkat per model, di bawah kunci [`modelSettings`](/docs/id/settings-reference#modelsettings) dalam pengaturan pengguna Anda, jadi setiap model menyimpan tingkat tersimpannya sendiri.

`max` adalah tingkat penalaran terdalam. Kecuali Anda menetapkannya melalui variabel lingkungan `CLAUDE_CODE_EFFORT_LEVEL`, Claude Code menerapkan `max` ke sesi saat ini saja.

<Note>
  Tingkat yang Anda pilih dari kontrol usaha pada ponsel atau browser yang terhubung melalui [Remote Control](/docs/id/remote-control#what-connected-devices-see) berlaku untuk sesi itu saja.
</Note>

<span id="non-interactive-effort" />

Ketika Anda menetapkan tingkat dengan `/effort` dalam [run `-p`](/docs/id/headless), Claude Code menerapkannya ke sesi itu saja dan tidak menyimpannya sebagai default Anda.

Menu `/effort` juga menawarkan `ultracode`. Ultracode adalah pengaturan Claude Code daripada tingkat usaha model: ia mengirim `xhigh` ke model dan selain itu memiliki Claude mengorkestra [alur kerja dinamis](/docs/id/workflows) untuk tugas-tugas substantif. Untuk di mana dapat diatur secara persisten, lihat pengaturan [`ultracode`](/docs/id/settings-reference#ultracode).

Anda dapat mengaktifkan ultracode melalui salah satu dari berikut ini:

* **`/effort`**: jalankan `/effort ultracode`, atau pilih dari menu
* **Flag `--effort`**: luncurkan dengan `claude --effort ultracode`, yang memulai sesi pada usaha `xhigh` dengan ultracode aktif
* **Pengaturan `ultracode`**: atur [`"ultracode": true`](/docs/id/settings-reference#ultracode) dalam file pengaturan, dengan `--settings`, atau dalam permintaan kontrol Agent SDK. Permintaan [`applyFlagSettings()`](/docs/id/agent-sdk/typescript#applyflagsettings) juga menerima `effortLevel: "ultracode"`
* **Pengambil `/model`**: pindahkan slider usaha ke `ultracode` dengan tombol panah saat Anda memilih model. Claude Code mengaktifkannya untuk sesi saat ini, bahkan ketika Anda menyimpan model tersebut sebagai default Anda

Melewatkan `ultracode` ke flag `--effort` atau nilai Agent SDK `effortLevel` memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.203, `--effort ultracode` mencetak `Unknown --effort value 'ultracode'` dan sesi dimulai pada usaha default.

Pengaturan `effortLevel` yang bertahan dan variabel lingkungan `CLAUDE_CODE_EFFORT_LEVEL` tidak menerima `ultracode`. Ketika `CLAUDE_CODE_EFFORT_LEVEL` diatur ke tingkat selain `xhigh`, permintaan berjalan pada tingkat itu dan orkestrasi alur kerja ultracode tetap tidak aktif. Memilih ultracode kemudian menampilkan peringatan bahwa variabel lingkungan mengganti usaha untuk sesi.

<span id="when-ultracode-is-available" />

Ultracode tidak tersedia ketika:

* [Alur kerja dimatikan](/docs/id/workflows#turn-workflows-off)
* Model tidak mendukung usaha `xhigh`
* [Batas usaha](#organization-effort-limits) di bawah `xhigh` berlaku untuk model

Dalam kasus tersebut `--effort ultracode` memulai sesi dengan ultracode mati, pada tingkat usaha tertinggi yang model dan batas apa pun izinkan, hingga `xhigh`.

<h4 id="choose-an-effort-level">
  Pilih tingkat usaha
</h4>

Setiap tingkat menukar pengeluaran token terhadap kemampuan. Default cocok untuk sebagian besar tugas pengkodean; sesuaikan ketika Anda menginginkan keseimbangan berbeda.

| Tingkat     | Kapan menggunakannya                                                                                                                                                |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `low`       | Cadangkan untuk tugas pendek, terbatas, sensitif latensi yang tidak sensitif intelijen                                                                              |
| `medium`    | Mengurangi penggunaan token untuk pekerjaan sensitif biaya yang dapat menukar beberapa intelijen. Default pada Opus 5.5                                             |
| `high`      | Menyeimbangkan penggunaan token dan intelijen. Default pada setiap model kecuali Opus 5.5 dan Opus 4.7                                                              |
| `xhigh`     | Penalaran lebih dalam pada pengeluaran token lebih tinggi. Default pada Opus 4.7                                                                                    |
| `max`       | Dapat meningkatkan kinerja pada tugas menuntut tetapi mungkin menunjukkan hasil yang berkurang dan rentan terhadap overthinking. Uji sebelum mengadopsi secara luas |
| `ultracode` | Pengaturan Claude Code yang merencanakan [alur kerja dinamis](/docs/id/workflows) untuk setiap tugas substantif dengan penalaran `xhigh` per pesan                       |

Skala usaha dikalibrasi per model, jadi nama tingkat yang sama tidak mewakili nilai dasar yang sama di seluruh model.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Gunakan ultrathink untuk penalaran mendalam sekali jalan
</h4>

Sertakan `ultrathink` di mana saja dalam prompt Anda untuk meminta penalaran lebih dalam pada giliran itu tanpa mengubah pengaturan usaha sesi Anda. Claude Code mengenali kata kunci dan menambahkan instruksi dalam konteks. Tingkat usaha yang dikirim ke API tidak berubah. Claude Code melewatkan frasa lain seperti "think", "think hard", dan "think more" sebagai teks prompt biasa dan tidak mengenalinya sebagai kata kunci.

<h4 id="set-the-effort-level">
  Atur tingkat usaha
</h4>

Anda dapat mengubah usaha melalui salah satu dari berikut ini:

* **`/effort`**: jalankan `/effort` tanpa argumen untuk membuka slider interaktif, `/effort` diikuti dengan nama tingkat untuk menetapkannya secara langsung, atau `/effort auto` untuk menghapus tingkat tersimpan Anda untuk model aktif. Anda dapat menjalankannya saat Claude sedang bekerja, dan setelah Anda mengkonfirmasi [peringatan cache](/docs/id/prompt-caching#changing-effort-level), jika Claude Code menampilkan satu, Claude Code menerapkan tingkat baru ke permintaan berikutnya dalam giliran
* **Dalam `/model`**: gunakan tombol panah kiri/kanan untuk menyesuaikan slider usaha saat memilih model
* **Flag `--effort`**: teruskan nama tingkat untuk menetapkannya untuk satu sesi saat meluncurkan Claude Code
* **Variabel lingkungan**: atur `CLAUDE_CODE_EFFORT_LEVEL` ke nama tingkat atau `auto`
* **Pengaturan**: atur tingkat per model dalam [`modelSettings`](/docs/id/settings-reference#modelsettings), atau atur [`effortLevel`](/docs/id/settings-reference#effortlevel) ke `low`, `medium`, `high`, atau `xhigh` sebagai default untuk model tanpa satu. `max` tidak diterima di kunci mana pun, dan `ultracode` memiliki kunci [`ultracode`](/docs/id/settings-reference#ultracode) sendiri
* **Dari perangkat yang terhubung**: dalam sesi [Remote Control](/docs/id/remote-control#what-connected-devices-see), pilih tingkat dari kontrol usaha pada ponsel Anda atau di browser Anda. Tingkat berlaku untuk sesi saat ini saja. Memerlukan Claude Code v2.1.234 atau lebih baru
* **Frontmatter skill dan subagent**: atur `effort` dalam file markdown [skill](/docs/id/skills#frontmatter-reference) atau [subagent](/docs/id/sub-agents#supported-frontmatter-fields) untuk mengganti tingkat usaha ketika skill atau subagent itu berjalan

Usaha frontmatter berlaku ketika skill atau subagent itu aktif, mengganti tingkat sesi tetapi bukan variabel lingkungan. [`maxEffortLevel`](/docs/id/settings-reference#maxeffortlevel) atau [batas usaha organisasi](#organization-effort-limits) masih membatasi tingkat yang skill atau subagent berjalan.

Jika Anda menetapkan `effortLevel` dalam [pengaturan terkelola](/docs/id/managed-settings), Claude Code menerapkannya pada langkah pengaturan dari [urutan resolusi usaha](#adjust-effort-level), dan pengguna masih dapat mengubah tingkat dengan `/effort` atau `--effort`. Untuk menjaga pengguna pada atau di bawah tingkat, atur [`maxEffortLevel`](/docs/id/settings-reference#maxeffortlevel).

Slider usaha muncul dalam `/model` ketika model yang didukung dipilih. Tingkat usaha saat ini juga ditampilkan dalam header sesi di sebelah nama model, misalnya "with low effort", jadi Anda dapat mengkonfirmasi pengaturan mana yang aktif tanpa membuka `/model`. Footer juga secara singkat menampilkan tingkat usaha saat startup dan saat berubah.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Penalaran adaptif dan anggaran pemikiran tetap
</h4>

Penalaran adaptif membuat pemikiran opsional pada setiap langkah, jadi Claude dapat merespons lebih cepat ke prompt rutin dan menyisihkan pemikiran lebih dalam untuk langkah-langkah yang mendapat manfaat darinya. Jika Anda ingin Claude berpikir lebih atau kurang sering daripada tingkat saat ini menghasilkan, Anda dapat mengatakan demikian langsung dalam prompt Anda atau dalam `CLAUDE.md`; model merespons panduan itu dalam pengaturan usahanya.

Model Fable, Sonnet 5, dan Opus 4.7 dan lebih baru selalu menggunakan penalaran adaptif. Mode anggaran pemikiran tetap dan `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` tidak berlaku untuk mereka.

Pada Opus 4.6 dan Sonnet 4.6, Anda dapat menetapkan `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` untuk kembali ke mode anggaran pemikiran tetap sebelumnya yang dikendalikan oleh `MAX_THINKING_TOKENS`. Lihat [variabel lingkungan](/docs/id/env-vars).

<h3 id="extended-thinking">
  Pemikiran diperpanjang
</h3>

Pemikiran yang diperpanjang adalah penalaran yang Claude keluarkan sebelum merespons. Pada model yang mendukung [penalaran adaptif](#adjust-effort-level), tingkat usaha adalah kontrol utama untuk berapa banyak pemikiran yang terjadi; pengaturan di bawah mengaktifkan atau menonaktifkan pemikiran dan mengontrol cara ditampilkan. Dengan pemikiran dimatikan pada Anthropic API, Claude Code mengirim usaha `high` alih-alih tingkat lebih tinggi ke model yang diketahui [tidak menerima kombinasi itu](/docs/id/errors#effort-isnt-available-with-thinking-turned-off), seperti Opus 5.

| Kontrol                                 | Cara menetapkannya                                                                                                                                                                                                                                                                                                                                                                                                               |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Alihkan untuk sesi saat ini             | Tekan `Option+T` pada macOS atau `Alt+T` pada Windows dan Linux                                                                                                                                                                                                                                                                                                                                                                  |
| Atur default global                     | Jalankan `/config` dan alihkan mode pemikiran. Disimpan sebagai `alwaysThinkingEnabled` dalam `~/.claude/settings.json`                                                                                                                                                                                                                                                                                                          |
| Nonaktifkan melalui variabel lingkungan | Atur [`MAX_THINKING_TOKENS=0`](/docs/id/env-vars), yang menonaktifkan pemikiran pada Anthropic API kecuali pada Opus 5.5 dan model Fable. Pada [penyedia pihak ketiga](/docs/id/third-party-integrations), Claude Code menghilangkan parameter `thinking` sebagai gantinya, dan model penalaran adaptif mungkin masih berpikir. Nilai lain hanya berlaku dengan [anggaran pemikiran tetap](#adaptive-reasoning-and-fixed-thinking-budgets) |

Pemikiran tidak dapat dimatikan pada Opus 5.5 atau model Fable. Alihan sesi, `alwaysThinkingEnabled`, dan `MAX_THINKING_TOKENS=0` tidak berpengaruh di sana, dan model memutuskan per langkah berapa banyak untuk berpikir berdasarkan tingkat usaha.

Claude Code meruntuhkan output pemikiran secara default. Tekan `Ctrl+O` untuk mengalihkan mode verbose dan lihat penalaran sebagai teks miring abu-abu. Sesi interaktif pada Anthropic API menerima blok pemikiran yang diredaksi secara default, jadi atur `showThinkingSummaries: true` dalam [pengaturan](/docs/id/settings) jika Anda menginginkan ringkasan lengkap yang tersedia saat Anda memperluas. Anda dikenakan biaya untuk semua token pemikiran yang dihasilkan, bahkan ketika diruntuhkan atau diredaksi.

<h3 id="extended-context">
  Konteks diperpanjang
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 dan lebih baru, dan Sonnet 4.6 mendukung [jendela konteks 1 juta token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang dengan codebase besar.

Pada Anthropic API, Fable 5.1, Fable 5, Sonnet 5, dan Opus 4.7 dan lebih baru berjalan dengan jendela 1M pada setiap paket, termasuk Pro. Anda tidak memilih varian `[1m]` atau mengaktifkan kredit penggunaan untuk jendela 1M pada model ini. Penggunaan Fable itu sendiri dapat ditagih ke kredit penggunaan pada beberapa paket; lihat [Fable dan kredit penggunaan](#fable-and-usage-credits).

Opus 4.6 dan Sonnet 4.6 mencapai 1M hanya melalui varian `[1m]` mereka, dan akses ke varian itu tergantung pada paket Anda. Pada paket Max, Team, dan Enterprise, termasuk kursi Team Standard dan Team Premium, Opus 4.6 dengan konteks 1M disertakan dengan langganan Anda. Sonnet 4.6 dengan konteks 1M memerlukan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) pada setiap paket langganan, termasuk Max.

| Paket                     | Opus 4.6 dengan konteks 1M                                                                                        | Sonnet 4.6 dengan konteks 1M                                                                                      |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Max, Team, dan Enterprise | Disertakan dengan langganan                                                                                       | Memerlukan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                       | Memerlukan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Memerlukan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API dan pay-as-you-go     | Akses penuh                                                                                                       | Akses penuh                                                                                                       |

Claude Code memeriksa persyaratan paket ini hanya ketika terhubung ke Anthropic API secara langsung. Jika Anda menunjuk `ANTHROPIC_BASE_URL` pada [gateway LLM](/docs/id/llm-gateway#subscriptions-and-gateways) dan login claude.ai tersimpan Anda tetap menjadi kredensial aktif, Claude Code tidak memeriksa kredit penggunaan paket Anda. Opsi `[1m]` tetap tersedia dalam `/model`, dan gateway memutuskan apakah permintaan berhasil. Sebelum v2.1.229, Claude Code menolak `/model sonnet[1m]` dalam konfigurasi itu ketika tidak dapat mengkonfirmasi kredit penggunaan pada akun.

Untuk menonaktifkan konteks 1M, atur `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code menghapus varian model 1M dari pengambil model. Pada model dengan jendela 1M asli, seperti Sonnet 5 dan model Fable, ini juga memperlakukan model sebagai memiliki jendela konteks 200K:

* Dengan auto-compaction aktif, sesi compact pada batas 200K melalui [auto-compaction](#set-the-auto-compact-window). Menetapkan jendela auto-compact di atas 200K tidak mengangkat hold, karena Claude Code membatasi jendela itu pada jendela konteks model.
* Dengan auto-compaction mati, sesi berhenti pada batas 200K dengan [error context-limit](/docs/id/errors#prompt-is-too-long) alih-alih compacting.

Sebelum v2.1.223, Claude Code hanya menyimpan sesi Sonnet 5, Opus 4.8, dan Opus 5 ke 200K. Lihat [variabel lingkungan](/docs/id/env-vars).

Jendela konteks 1M menggunakan harga model standar tanpa premium untuk token di luar 200K. Untuk paket di mana konteks diperpanjang disertakan dengan langganan Anda, penggunaan tetap tercakup oleh langganan Anda. Untuk paket yang mengakses konteks diperpanjang melalui kredit penggunaan, token ditagih ke kredit penggunaan.

Jika akun Anda mendukung konteks 1M, opsi muncul dalam pengambil `/model` dalam versi terbaru Claude Code. Jika Anda tidak melihatnya, coba mulai ulang sesi Anda.

Anda juga dapat menggunakan sufiks `[1m]` dengan alias model atau nama model lengkap:

```text theme={null}
# Gunakan alias opus[1m] atau sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# Atau tambahkan [1m] ke nama model lengkap
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Jendela konteks Sonnet 5
</h4>

Pada Anthropic API, Sonnet 5 selalu berjalan dengan jendela konteks 1M. Tidak ada varian 200K, tidak ada sufiks `[1m]` untuk dipilih, dan tidak ada kredit penggunaan yang diperlukan pada paket apa pun. Sesi auto-compact sebelum jendela penuh, pada sekitar 967K token secara default; atur [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars) untuk memilih ambang batas berbeda.

Dua konfigurasi menganggarkan jendela pada 200K:

* **Gateway LLM**: ketika `ANTHROPIC_BASE_URL` menunjuk pada [gateway](/docs/id/llm-gateway), Claude Code tidak dapat memverifikasi dukungan 1M. Untuk menggunakan jendela penuh, pilih Sonnet 5 (1M context) dalam pengambil model, yang memetakan ke `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: menyimpan sesi pada setiap model dengan jendela 1M asli ke jendela 200K; lihat [Konteks diperpanjang](#extended-context) untuk cara hold ditegakkan. Berguna untuk penerapan yang perlu membatasi konteks.

<h2 id="context-window-and-auto-compaction">
  Jendela konteks dan auto-compaction
</h2>

Jendela auto-compact adalah seberapa penuh jendela konteks dapat menjadi sebelum Claude Code melakukan pemadatan percakapan. Untuk apa yang disimpan dan dihapus pemadatan per mekanisme, lihat [What survives compaction](/docs/id/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Atur jendela auto-compact
</h3>

Anda dapat mengatur jendela auto-compact di tiga tempat:

* **Untuk sesi ini dan sesi berikutnya**: jalankan `/autocompact` dengan nilai, seperti `/autocompact 500k`. Claude Code menyimpannya ke pengaturan pengguna Anda sebagai [`autoCompactWindow`](/docs/id/settings-reference#autocompactwindow) dan menerapkannya ke sesi saat ini; jika [cakupan pengaturan](/docs/id/settings#settings-precedence) dengan prioritas lebih tinggi seperti pengaturan terkelola menetapkan kunci, perintah menyimpan nilai Anda tetapi sesi mempertahankan jendela cakupan tersebut, dan perintah mengatakan demikian. Jalankan `/autocompact auto` untuk kembali ke jendela yang disesuaikan untuk model Anda.
* **Untuk satu peluncuran**: teruskan [`--autocompact`](/docs/id/cli-reference#cli-flags) saat memulai Claude Code. Bendera menimpa pengaturan tersimpan Anda untuk peluncuran itu tanpa mengubahnya, dan `claude --autocompact auto` menjalankan sesi pada jendela yang disesuaikan bahkan jika pengaturan tersimpan Anda memiliki nilai. Tidak seperti `/autocompact`, bendera tidak didahului oleh cakupan pengaturan dengan prioritas lebih tinggi seperti pengaturan terkelola.
* **Dalam skrip dan lingkungan cloud**: atur [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars). Saat diatur, itu memiliki prioritas atas perintah, bendera, dan pengaturan, dan `/autocompact` melaporkan penggantian alih-alih mengubah jendela.

Perintah dan bendera menerima ukuran jendela dari 100K hingga 1M token, dalam salah satu bentuk ini:

* Hitungan token biasa, seperti `200000`
* Akhiran `k` atau `M`, seperti `500k` atau `1M`
* Angka biasa dari 100 hingga 1000, berarti ribuan, jadi `200` menetapkan 200.000

Variabel lingkungan hanya menerima hitungan token biasa. Claude Code membatasi jendela pada jendela konteks model.

<h3 id="default-auto-compact-thresholds">
  Ambang batas auto-compact default
</h3>

Jika Anda tidak mengatur jendela auto-compact, Claude Code melakukan pemadatan ketika percakapan mencapai batas konteks model, kecuali dalam sesi ini:

* [Sesi cloud](/docs/id/claude-code-on-the-web) melakukan pemadatan saat percakapan mendekati batas model
* Sonnet 4.6 dan Opus 4.6 tanpa [konteks diperluas](#extended-context) melakukan pemadatan pada batas 200K, dan begitu juga Opus 4.8 dan yang lebih baru ketika mereka berjalan dengan jendela konteks 200K, seperti di Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry
* Ketika Anda mengatur [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/id/env-vars), model dengan jendela asli 1M, seperti Sonnet 5 dan model Fable, melakukan pemadatan pada batas 200K
* Model yang berjalan dengan jendela asli 1M, seperti Sonnet 5, model Fable, dan Opus 4.7 dan yang lebih baru di Anthropic API, melakukan pemadatan sebelum jendela penuh, pada sekitar 967K token secara default. Di Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry, [Pin models for third-party deployments](#pin-models-for-third-party-deployments) mengatakan model mana yang berjalan dengan jendela itu; untuk konfigurasi yang menganggarkan Sonnet 5 pada 200K sebagai gantinya, lihat [Sonnet 5 context window](#sonnet-5-context-window)
* Sesi pada ID model yang Claude Code tidak kenali, seperti alias [gateway LLM](/docs/id/llm-gateway), melakukan pemadatan pada jendela konteks yang Claude Code asumsikan untuk ID; lihat [Correct the window for a gateway or custom model ID](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Perbaiki jendela untuk gateway atau ID model kustom
</h3>

Pada [gateway LLM](/docs/id/llm-gateway) atau penyebaran kustom lainnya, Claude Code dapat mengasumsikan jendela konteks untuk ID model yang berbeda dari jendela asli model, terlepas dari apakah itu menyelesaikan ID ke model Claude. Atur [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/id/env-vars) ke jendela yang Claude Code harus asumsikan sebagai gantinya.

Bagaimana variabel diterapkan tergantung pada ID. Claude Code memperlakukan ID sebagai penyedia atau ejaan kustom ketika tidak dimulai dengan `claude-`, dalam huruf apa pun, atau ketika membawa akhiran yang Claude Code lepaskan saat membaca ID, seperti akhiran tanggal `@YYYYMMDD` yang digunakan di Platform Agen Google Cloud. Sebelum v2.1.259, Claude Code tidak menghitung akhiran yang dilucuti, jadi ID `claude-` yang tidak dikenali dengan akhiran tanggal diperlakukan sebagai nama `claude-` biasa.

Penyedia atau ejaan kustom yang tidak dikenali, ejaan yang sama dengan `[1m]`, dan setiap ID lainnya adalah tiga kasus terpisah:

* Jika Claude Code tidak dapat menyelesaikan penyedia atau ejaan kustom ke model yang dikenalinya dan ID tidak berisi `[1m]`, variabel diterapkan secara langsung dan pemadatan proaktif berlanjut pada jendela yang dideklarasikan.
* Jika Claude Code tidak dapat menyelesaikan penyedia atau ejaan kustom ke model yang dikenalinya dan ID berisi `[1m]`, dalam huruf apa pun, Claude Code mengasumsikan jendela 1M untuk itu dan variabel tidak diterapkan dengan sendirinya. Untuk memperbaiki jendela sambil mempertahankan pemadatan proaktif, juga atur [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/id/env-vars). Dengan variabel itu diatur, Claude Code mengukur ID seperti ejaan yang sama tanpa `[1m]`, jadi `CLAUDE_CODE_MAX_CONTEXT_TOKENS` diterapkan ketika itu akan diterapkan pada ejaan tanpa tag itu.

  Dengan jendela yang dideklarasikan di atas 200K, Claude Code kemudian menampilkan [peringatan startup](/docs/id/errors#the-200k-limit-isnt-enforced) bahwa batas 200K tidak diterapkan. Peringatan diharapkan dalam konfigurasi ini.
* Jika ID diselesaikan ke model Claude Code yang dikenali, atau ID adalah nama `claude-` biasa tanpa akhiran untuk Claude Code lepaskan, dalam huruf apa pun, variabel hanya berlaku ketika Anda juga mengatur [`DISABLE_COMPACT`](/docs/id/env-vars), yang menonaktifkan semua pemadatan.

  Misalnya, ID yang berisi nama model Claude yang Claude Code ketahui, seperti `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, atau `claude-sonnet-4-5@20250929` yang bertanggal, diselesaikan ke model itu. Ini termasuk ID yang juga berisi `[1m]`: Claude Code menyelesaikan `claude-opus-4-8[1m]` ke Opus 4.8 bahkan dengan `CLAUDE_CODE_DISABLE_1M_CONTEXT` diatur.

Untuk ID model yang Claude Code tidak kenali, atur [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/id/env-vars) agar Claude Code melakukan pemadatan hanya setelah API menolak percakapan dengan [kesalahan terlalu panjang yang Claude Code kenali](/docs/id/errors#prompt-is-too-long). Claude Code tidak menjalankan pemulihan itu ketika gateway [menulis ulang kesalahan](/docs/id/llm-gateway-connect#troubleshoot-gateway-errors) ke kata-kata yang Claude Code tidak kenali.

<h2 id="checking-your-current-model">
  Memeriksa model Anda saat ini
</h2>

Anda dapat melihat model mana yang sedang Anda gunakan di dua tempat:

* Dalam [status line](/docs/id/statusline), jika Anda memiliki satu yang dikonfigurasi
* Dalam `/status`, yang juga menampilkan informasi akun Anda

<h2 id="add-a-custom-model-option">
  Tambahkan opsi model kustom
</h2>

Gunakan `ANTHROPIC_CUSTOM_MODEL_OPTION` untuk menambahkan satu entri kustom ke pemilih `/model` tanpa mengganti alias bawaan. Ini berguna untuk pengujian ID model yang tidak tercantum Claude Code secara default. Untuk deployment gateway LLM, Claude Code dapat mengisi pemilih dari endpoint `/v1/models` gateway ketika `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` diatur, jadi variabel ini diperlukan hanya ketika penemuan dinonaktifkan atau tidak mengembalikan model yang Anda inginkan. Lihat [penemuan model gateway](/docs/id/llm-gateway-protocol#model-discovery).

Untuk membuat daftar beberapa model sebagai gantinya, dalam urutan Anda sendiri dan di bawah label yang Anda pilih, atur [`modelPicker`](/docs/id/settings-reference#modelpicker). Entrinya mengatakan baris mana yang disimpan pemilih ketika lineup itu menggantikan yang bawaan.

Contoh ini menetapkan ketiga variabel untuk membuat deployment Opus yang dirutekan gateway dapat dipilih. Claude Code membaca variabel lingkungan saat startup, jadi jalankan ekspor sebelum meluncurkan `claude`, atau mulai ulang sesi yang ada untuk mengambilnya:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` dan `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` bersifat opsional:

* Jika Anda menghilangkan nama, entri menampilkan nama model ketika Claude Code [mengenali ID](#customize-pinned-model-display-and-capabilities), dan ID model sebaliknya.
* Jika Anda menghilangkan deskripsi, Claude Code menggunakan `Custom model (<model-id>)`.

Claude Code membuat daftar entri kustom setelah entri bawaan, dan baris [`modelPicker`](/docs/id/settings-reference#modelpicker) apa pun yang Anda tambahkan datang setelahnya.

Claude Code melewati validasi untuk ID model yang ditetapkan dalam `ANTHROPIC_CUSTOM_MODEL_OPTION`, sehingga Anda dapat menggunakan string apa pun yang diterima endpoint API Anda.

Ketika [`availableModels`](#restrict-model-selection) diatur, sertakan ID model kustom dalam daftar izin juga. Jika tidak, Claude Code menyaring entri kustom dari pemilih dan menolak pemilihan `--model` darinya seperti model yang dikecualikan lainnya.

ID kustom yang menyematkan nama keluarga, seperti `my-gateway/claude-opus-5-5`, dihitung sebagai entri spesifik untuk keluarga itu dan menonaktifkan wildcard-nya, jadi juga daftarkan versi yang ingin Anda pertahankan dapat dipilih. Lihat [Perilaku penggabungan](#merge-behavior).

<h2 id="environment-variables">
  Variabel lingkungan
</h2>

Gunakan variabel lingkungan berikut untuk mengontrol nama model yang dipetakan alias. Setiap nilai harus berupa nama model lengkap, atau pengenal setara untuk penyedia API Anda. Untuk memilih model yang dimulai sesi Anda, atur [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), yang tabel ini abaikan.

| Variabel lingkungan              | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Model yang digunakan untuk `fable`, dan ID model yang dikenali Claude Code sebagai model Fable untuk [fallback model otomatis](#automatic-model-fallback) pada penyedia pihak ketiga                                                                                                                                                                                                                                                                                                       |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Model yang digunakan untuk `opus`, atau untuk `opusplan` ketika Plan Mode aktif.                                                                                                                                                                                                                                                                                                                                                                                                           |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Model yang digunakan untuk `sonnet`, atau untuk `opusplan` ketika Plan Mode tidak aktif.                                                                                                                                                                                                                                                                                                                                                                                                   |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Model yang digunakan untuk `haiku`, atau [fungsionalitas latar belakang](/docs/id/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                 |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Model default untuk [subagents](/docs/id/sub-agents#choose-a-model), [agent team](/docs/id/agent-teams#specify-teammates-and-models) rekan kerja, dan agen [workflow](/docs/id/workflows) yang tidak ditugaskan model dengan cara lain. Menerima alias seperti `haiku` atau nama model lengkap. Model per-invocation atau bidang `model` definisi, termasuk `inherit`, mengambil prioritas. Untuk mengubah itu, atur [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/id/sub-agents#run-every-subagent-on-one-model) |

Catatan: `ANTHROPIC_SMALL_FAST_MODEL` sudah usang dan digantikan oleh `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Tetapkan model untuk deployment pihak ketiga
</h3>

Saat menerapkan Claude Code melalui [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws), tetapkan versi model sebelum meluncurkan ke pengguna.

Tanpa penentapan, Claude Code menggunakan alias model seperti `fable`, `opus`, `sonnet`, dan `haiku` yang diselesaikan ke ID model default bawaan untuk setiap penyedia. Default tersebut dapat tertinggal dari rilis Anthropic terbaru, dan model yang ditunjuknya mungkin belum diaktifkan di akun pengguna. Ketika default tidak tersedia, pengguna Amazon Bedrock dan Google Cloud's Agent Platform melihat pemberitahuan dan sesi kembali ke versi sebelumnya dari model default, atau ke model Sonnet default ketika default adalah model Opus dan tidak ada versi Opus yang tersedia. Pengguna Microsoft Foundry melihat kesalahan sebagai gantinya, karena Microsoft Foundry tidak memiliki pemeriksaan startup yang setara.

Di Amazon Bedrock dan Google Cloud's Agent Platform, pengguna yang memulai sesi pada versi Sonnet atau Opus tertentu, misalnya dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`, menetapkan versi itu sebagai default sesi untuk alias yang cocok: pemeriksaan startup melewati default bawaan yang digantikannya dan tidak menampilkan pemberitahuan fallback. Sebelum v2.1.211, pemeriksaan berjalan dan dapat menampilkan pemberitahuan bahkan ketika model sesi dikonfigurasi secara eksplisit.

<Warning>
  Atur variabel lingkungan model ke ID versi spesifik sebagai bagian dari pengaturan awal Anda. Penentapan memungkinkan Anda mengontrol kapan pengguna Anda pindah ke model baru.
</Warning>

Gunakan variabel lingkungan berikut dengan ID model spesifik versi untuk penyedia Anda:

| Penyedia                      | Contoh                                                               |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Terapkan pola yang sama untuk `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, dan `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Untuk ID model saat ini dan warisan di semua penyedia, lihat [Ikhtisar Model](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk meningkatkan pengguna ke versi model baru, perbarui variabel lingkungan ini dan terapkan kembali.

Untuk mengaktifkan [konteks diperluas](#extended-context) untuk model yang ditetapkan, tambahkan `[1m]` ke ID model dalam `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, atau `ANTHROPIC_DEFAULT_FABLE_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Dengan akhiran `[1m]`, jendela konteks 1M berlaku untuk semua penggunaan alias yang ditetapkan, termasuk fase Opus mode-rencana dari [`opusplan`](#opusplan-model-setting) dan [subagents](/docs/id/sub-agents#choose-a-model) yang `model` frontmatter mereka menamai alias.

* Claude Code menghapus akhiran sebelum mengirim ID model ke penyedia Anda.
* Hanya tambahkan `[1m]` ketika model yang mendasar [mendukung konteks 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Akhiran dibaca per variabel, bukan per model. Di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, ID model tanpa `[1m]` dalam satu variabel menggunakan konteks 200K bahkan jika variabel lain menetapkan model yang sama dengan akhiran. Sonnet 5 selalu berjalan dengan jendela 1M pada penyedia ini dan tidak pernah memerlukan akhiran.

<Note>
  Allowlist `availableModels` yang dikirimkan melalui [MDM atau file pengaturan terkelola](/docs/id/managed-settings#delivery-mechanisms) masih berlaku saat menggunakan penyedia pihak ketiga; [pengaturan yang dikelola server tidak dikirimkan di sana](/docs/id/server-managed-settings#platform-availability).

  Penyaringan cocok pada alias model seperti `opus`, awalan versi seperti `claude-opus-4-8`, atau ID model lengkap bentuk penyedia. Awalan spesifik penyedia seperti `us.anthropic.` tidak dilepas, jadi untuk memungkinkan model tertentu, daftarkan ID bentuk penyedia lengkapnya, atau petakan melalui [`modelOverrides`](#override-model-ids-per-version). Untuk model yang ditetapkan, ID itu adalah nilai yang Anda atur dalam variabel `ANTHROPIC_DEFAULT_*_MODEL` miliknya. Akhiran `[1m]` apa pun dilepas dari entri allowlist dan model yang diminta sebelum pencocokan.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Sesuaikan tampilan dan kemampuan model yang ditetapkan
</h3>

Ketika Anda menetapkan model pada penyedia pihak ketiga, barisnya di pemilih `/model` menampilkan nama model secara default jika Claude Code mengenali ID yang ditetapkan, dan ID mentah sebaliknya:

* **Dikenali**: ID model yang tepat yang dikenal Claude Code, seperti ID API Anthropic-nya atau bentuk penyedia atau gateway Anda, dengan atau tanpa akhiran `[1m]`. Tetapkan `us.anthropic.claude-sonnet-4-5-20250929-v1:0` dan baris membaca `Sonnet 4.5`.
* **Tidak dikenali**: ID apa pun yang lain, seperti ARN profil inferensi aplikasi atau versi model yang tidak dikenal Claude Code, kecuali entri [`modelOverrides`](#override-model-ids-per-version) memetakan model ke string yang tepat itu. Di Microsoft Foundry, nama deployment ditentukan pengguna, jadi Claude Code tidak pernah mengenali ID yang ditetapkan di sana, dipetakan atau tidak, dan baris menampilkan nama deployment secara default.

Ketika baris menampilkan nama model, deskripsi defaultnya mencakup ID yang ditetapkan sehingga Anda masih dapat melihat ID mana yang ditetapkan.

Claude Code juga mungkin tidak mengenali fitur mana yang didukung model yang ditetapkan. Anda dapat menetapkan nama tampilan dan deskripsi sendiri dan mendeklarasikan kemampuan dengan variabel lingkungan pendamping untuk setiap model yang ditetapkan.

Variabel ini berlaku pada penyedia pihak ketiga seperti Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Variabel `_NAME` dan `_DESCRIPTION` juga berlaku ketika `ANTHROPIC_BASE_URL` menunjuk ke [gateway LLM](/docs/id/llm-gateway). Mereka tidak berpengaruh saat menghubungkan langsung ke `api.anthropic.com`.

| Variabel lingkungan                                   | Deskripsi                                                                                                                                                                                              |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nama tampilan untuk model Opus yang ditetapkan di pemilih `/model`. Ketika tidak diatur, baris menampilkan nama model jika Claude Code mengenali ID yang ditetapkan, dan ID yang ditetapkan sebaliknya |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Deskripsi tampilan untuk model Opus yang ditetapkan di pemilih `/model`. Ketika tidak diatur, baris menampilkan deskripsi default yang dimulai dengan `Custom Opus model`                              |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Daftar kemampuan yang dipisahkan koma yang didukung model Opus yang ditetapkan                                                                                                                         |

Akhiran `_NAME`, `_DESCRIPTION`, dan `_SUPPORTED_CAPABILITIES` yang sama tersedia untuk `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL`, dan `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code mengaktifkan fitur seperti [tingkat usaha](#adjust-effort-level) dan [extended thinking](#extended-thinking) dengan mencocokkan ID model terhadap pola yang dikenal. ID spesifik penyedia seperti ARN Amazon Bedrock atau nama deployment kustom sering kali tidak cocok dengan pola ini, meninggalkan fitur yang didukung dinonaktifkan. Atur `_SUPPORTED_CAPABILITIES` untuk memberi tahu Claude Code fitur mana yang benar-benar didukung model:

| Nilai kemampuan        | Mengaktifkan                                                                                  |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| `effort`               | [Tingkat usaha](#adjust-effort-level) dan perintah `/effort`                                  |
| `xhigh_effort`         | Tingkat usaha `xhigh`                                                                         |
| `max_effort`           | Tingkat usaha `max`                                                                           |
| `thinking`             | [Extended thinking](#extended-thinking)                                                       |
| `adaptive_thinking`    | Penalaran adaptif yang secara dinamis mengalokasikan pemikiran berdasarkan kompleksitas tugas |
| `interleaved_thinking` | Pemikiran antara panggilan alat                                                               |

Ketika `_SUPPORTED_CAPABILITIES` diatur, kemampuan yang tercantum diaktifkan dan kemampuan yang tidak tercantum dinonaktifkan untuk model yang ditetapkan yang cocok. Ketika variabel tidak diatur, Claude Code kembali ke deteksi bawaan berdasarkan ID model.

Contoh ini menetapkan Opus ke ARN model kustom Amazon Bedrock, menetapkan nama yang ramah, dan mendeklarasikan kemampuannya:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Ganti ID model per versi
</h3>

Di platform yang menyematkan Claude Code dan menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars), konfigurasi model host mengambil prioritas atas pengaturan model terkelola, sementara allowlist `availableModels` terkelola tetap berlaku kecuali host menyediakan miliknya sendiri; [Pengecualian untuk prioritas pengaturan terkelola](/docs/id/settings#exceptions-to-managed-settings-precedence) mengatakan kunci dan variabel mana yang diganti host.

Variabel lingkungan tingkat keluarga di atas mengonfigurasi satu ID model per alias keluarga. Jika Anda perlu memetakan beberapa versi dalam keluarga yang sama ke ID penyedia yang berbeda, gunakan pengaturan `modelOverrides` sebagai gantinya.

`modelOverrides` memetakan ID model Anthropic individual ke string spesifik penyedia yang dikirim Claude Code ke API penyedia Anda. Ketika pengguna memilih model yang dipetakan di pemilih `/model`, Claude Code menggunakan nilai yang Anda konfigurasi alih-alih default bawaan.

Ini memungkinkan administrator enterprise untuk merutekan setiap versi model ke ARN profil inferensi Amazon Bedrock tertentu, nama versi Google Cloud's Agent Platform, atau nama deployment Microsoft Foundry untuk tata kelola, alokasi biaya, atau perutean regional.

Atur `modelOverrides` dalam [file pengaturan](/docs/id/settings#where-settings-live) Anda:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Kunci harus berupa ID model Anthropic seperti yang tercantum dalam [Ikhtisar Model](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk ID model bertanggal, sertakan akhiran tanggal persis seperti yang muncul di sana. Kunci yang tidak dikenal diabaikan.

Untuk menghentikan [baris diagnostik](/docs/id/errors#unrecognized-model-id-on-a-request) `[claude-code:unrecognized_model]` untuk ID seperti alias gateway, tambahkan entri dengan ID itu sebagai nilainya.

Penggantian menggantikan ID model bawaan yang mendukung setiap entri di pemilih `/model`. Di Amazon Bedrock, entri `modelOverrides` mengambil prioritas atas profil inferensi apa pun yang ditemukan Claude Code secara otomatis saat startup. Claude Code meneruskan nilai yang sudah bentuk penyedia asli, seperti ARN profil inferensi Amazon Bedrock atau nama deployment Microsoft Foundry, ke penyedia apa adanya.

Penggantian juga berlaku ketika Anda meneruskan ID model Anthropic secara langsung melalui `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau variabel lingkungan `ANTHROPIC_DEFAULT_*_MODEL`. Di Amazon Bedrock, Google Cloud's Agent Platform, dan [Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint), ID model Anthropic tanpa entri `modelOverrides` diselesaikan ke ID spesifik penyedia yang sama dengan baris pemilih `/model` untuk versi itu, ketika penyedia mendukung versi itu. Mantle mendukung subset versi. Untuk ID model Anthropic di luar subset itu, Claude Code mengirim ID mentah ke Mantle tanpa memetakannya, kecuali entri `modelOverrides` mencakupnya. Sebelum v2.1.200, `--model` dan nilai variabel lingkungan mencapai penyedia apa adanya tanpa melewati peta penggantian.

`modelOverrides` bekerja bersama `availableModels`. Allowlist dievaluasi terhadap ID model Anthropic, bukan nilai penggantian, jadi entri seperti `"opus"` dalam `availableModels` terus cocok bahkan ketika versi Opus dipetakan ke ARN. Ketika `enforceAvailableModels` diatur dalam pengaturan terkelola, Default yang diterapkan diselesaikan melalui `modelOverrides` dari [pengaturan terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) saja. Pemetaan admin, seperti versi yang ditetapkan ke ARN profil inferensi, dihormati dalam Default yang diterapkan. Penggantian dari pengaturan pengguna atau proyek tidak mempengaruhinya.

Ketika `availableModels` diatur dalam [pengaturan terkelola](/docs/id/managed-settings), hanya `modelOverrides` dari pengaturan terkelola yang berlaku untuk ID model Anthropic yang diteruskan secara langsung melalui `--model` atau variabel lingkungan di atas. Claude Code mengabaikan penggantian dalam pengaturan pengguna atau proyek untuk ID tersebut, dan tidak pernah menyelesaikan ID yang dikecualikan daftar terkelola melalui `modelOverrides` dari sumber pengaturan apa pun. Pembatasan sumber terkelola ini memerlukan Claude Code v2.1.200 atau lebih baru. Lihat [Batasi pemilihan model](#restrict-model-selection) untuk cara ID yang diblokir ditangani.

<h3 id="prompt-caching-configuration">
  Konfigurasi prompt caching
</h3>

Claude Code secara otomatis menggunakan [prompt caching](/docs/id/prompt-caching) untuk mengoptimalkan kinerja dan mengurangi biaya. Anda dapat menonaktifkan prompt caching secara global atau untuk tingkat model tertentu:

| Variabel lingkungan             | Deskripsi                                                                                             |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Atur ke `1` untuk menonaktifkan prompt caching untuk semua model. Mengambil alih pengaturan per-model |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Haiku                                |
| `DISABLE_PROMPT_CACHING_SONNET` | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Sonnet                               |
| `DISABLE_PROMPT_CACHING_OPUS`   | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Opus                                 |
| `DISABLE_PROMPT_CACHING_FABLE`  | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Fable                                |

Untuk memilih cache TTL untuk percakapan utama dan untuk subagents secara terpisah, lihat [pilih TTL sendiri](/docs/id/prompt-caching#choose-the-ttl-yourself). Untuk apa yang memicu cache miss, lihat [Bagaimana Claude Code menggunakan prompt caching](/docs/id/prompt-caching).
